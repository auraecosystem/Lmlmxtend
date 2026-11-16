//! Permission-aware agent lifecycle and bounded delegation.
//!
//! Agents run in-process and are trusted code. This module provides explicit
//! capability checks and bounded orchestration, not an operating-system sandbox.

use lmlmxtend_api::{CapabilityId, ExecutionContext, ExtensionResponse};
use lmlmxtend_intelligence::{IntelligenceRouter, TaskIntent};
use lmlmxtend_runtime::Runtime;
use std::collections::{BTreeMap, BTreeSet};
use std::fmt;
use std::sync::Arc;

/// Agent contract version. Independent from extension API versioning.
pub const AGENT_API_VERSION: u16 = 1;

/// Immutable identity and declared capability set for an agent.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AgentManifest {
    pub id: String,
    pub version: String,
    pub api_version: u16,
    pub capabilities: Vec<CapabilityId>,
}

/// A bounded unit of work submitted to an agent.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AgentTask {
    pub task_id: String,
    pub input: String,
    pub correlation_id: Option<String>,
}

impl AgentTask {
    pub fn new(task_id: impl Into<String>, input: impl Into<String>) -> Self {
        Self {
            task_id: task_id.into(),
            input: input.into(),
            correlation_id: None,
        }
    }

    pub fn with_correlation_id(mut self, id: impl Into<String>) -> Self {
        self.correlation_id = Some(id.into());
        self
    }
}

/// Result returned by an agent.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AgentResponse {
    pub output: String,
    pub media_type: String,
}

impl AgentResponse {
    pub fn text(output: impl Into<String>) -> Self {
        Self {
            output: output.into(),
            media_type: "text/plain; charset=utf-8".into(),
        }
    }
}

/// Resource limits for one agent-runtime host.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct AgentRuntimeConfig {
    pub max_task_input_bytes: usize,
    pub max_task_output_bytes: usize,
    /// Maximum number of nested agent delegations, excluding the root agent.
    pub max_delegation_depth: usize,
}

impl Default for AgentRuntimeConfig {
    fn default() -> Self {
        Self {
            max_task_input_bytes: 1_048_576,
            max_task_output_bytes: 1_048_576,
            max_delegation_depth: 8,
        }
    }
}

/// Errors raised by registration, policy checks, delegation, or agent execution.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum AgentError {
    InvalidManifest(String),
    DuplicateAgent(String),
    AgentNotFound(String),
    InputTooLarge,
    OutputTooLarge,
    CapabilityDenied(String),
    CapabilityExecutionFailed(String),
    DelegationCycle(String),
    DelegationDepthExceeded,
    Cancelled,
    DeadlineExceeded,
    ExecutionFailed(String),
}

impl fmt::Display for AgentError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidManifest(reason) => write!(f, "invalid agent manifest: {reason}"),
            Self::DuplicateAgent(id) => write!(f, "agent already registered: {id}"),
            Self::AgentNotFound(id) => write!(f, "agent not found: {id}"),
            Self::InputTooLarge => f.write_str("agent task input exceeds configured limit"),
            Self::OutputTooLarge => f.write_str("agent output exceeds configured limit"),
            Self::CapabilityDenied(capability) => {
                write!(f, "agent is not permitted to use capability '{capability}'")
            }
            Self::CapabilityExecutionFailed(reason) => {
                write!(f, "capability execution failed: {reason}")
            }
            Self::DelegationCycle(id) => write!(f, "agent delegation cycle detected at '{id}'"),
            Self::DelegationDepthExceeded => f.write_str("maximum agent delegation depth exceeded"),
            Self::Cancelled => f.write_str("agent execution cancelled"),
            Self::DeadlineExceeded => f.write_str("agent execution deadline exceeded"),
            Self::ExecutionFailed(reason) => write!(f, "agent execution failed: {reason}"),
        }
    }
}

impl std::error::Error for AgentError {}

/// Implement this contract to register an agent with the local host.
pub trait Agent: Send + Sync {
    fn manifest(&self) -> AgentManifest;
    fn execute(
        &self,
        task: AgentTask,
        context: &AgentContext<'_>,
    ) -> Result<AgentResponse, AgentError>;
}

#[derive(Clone)]
struct AgentEntry {
    manifest: AgentManifest,
    agent: Arc<dyn Agent>,
}

/// A read-only view of an agent's identity and granted capabilities.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AgentDescriptor {
    pub id: String,
    pub version: String,
    pub api_version: u16,
    pub capabilities: Vec<CapabilityId>,
}

/// Host-provided execution context. Capability access is checked against the
/// registered agent manifest on every call.
pub struct AgentContext<'a> {
    agent_id: String,
    runtime: &'a Runtime,
    agent_runtime: &'a AgentRuntime,
    allowed_capabilities: &'a BTreeSet<CapabilityId>,
    execution: &'a ExecutionContext,
    correlation_id: Option<String>,
    delegation_stack: Vec<String>,
}

impl<'a> AgentContext<'a> {
    pub fn agent_id(&self) -> &str {
        &self.agent_id
    }

    pub fn execution_context(&self) -> &ExecutionContext {
        self.execution
    }

    /// Execute an extension capability only if this agent declared it in its manifest.
    pub fn execute_capability(
        &self,
        capability: CapabilityId,
        input: impl Into<String>,
    ) -> Result<ExtensionResponse, AgentError> {
        if !self.allowed_capabilities.contains(&capability) {
            return Err(AgentError::CapabilityDenied(capability.to_string()));
        }
        self.execution
            .check_cancelled()
            .map_err(map_lifecycle_error)?;
        let mut intent = TaskIntent::new(capability, input);
        if let Some(id) = &self.correlation_id {
            intent = intent.with_correlation_id(id.clone());
        }
        IntelligenceRouter
            .execute_with_context(self.runtime, intent, self.execution)
            .map_err(|error| AgentError::CapabilityExecutionFailed(error.to_string()))
    }

    /// Delegate a subtask to another registered agent. Cycles and excessive
    /// nesting are rejected before the delegated agent is invoked.
    pub fn delegate(
        &self,
        target_agent: &str,
        task: AgentTask,
    ) -> Result<AgentResponse, AgentError> {
        let task = if task.correlation_id.is_none() {
            match &self.correlation_id {
                Some(id) => task.with_correlation_id(id.clone()),
                None => task,
            }
        } else {
            task
        };
        self.agent_runtime.execute_inner(
            self.runtime,
            target_agent,
            task,
            self.execution,
            self.delegation_stack.clone(),
        )
    }
}

/// Local registry and synchronous agent orchestrator.
pub struct AgentRuntime {
    config: AgentRuntimeConfig,
    agents: BTreeMap<String, AgentEntry>,
}

impl Default for AgentRuntime {
    fn default() -> Self {
        Self::new(AgentRuntimeConfig::default())
    }
}

impl AgentRuntime {
    pub fn new(config: AgentRuntimeConfig) -> Self {
        Self {
            config,
            agents: BTreeMap::new(),
        }
    }

    /// Validate and register an agent. The manifest is snapshotted at registration.
    pub fn register(&mut self, agent: Arc<dyn Agent>) -> Result<(), AgentError> {
        let manifest = agent.manifest();
        if !valid_agent_id(&manifest.id) || manifest.version.trim().is_empty() {
            return Err(AgentError::InvalidManifest(
                "id must be a lowercase namespaced identifier and version is required".into(),
            ));
        }
        if manifest.api_version != AGENT_API_VERSION {
            return Err(AgentError::InvalidManifest(
                "incompatible agent API version".into(),
            ));
        }
        if self.agents.contains_key(&manifest.id) {
            return Err(AgentError::DuplicateAgent(manifest.id));
        }
        let mut declared = BTreeSet::new();
        for capability in &manifest.capabilities {
            if !declared.insert(capability.clone()) {
                return Err(AgentError::InvalidManifest(format!(
                    "duplicate capability declaration: {capability}"
                )));
            }
        }
        self.agents
            .insert(manifest.id.clone(), AgentEntry { manifest, agent });
        Ok(())
    }

    pub fn agent_count(&self) -> usize {
        self.agents.len()
    }

    pub fn agents(&self) -> Vec<AgentDescriptor> {
        self.agents
            .values()
            .map(|entry| AgentDescriptor {
                id: entry.manifest.id.clone(),
                version: entry.manifest.version.clone(),
                api_version: entry.manifest.api_version,
                capabilities: entry.manifest.capabilities.clone(),
            })
            .collect()
    }

    pub fn execute(
        &self,
        runtime: &Runtime,
        agent_id: &str,
        task: AgentTask,
        execution: &ExecutionContext,
    ) -> Result<AgentResponse, AgentError> {
        self.execute_inner(runtime, agent_id, task, execution, Vec::new())
    }

    fn execute_inner(
        &self,
        runtime: &Runtime,
        agent_id: &str,
        task: AgentTask,
        execution: &ExecutionContext,
        mut delegation_stack: Vec<String>,
    ) -> Result<AgentResponse, AgentError> {
        execution.check_cancelled().map_err(map_lifecycle_error)?;
        if task.input.len() > self.config.max_task_input_bytes {
            return Err(AgentError::InputTooLarge);
        }
        if delegation_stack.iter().any(|id| id == agent_id) {
            return Err(AgentError::DelegationCycle(agent_id.to_owned()));
        }
        if delegation_stack.len() > self.config.max_delegation_depth {
            return Err(AgentError::DelegationDepthExceeded);
        }
        let entry = self
            .agents
            .get(agent_id)
            .ok_or_else(|| AgentError::AgentNotFound(agent_id.to_owned()))?;
        let agent = Arc::clone(&entry.agent);
        let allowed: BTreeSet<CapabilityId> = entry.manifest.capabilities.iter().cloned().collect();
        delegation_stack.push(agent_id.to_owned());
        let context = AgentContext {
            agent_id: agent_id.to_owned(),
            runtime,
            agent_runtime: self,
            allowed_capabilities: &allowed,
            execution,
            correlation_id: task.correlation_id.clone(),
            delegation_stack,
        };
        let response = match agent.execute(task, &context) {
            Ok(response) => response,
            Err(error) => {
                execution.check_cancelled().map_err(map_lifecycle_error)?;
                return Err(error);
            }
        };
        execution.check_cancelled().map_err(map_lifecycle_error)?;
        if response.output.len() > self.config.max_task_output_bytes {
            return Err(AgentError::OutputTooLarge);
        }
        Ok(response)
    }
}

fn valid_agent_id(id: &str) -> bool {
    !id.is_empty()
        && id.len() <= 128
        && id.split('.').all(|part| {
            !part.is_empty()
                && part
                    .chars()
                    .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-' || c == '_')
        })
}

fn map_lifecycle_error(error: lmlmxtend_api::ExecutionControlError) -> AgentError {
    match error {
        lmlmxtend_api::ExecutionControlError::Cancelled => AgentError::Cancelled,
        lmlmxtend_api::ExecutionControlError::DeadlineExceeded => AgentError::DeadlineExceeded,
        lmlmxtend_api::ExecutionControlError::OutputTooLarge => AgentError::OutputTooLarge,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use lmlmxtend_api::{
        CancellationToken, Extension, ExtensionError, ExtensionManifest, ExtensionRequest,
    };
    use lmlmxtend_core::RuntimeConfig;
    use std::sync::Arc;

    struct EchoAgent {
        id: &'static str,
    }

    impl Agent for EchoAgent {
        fn manifest(&self) -> AgentManifest {
            AgentManifest {
                id: self.id.into(),
                version: "1.0.0".into(),
                api_version: AGENT_API_VERSION,
                capabilities: vec![],
            }
        }

        fn execute(
            &self,
            task: AgentTask,
            _context: &AgentContext<'_>,
        ) -> Result<AgentResponse, AgentError> {
            Ok(AgentResponse::text(task.input))
        }
    }

    struct DelegatingAgent;

    impl Agent for DelegatingAgent {
        fn manifest(&self) -> AgentManifest {
            AgentManifest {
                id: "agent.parent".into(),
                version: "1.0.0".into(),
                api_version: AGENT_API_VERSION,
                capabilities: vec![],
            }
        }

        fn execute(
            &self,
            task: AgentTask,
            context: &AgentContext<'_>,
        ) -> Result<AgentResponse, AgentError> {
            context.delegate("agent.echo", AgentTask::new(task.task_id, task.input))
        }
    }

    struct CorrelationAgent;

    impl Agent for CorrelationAgent {
        fn manifest(&self) -> AgentManifest {
            AgentManifest {
                id: "agent.correlation".into(),
                version: "1.0.0".into(),
                api_version: AGENT_API_VERSION,
                capabilities: vec![],
            }
        }

        fn execute(
            &self,
            task: AgentTask,
            _context: &AgentContext<'_>,
        ) -> Result<AgentResponse, AgentError> {
            Ok(AgentResponse::text(task.correlation_id.unwrap_or_default()))
        }
    }

    struct CorrelationParent;

    impl Agent for CorrelationParent {
        fn manifest(&self) -> AgentManifest {
            AgentManifest {
                id: "agent.correlation-parent".into(),
                version: "1.0.0".into(),
                api_version: AGENT_API_VERSION,
                capabilities: vec![],
            }
        }

        fn execute(
            &self,
            task: AgentTask,
            context: &AgentContext<'_>,
        ) -> Result<AgentResponse, AgentError> {
            context.delegate(
                "agent.correlation",
                AgentTask::new(task.task_id, task.input),
            )
        }
    }

    struct LoopAgent;

    impl Agent for LoopAgent {
        fn manifest(&self) -> AgentManifest {
            AgentManifest {
                id: "agent.loop".into(),
                version: "1.0.0".into(),
                api_version: AGENT_API_VERSION,
                capabilities: vec![],
            }
        }

        fn execute(
            &self,
            task: AgentTask,
            context: &AgentContext<'_>,
        ) -> Result<AgentResponse, AgentError> {
            context.delegate("agent.loop", task)
        }
    }

    struct TextExtension;

    impl Extension for TextExtension {
        fn manifest(&self) -> ExtensionManifest {
            ExtensionManifest {
                id: "extension.text".into(),
                version: "1.0.0".into(),
                api_version: lmlmxtend_api::API_VERSION,
                capabilities: vec![CapabilityId::new("text.transform").unwrap()],
            }
        }

        fn execute(&self, request: ExtensionRequest) -> Result<ExtensionResponse, ExtensionError> {
            Ok(ExtensionResponse::text(request.input.to_uppercase()))
        }
    }

    struct CapabilityAgent;

    impl Agent for CapabilityAgent {
        fn manifest(&self) -> AgentManifest {
            AgentManifest {
                id: "agent.writer".into(),
                version: "1.0.0".into(),
                api_version: AGENT_API_VERSION,
                capabilities: vec![CapabilityId::new("text.transform").unwrap()],
            }
        }

        fn execute(
            &self,
            task: AgentTask,
            context: &AgentContext<'_>,
        ) -> Result<AgentResponse, AgentError> {
            let result = context
                .execute_capability(CapabilityId::new("text.transform").unwrap(), task.input)?;
            Ok(AgentResponse::text(result.output))
        }
    }

    fn execution() -> ExecutionContext {
        ExecutionContext::new(CancellationToken::new(), None, usize::MAX)
    }

    fn empty_runtime() -> Runtime {
        Runtime::new(RuntimeConfig::default())
    }

    #[test]
    fn registers_and_runs_agent() {
        let mut agents = AgentRuntime::default();
        agents
            .register(Arc::new(EchoAgent { id: "agent.echo" }))
            .unwrap();
        let result = agents
            .execute(
                &empty_runtime(),
                "agent.echo",
                AgentTask::new("t1", "hello"),
                &execution(),
            )
            .unwrap();
        assert_eq!(result.output, "hello");
        assert_eq!(agents.agent_count(), 1);
    }

    #[test]
    fn delegates_to_registered_agent() {
        let mut agents = AgentRuntime::default();
        agents
            .register(Arc::new(EchoAgent { id: "agent.echo" }))
            .unwrap();
        agents.register(Arc::new(DelegatingAgent)).unwrap();
        let result = agents
            .execute(
                &empty_runtime(),
                "agent.parent",
                AgentTask::new("t2", "delegated"),
                &execution(),
            )
            .unwrap();
        assert_eq!(result.output, "delegated");
    }

    #[test]
    fn propagates_correlation_id_through_delegation() {
        let mut agents = AgentRuntime::default();
        agents.register(Arc::new(CorrelationAgent)).unwrap();
        agents.register(Arc::new(CorrelationParent)).unwrap();
        let task = AgentTask::new("t-correlation", "payload").with_correlation_id("corr-42");
        let result = agents
            .execute(
                &empty_runtime(),
                "agent.correlation-parent",
                task,
                &execution(),
            )
            .unwrap();
        assert_eq!(result.output, "corr-42");
    }

    #[test]
    fn rejects_invalid_agent_identity() {
        let mut agents = AgentRuntime::default();
        assert_eq!(
            agents.register(Arc::new(EchoAgent {
                id: "Invalid Agent"
            })),
            Err(AgentError::InvalidManifest(
                "id must be a lowercase namespaced identifier and version is required".into()
            ))
        );
    }

    #[test]
    fn rejects_delegation_cycles() {
        let mut agents = AgentRuntime::default();
        agents.register(Arc::new(LoopAgent)).unwrap();
        assert_eq!(
            agents.execute(
                &empty_runtime(),
                "agent.loop",
                AgentTask::new("t3", "loop"),
                &execution()
            ),
            Err(AgentError::DelegationCycle("agent.loop".into()))
        );
    }

    #[test]
    fn enforces_delegation_depth_limit() {
        let mut agents = AgentRuntime::new(AgentRuntimeConfig {
            max_task_input_bytes: 1024,
            max_task_output_bytes: 1024,
            max_delegation_depth: 0,
        });
        agents
            .register(Arc::new(EchoAgent { id: "agent.echo" }))
            .unwrap();
        agents.register(Arc::new(DelegatingAgent)).unwrap();
        assert_eq!(
            agents.execute(
                &empty_runtime(),
                "agent.parent",
                AgentTask::new("t-depth", "hello"),
                &execution()
            ),
            Err(AgentError::DelegationDepthExceeded)
        );
    }

    #[test]
    fn enforces_declared_capabilities_for_agent() {
        let mut agents = AgentRuntime::default();
        agents.register(Arc::new(CapabilityAgent)).unwrap();
        let mut runtime = empty_runtime();
        runtime.register(Arc::new(TextExtension)).unwrap();
        let result = agents
            .execute(
                &runtime,
                "agent.writer",
                AgentTask::new("t4", "hello"),
                &execution(),
            )
            .unwrap();
        assert_eq!(result.output, "HELLO");
    }

    #[test]
    fn rejects_undeclared_capability() {
        struct Unprivileged;
        impl Agent for Unprivileged {
            fn manifest(&self) -> AgentManifest {
                AgentManifest {
                    id: "agent.unprivileged".into(),
                    version: "1.0.0".into(),
                    api_version: AGENT_API_VERSION,
                    capabilities: vec![],
                }
            }
            fn execute(
                &self,
                _task: AgentTask,
                context: &AgentContext<'_>,
            ) -> Result<AgentResponse, AgentError> {
                context
                    .execute_capability(CapabilityId::new("text.transform").unwrap(), "no")
                    .map(|response| AgentResponse::text(response.output))
            }
        }
        let mut agents = AgentRuntime::default();
        agents.register(Arc::new(Unprivileged)).unwrap();
        assert_eq!(
            agents.execute(
                &empty_runtime(),
                "agent.unprivileged",
                AgentTask::new("t5", "x"),
                &execution()
            ),
            Err(AgentError::CapabilityDenied("text.transform".into()))
        );
    }

    #[test]
    fn enforces_task_input_and_output_limits() {
        let mut agents = AgentRuntime::new(AgentRuntimeConfig {
            max_task_input_bytes: 3,
            max_task_output_bytes: 2,
            max_delegation_depth: 1,
        });
        agents
            .register(Arc::new(EchoAgent { id: "agent.echo" }))
            .unwrap();
        assert_eq!(
            agents.execute(
                &empty_runtime(),
                "agent.echo",
                AgentTask::new("t6", "long"),
                &execution()
            ),
            Err(AgentError::InputTooLarge)
        );
        assert_eq!(
            agents.execute(
                &empty_runtime(),
                "agent.echo",
                AgentTask::new("t7", "abc"),
                &execution()
            ),
            Err(AgentError::OutputTooLarge)
        );
    }

    #[test]
    fn rejects_cancelled_tasks_before_agent_execution() {
        let mut agents = AgentRuntime::default();
        agents
            .register(Arc::new(EchoAgent { id: "agent.echo" }))
            .unwrap();
        let token = CancellationToken::new();
        token.cancel();
        let context = ExecutionContext::new(token, None, usize::MAX);
        assert_eq!(
            agents.execute(
                &empty_runtime(),
                "agent.echo",
                AgentTask::new("t8", "hello"),
                &context
            ),
            Err(AgentError::Cancelled)
        );
    }
}

mod orchestration;
pub use orchestration::{AgentPlan, PlanError, PlanNode, PlanResult, MAX_PLAN_NODES};
