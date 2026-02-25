#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../../../../../.." && pwd)"
BIN="${NYASH_BIN:-$ROOT_DIR/target/release/hakorune}"

if [ ! -x "$BIN" ]; then
  echo "[analyze] hakorune not built: $BIN" >&2
  echo "Run: cargo build --release" >&2
  exit 2
fi

# Run analyzer rule tests (HC011 dead methods) via run_tests.sh
pushd "$ROOT_DIR" >/dev/null
bash tools/hako_check/run_tests.sh
popd >/dev/null

echo "[analyze/quick] HC011 dead methods tests: OK"
exit 0
