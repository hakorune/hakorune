#!/bin/bash
# canonicalize_closure_captures_vm.sh — v1 bridge Closure captures (acceptance)

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
if ROOT_GIT=$(git -C "$SCRIPT_DIR" rev-parse --show-toplevel 2>/dev/null); then
  ROOT="$ROOT_GIT"
else
  ROOT="$(cd "$SCRIPT_DIR/../../../../../../../../.." && pwd)"
fi
source "$ROOT/tools/smokes/v2/lib/test_runner.sh"
require_env || exit 2

# Opt-in: enable with SMOKES_ENABLE_BRIDGE_CLOSURE=1
if [ "${SMOKES_ENABLE_BRIDGE_CLOSURE:-0}" != "1" ]; then
  test_skip canonicalize_closure_captures_vm "opt-in (SMOKES_ENABLE_BRIDGE_CLOSURE=1)"
  exit 0
fi

# Minimal v1 JSON: create a Closure (dst=3) with captures; ensure bridge accepts it.
json_path="/tmp/ny_v1_closure_caps_$$.json"
cat >"$json_path" <<'JSON'
{"schema_version":"1.0","functions":[{"name":"main","blocks":[{"id":0,"instructions":[
  {"op":"mir_call","dst":3,
    "callee":{"type":"Closure","params":["a","b"],"captures":[["x",1],["y",2]]},
    "args":[1,2]},
  {"op":"ret"}
]}]}]}
JSON

set +e
HAKO_NYVM_V1_DOWNCONVERT=1 "$NYASH_BIN" --json-file "$json_path" >/dev/null 2>&1
rc=$?
set -e
rm -f "$json_path"

if [ "$rc" = 0 ]; then
  echo "[PASS] canonicalize_closure_captures_vm"
  exit 0
else
  echo "[FAIL] canonicalize_closure_captures_vm (rc=$rc)" >&2
  exit 1
fi
