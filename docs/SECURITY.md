# Security model

## Current guarantees

The initial runtime provides:
- capability declaration and ownership checks;
- optional capability allow-lists;
- a configurable maximum request-input size;
- manifest API-version validation;
- duplicate extension and capability rejection;
- Rust unsafe-code prohibition at workspace level.

## Important limitation

The current in-process extension trait is **not a security sandbox**. Registered extension code executes inside the host process with the host process's operating-system permissions. Do not load untrusted extensions into this runtime.

## Required before third-party extensions

1. Process or WASM sandbox with explicit filesystem, network, and device permissions.
2. Per-extension memory, CPU, wall-clock, and output limits.
3. Cancellation and shutdown semantics.
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
