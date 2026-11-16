# Architecture

## Responsibility model

| Component | Owns | Does not own |
|---|---|---|
| LMLM | Neural processor, model internals, learned parameters, inference implementation | Application-specific UI |
| LMLMxtend | Extension contracts, capability dispatch, policy hooks, adapters, SDKs, orchestration | A replacement neural processor |
| Olgin | Creative documents, editing tools, rendering, user experience | LMLM neural core |

LMLMxtend must remain useful to multiple host applications. Olgin integration belongs in an adapter and must not become a dependency of the core crates.

## Layering

1. **API contracts** define manifests, capability identifiers, requests, responses, and extension errors.
2. **Core** defines host configuration and policy primitives.
3. **Runtime** validates extension manifests, registers capabilities, and dispatches requests.
4. **Intelligence services** will coordinate tasks and route work to LMLM through a separately versioned processor bridge.
5. **Multimodal services** will define typed media inputs/outputs and streaming contracts.
6. **Adapters and SDKs** expose the framework to applications and language ecosystems.

## Dependency direction

```text
Application (Olgin or another host)
              |
        LMLMxtend SDK
              |
     LMLMxtend runtime
        /         \
  Extension API   LMLM bridge
                      |
                     LMLM
```

The LMLM bridge is an adapter boundary. The core must not assume a specific LMLM transport or internal implementation. No dependency points from LMLM back to LMLMxtend or from LMLMxtend core to Olgin.

## Execution lifecycle

1. Host constructs a runtime with explicit limits and policy.
2. Extension supplies a manifest with ID, version, API version, and capabilities.
3. Runtime validates manifest compatibility and rejects duplicate ownership.
4. Host submits a capability request with input and optional correlation ID.
5. Policy checks capability allow-list and input size.
6. Runtime dispatches only to the extension that declared the capability.
7. Extension returns a typed response or a structured error.
8. Host records telemetry and applies application-level cancellation/timeouts.

## Extension packaging (planned)

A future package manifest will declare package identity, semantic version, API compatibility, entrypoint, requested permissions, resource limits, and integrity metadata. Loading executable extensions from disk is intentionally not implemented in this first milestone.

## Compatibility

- API version is explicit in every extension manifest.
- Breaking contract changes require a major API version.
- Runtime rejects incompatible extension manifests.
- Adapters must negotiate supported API and capability versions.
- Contract tests should run against every supported SDK and adapter.
