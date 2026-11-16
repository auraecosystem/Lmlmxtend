# Intelligence routing

The `lmlmxtend-intelligence` crate provides explicit, deterministic task planning
over capabilities registered with the local runtime. It is a coordination layer,
not a model implementation and not a replacement for the LMLM neural processor.

## Flow

1. An application creates a `TaskIntent` with a validated capability ID and input.
2. The router discovers registered providers for that exact capability.
3. The router checks the required extension API version.
4. An optional preferred extension ID is honored exactly; if unavailable, planning
   fails rather than silently choosing a different provider.
5. The resulting `TaskPlan` records the selected extension, version, capability,
   and selection reason.
6. Execution is delegated to the runtime, which still enforces capability policy,
   input/output limits, cancellation, and deadlines.

## Example

```rust
use lmlmxtend_api::CapabilityId;
use lmlmxtend_intelligence::{IntelligenceRouter, TaskIntent};
use lmlmxtend_runtime::Runtime;

let router = IntelligenceRouter;
let intent = TaskIntent::new(
    CapabilityId::new("creative.compose")?,
    "Create a scene outline",
);
let plan = router.plan(&runtime, &intent)?;
let response = router.execute(&runtime, intent)?;
```

The runtime currently enforces one registered owner per capability. Therefore,
discovery may return zero or one provider today. The routing API is deliberately
separated so future multi-provider selection can be introduced with explicit
ranking and policy without changing application task intent.

## Scope and limitations

- Discovery is local and read-only; it does not load packages from disk.
- Compatibility currently means exact extension API version matching.
- There is no semantic task inference, model invocation, automatic delegation,
  multi-agent negotiation, or remote provider discovery yet.
- Execution remains subject to the runtime's in-process trust boundary.
