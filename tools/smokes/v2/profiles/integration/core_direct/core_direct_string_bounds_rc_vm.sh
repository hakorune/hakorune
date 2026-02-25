#!/bin/bash
# core_direct_string_bounds_rc_vm.sh — Core direct path: substring bounds → non‑zero rc

set -euo pipefail
if [ "${SMOKES_ENABLE_CORE_DIRECT:-0}" != "1" ]; then
  echo "[SKIP] core_direct_string_bounds_rc_vm (SMOKES_ENABLE_CORE_DIRECT=1 to enable)"
  exit 0
fi
SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
if ROOT_GIT=$(git -C "$SCRIPT_DIR" rev-parse --show-toplevel 2>/dev/null); then
  ROOT="$ROOT_GIT"
else
  ROOT="$(cd "$SCRIPT_DIR/../../../../../../../../.." && pwd)"
fi
source "$ROOT/tools/smokes/v2/lib/test_runner.sh"
source "$ROOT/tools/smokes/v2/lib/stageb_helpers.sh"
require_env || exit 2

code='static box Main { method main(args) { local s="abc"; local t=s.substring(-1,1); return 0 } }'
json=$(stageb_compile_to_json "$code") || { echo "[FAIL] core_direct_string_bounds_rc_vm (emit failed)" >&2; exit 1; }

set +e
NYASH_GATE_C_CORE=1 HAKO_GATE_C_CORE=1 HAKO_CORE_DIRECT=1 HAKO_CORE_DIRECT_INPROC=1 \
  NYASH_QUIET=1 HAKO_QUIET=1 NYASH_CLI_VERBOSE=0 NYASH_NYRT_SILENT_RESULT=1 \
  "$NYASH_BIN" --json-file "$json" >/dev/null 2>&1
rc=$?
set -e
rm -f "$json"

if [ "$rc" -ne 0 ]; then
  echo "[PASS] core_direct_string_bounds_rc_vm"
else
  echo "[FAIL] core_direct_string_bounds_rc_vm (rc=$rc)" >&2; exit 1
fi
