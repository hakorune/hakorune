#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

require() {
  grep -Fq "$2" "$1" || {
    echo "[instance-constructor-semantic] missing '$2' in $1" >&2
    exit 1
  }
}

PARSER=src/parser/constructor_source_catalog.rs
ISSUER=src/mir/normal_callable_semantic_package/instance_constructor_semantic.rs
PACKAGE=src/mir/normal_callable_semantic_package/model.rs
RAW=src/mir/builder/calls/function_call_preflight_route.rs

require "$PARSER" 'ConstructorSourceIdV1'
require "$PARSER" 'syntax_loan'
require "$ISSUER" 'resolve_selected_callable_forests_with_body_shapes_and_brand_catalog'
require "$ISSUER" 'ReceiverPolicyV1::DeclaredInstance'
require "$PACKAGE" 'instance_constructors'
# Physical consumption and the legacy probe retirement are later rows.
require "$RAW" 'is_brand_declared'

for file in "$PARSER" "$ISSUER" "$PACKAGE"; do
  lines="$(wc -l < "$file")"
  if (( lines >= 760 )); then
    echo "[instance-constructor-semantic] source split required: $file has $lines lines" >&2
    exit 1
  fi
done

CARGO_BUILD_JOBS=4 cargo test --profile quick -q -p nyash-rust \
  instance_constructor_semantics_keep_parser_identity_and_nested_brand_relations --lib

echo "[instance-constructor-semantic] OK"
