#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../../../.." && pwd)"
source "$ROOT/tools/smokes/v2/lib/test_runner.sh"
require_env >/dev/null || exit 2

SOURCE="$ROOT/lang/c-abi/shims/hako_llvmc_ffi.c"

require_line() {
  local pattern="$1"
  if ! grep -Fq "$pattern" "$SOURCE"; then
    echo "[FAIL] phase29ck_boundary_fallback_inventory_probe: missing source line: $pattern" >&2
    exit 1
  fi
}

require_line "static int compile_json_via_default_forwarder("
require_line "static int compile_json_via_explicit_compat_harness_replay("
require_line "static int compile_json_via_pure_first_lane("
require_line "return compile_json_via_default_forwarder(json_in, obj_out, err_out);"
require_line "return compile_json_via_pure_first_lane(json_in, obj_out, err_out);"
require_line "return compile_json_via_explicit_compat_harness_replay(json_in, obj_out, err_out);"

raw_replay_calls=$(
  rg -n "return compile_json_compat_harness_keep\\(json_in, obj_out, err_out\\);" "$SOURCE" | wc -l | tr -d ' '
)
if [ "$raw_replay_calls" -ne 1 ]; then
  echo "[FAIL] phase29ck_boundary_fallback_inventory_probe: expected exactly one raw compat replay call, got $raw_replay_calls" >&2
  exit 1
fi

echo "[PASS] phase29ck_boundary_fallback_inventory_probe"
