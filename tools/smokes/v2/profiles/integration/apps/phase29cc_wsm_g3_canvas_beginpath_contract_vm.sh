#!/bin/bash
# phase29cc_wsm_g3_canvas_beginpath_contract_vm.sh
# Contract pin:
# - WSM-G3-min4 adds env.canvas.beginPath extern contract mapping.
# - runtime JS binding includes canvas_beginPath + beginPath usage.

set -euo pipefail

source "$(dirname "$0")/../../../lib/test_runner.sh"
source "$(dirname "$0")/../../../lib/wasm_g3_contract.sh"
require_env || exit 2

run_wasm_g3_contract_smoke \
  "phase29cc_wsm_g3_canvas_beginpath_contract_vm" \
  "extern_contract_canvas_begin_path_supported" \
  "runtime_imports_canvas_begin_path_js_binding" \
  "phase29cc_wsm_g3_canvas_beginpath_contract_vm: PASS (WSM-G3-min4 canvas.beginPath contract lock)"
