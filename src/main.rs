use lmlmxtend_agent_runtime::{AgentPlan, AgentTask, PlanNode};

// Assume `agents`, `extension_runtime`, and `execution_context` are configured.
let mut plan = AgentPlan::new();
plan.add_node(PlanNode::new(
    "research",
    "agent.research",
    AgentTask::new("task-research", "Gather relevant facts"),
))?;
plan.add_node(
    PlanNode::new(
        "draft",
        "agent.writer",
        AgentTask::new("task-draft", "Write a concise draft"),
    )
    .after("research"),
)?;
let result = agents.execute_plan(&extension_runtime, &plan, &execution_context)?;
assert_eq!(result.execution_order, vec!["research", "draft"]);
