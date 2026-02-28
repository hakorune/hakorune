#!/bin/bash
# phase29cc_wsm_freeze_min1_route_policy_rust_native_env_vm.sh
# Contract pin:
# - WSM-Freeze-min1: NYASH_WASM_ROUTE_POLICY accepts default|rust_native and rejects legacy aliases.

set -euo pipefail

source "$(dirname "$0")/../../../lib/test_runner.sh"
require_env || exit 2

set +e
output=$(cd "$NYASH_ROOT" && cargo test --features wasm-backend wasm_route_policy_ -- --nocapture 2>&1)
rc=$?
set -e
if [ "$rc" -ne 0 ]; then
  test_fail "phase29cc_wsm_freeze_min1_route_policy_rust_native_env_vm: route-policy tests failed (rc=$rc)"
  printf '%s\n' "$output" | sed -n '1,200p'
  exit 1
fi

for marker in \
  "wasm_route_policy_defaults_to_default" \
  "wasm_route_policy_accepts_rust_native" \
  "wasm_route_policy_rejects_legacy_aliases"; do
  if ! printf '%s\n' "$output" | grep -q "$marker"; then
    test_fail "phase29cc_wsm_freeze_min1_route_policy_rust_native_env_vm: missing marker: $marker"
    printf '%s\n' "$output" | sed -n '1,200p'
    exit 1
  fi
done

test_pass "phase29cc_wsm_freeze_min1_route_policy_rust_native_env_vm: PASS (WSM-Freeze-min1 route policy freeze contract)"
