#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
DOC="$ROOT_DIR/docs/development/current/main/phases/phase-296x/296x-335-DIRECT-SLOT-NATIVEDIRECT-LOWERING-OWNER-SELECTION.md"
PREV="$ROOT_DIR/docs/development/current/main/phases/phase-296x/296x-334-DIRECT-SLOT-NATIVEDIRECT-LOWERING-GUARD-SURFACE.md"

require_line() {
  local file="$1"
  local expected="$2"
  if ! grep -Fqx "$expected" "$file"; then
    echo "[row335-direct-slot-nativedirect-lowering-owner-selection] missing line in ${file#$ROOT_DIR/}: $expected" >&2
    exit 1
  fi
}

require_line "$DOC" "Status: Landed"
require_line "$PREV" "Status: Landed"
require_line "$DOC" "output_contract=direct-slot-nativedirect-lowering-owner-selection-v0"
require_line "$DOC" "input_contract=direct-slot-nativedirect-lowering-guard-surface-v0"
require_line "$DOC" "selected_owner=llvm_field_access_direct_slot_nativedirect_selected_method_hook"
require_line "$DOC" "selected_owner_file=src/llvm_py/instructions/field_access.py"
require_line "$DOC" "selected_method=HakoAllocPageModel.acquire_usize/1"
require_line "$DOC" "selected_backend=direct_slot_exact"
require_line "$DOC" "mirbuilder_changes_allowed=0"
require_line "$DOC" "hako_source_changes_allowed=0"
require_line "$DOC" "runtime_helper_semantics_changes_allowed=0"
require_line "$DOC" "generic_direct_slot_rewrite_allowed=0"
require_line "$DOC" "selected_method_only=1"
require_line "$DOC" "direct_slot_exact_only=1"
require_line "$DOC" "default_backend_emission=0"
require_line "$DOC" "fallback_boundary=explicit_materialized_view_handle"
require_line "$DOC" "silent_fallback_allowed=0"
require_line "$DOC" "helper_load_writeback_substitution_allowed=0"
require_line "$DOC" "by_name_hako_alloc_special_case=0"
require_line "$DOC" "llvm_lowering_open=0"
require_line "$DOC" "selected_next=direct_slot_nativedirect_lowering_selected_method_pilot"
require_line "$DOC" "summary=ok"

echo "[row335-direct-slot-nativedirect-lowering-owner-selection] ok"
