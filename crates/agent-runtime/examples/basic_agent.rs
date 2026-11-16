use lmlmxtend_agent_runtime::{
    Agent, AgentContext, AgentError, AgentManifest, AgentResponse, AgentRuntime, AgentTask,
    AGENT_API_VERSION,
};
use lmlmxtend_api::ExecutionContext;
use lmlmxtend_core::RuntimeConfig;
use lmlmxtend_runtime::Runtime;
use std::sync::Arc;

struct GreetingAgent;

impl Agent for GreetingAgent {
    fn manifest(&self) -> AgentManifest {
        AgentManifest {
            id: "agent.greeting".into(),
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
        Ok(AgentResponse::text(format!(
            "{} says: Hello, {}!",
            context.agent_id(),
            task.input
        )))
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let extension_runtime = Runtime::new(RuntimeConfig::default());
    let mut agents = AgentRuntime::default();
    agents.register(Arc::new(GreetingAgent))?;

    let response = agents.execute(
        &extension_runtime,
        "agent.greeting",
        AgentTask::new("greeting-001", "LMLMxtend"),
        &ExecutionContext::default(),
    )?;

    println!("{}", response.output);
    Ok(())
}
