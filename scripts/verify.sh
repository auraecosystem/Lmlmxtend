#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

echo "==> Rust formatting"
cargo fmt --all -- --check

echo "==> Clippy (all workspace targets)"
cargo clippy --workspace --all-targets -- -D warnings

echo "==> Workspace tests"
cargo test --workspace

echo "LMLMxtend verification completed successfully."
