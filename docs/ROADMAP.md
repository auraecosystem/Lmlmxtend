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

## Milestone 2 — LMLM Processor v1 integration

- [x] Agree and document the stable `lmlm.processor.v1` JSON contract with the LMLM repository.
- [x] Implement the local loopback HTTP adapter and health handshake.
- [x] Validate protocol version, response envelope, and correlation/request ID.
- [x] Register the concrete adapter through the `lmlm.process` runtime capability.
- [x] Keep the LMLM model implementation and model assets in the LMLM repository.
- [ ] Add an automated cross-project end-to-end test that starts the LMLM service and uses a real installed model.
- [ ] Add a typed generation-parameter API and forward supported parameters through the Rust bridge.

**Milestone status:** the text-processing integration foundation is implemented. This does not mean the full multimodal processor is complete.

## Milestone 3 — Intelligence and multimodal services

- [x] Model-agnostic goal-planner contract and guarded proposal execution.
- [x] Deterministic capability discovery and task planning (single-provider registry).
- [ ] Multi-provider ranking and semantic task orchestration.
- [x] Concrete local LMLM Processor v1 adapter for text inference.
- [x] Initial synchronous agent lifecycle and nested task delegation with cycle/depth limits.
- [x] Deterministic DAG validation, dependency waves, and fail-fast agent-plan execution.
- [x] Bounded parallel execution (1–32 workers) with dependency barriers and panic/error handling.
- [x] Per-root-execution shared key/value state and bounded in-process agent mailboxes.
- [ ] Durable shared state, cross-process messaging, retries, rollback, and distributed scheduling.
- [ ] Typed image, audio, video, and spatial inference contracts with LMLM-native backend support.
- [ ] Streaming and end-to-end cancellation semantics.
- [ ] Memory interface with explicit ownership and retention controls.
- [ ] Device selection, batching, and inference resource accounting.

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

A milestone is complete only when implementation, tests, documentation, and compatibility checks are present. Planned items are not current capabilities. The processor v1 checklist above distinguishes the implemented text integration from the still-missing automated real-model end-to-end test and generation-parameter forwarding.
