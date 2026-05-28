#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
DOC="$ROOT_DIR/docs/development/current/main/phases/phase-296x/296x-220-MIR-TYPED-FIELD-DIRECT-OP-SELECTED-METHOD-KEEPER.md"

require_line() {
  local line="$1"
  if ! grep -Fqx "$line" "$DOC"; then
    echo "[row220-direct-op-feasibility] missing line: $line" >&2
    exit 1
  fi
}

require_line "output_contract=mir-typed-field-direct-op-selected-method-feasibility-v0"
require_line "input_contract=mir-typed-field-direct-op-guard-surface-v0"
require_line "selected_method=HakoAllocPageModel.acquire_usize/1"
require_line "requested_helper_free_direct_op=1"
require_line "feasible_with_current_storage_abi=0"
require_line "selected_method_keeper_open=0"
require_line "rejected_owner=helper_free_typed_field_direct_op"
require_line "rejected_reason=typed_object_storage_is_rust_tls_vec_and_llvm_only_has_opaque_handles"
require_line "next_owner=typed_object_field_rmw_fusion_selection"
require_line "by_name_special_case=0"
require_line "source_rewrite=0"
require_line "winner_claim=0"
require_line "replacement_active=0"
require_line "hook_installed=0"
require_line "global_allocator=0"
require_line "summary=ok"

echo "mir_typed_field_direct_op_selected_method_feasibility_guard=ok"
