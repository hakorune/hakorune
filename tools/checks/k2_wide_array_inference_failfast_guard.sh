#!/usr/bin/env bash
set -euo pipefail

card='docs/development/current/main/phases/phase-293x/293x-317-ARRAY-002C-UNSUPPORTED-ARRAY-INFERENCE-FAILFAST.md'
ssot='docs/development/current/main/design/typed-array-inference-failfast-ssot.md'

require_text() {
  local file="$1"
  local text="$2"
  if ! grep -Fq "$text" "$file"; then
    echo "[array-inference] missing '$text' in $file" >&2
    exit 1
  fi
}

require_text "$card" "293x-317 ARRAY-002C unsupported Array inference fail-fast"
require_text "$ssot" "Typed Array Inference Fail-Fast SSOT"
require_text docs/reference/language/EBNF.md 'ARRAY-002C keeps unsupported `Array<T>` inference fail-fast.'
require_text src/stage1/program_json_v0/lowering.rs "[array/inference]"
require_text src/stage1/program_json_v0/lowering.rs "array_element_type_has_unresolved_generic"
require_text docs/tools/check-scripts-index.md "k2_wide_array_inference_failfast_guard.sh"

cargo test -q typed_array_inference --lib
cargo test -q untyped_empty_array_literal --lib

echo "[array-inference] OK"
