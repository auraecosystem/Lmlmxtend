# Contributing

## Development

Use a stable Rust toolchain.

```bash
cargo fmt --all -- --check
cargo test --workspace
cargo clippy --workspace --all-targets -- -D warnings
```

## Engineering rules

- Preserve the LMLM / LMLMxtend / application responsibility boundaries.
- Do not add a neural model implementation here; integrate through a processor bridge.
- Keep core crates independent of Olgin and external provider SDKs.
- Avoid unsafe Rust.
- Add tests for every contract or policy change.
- Document whether a feature is implemented, experimental, or planned.
- Never describe in-process extension execution as sandboxed.
