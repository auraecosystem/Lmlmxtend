# Extension catalogue

LMLMxtend extensions are grouped by capability domain:

- creative/ — scene, image, document, and creative workflows.
- language/ — text processing, translation, and language tools.
- vision/ — image and visual understanding interfaces.
- audio/ — speech, sound, and audio-processing interfaces.
- spatial/ — 3D, XR, geometry, and spatial interaction interfaces.
- agents/ — task-oriented agent extensions.
- productivity/ — local file intelligence, organization, and workflow utilities.

Create an extension with the native generator:

    cargo run -p lmlmxtend-extension-forge -- new --name scene-composer --category creative --capability creative.compose

The generator creates a standalone Rust crate, extension descriptor, tested API
implementation template, and README. It refuses to overwrite existing files.
The productivity file-intelligence extension is available at
`extensions/productivity/file-intelligence`; see its README and
`docs/FILE_INTELLIGENCE.md`.

Generated extensions are not automatically loaded by the runtime. Registration,
permission policy, and sandboxing remain explicit host responsibilities.
