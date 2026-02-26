#!/bin/bash
# phase29cc_wsm_g3_canvas_clear_contract_vm.sh
# Contract pin:
# - WSM-G3-min2 adds env.canvas.clear extern contract mapping.
# - runtime imports and JS bindings include canvas_clear + clearRect behavior.

set -euo pipefail

source "$(dirname "$0")/../../../lib/test_runner.sh"
require_env || exit 2

set +e
output1=$(cd "$NYASH_ROOT" && cargo test --features wasm-backend extern_contract_supported_name_maps_to_import -- --nocapture 2>&1)
rc1=$?
set -e

if [ "$rc1" -ne 0 ]; then
  test_fail "phase29cc_wsm_g3_canvas_clear_contract_vm: extern contract test failed (rc=$rc1)"
  printf '%s\n' "$output1" | sed -n '1,200p'
  exit 1
fi

set +e
output2=$(cd "$NYASH_ROOT" && cargo test --features wasm-backend test_js_import_object_generation -- --nocapture 2>&1)
rc2=$?
set -e

if [ "$rc2" -ne 0 ]; then
  test_fail "phase29cc_wsm_g3_canvas_clear_contract_vm: runtime JS import test failed (rc=$rc2)"
  printf '%s\n' "$output2" | sed -n '1,200p'
  exit 1
fi

test_pass "phase29cc_wsm_g3_canvas_clear_contract_vm: PASS (WSM-G3-min2 canvas.clear contract lock)"
