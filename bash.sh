#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$ROOT"

bash scripts/verify.sh
bash scripts/smoke.sh
cargo run -p lmlmxtend-extension-forge -- new --name scene-composer --category creative --capability creative.compose
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
