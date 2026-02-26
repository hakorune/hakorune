#!/bin/bash
# phase29cc_wsm_g3_canvas_setstrokestyle_contract_vm.sh
# Contract pin:
# - WSM-G3-min9 adds env.canvas.setStrokeStyle extern contract mapping.
# - runtime JS binding includes canvas_setStrokeStyle + ctx.strokeStyle assignment.

set -euo pipefail

source "$(dirname "$0")/../../../lib/test_runner.sh"
source "$(dirname "$0")/../../../lib/wasm_g3_contract.sh"
require_env || exit 2

run_wasm_g3_contract_smoke \
  "phase29cc_wsm_g3_canvas_setstrokestyle_contract_vm" \
  "extern_contract_canvas_set_stroke_style_supported" \
  "runtime_imports_canvas_set_stroke_style_js_binding" \
  "phase29cc_wsm_g3_canvas_setstrokestyle_contract_vm: PASS (WSM-G3-min9 canvas.setStrokeStyle contract lock)"
