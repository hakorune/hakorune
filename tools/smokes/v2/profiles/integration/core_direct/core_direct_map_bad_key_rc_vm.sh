#!/bin/bash
# core_direct_map_bad_key_rc_vm.sh — Core Direct: Map.get with non-string key → non‑zero rc

set -euo pipefail
SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
if ROOT_GIT=$(git -C "$SCRIPT_DIR" rev-parse --show-toplevel 2>/dev/null); then
  ROOT="$ROOT_GIT"
else
  ROOT="$(cd "$SCRIPT_DIR/../../../../../../../../.." && pwd)"
fi
source "$ROOT/tools/smokes/v2/lib/test_runner.sh"
source "$ROOT/tools/smokes/v2/lib/stageb_helpers.sh"
require_env || exit 2

# Map.get requires string key; using int key should raise [map/bad-key] and map to rc != 0
code='static box Main { method main(args) { local m=MapBox(); local x=m.get(123); return 0 } }'
json=$(stageb_compile_to_json "$code") || { echo "[FAIL] core_direct_map_bad_key_rc_vm (emit failed)" >&2; exit 1; }

set +e
NYASH_GATE_C_CORE=1 HAKO_GATE_C_CORE=1 HAKO_CORE_DIRECT=1 \
  NYASH_QUIET=1 HAKO_QUIET=1 NYASH_CLI_VERBOSE=0 NYASH_NYRT_SILENT_RESULT=1 \
  "$NYASH_BIN" --json-file "$json" >/dev/null 2>&1
rc=$?
set -e
rm -f "$json"

if [ "$rc" -ne 0 ]; then
  echo "[PASS] core_direct_map_bad_key_rc_vm"
else
  echo "[FAIL] core_direct_map_bad_key_rc_vm (rc=$rc)" >&2; exit 1
fi

