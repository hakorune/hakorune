#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
DOC="$ROOT_DIR/docs/development/current/main/phases/phase-296x/296x-366-DIRECT-ARRAY-I64-CONSTRUCTOR-LOWERING-PILOT.md"
PREV="$ROOT_DIR/docs/development/current/main/phases/phase-296x/296x-365-DIRECT-ARRAY-I64-CONSTRUCTOR-LOWERING-SELECTION.md"
NEWBOX="$ROOT_DIR/src/llvm_py/instructions/newbox.py"
CONSTRUCTOR="$ROOT_DIR/src/llvm_py/instructions/mir_call/constructor_call.py"
TEST="$ROOT_DIR/src/llvm_py/tests/test_direct_array_i64_constructor_lowering.py"

require_line() {
  local file="$1"
  local expected="$2"
  if ! grep -Fqx "$expected" "$file"; then
    echo "[row366-direct-array-i64-constructor-lowering-pilot] missing line in ${file#$ROOT_DIR/}: $expected" >&2
    exit 1
  fi
}

require_contains() {
  local file="$1"
  local expected="$2"
  if ! grep -Fq "$expected" "$file"; then
    echo "[row366-direct-array-i64-constructor-lowering-pilot] missing text in ${file#$ROOT_DIR/}: $expected" >&2
    exit 1
  fi
}

require_line "$DOC" "Status: Landed"
require_line "$PREV" "Status: Landed"
require_line "$DOC" "output_contract=direct-array-i64-constructor-lowering-pilot-v0"
require_line "$DOC" "input_contract=direct-array-i64-constructor-lowering-selection-v0"
require_line "$DOC" "implemented_owner=llvm_arraybox_constructor_direct_array_birth_hook"
require_line "$DOC" "selected_direct_birth_symbol=nyash.array.direct_i64.birth_h"
require_line "$DOC" "default_public_birth_symbol=nyash.array.birth_h"
require_line "$DOC" "default_public_birth_unchanged=1"
require_line "$DOC" "direct_array_birth_requires_env=HAKO_ARRAY_SLOT_STORE=direct_array_i64_exact"
require_line "$DOC" "direct_array_birth_default_emission=0"
require_line "$DOC" "direct_array_origin_fact=resolver.direct_array_i64_ids"
require_line "$DOC" "direct_array_origin_fact_recorded=1"
require_line "$DOC" "public_arraybox_handle_as_direct_buffer_allowed=0"
require_line "$DOC" "generic_arraybox_rewrite_allowed=0"
require_line "$DOC" "selected_method_lowering_open=0"
require_line "$DOC" "direct_load_store_open=0"
require_line "$DOC" "python_unit_smoke=ok"
require_line "$DOC" "selected_next=array_slot_nativedirect_lowering_readiness_refresh_after_constructor"
require_line "$DOC" "summary=ok"

require_contains "$NEWBOX" "DIRECT_ARRAY_I64_BIRTH_SYMBOL"
require_contains "$NEWBOX" "resolver.direct_array_i64_ids"
require_contains "$CONSTRUCTOR" "DIRECT_ARRAY_I64_BIRTH_SYMBOL"
require_contains "$CONSTRUCTOR" "resolver.direct_array_i64_ids"

python3 -m unittest "$TEST"

echo "[row366-direct-array-i64-constructor-lowering-pilot] ok"
