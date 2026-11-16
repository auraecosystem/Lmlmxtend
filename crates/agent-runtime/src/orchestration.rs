//! Deterministic task-graph planning and execution for registered agents.
//!
//! This first scheduler is deliberately sequential and fail-fast. Dependencies
//! define execution order; task inputs remain explicit and are not implicitly
//! rewritten with predecessor output.

use crate::{AgentError, AgentResponse, AgentRuntime, AgentTask};
use lmlmxtend_api::ExecutionContext;
use lmlmxtend_runtime::Runtime;
use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

/// Maximum number of nodes accepted in one in-memory plan.
pub const MAX_PLAN_NODES: usize = 256;

/// One agent invocation in a plan.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanNode {
    pub id: String,
    pub agent_id: String,
    pub task: AgentTask,
    /// Node IDs that must finish successfully before this node starts.
    pub depends_on: Vec<String>,
}

impl PlanNode {
    pub fn new(
        id: impl Into<String>,
        agent_id: impl Into<String>,
        task: AgentTask,
    ) -> Self {
        Self { id: id.into(), agent_id: agent_id.into(), task, depends_on: Vec::new() }
    }

    pub fn after(mut self, dependency: impl Into<String>) -> Self {
        self.depends_on.push(dependency.into());
        self
    }
}

/// A bounded directed acyclic graph of agent tasks.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct AgentPlan {
    nodes: Vec<PlanNode>,
}

impl AgentPlan {
    pub fn new() -> Self { Self::default() }

    pub fn add_node(&mut self, node: PlanNode) -> Result<(), PlanError> {
        if self.nodes.len() >= MAX_PLAN_NODES {
            return Err(PlanError::TooManyNodes);
        }
        if node.id.trim().is_empty() || node.agent_id.trim().is_empty() {
            return Err(PlanError::InvalidNode("node and agent IDs are required".into()));
        }
        if self.nodes.iter().any(|existing| existing.id == node.id) {
            return Err(PlanError::DuplicateNode(node.id));
        }
        self.nodes.push(node);
        Ok(())
    }

    pub fn nodes(&self) -> &[PlanNode] { &self.nodes }

    /// Validate references and cycles, returning a stable topological order.
    /// Among simultaneously ready nodes, lexical node ID order is used.
    pub fn execution_order(&self) -> Result<Vec<String>, PlanError> {
        if self.nodes.is_empty() {
            return Err(PlanError::EmptyPlan);
        }
        if self.nodes.len() > MAX_PLAN_NODES {
            return Err(PlanError::TooManyNodes);
        }

        let mut by_id = BTreeMap::new();
        for node in &self.nodes {
            if node.id.trim().is_empty() || node.agent_id.trim().is_empty() {
                return Err(PlanError::InvalidNode("node and agent IDs are required".into()));
            }
            if by_id.insert(node.id.as_str(), node).is_some() {
                return Err(PlanError::DuplicateNode(node.id.clone()));
            }
        }

        let mut indegree: BTreeMap<&str, usize> =
            by_id.keys().map(|id| (*id, 0)).collect();
        let mut outgoing: BTreeMap<&str, Vec<&str>> = BTreeMap::new();

        for node in &self.nodes {
            let mut unique_dependencies = BTreeSet::new();
            for dependency in &node.depends_on {
                if dependency == &node.id {
                    return Err(PlanError::SelfDependency(node.id.clone()));
                }
                if !unique_dependencies.insert(dependency.as_str()) {
                    return Err(PlanError::DuplicateDependency {
                        node_id: node.id.clone(),
                        dependency: dependency.clone(),
                    });
                }
                if !by_id.contains_key(dependency.as_str()) {
                    return Err(PlanError::MissingDependency {
                        node_id: node.id.clone(),
                        dependency: dependency.clone(),
                    });
                }
                *indegree.get_mut(node.id.as_str()).expect("node was indexed") += 1;
                outgoing.entry(dependency.as_str()).or_default().push(node.id.as_str());
            }
        }

        let mut ready: BTreeSet<&str> = indegree.iter()
            .filter_map(|(id, degree)| (*degree == 0).then_some(*id))
            .collect();
        let mut order = Vec::with_capacity(self.nodes.len());

        while let Some(id) = ready.iter().next().copied() {
            ready.remove(id);
            order.push(id.to_owned());
            if let Some(children) = outgoing.get(id) {
                for child in children {
                    let degree = indegree.get_mut(child).expect("child was indexed");
                    *degree -= 1;
                    if *degree == 0 {
                        ready.insert(child);
                    }
                }
            }
        }

        if order.len() != self.nodes.len() {
            return Err(PlanError::CycleDetected);
        }
        Ok(order)
    }
}

/// Validated outputs keyed by plan-node ID, plus the actual execution order.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanResult {
    pub execution_order: Vec<String>,
    pub outputs: BTreeMap<String, AgentResponse>,
}

/// Validation or execution failure for an agent plan.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum PlanError {
    EmptyPlan,
    TooManyNodes,
    InvalidNode(String),
    DuplicateNode(String),
    SelfDependency(String),
    DuplicateDependency { node_id: String, dependency: String },
    MissingDependency { node_id: String, dependency: String },
    CycleDetected,
    AgentFailed { node_id: String, source: AgentError },
}

impl fmt::Display for PlanError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyPlan => f.write_str("agent plan is empty"),
            Self::TooManyNodes => write!(f, "agent plan exceeds the {MAX_PLAN_NODES}-node limit"),
            Self::InvalidNode(reason) => write!(f, "invalid plan node: {reason}"),
            Self::DuplicateNode(id) => write!(f, "duplicate plan node: {id}"),
            Self::SelfDependency(id) => write!(f, "plan node '{id}' depends on itself"),
            Self::DuplicateDependency { node_id, dependency } =>
                write!(f, "plan node '{node_id}' repeats dependency '{dependency}'"),
            Self::MissingDependency { node_id, dependency } =>
                write!(f, "plan node '{node_id}' references missing dependency '{dependency}'"),
            Self::CycleDetected => f.write_str("agent plan contains a dependency cycle"),
            Self::AgentFailed { node_id, source } =>
                write!(f, "agent task '{node_id}' failed: {source}"),
        }
    }
}

impl std::error::Error for PlanError {}

impl AgentRuntime {
    /// Validate the complete plan before invoking any agent, then execute its
    /// nodes sequentially in deterministic topological order. Stops on first
    /// failure; successful earlier node outputs are returned only on success.
    pub fn execute_plan(
        &self,
        runtime: &Runtime,
        plan: &AgentPlan,
        execution: &ExecutionContext,
    ) -> Result<PlanResult, PlanError> {
        let order = plan.execution_order()?;
        let nodes: BTreeMap<&str, &PlanNode> =
            plan.nodes().iter().map(|node| (node.id.as_str(), node)).collect();
        let mut outputs = BTreeMap::new();

        for node_id in &order {
            let node = nodes.get(node_id.as_str()).expect("validated node exists");
            let response = self.execute(
                runtime,
                &node.agent_id,
                node.task.clone(),
                execution,
            ).map_err(|source| PlanError::AgentFailed {
                node_id: node.id.clone(),
                source,
            })?;
            outputs.insert(node.id.clone(), response);
        }

        Ok(PlanResult { execution_order: order, outputs })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Agent, AgentContext, AgentManifest, AgentResponse, AGENT_API_VERSION};
    use lmlmxtend_api::{CancellationToken, ExecutionContext};
    use lmlmxtend_core::RuntimeConfig;
    use std::sync::Arc;

    struct EchoAgent;

    impl Agent for EchoAgent {
        fn manifest(&self) -> AgentManifest {
            AgentManifest {
                id: "agent.echo".into(),
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

    fn context() -> ExecutionContext {
        ExecutionContext::new(CancellationToken::new(), None, usize::MAX)
    }

    #[test]
    fn produces_stable_topological_order() {
        let mut plan = AgentPlan::new();
        plan.add_node(PlanNode::new("c", "agent.echo", AgentTask::new("c", "C")).after("a")).unwrap();
        plan.add_node(PlanNode::new("b", "agent.echo", AgentTask::new("b", "B"))).unwrap();
        plan.add_node(PlanNode::new("a", "agent.echo", AgentTask::new("a", "A"))).unwrap();
        assert_eq!(plan.execution_order().unwrap(), vec!["a", "b", "c"]);
    }

    #[test]
    fn rejects_missing_dependency_before_execution() {
        let mut plan = AgentPlan::new();
        plan.add_node(PlanNode::new("child", "agent.echo", AgentTask::new("t", "x")).after("missing")).unwrap();
        assert!(matches!(
            plan.execution_order(),
            Err(PlanError::MissingDependency { .. })
        ));
    }

    #[test]
    fn rejects_dependency_cycles() {
        let mut plan = AgentPlan::new();
        plan.add_node(PlanNode::new("a", "agent.echo", AgentTask::new("a", "A")).after("b")).unwrap();
        plan.add_node(PlanNode::new("b", "agent.echo", AgentTask::new("b", "B")).after("a")).unwrap();
        assert_eq!(plan.execution_order(), Err(PlanError::CycleDetected));
    }

    #[test]
    fn executes_plan_and_collects_node_outputs() {
        let mut agents = AgentRuntime::default();
        agents.register(Arc::new(EchoAgent)).unwrap();
        let mut plan = AgentPlan::new();
        plan.add_node(PlanNode::new("second", "agent.echo", AgentTask::new("t2", "two")).after("first")).unwrap();
        plan.add_node(PlanNode::new("first", "agent.echo", AgentTask::new("t1", "one"))).unwrap();

        let runtime = Runtime::new(RuntimeConfig::default());
        let result = agents.execute_plan(&runtime, &plan, &context()).unwrap();
        assert_eq!(result.execution_order, vec!["first", "second"]);
        assert_eq!(result.outputs["first"].output, "one");
        assert_eq!(result.outputs["second"].output, "two");
    }

    #[test]
    fn rejects_empty_plan() {
        assert_eq!(AgentPlan::new().execution_order(), Err(PlanError::EmptyPlan));
    }
}
