#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
TAG="normal-root-execution-reference-route"
source "$ROOT/tools/checks/lib/guard_common.sh"

SURFACE="$ROOT/src/parser/callable_parameter_source/normal_source_plan_surface.rs"
ROOT_ISSUER="$ROOT/src/parser/callable_parameter_source/normal_root_execution/issuer.rs"
ROOT_MODEL="$ROOT/src/parser/callable_parameter_source/normal_root_execution/model.rs"
ROOT_COMPAT="$ROOT/src/parser/callable_parameter_source/normal_root_execution/compatibility.rs"
ROOT_TESTS="$ROOT/src/parser/callable_parameter_source/normal_root_execution/tests.rs"
TEST_TERMINAL="$ROOT/src/parser/callable_parameter_source/normal_root_execution/test_terminal.rs"
PRODUCT="$ROOT/src/parser/callable_parameter_source/product.rs"
RETAINED="$ROOT/src/parser/callable_parameter_source/retained.rs"
RETAINED_TESTS="$ROOT/src/parser/callable_parameter_source/retained_tests.rs"
PLAN_CONSUMER="$ROOT/src/parser/callable_parameter_source/normal_source_plan_consumer.rs"
PRESERVATION="$ROOT/src/parser/normal_callable_program_source/normal_root_execution_preservation.rs"
PRESERVATION_TESTS="$ROOT/src/parser/normal_callable_program_source/normal_root_execution_preservation_tests.rs"
FINAL_SOURCE_MODEL="$ROOT/src/parser/normal_callable_program_source/model.rs"
FINAL_SOURCE_TESTS="$ROOT/src/parser/normal_callable_program_source/tests.rs"
TRANSFORM="$ROOT/src/parser/normal_callable_program_source/transform.rs"
POSTPASS="$ROOT/src/parser/postpass_envelope.rs"
POSTPASS_PROGRAM="$ROOT/src/parser/postpass_envelope/normal_callable_program.rs"
POSTPASS_ROWS="$ROOT/src/parser/postpass_envelope/source_rows.rs"
SOURCE_SEAL_FINALIZE="$ROOT/src/parser/source_seal/finalize.rs"
MACRO_TRANSFORM="$ROOT/src/macro/normal_callable_transform.rs"
MACRO_TRANSFORM_TESTS="$ROOT/src/macro/normal_callable_transform_tests.rs"
POLICY="$ROOT/src/mir/compiler/normal_source_plan/parser_bound_policy.rs"
POLICY_TESTS="$ROOT/src/mir/compiler/normal_source_plan/parser_bound_policy_tests.rs"
SOURCE_PLAN_ENTRY="$ROOT/src/runner/reference/normal_file_vm_frontdoor/source_plan_input.rs"
PARSER_HANDOFF="$ROOT/src/runner/reference/normal_file_vm_frontdoor/parser_source_handoff.rs"
RAW_HANDOFF="$ROOT/src/runner/reference/normal_file_vm_frontdoor/raw_source_handoff.rs"
FRONTDOOR="$ROOT/src/runner/reference/normal_file_vm_frontdoor.rs"
FRONTDOOR_TESTS="$ROOT/src/runner/reference/normal_file_vm_frontdoor/tests.rs"
ATOMIC_CUTOVER_TESTS="$ROOT/src/runner/reference/normal_file_vm_frontdoor/atomic_root_cutover_tests.rs"
BUILDER_CONSUMER="$ROOT/src/mir/builder/normal_root_execution/consumer.rs"
BUILDER_MODEL="$ROOT/src/mir/builder/normal_root_execution/model.rs"
BUILDER_TESTS="$ROOT/src/mir/builder/normal_root_execution/tests.rs"
BUILDER_PROJECTION="$ROOT/src/mir/builder/main_expansion/admitted_projection.rs"
PROGRAM_ROOT="$ROOT/src/mir/builder/normal_default_program_root.rs"
LIFECYCLE="$ROOT/src/mir/builder/normal_default_root_catalog_lifecycle.rs"
SEMANTIC_ISSUER="$ROOT/src/mir/normal_callable_semantic_package/issuer.rs"
README="$ROOT/src/parser/callable_parameter_source/README.md"
BUILDER_README="$ROOT/src/mir/builder/README.md"
CARD="$ROOT/docs/development/current/main/investigations/normal-root-execution-atomic-cutover-manifest-d0-2026-08-23.md"
INDEX="$ROOT/docs/tools/check-scripts-index.md"

FILES=(
  "$SURFACE" "$ROOT_ISSUER" "$ROOT_MODEL" "$ROOT_COMPAT" "$ROOT_TESTS" "$TEST_TERMINAL"
  "$PRODUCT" "$RETAINED" "$RETAINED_TESTS" "$PLAN_CONSUMER"
  "$PRESERVATION" "$PRESERVATION_TESTS" "$FINAL_SOURCE_MODEL" "$FINAL_SOURCE_TESTS"
  "$TRANSFORM" "$POSTPASS" "$POSTPASS_PROGRAM" "$POSTPASS_ROWS"
  "$SOURCE_SEAL_FINALIZE" "$MACRO_TRANSFORM" "$MACRO_TRANSFORM_TESTS"
  "$POLICY" "$POLICY_TESTS" "$SOURCE_PLAN_ENTRY"
  "$PARSER_HANDOFF" "$RAW_HANDOFF" "$FRONTDOOR" "$FRONTDOOR_TESTS"
  "$ATOMIC_CUTOVER_TESTS"
  "$BUILDER_CONSUMER" "$BUILDER_MODEL" "$BUILDER_TESTS" "$BUILDER_PROJECTION"
  "$PROGRAM_ROOT" "$LIFECYCLE"
  "$SEMANTIC_ISSUER" "$README" "$BUILDER_README" "$CARD" "$INDEX"
  "$ROOT/src/macro/normal_callable_transform.rs"
  "$ROOT/src/macro/normal_callable_transform_tests.rs"
  "$ROOT/src/mir/builder.rs"
  "$ROOT/src/mir/builder/callable_declaration_catalog/mod.rs"
  "$ROOT/src/mir/builder/callable_declaration_catalog/source_backed.rs"
  "$ROOT/src/mir/builder/main_expansion.rs"
  "$ROOT/src/mir/builder/main_expansion_tests.rs"
  "$ROOT/src/mir/builder/normal_default_program_root.rs"
  "$ROOT/src/mir/builder/normal_default_root_catalog_lifecycle_tests.rs"
  "$ROOT/src/mir/builder/normal_root_execution/mod.rs"
  "$ROOT/src/mir/builder/normal_script_semantic_source.rs"
  "$ROOT/src/mir/callable_semantic_batch/model.rs"
  "$ROOT/src/mir/compiler/canonical_core_dispatch.rs"
  "$ROOT/src/mir/compiler/canonical_core_source_plan_request.rs"
  "$ROOT/src/mir/compiler/canonical_script_source_a_input.rs"
  "$ROOT/src/mir/compiler/normal_source_plan/classifier.rs"
  "$ROOT/src/mir/compiler/normal_source_plan/main_source.rs"
  "$ROOT/src/mir/compiler/normal_source_plan/mod.rs"
  "$ROOT/src/mir/compiler/normal_source_plan/product.rs"
  "$ROOT/src/mir/compiler/normal_source_plan/rejection.rs"
  "$ROOT/src/mir/compiler/normal_source_plan/script_recipe.rs"
  "$ROOT/src/mir/mod.rs"
  "$ROOT/src/mir/normal_callable_semantic_package/brand_catalog_tests.rs"
  "$ROOT/src/mir/normal_callable_semantic_package/install.rs"
  "$ROOT/src/mir/normal_callable_semantic_package/install/signature_loan.rs"
  "$ROOT/src/mir/normal_callable_semantic_package/main_static_child_tests.rs"
  "$ROOT/src/mir/normal_callable_semantic_package/mod.rs"
  "$ROOT/src/mir/normal_callable_semantic_package/model.rs"
  "$ROOT/src/mir/normal_callable_semantic_package/resolver_deferred_tests.rs"
  "$ROOT/src/mir/normal_callable_semantic_package/tests.rs"
  "$ROOT/src/parser/callable_parameter_source/canonical_script_source_admission.rs"
  "$ROOT/src/parser/callable_parameter_source/composite_source/loan.rs"
  "$ROOT/src/parser/callable_parameter_source/composite_source/mod.rs"
  "$ROOT/src/parser/callable_parameter_source/composite_source/transform_guard.rs"
  "$ROOT/src/parser/callable_parameter_source/mod.rs"
  "$ROOT/src/parser/callable_parameter_source/normal_root_execution/mod.rs"
  "$ROOT/src/parser/callable_parameter_source/normal_source_plan_surface_tests.rs"
  "$ROOT/src/parser/callable_parameter_source/script_source_authority/loan.rs"
  "$ROOT/src/parser/callable_parameter_source/script_source_authority/model.rs"
  "$ROOT/src/parser/callable_parameter_source/script_source_authority/mod.rs"
  "$ROOT/src/parser/callable_parameter_source/script_source_authority/module_rows_tests.rs"
  "$ROOT/src/parser/callable_parameter_source/script_source_authority/transform_guard.rs"
  "$ROOT/src/parser/callable_parameter_source/script_source_rows.rs"
  "$ROOT/src/parser/callable_parameter_source/script_source_rows_model.rs"
  "$ROOT/src/parser/callable_parameter_source/script_source_rows_tests.rs"
  "$ROOT/src/parser/callable_parameter_source/static_box_source.rs"
  "$ROOT/src/parser/callable_parameter_source/static_box_source_tests.rs"
  "$ROOT/src/parser/callable_parameter_source/syntax_loan.rs"
  "$ROOT/src/parser/callable_parameter_source/tests.rs"
  "$ROOT/src/parser/callable_source_anchor.rs"
  "$ROOT/src/parser/mod.rs"
  "$ROOT/src/parser/normal_callable_program_source/mod.rs"
  "$ROOT/src/parser/public_api.rs"
  "$ROOT/src/parser/source_seal/gate_projection.rs"
  "$ROOT/src/runner/modes/common_util/normal_callable.rs"
  "$ROOT/src/runner/reference/normal_file_canonical_core_vm.rs"
  "$ROOT/src/runner/reference/normal_file_vm.rs"
  "$ROOT/src/runner/reference/normal_file_vm_frontdoor/script_source_input.rs"
  "$ROOT/src/runner/reference/normal_file_vm_frontdoor/source_plan_input_tests.rs"
)

guard_require_command "$TAG" rg
guard_require_command "$TAG" wc
guard_require_files "$TAG" "${FILES[@]}"

expect_fixed_count() {
  local expected="$1"
  local pattern="$2"
  local file="$3"
  local message="$4"
  local actual
  actual="$(rg -F -c -- "$pattern" "$file" || true)"
  if [[ "$actual" != "$expected" ]]; then
    guard_fail "$TAG" "$message: expected=$expected actual=${actual:-0} file=${file#"$ROOT/"}"
  fi
}

expect_global_fixed_count() {
  local expected="$1"
  local pattern="$2"
  local message="$3"
  local hits=()
  mapfile -t hits < <(rg -F -o --glob '*.rs' -- "$pattern" "$ROOT/src" || true)
  if [[ "${#hits[@]}" != "$expected" ]]; then
    guard_fail "$TAG" "$message: expected=$expected actual=${#hits[@]}"
  fi
}

# One parser surface, one total root relation, and one exact transform owner.
expect_fixed_count 1 \
  "pub(in crate::parser) struct ParserNormalSourcePlanSurfaceIssuerV1" \
  "$SURFACE" "parser source-surface issuer definition drifted"
expect_fixed_count 1 \
  "ParserNormalSourcePlanSurfaceIssuerV1::issue_once(" \
  "$PRODUCT" "parser source-surface issuer must have one product caller"
expect_fixed_count 1 \
  "pub(in crate::parser) struct ParserNormalRootExecutionIssuerV1" \
  "$ROOT_ISSUER" "total root issuer definition drifted"
expect_fixed_count 1 \
  "ParserNormalRootExecutionIssuerV1::issue_once(normal_source_plan_surface)" \
  "$PRODUCT" "total root issuer must have one product caller"
expect_fixed_count 1 \
  "pub(crate) struct ParserNormalRootExecutionPreservationIssuerV1" \
  "$PRESERVATION" "total preservation issuer definition drifted"
expect_fixed_count 2 \
  "ParserNormalRootExecutionPreservationIssuerV1::seal_after_transform(" \
  "$TRANSFORM" "preservation caller census must stay one production plus one negative probe"
expect_global_fixed_count 1 \
  "ParserNormalSourcePlanSurfaceIssuerV1::issue_once(" \
  "parser source-surface issuer gained another caller"
expect_global_fixed_count 3 \
  "ParserNormalRootExecutionIssuerV1::issue_once(" \
  "total root issuer caller census drifted (one production and two terminal probes)"
expect_global_fixed_count 2 \
  "ParserNormalRootExecutionPreservationIssuerV1::seal_after_transform(" \
  "preservation issuer caller census drifted (one production and one negative probe)"

# The parser-to-compiler boundary is one opaque owner and one scoped loan.
expect_fixed_count 1 \
  "pub(crate) struct ParserNormalRootSourcePlanConsumerV1" \
  "$PLAN_CONSUMER" "source-plan consumer definition drifted"
expect_fixed_count 1 \
  "ParserNormalRootSourcePlanConsumerV1::consume_once(source, lineage)" \
  "$SOURCE_PLAN_ENTRY" "canonical source-plan consumer must have one front-door caller"
expect_fixed_count 1 \
  "NormalSourcePlanClassifierV1::seal_parser_bound(input)" \
  "$SOURCE_PLAN_ENTRY" "parser-bound policy must have one production caller"
expect_fixed_count 1 \
  "pub(crate) fn observe_surface_once<R>(" \
  "$PLAN_CONSUMER" "source-plan surface must expose one consuming scoped loan"
expect_global_fixed_count 1 \
  "observe_surface_once(" \
  "source-plan surface must have exactly one loan call"
expect_fixed_count 1 \
  "pub(in crate::parser) fn observe_once<R>(" \
  "$TEST_TERMINAL" "parser tests must share one consuming observation terminal"
expect_global_fixed_count 27 \
  "ParserNormalRootExecutionTestTerminalV1::observe_once(" \
  "whole-product observation terminal caller census drifted"
expect_global_fixed_count 6 \
  "ParserNormalRootExecutionTestTerminalV1::consume_once(" \
  "whole-product syntax terminal caller census drifted"
expect_fixed_count 1 \
  "source.observe_test_terminal_once(callback)" \
  "$TEST_TERMINAL" "whole-product observation delegation drifted"
expect_fixed_count 1 \
  "source.consume_test_terminal_once(callback)" \
  "$TEST_TERMINAL" "whole-product syntax delegation drifted"
expect_global_fixed_count 1 \
  "ParserNormalRootSourcePlanConsumerV1::consume_once(" \
  "canonical source-plan consumer gained another production caller"
expect_global_fixed_count 2 \
  "NormalSourcePlanClassifierV1::seal_parser_bound(" \
  "parser-bound policy caller census drifted (one production and one test helper)"
if rg -n 'fn (static_box_parent_source|callable_parameter_source|normal_root_execution_for_test|canonical_script_source_rows_for_test|normal_root_execution_role_for_test|normal_module_source_rows|consume_script_rows_test_terminal_once)' \
  "$PRODUCT"; then
  guard_fail "$TAG" "parser product regained a borrow-only or partial test escape"
fi
if rg -n 'take_script_rows_once|consume_script_rows_test_terminal_once' \
  "$ROOT/src/parser/callable_parameter_source" -g '*.rs'; then
  guard_fail "$TAG" "parser tests regained a partial Script-A terminal"
fi

# Retained syntax has one test-only affine terminal and no repeatable owner loan.
expect_global_fixed_count 1 \
  "ParserNormalRootExecutionTestTerminalV1::consume_retained_once(" \
  "retained terminal must have exactly one test caller and no production caller"
expect_fixed_count 1 \
  "source.consume_retained_test_terminal_once(callback)" \
  "$TEST_TERMINAL" "retained test terminal delegation drifted"
expect_fixed_count 1 \
  "pub(super) fn consume_retained_test_terminal_once<R>(" \
  "$RETAINED" "retained owner must expose one consuming terminal"
expect_fixed_count 1 \
  "consume_retained_fields_at_named_test_terminal(" \
  "$RETAINED" "retained terminal must share one five-field epilogue"
if rg -n 'with_callable_declaration_syntax|repeatable loan|pub\([^)]*\)?[[:space:]]+fn [^(]+\(&self' \
  "$RETAINED"; then
  guard_fail "$TAG" "retained owner regained a repeatable borrow-only escape"
fi
if rg -n '#\[derive\([^]]*Clone|impl Clone|fn (into_parts|into_source|into_ast|source_ast)\b' \
  "$RETAINED"; then
  guard_fail "$TAG" "retained owner regained Clone or an owned-parts/AST escape"
fi
guard_expect_fixed_in_file "$TAG" \
  "callback: impl for<'source> FnOnce(" "$RETAINED" \
  "retained scoped loan must remain consuming FnOnce"
if rg -n 'pub\([^)]*\)?[[:space:]]+fn (into_parts|into_rows|into_bound)' \
  "$PLAN_CONSUMER" "$PARSER_HANDOFF"; then
  guard_fail "$TAG" "opaque parser boundary leaked a general owned-parts escape"
fi
for accessor in bound surface callable_syntax callable_syntax_rows; do
  if rg -n "pub\(crate\).*fn ${accessor}\\b" "$ROOT_MODEL" "$SURFACE"; then
    guard_fail "$TAG" "opaque parser boundary widened ${accessor} beyond crate::parser"
  fi
done
if rg -n 'ParserNormalSourcePlanUnsupportedKindV1::Box' "$SURFACE" "$PLAN_CONSUMER"; then
  guard_fail "$TAG" "parser SourceSurface regressed to a lossy Unsupported(Box) row"
fi

# Builder consumes the preserved total relation before selected-normal effects.
expect_fixed_count 1 \
  "pub(in crate::mir) struct NormalRootExecutionConsumerV1" \
  "$BUILDER_CONSUMER" "Builder root consumer definition drifted"
expect_fixed_count 1 \
  "NormalRootExecutionConsumerV1::consume_once(source)" \
  "$PROGRAM_ROOT" "normal/default facade must have one pre-effect root consumer"
expect_fixed_count 1 \
  "pub(super) fn consume_source_backed_root_once(self)" \
  "$PROGRAM_ROOT" "normal/default lifecycle facade definition drifted"
expect_fixed_count 1 \
  ".consume_source_backed_root_once()" \
  "$LIFECYCLE" "normal/default lifecycle must consume its closed facade once"
expect_global_fixed_count 3 \
  "consume_source_backed_root_once(" \
  "normal/default facade must have one definition, one production caller, and one test caller"
expect_global_fixed_count 6 \
  "NormalRootExecutionConsumerV1::consume_once(" \
  "root consumer caller census drifted (one production facade and five test callers)"
expect_fixed_count 1 \
  "pub(in crate::mir::builder) fn issue(" \
  "$BUILDER_PROJECTION" "root projector issuer visibility drifted"
expect_fixed_count 1 \
  "_permit: NormalRootExecutionProjectionPermitV1" \
  "$BUILDER_PROJECTION" "root projector issuer lost its unforgeable consumer permit"
expect_global_fixed_count 1 \
  "PreparedAdmittedNormalRootExpansionV1::issue(" \
  "root projector issuer gained a caller outside the sole consumer"
expect_fixed_count 1 \
  "const fn issue_for_consumer()" \
  "$ROOT/src/mir/builder/normal_root_execution/mod.rs" \
  "sole consumer projection permit constructor drifted"
expect_global_fixed_count 1 \
  "NormalRootExecutionProjectionPermitV1::issue_for_consumer()" \
  "root projection permit gained a caller outside the sole consumer"
if rg -n '#\[derive\([^]]*(Clone|Copy)|impl (Clone|Copy) for NormalRootExecutionProjectionPermitV1' \
  "$ROOT/src/mir/builder/normal_root_execution/mod.rs"; then
  guard_fail "$TAG" "root projection permit became duplicable"
fi
if rg -n 'pub\([^)]*\)?.*with_admitted_normal_root_expansion_v1' "$ROOT/src/mir" -g '*.rs'; then
  guard_fail "$TAG" "root projector regained a callable bypass outside the sole consumer"
fi
if rg -n 'fn into_source\(|\.into_source\(\)' "$BUILDER_CONSUMER" "$PROGRAM_ROOT"; then
  guard_fail "$TAG" "root-consumer rejection regained a retry/source extraction edge"
fi
if rg -n 'with_lowering_view' \
  "$ROOT/src/mir/builder/main_expansion" "$LIFECYCLE" "$SEMANTIC_ISSUER"; then
  guard_fail "$TAG" "admitted root projection regained a repeatable lowering loan"
fi
expect_fixed_count 2 \
  "fn consume_lowering_view_once<R>(" \
  "$BUILDER_PROJECTION" \
  "owned root projection must keep one public and one private consuming loan"
expect_global_fixed_count 3 \
  ".consume_lowering_view_once(" \
  "consuming lowering-loan caller census drifted"
if rg -n 'VerifiedRawRootExpansionV1::from_program|NormalSourceSurfaceInventoryV1|root_is_app_mode' \
  "$BUILDER_CONSUMER" "$BUILDER_MODEL" "$SEMANTIC_ISSUER"; then
  guard_fail "$TAG" "source-backed root consumer re-entered raw classification or Builder mode state"
fi
expect_fixed_count 2 \
  "VerifiedRawRootExpansionV1::from_program" \
  "$LIFECYCLE" "only the two explicit compatibility projections may retain raw expansion"
expect_fixed_count 2 \
  "expansion.is_app_mode()" \
  "$LIFECYCLE" "raw expansion mode reads must stay limited to compatibility preflight and drift check"
expect_fixed_count 1 \
  "                            preflight_is_app_mode," \
  "$LIFECYCLE" "selected work-plan mode must reuse the pre-effect projection"
expect_fixed_count 1 \
  "[mir/main-expansion/compatibility-preflight]" \
  "$LIFECYCLE" "compatibility preflight marker is missing"
expect_fixed_count 2 \
  "[mir/main-expansion/compatibility-retained]" \
  "$LIFECYCLE" "compatibility retained projection/drift markers are missing"

# Raw source-backed and explicit compatibility extraction are separate affine issuers.
expect_fixed_count 1 \
  "ParserNormalRootExecutionRawVmDiscardIssuerV1::issue_once(" \
  "$RAW_HANDOFF" "source-backed Raw discard/extraction caller drifted"
expect_fixed_count 1 \
  "RawCompatibilitySourceExtractionIssuerV1::issue_once(" \
  "$RAW_HANDOFF" "compatibility Raw extraction caller drifted"
expect_fixed_count 1 \
  "source.prepare_raw_vm_source_route()" \
  "$RAW_HANDOFF" "Raw parser route must be consumed once"
expect_global_fixed_count 1 \
  "ParserNormalRootExecutionRawVmDiscardIssuerV1::issue_once(" \
  "source-backed Raw issuer gained another caller"
expect_global_fixed_count 1 \
  "RawCompatibilitySourceExtractionIssuerV1::issue_once(" \
  "compatibility Raw issuer gained another caller"
for marker in \
  ReadySourceCannotBecomeCompatibility \
  SourceFailureCannotBecomeCompatibility \
  IncompleteCannotBecomeCompatibility \
  IntegrityInvalidCannotBecomeCompatibility; do
  guard_expect_fixed_in_file "$TAG" "$marker" "$ROOT_COMPAT" \
    "compatibility closure is missing typed rejection $marker"
done

# Production parser-bound policy may not invoke the AST fixture classifier.
if rg -n 'ASTNode|NormalSourceSurfaceInventoryV1|VerifiedRawRootExpansionV1|MirBuilder|root_is_app_mode' \
  "$POLICY"; then
  guard_fail "$TAG" "parser-bound policy leaked AST inventory, raw-root, or Builder authority"
fi
if rg -n 'NormalSourceSurfaceInventoryV1|VerifiedRawRootExpansionV1|MirBuilder|root_is_app_mode|fn source_ast\(' \
  "$PLAN_CONSUMER"; then
  guard_fail "$TAG" "opaque parser loan leaked pre-policy AST or downstream authority"
fi
expect_fixed_count 1 \
  "pub(crate) struct AdmittedSourcePlanBoundNormalCallableSourceV1" \
  "$PLAN_CONSUMER" "post-policy affine syntax state is missing"
expect_fixed_count 1 \
  "pub(crate) fn resolve_policy_once<T>(" \
  "$PLAN_CONSUMER" "policy Result must resolve through one affine typestate gate"
expect_global_fixed_count 1 \
  "resolve_policy_once(" \
  "policy typestate gate must have exactly one compiler caller"
if sed -n '/^impl SourcePlanBoundNormalCallableSourceV1 {/,/^impl AdmittedSourcePlanBoundNormalCallableSourceV1 {/p' \
  "$PLAN_CONSUMER" | rg -n 'ASTNode|source_ast_for_bound_terminal|into_ast_after_source_plan_terminal|fn [a-zA-Z0-9_]*ast'; then
  guard_fail "$TAG" "raw pre-policy owner regained an AST capability"
fi
expect_fixed_count 1 \
  "pub(crate) fn source_ast_after_policy(&self)" \
  "$PLAN_CONSUMER" "admitted syntax loan must remain explicitly named"
expect_fixed_count 1 \
  "pub(crate) fn into_ast_after_policy(self)" \
  "$PLAN_CONSUMER" "admitted terminal extraction must remain explicitly named"
expect_global_fixed_count 1 \
  "source.source_ast_after_policy()" \
  "admitted AST loan must stay behind the sealed source-plan owner"
expect_global_fixed_count 1 \
  "source.into_ast_after_policy()" \
  "admitted AST extraction must stay behind the sealed source-plan owner"
if rg -n 'from_parser_bound\(' "$POLICY" "$ROOT/src/mir/compiler/normal_source_plan/product.rs"; then
  guard_fail "$TAG" "untyped parser-bound source-plan owner constructor returned"
fi
expect_global_fixed_count 2 \
  "from_parser_bound_admitted(" \
  "admitted source-plan owner must have one definition and one policy caller"
expect_global_fixed_count 4 \
  "from_parser_bound_rejected(" \
  "rejected source-plan owner definition/caller census drifted"
if rg -n 'PreparedNormalSourcePlanInputV1|NormalSourceSurfaceInventoryV1::collect|NormalSourcePlanClassifierV1::seal\(' \
  "$SOURCE_PLAN_ENTRY" "$PARSER_HANDOFF" "$RAW_HANDOFF" "$BUILDER_CONSUMER"; then
  guard_fail "$TAG" "production reference route re-entered the AST-only source-plan fixture"
fi

# Superseded narrow/generic authorities and silent sibling drops stay retired.
OLD_PATTERN='ParserMainAppEntry|ParserNormalRootSourceDispositionV1|ParserNormalRootPreserv(ed|ation)V1|ParserCallableSourceDispositionV1|NormalParserCallableSourceHandoffV1|from_parser_callable_source|into_source_disposition|discard_root_before_a|root_is_discarded_before_a|normal_root_source[[:space:]]*:|normal_root_execution[[:space:]]*:[[:space:]]*_|canonical_script_source_rows[[:space:]]*:[[:space:]]*_'
if rg -n "$OLD_PATTERN" "$ROOT/src" -g '*.rs'; then
  guard_fail "$TAG" "superseded narrow/generic root authority or silent drop returned"
fi
if rg -n 'fallback|retry|or_else|unwrap_or_else' \
  "$ROOT_ISSUER" "$ROOT_COMPAT" "$PLAN_CONSUMER" "$PRESERVATION" \
  "$POLICY" "$SOURCE_PLAN_ENTRY" "$PARSER_HANDOFF" "$RAW_HANDOFF" \
  "$BUILDER_CONSUMER"; then
  guard_fail "$TAG" "canonical root route gained fallback or retry"
fi
if rg -n 'fn (into_source|retry)\b|\.retry\(' \
  "$RAW_HANDOFF" "$PRODUCT" "$BUILDER_CONSUMER"; then
  guard_fail "$TAG" "rejected Raw/Builder owner regained a retry edge"
fi

# Every transform/postpass error exit reaches a named owner terminal.
expect_fixed_count 3 \
  "discard_at_named_transform_reject_terminal" \
  "$FINAL_SOURCE_MODEL" "prepared final-source reject terminal census drifted"
expect_fixed_count 5 \
  "input.discard_at_named_transform_reject_terminal" \
  "$TRANSFORM" "final transform no longer closes all five pre-root exits"
expect_fixed_count 3 \
  "discard_transform_remainder_at_named_terminal" \
  "$TRANSFORM" "final transform remainder terminal census drifted"
expect_fixed_count 3 \
  "initial.discard_at_named_transform_reject_terminal();" \
  "$MACRO_TRANSFORM" "macro transform no longer closes every typed reject family"
expect_fixed_count 4 \
  "discard_pretransform_normal_callable_at_named_terminal" \
  "$POSTPASS_PROGRAM" "postpass normal-callable terminal census drifted"
expect_fixed_count 3 \
  "discard_at_named_transform_reject_terminal" \
  "$PRESERVATION" "root-preservation reject terminal census drifted"

# Mixed ordinary/static programs keep the finalized ordinary row; static rows
# remain explicit compatibility observations instead of erasing source truth.
expect_fixed_count 1 \
  "pub(super) fn source_backed_compatibility_rows(" \
  "$POSTPASS_ROWS" "mixed-source row retention owner is missing"
guard_expect_fixed_in_file "$TAG" \
  "s0_coordinator_selects_explicit_compatibility_arm" "$POSTPASS" \
  "mixed ordinary/static postpass regression is missing"
guard_expect_fixed_in_file "$TAG" \
  "mixed_compatibility_source_carries_constructor_catalog_without_widening_cohort" \
  "$FINAL_SOURCE_TESTS" "mixed-source final owner regression is missing"

# Focused positive and negative evidence stays attached to the owning boundary.
for test_name in \
  total_root_distinguishes_program_runtime_and_app \
  empty_and_non_main_provider_remain_program_runtime \
  app_relation_retains_main_and_static_children \
  non_static_main_remains_one_app_relation_with_a_policy_fact \
  missing_main_method_is_a_typed_incomplete_terminal \
  app_relation_keeps_top_level_siblings_in_the_same_surface \
  duplicate_main_is_a_typed_integrity_terminal \
  surface_missing_and_integrity_terminals_propagate_without_reclassification; do
  guard_expect_fixed_in_file "$TAG" "$test_name" "$ROOT_TESTS" \
    "missing total-root evidence $test_name"
done
for test_name in \
  exact_app_relation_preserves_main_and_static_children_as_one_aggregate \
  total_root_preservation_rejects_foreign_parser_witness \
  main_helper_is_preserved_as_app_source_not_downgraded_to_terminal \
  preservation_rejects_root_statement_replacement \
  preservation_rejects_root_statement_addition \
  preservation_rejects_root_statement_removal \
  preservation_rejects_root_statement_reorder; do
  guard_expect_fixed_in_file "$TAG" "$test_name" "$PRESERVATION_TESTS" \
    "missing transform-preservation evidence $test_name"
done
for test_name in \
  root_statement_replacement_is_rejected_before_final_source \
  root_statement_addition_is_rejected_before_final_source \
  root_statement_removal_is_rejected_before_final_source \
  root_statement_reorder_is_rejected_before_final_source \
  added_or_changed_callable_rejects_without_compatibility_fallback; do
  guard_expect_fixed_in_file "$TAG" "$test_name" "$FINAL_SOURCE_TESTS" \
    "missing exact-transform rejection evidence $test_name"
done
for test_name in \
  parser_bound_empty_and_executable_sources_are_script \
  parser_bound_static_main_zero_is_main0 \
  parser_bound_main_helpers_and_top_level_callable_form_one_module \
  parser_bound_non_static_main_is_policy_rejected \
  parser_bound_mixed_app_and_executable_source_is_rejected \
  parser_bound_non_main_box_is_unsupported_not_script \
  parser_bound_main_arity_is_checked_by_policy_once \
  parser_bound_main_member_coverage_is_checked_by_policy_once \
  parser_bound_top_level_callable_without_main_is_missing_entry; do
  guard_expect_fixed_in_file "$TAG" "$test_name" "$POLICY_TESTS" \
    "missing parser-bound policy evidence $test_name"
done
for test_name in \
  consumes_program_runtime_once_without_ast_classification \
  consumes_app_once_from_parser_relation \
  same_named_top_level_and_main_child_are_paired_by_parser_identity \
  non_static_main_rejects_with_source_policy_before_projection; do
  guard_expect_fixed_in_file "$TAG" "$test_name" "$BUILDER_TESTS" \
    "missing Builder root-consumer evidence $test_name"
done
guard_expect_fixed_in_file "$TAG" \
  "source_backed_non_static_main_rejects_with_policy_before_builder_effects" \
  "$ROOT/src/mir/builder/normal_default_root_catalog_lifecycle_tests.rs" \
  "missing lifecycle-level pre-effect MainMustBeStatic evidence"
guard_expect_fixed_in_file "$TAG" \
  "source_backed_lifecycle_facade_consumes_program_runtime_and_app_once" \
  "$ROOT/src/mir/builder/normal_default_root_catalog_lifecycle_tests.rs" \
  "missing direct lifecycle-facade App/ProgramRuntime evidence"
expect_fixed_count 5 \
  "rejected.discard();" \
  "$ROOT/src/mir/builder/normal_default_root_catalog_lifecycle_tests.rs" \
  "every lifecycle rejection test must close through the named terminal"
for test_name in \
  one_scoped_loan_retains_the_exact_declaration_rows \
  retained_source_keeps_total_app_relation_and_script_sibling \
  retained_source_does_not_turn_main_arity_into_parser_policy \
  equal_source_text_does_not_merge_parser_authority \
  rejected_retention_keeps_the_atomic_parser_owner_until_named_discard; do
  guard_expect_fixed_in_file "$TAG" "$test_name" "$RETAINED_TESTS" \
    "missing retained one-shot evidence $test_name"
done
for test_name in \
  disabled_macro_keeps_static_source_backed \
  source_backed_default_derive_rejects_root_authority_loss \
  source_backed_test_harness_tail_rejects_root_authority_loss \
  unclassified_macro_mutation_is_not_exact_or_compatibility; do
  guard_expect_fixed_in_file "$TAG" "$test_name" "$MACRO_TRANSFORM_TESTS" \
    "missing transform terminal evidence $test_name"
done
for test_name in \
  source_using_rejects_after_authorized_raw_extraction \
  consuming_handoff_keeps_the_existing_raw_profile_paired \
  canonical_core_profile_rejects_raw_handoff_without_losing_source_owner; do
  guard_expect_fixed_in_file "$TAG" "$test_name" "$FRONTDOOR_TESTS" \
    "missing Raw/front-door evidence $test_name"
done
for test_name in \
  raw_route_closes_parser_rows_without_canonical_co_seal \
  raw_compatibility_route_extracts_once_only_after_typed_absence \
  raw_source_failures_never_become_compatibility_extractions \
  raw_route_is_a_typed_reject_at_the_canonical_policy_boundary \
  canonical_route_is_a_typed_reject_at_the_raw_boundary; do
  guard_expect_fixed_in_file "$TAG" "$test_name" "$ATOMIC_CUTOVER_TESTS" \
    "missing atomic C0 route evidence $test_name"
done

guard_expect_fixed_in_file "$TAG" \
  "Parser SourceSurface" "$README" \
  "parser owner README must document the C0 authority chain"
guard_expect_fixed_in_file "$TAG" \
  "Normal-root pre-effect consumer (C0)" "$BUILDER_README" \
  "Builder owner README must document the C0 pre-effect boundary"
guard_expect_fixed_in_file "$TAG" \
  "NORMAL-ROOT-EXECUTION-ATOMIC-CUTOVER-C0" "$CARD" \
  "active C0 card is missing"
guard_expect_fixed_in_file "$TAG" \
  "normal_root_execution_reference_route_guard.sh" "$INDEX" \
  "check index must list the reusable root-route guard"

for file in "${FILES[@]}"; do
  case "$file" in
    *.rs)
      lines="$(wc -l < "$file" | tr -d '[:space:]')"
      (( lines < 760 )) || guard_fail "$TAG" \
        "source crossed split trigger: ${file#"$ROOT/"} ($lines)"
      ;;
  esac
done

echo "[$TAG] parser surface/root/preservation issuers=1/1/1"
echo "[$TAG] canonical policy and Builder consumer callers=1/1"
echo "[$TAG] source-backed/compatibility Raw issuers=1/1"
echo "[$TAG] old narrow/generic authorities and canonical fallback=0"
echo "[$TAG] focused evidence and touched Rust source-size limits=1"
echo "[$TAG] PASS"
