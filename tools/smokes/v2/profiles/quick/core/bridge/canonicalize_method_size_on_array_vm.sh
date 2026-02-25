#!/bin/bash
# canonicalize_method_size_on_array_vm.sh — v1 bridge positive: Constructor + Method(size) → rc=0

set -euo pipefail
SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
if ROOT_GIT=$(git -C "$SCRIPT_DIR" rev-parse --show-toplevel 2>/dev/null); then
  ROOT="$ROOT_GIT"
else
  ROOT="$(cd "$SCRIPT_DIR/../../../../../../../../.." && pwd)"
fi
source "$ROOT/tools/smokes/v2/lib/test_runner.sh"
require_env || exit 2

json_path="/tmp/ny_v1_ctor_method_size_$$.json"
cat >"$json_path" <<'JSON'
{"schema_version":"1.0","functions":[{"name":"main","blocks":[{"id":0,"instructions":[
  {"op":"mir_call","dst":1, "callee":{"type":"Constructor","box_type":"ArrayBox"}, "args":[]},
  {"op":"mir_call","dst":2, "callee":{"type":"Method","method":"size","receiver":1}, "args":[]},
  {"op":"ret","value":2}
]}]}]}
JSON

set +e
HAKO_NYVM_V1_DOWNCONVERT=1 "$NYASH_BIN" --json-file "$json_path" >/dev/null 2>&1
rc=$?
set -e
rm -f "$json_path"

if [ "$rc" = "0" ]; then
  echo "[PASS] canonicalize_method_size_on_array_vm"
else
  echo "[FAIL] canonicalize_method_size_on_array_vm (rc=$rc)" >&2
  exit 1
fi

