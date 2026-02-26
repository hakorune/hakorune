#!/bin/bash
set -euo pipefail
bash "$(dirname "$0")/phase29cc_wsm_g3_canvas_contract_common_vm.sh" \
  "phase29cc_wsm_g3_canvas_setfillstyle_contract_vm" \
  "extern_contract_canvas_set_fill_style_supported" \
  "runtime_imports_canvas_set_fill_style_js_binding" \
  "phase29cc_wsm_g3_canvas_setfillstyle_contract_vm: PASS (WSM-G3-min8 canvas.setFillStyle contract lock)"
