#!/bin/bash
# canonicalize_map_len_on_off_vm.sh — Verify MapBox.len ModuleFunction → Method rewrite

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
if ROOT_GIT=$(git -C "$SCRIPT_DIR" rev-parse --show-toplevel 2>/dev/null); then
  ROOT="$ROOT_GIT"
else
  ROOT="$(cd "$SCRIPT_DIR/../../../../../../../../.." && pwd)"
fi
source "$ROOT/tools/smokes/v2/lib/test_runner.sh"
require_env || exit 2

json_path="/tmp/ny_v1_map_len_$$.json"
mut_on="/tmp/ny_v1_map_len_on_$$.json"
mut_off="/tmp/ny_v1_map_len_off_$$.json"

cat >"$json_path" <<'JSON'
{"schema_version":"1.0","functions":[{"name":"main","blocks":[{"id":0,"instructions":[{"op":"const","dst":1,"value":{"type":"i64","value":0}},{"op":"mir_call","mir_call":{"callee":{"type":"ModuleFunction","name":"MapBox.len"},"args":[1]}},{"op":"ret"}]}]}]}
JSON

# ON → should rewrite to Method(box=MapBox, method=len, receiver=1)
set +e
HAKO_NYVM_V1_DOWNCONVERT=1 HAKO_BRIDGE_INJECT_SINGLETON=1 HAKO_DEBUG_NYVM_BRIDGE_DUMP_MUT="$mut_on" \
  "$NYASH_BIN" --json-file "$json_path" >/dev/null 2>&1
set -e || true
if [ ! -f "$mut_on" ] || ! grep -q '"type":"Method"' "$mut_on" || ! grep -q '"box_name":"MapBox"' "$mut_on" || ! grep -q '"method":"len"' "$mut_on"; then
  echo "[FAIL] canonicalize_map_len_on_off_vm (ON)" >&2; exit 1
fi

# OFF → should keep ModuleFunction (or no mutation dump)
set +e
HAKO_NYVM_V1_DOWNCONVERT=1 HAKO_DEBUG_NYVM_BRIDGE_DUMP_MUT="$mut_off" \
  "$NYASH_BIN" --json-file "$json_path" >/dev/null 2>&1
set -e || true
if [ -f "$mut_off" ] && ! grep -q '"type":"ModuleFunction"' "$mut_off"; then
  echo "[FAIL] canonicalize_map_len_on_off_vm (OFF)" >&2; exit 1
fi

echo "[PASS] canonicalize_map_len_on_off_vm"
rm -f "$json_path" "$mut_on" "$mut_off"
exit 0
