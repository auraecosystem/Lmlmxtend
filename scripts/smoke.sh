#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

echo "==> Extension runtime example"
cargo run --example basic_extension -p lmlmxtend-runtime

echo "==> Agent runtime example"
cargo run --example basic_agent -p lmlmxtend-agent-runtime

echo "==> Extension Forge help"
cargo run -p lmlmxtend-extension-forge -- help

echo "==> Generate and compile a temporary extension"
mkdir -p extensions
TEMP_ROOT="$(mktemp -d extensions/.lmlmxtend-smoke.XXXXXX)"
cleanup() { rm -rf "$TEMP_ROOT"; }
trap cleanup EXIT

cargo run -p lmlmxtend-extension-forge -- new \
  --name smoke-test \
  --category creative \
  --capability creative.compose \
  --root "$TEMP_ROOT"

cargo test --manifest-path "$TEMP_ROOT/creative/smoke-test/Cargo.toml"

echo "LMLMxtend smoke checks completed successfully."
