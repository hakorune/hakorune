#!/bin/bash
# gate_c_oob_write_file_vm.sh — Gate‑C(Core): array OOB write should fail (file mode)
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

if [ "${SMOKES_ENABLE_OOB_FILE:-0}" != "1" ]; then
  echo "[SKIP] SMOKES_ENABLE_OOB_FILE!=1; skipping OOB file write canary" >&2
  exit 0
fi

code='box Main { static method main() { local a=[1,2]; a[5]=9; return 0; } }'
json=$(stageb_compile_to_json "$code") || { echo "[FAIL] stageb emit failed" >&2; exit 1; }
if [ ! -s "$json" ]; then echo "[FAIL] stageb json empty" >&2; exit 1; fi
set +e
HAKO_OOB_STRICT=1 HAKO_OOB_STRICT_FAIL=1 NYASH_QUIET=1 HAKO_QUIET=1 NYASH_CLI_VERBOSE=0 \
  "$NYASH_BIN" --json-file "$json" >/dev/null 2>&1
rc=$?
set -e
rm -f "$json"
if [ $rc -ne 0 ]; then echo "[PASS] gate_c_oob_write_file_vm"; else echo "[FAIL] gate_c_oob_write_file_vm rc=$rc" >&2; exit 1; fi
