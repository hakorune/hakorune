#!/bin/bash
# gate_c_parity_pipe_vm.sh — Gate‑C (stdin pipe) exit-code mirrors return (quick)

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
ROOT="$(cd "$SCRIPT_DIR/../../../../../.." && pwd)"
source "$ROOT/tools/smokes/v2/lib/test_runner.sh"
require_env || exit 2

payload='{"version":0,"kind":"Program","body":[{"type":"Return","expr":{"type":"Int","value":9}}]}'
set +e
echo "$payload" | "$NYASH_BIN" --ny-parser-pipe >/dev/null 2>&1
rc=$?
set -e

if [ $rc -eq 9 ]; then
  echo "[PASS] gate_c_parity_pipe_vm"
  exit 0
else
  echo "[FAIL] gate_c_parity_pipe_vm: expected rc=9 got $rc" >&2
  exit 1
fi
