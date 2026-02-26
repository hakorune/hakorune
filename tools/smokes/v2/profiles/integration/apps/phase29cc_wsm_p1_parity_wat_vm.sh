#!/bin/bash
# phase29cc_wsm_p1_parity_wat_vm.sh
# Contract pin:
# - WSM-P1-min2: fixture-level WAT parity between direct compile_to_wat and --emit-wat CLI.

set -euo pipefail

source "$(dirname "$0")/../../../lib/test_runner.sh"
require_env || exit 2

set +e
output=$(cd "$NYASH_ROOT" && cargo test --features wasm-backend wasm_demo_min_fixture_emit_wat_parity_contract -- --nocapture 2>&1)
rc=$?
set -e

if [ "$rc" -ne 0 ]; then
  test_fail "phase29cc_wsm_p1_parity_wat_vm: parity test failed (rc=$rc)"
  printf '%s\n' "$output" | sed -n '1,200p'
  exit 1
fi

if ! printf '%s\n' "$output" | grep -q "wasm_demo_min_fixture_emit_wat_parity_contract"; then
  test_fail "phase29cc_wsm_p1_parity_wat_vm: expected test marker missing"
  printf '%s\n' "$output" | sed -n '1,120p'
  exit 1
fi

test_pass "phase29cc_wsm_p1_parity_wat_vm: PASS (WSM-P1-min2 fixture WAT parity lock)"
