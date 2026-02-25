#!/bin/bash
# phase21_5_perf_kilo_aot_safepoint_toggle_contract_vm.sh
#
# Contract pin:
# - bench4 AOT lane respects PERF_AOT_AUTO_SAFEPOINT toggle.
# - On kilo_kernel_small, safepoint=0 must not be slower than safepoint=1.

set -euo pipefail

SMOKE_NAME="phase21_5_perf_kilo_aot_safepoint_toggle_contract_vm"
KEY="kilo_kernel_small"

source "$(dirname "$0")/../../../lib/test_runner.sh"
require_env || exit 2

SCRIPT="$NYASH_ROOT/tools/perf/bench_compare_c_py_vs_hako.sh"
if [ ! -f "$SCRIPT" ]; then
  test_fail "$SMOKE_NAME: missing script: $SCRIPT"
  exit 2
fi

run_case() {
  local safepoint="$1"
  PERF_AOT=1 \
  PERF_SKIP_VM_PREFLIGHT=1 \
  NYASH_LLVM_FAST=1 \
  NYASH_LLVM_SKIP_BUILD="${NYASH_LLVM_SKIP_BUILD:-1}" \
  PERF_AOT_AUTO_SAFEPOINT="$safepoint" \
  bash "$SCRIPT" "$KEY" 1 3
}

OUT_ON="$(run_case 1)"
OUT_OFF="$(run_case 0)"

if ! printf '%s\n' "$OUT_ON" | grep -q "\\[bench4\\] name=${KEY} "; then
  printf '%s\n' "$OUT_ON"
  test_fail "$SMOKE_NAME: missing bench4 line (safepoint=1)"
  exit 1
fi
if ! printf '%s\n' "$OUT_OFF" | grep -q "\\[bench4\\] name=${KEY} "; then
  printf '%s\n' "$OUT_OFF"
  test_fail "$SMOKE_NAME: missing bench4 line (safepoint=0)"
  exit 1
fi

if ! printf '%s\n' "$OUT_ON" | grep -q 'aot_status=ok'; then
  printf '%s\n' "$OUT_ON"
  test_fail "$SMOKE_NAME: aot_status not ok (safepoint=1)"
  exit 1
fi
if ! printf '%s\n' "$OUT_OFF" | grep -q 'aot_status=ok'; then
  printf '%s\n' "$OUT_OFF"
  test_fail "$SMOKE_NAME: aot_status not ok (safepoint=0)"
  exit 1
fi

AOT_ON="$(printf '%s\n' "$OUT_ON" | grep -oE 'ny_aot_ms=[0-9]+' | head -n1 | cut -d= -f2)"
AOT_OFF="$(printf '%s\n' "$OUT_OFF" | grep -oE 'ny_aot_ms=[0-9]+' | head -n1 | cut -d= -f2)"

if [ -z "$AOT_ON" ] || [ -z "$AOT_OFF" ]; then
  printf '%s\n' "$OUT_ON"
  printf '%s\n' "$OUT_OFF"
  test_fail "$SMOKE_NAME: failed to parse ny_aot_ms"
  exit 1
fi

if [ "$AOT_OFF" -gt "$AOT_ON" ]; then
  printf '%s\n' "$OUT_ON"
  printf '%s\n' "$OUT_OFF"
  test_fail "$SMOKE_NAME: expected safepoint=0 to be <= safepoint=1 (off=${AOT_OFF}, on=${AOT_ON})"
  exit 1
fi

test_pass "$SMOKE_NAME"
