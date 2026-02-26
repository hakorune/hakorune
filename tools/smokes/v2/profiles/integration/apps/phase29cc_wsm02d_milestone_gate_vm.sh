#!/bin/bash
# phase29cc_wsm02d_milestone_gate_vm.sh
# Contract pin:
# - WSM-02d boundary pack runs as milestone gate (min1/min2/min3).

set -euo pipefail

source "$(dirname "$0")/../../../lib/test_runner.sh"
require_env || exit 2

set +e
output_min2=$(bash "$NYASH_ROOT/tools/smokes/v2/profiles/integration/apps/phase29cc_wsm02d_demo_min_boundary_vm.sh" 2>&1)
rc_min2=$?
set -e
if [ "$rc_min2" -ne 0 ]; then
  test_fail "phase29cc_wsm02d_milestone_gate_vm: min2 failed (rc=$rc_min2)"
  printf '%s\n' "$output_min2" | sed -n '1,200p'
  exit 1
fi

set +e
output_min3=$(bash "$NYASH_ROOT/tools/smokes/v2/profiles/integration/apps/phase29cc_wsm02d_demo_unsupported_boundary_vm.sh" 2>&1)
rc_min3=$?
set -e
if [ "$rc_min3" -ne 0 ]; then
  test_fail "phase29cc_wsm02d_milestone_gate_vm: min3 failed (rc=$rc_min3)"
  printf '%s\n' "$output_min3" | sed -n '1,200p'
  exit 1
fi

set +e
unit_output_1=$(cd "$NYASH_ROOT" && cargo test --features wasm-backend extern_contract_supported_name_maps_to_import -- --nocapture 2>&1)
unit_rc_1=$?
set -e
if [ "$unit_rc_1" -ne 0 ]; then
  test_fail "phase29cc_wsm02d_milestone_gate_vm: extern contract unit failed (rc=$unit_rc_1)"
  printf '%s\n' "$unit_output_1" | sed -n '1,200p'
  exit 1
fi

set +e
unit_output_2=$(cd "$NYASH_ROOT" && cargo test --features wasm-backend test_unsupported_extern_call_fails_fast_with_supported_list -- --nocapture 2>&1)
unit_rc_2=$?
set -e
if [ "$unit_rc_2" -ne 0 ]; then
  test_fail "phase29cc_wsm02d_milestone_gate_vm: unsupported extern unit failed (rc=$unit_rc_2)"
  printf '%s\n' "$unit_output_2" | sed -n '1,200p'
  exit 1
fi

set +e
unit_output_3=$(cd "$NYASH_ROOT" && cargo test --features wasm-backend test_unsupported_boxcall_method_fails_fast_with_supported_list -- --nocapture 2>&1)
unit_rc_3=$?
set -e
if [ "$unit_rc_3" -ne 0 ]; then
  test_fail "phase29cc_wsm02d_milestone_gate_vm: unsupported boxcall unit failed (rc=$unit_rc_3)"
  printf '%s\n' "$unit_output_3" | sed -n '1,200p'
  exit 1
fi

test_pass "phase29cc_wsm02d_milestone_gate_vm: PASS (WSM-02d milestone gate pack locked)"
