#!/usr/bin/env python3
"""Reusable canonical normal-module transaction checks."""

from pathlib import Path


def require(text: str, fragment: str, label: str) -> None:
    if fragment not in text:
        raise AssertionError(f"missing {label}: {fragment!r}")


def require_count(text: str, fragment: str, expected: int, label: str) -> None:
    actual = text.count(fragment)
    if actual != expected:
        raise AssertionError(
            f"{label}: expected {expected} occurrences of {fragment!r}, got {actual}"
        )


def check_transaction(root: Path, directory: Path, task_path: Path) -> None:
    task = task_path.read_text()
    transaction_path = directory / "main_transaction.rs"
    physical_path = directory / "physical_thunk.rs"
    source_path = directory / "source_draft.rs"
    callable_main_physical_path = directory / "callable_main_physical.rs"
    callable_main_physical_tests_path = directory / "callable_main_physical_tests.rs"
    tests_path = directory / "main_transaction_tests.rs"
    thunk_plan_path = (
        root
        / "src/mir/compiler/normal_source_plan/main_thunk_plan.rs"
    )
    transaction = transaction_path.read_text()
    physical = physical_path.read_text()
    source = source_path.read_text()
    callable_main_physical = callable_main_physical_path.read_text()
    tests = tests_path.read_text()
    thunk_plan = thunk_plan_path.read_text()
    production = "\n".join(
        path.read_text()
        for path in (
            transaction_path,
            physical_path,
            source_path,
            callable_main_physical_path,
            directory / "result_type.rs",
        )
    )

    for fragment in (
        "NORMAL-MAIN0-TX0-RETENTION-prime-r1",
        "retained semantic evidence + restored Builder",
        "no recoverable source-plan owner",
        "TX0-A EVIDENCE0",
        "TX0-B PHYSICAL0",
        "TX0-C PREPARE0",
        "TX0-D COMMIT0",
        "TX0-E FAILURE-REUSE0",
        "TX0-F G0",
        "docs_only_closeout: forbidden",
        "code_or_artifact_delta_required: 1",
    ):
        require(task, fragment, f"TX0 task contract {fragment}")

    for definition in (
        "struct RetainedNormalMainTransactionEvidenceV1",
        "enum RetainedNormalMainPreparedDraftsV1",
        "struct VerifiedNormalMainSourceDraftV1",
        "struct VerifiedNormalMainPhysicalThunkDraftV1",
        "struct PreparedNormalMainModuleTransactionV1",
        "struct CompletedNormalMainModuleCandidateV1",
        "struct RejectedNormalMainModuleTransactionV1",
        "enum NormalMainModuleTransactionStageV1",
        "enum NormalMainModuleTransactionErrorV1",
    ):
        require_count(production, definition, 1, f"sole transaction owner {definition}")

    for fragment in (
        "fn prepare_normal_main_module_transaction",
        "lower_resolved_trivial_function_draft(",
        "VerifiedNormalMainSourceDraftV1::seal(",
        "VerifiedNormalMainPhysicalThunkDraftV1::prepare(",
        "fn validate_correspondence(",
        "fn verify_candidate(",
        "ModuleLoweringShellDrainInventoryV1::from_symbols",
        "ModuleLoweringShellV1::from_empty_module",
        "commit_preflighted(functions)",
    ):
        require(transaction, fragment, f"TX0 owner chain {fragment}")

    for fragment in (
        "MirInstruction::Call",
        "func: ValueId::INVALID",
        "Callee::Global(",
        "args: Vec::new()",
        "MirInstruction::Return { value: returned }",
        "verify_completed_draft_typed_value_definitions_v1",
        "MirVerifier::new()",
        "VerifiedDirectCallEffectV1::ConservativeBarrier",
    ):
        require(physical, fragment, f"physical thunk contract {fragment}")

    for fragment in (
        "Unit { origin: FunctionUnitOriginV1 }",
        "ExplicitUnit { origin, .. }",
        "ImplicitUnit { origin, .. }",
        "Unit { origin: *origin }",
    ):
        require(thunk_plan, fragment, f"lossless Main Unit evidence {fragment}")

    for forbidden in (
        "ASTNode",
        "NYASH_ENTRY",
        "RawMainEntryTargetV1",
        "LegacyReplaceWholePair",
        "compile_with_source",
        "build_module",
        "process::",
        "SourceEntryResult",
        "ProcessExitProjection",
        "retry",
        "fallback",
    ):
        if forbidden in production:
            raise AssertionError(f"normal Main transaction gained forbidden authority: {forbidden}")

    commit_start = transaction.index("pub(in crate::mir) fn commit(self)")
    commit_end = transaction.index(
        "impl RejectedNormalMainModuleTransactionV1", commit_start
    )
    commit = transaction[commit_start:commit_end]
    for forbidden in (
        "Result<",
        "?",
        "try_add_functions_atomic",
        "verify_",
        "lookup",
        "get_function",
    ):
        if forbidden in commit:
            raise AssertionError(f"TX0 commit gained fallible authority: {forbidden}")

    for fragment in (
        "fn transaction_commits_exact_source_main_and_physical_thunk(",
        "fn same_builder_can_prepare_successive_normal_main_candidates(",
        "fn transaction_admits_exact_unit_spelling_and_annotation_matrix(",
        "fn every_rejection_stage_retains_exact_progress_and_builder_reuse(",
        'module.get_function("main/0")',
        'module.get_function("main")',
        'Callee::Global("main/0".to_owned())',
        "assert_eq!(dst, value)",
        "rejected.has_restoration_receipt()",
    ):
        require(tests, fragment, f"TX0 fixture {fragment}")

    for fragment in (
        "expected_symbol",
        "expected_arity",
        "normal_main_result_mir_type(result)",
        "fn into_draft(self)",
    ):
        require(source, fragment, f"source draft wrapper {fragment}")

    for path in (
        transaction_path,
        physical_path,
        source_path,
        callable_main_physical_path,
        callable_main_physical_tests_path,
        tests_path,
        directory / "result_type.rs",
        thunk_plan_path,
        Path(__file__),
    ):
        if len(path.read_text().splitlines()) >= 800:
            raise AssertionError(
                f"file must remain below 800 lines: {path.relative_to(root)}"
            )

    for fragment in (
        "struct PreparedNormalCallableMainPhysicalV1",
        "enum RejectedNormalCallableMainPhysicalV1",
        "with_main_lowering_plan",
        "lower_resolved_trivial_function_draft_retaining_failure_v1",
        "VerifiedNormalMainPhysicalThunkDraftV1::prepare",
    ):
        require(callable_main_physical, fragment, f"MAIN-PHYSICAL0 owner {fragment}")
    for forbidden in (
        "NormalCanonicalModuleBatchV1",
        "try_add_function",
        "MirModule",
        "compile_with_source",
        "build_module",
        "fallback",
        "retry",
    ):
        if forbidden in callable_main_physical:
            raise AssertionError(f"MAIN-PHYSICAL0 gained forbidden authority: {forbidden}")
    require(
        callable_main_physical_tests_path.read_text(),
        "fn main_and_physical_extend_the_completed_helper_prefix_without_a_batch(",
        "MAIN-PHYSICAL0 success fixture",
    )
    require(
        callable_main_physical_tests_path.read_text(),
        "fn injected_main_stages_retain_helpers_and_restore_builder_for_later_success(",
        "MAIN-PHYSICAL0 failure/reuse fixture",
    )
