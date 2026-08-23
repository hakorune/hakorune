#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="parser-normal-root-preservation-a-i0"
source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

MODEL="$ROOT_DIR/src/parser/normal_callable_program_source/model.rs"
TRANSFORM="$ROOT_DIR/src/parser/normal_callable_program_source/transform.rs"
TRANSFORM_TESTS="$ROOT_DIR/src/parser/normal_callable_program_source/tests.rs"
ROOT_RELATION_TESTS="$ROOT_DIR/src/parser/normal_callable_program_source/normal_root_preservation_tests.rs"
PRESERVATION="$ROOT_DIR/src/parser/callable_parameter_source/normal_root_preservation.rs"
CONSUMER_LOAN="$ROOT_DIR/src/parser/callable_parameter_source/normal_root_preservation/consumer_loan.rs"
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
LIFECYCLE="$ROOT_DIR/src/mir/builder/normal_default_root_catalog_lifecycle.rs"

guard_require_command "$TAG" rg
guard_require_command "$TAG" wc
guard_require_files "$TAG" "$MODEL" "$TRANSFORM" "$TRANSFORM_TESTS" "$ROOT_RELATION_TESTS" "$PRESERVATION" \
  "$MACRO_TRANSFORM" "$MACRO_TRANSFORM_TESTS" "$TEST_HARNESS" "$PARSER_MOD" \
  "$SOURCE_MOD" "$PARAM_MOD" "$README" "$CARD" "$ACTIVE_CARD" "$STATE" \
  "$CONSUMER_LOAN" "$LIFECYCLE"

for file in "$MODEL" "$TRANSFORM" "$TRANSFORM_TESTS" "$ROOT_RELATION_TESTS" "$PRESERVATION" \
  "$MACRO_TRANSFORM" "$MACRO_TRANSFORM_TESTS" "$TEST_HARNESS" "$CONSUMER_LOAN"; do
  lines="$(wc -l < "$file" | tr -d '[:space:]')"
  if (( lines >= 760 )); then
    guard_fail "$TAG" "760-line split boundary exceeded: ${file#"$ROOT_DIR/"}=$lines"
  fi
done

guard_expect_fixed_in_file "$TAG" \
  "pub(crate) fn with_parser_normal_root_consumer_loan<R>" \
  "$CONSUMER_LOAN" "root view must have one parser-owned scoped issuer"
if (( "$(rg -c 'pub\(crate\) fn with_parser_normal_root_consumer_loan<R>' "$CONSUMER_LOAN")" != 1 )); then
  guard_fail "$TAG" "parser root-loan issuer definition must be exactly one"
fi
guard_expect_fixed_in_file "$TAG" \
  "pub(crate) fn with_normal_root_consumer_loan<R>" \
  "$MODEL" "final source owner must expose one thin root-loan facade"
if (( "$(rg -c 'pub\(crate\) fn with_normal_root_consumer_loan<R>' "$MODEL")" != 1 )); then
  guard_fail "$TAG" "final-source root-loan facade definition must be exactly one"
fi
guard_expect_fixed_in_file "$TAG" \
  "impl for<'source> FnOnce(ParserNormalRootConsumerLoanV1<'source>) -> R" \
  "$MODEL" "root-loan facade must retain the HRTB callback"

root_loan_callers="$(rg -n '\.with_normal_root_consumer_loan\(' "$ROOT_DIR/src" \
  --glob '*.rs' --glob '!*tests.rs' || true)"
root_loan_caller_count="$(printf '%s\n' "$root_loan_callers" | sed '/^$/d' | wc -l | tr -d '[:space:]')"
if rg -Fq 'struct PreparedNormalDefaultProgramRootAfterLoanV1' "$LIFECYCLE"; then
  if (( root_loan_caller_count != 1 )); then
    printf '%s\n' "$root_loan_callers" >&2
    guard_fail "$TAG" "I0 lifecycle must own exactly one production root-loan call"
  fi
elif (( root_loan_caller_count != 0 )); then
  printf '%s\n' "$root_loan_callers" >&2
  guard_fail "$TAG" "caller-zero S0 must not have a production root-loan call"
fi

for symbol in \
  ParserNormalRootConsumerLoanV1 \
  ParserNormalAppRootLoanV1 \
  ParserNormalAppProgramCursorV1 \
  ParserNormalScriptRootLoanV1 \
  ParserNormalScriptStatementCursorV1; do
  if rg -U -n "derive\\([^)]*Clone[^)]*\\)[[:space:]]*pub\\(crate\\) (enum|struct) ${symbol}" \
    "$CONSUMER_LOAN"; then
    guard_fail "$TAG" "root loan/cursor must remain non-Clone: $symbol"
  fi
done
if rg -n 'pub\(crate\).*fn (program|source_row|position|parser_[a-z_]*|[a-z_]*ordinal)\(|as \*const|usize_from_ptr' \
  "$CONSUMER_LOAN"; then
  guard_fail "$TAG" "root view must not expose Program, source-row, identity, ordinal, or pointer access"
fi
if rg -n 'crate::mir|CanonicalScript|ValueId|BasicBlockId|MirType|Recipe|Join' "$CONSUMER_LOAN"; then
  guard_fail "$TAG" "parser root loan must not import Script-A, Recipe, Join, or MIR authority"
fi
guard_expect_fixed_in_file "$TAG" \
  "ParserNormalAppProgramItemLoanV1::RootMain" "$CONSUMER_LOAN" \
  "App cursor must hide the admitted Main declaration"
guard_expect_fixed_in_file "$TAG" \
  "ParserNormalScriptStatementLoanV1" "$CONSUMER_LOAN" \
  "Script root must expose one paired statement loan"
for test_name in \
  app_root_consumer_loan_hides_main_and_lends_only_root_body \
  app_root_consumer_loan_preserves_sibling_then_root_main_order \
  empty_script_root_consumer_loan_is_complete_zero \
  nonempty_script_root_consumer_loan_keeps_paired_statement_order \
  nonzero_main_arity_rejects_before_root_consumer_callback; do
  guard_expect_fixed_in_file "$TAG" "$test_name" "$ROOT_RELATION_TESTS" \
    "focused root-loan evidence is missing: $test_name"
done
guard_expect_fixed_in_file "$TAG" "## Root consumer loan S0" "$README" \
  "normal callable README must document the scoped root view"
guard_expect_fixed_in_file "$TAG" "RootMain" "$README" \
  "normal callable README must document the opaque App cursor item"

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
guard_expect_fixed_in_file "$TAG" \
  "enum ParserNormalRootRelationV1" "$PRESERVATION" \
  "root token must use one closed App/Script relation"
guard_expect_fixed_in_file "$TAG" \
  "_app_entry: ParserMainAppEntrySealV1" "$PRESERVATION" \
  "App relation must move the existing private admission seal"
guard_expect_fixed_in_file "$TAG" \
  "_final_slot: InitialCallableFinalSlotV1" "$PRESERVATION" \
  "App relation must retain the already-paired final slot privately"
guard_expect_fixed_in_file "$TAG" \
  "_main_is_root: ParserCallableMainIsRootV1" "$PRESERVATION" \
  "App relation must co-seal callable Main as root"
guard_expect_fixed_in_file "$TAG" \
  "_no_static_children: ParserNoStaticChildrenV1" "$PRESERVATION" \
  "App relation must retain the no-static-children proof"
for error_name in \
  CallablePairingCardinalityMismatch \
  AppCallableIdentityMissing \
  AppCallableIdentityDuplicate \
  AppCallableParserMismatch \
  AppCallableKindMismatch \
  AppCallableSourceRelationMismatch \
  AppCallableFinalSlotMismatch; do
  guard_expect_fixed_in_file "$TAG" "$error_name" "$PRESERVATION" \
    "typed App relation rejection is missing: $error_name"
done
if rg -n 'Option<ParserNormalAppRootRelationV1>|derive\([^)]*Clone[^)]*\).*ParserNormalAppRootRelationV1' \
  "$PRESERVATION"; then
  guard_fail "$TAG" "App relation must remain closed and move-only"
fi
if rg -n 'diagnostic_name|==[[:space:]]*"(Main|main)"|Verified(Main|RawRoot)ExpansionV1|crate::mir' \
  "$PRESERVATION"; then
  guard_fail "$TAG" "App relation must not reclassify by name or depend on Builder/MIR"
fi
for test_name in \
  app_root_relation_accepts_exact_main_with_top_level_callable_sibling \
  app_root_relation_rejects_structurally_equal_foreign_callable_identity \
  app_root_relation_rejects_foreign_parser_witness_before_pairing \
  app_root_relation_rejects_unpaired_final_slot \
  app_root_relation_rejects_callable_pairing_cardinality_drift \
  main_helper_stays_terminal_before_app_root_relation; do
  guard_expect_fixed_in_file "$TAG" "$test_name" "$ROOT_RELATION_TESTS" \
    "focused App root-relation evidence is missing: $test_name"
done
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
