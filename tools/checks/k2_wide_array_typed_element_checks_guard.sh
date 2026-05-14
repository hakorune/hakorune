#!/usr/bin/env bash
set -euo pipefail

card='docs/development/current/main/phases/phase-293x/293x-316-ARRAY-002B-TYPED-ARRAY-ELEMENT-CHECKS.md'
ssot='docs/development/current/main/design/typed-array-element-checks-ssot.md'

require_text() {
  local file="$1"
  local text="$2"
  if ! grep -Fq "$text" "$file"; then
    echo "[array-element-checks] missing '$text' in $file" >&2
    exit 1
  fi
}

require_text "$card" "293x-316 ARRAY-002B typed local Array element checks"
require_text "$ssot" "Typed Array Element Checks SSOT"
require_text docs/reference/language/EBNF.md 'ARRAY-002B implements direct typed `Array<T>` element diagnostics.'
require_text src/stage1/program_json_v0/lowering.rs "[array/element-type]"
require_text src/stage1/program_json_v0/lowering.rs "validate_array_element_expr"
require_text docs/tools/check-scripts-index.md "k2_wide_array_typed_element_checks_guard.sh"

cargo test -q typed_array_element --lib

echo "[array-element-checks] OK"
