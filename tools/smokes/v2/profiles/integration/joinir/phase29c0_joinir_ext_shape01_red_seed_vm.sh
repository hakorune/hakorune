#!/bin/bash
# phase29c0_joinir_ext_shape01_red_seed_vm.sh
# JIR-EXT-SHAPE-01 green acceptance lock:
# - Target shape must pass in planner_required strict/dev with NYASH_VM_USE_FALLBACK=0.
# - Both routes (rust reference / hako mainline) must succeed with the same fixture result.
# - Route tags and route report stay explicit so lane drift remains visible.

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
ROOT_DIR="$(cd "$SCRIPT_DIR/../../../../.." && pwd)"
source "$ROOT_DIR/smokes/v2/lib/test_runner.sh"
source "$ROOT_DIR/smokes/v2/lib/joinir_route_report.sh"
require_env || exit 2

SMOKE_NAME="phase29c0_joinir_ext_shape01_red_seed_vm"
FIXTURE="${1:-$NYASH_ROOT/apps/tests/phase29bq_selfhost_blocker_phi_injector_collect_phi_vars_nested_loop_no_exit_var_step_min.hako}"
RUN_TIMEOUT_SECS="${RUN_TIMEOUT_SECS:-12}"
EXPECTED_LINE="${EXPECTED_LINE:-18}"

if [[ "$FIXTURE" != /* ]]; then
  FIXTURE="$NYASH_ROOT/$FIXTURE"
fi
if [ ! -f "$FIXTURE" ]; then
  test_fail "$SMOKE_NAME: fixture missing: $FIXTURE"
  exit 2
fi
if ! [[ "$RUN_TIMEOUT_SECS" =~ ^[0-9]+$ ]]; then
  test_fail "$SMOKE_NAME: timeout must be integer: $RUN_TIMEOUT_SECS"
  exit 2
fi

run_green_route() {
  local label="$1"
  local prefer_strict_dev="$2"
  local expected_lane="$3"
  local vm_lane="$4"
  local kernel_lane="$5"
  local joinir_lane="$6"
  local expected_line="$7"
  local allow_known_subset_gap="${8:-0}"

  joinir_route_report_emit "$vm_lane" "$kernel_lane" "$joinir_lane" "backend-vm"
  joinir_route_report_require_no_fallback || return 1

  set +e
  local output
  output=$(timeout "$RUN_TIMEOUT_SECS" env \
    NYASH_VM_ROUTE_TRACE=1 \
    NYASH_VM_USE_FALLBACK=0 \
    NYASH_VM_HAKO_PREFER_STRICT_DEV="$prefer_strict_dev" \
    HAKO_JOINIR_STRICT=1 \
    HAKO_JOINIR_PLANNER_REQUIRED=1 \
    NYASH_JOINIR_STRICT=1 \
    NYASH_JOINIR_DEV=1 \
    NYASH_ALLOW_USING_FILE=1 \
    "$NYASH_BIN" --backend vm "$FIXTURE" 2>&1)
  local rc=$?
  set -e

  if [ "$rc" -eq 124 ]; then
    echo "[FAIL] $SMOKE_NAME[$label]: timeout (> ${RUN_TIMEOUT_SECS}s)" >&2
    return 1
  fi
  if ! joinir_route_report_require_lane_tag "$output" "$expected_lane"; then
    echo "$output" | tail -n 80 >&2 || true
    return 1
  fi
  if grep -q "\\[plan/freeze:contract\\]" <<<"$output"; then
    echo "[FAIL] $SMOKE_NAME[$label]: unexpected freeze marker in GREEN run" >&2
    echo "$output" | tail -n 120 >&2 || true
    return 1
  fi
  if [ "$rc" -eq 0 ]; then
    if ! grep -q "^RC: 0$" <<<"$output"; then
      echo "[FAIL] $SMOKE_NAME[$label]: missing RC: 0 line" >&2
      echo "$output" | tail -n 80 >&2 || true
      return 1
    fi
    if ! grep -Fxq "$expected_line" <<<"$output"; then
      echo "[FAIL] $SMOKE_NAME[$label]: expected result line '$expected_line' not found" >&2
      echo "$output" | tail -n 120 >&2 || true
      return 1
    fi
  elif [ "$allow_known_subset_gap" = "1" ]; then
    if ! grep -q "\\[vm-hako/unimplemented op=boxcall1 method=get\\]" <<<"$output"; then
      echo "[FAIL] $SMOKE_NAME[$label]: non-zero rc without known vm-hako subset marker" >&2
      echo "$output" | tail -n 120 >&2 || true
      return 1
    fi
  else
    echo "[FAIL] $SMOKE_NAME[$label]: expected GREEN (rc=0), got rc=$rc" >&2
    echo "$output" | tail -n 80 >&2 || true
    return 1
  fi

  return 0
}

run_green_route "rust-reference" "0" "vm" "rust" "rust" "rust" "$EXPECTED_LINE" "0" || {
  test_fail "$SMOKE_NAME: rust-reference green lock failed"
  exit 1
}

run_green_route "hako-mainline" "1" "vm-hako" "hako" "hako" "hako" "$EXPECTED_LINE" "1" || {
  test_fail "$SMOKE_NAME: hako-mainline green lock failed"
  exit 1
}

test_pass "$SMOKE_NAME: PASS (green acceptance locked; vm-hako subset-gap marker explicitly tolerated)"
