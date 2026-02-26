#!/bin/bash
# phase29cc_wsm_p2_min1_bridge_lock_vm.sh
# Contract pin:
# - WSM-P2-min1: wat2wasm bridge lock (normal/boundary/error) for compile-wasm route.

set -euo pipefail

source "$(dirname "$0")/../../../lib/test_runner.sh"
require_env || exit 2

set +e
output=$(cd "$NYASH_ROOT" && cargo test --features wasm-backend wasm_demo_ -- --nocapture 2>&1)
rc=$?
set -e

if [ "$rc" -ne 0 ]; then
  test_fail "phase29cc_wsm_p2_min1_bridge_lock_vm: bridge contract tests failed (rc=$rc)"
  printf '%s\n' "$output" | sed -n '1,220p'
  exit 1
fi

for marker in \
  "wasm_demo_min_fixture_compile_to_wasm_contract" \
  "wasm_demo_min_fixture_compile_wasm_cli_emits_wasm_contract" \
  "wasm_demo_wat2wasm_ascii_guard_contract" \
  "wasm_demo_wat2wasm_invalid_wat_contract"; do
  if ! printf '%s\n' "$output" | grep -q "$marker"; then
    test_fail "phase29cc_wsm_p2_min1_bridge_lock_vm: expected marker missing: $marker"
    printf '%s\n' "$output" | sed -n '1,220p'
    exit 1
  fi
done

test_pass "phase29cc_wsm_p2_min1_bridge_lock_vm: PASS (WSM-P2-min1 wat2wasm bridge lock)"
