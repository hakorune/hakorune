#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
TAG="frontend-normal-root-source-transport-i0"
source "$ROOT/tools/checks/lib/guard_common.sh"

ISSUER="$ROOT/src/parser/callable_parameter_source/main_app_entry.rs"
PRODUCT="$ROOT/src/parser/callable_parameter_source/product.rs"
POSTPASS="$ROOT/src/parser/postpass_envelope.rs"
MODEL="$ROOT/src/parser/normal_callable_program_source/model.rs"
TRANSFORM="$ROOT/src/parser/normal_callable_program_source/transform.rs"
RETAINED="$ROOT/src/parser/callable_parameter_source/retained.rs"
ROOT_SOURCE="$ROOT/src/parser/callable_parameter_source/normal_root_source.rs"
NORMAL_TESTS="$ROOT/src/parser/normal_callable_program_source/tests.rs"
RETAINED_TESTS="$ROOT/src/parser/callable_parameter_source/retained_tests.rs"
ROOT_TESTS="$ROOT/src/parser/callable_parameter_source/normal_root_source_tests.rs"
CARD="$ROOT/docs/development/current/main/investigations/normal-main-app-root-source-disposition-d0-2026-08-23.md"
README="$ROOT/src/parser/normal_callable_program_source/README.md"
INDEX="$ROOT/docs/tools/check-scripts-index.md"

guard_require_command "$TAG" rg
guard_require_command "$TAG" wc
guard_require_files "$TAG" "$ISSUER" "$PRODUCT" "$POSTPASS" "$MODEL" \
  "$TRANSFORM" "$RETAINED" "$ROOT_SOURCE" "$NORMAL_TESTS" "$RETAINED_TESTS" \
  "$ROOT_TESTS" "$CARD" \
  "$README" "$INDEX"

[[ "$(rg -c 'pub\(in crate::parser\) fn issue_parser_main_app_entry_v1' "$ISSUER")" == 1 ]]
[[ "$(rg -c 'pub\(super\) fn issue_parser_normal_root_source_v1' "$ROOT_SOURCE")" == 1 ]]
[[ "$(rg -c '^    normal_root_source: ParserNormalRootSourceDispositionV1' "$PRODUCT")" == 1 ]]
[[ "$(rg -c '^    normal_root_source: ParserNormalRootSourceDispositionV1' "$MODEL")" == 2 ]]
[[ "$(rg -c '^    normal_root_source: ParserNormalRootSourceDispositionV1' "$RETAINED")" == 1 ]]
rg -q 'NormalCallableProgramAdmissionV1::SourceBacked\(normal_root_source\)' "$POSTPASS"
rg -q 'normal_root_source,\s*$' "$TRANSFORM"

if rg -n '^    main_app_entry:|^    canonical_script_admission:' "$PRODUCT" "$POSTPASS" "$MODEL" \
  "$TRANSFORM" "$RETAINED"; then
  guard_fail "$TAG" "parallel App/Script root fields remain outside the unified disposition"
fi

if rg -n 'NormalCompileRequestV1|MirBuilder|VerifiedRawRootExpansionV1|root_is_app_mode|fallback|retry' \
  "$POSTPASS" "$MODEL" "$TRANSFORM" "$RETAINED"; then
  guard_fail "$TAG" "transport I0 leaked downstream or re-observation authority"
fi

for needle in \
  'main_app_disposition_moves_through_prepared_and_final_source' \
  'main_app_non_ready_disposition_moves_without_reclassification' \
  'retained_source_keeps_main_app_disposition' \
  'retained_source_keeps_typed_non_ready_disposition' \
  'pure_script_has_one_same_invocation_root_witness' \
  'app_ready_cannot_be_discarded_into_script_a' \
  'script_a_discard_is_explicit_and_keeps_rows_separate'; do
  rg -q "$needle" "$NORMAL_TESTS" "$RETAINED_TESTS" "$ROOT_TESTS" \
    || guard_fail "$TAG" "missing focused evidence: $needle"
done

rg -q 'NORMAL-MAIN-APP-ROOT-SOURCE-DISPOSITION-I0' "$CARD"
rg -q 'frontend_main_app_entry_transport_i0_guard.sh' "$INDEX"

for file in "$ISSUER" "$PRODUCT" "$POSTPASS" "$MODEL" "$TRANSFORM" "$RETAINED" \
  "$ROOT_SOURCE" "$NORMAL_TESTS" "$RETAINED_TESTS" "$ROOT_TESTS"; do
  lines="$(wc -l < "$file" | tr -d '[:space:]')"
  (( lines < 760 )) || guard_fail "$TAG" "source crossed split trigger: $file ($lines)"
done

echo "[$TAG] one parser root co-seal issuer=1"
echo "[$TAG] one unified root field through prepared/final/retained=1"
echo "[$TAG] Script rows remain A-only and AppReady A reject=1"
echo "[$TAG] no downstream/reclassification/fallback edge=1"
echo "[$TAG] focused root transport/discard evidence=1"
echo "[$TAG] source-size limits=1"
echo "[$TAG] PASS"
