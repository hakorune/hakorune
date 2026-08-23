#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="parser-normal-root-preservation-a-i0"
source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

MODEL="$ROOT_DIR/src/parser/normal_callable_program_source/model.rs"
TRANSFORM="$ROOT_DIR/src/parser/normal_callable_program_source/transform.rs"
TRANSFORM_TESTS="$ROOT_DIR/src/parser/normal_callable_program_source/tests.rs"
PRESERVATION="$ROOT_DIR/src/parser/callable_parameter_source/normal_root_preservation.rs"
MACRO_TRANSFORM="$ROOT_DIR/src/macro/normal_callable_transform.rs"
MACRO_TRANSFORM_TESTS="$ROOT_DIR/src/macro/normal_callable_transform_tests.rs"
TEST_HARNESS="$ROOT_DIR/src/macro/test_harness.rs"
PARSER_MOD="$ROOT_DIR/src/parser/mod.rs"
SOURCE_MOD="$ROOT_DIR/src/parser/normal_callable_program_source/mod.rs"
PARAM_MOD="$ROOT_DIR/src/parser/callable_parameter_source/mod.rs"
README="$ROOT_DIR/src/parser/normal_callable_program_source/README.md"
CARD="$ROOT_DIR/docs/development/current/main/investigations/normal-main-root-preservation-a-i0-2026-08-23.md"
ACTIVE_CARD="$ROOT_DIR/docs/development/current/main/investigations/normal-main-app-root-consumer-d0-2026-08-23.md"
STATE="$ROOT_DIR/docs/development/current/main/CURRENT_STATE.toml"

guard_require_command "$TAG" rg
guard_require_command "$TAG" wc
guard_require_files "$TAG" "$MODEL" "$TRANSFORM" "$TRANSFORM_TESTS" "$PRESERVATION" \
  "$MACRO_TRANSFORM" "$MACRO_TRANSFORM_TESTS" "$TEST_HARNESS" "$PARSER_MOD" \
  "$SOURCE_MOD" "$PARAM_MOD" "$README" "$CARD" "$ACTIVE_CARD" "$STATE"

for file in "$MODEL" "$TRANSFORM" "$TRANSFORM_TESTS" "$PRESERVATION" \
  "$MACRO_TRANSFORM" "$MACRO_TRANSFORM_TESTS" "$TEST_HARNESS"; do
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
  "pub(crate) fn finish_exact(" "$MODEL" \
  "transform session must expose only the exact production finish"
if rg -n 'pub\(crate\) fn finish\(' "$MODEL"; then
  guard_fail "$TAG" "production raw-AST transform callback must remain absent"
fi
if ! rg -U -q '#\[cfg\(test\)\]\n    pub\(super\) fn finish_test_transform' "$MODEL"; then
  guard_fail "$TAG" "raw-AST transform hook must remain test-only"
fi
if rg -n 'pub\(crate\) fn issue_final_callable_program_source_v1|issue_final_callable_program_source_v1\(' \
  "$PARSER_MOD" "$SOURCE_MOD" "$ROOT_DIR/src/macro"; then
  guard_fail "$TAG" "free final-source production entry or macro direct call remains"
fi
guard_expect_fixed_in_file "$TAG" \
  "pub(super) fn issue_exact_callable_program_source_v1(" \
  "$TRANSFORM" "only the parser parent may reach the internal exact issuer"
if (( "$(rg -c 'issue_exact_callable_program_source_v1' "$MODEL")" != 1 )); then
  guard_fail "$TAG" "transform session must call the exact final-source issuer once"
fi
if (( "$(rg -c '\.finish_exact\(\)' "$MACRO_TRANSFORM")" != 1 )); then
  guard_fail "$TAG" "normal callable production must have one exact finish caller"
fi
if rg -n '\.finish\s*\(' "$MACRO_TRANSFORM"; then
  guard_fail "$TAG" "normal callable production must not pass a raw transform callback"
fi

guard_expect_fixed_in_file "$TAG" \
  "pub(super) enum TestHarnessTransformDispositionV1" "$TEST_HARNESS" \
  "test-harness owner must expose one closed transform disposition"
guard_expect_fixed_in_file "$TAG" \
  "pub(super) fn issue_test_harness_transform_v1" "$TEST_HARNESS" \
  "test-harness owner must retain the generated-tail issuer"
if rg -n 'maybe_inject_test_harness' "$ROOT_DIR/src/macro"; then
  guard_fail "$TAG" "retired AST-only test-harness wrapper must remain absent"
fi
if (( "$(rg -c 'return TestHarnessTransformDispositionV1::GeneratedTail' "$TEST_HARNESS")" != 1 )); then
  guard_fail "$TAG" "actual generated-tail construction must have one issuer"
fi
guard_expect_fixed_in_file "$TAG" \
  "NormalCallableTransformCompatibilityV1::TestHarnessGeneratedTail" \
  "$MACRO_TRANSFORM" "generated tail must enter typed compatibility"
guard_expect_fixed_in_file "$TAG" \
  "NormalCallableTransformRejectV1::UnclassifiedSourceMutation" \
  "$MACRO_TRANSFORM" "unknown macro mutation must be a typed reject"
if rg -n 'fallback|retry|or_else|unwrap_or_else' "$MACRO_TRANSFORM"; then
  guard_fail "$TAG" "normal callable transform must not add fallback or retry"
fi
if rg -n 'config::env|std::env|env::var' "$MODEL" "$TRANSFORM" "$PRESERVATION"; then
  guard_fail "$TAG" "parser preservation owners must not reread transform environment"
fi

generated_line="$(rg -n 'TestHarnessTransformDispositionV1::GeneratedTail' "$MACRO_TRANSFORM" | head -n 1 | cut -d: -f1)"
session_line="$(rg -n 'initial\.begin_transform\(\)' "$MACRO_TRANSFORM" | cut -d: -f1)"
if [[ -z "$generated_line" || -z "$session_line" ]] || (( generated_line >= session_line )); then
  guard_fail "$TAG" "generated-tail disposition must terminate before parser final-source issuance"
fi

if rg -n 'derive\([^)]*Clone[^)]*\).*ParserNormalRootPreservation|derive\([^)]*Clone[^)]*\).*ParserNormalRootPreserved' \
  "$PRESERVATION"; then
  guard_fail "$TAG" "root preservation token must remain move-only"
fi
guard_expect_fixed_in_file "$TAG" \
  "ParserNormalRootPreservationV1::Ready" "$TRANSFORM_TESTS" \
  "positive root-preservation evidence is missing"
for test_name in \
  root_statement_replacement_is_rejected_before_final_source \
  root_statement_addition_is_rejected_before_final_source \
  root_statement_removal_is_rejected_before_final_source \
  root_statement_reorder_is_rejected_before_final_source \
  foreign_transform_output_is_rejected_by_parser_session; do
  guard_expect_fixed_in_file "$TAG" "$test_name" "$TRANSFORM_TESTS" \
    "focused negative evidence is missing: $test_name"
done
guard_expect_fixed_in_file "$TAG" \
  "SourceBodyCardinalityMismatch" "$PRESERVATION" \
  "root issuer must compare source, initial, and transformed cardinality"
guard_expect_fixed_in_file "$TAG" \
  "SourceStatementChanged" "$PRESERVATION" \
  "root issuer must reject exact statement drift"
if rg -n 'SourcePrefix|is_static_main_box|transformed_statements\[[^]]*\.\.' "$PRESERVATION"; then
  guard_fail "$TAG" "prefix-only or name-based suffix preservation must remain retired"
fi
for test_name in \
  enabled_macro_with_no_actual_test_tail_stays_source_backed \
  actual_test_harness_tail_enters_typed_compatibility \
  composite_ready_test_tail_rejects_compatibility_loss \
  unclassified_macro_mutation_is_not_exact_or_compatibility; do
  guard_expect_fixed_in_file "$TAG" "$test_name" "$MACRO_TRANSFORM_TESTS" \
    "focused transform-disposition evidence is missing: $test_name"
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
guard_expect_fixed_in_file "$TAG" \
  "NORMAL-CALLABLE-SOURCE-TRANSFORM-DISPOSITION-I0" "$ACTIVE_CARD" \
  "active root-consumer card must retain the G0 prerequisite"
guard_expect_fixed_in_file "$TAG" \
  "TestHarnessGeneratedTail" "$ACTIVE_CARD" \
  "active card must retain the generated-tail compatibility contract"

if rg -Fq 'current_execution_row = "NORMAL-MAIN-ROOT-PRESERVATION-A-I0"' "$STATE"; then
  guard_expect_fixed_in_file "$TAG" \
    'current_design_stop = "none: guarded parser root-preservation I0 is authorized' "$STATE" \
    "fast A-I0 state must keep the root consumer outside this slice"
else
  guard_expect_fixed_in_file "$TAG" \
    'current_execution_design = "docs/development/current/main/investigations/normal-main-app-root-consumer-d0-2026-08-23.md"' \
    "$STATE" "closed A-I0 state must remain in the accepted root-consumer series"
fi

echo "[$TAG] ok"
