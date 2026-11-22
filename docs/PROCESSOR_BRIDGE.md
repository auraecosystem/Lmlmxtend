# LMLM Processor Bridge

## Purpose

The processor bridge defines a typed, local boundary between LMLMxtend and the LMLM neural processor. LMLM remains the authoritative model implementation. LMLMxtend owns extension discovery, capability policy, request lifecycle integration, and the adapter boundary; it does not own model weights or inference kernels.

## Versioned LMLM integration

The concrete cross-repository integration is **LMLM Processor v1**:

- LMLM exposes a loopback-only HTTP service at `127.0.0.1:8765`.
- LMLMxtend's `HttpLmlmProcessor` speaks the `lmlm.processor.v1` JSON contract.
- The Rust client accepts a `SocketAddr` and rejects non-loopback addresses; it cannot be configured with a remote host name or cloud URL.
- The wire protocol preserves payload bytes as hexadecimal data and carries request IDs, modality, content type, and optional prompt.

See the LMLM repository's `docs/PROCESSOR_V1.md` for the service setup and wire schema.

## Host integration

1. Start the local inference process with the desired local model, bound to loopback.
2. Start the LMLM Processor v1 service in the LMLM repository.
3. Construct `HttpLmlmProcessor::new("127.0.0.1:8765".parse()?)`.
4. Inject it into `ProcessorBridge::new(Arc::new(processor))` and register the bridge with `lmlmxtend_runtime::Runtime`.
5. Dispatch requests through the `lmlm.process` capability.

Runnable example: `cargo run -p lmlmxtend-processor-bridge --example local_lmlm_http`.

## Modality support

The typed Rust bridge and v1 wire schema carry text, image, audio, video, and mixed-modality payloads. The initial LMLM local inference adapter currently executes **text only**. Other modalities are rejected by the Python backend until modality-specific LMLM preprocessing and inference are implemented. The existing generic Extension contract is string-based; binary callers use `ProcessorBridge::process` directly.

## Safety and limitations

- The LMLM service binds only to loopback and the Rust client rejects non-loopback addresses.
- The service calls only a loopback OpenAI-compatible inference endpoint; it does not download weights or fall back to a remote provider.
- The local model runtime and model weights must already be installed by the operator.
- Request/response payload limits are enforced at the bridge and service boundaries.
- Transport timeouts are bounded (default 120 seconds). Cancellation is checked before and after the HTTP call; an in-flight synchronous HTTP request is not instantly interruptible.
- This is not a process sandbox and does not enforce hard CPU, memory, GPU, or wall-clock quotas.
- Streaming, batching, device selection, and binary modality-specific inference remain future contracts.

## Verification

Run:

```bash
cargo test --workspace
bash scripts/verify.sh
bash scripts/smoke.sh
``
