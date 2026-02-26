#!/bin/bash
# phase29cc_wsm_g3_canvas_clear_contract_vm.sh
# Contract pin:
# - WSM-G3-min2 adds env.canvas.clear extern contract mapping.
# - runtime imports and JS bindings include canvas_clear + clearRect behavior.

set -euo pipefail

source "$(dirname "$0")/../../../lib/test_runner.sh"
source "$(dirname "$0")/../../../lib/wasm_g3_contract.sh"
require_env || exit 2

run_wasm_g3_contract_smoke \
  "phase29cc_wsm_g3_canvas_clear_contract_vm" \
  "extern_contract_supported_name_maps_to_import" \
  "test_js_import_object_generation" \
  "phase29cc_wsm_g3_canvas_clear_contract_vm: PASS (WSM-G3-min2 canvas.clear contract lock)"
