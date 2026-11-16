# Multi-Agent Runtime

The `lmlmxtend-agent-runtime` crate provides a local, synchronous agent registry
and orchestration layer. It is separate from the LMLM neural processor and
executes registered Rust agent implementations through explicit host contracts.

## Agent contract

Each agent registers an `AgentManifest` containing:
- a stable agent ID and version;
- the supported agent API version;
- the exact extension capabilities it is permitted to invoke.

The runtime snapshots this manifest during registration. Agent implementations
receive a structured `AgentContext`. Its provided capability-execution method
checks each requested capability against the registered manifest, and the context
also exposes lifecycle controls and bounded delegation to another registered agent.

## Delegation and permissions

- Capability calls are checked against the caller's registered manifest before
  being sent through the intelligence router and extension runtime.
- Delegated work must target a registered agent.
- The runtime rejects delegation cycles and enforces a configurable maximum
  delegation depth.
- Task input and returned output are bounded by host configuration.
- Cancellation and deadlines are propagated through nested calls.
- Explicit task and correlation IDs are carried by the task contract.

## Example

```rust
use lmlmxtend_agent_runtime::{
    Agent, AgentContext, AgentManifest, AgentResponse, AgentRuntime, AgentTask,
    AGENT_API_VERSION,
};
use lmlmxtend_api::ExecutionContext;
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
    ) -> Result<AgentResponse, lmlmxtend_agent_runtime::AgentError> {
        Ok(AgentResponse::text(task.input))
    }
}

use lmlmxtend_core::RuntimeConfig;
use lmlmxtend_runtime::Runtime;

let mut agents = AgentRuntime::default();
agents.register(Arc::new(EchoAgent))?;
let extension_runtime = Runtime::new(RuntimeConfig::default());
let result = agents.execute(
    &extension_runtime,
    "agent.echo",
    AgentTask::new("task-001", "Hello from an agent"),
    &ExecutionContext::default(),
)?;
```

## Current boundaries

- Agents are synchronous, in-process Rust implementations.
- Delegation is nested and synchronous; this is not yet a distributed message bus,
  parallel scheduler, or durable workflow engine.
- Agent manifests express capability grants but do not sandbox trusted Rust code.
  An agent can still use any ambient operating-system authority available to the
  host process. Do not register untrusted agent code.
- Memory stores, persistent task state, independent worker processes, WASM
  isolation, resource accounting, and forced termination are future work.
- The runtime does not infer goals or implement model reasoning. LMLM remains the
  neural intelligence processor; agents coordinate bounded tasks around it.
