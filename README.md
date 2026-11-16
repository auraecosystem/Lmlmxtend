# LMLMxtend

**LMLMxtend** is the independent extension, integration, and extended-intelligence framework built around the LMLM neural processor.

It provides stable contracts and application-facing capabilities without reimplementing or replacing the LMLM processor.

## Project boundaries

- **LMLM** — the Large Multi-Functional Linear Model: the in-built neural processor and intelligence core.
- **LMLMxtend** — extension contracts, capability orchestration, runtime services, multimodal interfaces, agents, adapters, and SDKs.
- **Olgin** — a creative application that may consume LMLMxtend through an optional integration.

LMLMxtend is application-independent. Olgin is one possible consumer, not a dependency of the framework.

## Current implementation

The initial Rust workspace contains:

- `lmlmxtend-api`: versioned extension and capability contracts.
- `lmlmxtend-core`: runtime configuration, capability identifiers, and execution policy.
- `lmlmxtend-runtime`: an in-process extension registry, read-only capability discovery, and policy-checked dispatcher.
- `lmlmxtend-intelligence`: deterministic capability-aware task planning and dispatch with API-version checks and explicit provider preferences.
- `lmlmxtend-agent-runtime`: permission-aware agent registration, bounded delegation, capability invocation, and lifecycle propagation.
- `lmlmxtend-extension-forge`: a native CLI that scaffolds validated, standalone extension crates.

The current dispatcher is an integration foundation. It does **not** contain neural-network weights, inference kernels, a model implementation, or a production sandbox.

## Quick start

Requires a stable Rust toolchain.

```bash
cargo test --workspace
cargo run --example basic_extension -p lmlmxtend-runtime
cargo run --example basic_agent -p lmlmxtend-agent-runtime

# Generate a new creative extension
cargo run -p lmlmxtend-extension-forge -- new --name scene-composer --category creative --capability creative.compose
```

See [Architecture](docs/ARCHITECTURE.md), [Security](docs/SECURITY.md), [Extension Forge](docs/EXTENSION_FORGE.md), [Intelligence Routing](docs/INTELLIGENCE_ROUTING.md), [Agent Runtime](docs/AGENT_RUNTIME.md), and [Roadmap](docs/ROADMAP.md).

## Design principles

1. LMLM remains the authoritative neural processor.
2. No circular dependency between LMLM, LMLMxtend, and applications.
3. Local-first operation; remote services are optional adapters.
4. Explicit capabilities and least-privilege execution.
5. Versioned contracts and compatibility tests.
6. A host application remains useful when LMLMxtend is unavailable.

## Status

Early foundation / API design. Interfaces will evolve before a stable 1.0 release. See the roadmap for implementation stages.
