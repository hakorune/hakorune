#!/bin/bash
set -euo pipefail
bash "$(dirname "$0")/lib/phase29cc_wsm_g3_canvas_contract_common_vm.sh" \
  "phase29cc_wsm_g3_canvas_drawline_contract_vm" \
  "extern_contract_canvas_draw_line_supported" \
  "runtime_imports_canvas_draw_line_js_binding" \
  "phase29cc_wsm_g3_canvas_drawline_contract_vm: PASS (WSM-G3-min13 canvas.drawLine contract lock)"
