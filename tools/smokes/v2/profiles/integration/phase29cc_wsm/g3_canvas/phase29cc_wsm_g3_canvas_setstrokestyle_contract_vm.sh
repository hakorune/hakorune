#!/bin/bash
set -euo pipefail
bash "$(dirname "$0")/lib/phase29cc_wsm_g3_canvas_contract_common_vm.sh" \
  "phase29cc_wsm_g3_canvas_setstrokestyle_contract_vm" \
  "extern_contract_canvas_set_stroke_style_supported" \
  "runtime_imports_canvas_set_stroke_style_js_binding" \
  "phase29cc_wsm_g3_canvas_setstrokestyle_contract_vm: PASS (WSM-G3-min9 canvas.setStrokeStyle contract lock)"
