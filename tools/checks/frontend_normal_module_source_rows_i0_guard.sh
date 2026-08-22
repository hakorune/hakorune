#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
TAG="frontend-normal-module-source-rows-i0"
source "$ROOT/tools/checks/lib/guard_common.sh"

ISSUER="$ROOT/src/parser/callable_parameter_source/script_source_authority/module_rows.rs"
PARENT_ISSUER="$ROOT/src/parser/callable_parameter_source/script_source_authority/issuer.rs"
MODEL="$ROOT/src/parser/callable_parameter_source/script_source_authority/model.rs"
TRANSFORM="$ROOT/src/parser/callable_parameter_source/script_source_authority/transform_guard.rs"
POSTPASS="$ROOT/src/parser/postpass_envelope.rs"
PATH_MODEL="$ROOT/src/parser/source_path.rs"
PRODUCT="$ROOT/src/parser/callable_parameter_source/product.rs"
TESTS="$ROOT/src/parser/callable_parameter_source/script_source_authority/module_rows_tests.rs"
README="$ROOT/src/parser/callable_parameter_source/README.md"
CARD="$ROOT/docs/development/current/main/investigations/normal-module-parser-source-rows-d0-2026-08-23.md"
INDEX="$ROOT/docs/tools/check-scripts-index.md"

guard_require_command "$TAG" rg
guard_require_command "$TAG" wc
guard_require_files "$TAG" "$ISSUER" "$PARENT_ISSUER" "$MODEL" "$TRANSFORM" \
  "$POSTPASS" "$PATH_MODEL" "$PRODUCT" "$TESTS" "$README" "$CARD" "$INDEX"

[[ "$(rg -c 'pub\(super\) struct ParserNormalModuleSourceAuthorityIssuerV1' "$ISSUER")" == 1 ]]
[[ "$(rg -c 'pub\(super\) fn issue_once' "$ISSUER")" == 1 ]]
[[ "$(rg -c 'ParserNormalModuleSourceAuthorityIssuerV1::issue_once' "$PARENT_ISSUER")" == 1 ]]
[[ "$(rg -c 'source_sealed\(&self\)' "$POSTPASS")" == 1 ]]
[[ "$(rg -c 'box_method_parts' "$PATH_MODEL")" == 1 ]]
rg -q 'module_rows: ParserNormalModuleSourceRowsDispositionV1' "$MODEL"
[[ "$(rg -c 'let \(invocation, body_rows, composite, module_rows\) = authority.into_parts\(\)' "$TRANSFORM")" == 1 ]]
[[ "$(rg -c 'normal_module_source_rows' "$PRODUCT")" == 1 ]]

if rg -n 'ASTNode|NormalCompileRequest|MirBuilder|ValueId|BasicBlockId|MirType|Recipe|Join|fallback|retry|static[[:space:]]+Main|"Main"' "$ISSUER"; then
  guard_fail "$TAG" "module-row issuer leaked AST/downstream/name-based authority"
fi

if rg -n 'derive\([^)]*Clone' "$ISSUER" | rg 'ParserNormalModuleSourceRowsV1|ParserNormalModuleBoxSourceRowV1|ParserNormalModuleMethodSourceRowV1'; then
  guard_fail "$TAG" "module-row product became Cloneable"
fi

for needle in \
  'ordinary_box_and_direct_instance_method_form_one_source_row' \
  'multiple_ordinary_boxes_are_outside_the_bounded_row_cohort' \
  'static_box_entry_stops_on_missing_ordinary_source_seal'; do
  rg -q "$needle" "$TESTS" || guard_fail "$TAG" "missing focused evidence: $needle"
done

rg -q 'ParserNormalModuleSourceAuthorityIssuerV1::issue_once' "$README"
rg -q 'Incomplete\(BoxSourceSealMissing\)' "$README"
rg -q 'NORMAL-GENERAL-PROGRAM-PARSER-MODULE-ROWS-I0' "$CARD"
rg -q 'static Box parent source' "$CARD"
rg -q 'frontend_normal_module_source_rows_i0_guard.sh' "$INDEX"

for file in "$ISSUER" "$MODEL" "$PARENT_ISSUER" "$TRANSFORM"; do
  lines="$(wc -l < "$file" | tr -d '[:space:]')"
  (( lines < 800 )) || guard_fail "$TAG" "source crossed hard stop: $file ($lines)"
done
issuer_lines="$(wc -l < "$ISSUER" | tr -d '[:space:]')"
(( issuer_lines < 760 )) || guard_fail "$TAG" "issuer crossed split trigger: $issuer_lines"

echo "[$TAG] one issuer=1"
echo "[$TAG] AST/downstream authority=0"
echo "[$TAG] ordinary-only focused evidence=1"
echo "[$TAG] source-size limits=1"
echo "[$TAG] PASS"
