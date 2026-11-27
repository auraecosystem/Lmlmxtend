#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$ROOT"

bash scripts/verify.sh
bash scripts/smoke.sh
cargo run -p lmlmxtend-extension-forge -- new --name scene-composer --category creative --capability creative.compose
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
git clone https://github.com/auraecosystem/Lmlmxtend.git
cd Lmlmxtend

LMLM_REPO_ROOT=/path/to/lmlm \
LMLM_MODEL_DIR=/path/to/installed-model \
bash scripts/e2e_native_lmlm.sh
