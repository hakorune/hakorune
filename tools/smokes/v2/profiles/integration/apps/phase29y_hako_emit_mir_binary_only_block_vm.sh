#!/bin/bash
# phase29y_hako_emit_mir_binary_only_block_vm.sh
# Compatibility wrapper:
# - BINARY-ONLY-B01 blocked pin is historical.
# - Active contract is BINARY-ONLY-B04 ported route.
# - Keep this file only for legacy entry compatibility.

set -euo pipefail

echo "[INFO] phase29y_hako_emit_mir_binary_only_block_vm: legacy alias -> phase29y_hako_emit_mir_binary_only_ported_vm.sh" >&2
exec "$(dirname "$0")/phase29y_hako_emit_mir_binary_only_ported_vm.sh" "$@"
