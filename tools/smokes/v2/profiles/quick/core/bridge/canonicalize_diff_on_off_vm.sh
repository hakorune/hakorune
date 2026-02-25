#!/bin/bash
# canonicalize_diff_on_off_vm.sh — Verify ModuleFunction→Method rewrite via dump-mut JSON

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
if ROOT_GIT=$(git -C "$SCRIPT_DIR" rev-parse --show-toplevel 2>/dev/null); then
  ROOT="$ROOT_GIT"
else
  ROOT="$(cd "$SCRIPT_DIR/../../../../../../../../.." && pwd)"
fi
source "$ROOT/tools/smokes/v2/lib/test_runner.sh"
require_env || exit 2

# Always run (stable); dumps mutated JSON to /tmp for comparison

json_path="/tmp/ny_v1_mircall_$$.json"
mut_on="/tmp/ny_v1_mut_on_$$.json"
mut_off="/tmp/ny_v1_mut_off_$$.json"

# Minimal v1 JSON with a mir_call ModuleFunction callee
cat >"$json_path" <<'JSON'
{"schema_version":"1.0","functions":[{"name":"main","blocks":[{"id":0,"instructions":[{"op":"mir_call","mir_call":{"callee":{"type":"ModuleFunction","name":"LLVMPhiInstructionBox.lower_phi"},"args":[1]}},{"op":"ret"}]}]}]}
JSON

# Run with inject_singleton=ON and dump mutated JSON
set +e
HAKO_NYVM_V1_DOWNCONVERT=1 HAKO_BRIDGE_INJECT_SINGLETON=1 HAKO_DEBUG_NYVM_BRIDGE_DUMP_MUT="$mut_on" \
  "$NYASH_BIN" --json-file "$json_path" >/dev/null 2>&1
set -e || true

# Run with inject_singleton=OFF and dump mutated JSON (should not be created or differ)
set +e
HAKO_NYVM_V1_DOWNCONVERT=1 HAKO_DEBUG_NYVM_BRIDGE_DUMP_MUT="$mut_off" \
  "$NYASH_BIN" --json-file "$json_path" >/dev/null 2>&1
set -e || true

if [ -f "$mut_on" ]; then
  if grep -q '"type":"Method"' "$mut_on"; then
    echo "[PASS] bridge_canonicalize_diff_on_off (ON produced Method)"
  else
    echo "[FAIL] bridge_canonicalize_diff_on_off: mutated JSON lacks Method callee" >&2
    exit 1
  fi
else
  echo "[FAIL] bridge_canonicalize_diff_on_off: mutated dump (ON) not created" >&2
  exit 1
fi

# OFF case may not create a file at all; if it exists, it must keep ModuleFunction
if [ -f "$mut_off" ]; then
  if grep -q '"type":"ModuleFunction"' "$mut_off"; then
    echo "[PASS] bridge_canonicalize_diff_on_off (OFF kept ModuleFunction)"
  else
    echo "[FAIL] bridge_canonicalize_diff_on_off: OFF mutated JSON unexpected" >&2
    exit 1
  fi
else
  echo "[PASS] bridge_canonicalize_diff_on_off (OFF no mutation dump)"
fi

rm -f "$json_path" "$mut_on" "$mut_off"
exit 0
