#!/bin/bash
# phase29cc_wsm_g3_canvas_setlinewidth_contract_vm.sh
# Contract pin:
# - WSM-G3-min10 adds env.canvas.setLineWidth extern contract mapping.
# - runtime JS binding includes canvas_setLineWidth + ctx.lineWidth assignment.

set -euo pipefail

source "$(dirname "$0")/../../../lib/test_runner.sh"
source "$(dirname "$0")/../../../lib/wasm_g3_contract.sh"
require_env || exit 2

run_wasm_g3_contract_smoke \
  "phase29cc_wsm_g3_canvas_setlinewidth_contract_vm" \
  "extern_contract_canvas_set_line_width_supported" \
  "runtime_imports_canvas_set_line_width_js_binding" \
  "phase29cc_wsm_g3_canvas_setlinewidth_contract_vm: PASS (WSM-G3-min10 canvas.setLineWidth contract lock)"
