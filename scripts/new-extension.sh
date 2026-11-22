#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

if [[ $# -lt 1 ]]; then
  echo "Usage: bash scripts/new-extension.sh --name <slug> --category <category> --capability <namespace.action> [--root <path>]" >&2
  exit 2
fi

cargo run -p lmlmxtend-extension-forge -- new "$@"
