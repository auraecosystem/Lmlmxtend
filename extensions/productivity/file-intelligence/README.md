# LMLMxtend File Intelligence

A local-first Rust extension for studying directories, organizing loose files, and merging text, PDF, and supported DOCX documents.

## Capabilities

- `files.study` — recursively inventories a directory, classifies files, records size and modification time, and finds duplicate-content candidates with SHA-256.
- `files.organize` — previews category moves by default. A confirmed local-folder run moves only direct child regular files. A directory inside a Git repository is always report-only. The JSON operation may be `organize` or its `arrange` alias.
- `files.merge` — merges two or more files of one supported format into a new output file: UTF-8 text, PDF pages, or DOCX body content. It never overwrites an existing file or modifies inputs.

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

Merge PDF pages:

```json
{"operation":"merge","files":["/path/part-1.pdf","/path/part-2.pdf"],"output":"/path/combined.pdf"}
```

Merge Word documents:

```json
{"operation":"merge","files":["/path/part-1.docx","/path/part-2.docx"],"output":"/path/combined.docx"}
```

The output format is inferred from the shared input extension. Mixing formats is rejected.

## Safety behavior

- No deletion, overwrite, Git staging, commit, push, reset, or history rewrite.
- Repository organization is always a plan; tracked and untracked repository files are not moved.
- Local organization touches only direct child regular files, never recursively moves nested files, and avoids name collisions.
- Symlinks and common dependency/build/cache directories are excluded from study.
- Merge rejects symlink inputs and mixed extensions, and creates output with exclusive-create semantics. Text merge is capped at 32 MiB; PDF and DOCX merge are capped at 128 MiB combined input.
- Study hashes regular files up to 512 MiB each; it does not load entire files into memory.
- Long operations poll the host cancellation/deadline context.

## Limitations

- PDF merge combines page trees and page resources. It rejects encrypted PDFs. Source bookmarks/outlines and document-level form/catalog features are not combined; page content, page resources, and page-level annotations are retained where represented in the page objects.
- DOCX merge appends paragraphs and tables, inserts a source-document separator, and merges non-conflicting style definitions. The first DOCX supplies the package-level settings and page layout. Appended documents containing images, hyperlinks, numbering, comments, footnotes, headers, footers, custom XML, or other relationship-dependent body elements are rejected rather than silently producing broken references.
- Legacy `.doc`, `.docm`, `.odt`, `.rtf`, `.xlsx`, and `.pptx` are not merged by this version.
- This runs in-process and is not an operating-system sandbox. Register only trusted extension code and apply host filesystem policy before exposing it to an application.
- The extension returns reports to the host; it does not upload files or access remote repositories.

## Build and test

```bash
cargo test --manifest-path extensions/productivity/file-intelligence/Cargo.toml
```
