# Roadmap

## Milestone 0 — Foundation (current)

- [x] Rust workspace and crate boundaries.
- [x] Versioned extension contract.
- [x] Capability identifiers and manifests.
- [x] In-process registry and dispatcher.
- [x] Basic policy checks and unit tests.
- [x] Architecture and security documentation.
- [x] Native Extension Forge CLI with safe scaffolding and starter tests.

## Milestone 1 — Runtime hardening

- [x] Structured runtime errors with Display/Error implementations.
- [ ] Tracing hooks.
- [x] Cooperative cancellation tokens and monotonic request deadlines.
- [x] Host-enforced response output-byte limits.
- [ ] CPU, memory, concurrency accounting, and forced termination (requires isolation).
- [x] Read-only extension health reporting.
- [ ] Property tests and fuzz targets.
- [x] CI for formatting, linting, and workspace tests.

- [x] Initial intelligence crate with capability discovery, API-version checks, explicit provider preference, and runtime-mediated dispatch.
- [x] Initial agent-runtime workspace boundary with permission-aware registration and bounded delegation.
- [ ] Complete target workspace boundaries for multimodal, execution, and security.

## Milestone 2 — LMLM processor bridge

- [ ] Agree a stable processor-facing contract with the LMLM repository.
- [ ] Local IPC/in-process adapter selected based on LMLM's actual runtime.
- [ ] Capability negotiation and version compatibility.
- [ ] No assumptions about model internals or weights.

## Milestone 3 — Intelligence and multimodal services

- [x] Deterministic capability discovery and task planning (single-provider registry).
- [ ] Multi-provider ranking and semantic task orchestration.
- [x] Initial synchronous agent lifecycle and nested task delegation with cycle/depth limits.
- [x] Deterministic DAG validation, dependency waves, and fail-fast agent-plan execution.
- [x] Bounded parallel execution (1–32 workers) with dependency barriers and panic/error handling.
- [x] Per-root-execution shared key/value state and bounded in-process agent mailboxes.
- [ ] Durable shared state, cross-process messaging, retries, rollback, and distributed scheduling.
- [ ] Parallel scheduling, durable task state, and inter-agent messaging.
- [ ] Typed image, audio, video, and spatial data contracts.
- [ ] Streaming and cancellation semantics.
- [ ] Memory interface with explicit ownership and retention controls.

## Milestone 4 — Application adapters and SDKs

- [ ] Olgin adapter, kept optional and isolated.
- [ ] General application adapter.
- [ ] Rust SDK stabilization.
- [ ] Python and TypeScript SDKs.
- [ ] Integration examples and compatibility matrix.

## Milestone 5 — Secure extension distribution

- [ ] Sandboxed WASM or process-based execution.
- [ ] Signed packages and integrity verification.
- [ ] Permission grants and revocation.
- [ ] Resource quotas and audit trail.

A milestone is complete only when implementation, tests, documentation, and compatibility checks are present. Planned items are not current capabilities.
