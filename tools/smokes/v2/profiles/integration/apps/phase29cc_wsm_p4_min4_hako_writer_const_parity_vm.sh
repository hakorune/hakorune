#!/bin/bash
# phase29cc_wsm_p4_min4_hako_writer_const_parity_vm.sh
# Contract pin:
# - WSM-P4-min4: const-return fixture parity between compile_module output and binary writer baseline.

set -euo pipefail

source "$(dirname "$0")/../../../lib/test_runner.sh"
require_env || exit 2

set +e
output=$(cd "$NYASH_ROOT" && cargo test --features wasm-backend wasm_demo_min_const_return_binary_writer_parity_contract -- --nocapture 2>&1)
rc=$?
set -e

if [ "$rc" -ne 0 ]; then
  test_fail "phase29cc_wsm_p4_min4_hako_writer_const_parity_vm: const parity contract test failed (rc=$rc)"
  printf '%s\n' "$output" | sed -n '1,220p'
  exit 1
fi

if ! printf '%s\n' "$output" | grep -q "wasm_demo_min_const_return_binary_writer_parity_contract"; then
  test_fail "phase29cc_wsm_p4_min4_hako_writer_const_parity_vm: expected marker missing"
  printf '%s\n' "$output" | sed -n '1,220p'
  exit 1
fi

test_pass "phase29cc_wsm_p4_min4_hako_writer_const_parity_vm: PASS (WSM-P4-min4 const parity lock)"
