#!/bin/bash
# canonicalize_noop_method_on_vm.sh — ONでもMethodは変異しない（dump-mut未生成）

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
if ROOT_GIT=$(git -C "$SCRIPT_DIR" rev-parse --show-toplevel 2>/dev/null); then
  ROOT="$ROOT_GIT"
else
  ROOT="$(cd "$SCRIPT_DIR/../../../../../../../../.." && pwd)"
fi
source "$ROOT/tools/smokes/v2/lib/test_runner.sh"
require_env || exit 2

json_path="/tmp/ny_v1_noop_method_$$.json"
mut_on="/tmp/ny_v1_noop_method_on_$$.json"

cat >"$json_path" <<'JSON'
{"schema_version":"1.0","functions":[{"name":"main","blocks":[{"id":0,"instructions":[{"op":"mir_call","mir_call":{"callee":{"type":"Method","box_name":"ArrayBox","method":"size","receiver":1},"args":[] }},{"op":"ret"}]}]}]}
JSON

set +e
HAKO_NYVM_V1_DOWNCONVERT=1 HAKO_BRIDGE_INJECT_SINGLETON=1 HAKO_DEBUG_NYVM_BRIDGE_DUMP_MUT="$mut_on" \
  "$ROOT/target/release/nyash" --json-file "$json_path" >/dev/null 2>&1
set -e || true

if [ -f "$mut_on" ]; then
  echo "[FAIL] canonicalize_noop_method_on_vm: mutated dump should not be created for Method" >&2
  exit 1
fi

echo "[PASS] canonicalize_noop_method_on_vm"
rm -f "$json_path" "$mut_on"
exit 0

