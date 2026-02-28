#!/bin/bash
# phase29cc_wsm_freeze_min3_route_policy_scope_emit_wat_vm.sh
# Contract pin:
# - WSM-Freeze-min3: rust_native policy is compile-wasm only; emit-wat must fail-fast.

set -euo pipefail

source "$(dirname "$0")/../../../lib/test_runner.sh"
require_env || exit 2

set +e
output=$(cd "$NYASH_ROOT" && cargo test --features wasm-backend wasm_demo_emit_wat_rejects_rust_native_policy_scope_contract -- --nocapture 2>&1)
rc=$?
set -e
if [ "$rc" -ne 0 ]; then
  test_fail "phase29cc_wsm_freeze_min3_route_policy_scope_emit_wat_vm: emit-wat policy-scope test failed (rc=$rc)"
  printf '%s\n' "$output" | sed -n '1,200p'
  exit 1
fi

if ! printf '%s\n' "$output" | grep -q "wasm_demo_emit_wat_rejects_rust_native_policy_scope_contract"; then
  test_fail "phase29cc_wsm_freeze_min3_route_policy_scope_emit_wat_vm: expected test marker missing"
  printf '%s\n' "$output" | sed -n '1,200p'
  exit 1
fi

test_pass "phase29cc_wsm_freeze_min3_route_policy_scope_emit_wat_vm: PASS (WSM-Freeze-min3 rust_native compile-wasm-only lock)"
