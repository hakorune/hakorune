#!/bin/bash
# canonicalize_method_missing_receiver_negative_vm.sh — v1 bridge Method without receiver should fail

set -euo pipefail
SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
if ROOT_GIT=$(git -C "$SCRIPT_DIR" rev-parse --show-toplevel 2>/dev/null); then
  ROOT="$ROOT_GIT"
else
  ROOT="$(cd "$SCRIPT_DIR/../../../../../../../../.." && pwd)"
fi
source "$ROOT/tools/smokes/v2/lib/test_runner.sh"
require_env || exit 2

if [ "${SMOKES_ENABLE_BRIDGE_CANON:-0}" != "1" ]; then
  echo "[SKIP] canonicalize_method_missing_receiver_negative_vm (SMOKES_ENABLE_BRIDGE_CANON=1)"
  exit 0
fi

json_path="/tmp/ny_v1_method_missing_recv_$$.json"
cat >"$json_path" <<'JSON'
{"schema_version":"1.0","functions":[{"name":"main","blocks":[{"id":0,"instructions":[
  {"op":"mir_call","dst":1,
    "callee":{"type":"Method","method":"size"},
    "args":[]},
  {"op":"ret"}
]}]}]}
JSON

set +e
HAKO_NYVM_V1_DOWNCONVERT=1 "$NYASH_BIN" --json-file "$json_path" >/dev/null 2>&1
rc=$?
set -e
rm -f "$json_path"

if [ "$rc" != 0 ]; then
  echo "[PASS] canonicalize_method_missing_receiver_negative_vm"
else
  echo "[FAIL] canonicalize_method_missing_receiver_negative_vm (unexpected rc=0)" >&2
  exit 1
fi

