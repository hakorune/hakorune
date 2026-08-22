#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="normal-callable-complete-batch"
source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

PARSER_LOAN="$ROOT_DIR/src/parser/normal_callable_program_source/semantic_syntax_loan.rs"
PARSER_ANCHOR="$ROOT_DIR/src/parser/callable_source_anchor.rs"
BATCH_ISSUER="$ROOT_DIR/src/mir/callable_semantic_batch/issuer.rs"
BATCH_RESOLVER="$ROOT_DIR/src/mir/resolved_semantics/owner_resolver.rs"
CONTRACT_DIR="$ROOT_DIR/src/mir/callable_parameter_contract"
CONTRACT_ISSUER="$CONTRACT_DIR/issuer.rs"
CONTRACT_MODEL="$CONTRACT_DIR/model.rs"
CONTRACT_TESTS="$CONTRACT_DIR/tests.rs"
PACKAGE_ISSUER="$ROOT_DIR/src/mir/normal_callable_semantic_package/issuer.rs"
CONSTRUCTOR_ISSUER="$ROOT_DIR/src/mir/normal_callable_semantic_package/instance_constructor_semantic.rs"
CONSTRUCTOR_TESTS="$ROOT_DIR/src/mir/normal_callable_semantic_package/resolver_deferred_tests.rs"
PACKAGE_MODEL="$ROOT_DIR/src/mir/normal_callable_semantic_package/model.rs"
PACKAGE_INSTALL="$ROOT_DIR/src/mir/normal_callable_semantic_package/install.rs"
PACKAGE_COMPLETION_SEED="$ROOT_DIR/src/mir/normal_callable_semantic_package/completion_seed.rs"
PACKAGE_PHYSICAL_HEADER="$ROOT_DIR/src/mir/normal_callable_semantic_package/physical_header.rs"
PACKAGE_PHYSICAL_HEADER_TESTS="$ROOT_DIR/src/mir/normal_callable_semantic_package/physical_header_tests.rs"
SELECTED_MAPPING="$ROOT_DIR/src/mir/normal_callable_semantic_package/selected_mapping.rs"
SOURCE_CATALOG="$ROOT_DIR/src/mir/builder/callable_declaration_catalog/source_backed.rs"
INVOCATION_CLEANUP="$ROOT_DIR/src/mir/compiler/dynamic_full_body_recipe/coseal/semantic_program/invocation_cleanup.rs"
BATCH_TESTS="$ROOT_DIR/src/mir/callable_semantic_batch/tests.rs"
PARSER_TESTS="$ROOT_DIR/src/parser/normal_callable_program_source/tests.rs"
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
  "$PARSER_LOAN" "$PARSER_ANCHOR" "$BATCH_ISSUER" "$BATCH_RESOLVER" \
  "$CONTRACT_ISSUER" \
  "$CONTRACT_MODEL" "$CONTRACT_TESTS" \
  "$PACKAGE_ISSUER" "$CONSTRUCTOR_ISSUER" "$CONSTRUCTOR_TESTS" \
  "$PACKAGE_MODEL" "$PACKAGE_INSTALL" "$PACKAGE_COMPLETION_SEED" \
  "$PACKAGE_PHYSICAL_HEADER" "$PACKAGE_PHYSICAL_HEADER_TESTS" "$SELECTED_MAPPING" \
  "$SOURCE_CATALOG" "$INVOCATION_CLEANUP" "$BATCH_TESTS" "$PACKAGE_TESTS" \
  "$PARSER_TESTS" \
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
  "declared_type_name: Option<&'source str>" "$PARSER_LOAN" \
  "final syntax loan must preserve declared parameter spelling"
guard_expect_fixed_in_file "$TAG" \
  "declared_type_name: Option<&'batch str>" "$ROOT_DIR/src/mir/callable_semantic_batch/model.rs" \
  "resolved batch loan must preserve declared parameter spelling"
guard_expect_fixed_in_file "$TAG" \
  ".with_callable_semantic_syntax" "$BATCH_ISSUER" \
  "semantic batch must traverse complete final callable syntax"
guard_expect_fixed_in_file "$TAG" \
  "struct SourceBoundSelectedCallableResolverRejectV1" "$BATCH_RESOLVER" \
  "selected callable hard rejects must retain their parser source identity"
guard_expect_fixed_in_file "$TAG" \
  "SourceBoundSelectedCallableResolverRejectV1" "$BATCH_ISSUER" \
  "callable batch must expose only source-bound resolver hard rejects"
guard_expect_fixed_in_file "$TAG" \
  "SourceBoundSelectedCallableResolverRejectV1" "$CONSTRUCTOR_ISSUER" \
  "constructor batch must expose only source-bound resolver hard rejects"
reject_fixed_in_file \
  "Resolver(ResolveOwnerForestErrorV1)" "$BATCH_ISSUER" \
  "callable batch still exposes an unbound resolver hard reject"
reject_fixed_in_file \
  "Resolver(ResolveOwnerForestErrorV1)" "$CONSTRUCTOR_ISSUER" \
  "constructor batch still exposes an unbound resolver hard reject"
guard_expect_fixed_in_file "$TAG" \
  "construction_reject_keeps_the_exact_callable_identity" "$BATCH_TESTS" \
  "callable construction hard-reject identity negative is missing"
guard_expect_fixed_in_file "$TAG" \
  "program_contained_if_resolves_with_the_exact_callable_identity" "$BATCH_TESTS" \
  "Program-contained If containment regression evidence is missing"
guard_expect_fixed_in_file "$TAG" \
  "constructor_construction_reject_keeps_the_exact_parser_source_id" "$CONSTRUCTOR_TESTS" \
  "constructor hard-reject identity negative is missing"
reject_fixed_in_file \
  "with_callable_parameter_syntax" "$BATCH_ISSUER" \
  "parameter catalog must not define semantic batch membership"
reject_fixed_in_file \
  "missing_parameter_contract" "$PACKAGE_COMPLETION_SEED" \
  "physical-header availability must not use a package-wide missing-contract bit"
reject_fixed_in_file \
  "missing_result_annotation" "$PACKAGE_PHYSICAL_HEADER" \
  "physical-header availability must not use a package-wide missing-result bit"
reject_fixed_in_file \
  "Option<super::physical_header::VerifiedCallablePhysicalHeaderCohortV1>" "$PACKAGE_MODEL" \
  "the package must always own one sparse physical-header cohort"
guard_expect_fixed_in_file "$TAG" \
  "mixed_package_lends_only_the_eligible_physical_header_row" "$PACKAGE_PHYSICAL_HEADER_TESTS" \
  "mixed packages must prove that missing siblings cannot erase an eligible header row"
guard_expect_fixed_in_file "$TAG" \
  "let Some(source_parameters) = row.parameters() else" "$CONTRACT_ISSUER" \
  "parameter contract must skip unprojected callable rows without inference"
guard_expect_fixed_in_file "$TAG" \
  "UnsupportedDeclaredType" "$CONTRACT_ISSUER" \
  "unsupported explicit parameter types must reject without opaque fallback"
guard_expect_fixed_in_file "$TAG" \
  "OpaqueHandle" "$CONTRACT_MODEL" \
  "contract model must retain the opaque ordinary case"
guard_expect_fixed_in_file "$TAG" \
  "ExactTrivial" "$CONTRACT_MODEL" \
  "contract model must retain the explicit exact-trivial case"
reject_fixed_in_file \
  "parameter_contracts.len() != batch.declarations().len()" "$PACKAGE_ISSUER" \
  "package must not equate partial contract count with complete batch count"
guard_expect_fixed_in_file "$TAG" \
  "MissingDynamicParameterContract" "$PACKAGE_ISSUER" \
  "selected Dynamic candidate must fail closed without parameter contract authority"
guard_expect_fixed_in_file "$TAG" \
  "issue_callable_parameter_contract_v1" "$PACKAGE_ISSUER" \
  "package must use the sole parameter contract issuer"
reject_fixed_in_file \
  "callable_parameter_demand" "$PACKAGE_ISSUER" \
  "old parameter demand owner must not remain a package dependency"
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
  "issue_dynamic_invocation_cleanup_projection_i0" "$PACKAGE_ISSUER" \
  "selected Dynamic package must co-seal invocation cleanup"
guard_expect_fixed_in_file "$TAG" \
  "ExactI64TrivialNoEnd" "$INVOCATION_CLEANUP" \
  "mixed Recipe must retain the exact trivial-I64 induction lifecycle marker"
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
  "typed_parameter_spelling_survives_resolved_batch_loan" "$BATCH_TESTS" \
  "resolved parameter spelling transport fixture is missing"
guard_expect_fixed_in_file "$TAG" \
  "top_level_callable_does_not_fabricate_parameter_source" "$PARSER_TESTS" \
  "top-level parameter projection negative is missing"
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
  "package_scoped_loan_retains_exact_parameter_contract" "$PACKAGE_TESTS" \
  "package scoped exact parameter-contract loan fixture is absent"
guard_expect_fixed_in_file "$TAG" \
  "projects_exact_i64_and_opaque_parameters_from_one_batch" "$CONTRACT_TESTS" \
  "mixed exact/opaque parameter contract fixture is absent"
guard_expect_fixed_in_file "$TAG" \
  "unsupported_explicit_type_rejects_without_opaque_fallback" "$CONTRACT_TESTS" \
  "unsupported explicit parameter negative is absent"
guard_expect_fixed_in_file "$TAG" \
  "parser_scan_loop_box_preserves_exact_i64_parameter_contracts" "$CONTRACT_TESTS" \
  "bounded Dynamic source contract fixture is absent"
guard_expect_fixed_in_file "$TAG" \
  "issuer_keeps_resolver_and_forest_outside_the_contract_owner" "$CONTRACT_TESTS" \
  "parameter contract owner boundary test is absent"
guard_expect_fixed_in_file "$TAG" \
  "issue_source_bound_dynamic_member_calls_v1" "$DYNAMIC_TARGET" \
  "route-neutral Dynamic source relation issuer is missing"

python3 - "$DYNAMIC_TARGET" "$ROOT_DIR/src/mir" <<'PY'
import pathlib
import re
import sys

target = pathlib.Path(sys.argv[1])
mir_root = pathlib.Path(sys.argv[2])
text = target.read_text()
signature = re.search(
    r"pub\(crate\) fn issue_source_bound_dynamic_member_calls_v1\(.*?\n\}",
    text,
    re.S,
)
if signature is None or "dynamic: &VerifiedSourceBackedDynamicCallableV1" not in signature.group(0):
    raise SystemExit("Dynamic relation issuer must borrow the verified source product")

body_start = text.index("pub(crate) fn issue_source_bound_dynamic_member_calls_v1")
brace = text.index("{", body_start)
depth = 0
body_end = None
for index in range(brace, len(text)):
    if text[index] == "{":
        depth += 1
    elif text[index] == "}":
        depth -= 1
        if depth == 0:
            body_end = index + 1
            break
if body_end is None:
    raise SystemExit("Dynamic relation issuer body is not parseable")
body = text[body_start:body_end]
if "issue_source_backed_dynamic_callable_v1" in body:
    raise SystemExit("Dynamic relation issuer must not reissue source-backed Facts")
if "dynamic.owner() != owner" not in body:
    raise SystemExit("Dynamic relation issuer must reject a foreign source owner first")

needle = "issue_source_bound_dynamic_member_calls_v1("
occurrences = sum(path.read_text().count(needle) for path in mir_root.rglob("*.rs"))
if occurrences != 4:
    raise SystemExit(f"Dynamic relation issuer definition/caller census drift: {occurrences}")
print("dynamic_source_fact_borrow=1 relation_reissue=0 caller_census=4")
PY

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

if [[ -e "$ROOT_DIR/src/mir/callable_parameter_demand/mod.rs" ]]; then
  guard_fail "$TAG" "old callable_parameter_demand owner remains"
fi

for file in \
  "$PARSER_LOAN" "$PARSER_ANCHOR" "$BATCH_ISSUER" "$CONTRACT_ISSUER" \
  "$CONTRACT_MODEL" "$CONTRACT_TESTS" \
  "$PACKAGE_ISSUER" "$PACKAGE_MODEL" "$PACKAGE_INSTALL" "$SELECTED_MAPPING" \
  "$SOURCE_CATALOG" "$INVOCATION_CLEANUP" "$BATCH_TESTS" "$PACKAGE_TESTS" \
  "$PARSER_TESTS" \
  "$DYNAMIC_TARGET" "$DYNAMIC_CALLS" "$DYNAMIC_COSEAL" "$DYNAMIC_SEMANTIC" \
  "$PRODUCTION_LIFECYCLE" "$PRODUCTION_LOWERING" "$PACKAGE_PORT_ADAPTER" \
  "$RAW_SOURCE_TRANSPORT"; do
  lines="$(wc -l < "$file" | tr -d '[:space:]')"
  if (( lines >= 800 )); then
    guard_fail "$TAG" "source file reached hard 800-line boundary: ${file#"$ROOT_DIR/"} has $lines"
  fi
done

echo "[$TAG] ok"
