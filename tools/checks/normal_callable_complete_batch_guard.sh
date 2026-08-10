#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="normal-callable-complete-batch"
source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

PARSER_LOAN="$ROOT_DIR/src/parser/normal_callable_program_source/semantic_syntax_loan.rs"
PARSER_ANCHOR="$ROOT_DIR/src/parser/callable_source_anchor.rs"
BATCH_ISSUER="$ROOT_DIR/src/mir/callable_semantic_batch/issuer.rs"
DEMAND_ISSUER="$ROOT_DIR/src/mir/callable_parameter_demand/issuer.rs"
PACKAGE_ISSUER="$ROOT_DIR/src/mir/normal_callable_semantic_package/issuer.rs"
PACKAGE_MODEL="$ROOT_DIR/src/mir/normal_callable_semantic_package/model.rs"
PACKAGE_INSTALL="$ROOT_DIR/src/mir/normal_callable_semantic_package/install.rs"
SELECTED_MAPPING="$ROOT_DIR/src/mir/normal_callable_semantic_package/selected_mapping.rs"
SOURCE_CATALOG="$ROOT_DIR/src/mir/builder/callable_declaration_catalog/source_backed.rs"
INGRESS="$ROOT_DIR/src/mir/compiler/dynamic_full_body_recipe/coseal/semantic_program/ingress.rs"
BATCH_TESTS="$ROOT_DIR/src/mir/callable_semantic_batch/tests.rs"
PACKAGE_TESTS="$ROOT_DIR/src/mir/normal_callable_semantic_package/tests.rs"
DYNAMIC_TARGET="$ROOT_DIR/src/mir/source_call_target/dynamic_member.rs"
DYNAMIC_CALLS="$ROOT_DIR/src/mir/compiler/dynamic_full_body_recipe/coseal/calls.rs"
DYNAMIC_COSEAL="$ROOT_DIR/src/mir/compiler/dynamic_full_body_recipe/coseal/mod.rs"
DYNAMIC_SEMANTIC="$ROOT_DIR/src/mir/compiler/dynamic_full_body_recipe/coseal/semantic_program/mod.rs"
PRODUCTION_LIFECYCLE="$ROOT_DIR/src/mir/builder/normal_default_root_catalog_lifecycle.rs"
PRODUCTION_LOWERING="$ROOT_DIR/src/mir/builder/program_root_lowering.rs"
PACKAGE_PORT_ADAPTER="$ROOT_DIR/src/mir/builder/normal_callable_semantic_loan_port.rs"
RAW_SOURCE_TRANSPORT="$ROOT_DIR/src/mir/builder/raw_invocation_source_transport.rs"

guard_require_command "$TAG" rg
guard_require_command "$TAG" wc
guard_require_files "$TAG" \
  "$PARSER_LOAN" "$PARSER_ANCHOR" "$BATCH_ISSUER" "$DEMAND_ISSUER" \
  "$PACKAGE_ISSUER" "$PACKAGE_MODEL" "$PACKAGE_INSTALL" "$SELECTED_MAPPING" \
  "$SOURCE_CATALOG" "$INGRESS" "$BATCH_TESTS" "$PACKAGE_TESTS" \
  "$DYNAMIC_TARGET" "$DYNAMIC_CALLS" "$DYNAMIC_COSEAL" "$DYNAMIC_SEMANTIC" \
  "$PRODUCTION_LIFECYCLE" "$PRODUCTION_LOWERING" "$PACKAGE_PORT_ADAPTER" \
  "$RAW_SOURCE_TRANSPORT"

reject_fixed_in_file() {
  local pattern="$1"
  local file="$2"
  local message="$3"
  if rg -F -q -- "$pattern" "$file"; then
    guard_fail "$TAG" "$message"
  fi
}

guard_expect_fixed_in_file "$TAG" \
  "FinalCallableSemanticSyntaxLoanV1" "$PARSER_LOAN" \
  "final source must own the complete callable syntax loan"
guard_expect_fixed_in_file "$TAG" \
  "CallableDeclarationIdentityV1(Arc<()>)" "$PARSER_ANCHOR" \
  "parser anchor must own the comparison-only declaration identity"
guard_expect_fixed_in_file "$TAG" \
  "Arc::ptr_eq" "$PARSER_ANCHOR" \
  "declaration identity must compare opaque anchor allocation"
reject_fixed_in_file \
  "Serialize for CallableDeclarationIdentityV1" "$PARSER_ANCHOR" \
  "opaque declaration identity must not become portable wire"
reject_fixed_in_file \
  "Arc::as_ptr" "$PARSER_ANCHOR" \
  "opaque declaration identity must not expose a raw pointer"
guard_expect_fixed_in_file "$TAG" \
  "parameters: Option<Box" "$PARSER_LOAN" \
  "parameter source must remain an exact partial projection"
guard_expect_fixed_in_file "$TAG" \
  ".with_callable_semantic_syntax" "$BATCH_ISSUER" \
  "semantic batch must traverse complete final callable syntax"
reject_fixed_in_file \
  "with_callable_parameter_syntax" "$BATCH_ISSUER" \
  "parameter catalog must not define semantic batch membership"
guard_expect_fixed_in_file "$TAG" \
  "let Some(source_parameters) = row.parameters() else" "$DEMAND_ISSUER" \
  "parameter demand must skip unprojected callable rows without inference"
reject_fixed_in_file \
  "parameter_demands.len() != batch.declarations().len()" "$PACKAGE_ISSUER" \
  "package must not equate partial demand count with complete batch count"
guard_expect_fixed_in_file "$TAG" \
  "MissingDynamicParameterDemand" "$PACKAGE_ISSUER" \
  "selected Dynamic candidate must fail closed without parameter authority"
guard_expect_fixed_in_file "$TAG" \
  "same_declaration_identity" "$SELECTED_MAPPING" \
  "selected mapping must use exact parser declaration identity"
if rg -q 'statement|ordinal|arity|FunctionOwnerId|ptr::eq|\.zip\(' "$SELECTED_MAPPING"; then
  guard_fail "$TAG" "selected mapping contains forbidden identity-repair vocabulary"
fi
reject_fixed_in_file \
  "seal_root(" "$SOURCE_CATALOG" \
  "source-backed catalog must not enter through the AST-only root seal"
reject_fixed_in_file \
  "seal_program(" "$SOURCE_CATALOG" \
  "source-backed catalog must not enter through the AST-only program seal"
guard_expect_fixed_in_file "$TAG" \
  "issue_dynamic_carrier_ingress_lifecycle_program_v1" "$PACKAGE_ISSUER" \
  "selected Dynamic package must co-seal parameter #1 ingress"
guard_expect_fixed_in_file "$TAG" \
  "BorrowedIngressNoEndV1" "$INGRESS" \
  "Dynamic ingress must retain the borrowed-no-end lifecycle marker"
guard_expect_fixed_in_file "$TAG" \
  "NormalCallableSemanticPackagePortV1" "$PACKAGE_INSTALL" \
  "installed package must expose one exactly-once selected lowering port"
guard_expect_fixed_in_file "$TAG" \
  "IncompleteSelectedCoverage" "$PACKAGE_INSTALL" \
  "package port must reject incomplete selected-key consumption"
reject_fixed_in_file \
  "pub(crate) fn batch_slot" "$SELECTED_MAPPING" \
  "selected mapping must not expose raw batch slots outside the package owner"
reject_fixed_in_file \
  "fn into_parts" "$PACKAGE_MODEL" \
  "whole semantic package must not expose a consuming parts API"
guard_expect_fixed_in_file "$TAG" \
  "top_level_and_box_methods_share_one_complete_batch" "$BATCH_TESTS" \
  "mixed top-level complete-batch fixture is missing"
guard_expect_fixed_in_file "$TAG" \
  "top_level_and_dynamic_candidate_share_one_complete_package_batch" "$PACKAGE_TESTS" \
  "mixed top-level plus Dynamic package fixture is missing"
guard_expect_fixed_in_file "$TAG" \
  "selected_gate_dynamic_candidate_rejects_without_parameter_authority" "$PACKAGE_TESTS" \
  "missing Dynamic parameter-authority negative is absent"
guard_expect_fixed_in_file "$TAG" \
  "unselected_main_candidate_does_not_duplicate_one_selected_dynamic_candidate" "$PACKAGE_TESTS" \
  "selected-map Dynamic filtering negative is absent"
guard_expect_fixed_in_file "$TAG" \
  "consuming_install_and_port_enforce_exact_selected_coverage" "$PACKAGE_TESTS" \
  "consuming install and exactly-once port test is absent"
guard_expect_fixed_in_file "$TAG" \
  "issue_source_bound_dynamic_member_calls_v1" "$DYNAMIC_TARGET" \
  "route-neutral Dynamic source relation issuer is missing"
guard_expect_fixed_in_file "$TAG" \
  "target: VerifiedSourceBoundDynamicMemberCallV1" "$DYNAMIC_CALLS" \
  "CallSlot relation must retain the exact source-bound target"
guard_expect_fixed_in_file "$TAG" \
  "TargetDispatchMismatch" "$DYNAMIC_CALLS" \
  "CallSlot handoff must verify selector and arity"
guard_expect_fixed_in_file "$TAG" \
  "TargetCountMismatch" "$DYNAMIC_CALLS" \
  "CallSlot handoff must reject extra target rows"
guard_expect_fixed_in_file "$TAG" \
  "targets: Box<[VerifiedSourceBoundDynamicMemberCallV1]>" "$DYNAMIC_COSEAL" \
  "CallSlot handoff must consume owned source-bound target rows"
guard_expect_fixed_in_file "$TAG" \
  "extra_source_bound_target_rows_reject_before_relation_issuance" "$ROOT_DIR/src/mir/compiler/dynamic_full_body_recipe/coseal/tests.rs" \
  "extra source-bound target negative is missing"
guard_expect_fixed_in_file "$TAG" \
  "dispatch_selector_and_arity_are_part_of_the_source_target_contract" "$DYNAMIC_CALLS" \
  "selector/arity negative is missing"
reject_fixed_in_file \
  "CanonicalSameModuleCallableKeyV1" "$DYNAMIC_CALLS" \
  "Recipe call relations must not retain a callable-catalog key"
reject_fixed_in_file \
  "VerifiedDynamicInvocationEnvelopeCatalogV1" "$DYNAMIC_COSEAL" \
  "Recipe co-seal must not retain the legacy Dynamic envelope catalog"
reject_fixed_in_file \
  "<'env, 'decl>" "$DYNAMIC_SEMANTIC" \
  "semantic program must not retain callable-catalog lifetimes"
guard_expect_fixed_in_file "$TAG" \
  "NormalCallableDynamicProjectionV1::Selected" "$PACKAGE_ISSUER" \
  "whole package must own the selected completed Dynamic lifecycle program"
guard_expect_fixed_in_file "$TAG" \
  "zero_dynamic_candidates_are_valid_unselected_without_default_or_name_selection" "$PACKAGE_TESTS" \
  "fully observed zero-Dynamic package must remain typed valid-unselected"
guard_expect_fixed_in_file "$TAG" \
  "issue_normal_callable_semantic_package_v1(" "$PRODUCTION_LIFECYCLE" \
  "normal/default production lifecycle must issue the source-backed package"
guard_expect_fixed_in_file "$TAG" \
  "NormalCallableSemanticPackageMode::Installed" "$PRODUCTION_LOWERING" \
  "selected source-backed lowering must use the installed package"
guard_expect_fixed_in_file "$TAG" \
  "NormalCallableSemanticPackagePortAdapterV1" "$PACKAGE_PORT_ADAPTER" \
  "Builder must retain only the thin package-port adapter"
guard_expect_fixed_in_file "$TAG" \
  "source_backed_selected_callable_uses_the_installed_package_port" "$PRODUCTION_LIFECYCLE" \
  "source-backed production package positive is missing"
guard_expect_fixed_in_file "$TAG" \
  "source_backed_package_failure_is_terminal_before_builder_effects" "$PRODUCTION_LIFECYCLE" \
  "source-backed terminal failure negative is missing"

for old_edge in \
  "VerifiedNormalCallableSemanticSourceV1::seal" \
  "extend_complete_dynamic_sources" \
  "NormalCallableSemanticSourceMode::Complete" \
  "NormalCallableSemanticLoanPortV1::new" \
  "callable_semantic_root"; do
  if rg -F -q -- "$old_edge" \
    "$PRODUCTION_LIFECYCLE" "$PRODUCTION_LOWERING" \
    "$PACKAGE_PORT_ADAPTER" "$RAW_SOURCE_TRANSPORT"; then
    guard_fail "$TAG" "old production semantic edge remains: $old_edge"
  fi
done

for file in \
  "$PARSER_LOAN" "$PARSER_ANCHOR" "$BATCH_ISSUER" "$DEMAND_ISSUER" \
  "$PACKAGE_ISSUER" "$PACKAGE_MODEL" "$PACKAGE_INSTALL" "$SELECTED_MAPPING" \
  "$SOURCE_CATALOG" "$INGRESS" "$BATCH_TESTS" "$PACKAGE_TESTS" \
  "$DYNAMIC_TARGET" "$DYNAMIC_CALLS" "$DYNAMIC_COSEAL" "$DYNAMIC_SEMANTIC" \
  "$PRODUCTION_LIFECYCLE" "$PRODUCTION_LOWERING" "$PACKAGE_PORT_ADAPTER" \
  "$RAW_SOURCE_TRANSPORT"; do
  lines="$(wc -l < "$file" | tr -d '[:space:]')"
  if (( lines >= 800 )); then
    guard_fail "$TAG" "source file reached hard 800-line boundary: ${file#"$ROOT_DIR/"} has $lines"
  fi
done

echo "[$TAG] ok"
