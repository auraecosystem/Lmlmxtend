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
receive a structured `AgentContext`. Its capability-execution method checks
each requested capability against the registered manifest, and the context
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

## Deterministic task plans

`AgentPlan` describes a directed acyclic graph of `PlanNode` tasks. Each node
names a registered agent, carries an explicit `AgentTask`, and may list node IDs
that must complete before it can start.

```rust
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
```

Before invoking any agent, the scheduler validates duplicate IDs, missing
dependencies, repeated dependencies, self-dependencies, cycles, and the
256-node limit. It then runs nodes sequentially in stable topological order
(lexical node ID order among equally-ready nodes), collecting outputs by node
ID. A task's input is never silently rewritten with predecessor output; agents
must receive any required data through explicit task construction by the host.

Execution is fail-fast. A failed node stops the plan and is reported with its
node ID and underlying `AgentError`. There is no implicit retry, rollback,
parallel execution, or durable checkpointing.

## Current boundaries

- Agents are synchronous, in-process Rust implementations.
- Task-graph execution is deterministic and sequential; it is not a parallel
  scheduler, distributed message bus, or durable workflow engine.
- Agent manifests express capability grants but do not sandbox trusted Rust code.
  An agent can still use any ambient operating-system authority available to the
  host process. Do not register untrusted agent code.
- Memory stores, persistent task state, independent worker processes, WASM
  isolation, resource accounting, and forced termination are future work.
- The runtime does not infer goals or implement model reasoning. LMLM remains the
  neural intelligence processor; agents coordinate bounded tasks around it.
