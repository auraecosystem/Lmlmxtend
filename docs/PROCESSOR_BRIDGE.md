# LMLM Processor Bridge

## Purpose

The processor bridge defines a typed, local boundary between LMLMxtend and the LMLM neural processor. LMLM remains the authoritative model implementation. LMLMxtend owns extension discovery, capability policy, request lifecycle integration, and the adapter boundary; it does not own model weights or inference kernels.

## Current repository compatibility

The current `auraecosystem/lmlm` repository does not yet expose a verified, stable Rust processor crate or a documented Rust inference ABI. Its checked-in `Api.ts` describes a broad client surface, while the repository README describes a Python/local toolkit. Neither is sufficient evidence for a concrete Rust backend binding.

For that reason this milestone introduces an explicit Rust trait contract rather than guessing at an internal LMLM API:

- `LmlmProcessor`: backend contract implemented by LMLM or a host-owned adapter.
- `ProcessorRequest`: modality, opaque payload bytes, media type, optional prompt, and correlation ID.
- `ProcessorResponse`: opaque output bytes and declared media type.
- `ExecutionContext`: cooperative cancellation, deadline, and output-size controls.
- `ProcessorBridge`: an injectable adapter that implements the existing LMLMxtend `Extension` contract.

## Modality support

The typed bridge carries text, image, audio, video, and mixed-modality payloads without interpreting or rewriting their bytes. The concrete backend is responsible for decoding, preprocessing, model execution, and output encoding.

The existing extension contract is string-based. Its `lmlm.process` capability therefore provides a UTF-8 text path only. Binary and multimodal callers use `ProcessorBridge::process` directly until a versioned binary extension transport is designed.

## Host integration

1. Implement `LmlmProcessor` in the LMLM repository or in an integration crate owned by the host.
2. Inject that implementation with `ProcessorBridge::new(Arc::new(your_backend))`.
3. Register the bridge with `lmlmxtend_runtime::Runtime`.
4. Dispatch text requests through the `lmlm.process` capability, or call the typed bridge API for binary/multimodal requests.

No cloud model provider is required. No remote endpoint is assumed.

## Safety and limitations

- Request and response byte limits are checked by the bridge.
- Cancellation and deadlines are cooperative. A backend must check the supplied context during bounded work; Rust cannot safely force-stop an arbitrary in-process implementation.
- This adapter is not a process sandbox and does not enforce hard CPU, memory, GPU, or wall-clock quotas.
- The bridge does not claim to perform inference. Until LMLM publishes a stable backend implementation, a concrete processor must be supplied by the integrating host.
- The bridge is synchronous in this milestone. Streaming, async execution, tensor handles, zero-copy buffers, batching, and device selection require separately versioned contracts.

## Verification

Run:

```bash
cargo test --workspace
bash scripts/verify.sh
bash scripts/smoke.sh
```
