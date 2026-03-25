#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
if ROOT_GIT=$(git -C "$SCRIPT_DIR" rev-parse --show-toplevel 2>/dev/null); then
  ROOT="$ROOT_GIT"
else
  ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"
fi

source "$ROOT/tools/smokes/v2/lib/test_runner.sh"
require_env >/dev/null || exit 2

tmp_mir_route="$(mktemp /tmp/phase29ci_verify_core_route_XXXX.json)"
tmp_prog="$(mktemp /tmp/phase29ci_verify_internal_new_array_XXXX.json)"

cleanup() {
  rm -f "$tmp_mir_route" "$tmp_prog" || true
}
trap cleanup EXIT

mir_json='{"cfg":{"functions":[{"blocks":[{"id":0,"reachable":false,"successors":[],"terminator":"Return"}],"entry_block":0,"name":"main"}]},"functions":[{"attrs":{"runes":[]},"blocks":[{"id":0,"instructions":[{"args":[],"dst":1,"op":"newbox","type":"ArrayBox"},{"dst":2,"op":"const","value":{"type":"i64","value":0}},{"op":"ret","value":2}]}],"metadata":{"value_types":{}},"name":"main","params":[0]}],"user_box_decls":[],"version":0}'

set +e
HAKO_VERIFY_PRIMARY=core run_built_mir_json_via_verify_routes "$mir_json" "$tmp_mir_route"
route_rc=$?
set -e

if [ "$route_rc" -ne 0 ]; then
  echo "[FAIL] phase29ci_verify_primary_core_route_probe: core-primary verify route rc=$route_rc" >&2
  exit 1
fi

cat >"$tmp_prog" <<'JSON'
{"version":0,"kind":"Program","body":[
  {"type":"Local","name":"a","expr":{"type":"New","class":"ArrayBox","args":[]}},
  {"type":"Return","expr":{"type":"Int","value":0}}
]}
JSON

set +e
run_verify_program_via_internal_builder_no_methods_to_core "$tmp_prog"
internal_rc=$?
set -e

if [ "$internal_rc" -ne 0 ]; then
  echo "[FAIL] phase29ci_verify_primary_core_route_probe: internal no-methods verify rc=$internal_rc" >&2
  exit 1
fi

echo "[PASS] phase29ci_verify_primary_core_route_probe"
