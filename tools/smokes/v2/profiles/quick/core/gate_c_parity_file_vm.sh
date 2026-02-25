#!/bin/bash
# gate_c_parity_file_vm.sh — Gate‑C (file) exit-code mirrors return (quick)

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
ROOT="$(cd "$SCRIPT_DIR/../../../../../.." && pwd)"
source "$ROOT/tools/smokes/v2/lib/test_runner.sh"
require_env || exit 2

json_path="/tmp/ny_gatec_parity_$$.json"
cat >"$json_path" <<'JSON'
{"version":0,"kind":"Program","body":[{"type":"Return","expr":{"type":"Int","value":42}}]}
JSON

set +e
"$NYASH_BIN" --json-file "$json_path" >/dev/null 2>&1
rc=$?
set -e
rm -f "$json_path"

if [ $rc -eq 42 ]; then
  echo "[PASS] gate_c_parity_file_vm"
  exit 0
else
  echo "[FAIL] gate_c_parity_file_vm: expected rc=42 got $rc" >&2
  exit 1
fi
