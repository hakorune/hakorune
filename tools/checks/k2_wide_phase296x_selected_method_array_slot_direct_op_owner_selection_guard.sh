#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
DOC="$ROOT_DIR/docs/development/current/main/phases/phase-296x/296x-210-SELECTED-METHOD-ARRAY-SLOT-DIRECT-OP-OWNER-SELECTION.md"
PREV="$ROOT_DIR/docs/development/current/main/phases/phase-296x/296x-209-MIR-ARRAY-SLOT-RESIDENCE-SELECTED-METHOD-GUARD-SURFACE.md"
TOOL="$ROOT_DIR/tools/allocator/selected_method_array_slot_direct_op_owner_selection.py"
TMP_DIR="$(mktemp -d /tmp/hakorune_row210_array_owner.XXXXXX)"
trap 'rm -rf "$TMP_DIR"' EXIT
REPORT="$TMP_DIR/report.out"

require_line() {
  local file="$1"
  local expected="$2"
  if ! grep -q "^${expected}$" "$file"; then
    echo "[row210-array-owner] missing line in ${file#$ROOT_DIR/}: $expected" >&2
    exit 1
  fi
}

require_line "$DOC" "Status: Landed"
require_line "$PREV" "Status: Landed"
require_line "$DOC" "Decision: accepted"
require_line "$DOC" "selected_owner=c_abi_same_module_array_slot_direct_op_fusion"
require_line "$DOC" "implementation_owner_file=lang/c-abi/shims/hako_llvmc_ffi_same_module_body_emit.inc"
require_line "$DOC" "declaration_owner_file=lang/c-abi/shims/hako_llvmc_ffi_pure_compile_generic_lowering_prescan.inc"
require_line "$DOC" "runtime_export_owner=crates/nyash_kernel/src/plugin/array_direct_slot_op.rs"
require_line "$DOC" "runtime_mod_owner=crates/nyash_kernel/src/plugin/mod.rs"
require_line "$DOC" "planned_fused_runtime_symbol=nyash.array.slot_load_store_i64_hihi"
require_line "$DOC" "planned_erased_get_set_helper_calls=2"
require_line "$DOC" "planned_added_fused_helper_calls=1"
require_line "$DOC" "planned_net_helper_call_delta=1"
require_line "$DOC" "generic_array_residence_open=0"
require_line "$DOC" "by_name_hako_alloc_special_case=0"
require_line "$DOC" "summary=ok"

"$TOOL" --out "$REPORT"

require_line "$REPORT" "output_contract=selected-method-array-slot-direct-op-owner-selection-v0"
require_line "$REPORT" "selected_owner=c_abi_same_module_array_slot_direct_op_fusion"
require_line "$REPORT" "implementation_owner_file=lang/c-abi/shims/hako_llvmc_ffi_same_module_body_emit.inc"
require_line "$REPORT" "declaration_owner_file=lang/c-abi/shims/hako_llvmc_ffi_pure_compile_generic_lowering_prescan.inc"
require_line "$REPORT" "runtime_export_owner=crates/nyash_kernel/src/plugin/array_direct_slot_op.rs"
require_line "$REPORT" "runtime_mod_owner=crates/nyash_kernel/src/plugin/mod.rs"
require_line "$REPORT" "rejected_owner_0=boxcall_runtime_data_individual_get_set_lowering"
require_line "$REPORT" "rejected_owner_1=generic_mir_array_residence_transform"
require_line "$REPORT" "rejected_owner_2=hako_alloc_by_name_source_rewrite"
require_line "$REPORT" "planned_fused_runtime_symbol=nyash.array.slot_load_store_i64_hihi"
require_line "$REPORT" "planned_net_helper_call_delta=1"
require_line "$REPORT" "generic_array_residence_open=0"
require_line "$REPORT" "by_name_hako_alloc_special_case=0"
require_line "$REPORT" "summary=ok"

cat "$REPORT"
