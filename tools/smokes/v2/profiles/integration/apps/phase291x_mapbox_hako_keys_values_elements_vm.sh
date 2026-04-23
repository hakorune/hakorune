#!/bin/bash
# Phase 291x slice 3: vm-hako MapBox keys()/values() element publication acceptance smoke.
# Pins: keys().size(), keys().get(0), keys().get(1),
#       values().size(), values().get(0), values().get(1)
# plus keys().get(0) after values() to ensure result arrays do not overwrite
# each other's element state for a deterministic two-entry map ("a"->1, "b"->2).
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

SMOKE_NAME="phase291x_mapbox_hako_keys_values_elements_vm"
INPUT="${1:-$ROOT/apps/tests/phase291x_mapbox_hako_keys_values_elements_min.hako}"
RUN_TIMEOUT_SECS="${RUN_TIMEOUT_SECS:-30}"

vm_hako_caps_require_fixture "$SMOKE_NAME" "$INPUT" || exit 1

vm_hako_caps_run_vm_hako_or_fail_timeout "$SMOKE_NAME" "$RUN_TIMEOUT_SECS" "$INPUT" || exit 1

OUTPUT_CLEAN=$(printf '%s\n' "$VM_HAKO_CAPS_OUTPUT" | filter_noise || true)

if printf '%s\n' "$OUTPUT_CLEAN" | rg -q '^\[vm-hako/unimplemented'; then
  echo "$OUTPUT_CLEAN" | tail -n 120 || true
  test_fail "$SMOKE_NAME: unexpected vm-hako unimplemented tag"
  exit 1
fi
if printf '%s\n' "$OUTPUT_CLEAN" | rg -q '^\[vm-hako/contract'; then
  echo "$OUTPUT_CLEAN" | tail -n 120 || true
  test_fail "$SMOKE_NAME: unexpected vm-hako contract tag"
  exit 1
fi

LINES=$(printf '%s\n' "$OUTPUT_CLEAN" | rg -c . || echo "0")
if [ "$LINES" -ne 7 ]; then
  echo "$OUTPUT_CLEAN" | tail -n 120 || true
  test_fail "$SMOKE_NAME: expected exactly 7 output lines, got $LINES"
  exit 1
fi

LINE1=$(printf '%s\n' "$OUTPUT_CLEAN" | sed -n '1p')
LINE2=$(printf '%s\n' "$OUTPUT_CLEAN" | sed -n '2p')
LINE3=$(printf '%s\n' "$OUTPUT_CLEAN" | sed -n '3p')
LINE4=$(printf '%s\n' "$OUTPUT_CLEAN" | sed -n '4p')
LINE5=$(printf '%s\n' "$OUTPUT_CLEAN" | sed -n '5p')
LINE6=$(printf '%s\n' "$OUTPUT_CLEAN" | sed -n '6p')
LINE7=$(printf '%s\n' "$OUTPUT_CLEAN" | sed -n '7p')

if [ "$LINE1" != "2" ]; then
  echo "$OUTPUT_CLEAN" | tail -n 120 || true
  test_fail "$SMOKE_NAME: expected keys().size()=2 on line 1, got '$LINE1'"
  exit 1
fi
if [ "$LINE2" != "a" ]; then
  echo "$OUTPUT_CLEAN" | tail -n 120 || true
  test_fail "$SMOKE_NAME: expected keys().get(0)=a on line 2, got '$LINE2'"
  exit 1
fi
if [ "$LINE3" != "b" ]; then
  echo "$OUTPUT_CLEAN" | tail -n 120 || true
  test_fail "$SMOKE_NAME: expected keys().get(1)=b on line 3, got '$LINE3'"
  exit 1
fi
if [ "$LINE4" != "2" ]; then
  echo "$OUTPUT_CLEAN" | tail -n 120 || true
  test_fail "$SMOKE_NAME: expected values().size()=2 on line 4, got '$LINE4'"
  exit 1
fi
if [ "$LINE5" != "1" ]; then
  echo "$OUTPUT_CLEAN" | tail -n 120 || true
  test_fail "$SMOKE_NAME: expected values().get(0)=1 on line 5, got '$LINE5'"
  exit 1
fi
if [ "$LINE6" != "2" ]; then
  echo "$OUTPUT_CLEAN" | tail -n 120 || true
  test_fail "$SMOKE_NAME: expected values().get(1)=2 on line 6, got '$LINE6'"
  exit 1
fi
if [ "$LINE7" != "a" ]; then
  echo "$OUTPUT_CLEAN" | tail -n 120 || true
  test_fail "$SMOKE_NAME: expected keys().get(0)=a after values() on line 7, got '$LINE7'"
  exit 1
fi

if [ "$VM_HAKO_CAPS_EXIT_CODE" -ne 0 ]; then
  echo "$OUTPUT_CLEAN" | tail -n 120 || true
  test_fail "$SMOKE_NAME: expected RC=0 (got rc=$VM_HAKO_CAPS_EXIT_CODE)"
  exit 1
fi

test_pass "$SMOKE_NAME: PASS (MapBox keys()/values() element publication pinned)"
