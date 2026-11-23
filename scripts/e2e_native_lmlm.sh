#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
LMLM_ROOT="${LMLM_REPO_ROOT:-}"
MODEL_DIR="${LMLM_MODEL_DIR:-}"
PYTHON_BIN="${PYTHON:-python3}"
PORT="${LMLM_PROCESSOR_PORT:-8765}"
ADDRESS="127.0.0.1:${PORT}"
LOG_FILE="$(mktemp)"

cleanup() {
  if [[ -n "${SERVER_PID:-}" ]]; then
    kill "$SERVER_PID" 2>/dev/null || true
    wait "$SERVER_PID" 2>/dev/null || true
  fi
  rm -f "$LOG_FILE"
}
trap cleanup EXIT

if [[ -z "$LMLM_ROOT" || ! -d "$LMLM_ROOT/lmlm_processor" ]]; then
  echo "Set LMLM_REPO_ROOT to a checkout containing lmlm_processor/." >&2
  exit 2
fi
if [[ -z "$MODEL_DIR" || ! -f "$MODEL_DIR/lmlm-model.json" ]]; then
  echo "Set LMLM_MODEL_DIR to an installed native LMLM model directory containing lmlm-model.json." >&2
  exit 2
fi
if ! command -v "$PYTHON_BIN" >/dev/null 2>&1; then
  echo "Python executable not found: $PYTHON_BIN" >&2
  exit 2
fi
if ! command -v cargo >/dev/null 2>&1; then
  echo "Cargo is required to compile and run the Rust bridge example." >&2
  exit 2
fi

MODEL_DIR="$(cd "$MODEL_DIR" && pwd)"
export LMLM_PROCESSOR_ADDR="$ADDRESS"

echo "Starting native LMLM Processor v1 at $ADDRESS"
(
  cd "$LMLM_ROOT"
  export PYTHONPATH="$LMLM_ROOT${PYTHONPATH:+:$PYTHONPATH}"
  export LMLM_BACKEND=native_cpu
  export LMLM_MODEL_DIR="$MODEL_DIR"
  export LMLM_PROCESSOR_HOST=127.0.0.1
  export LMLM_PROCESSOR_PORT="$PORT"
  exec "$PYTHON_BIN" -m lmlm_processor.server
) >"$LOG_FILE" 2>&1 &
SERVER_PID=$!

"$PYTHON_BIN" - "$PORT" "$SERVER_PID" "$LOG_FILE" <<'PY'
import sys
import time
import urllib.error
import urllib.request

port, server_pid, log_file = sys.argv[1:]
url = f"http://127.0.0.1:{port}/v1/health"
for _ in range(120):
    try:
        with urllib.request.urlopen(url, timeout=1) as response:
            if response.status == 200:
                print("LMLM Processor v1 health check passed.")
                raise SystemExit(0)
    except (OSError, urllib.error.URLError):
        pass
    try:
        import os
        os.kill(int(server_pid), 0)
    except OSError:
        print("LMLM processor exited before becoming ready:", file=sys.stderr)
        with open(log_file, encoding="utf-8", errors="replace") as log:
            print(log.read(), file=sys.stderr)
        raise SystemExit(1)
    time.sleep(1)
print("Timed out waiting for LMLM Processor v1 health.", file=sys.stderr)
with open(log_file, encoding="utf-8", errors="replace") as log:
    print(log.read(), file=sys.stderr)
raise SystemExit(1)
PY

echo "Running Rust bridge against the real native LMLM model"
cargo run --manifest-path "$ROOT/Cargo.toml" \
  -p lmlmxtend-processor-bridge \
  --example local_lmlm_http_generation

echo "Native LMLM cross-project end-to-end test passed."
