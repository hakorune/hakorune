#!/usr/bin/env bash
set -euo pipefail

card='docs/development/current/main/phases/phase-293x/293x-313-ARRAY-001-TYPED-CONTEXT-ARRAY-LITERAL.md'
ssot='docs/development/current/main/design/typed-array-literal-context-ssot.md'

require_text() {
  local file="$1"
  local text="$2"
  if ! grep -Fq "$text" "$file"; then
    echo "[array-literal-context] missing '$text' in $file" >&2
    exit 1
  fi
}

require_text "$card" "293x-313 ARRAY-001 typed context array literal"
require_text "$ssot" "Typed Array Literal Context SSOT"
require_text docs/reference/language/EBNF.md 'ARRAY-001 implements typed-context array literal lowering for `Array<T>` only.'
require_text src/parser/expr/primary.rs "Stage1 owns typed-context checks"
require_text src/parser/expr_cursor.rs "Stage1 owns typed-context checks"
require_text src/stage1/program_json_v0/lowering.rs '"type": "ArrayLiteral"'
require_text src/stage1/program_json_v0/lowering.rs "PackedArray literal lowering is deferred; no Array<T> fallback"
require_text src/runner/json_v0_bridge/ast.rs "ArrayLiteral"
require_text src/runner/json_v0_bridge/lowering/expr/call_ops.rs "lower_array_values_expr"
require_text docs/tools/check-scripts-index.md "k2_wide_array_typed_context_literal_guard.sh"

cargo test -q array_literal --lib

echo "[array-literal-context] OK"
