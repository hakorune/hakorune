#!/bin/bash
# phase29cc_wsm_p1_emit_wat_cli_vm.sh
# Contract pin:
# - P1-min1: --emit-wat CLI route exists and argument contract is fixed.
# - emit-wat parsing and conflict boundary are covered by unit tests.

set -euo pipefail

source "$(dirname "$0")/../../../lib/test_runner.sh"
require_env || exit 2

set +e
output1=$(cd "$NYASH_ROOT" && cargo test emit_wat_route_parses_and_sets_output_path -- --nocapture 2>&1)
rc=$?
set -e

if [ "$rc" -ne 0 ]; then
  test_fail "phase29cc_wsm_p1_emit_wat_cli_vm: parse contract test failed (rc=$rc)"
  printf '%s\n' "$output1" | sed -n '1,160p'
  exit 1
fi

set +e
output2=$(cd "$NYASH_ROOT" && cargo test emit_wat_conflicts_with_compile_wasm -- --nocapture 2>&1)
rc=$?
set -e

if [ "$rc" -ne 0 ]; then
  test_fail "phase29cc_wsm_p1_emit_wat_cli_vm: conflict contract test failed (rc=$rc)"
  printf '%s\n' "$output2" | sed -n '1,160p'
  exit 1
fi

if ! printf '%s\n' "$output1" | grep -q "emit_wat_route_parses_and_sets_output_path"; then
  test_fail "phase29cc_wsm_p1_emit_wat_cli_vm: parse contract marker missing"
  exit 1
fi

if ! printf '%s\n' "$output2" | grep -q "emit_wat_conflicts_with_compile_wasm"; then
  test_fail "phase29cc_wsm_p1_emit_wat_cli_vm: conflict contract marker missing"
  exit 1
fi

test_pass "phase29cc_wsm_p1_emit_wat_cli_vm: PASS (WSM-P1-min1 --emit-wat CLI lock)"
