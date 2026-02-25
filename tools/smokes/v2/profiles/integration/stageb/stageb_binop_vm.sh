#!/bin/bash
# stageb_binop_vm.sh — Stage‑B: binop positive (return 1+2 → rc=3)

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

code='static box Main { method main(args) { return 1+2 } }'
json=$(stageb_compile_to_json "$code") || { echo "[FAIL] Stage‑B emit failed (direct)" >&2; exit 1; }
if stageb_json_nonempty "$json"; then
  # Execute via Gate‑C(Core) and expect rc=3
  stageb_gatec_expect_rc "$json" 3 || { rm -f "$json"; exit 1; }
  rm -f "$json"; echo "[PASS] stageb_binop_vm"; exit 0
else
  echo "[FAIL] stageb_binop_vm (emit json missing header)" >&2
  test -f "$json" && { echo "--- json ---" >&2; head -n1 "$json" >&2; }
  rm -f "$json"; exit 1
fi
