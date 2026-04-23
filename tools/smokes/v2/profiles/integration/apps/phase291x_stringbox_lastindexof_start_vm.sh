#!/bin/bash
# Phase 291x: vm-hako StringBox.lastIndexOf(needle, start_pos) acceptance smoke.
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
if ROOT_GIT=$(git -C "$SCRIPT_DIR" rev-parse --show-toplevel 2>/dev/null); then
  ROOT="$ROOT_GIT"
else
  ROOT="$(cd "$SCRIPT_DIR/../../../../../../.." && pwd)"
fi

source "$ROOT/tools/smokes/v2/lib/test_runner.sh"
source "$ROOT/tools/smokes/v2/profiles/integration/vm_hako_caps/lib/vm_hako_caps_common.sh"
require_env || exit 2

SMOKE_NAME="phase291x_stringbox_lastindexof_start_vm"
INPUT="${1:-$ROOT/apps/tests/phase291x_stringbox_lastindexof_start_min.hako}"
RUN_TIMEOUT_SECS="${RUN_TIMEOUT_SECS:-30}"

vm_hako_caps_require_fixture "$SMOKE_NAME" "$INPUT" || exit 1
vm_hako_caps_run_vm_hako_or_fail_timeout "$SMOKE_NAME" "$RUN_TIMEOUT_SECS" "$INPUT" || exit 1

OUTPUT_CLEAN=$(printf '%s\n' "$VM_HAKO_CAPS_OUTPUT" | filter_noise || true)

if printf '%s\n' "$OUTPUT_CLEAN" | rg -q '^\[vm-hako/unimplemented|^\[vm-hako/contract'; then
  echo "$OUTPUT_CLEAN" | tail -n 120 || true
  test_fail "$SMOKE_NAME: unexpected vm-hako diagnostic tag"
  exit 1
fi

ACTUAL=$(printf '%s\n' "$OUTPUT_CLEAN" | rg '^(4|2|-1|3)$' || true)
EXPECTED=$(cat <<'EXPECT'
4
2
-1
3
EXPECT
)

if [ "$ACTUAL" != "$EXPECTED" ]; then
  echo "$OUTPUT_CLEAN" | tail -n 120 || true
  echo "[FAIL] expected:" >&2
  printf '%s\n' "$EXPECTED" >&2
  echo "[FAIL] actual:" >&2
  printf '%s\n' "$ACTUAL" >&2
  test_fail "$SMOKE_NAME: output mismatch"
  exit 1
fi

if [ "$VM_HAKO_CAPS_EXIT_CODE" -ne 0 ]; then
  echo "$OUTPUT_CLEAN" | tail -n 120 || true
  test_fail "$SMOKE_NAME: expected RC=0 (got rc=$VM_HAKO_CAPS_EXIT_CODE)"
  exit 1
fi

test_pass "$SMOKE_NAME: PASS"
