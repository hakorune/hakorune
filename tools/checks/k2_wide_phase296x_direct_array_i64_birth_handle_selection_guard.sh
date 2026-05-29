#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
DOC="$ROOT_DIR/docs/development/current/main/phases/phase-296x/296x-363-DIRECT-ARRAY-I64-BIRTH-HANDLE-SELECTION.md"
PREV="$ROOT_DIR/docs/development/current/main/phases/phase-296x/296x-362-ARRAY-SLOT-NATIVEDIRECT-LOWERING-SELECTED-METHOD-PILOT-PREFLIGHT.md"

require_line() {
  local file="$1"
  local expected="$2"
  if ! grep -Fqx "$expected" "$file"; then
    echo "[row363-direct-array-i64-birth-handle-selection] missing line in ${file#$ROOT_DIR/}: $expected" >&2
    exit 1
  fi
}

require_line "$DOC" "Status: Landed"
require_line "$PREV" "Status: Landed"
require_line "$DOC" "output_contract=direct-array-i64-birth-handle-selection-v0"
require_line "$DOC" "input_contract=array-slot-nativedirect-lowering-selected-method-pilot-preflight-v0"
require_line "$DOC" "selected_owner=direct_array_i64_birth_handle_producer"
require_line "$DOC" "selected_runtime_owner=crates/nyash_kernel/src/plugin/array_direct_i64_buffer.rs"
require_line "$DOC" "selected_backend=direct_array_i64_exact"
require_line "$DOC" "default_arraybox_constructor_symbol=nyash.array.birth_h"
require_line "$DOC" "default_arraybox_constructor_unchanged=1"
require_line "$DOC" "new_direct_array_birth_symbol_required=1"
require_line "$DOC" "proposed_direct_array_birth_symbol=nyash.array.direct_i64.birth_h"
require_line "$DOC" "direct_array_handle_kind=tagged_or_positive_direct_array_i64_buffer_handle"
require_line "$DOC" "public_arraybox_handle_kind=public_arraybox_host_handle"
require_line "$DOC" "handle_kinds_must_not_alias=1"
require_line "$DOC" "selected_method_lowering_open=0"
require_line "$DOC" "constructor_lowering_open_next=1"
require_line "$DOC" "generic_arraybox_rewrite_allowed=0"
require_line "$DOC" "runtime_helper_semantics_changes_allowed=0"
require_line "$DOC" "public_arraybox_semantics_unchanged=1"
require_line "$DOC" "materialized_view_boundary=explicit_public_arraybox_snapshot_handle"
require_line "$DOC" "silent_fallback_allowed=0"
require_line "$DOC" "by_name_hako_alloc_special_case=0"
require_line "$DOC" "selected_next=direct_array_i64_birth_handle_pilot"
require_line "$DOC" "llvm_lowering_open=0"
require_line "$DOC" "direct_load_store_open=0"
require_line "$DOC" "summary=ok"

echo "[row363-direct-array-i64-birth-handle-selection] ok"
