#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
TAG="frontend-static-box-parent-source-i0"
source "$ROOT/tools/checks/lib/guard_common.sh"

SOURCE="$ROOT/src/parser/callable_parameter_source/static_box_source.rs"
TESTS="$ROOT/src/parser/callable_parameter_source/static_box_source_tests.rs"
PARSER="$ROOT/src/parser/declarations/static_def/mod.rs"
FINALIZER="$ROOT/src/parser/source_seal/finalize.rs"
POSTPASS="$ROOT/src/parser/postpass_envelope.rs"
README="$ROOT/src/parser/callable_parameter_source/README.md"
CARD="$ROOT/docs/development/current/main/investigations/normal-static-box-parent-source-d0-2026-08-23.md"
INDEX="$ROOT/docs/tools/check-scripts-index.md"

guard_require_command "$TAG" rg
guard_require_command "$TAG" wc
guard_require_files "$TAG" "$SOURCE" "$TESTS" "$PARSER" "$FINALIZER" \
  "$POSTPASS" "$README" "$CARD" "$INDEX"

[[ "$(rg -c 'struct ParserStaticBoxParentSourceAuthorityIssuerV1' "$SOURCE")" == 1 ]]
[[ "$(rg -c 'pub\(in crate::parser\) fn issue_once' "$SOURCE")" == 1 ]]
[[ "$(rg -c 'ParserStaticBoxParentSourceAuthorityIssuerV1::issue_once' "$FINALIZER")" == 1 ]]
[[ "$(rg -c 'OpenParserStaticBoxSourceTransactionV1::open' "$PARSER")" == 1 ]]
[[ "$(rg -c 'register_prepared_static_box_source' "$PARSER")" == 1 ]]
rg -q 'static_box_parent_source: ParserStaticBoxParentSourceDispositionV1' "$POSTPASS"

if rg -n 'ASTNode|NormalCompileRequest|MirBuilder|ValueId|BasicBlockId|MirType|Recipe|Join|fallback|retry|parse_from_string|static[[:space:]]+Main' "$SOURCE"; then
  guard_fail "$TAG" "static parent issuer leaked downstream/name/reparse authority"
fi

if rg -n 'use .*ParserBoxSourceSealV1|ParserBoxSourceSealV1[[:space:]]*\{|SourceSealedOrdinary|BoxCallableRegistry|runtime' "$SOURCE"; then
  guard_fail "$TAG" "static parent source was merged with ordinary/runtime authority"
fi

for needle in \
  'bounded_static_box_parent_issues_one_parser_owned_ready_seal' \
  'unsupported_static_parent_member_is_explicit_outside' \
  'multiple_static_methods_are_outside_the_first_cohort' \
  'ordinary_source_path_does_not_reuse_static_parent_seal' \
  'mixed_program_is_outside_without_static_parent_repair'; do
  rg -q "$needle" "$TESTS" || guard_fail "$TAG" "missing focused evidence: $needle"
done

rg -q 'ParserStaticBoxParentSourceAuthorityIssuerV1::issue_once' "$README"
rg -q 'NORMAL-GENERAL-PROGRAM-PARSER-STATIC-BOX-PARENT-SOURCE-I0' "$CARD"
rg -q 'frontend_static_box_parent_source_i0_guard.sh' "$INDEX"

for file in "$SOURCE" "$TESTS"; do
  lines="$(wc -l < "$file" | tr -d '[:space:]')"
  (( lines < 760 )) || guard_fail "$TAG" "source crossed split trigger: $file ($lines)"
done

echo "[$TAG] sole parser issuer=1"
echo "[$TAG] static source sibling transport=1"
echo "[$TAG] downstream/name/reparse authority=0"
echo "[$TAG] focused parser evidence=1"
echo "[$TAG] source-size limits=1"
echo "[$TAG] PASS"
