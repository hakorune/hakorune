#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
DOC="$ROOT_DIR/docs/development/current/main/phases/phase-296x/296x-319-DIRECT-SLOT-LEASE-ADDRESSABLE-SLOT-BRIDGE-SSOT.md"
SSOT="$ROOT_DIR/docs/development/current/main/design/direct-slot-lease-addressable-slot-bridge-ssot.md"
PREV="$ROOT_DIR/docs/development/current/main/phases/phase-296x/296x-318-DIRECT-SLOT-LEASE-LOWERING-PILOT-FEASIBILITY-CLOSEOUT.md"

require_line() {
  local file="$1"
  local expected="$2"
  if ! grep -Fqx "$expected" "$file"; then
    echo "[row319-direct-slot-lease-addressable-slot-bridge-ssot] missing line in ${file#$ROOT_DIR/}: $expected" >&2
    exit 1
  fi
}

require_line "$DOC" "Status: Landed"
require_line "$SSOT" "Status: Accepted"
require_line "$PREV" "Status: Landed"
require_line "$DOC" "output_contract=direct-slot-lease-addressable-slot-bridge-ssot-v0"
require_line "$DOC" "input_contract=direct-slot-lease-lowering-pilot-feasibility-v0"
require_line "$DOC" "selected_bridge=direct_slot_cell_storage"
require_line "$DOC" "stable_cell_layout_required=1"
require_line "$DOC" "llvm_consumable_slot_address_required=1"
require_line "$DOC" "handle_resolution_contract_required=1"
require_line "$DOC" "generation_or_identity_validation_required=1"
require_line "$DOC" "cell_storage_classes=i64|u64|handle"
require_line "$DOC" "raw_runtime_vec_pointer_exposure=0"
require_line "$DOC" "thread_local_refcell_pointer_exposure=0"
require_line "$DOC" "rust_enum_layout_direct_load=0"
require_line "$DOC" "c_abi_load_writeback_helper_bridge=0"
require_line "$DOC" "by_name_hako_alloc_special_case=0"
require_line "$DOC" "selected_plan_silent_fallback_allowed=0"
require_line "$DOC" "existing_helper_abi_unchanged=1"
require_line "$DOC" "lowering_open=0"
require_line "$DOC" "native_direct_open=0"
require_line "$DOC" "direct_load_store_open=0"
require_line "$DOC" "implementation_open=0"
require_line "$DOC" "selected_next=direct_slot_cell_storage_layout_selection"
require_line "$DOC" "summary=ok"

require_line "$SSOT" "bridge_kind=direct_slot_cell_storage"
require_line "$SSOT" "slot_cell_layout=stable_abi"
require_line "$SSOT" "raw_vec_slot_pointer_bridge=0"
require_line "$SSOT" "rust_enum_layout_direct_load=0"
require_line "$SSOT" "c_abi_load_writeback_helper_bridge=0"
require_line "$SSOT" "addressable_slot_bridge_available=1"
require_line "$SSOT" "stable_cell_layout_defined=1"

echo "[row319-direct-slot-lease-addressable-slot-bridge-ssot] ok"
