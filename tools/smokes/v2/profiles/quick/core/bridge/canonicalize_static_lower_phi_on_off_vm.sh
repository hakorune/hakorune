#!/bin/bash
# canonicalize_static_lower_phi_on_off_vm.sh — Verify PhiInst.lower_phi rewrite (ModuleFunction → Method)

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
if ROOT_GIT=$(git -C "$SCRIPT_DIR" rev-parse --show-toplevel 2>/dev/null); then
  ROOT="$ROOT_GIT"
else
  ROOT="$(cd "$SCRIPT_DIR/../../../../../../../../.." && pwd)"
fi
source "$ROOT/tools/smokes/v2/lib/test_runner.sh"
require_env || exit 2

json_path="/tmp/ny_v1_lower_phi_$$.json"
mut_on="/tmp/ny_v1_lower_phi_on_$$.json"
mut_off="/tmp/ny_v1_lower_phi_off_$$.json"

cat >"$json_path" <<'JSON'
{"schema_version":"1.0","functions":[{"name":"main","blocks":[{"id":0,"instructions":[{"op":"mir_call","mir_call":{"callee":{"type":"ModuleFunction","name":"PhiInst.lower_phi"},"args":[1]}},{"op":"ret"}]}]}]}
JSON

set +e
HAKO_NYVM_V1_DOWNCONVERT=1 HAKO_BRIDGE_INJECT_SINGLETON=1 HAKO_DEBUG_NYVM_BRIDGE_DUMP_MUT="$mut_on" \
  "$NYASH_BIN" --json-file "$json_path" >/dev/null 2>&1
set -e || true
if [ ! -f "$mut_on" ] || ! grep -q '"type":"Method"' "$mut_on" || ! grep -q '"box_name":"PhiInst"' "$mut_on" || ! grep -q '"method":"lower_phi"' "$mut_on"; then
  echo "[FAIL] canonicalize_static_lower_phi_on_off_vm (ON)" >&2; exit 1
fi

set +e
HAKO_NYVM_V1_DOWNCONVERT=1 HAKO_DEBUG_NYVM_BRIDGE_DUMP_MUT="$mut_off" \
  "$NYASH_BIN" --json-file "$json_path" >/dev/null 2>&1
set -e || true
if [ -f "$mut_off" ] && ! grep -q '"type":"ModuleFunction"' "$mut_off"; then
  echo "[FAIL] canonicalize_static_lower_phi_on_off_vm (OFF)" >&2; exit 1
fi

echo "[PASS] canonicalize_static_lower_phi_on_off_vm"
rm -f "$json_path" "$mut_on" "$mut_off"
exit 0
