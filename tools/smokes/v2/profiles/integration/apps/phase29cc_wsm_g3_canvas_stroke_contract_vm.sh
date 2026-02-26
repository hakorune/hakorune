#!/bin/bash
set -euo pipefail
bash "$(dirname "$0")/phase29cc_wsm_g3_canvas_contract_common_vm.sh" \
  "phase29cc_wsm_g3_canvas_stroke_contract_vm" \
  "extern_contract_canvas_stroke_supported" \
  "runtime_imports_canvas_stroke_js_binding" \
  "phase29cc_wsm_g3_canvas_stroke_contract_vm: PASS (WSM-G3-min7 canvas.stroke contract lock)"
