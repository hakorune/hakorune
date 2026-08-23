#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="parser-normal-root-preservation-a-i0"
source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

MODEL="$ROOT_DIR/src/parser/normal_callable_program_source/model.rs"
TRANSFORM="$ROOT_DIR/src/parser/normal_callable_program_source/transform.rs"
TRANSFORM_TESTS="$ROOT_DIR/src/parser/normal_callable_program_source/tests.rs"
PRESERVATION="$ROOT_DIR/src/parser/callable_parameter_source/normal_root_preservation.rs"
PARSER_MOD="$ROOT_DIR/src/parser/mod.rs"
SOURCE_MOD="$ROOT_DIR/src/parser/normal_callable_program_source/mod.rs"
PARAM_MOD="$ROOT_DIR/src/parser/callable_parameter_source/mod.rs"
README="$ROOT_DIR/src/parser/normal_callable_program_source/README.md"
CARD="$ROOT_DIR/docs/development/current/main/investigations/normal-main-root-preservation-a-i0-2026-08-23.md"
STATE="$ROOT_DIR/docs/development/current/main/CURRENT_STATE.toml"

guard_require_command "$TAG" rg
guard_require_command "$TAG" wc
guard_require_files "$TAG" "$MODEL" "$TRANSFORM" "$TRANSFORM_TESTS" "$PRESERVATION" \
  "$PARSER_MOD" "$SOURCE_MOD" "$PARAM_MOD" "$README" "$CARD" "$STATE"

for file in "$MODEL" "$TRANSFORM" "$TRANSFORM_TESTS" "$PRESERVATION"; do
  lines="$(wc -l < "$file" | tr -d '[:space:]')"
  if (( lines >= 760 )); then
    guard_fail "$TAG" "760-line split boundary exceeded: ${file#"$ROOT_DIR/"}=$lines"
  fi
done

guard_expect_fixed_in_file "$TAG" \
  "ParserNormalRootPreservationIssuerV1::seal_after_transform" \
  "$TRANSFORM" "final root preservation must have one named parser issuer"
if (( "$(rg -c 'ParserNormalRootPreservationIssuerV1::seal_after_transform' "$TRANSFORM")" != 1 )); then
  guard_fail "$TAG" "final root preservation issuer must have exactly one production call"
fi

guard_expect_fixed_in_file "$TAG" \
  "pub(crate) fn begin_transform(self) -> ParserNormalCallableTransformSessionV1" \
  "$MODEL" "prepared source must open the parser-owned transform session"
guard_expect_fixed_in_file "$TAG" \
  "pub(crate) fn finish(" "$MODEL" "transform session must own final-source issuance"
if rg -n 'pub\(crate\) fn issue_final_callable_program_source_v1|issue_final_callable_program_source_v1\(' \
  "$PARSER_MOD" "$SOURCE_MOD" "$ROOT_DIR/src/macro"; then
  guard_fail "$TAG" "free final-source production entry or macro direct call remains"
fi
guard_expect_fixed_in_file "$TAG" \
  "pub(super) fn issue_final_callable_program_source_v1(" \
  "$TRANSFORM" "only the parser parent may reach the internal final issuer"

if rg -n 'derive\([^)]*Clone[^)]*\).*ParserNormalRootPreservation|derive\([^)]*Clone[^)]*\).*ParserNormalRootPreserved' \
  "$PRESERVATION"; then
  guard_fail "$TAG" "root preservation token must remain move-only"
fi
guard_expect_fixed_in_file "$TAG" \
  "ParserNormalRootPreservationV1::Ready" "$TRANSFORM_TESTS" \
  "positive root-preservation evidence is missing"
for test_name in \
  root_prefix_structural_drift_is_rejected_before_final_source \
  foreign_transform_output_is_rejected_by_parser_session \
  second_static_main_in_transform_suffix_is_rejected; do
  guard_expect_fixed_in_file "$TAG" "$test_name" "$TRANSFORM_TESTS" \
    "focused negative evidence is missing: $test_name"
done

guard_expect_fixed_in_file "$TAG" \
  "ParserNormalRootPreservationV1" "$README" \
  "normal callable README must document the preserved root token"
guard_expect_fixed_in_file "$TAG" \
  "NORMAL-MAIN-ROOT-PRESERVATION-A-I0" "$CARD" \
  "A-I0 card is missing"
guard_expect_fixed_in_file "$TAG" \
  "ParserNormalRootPreservationV1" "$CARD" \
  "A-I0 card must retain the parser preservation contract"
guard_expect_fixed_in_file "$TAG" \
  "root consumer" "$CARD" \
  "A-I0 card must keep the root consumer outside this slice"
guard_expect_fixed_in_file "$TAG" \
  "a906f4aec2" "$CARD" \
  "A-I0 closeout commit is missing from the active card"

if rg -Fq 'current_execution_row = "NORMAL-MAIN-ROOT-PRESERVATION-A-I0"' "$STATE"; then
  guard_expect_fixed_in_file "$TAG" \
    'current_design_stop = "none: guarded parser root-preservation I0 is authorized' "$STATE" \
    "fast A-I0 state must keep the root consumer outside this slice"
else
  guard_expect_fixed_in_file "$TAG" \
    'current_execution_row = "NORMAL-MAIN-APP-ROOT-CONSUMER-D0"' "$STATE" \
    "closed A-I0 state must point to the next root-consumer design stop"
  guard_expect_fixed_in_file "$TAG" \
    'work_mode = "design_stop"' "$STATE" \
    "closed A-I0 state must return to design_stop"
fi

echo "[$TAG] ok"
