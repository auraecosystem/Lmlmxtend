# LMLMxtend Extension Forge

The Extension Forge is the native Rust scaffolder for LMLMxtend extension
authors. It creates a consistent starting point without a runtime dependency
or a third-party code generator.

## Generate an extension

    cargo run -p lmlmxtend-extension-forge -- new --name scene-composer --category creative --capability creative.compose

Supported categories: creative, language, vision, audio, spatial, and agents.

Optional --root <path> changes the output root; the default is extensions/.
The command refuses to overwrite an existing destination.

## Generated files

- Cargo.toml: standalone Rust package with a local path dependency on lmlmxtend-api.
- extension.toml: extension identity and capability metadata.
- src/lib.rs: implementation of the existing Extension contract and starter tests.
- README.md: extension-specific development instructions.

## Validation and safety

Names are restricted to lowercase ASCII slugs. Capabilities are validated by
the same CapabilityId contract used by the runtime. Files are created with
exclusive-create semantics; existing files are never silently replaced.

The generated extension is a development scaffold, not a trusted package.
It is not automatically installed, registered, or sandboxed. Review its code
and permissions before loading it into a host.
