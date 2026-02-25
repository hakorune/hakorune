#!/bin/bash
# gate_c_invalid_header_vm.sh — Gate‑C(Core) invalid JSON header → 非0終了

set -euo pipefail
SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
if ROOT_GIT=$(git -C "$SCRIPT_DIR" rev-parse --show-toplevel 2>/dev/null); then
  ROOT="$ROOT_GIT"
else
  ROOT="$(cd "$SCRIPT_DIR/../../../../../../.." && pwd)"
fi
source "$ROOT/tools/smokes/v2/lib/test_runner.sh"
require_env || exit 2

bad="/tmp/gatec_bad_$$.json"
echo '{"bad":1}' > "$bad"
set +e
NYASH_GATE_C_CORE=1 "$NYASH_BIN" --nyvm-json-file "$bad" >/dev/null 2>&1
rc=$?
set -e
rm -f "$bad"
if [ $rc -ne 0 ]; then
  echo "[PASS] gate_c_invalid_header_vm"
else
  echo "[FAIL] gate_c_invalid_header_vm (rc=$rc)" >&2
  exit 1
fi

