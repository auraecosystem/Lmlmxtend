# Roadmap

## Milestone 0 — Foundation (current)

- [x] Rust workspace and crate boundaries.
- [x] Versioned extension contract.
- [x] Capability identifiers and manifests.
- [x] In-process registry and dispatcher.
- [x] Basic policy checks and unit tests.
- [x] Architecture and security documentation.

## Milestone 1 — Runtime hardening

- [x] Structured runtime errors with Display/Error implementations.
- [ ] Tracing hooks.
- [ ] Cancellation, deadlines, and resource accounting.
- [x] Read-only extension health reporting.
- [ ] Property tests and fuzz targets.
- [ ] CI for formatting, linting, and workspace tests.

## Milestone 2 — LMLM processor bridge

- [ ] Agree a stable processor-facing contract with the LMLM repository.
- [ ] Local IPC/in-process adapter selected based on LMLM's actual runtime.
- [ ] Capability negotiation and version compatibility.
- [ ] No assumptions about model internals or weights.

## Milestone 3 — Intelligence and multimodal services

- [ ] Task orchestration and agent lifecycle.
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
