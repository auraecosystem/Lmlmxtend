# File Intelligence Extension

The `productivity.file-intelligence` extension provides local-first file study, conservative organization, and text merging. It follows the existing LMLMxtend extension contract and does not add file-management behavior to the LMLM neural processor.

## Build and test

```bash
cargo test --manifest-path extensions/productivity/file-intelligence/Cargo.toml
```

## Register in a trusted host

Add the extension crate as a dependency to the host application, then register it through the existing runtime:

```rust
use lmlmxtend_file_intelligence::FileIntelligence;
use lmlmxtend_runtime::Runtime;
use lmlmxtend_core::RuntimeConfig;
use std::sync::Arc;

let mut runtime = Runtime::new(RuntimeConfig::default());
runtime.register(Arc::new(FileIntelligence))?;
```

The host must still enforce its own filesystem allow-list and user-consent policy. The extension is trusted in-process Rust code, not a sandbox.

## Capabilities and operations

| Capability | Operation | Behavior |
|---|---|---|
| `files.study` | `study` | Recursive inventory, category, size, modification time, duplicate candidates |
| `files.organize` | `organize` | Preview by default; confirmed moves only in non-repository folders |
| `files.merge` | `merge` | Merge supported UTF-8 text files into a new output |

Inputs are JSON strings in `ExtensionRequest.input`. See the extension README for exact request examples.

## Repository protection

If the selected root is inside a Git worktree (including a nested directory), organization remains report-only regardless of the confirmation flag. The extension does not call Git commands or modify the index, commits, refs, or remote repository. It does not move repository files.

## Merge constraints

Text merge requires at least two regular, non-symlink files with the same allow-listed extension. Combined input is capped at 32 MiB. The output must not already exist. Source files remain unchanged. PDF and office-document merge are not part of this initial Rust implementation.
