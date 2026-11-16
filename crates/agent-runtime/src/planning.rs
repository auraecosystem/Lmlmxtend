//! Model-agnostic goal planning and guarded execution.
//!
//! A processor adapter implements `GoalPlanner` and proposes an `AgentPlan`.
//! The host remains responsible for validating and executing that plan through
//! the registered-agent runtime. This module does not invoke a model itself.

use crate::{AgentDescriptor, AgentPlan, AgentRuntime, PlanError, PlanResult, MAX_PARALLEL_TASKS};
use lmlmxtend_api::ExecutionContext;
use lmlmxtend_runtime::Runtime;
use std::fmt;

/// Maximum UTF-8 byte length accepted for a user goal.
pub const MAX_GOAL_BYTES: usize = 64 * 1024;
/// Maximum aggregate UTF-8 payload and metadata size accepted in a proposed goal plan.
pub const MAX_GOAL_PLAN_BYTES: usize = 8 * 1024 * 1024;

/// A bounded natural-language goal submitted to a planner.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GoalRequest {
    pub goal_id: String,
    pub goal: String,
}

impl GoalRequest {
    pub fn new(
        goal_id: impl Into<String>,
        goal: impl Into<String>,
    ) -> Result<Self, GoalPlanningError> {
        let goal_id = goal_id.into();
        let goal = goal.into();
        if goal_id.trim().is_empty() || goal_id.len() > 128 {
            return Err(GoalPlanningError::InvalidRequest(
                "goal ID must contain 1–128 bytes".into(),
            ));
        }
        if goal.trim().is_empty() {
            return Err(GoalPlanningError::InvalidRequest(
                "goal text is required".into(),
            ));
        }
        if goal.len() > MAX_GOAL_BYTES {
            return Err(GoalPlanningError::GoalTooLarge);
        }
        Ok(Self { goal_id, goal })
    }
}

/// A processor-facing contract. Implementations may use an LMLM processor,
/// but must return a structured proposal rather than execute agents directly.
pub trait GoalPlanner: Send + Sync {
    fn propose(
        &self,
        request: &GoalRequest,
        available_agents: &[AgentDescriptor],
    ) -> Result<AgentPlan, GoalPlanningError>;
}

/// Errors produced while validating a goal or obtaining a proposal.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum GoalPlanningError {
    InvalidRequest(String),
    GoalTooLarge,
    PlanTooLarge,
    PlannerFailed(String),
}

impl fmt::Display for GoalPlanningError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidRequest(reason) => write!(f, "invalid goal request: {reason}"),
            Self::GoalTooLarge => write!(f, "goal exceeds the {MAX_GOAL_BYTES}-byte limit"),
            Self::PlanTooLarge => write!(f, "proposed plan exceeds the {MAX_GOAL_PLAN_BYTES}-byte limit"),
            Self::PlannerFailed(reason) => write!(f, "goal planner failed: {reason}"),
        }
    }
}

impl std::error::Error for GoalPlanningError {}

/// A goal together with the validated execution result.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GoalResult {
    pub goal_id: String,
    pub plan: AgentPlan,
    pub execution: PlanResult,
}

/// Planning or execution failure. A proposed plan is never trusted merely
/// because it came from a model adapter.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum GoalExecutionError {
    Planning(GoalPlanningError),
    InvalidConcurrency,
    Execution(PlanError),
}

impl fmt::Display for GoalExecutionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Planning(error) => write!(f, "{error}"),
            Self::InvalidConcurrency => write!(
                f,
                "parallel task limit must be between 1 and {MAX_PARALLEL_TASKS}"
            ),
            Self::Execution(error) => write!(f, "{error}"),
        }
    }
}

impl std::error::Error for GoalExecutionError {}

impl AgentRuntime {
    /// Ask a planner for a proposal, then validate and execute it using the
    /// same bounded scheduler and registered-agent checks as host-authored plans.
    pub fn execute_goal<P: GoalPlanner>(
        &self,
        runtime: &Runtime,
        planner: &P,
        request: &GoalRequest,
        execution: &ExecutionContext,
        max_parallel_tasks: usize,
    ) -> Result<GoalResult, GoalExecutionError> {
        if max_parallel_tasks == 0 || max_parallel_tasks > MAX_PARALLEL_TASKS {
            return Err(GoalExecutionError::InvalidConcurrency);
        }

        let available_agents = self.agents();
        let plan = planner
            .propose(request, &available_agents)
            .map_err(GoalExecutionError::Planning)?;
        let plan_bytes = plan.nodes().iter().fold(0usize, |total, node| {
            let dependency_bytes = node.depends_on.iter().fold(0usize, |sum, id| sum.saturating_add(id.len()));
            total
                .saturating_add(node.id.len())
                .saturating_add(node.agent_id.len())
                .saturating_add(node.task.task_id.len())
                .saturating_add(node.task.input.len())
                .saturating_add(node.task.correlation_id.as_ref().map_or(0, String::len))
                .saturating_add(dependency_bytes)
        });
        if plan_bytes > MAX_GOAL_PLAN_BYTES {
            return Err(GoalExecutionError::Planning(GoalPlanningError::PlanTooLarge));
        }

        let result = self
            .execute_plan_parallel(runtime, &plan, execution, max_parallel_tasks)
            .map_err(GoalExecutionError::Execution)?;

        Ok(GoalResult {
            goal_id: request.goal_id.clone(),
            plan,
            execution: result,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        Agent, AgentContext, AgentError, AgentManifest, AgentResponse, AgentTask, AGENT_API_VERSION,
    };
    use lmlmxtend_api::{CancellationToken, CapabilityId};
    use lmlmxtend_core::RuntimeConfig;
    use std::sync::Arc;

    struct EchoAgent;

    impl Agent for EchoAgent {
        fn manifest(&self) -> AgentManifest {
            AgentManifest {
                id: "agent.echo".into(),
                version: "1.0.0".into(),
                api_version: AGENT_API_VERSION,
                capabilities: vec![CapabilityId::new("text.echo").unwrap()],
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

    struct EchoPlanner;

    impl GoalPlanner for EchoPlanner {
        fn propose(
            &self,
            request: &GoalRequest,
            agents: &[AgentDescriptor],
        ) -> Result<AgentPlan, GoalPlanningError> {
            assert_eq!(agents.len(), 1);
            assert_eq!(agents[0].id, "agent.echo");
            let mut plan = AgentPlan::new();
            plan.add_node(crate::PlanNode::new(
                "execute-goal",
                "agent.echo",
                AgentTask::new(request.goal_id.clone(), request.goal.clone()),
            ))
            .map_err(|error| GoalPlanningError::PlannerFailed(error.to_string()))?;
            Ok(plan)
        }
    }

    fn execution_context() -> ExecutionContext {
        ExecutionContext::new(CancellationToken::new(), None, usize::MAX)
    }

    #[test]
    fn executes_planner_proposal_through_registered_agent_runtime() {
        let mut agents = AgentRuntime::default();
        agents.register(Arc::new(EchoAgent)).unwrap();
        let runtime = Runtime::new(RuntimeConfig::default());
        let request = GoalRequest::new("goal-1", "summarize this").unwrap();

        let result = agents
            .execute_goal(&runtime, &EchoPlanner, &request, &execution_context(), 2)
            .unwrap();

        assert_eq!(result.goal_id, "goal-1");
        assert_eq!(
            result.execution.outputs["execute-goal"].output,
            "summarize this"
        );
    }

    #[test]
    fn rejects_invalid_goal_requests() {
        assert!(matches!(
            GoalRequest::new(" ", "goal"),
            Err(GoalPlanningError::InvalidRequest(_))
        ));
        assert_eq!(
            GoalRequest::new("goal-1", "x".repeat(MAX_GOAL_BYTES + 1)),
            Err(GoalPlanningError::GoalTooLarge)
        );
    }

    #[test]
    fn validates_planner_proposal_before_invoking_agents() {
        struct InvalidPlanner;
        impl GoalPlanner for InvalidPlanner {
            fn propose(
                &self,
                _request: &GoalRequest,
                _agents: &[AgentDescriptor],
            ) -> Result<AgentPlan, GoalPlanningError> {
                let mut plan = AgentPlan::new();
                plan.add_node(crate::PlanNode::new(
                    "untrusted-node",
                    "agent.not-registered",
                    AgentTask::new("task-1", "must not run"),
                ))
                .unwrap();
                Ok(plan)
            }
        }

        let agents = AgentRuntime::default();
        let runtime = Runtime::new(RuntimeConfig::default());
        let request = GoalRequest::new("goal-2", "try invalid route").unwrap();
        assert!(matches!(
            agents.execute_goal(&runtime, &InvalidPlanner, &request, &execution_context(), 1),
            Err(GoalExecutionError::Execution(
                PlanError::AgentNotRegistered { .. }
            ))
        ));
    }

    #[test]
    fn rejects_oversized_planner_proposal_before_agent_execution() {
        struct OversizedPlanner;
        impl GoalPlanner for OversizedPlanner {
            fn propose(
                &self,
                _request: &GoalRequest,
                _agents: &[AgentDescriptor],
            ) -> Result<AgentPlan, GoalPlanningError> {
                let mut plan = AgentPlan::new();
                plan.add_node(crate::PlanNode::new(
                    "oversized",
                    "agent.echo",
                    AgentTask::new("task-oversized", "x".repeat(MAX_GOAL_PLAN_BYTES + 1)),
                ))
                .map_err(|error| GoalPlanningError::PlannerFailed(error.to_string()))?;
                Ok(plan)
            }
        }

        let mut agents = AgentRuntime::default();
        agents.register(Arc::new(EchoAgent)).unwrap();
        let runtime = Runtime::new(RuntimeConfig::default());
        let request = GoalRequest::new("goal-oversized", "run a bounded plan").unwrap();
        assert_eq!(
            agents.execute_goal(&runtime, &OversizedPlanner, &request, &execution_context(), 1),
            Err(GoalExecutionError::Planning(GoalPlanningError::PlanTooLarge))
        );
    }

    #[test]
    fn rejects_invalid_parallel_limit_before_planning() {
        struct NeverPlanner;
        impl GoalPlanner for NeverPlanner {
            fn propose(
                &self,
                _request: &GoalRequest,
                _agents: &[AgentDescriptor],
            ) -> Result<AgentPlan, GoalPlanningError> {
                panic!("planner must not run for invalid concurrency");
            }
        }

        let agents = AgentRuntime::default();
        let runtime = Runtime::new(RuntimeConfig::default());
        let request = GoalRequest::new("goal-3", "goal").unwrap();
        assert_eq!(
            agents.execute_goal(&runtime, &NeverPlanner, &request, &execution_context(), 0),
            Err(GoalExecutionError::InvalidConcurrency)
        );
    }
}
