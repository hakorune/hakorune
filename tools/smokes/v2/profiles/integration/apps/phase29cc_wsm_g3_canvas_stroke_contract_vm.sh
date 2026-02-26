#!/bin/bash
# phase29cc_wsm_g3_canvas_stroke_contract_vm.sh
# Contract pin:
# - WSM-G3-min7 adds env.canvas.stroke extern contract mapping.
# - runtime JS binding includes canvas_stroke + ctx.stroke usage.

set -euo pipefail

source "$(dirname "$0")/../../../lib/test_runner.sh"
source "$(dirname "$0")/../../../lib/wasm_g3_contract.sh"
require_env || exit 2

run_wasm_g3_contract_smoke \
  "phase29cc_wsm_g3_canvas_stroke_contract_vm" \
  "extern_contract_canvas_stroke_supported" \
  "runtime_imports_canvas_stroke_js_binding" \
  "phase29cc_wsm_g3_canvas_stroke_contract_vm: PASS (WSM-G3-min7 canvas.stroke contract lock)"
