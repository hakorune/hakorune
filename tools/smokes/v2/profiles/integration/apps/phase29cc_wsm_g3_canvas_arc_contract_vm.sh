#!/bin/bash
# phase29cc_wsm_g3_canvas_arc_contract_vm.sh
# Contract pin:
# - WSM-G3-min5 adds env.canvas.arc extern contract mapping.
# - runtime JS binding includes canvas_arc + ctx.arc usage.

set -euo pipefail

source "$(dirname "$0")/../../../lib/test_runner.sh"
source "$(dirname "$0")/../../../lib/wasm_g3_contract.sh"
require_env || exit 2

run_wasm_g3_contract_smoke \
  "phase29cc_wsm_g3_canvas_arc_contract_vm" \
  "extern_contract_canvas_arc_supported" \
  "runtime_imports_canvas_arc_js_binding" \
  "phase29cc_wsm_g3_canvas_arc_contract_vm: PASS (WSM-G3-min5 canvas.arc contract lock)"
