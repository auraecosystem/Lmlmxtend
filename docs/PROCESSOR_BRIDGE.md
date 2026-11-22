# LMLM Processor Bridge

## Purpose

The processor bridge is the versioned, local boundary between LMLMxtend and the LMLM neural processor. LMLM owns model assets, tokenization, inference, and neural execution. LMLMxtend owns the typed adapter, capability registration, request limits, and host-side lifecycle checks. It does not reimplement the model.

## Implemented integration: LMLM Processor v1

The current cross-project contract is:

- Protocol identifier: `lmlm.processor.v1`; protocol version: `1`.
- Service bind: IPv4 loopback only, `127.0.0.1:8765` by default.
- Health: `GET /v1/health`.
- Inference: `POST /v1/process`.
- Request envelope: `request_id`, `modality`, `content_type`, `payload_hex`, optional `prompt` and `parameters`.
- Response envelope: matching `request_id`, `content_type`, and `payload_hex`.
- Binary payloads are hex-encoded for lossless transport.

The Rust `HttpLmlmProcessor` accepts a `SocketAddr`, rejects non-loopback and IPv6 addresses, checks the v1 health response, validates the response protocol and request ID, and applies bounded connection/read/write timeouts. The bridge checks cancellation before and after the synchronous request. An in-flight socket operation is bounded by its timeout, not immediately interruptible by cancellation.

## LMLM service backends

The LMLM service currently has two local execution modes:

1. **`native_cpu` (recommended):** loads the LMLM model manifest and assets from `LMLM_MODEL_DIR`, then calls the in-process LMLM native CPU engine. No separate inference server is required.
2. **`local_openai` (compatibility):** forwards text generation to an already-running OpenAI-compatible inference endpoint on `127.0.0.1`. This is an adapter mode, not the LMLM model implementation.

Both modes are local-only. The service does not download model weights, call remote providers, or silently fall back to a cloud model.

### Start native CPU mode

From the LMLM repository root, prepare a model directory containing the files required by the LMLM native CPU inference format, then run:

```bash
LMLM_BACKEND=native_cpu \
LMLM_MODEL_DIR=./model \
LMLM_PROCESSOR_HOST=127.0.0.1 \
LMLM_PROCESSOR_PORT=8765 \
python -m lmlm_processor.server
```

### Start compatibility mode

```bash
LMLM_BACKEND=local_openai \
LMLM_INFERENCE_URL=http://127.0.0.1:8080/v1 \
LMLM_PROCESSOR_HOST=127.0.0.1 \
LMLM_PROCESSOR_PORT=8765 \
python -m lmlm_processor.server
```

The compatibility endpoint must provide `GET /health` and `POST /v1/chat/completions`. Its URL is restricted to plain HTTP on `127.0.0.1`; credentials, query strings, fragments, hostnames, and remote addresses are rejected.

## Connect from a Rust host

Start the LMLM service first, then run:

```bash
cargo run -p lmlmxtend-processor-bridge --example local_lmlm_http
```

The example constructs `HttpLmlmProcessor` for `127.0.0.1:8765`, performs a health check, injects it into `ProcessorBridge`, registers the bridge with `lmlmxtend_runtime::Runtime`, and dispatches the `lmlm.process` capability.

For an application host, the same composition can be embedded:

```rust
let processor = HttpLmlmProcessor::new("127.0.0.1:8765".parse()?)?;
processor.health_check()?;
let bridge = ProcessorBridge::new(Arc::new(processor));
runtime.register(Arc::new(bridge))?;
```

## Modality and parameter status

The Rust request type and wire protocol represent text, image, audio, video, and mixed-modality payloads. **The current LMLM backends execute text only.** Non-text requests are rejected by the backend; representing a modality in the envelope does not mean the model can process it.

The generic extension request is string-based. Binary or modality-specific callers must use `ProcessorBridge::process` directly. The Rust bridge now exposes `GenerationParameters` and forwards optional `max_tokens` (1–4096) and `temperature` (0–2) fields through the existing v1 `parameters` object. Unset values are omitted so each LMLM backend retains its own defaults. Validation occurs before network I/O. The v1 wire contract remains unchanged.

Capability discovery/negotiation is not exposed by the current LMLM v1 service; the client validates the fixed protocol identifier and version instead. Streaming, batching, device selection, modality-specific preprocessing/inference, and cross-process cancellation are not implemented.

## Limits and security boundaries

- The LMLM HTTP service refuses to bind to a non-loopback host.
- The Rust HTTP client refuses non-loopback and IPv6 destinations; it takes a socket address rather than an arbitrary URL.
- The LMLM protocol limits decoded request payloads to 8 MiB and request bodies to 18 MiB.
- LMLMxtend applies its own request and host-configured output limits.
- Transport timeouts are bounded (default 120 seconds; maximum 600 seconds).
- The service has no authentication. Keep it on loopback; do not expose it through a tunnel, public reverse proxy, or network port forward.
- Model assets must be installed locally by the operator.
- The bridge is not a process sandbox and does not impose hard CPU, memory, GPU, or wall-clock quotas on inference.

## Verification

From the LMLMxtend repository:

```bash
cargo test --workspace
bash scripts/verify.sh
bash scripts/smoke.sh
```

From the LMLM repository:

```bash
python -m unittest tests.test_processor_v1 -v
```

These commands are the expected validation path. A passing unit-test result does not replace an end-to-end run with a real local model installed and the service listening on loopback.
