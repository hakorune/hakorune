#!/bin/bash
set -euo pipefail
bash "$(dirname "$0")/lib/phase29cc_wsm_g3_canvas_contract_common_vm.sh" \
  "phase29cc_wsm_g3_canvas_clear_contract_vm" \
  "extern_contract_supported_name_maps_to_import" \
  "test_js_import_object_generation" \
  "phase29cc_wsm_g3_canvas_clear_contract_vm: PASS (WSM-G3-min2 canvas.clear contract lock)"
