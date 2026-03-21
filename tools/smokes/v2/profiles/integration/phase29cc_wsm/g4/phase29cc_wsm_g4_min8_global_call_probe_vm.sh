#!/bin/bash
# phase29cc_wsm_g4_min8_global_call_probe_vm.sh
# Contract pin:
# - WSM-G4-min8: global call native box lock

set -euo pipefail

source "$(dirname "$0")/../../../../lib/test_runner.sh"
require_env || exit 2

doc="$NYASH_ROOT/docs/development/current/main/phases/phase-29cc/29cc-205-wsm-g4-min8-global-call-native-box-lock-ssot.md"
fixture="$NYASH_ROOT/apps/tests/phase29cc_wsm_g4_min8_global_call_probe_min.hako"

if [ ! -f "$doc" ]; then
  test_fail "phase29cc_wsm_g4_min8_global_call_probe_vm: lock doc missing"
  exit 1
fi

for needle in \
  "WSM-G4-min8" \
  "Callee::Global" \
  "compile-to-wat"; do
  if ! grep -Fq "$needle" "$doc"; then
    test_fail "phase29cc_wsm_g4_min8_global_call_probe_vm: missing keyword in lock doc: $needle"
    exit 1
  fi
done

if [ ! -f "$fixture" ]; then
  test_fail "phase29cc_wsm_g4_min8_global_call_probe_vm: fixture missing"
  exit 1
fi

set +e
output=$(cd "$NYASH_ROOT" && cargo test --features wasm-backend wasm_demo_g4_min8_global_call_probe_compile_to_wat_contract -- --nocapture 2>&1)
rc=$?
set -e
if [ "$rc" -ne 0 ]; then
  test_fail "phase29cc_wsm_g4_min8_global_call_probe_vm: cargo test failed (rc=$rc)"
  printf '%s\n' "$output" | sed -n '1,220p'
  exit 1
fi
if ! printf '%s\n' "$output" | grep -q "wasm_demo_g4_min8_global_call_probe_compile_to_wat_contract"; then
  test_fail "phase29cc_wsm_g4_min8_global_call_probe_vm: expected test marker missing"
  printf '%s\n' "$output" | sed -n '1,220p'
  exit 1
fi

test_pass "phase29cc_wsm_g4_min8_global_call_probe_vm: PASS (WSM-G4-min8 global call native box lock)"
