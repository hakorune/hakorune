#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
DOC="$ROOT_DIR/docs/development/current/main/phases/phase-296x/296x-364-DIRECT-ARRAY-I64-BIRTH-HANDLE-PILOT.md"
PREV="$ROOT_DIR/docs/development/current/main/phases/phase-296x/296x-363-DIRECT-ARRAY-I64-BIRTH-HANDLE-SELECTION.md"
SRC="$ROOT_DIR/crates/nyash_kernel/src/plugin/array_direct_i64_buffer.rs"
BIRTH="$ROOT_DIR/crates/nyash_kernel/src/exports/birth.rs"

require_line() {
  local file="$1"
  local expected="$2"
  if ! grep -Fqx "$expected" "$file"; then
    echo "[row364-direct-array-i64-birth-handle-pilot] missing line in ${file#$ROOT_DIR/}: $expected" >&2
    exit 1
  fi
}

require_contains() {
  local file="$1"
  local expected="$2"
  if ! grep -Fq "$expected" "$file"; then
    echo "[row364-direct-array-i64-birth-handle-pilot] missing text in ${file#$ROOT_DIR/}: $expected" >&2
    exit 1
  fi
}

require_line "$DOC" "Status: Landed"
require_line "$PREV" "Status: Landed"
require_line "$DOC" "output_contract=direct-array-i64-birth-handle-pilot-v0"
require_line "$DOC" "input_contract=direct-array-i64-birth-handle-selection-v0"
require_line "$DOC" "implemented_symbol=nyash.array.direct_i64.birth_h"
require_line "$DOC" "default_arraybox_constructor_symbol=nyash.array.birth_h"
require_line "$DOC" "default_arraybox_constructor_unchanged=1"
require_line "$DOC" "direct_array_handle_kind=tagged_stable_direct_array_i64_buffer_pointer"
require_line "$DOC" "direct_array_handle_tag=3"
require_line "$DOC" "direct_array_default_capacity=64"
require_line "$DOC" "handle_kinds_do_not_alias=1"
require_line "$DOC" "host_handle_lookup_for_direct_array_handle=none"
require_line "$DOC" "public_arraybox_handle_as_direct_buffer_allowed=0"
require_line "$DOC" "constructor_lowering_changed=0"
require_line "$DOC" "selected_method_lowering_open=0"
require_line "$DOC" "llvm_lowering_open=0"
require_line "$DOC" "direct_load_store_open=0"
require_line "$DOC" "birth_handle_smoke=ok"
require_line "$DOC" "default_arraybox_birth_smoke=ok"
require_line "$DOC" "selected_next=direct_array_i64_constructor_lowering_selection"
require_line "$DOC" "summary=ok"

require_contains "$SRC" "DIRECT_ARRAY_I64_HANDLE_TAG: usize = 3"
require_contains "$SRC" "DEFAULT_DIRECT_ARRAY_I64_CAPACITY: usize = 64"
require_contains "$SRC" "DIRECT_ARRAY_I64_OBJECTS"
require_contains "$SRC" "nyash.array.direct_i64.birth_h"
require_contains "$SRC" "direct_array_i64_birth_handle_with_capacity"
require_contains "$BIRTH" "nyash.array.birth_h"
require_contains "$BIRTH" "ArrayBox::new()"

cargo test -p nyash_kernel direct_array_i64_birth_handle -- --nocapture

echo "[row364-direct-array-i64-birth-handle-pilot] ok"
