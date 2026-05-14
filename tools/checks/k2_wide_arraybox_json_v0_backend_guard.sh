#!/usr/bin/env bash
set -euo pipefail

card='docs/development/current/main/phases/phase-293x/293x-318-ARRAY-002D-ARRAYBOX-JSONV0-BACKEND-GUARD.md'
ssot='docs/development/current/main/design/arraybox-json-v0-backend-guard-ssot.md'

require_text() {
  local file="$1"
  local text="$2"
  if ! grep -Fq "$text" "$file"; then
    echo "[arraybox-json-v0] missing '$text' in $file" >&2
    exit 1
  fi
}

require_text "$card" "293x-318 ARRAY-002D ArrayBox JSON v0/backend guard"
require_text "$ssot" "ArrayBox JSON v0 Backend Guard SSOT"
require_text docs/reference/language/EBNF.md 'ARRAY-002D fixes the JSON v0 / ArrayBox guard for ordinary `Array<T>` and PackedArray no-fallback.'
require_text src/stage1/program_json_v0/lowering.rs '"type": "ArrayLiteral"'
require_text src/runner/json_v0_bridge/ast.rs "ArrayLiteral"
require_text src/runner/json_v0_bridge/lowering/expr/call_ops.rs "lower_array_values_expr"
require_text src/stage1/program_json_v0/lowering.rs "PackedArray literal lowering is deferred; no Array<T> fallback"
require_text docs/tools/check-scripts-index.md "k2_wide_arraybox_json_v0_backend_guard.sh"

cargo test -q array_literal --lib
cargo test -q packed_array_literal --lib

echo "[arraybox-json-v0] OK"
