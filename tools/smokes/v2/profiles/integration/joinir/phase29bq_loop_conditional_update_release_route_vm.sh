#!/bin/bash
# Phase 29bq BQ-A01: loop_cond_break_continue conditional-update release route
# manual release-route probe.
# Current canonical planner-required gate is `phase29bq_conditional_update_join_planner_required_vm.sh`.
#
# Contract pin:
# - In non-planner_required mode, conditional-update loop(cond) must lower via JoinIR (no freeze).

set -euo pipefail

source "$(dirname "$0")/../../../lib/test_runner.sh"
require_env || exit 2

INPUT="${1:-$NYASH_ROOT/apps/tests/phase29bq_loop_conditional_update_release_route_min.hako}"

if [ ! -f "$INPUT" ]; then
  test_fail "phase29bq_loop_conditional_update_release_route_vm: fixture missing: $INPUT"
  exit 2
fi

set +e
OUTPUT=$(NYASH_DISABLE_PLUGINS=1 \
  NYASH_VM_HAKO_PREFER_STRICT_DEV=0 \
  NYASH_JOINIR_DEV=0 \
  NYASH_JOINIR_STRICT=0 \
  HAKO_JOINIR_STRICT=0 \
  HAKO_JOINIR_PLANNER_REQUIRED=0 \
  run_nyash_vm "$INPUT")
RC=$?
set -e

if [ "$RC" -ne 0 ]; then
  echo "$OUTPUT" | tail -n 80 || true
  test_fail "phase29bq_loop_conditional_update_release_route_vm: expected rc=0, got $RC"
  exit 1
fi

compare_outputs "100" "$OUTPUT" "phase29bq_loop_conditional_update_release_route_vm" || exit 1

# Ensure no generic JoinIR freeze leaked in non-planner_required route.
if printf '%s\n' "$OUTPUT" | rg -q 'JoinIR does not support this pattern'; then
  test_fail "phase29bq_loop_conditional_update_release_route_vm: unexpected joinir freeze"
  exit 1
fi

test_pass "phase29bq_loop_conditional_update_release_route_vm: PASS (conditional-update release route)"
