#!/usr/bin/env python3
"""Callable-source child checks for the single NORMAL-SOURCE-PLAN0 guard."""

from pathlib import Path
from typing import Callable


Require = Callable[[str, str, str], None]
RequireCount = Callable[[str, str, int, str], None]


def check_callable_source(
    root: Path,
    source_dir: Path,
    require: Require,
    require_count: RequireCount,
) -> tuple[Path, ...]:
    task_path = root / (
        "docs/development/current/main/investigations/"
        "normal-callable-source0-s0-execution-task-2026-07-26.md"
    )
    direct_call_task_path = root / (
        "docs/development/current/main/investigations/"
        "normal-main-direct-call0-s0-execution-task-2026-07-26.md"
    )
    callable_source_path = source_dir / "callable_source.rs"
    callable_source_tests_path = source_dir / "callable_source_tests.rs"
    callable_catalog_source_path = source_dir / "callable_catalog_source.rs"
    direct_call_source_path = source_dir / "main_direct_call_source.rs"
    direct_call_source_tests_path = source_dir / "main_direct_call_source_tests.rs"
    direct_call_plan_path = source_dir / "main_direct_call_plan.rs"
    direct_call_plan_tests_path = source_dir / "main_direct_call_plan_tests.rs"
    capability_path = root / "src/mir/compiler/capability.rs"
    function_role_policy_path = (
        root / "src/mir/compiler/capability/function_role_policy.rs"
    )
    analyzer_policy_path = (
        root / "src/mir/resolved_value_profile/analyzer_policy.rs"
    )
    header_source_path = (
        root / "src/mir/resolved_semantics/callable_header_source_unit.rs"
    )
    header_source_tests_path = (
        root / "src/mir/resolved_semantics/callable_header_source_unit_tests.rs"
    )
    header_view_path = (
        root / "src/mir/resolved_semantics/callable_module_header_view.rs"
    )
    files = (
        task_path,
        direct_call_task_path,
        callable_source_path,
        callable_source_tests_path,
        callable_catalog_source_path,
        direct_call_source_path,
        direct_call_source_tests_path,
        direct_call_plan_path,
        direct_call_plan_tests_path,
        capability_path,
        function_role_policy_path,
        analyzer_policy_path,
        header_source_path,
        header_source_tests_path,
        header_view_path,
        Path(__file__),
    )

    task = task_path.read_text()
    direct_call_task = direct_call_task_path.read_text()
    callable_source = callable_source_path.read_text()
    callable_source_tests = callable_source_tests_path.read_text()
    callable_catalog_source = callable_catalog_source_path.read_text()
    direct_call_source = direct_call_source_path.read_text()
    direct_call_source_tests = direct_call_source_tests_path.read_text()
    direct_call_plan = direct_call_plan_path.read_text()
    direct_call_plan_tests = direct_call_plan_tests_path.read_text()
    capability = capability_path.read_text()
    function_role_policy = function_role_policy_path.read_text()
    header_source = header_source_path.read_text()
    header_source_tests = header_source_tests_path.read_text()
    header_view = header_view_path.read_text()

    for fragment in (
        "NORMAL-CALLABLE-SOURCE0-S0",
        "one owned original Program",
        "Main-box additional methods",
        "AST clone/rewrite                                  = 0",
        "callable owner issuance                            = 0",
        "all modified/new source/check files                < 800 lines",
    ):
        require(task, fragment, f"callable source task {fragment}")

    for definition in (
        "struct VerifiedNormalCallableSourceUnitV1",
        "struct RejectedNormalCallableSourceV1",
        "enum NormalCallableSourceStageV1",
        "enum NormalCallableSourceErrorV1",
    ):
        require_count(
            callable_source,
            definition,
            1,
            f"sole normal callable source definition {definition}",
        )
    for fragment in (
        "VerifiedCallableHeaderSourceUnitV1::validate_exact_sites(",
        "VerifiedCallableHeaderSourceUnitV1::seal_exact_sites(",
        "NormalAdditionalCallableSiteV1::TopLevel",
        "NormalAdditionalCallableSiteV1::MainMethod",
        "MainMethodHelperUnsupported",
        "fn stage(&self)",
        "fn error(&self)",
        "fn discard(self)",
    ):
        require(callable_source, fragment, f"normal callable source law {fragment}")
    for fragment in (
        "fn seal_exact_sites(",
        "fn validate_exact_sites(",
        "declaration_sites.sort_unstable()",
        "DuplicateDeclarationSite",
        "MissingProgramStatement",
    ):
        require(header_source, fragment, f"exact-site header owner {fragment}")
    require(
        header_view,
        "fn from_statement_index(",
        "checked callable declaration-site constructor",
    )

    for test_name in (
        "one_program_owner_exposes_exact_top_level_helper_sites",
        "helper_declaration_reorder_preserves_exact_selected_meaning",
        "main_box_helpers_reject_before_catalog_sealing",
    ):
        require(
            callable_source_tests,
            f"fn {test_name}(",
            f"normal callable source fixture {test_name}",
        )
    for test_name in (
        "exact_sites_keep_one_mixed_program_owner_without_reclassifying_main",
        "exact_sites_reject_empty_duplicate_missing_and_non_function_rows",
    ):
        require(
            header_source_tests,
            f"fn {test_name}(",
            f"exact-site source fixture {test_name}",
        )

    for fragment in (
        "NORMAL-MAIN-DIRECT-CALL0-S0",
        "one retained helper-catalog resolver continuation",
        "second helper index",
        "source/AST clone or rewrite",
        "retry/fallback",
    ):
        require(direct_call_task, fragment, f"Main direct-call task {fragment}")

    for definition in (
        "struct VerifiedNormalCallableCatalogSourceUnitV1",
        "struct RejectedNormalCallableCatalogSourceV1",
        "struct VerifiedNormalMainDirectCallSourceUnitV1",
        "struct RejectedNormalMainDirectCallSourceV1",
        "struct VerifiedNormalMainDirectCallPlanV1",
        "struct RejectedNormalMainDirectCallPlanV1",
    ):
        combined = callable_catalog_source + direct_call_source + direct_call_plan
        require_count(combined, definition, 1, f"sole direct-call owner {definition}")
    for fragment in (
        "PreparedOwnerFreeCallableCatalogV1::prepare(",
        "PreparedCallableCatalogSealV1::prepare(",
        "catalog_plan.commit(owner_free)",
    ):
        require(
            callable_catalog_source,
            fragment,
            f"borrow-only helper catalog preparation {fragment}",
        )
    for fragment in (
        "resolve_forest_with_callable_index(",
        "CatalogSealedResolverContinuationV1::restore(",
        "from_exact_parts_with_callable_index(",
        "source.catalog().index()",
    ):
        require(
            direct_call_source,
            fragment,
            f"continuation-owned Main resolution {fragment}",
        )
    for fragment in (
        "verify_normal_main0_function_with_finite_direct_calls_v1(",
        "profile.direct_calls()",
        "fn discard(self)",
    ):
        require(direct_call_plan, fragment, f"Main direct-call plan law {fragment}")
    for fragment in (
        "NormalMainDirectCall0",
        "allows_zero_parameter_direct_call",
        "zero_parameter_direct_call_not_activated",
    ):
        require(
            function_role_policy + capability,
            fragment,
            f"Main-only zero-parameter caller policy {fragment}",
        )
    for test_name in (
        "call_free_main_and_helper_share_one_retained_catalog",
        "main_direct_call_uses_helper_owner_from_the_same_compilation_brand",
        "unresolved_main_call_rejects_without_call_free_retry",
    ):
        require(
            direct_call_source_tests,
            f"fn {test_name}(",
            f"Main direct-call source fixture {test_name}",
        )
    for test_name in (
        "finite_main_plan_seals_exact_helper_call",
        "multiple_nested_main_calls_preserve_child_before_parent_rows",
        "helper_declaration_order_does_not_change_main_call_meaning",
        "call_free_main_does_not_enter_the_direct_call_plan",
    ):
        require(
            direct_call_plan_tests,
            f"fn {test_name}(",
            f"Main direct-call plan fixture {test_name}",
        )

    for forbidden in (
        "FunctionSemanticResolverSessionV1::new",
        "from_exact_parts_without_callable",
        "ASTNode::FunctionCall",
        "MirBuilder",
        "MirInstruction",
        "MirModule",
        "ValueId",
        "RawVm",
        "retry(",
        "fallback",
        ".clone()",
    ):
        if forbidden in direct_call_source + direct_call_plan:
            raise AssertionError(
                f"Main direct-call owner gained duplicate/lowering authority: {forbidden}"
            )

    for forbidden in (
        "MirBuilder",
        "MirInstruction",
        "MirType",
        "ValueId",
        "RawRoot",
        "NYASH_ENTRY",
        "compile",
        "execute",
        "retry",
        "fallback",
        ".clone()",
    ):
        if forbidden in callable_source:
            raise AssertionError(
                f"normal callable source gained lowering/retry authority: {forbidden}"
            )

    for path in files:
        if len(path.read_text().splitlines()) >= 800:
            raise AssertionError(
                f"file must remain below 800 lines: {path.relative_to(root)}"
            )
    return files
