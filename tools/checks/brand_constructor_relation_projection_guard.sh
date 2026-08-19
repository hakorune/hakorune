#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

MODEL=src/mir/builder/brand_constructor_lowering_projection.rs
CALLABLE=src/mir/builder/normal_callable_semantic_lowering_state.rs
SCRIPT=src/mir/builder/normal_script_semantic_lowering_projection.rs
RAW=src/mir/builder/calls/function_call_preflight_route.rs

require_text() {
  local file="$1"
  local text="$2"
  grep -Fq "$text" "$file" || {
    echo "[brand-constructor-projection] missing '$text' in $file" >&2
    exit 1
  }
}

reject_text() {
  local file="$1"
  local text="$2"
  if grep -Fq "$text" "$file"; then
    echo "[brand-constructor-projection] forbidden '$text' in $file" >&2
    exit 1
  fi
}

require_text "$MODEL" "BrandConstructorDispositionRefV1"
require_text "$MODEL" "MissingExpressionSite"
require_text "$MODEL" "RelationOutsideExpressionInventory"
require_text "$CALLABLE" "from_verified_owner"
require_text "$SCRIPT" "from_verified_owner"
require_text "$RAW" "is_brand_declared"
reject_text "$MODEL" "ASTNode"
reject_text "$MODEL" "ValueId"

for file in "$MODEL" "$CALLABLE" "$SCRIPT" \
  src/mir/builder/normal_script_semantic_lowering_state.rs \
  src/mir/resolved_semantics/product.rs \
  src/mir/resolved_semantics/source_site_inventory.rs; do
  lines="$(wc -l < "$file")"
  if (( lines >= 760 )); then
    echo "[brand-constructor-projection] source split required: $file has $lines lines" >&2
    exit 1
  fi
done

CARGO_BUILD_JOBS=4 cargo test --profile quick -q -p nyash-rust \
  mir::builder::brand_constructor_lowering_projection::tests --lib

echo "[brand-constructor-projection] OK"
