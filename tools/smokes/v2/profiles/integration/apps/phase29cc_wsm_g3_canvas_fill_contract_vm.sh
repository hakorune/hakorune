#!/bin/bash
# phase29cc_wsm_g3_canvas_fill_contract_vm.sh
# Contract pin:
# - WSM-G3-min6 adds env.canvas.fill extern contract mapping.
# - runtime JS binding includes canvas_fill + ctx.fill usage.

set -euo pipefail

source "$(dirname "$0")/../../../lib/test_runner.sh"
source "$(dirname "$0")/../../../lib/wasm_g3_contract.sh"
require_env || exit 2

run_wasm_g3_contract_smoke \
  "phase29cc_wsm_g3_canvas_fill_contract_vm" \
  "extern_contract_canvas_fill_supported" \
  "runtime_imports_canvas_fill_js_binding" \
  "phase29cc_wsm_g3_canvas_fill_contract_vm: PASS (WSM-G3-min6 canvas.fill contract lock)"
