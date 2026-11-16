# Security model

## Current guarantees

The initial runtime provides:
- capability declaration and ownership checks;
- optional capability allow-lists;
- a configurable maximum request-input size;
- a configurable maximum response-output size, checked by the host;
- request cancellation tokens and monotonic deadlines, with cooperative extension support;
- manifest API-version validation;
- duplicate extension and capability rejection;
- agent identity/version validation, per-agent capability grants, bounded task payloads, cancellation/deadline propagation, and delegation cycle/depth checks.
- Rust unsafe-code prohibition at workspace level.

## Important limitation

The current in-process extension trait is **not a security sandbox**. Registered extension code executes inside the host process with the host process's operating-system permissions. Do not load untrusted extensions into this runtime.

## Agent-specific limitation

Agent capability grants are enforced by the host API when an agent uses the provided context. They are not a sandbox: an in-process Rust agent can bypass the context and use any operating-system permissions available to the host. Nested delegation is synchronous and bounded, but it does not provide process isolation, parallel scheduling, or forced termination. Only register trusted agent implementations until an isolated worker boundary exists.

## Required before third-party extensions

1. Process or WASM sandbox with explicit filesystem, network, and device permissions.
2. Per-extension memory, CPU, wall-clock, and output limits.
3. Strong cancellation for isolated workers and bounded shutdown semantics.
4. Signed package metadata and verified content hashes.
5. Dependency and provenance policy.
6. Secret isolation; extensions receive scoped handles, never ambient credentials.
7. Audit events for installation, permission changes, and execution.
8. Fuzzing of manifest parsing and protocol boundaries.

## Safe defaults

- Local execution is the default.
- Network access is denied unless an adapter explicitly requests and receives permission.
- Application integrations are optional.
- Model/provider credentials are never embedded in extension manifests.
- Treat all extension input and output as untrusted data.

## Execution lifecycle limits

The current runtime checks cancellation and deadlines before dispatch and after an
extension returns. Extensions that override `execute_with_context` can poll the
shared cancellation token and deadline between bounded units of work. The default
compatibility implementation delegates to the legacy synchronous `execute`
method, so it cannot forcibly interrupt code that is already running.

The host enforces the configured output-byte cap after execution. This does not
prevent an extension from allocating excessive memory before returning. CPU time,
resident memory, concurrency quotas, and forced termination require a future
process/WASM isolation layer and are not provided by this in-process runtime.
