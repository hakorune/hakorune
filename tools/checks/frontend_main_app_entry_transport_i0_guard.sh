#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
TAG="frontend-main-app-entry-transport-i0"
source "$ROOT/tools/checks/lib/guard_common.sh"

ISSUER="$ROOT/src/parser/callable_parameter_source/main_app_entry.rs"
PRODUCT="$ROOT/src/parser/callable_parameter_source/product.rs"
POSTPASS="$ROOT/src/parser/postpass_envelope.rs"
MODEL="$ROOT/src/parser/normal_callable_program_source/model.rs"
TRANSFORM="$ROOT/src/parser/normal_callable_program_source/transform.rs"
RETAINED="$ROOT/src/parser/callable_parameter_source/retained.rs"
NORMAL_TESTS="$ROOT/src/parser/normal_callable_program_source/tests.rs"
RETAINED_TESTS="$ROOT/src/parser/callable_parameter_source/retained_tests.rs"
CARD="$ROOT/docs/development/current/main/investigations/normal-main-app-entry-transport-d0-2026-08-23.md"
README="$ROOT/src/parser/normal_callable_program_source/README.md"
INDEX="$ROOT/docs/tools/check-scripts-index.md"

guard_require_command "$TAG" rg
guard_require_command "$TAG" wc
guard_require_files "$TAG" "$ISSUER" "$PRODUCT" "$POSTPASS" "$MODEL" \
  "$TRANSFORM" "$RETAINED" "$NORMAL_TESTS" "$RETAINED_TESTS" "$CARD" \
  "$README" "$INDEX"

[[ "$(rg -c 'pub\(in crate::parser\) fn issue_parser_main_app_entry_v1' "$ISSUER")" == 1 ]]
[[ "$(rg -c 'into_normal_callable_program_with_main_app_entry' "$PRODUCT")" == 1 ]]
[[ "$(rg -c '^    main_app_entry: ParserMainAppEntryDispositionV1' "$MODEL")" == 2 ]]
[[ "$(rg -c '^    main_app_entry: ParserMainAppEntryDispositionV1' "$RETAINED")" == 1 ]]
rg -q 'NormalCallableProgramAdmissionV1::SourceBacked\(main_app_entry\)' "$POSTPASS"
rg -q 'main_app_entry,\s*$' "$TRANSFORM"

if rg -n 'ParserMainAppEntryDispositionV1::' "$PRODUCT" "$POSTPASS" "$MODEL" \
  "$TRANSFORM" "$RETAINED"; then
  guard_fail "$TAG" "Main/App disposition is reclassified outside the parser issuer"
fi

if rg -n 'NormalCompileRequestV1|MirBuilder|VerifiedRawRootExpansionV1|root_is_app_mode|fallback|retry' \
  "$POSTPASS" "$MODEL" "$TRANSFORM" "$RETAINED"; then
  guard_fail "$TAG" "transport I0 leaked downstream or re-observation authority"
fi

for needle in \
  'main_app_disposition_moves_through_prepared_and_final_source' \
  'main_app_non_ready_disposition_moves_without_reclassification' \
  'retained_source_keeps_main_app_disposition' \
  'retained_source_keeps_typed_non_ready_disposition'; do
  rg -q "$needle" "$NORMAL_TESTS" "$RETAINED_TESTS" \
    || guard_fail "$TAG" "missing focused evidence: $needle"
done

rg -q 'NORMAL-GENERAL-PROGRAM-PARSER-MAIN-APP-ENTRY-TRANSPORT-I0' "$CARD"
rg -q 'frontend_main_app_entry_transport_i0_guard.sh' "$INDEX"

for file in "$ISSUER" "$PRODUCT" "$POSTPASS" "$MODEL" "$TRANSFORM" "$RETAINED" \
  "$NORMAL_TESTS" "$RETAINED_TESTS"; do
  lines="$(wc -l < "$file" | tr -d '[:space:]')"
  (( lines < 760 )) || guard_fail "$TAG" "source crossed split trigger: $file ($lines)"
done

echo "[$TAG] one parser issuer=1"
echo "[$TAG] prepared/final/retained move fields=1"
echo "[$TAG] no downstream/reclassification edge=1"
echo "[$TAG] focused transport evidence=1"
echo "[$TAG] source-size limits=1"
echo "[$TAG] PASS"
