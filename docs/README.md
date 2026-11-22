# LMLM — Large Multimodal Local Model

LMLM is the in-built neural model designed to run within a device, local environment, or controlled execution playground. It is the intelligence processor itself—not merely an application, model registry, or connector to external AI providers.

LMLM's role is to provide the core model execution and multimodal intelligence. Its implementation, model parameters, tokenizer, runtime internals, and inference mechanisms belong to the LMLM project.

## Relationship to LMLMxtend

LMLMxtend is an independent extension and orchestration framework around LMLM.

- **LMLM** owns the neural model and its core intelligence.
- **LMLMxtend** owns extension contracts, capability routing, agent orchestration, adapters, SDKs, and integration boundaries.
- **Host applications** consume LMLMxtend through optional, versioned interfaces.

LMLMxtend must not duplicate or replace the LMLM neural core. A concrete processor bridge should be implemented only after the LMLM project exposes a stable public interface.

## Architectural principles

1. LMLM remains independently usable.
2. LMLMxtend remains application-independent.
3. Local-first execution is the default.
4. Remote services are optional adapters, never mandatory model dependencies.
5. Contracts are versioned and capability access is explicit.
6. No circular dependency between LMLM, LMLMxtend, and consuming applications.
7. Untrusted extensions require process or WASM isolation before execution.

## Current implementation status

The repository currently provides Rust extension contracts, a local in-process runtime, deterministic capability routing, bounded agent orchestration, and a native extension scaffolder.

It does not yet provide a concrete LMLM processor adapter, neural inference implementation, production sandbox, or complete multimodal streaming layer. See [Architecture](ARCHITECTURE.md), [Security](SECURITY.md), and [Roadmap](ROADMAP.md).
