#!/bin/bash
# phase29cc_wsm_g3_canvas_strokerect_contract_vm.sh
# Contract pin:
# - WSM-G3-min3 adds env.canvas.strokeRect extern contract mapping.
# - runtime JS binding includes canvas_strokeRect + strokeStyle/strokeRect usage.

set -euo pipefail

source "$(dirname "$0")/../../../lib/test_runner.sh"
require_env || exit 2

set +e
output1=$(cd "$NYASH_ROOT" && cargo test --features wasm-backend extern_contract_canvas_stroke_rect_supported -- --nocapture 2>&1)
rc1=$?
set -e

if [ "$rc1" -ne 0 ]; then
  test_fail "phase29cc_wsm_g3_canvas_strokerect_contract_vm: extern contract test failed (rc=$rc1)"
  printf '%s\n' "$output1" | sed -n '1,200p'
  exit 1
fi

set +e
output2=$(cd "$NYASH_ROOT" && cargo test --features wasm-backend runtime_imports_canvas_stroke_rect_js_binding -- --nocapture 2>&1)
rc2=$?
set -e

if [ "$rc2" -ne 0 ]; then
  test_fail "phase29cc_wsm_g3_canvas_strokerect_contract_vm: runtime JS import test failed (rc=$rc2)"
  printf '%s\n' "$output2" | sed -n '1,200p'
  exit 1
fi

test_pass "phase29cc_wsm_g3_canvas_strokerect_contract_vm: PASS (WSM-G3-min3 canvas.strokeRect contract lock)"
