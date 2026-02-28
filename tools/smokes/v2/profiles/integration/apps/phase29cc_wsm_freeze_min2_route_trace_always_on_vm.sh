#!/bin/bash
# phase29cc_wsm_freeze_min2_route_trace_always_on_vm.sh
# Contract pin:
# - WSM-Freeze-min2: compile-wasm emits route-trace line without NYASH_WASM_ROUTE_TRACE.

set -euo pipefail

source "$(dirname "$0")/../../../lib/test_runner.sh"
require_env || exit 2

set +e
output=$(cd "$NYASH_ROOT" && cargo test --features wasm-backend wasm_demo_route_trace_is_emitted_without_trace_env_contract -- --nocapture 2>&1)
rc=$?
set -e
if [ "$rc" -ne 0 ]; then
  test_fail "phase29cc_wsm_freeze_min2_route_trace_always_on_vm: route-trace always-on test failed (rc=$rc)"
  printf '%s\n' "$output" | sed -n '1,200p'
  exit 1
fi

if ! printf '%s\n' "$output" | grep -q "wasm_demo_route_trace_is_emitted_without_trace_env_contract"; then
  test_fail "phase29cc_wsm_freeze_min2_route_trace_always_on_vm: expected test marker missing"
  printf '%s\n' "$output" | sed -n '1,200p'
  exit 1
fi

set +e
output_forced=$(cd "$NYASH_ROOT" && cargo test --features wasm-backend wasm_demo_route_trace_reports_rust_native_forced_contract -- --nocapture 2>&1)
rc_forced=$?
set -e
if [ "$rc_forced" -ne 0 ]; then
  test_fail "phase29cc_wsm_freeze_min2_route_trace_always_on_vm: forced rust_native trace test failed (rc=$rc_forced)"
  printf '%s\n' "$output_forced" | sed -n '1,200p'
  exit 1
fi

if ! printf '%s\n' "$output_forced" | grep -q "wasm_demo_route_trace_reports_rust_native_forced_contract"; then
  test_fail "phase29cc_wsm_freeze_min2_route_trace_always_on_vm: missing forced route marker"
  printf '%s\n' "$output_forced" | sed -n '1,200p'
  exit 1
fi

test_pass "phase29cc_wsm_freeze_min2_route_trace_always_on_vm: PASS (WSM-Freeze-min2 route trace always-on contract)"
