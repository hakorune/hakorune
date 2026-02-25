#!/bin/bash
# stageb_print_vm.sh — Stage‑B: print positive case (emit→Gate‑C direct)

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

code='static box Main { method main(args) { print(3); return 0; } }'
# Compile via Stage‑B entry（emit only; no VM run here; fallback禁止）
json=$(stageb_compile_to_json "$code") || { echo "[FAIL] Stage‑B emit failed (direct)" >&2; exit 1; }
if stageb_json_nonempty "$json"; then
  # Execute via Gate‑C(Core) and expect rc=0 (return 0 after print)
  stageb_gatec_expect_rc "$json" 0 || { rm -f "$json"; exit 1; }
  rm -f "$json"
  echo "[PASS] stageb_print_vm"
  exit 0
else
  echo "[FAIL] stageb_print_vm (emit json missing header)" >&2
  test -f "$json" && { echo "--- json ---" >&2; head -n1 "$json" >&2; }
  rm -f "$json"
  exit 1
fi
