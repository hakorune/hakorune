#!/usr/bin/env bash
set -euo pipefail

card='docs/development/current/main/phases/phase-293x/293x-315-ARRAY-002A-TYPED-ARRAY-METHOD-CONTRACT.md'
ssot='docs/development/current/main/design/typed-array-method-contract-ssot.md'

require_text() {
  local file="$1"
  local text="$2"
  if ! grep -Fq "$text" "$file"; then
    echo "[array-method-contract] missing '$text' in $file" >&2
    exit 1
  fi
}

require_text "$card" "293x-315 ARRAY-002A typed Array method contract"
require_text "$ssot" "Typed Array Method Contract SSOT"
require_text docs/reference/language/EBNF.md 'ARRAY-002A implements typed `Array<T>` method-name and arity diagnostics.'
require_text docs/reference/language/quick-reference.md '`xs.push(v)`, `xs.get(i)`, `xs.set(i, v)`, `xs.length()`'
require_text src/stage1/program_json_v0/lowering.rs "[array/method-contract]"
require_text src/stage1/program_json_v0/lowering.rs "validate_typed_array_method_contract"
require_text docs/tools/check-scripts-index.md "k2_wide_array_typed_method_contract_guard.sh"

cargo test -q typed_array_method_contract --lib

echo "[array-method-contract] OK"
