#!/bin/bash
# phase29cc_wsm02d_demo_min_boundary_vm.sh
# Contract pin:
# - WSM-02d-min2 demo-min fixture compiles to WAT on wasm-backend feature lane.
# - nyash-wasm minimum console family (log/warn/error/info/debug) stays supported.

set -euo pipefail

source "$(dirname "$0")/../../../lib/test_runner.sh"
require_env || exit 2

FIXTURE="$NYASH_ROOT/apps/tests/phase29cc_wsm02d_demo_min.hako"

if [ ! -f "$FIXTURE" ]; then
  test_fail "phase29cc_wsm02d_demo_min_boundary_vm: fixture missing: $FIXTURE"
  exit 2
fi

set +e
output=$(cd "$NYASH_ROOT" && cargo test --features wasm-backend wasm_demo_min_fixture_compile_to_wat_contract -- --nocapture 2>&1)
rc=$?
set -e

if [ "$rc" -ne 0 ]; then
  test_fail "phase29cc_wsm02d_demo_min_boundary_vm: cargo test failed (rc=$rc)"
  printf '%s\n' "$output" | sed -n '1,200p'
  exit 1
fi

if ! printf '%s\n' "$output" | grep -q "wasm_demo_min_fixture_compile_to_wat_contract"; then
  test_fail "phase29cc_wsm02d_demo_min_boundary_vm: expected test marker missing"
  printf '%s\n' "$output" | sed -n '1,120p'
  exit 1
fi

test_pass "phase29cc_wsm02d_demo_min_boundary_vm: PASS (WSM-02d-min2 demo-min fixture lock)"
