#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

require_text() {
  local file="$1"
  local text="$2"
  if ! grep -Fq "$text" "$file"; then
    echo "[brand-source-relation] missing '$text' in $file" >&2
    exit 1
  fi
}

reject_text() {
  local file="$1"
  local text="$2"
  if grep -Fq "$text" "$file"; then
    echo "[brand-source-relation] forbidden '$text' in $file" >&2
    exit 1
  fi
}

MODEL=src/mir/resolved_semantics/brand_source_relation.rs
SHADOW=src/mir/resolved_semantics/shadow/expr.rs
LIFECYCLE=src/mir/builder/normal_default_root_catalog_lifecycle.rs
RAW=src/mir/builder/calls/function_call_preflight_route.rs

require_text "$MODEL" "VerifiedBrandCallSourceRelationV1"
require_text "$MODEL" "BrandDeclarationSourceIdV1"
require_text "$SHADOW" "BrandCallSourceRelationKindV1::Constructor"
require_text "$SHADOW" "BrandCallSourceRelationKindV1::Unwrap"
require_text "$SHADOW" "UnsupportedBrandStaticMethod"
require_text "$LIFECYCLE" "with_brand_catalog"
require_text src/mir/resolved_semantics/owner_resolver.rs \
  "resolve_selected_callable_forests_with_body_shapes_and_brand_catalog"
require_text src/mir/resolved_semantics/owner_resolver.rs \
  "resolve_script_forest_with_declaration_views"

# The physical consumer/cutover is deliberately a later row.
require_text "$RAW" "is_brand_declared"
reject_text "$MODEL" "ValueId"

for file in \
  "$MODEL" \
  src/mir/resolved_semantics/brand_source_relation_tests.rs \
  src/mir/resolved_semantics/shadow/resolver.rs \
  src/mir/resolved_semantics/shadow/expr.rs \
  src/mir/resolved_semantics/resolver.rs \
  src/mir/resolved_semantics/owner_resolver.rs \
  "$LIFECYCLE"; do
  lines="$(wc -l < "$file")"
  if (( lines >= 760 )); then
    echo "[brand-source-relation] source split required: $file has $lines lines" >&2
    exit 1
  fi
done

CARGO_BUILD_JOBS=4 cargo test --profile quick -q -p nyash-rust \
  mir::resolved_semantics::brand_source_relation_tests --lib

echo "[brand-source-relation] OK"
