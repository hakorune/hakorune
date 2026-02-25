#!/bin/bash
# phase29bs_loopframe_v1_nested_loop_strict_gate_vm.sh - LoopFrame v1 nested loop strict gate (strict/dev)

source "$(dirname "$0")/../../../lib/test_runner.sh"
require_env || exit 2

FIXTURE="$NYASH_ROOT/apps/tests/phase29bs_nested_loop_break_continue_depth_min.hako"
RUN_TIMEOUT_SECS=${RUN_TIMEOUT_SECS:-10}
expected=$(cat << 'TXT'
1
TXT
)

set +e
output=$(timeout "$RUN_TIMEOUT_SECS" env NYASH_DISABLE_PLUGINS=1 HAKO_JOINIR_STRICT=1 HAKO_JOINIR_PLANNER_REQUIRED=1 \
  "$NYASH_BIN" --backend vm "$FIXTURE" 2>&1)
exit_code=$?
set -e

if [ "$exit_code" -eq 124 ]; then
  log_error "phase29bs_loopframe_v1_nested_loop_strict_gate_vm: hakorune timed out (> ${RUN_TIMEOUT_SECS}s)"
  exit 1
fi

if [ "$exit_code" -ne 0 ]; then
  log_error "phase29bs_loopframe_v1_nested_loop_strict_gate_vm: expected exit code 0, got $exit_code"
  echo "$output"
  exit 1
fi

output_clean=$(echo "$output" | filter_noise | grep -v '^\[plugins\]' | grep -v '^\[WARN\] \[plugin/init\]' || true)
compare_outputs "$expected" "$output_clean" "phase29bs_loopframe_v1_nested_loop_strict_gate_vm" || exit 1

if ! grep -qF "[joinir/planner_first" <<<"$output"; then
  if ! grep -qF "[joinir/loopframe_v1" <<<"$output"; then
    echo "[FAIL] Missing planner tag ([joinir/planner_first...] or [joinir/loopframe_v1...])"
    echo "$output" | tail -n 40 || true
    test_fail "phase29bs_loopframe_v1_nested_loop_strict_gate_vm: Missing planner tag"
    exit 1
  fi
fi

log_success "phase29bs_loopframe_v1_nested_loop_strict_gate_vm: PASS (exit=$exit_code)"
