#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"
FILE_INTELLIGENCE="extensions/productivity/file-intelligence/Cargo.toml"

echo "==> Rust workspace formatting"
cargo fmt --all -- --check

echo "==> File Intelligence formatting"
cargo fmt --manifest-path "$FILE_INTELLIGENCE" -- --check

echo "==> Clippy (workspace targets)"
cargo clippy --workspace --all-targets -- -D warnings

echo "==> Clippy (File Intelligence)"
cargo clippy --manifest-path "$FILE_INTELLIGENCE" --all-targets -- -D warnings

echo "==> File Intelligence tests"
cargo test --manifest-path "$FILE_INTELLIGENCE"

echo "==> Workspace tests"
cargo test --workspace"

echo "LMLMxtend verification completed successfully."
