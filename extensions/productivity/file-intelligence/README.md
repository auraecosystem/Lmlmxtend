# LMLMxtend File Intelligence

A local-first Rust extension for studying directories, organizing loose files, and merging UTF-8 text files.

## Capabilities

- `files.study` — recursively inventories a directory, classifies files, records size and modification time, and finds duplicate-content candidates with SHA-256.
- `files.organize` — previews category moves by default. A confirmed local-folder run moves only direct child regular files. A directory inside a Git repository is always report-only. The JSON operation may be `organize` or its `arrange` alias.
- `files.merge` — merges two or more supported UTF-8 text files of one extension into a new output file. It never overwrites an existing file or modifies inputs.

## Request protocol

Send JSON as `ExtensionRequest.input`. The capability on the request must match the selected operation.

Study:

```json
{"operation":"study","root":"/path/to/folder"}
```

Organization preview:

```json
{"operation":"organize","root":"/path/to/folder","confirm":false}
```

Apply local organization (only for a non-repository folder):

```json
{"operation":"organize","root":"/path/to/folder","confirm":true}
```

Merge text files:

```json
{"operation":"merge","files":["/path/a.md","/path/b.md"],"output":"/path/combined.md"}
```

## Safety behavior

- No deletion, overwrite, Git staging, commit, push, reset, or history rewrite.
- Repository organization is always a plan; tracked and untracked repository files are not moved.
- Local organization touches only direct child regular files, never recursively moves nested files, and avoids name collisions.
- Symlinks and common dependency/build/cache directories are excluded from study.
- Merge accepts only a fixed allow-list of text extensions, caps combined input at 32 MiB, rejects symlink inputs, and creates output with exclusive-create semantics.
- Study hashes regular files up to 512 MiB each; it does not load entire files into memory.
- Long operations poll the host cancellation/deadline context.

## Limitations

- PDF and office-document merging are not implemented in this first Rust extension. The merge operation is text-only.
- This runs in-process and is not an operating-system sandbox. Register only trusted extension code and apply host filesystem policy before exposing it to an application.
- The extension returns reports to the host; it does not upload files or access remote repositories.

## Build and test

```bash
cargo test --manifest-path extensions/productivity/file-intelligence/Cargo.toml
```
