#!/bin/bash
set -euo pipefail
bash "$(dirname "$0")/phase29cc_wsm_g3_canvas_contract_common_vm.sh" \
  "phase29cc_wsm_g3_canvas_fill_contract_vm" \
  "extern_contract_canvas_fill_supported" \
  "runtime_imports_canvas_fill_js_binding" \
  "phase29cc_wsm_g3_canvas_fill_contract_vm: PASS (WSM-G3-min6 canvas.fill contract lock)"
