#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
TAG="frontend-main-app-entry-i0"
source "$ROOT/tools/checks/lib/guard_common.sh"

SOURCE="$ROOT/src/parser/callable_parameter_source/main_app_entry.rs"
TESTS="$ROOT/src/parser/callable_parameter_source/main_app_entry_tests.rs"
PRODUCT="$ROOT/src/parser/callable_parameter_source/product.rs"
STATIC_SOURCE="$ROOT/src/parser/callable_parameter_source/static_box_source.rs"
PARAM_MODEL="$ROOT/src/parser/callable_parameter_source/model.rs"
CARD="$ROOT/docs/development/current/main/investigations/normal-main-app-entry-admission-i0-2026-08-23.md"
INDEX="$ROOT/docs/tools/check-scripts-index.md"

guard_require_command "$TAG" rg
guard_require_command "$TAG" wc
guard_require_files "$TAG" "$SOURCE" "$TESTS" "$PRODUCT" "$STATIC_SOURCE" \
  "$PARAM_MODEL" "$CARD" "$INDEX"

[[ "$(rg -c 'pub\(in crate::parser\) fn issue_parser_main_app_entry_v1' "$SOURCE")" == 1 ]]
[[ "$(rg -c 'issue_parser_main_app_entry_v1\(&completed, &parameter_source\)' "$PRODUCT")" == 1 ]]
[[ "$(rg -c 'main_app_entry: ParserMainAppEntryDispositionV1' "$PRODUCT")" == 1 ]]
[[ "$(rg -c 'fn main_app_entry\(' "$PRODUCT")" == 1 ]]
[[ "$(rg -c 'method_site: SourceBoxMethodSiteV1' "$STATIC_SOURCE")" == 1 ]]
[[ "$(rg -c 'callable_identity: CallableDeclarationIdentityV1' "$PARAM_MODEL")" == 2 ]]

if rg -n 'ASTNode|NormalCompileRequest|MirBuilder|VerifiedRawRootExpansion|root_is_app_mode|Recipe|Join|MIR|fallback|retry|parse_from_string' "$SOURCE"; then
  guard_fail "$TAG" "Main/App issuer leaked AST/downstream/reselection authority"
fi

if rg -n 'ParserMainAppEntryDispositionV1::' "$PRODUCT"; then
  guard_fail "$TAG" "Main/App disposition is constructed outside the named issuer"
fi

for needle in \
  'one_static_main_zero_is_parser_ready_and_relation_bound' \
  'ordinary_program_is_not_app_main' \
  'non_main_static_box_is_explicit_outside' \
  'nonzero_main_arity_is_explicit_outside' \
  'mixed_program_is_outside_without_main_reselection' \
  'multiple_static_parents_remain_explicit_outside' \
  'unsupported_main_member_remains_explicit_outside'; do
  rg -q "$needle" "$TESTS" || guard_fail "$TAG" "missing focused evidence: $needle"
done

rg -q 'NORMAL-GENERAL-PROGRAM-PARSER-MAIN-APP-ENTRY-ADMISSION-I0' "$CARD"
rg -q 'frontend_main_app_entry_i0_guard.sh' "$INDEX"

for file in "$SOURCE" "$TESTS" "$PRODUCT" "$STATIC_SOURCE"; do
  lines="$(wc -l < "$file" | tr -d '[:space:]')"
  (( lines < 760 )) || guard_fail "$TAG" "source crossed split trigger: $file ($lines)"
done

echo "[$TAG] one parser issuer=1"
echo "[$TAG] sibling product transport=1"
echo "[$TAG] downstream/reselection authority=0"
echo "[$TAG] focused parser evidence=1"
echo "[$TAG] source-size limits=1"
echo "[$TAG] PASS"
