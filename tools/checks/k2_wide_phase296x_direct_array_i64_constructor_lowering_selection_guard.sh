#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
DOC="$ROOT_DIR/docs/development/current/main/phases/phase-296x/296x-365-DIRECT-ARRAY-I64-CONSTRUCTOR-LOWERING-SELECTION.md"
PREV="$ROOT_DIR/docs/development/current/main/phases/phase-296x/296x-364-DIRECT-ARRAY-I64-BIRTH-HANDLE-PILOT.md"
NEWBOX="$ROOT_DIR/src/llvm_py/instructions/newbox.py"
CONSTRUCTOR="$ROOT_DIR/src/llvm_py/instructions/mir_call/constructor_call.py"

require_line() {
  local file="$1"
  local expected="$2"
  if ! grep -Fqx "$expected" "$file"; then
    echo "[row365-direct-array-i64-constructor-lowering-selection] missing line in ${file#$ROOT_DIR/}: $expected" >&2
    exit 1
  fi
}

require_contains() {
  local file="$1"
  local expected="$2"
  if ! grep -Fq "$expected" "$file"; then
    echo "[row365-direct-array-i64-constructor-lowering-selection] missing text in ${file#$ROOT_DIR/}: $expected" >&2
    exit 1
  fi
}

require_line "$DOC" "Status: Landed"
require_line "$PREV" "Status: Landed"
require_line "$DOC" "output_contract=direct-array-i64-constructor-lowering-selection-v0"
require_line "$DOC" "input_contract=direct-array-i64-birth-handle-pilot-v0"
require_line "$DOC" "selected_owner=llvm_arraybox_constructor_direct_array_birth_hook"
require_line "$DOC" "selected_owner_file_0=src/llvm_py/instructions/newbox.py"
require_line "$DOC" "selected_owner_file_1=src/llvm_py/instructions/mir_call/constructor_call.py"
require_line "$DOC" "selected_backend=direct_array_i64_exact"
require_line "$DOC" "selected_direct_birth_symbol=nyash.array.direct_i64.birth_h"
require_line "$DOC" "default_public_birth_symbol=nyash.array.birth_h"
require_line "$DOC" "default_public_birth_unchanged=1"
require_line "$DOC" "direct_array_birth_requires_env=HAKO_ARRAY_SLOT_STORE=direct_array_i64_exact"
require_line "$DOC" "direct_array_birth_requires_exact_lane=1"
require_line "$DOC" "direct_array_birth_default_emission=0"
require_line "$DOC" "public_arraybox_handle_as_direct_buffer_allowed=0"
require_line "$DOC" "generic_arraybox_rewrite_allowed=0"
require_line "$DOC" "runtime_helper_semantics_changes_allowed=0"
require_line "$DOC" "selected_method_lowering_open=0"
require_line "$DOC" "llvm_lowering_open=0"
require_line "$DOC" "direct_load_store_open=0"
require_line "$DOC" "selected_next=direct_array_i64_constructor_lowering_pilot"
require_line "$DOC" "summary=ok"

require_contains "$NEWBOX" "nyash.array.birth_h"
require_contains "$CONSTRUCTOR" "nyash.array.birth_h"

echo "[row365-direct-array-i64-constructor-lowering-selection] ok"
