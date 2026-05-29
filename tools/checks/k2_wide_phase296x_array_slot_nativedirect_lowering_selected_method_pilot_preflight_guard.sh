#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
DOC="$ROOT_DIR/docs/development/current/main/phases/phase-296x/296x-362-ARRAY-SLOT-NATIVEDIRECT-LOWERING-SELECTED-METHOD-PILOT-PREFLIGHT.md"
PREV="$ROOT_DIR/docs/development/current/main/phases/phase-296x/296x-361-ARRAY-SLOT-NATIVEDIRECT-LOWERING-OWNER-SELECTION.md"
CONSTRUCTOR="$ROOT_DIR/src/llvm_py/instructions/mir_call/constructor_call.py"
NEWBOX="$ROOT_DIR/src/llvm_py/instructions/newbox.py"

require_line() {
  local file="$1"
  local expected="$2"
  if ! grep -Fqx "$expected" "$file"; then
    echo "[row362-array-slot-nativedirect-pilot-preflight] missing line in ${file#$ROOT_DIR/}: $expected" >&2
    exit 1
  fi
}

require_contains() {
  local file="$1"
  local expected="$2"
  if ! grep -Fq "$expected" "$file"; then
    echo "[row362-array-slot-nativedirect-pilot-preflight] missing text in ${file#$ROOT_DIR/}: $expected" >&2
    exit 1
  fi
}

require_line "$DOC" "Status: Landed"
require_line "$PREV" "Status: Landed"
require_line "$DOC" "output_contract=array-slot-nativedirect-lowering-selected-method-pilot-preflight-v0"
require_line "$DOC" "input_contract=array-slot-nativedirect-lowering-owner-selection-v0"
require_line "$DOC" "selected_method=HakoAllocPageModel.acquire_usize/1"
require_line "$DOC" "arraybox_constructor_symbol=nyash.array.birth_h"
require_line "$DOC" "current_arraybox_handle_kind=public_arraybox_host_handle"
require_line "$DOC" "required_arraybox_handle_kind=direct_array_i64_buffer_pointer_or_tagged_handle"
require_line "$DOC" "direct_array_birth_handle_producer_available=0"
require_line "$DOC" "unsafe_pointer_reinterpretation_risk=1"
require_line "$DOC" "selected_method_lowering_implemented=0"
require_line "$DOC" "selected_method_lowering_blocked=1"
require_line "$DOC" "blocked_reason=direct_array_handle_producer_missing"
require_line "$DOC" "direct_load_store_open=0"
require_line "$DOC" "native_direct_open=0"
require_line "$DOC" "llvm_lowering_open=0"
require_line "$DOC" "helper_route_reuse_allowed=0"
require_line "$DOC" "public_arraybox_handle_as_direct_buffer_allowed=0"
require_line "$DOC" "silent_fallback_allowed=0"
require_line "$DOC" "by_name_hako_alloc_special_case=0"
require_line "$DOC" "selected_next=direct_array_i64_birth_handle_selection"
require_line "$DOC" "summary=ok"

require_contains "$CONSTRUCTOR" "nyash.array.birth_h"
require_contains "$NEWBOX" "nyash.array.birth_h"

echo "[row362-array-slot-nativedirect-pilot-preflight] ok"
