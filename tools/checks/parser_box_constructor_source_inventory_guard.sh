#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

require_text() {
  local file="$1"
  local text="$2"
  if ! grep -Fq "$text" "$file"; then
    echo "[parser-constructor-source] missing '$text' in $file" >&2
    exit 1
  fi
}

MODEL=src/parser/source_authority/constructor_source.rs
OWNER=src/parser/source_authority.rs
BODY=src/parser/declarations/box_def/mod.rs

require_text "$MODEL" "ConstructorSourceOriginV1"
require_text "$MODEL" "GeneratedBirthInitializer"
require_text "$MODEL" "duplicate constructor source key"
require_text "$MODEL" "commit_constructor_at_current"
require_text "$MODEL" "seal_constructor_inventory"
require_text "$BODY" "source_tx.finish(&state.constructors)"
require_text src/parser/source_authority/selected_gate.rs \
  "selected gate duplicates constructor source key"

for file in \
  "$MODEL" \
  "$OWNER" \
  src/parser/source_authority/selected_gate.rs \
  src/parser/source_seal/model.rs \
  src/parser/source_seal/finalize.rs; do
  lines="$(wc -l < "$file")"
  if (( lines >= 760 )); then
    echo "[parser-constructor-source] source split required: $file has $lines lines" >&2
    exit 1
  fi
done

CARGO_BUILD_JOBS=4 cargo test --profile quick -q -p nyash-rust \
  parser::source_authority::tests --lib

echo "[parser-constructor-source] OK"
