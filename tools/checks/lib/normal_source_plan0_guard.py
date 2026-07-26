#!/usr/bin/env python3
"""NORMAL-SOURCE-PLAN0 reusable source-family authority guard."""

from pathlib import Path


ROOT = Path(__file__).resolve().parents[3]
SOURCE_DIR = ROOT / "src/mir/compiler/normal_source_plan"
COMPILER_MOD = ROOT / "src/mir/compiler/mod.rs"
TASK = ROOT / (
    "docs/development/current/main/investigations/"
    "normal-source-plan0-s0-execution-task-2026-07-26.md"
)
INPUT_TASK = ROOT / (
    "docs/development/current/main/investigations/"
    "normal-source-plan0-input0-s0-execution-task-2026-07-26.md"
)
MIR_ROOT = ROOT / "src/mir/mod.rs"
FRONTDOOR = ROOT / "src/runner/reference/normal_file_vm_frontdoor.rs"
FRONTDOOR_INPUT = (
    ROOT
    / "src/runner/reference/normal_file_vm_frontdoor/source_plan_input.rs"
)
FRONTDOOR_INPUT_TESTS = (
    ROOT
    / "src/runner/reference/normal_file_vm_frontdoor/source_plan_input_tests.rs"
)
PRODUCTION_FILES = tuple(
    SOURCE_DIR / name
    for name in ("mod.rs", "product.rs", "inventory.rs", "classifier.rs", "rejection.rs")
)
ALL_FILES = (*PRODUCTION_FILES, SOURCE_DIR / "tests.rs")


def require(text: str, fragment: str, label: str) -> None:
    if fragment not in text:
        raise AssertionError(f"missing {label}: {fragment!r}")


def require_count(text: str, fragment: str, expected: int, label: str) -> None:
    actual = text.count(fragment)
    if actual != expected:
        raise AssertionError(
            f"{label}: expected {expected} occurrences of {fragment!r}, got {actual}"
        )


def main() -> int:
    task = TASK.read_text()
    input_task = INPUT_TASK.read_text()
    production = "\n".join(path.read_text() for path in PRODUCTION_FILES)
    classifier = (SOURCE_DIR / "classifier.rs").read_text()
    tests = (SOURCE_DIR / "tests.rs").read_text()
    compiler_mod = COMPILER_MOD.read_text()
    mir_root = MIR_ROOT.read_text()
    frontdoor = FRONTDOOR.read_text()
    frontdoor_input = FRONTDOOR_INPUT.read_text()
    frontdoor_input_tests = FRONTDOOR_INPUT_TESTS.read_text()

    for fragment in (
        "NORMAL-SOURCE-PLAN0-S0",
        "profile-admission-free",
        "Builder-free, MIR-free, backend-free, runner-free",
        "production consumer                        = 0",
        "all touched source/check files             < 800 lines",
    ):
        require(task, fragment, f"S0 task contract {fragment}")

    for fragment in (
        "NORMAL-SOURCE-PLAN0-INPUT0-S0",
        "one disconnected consuming NormalFile-to-source-plan request",
        "Profile admission remains the later",
        "existing prepare_raw_vm_handoff caller count = unchanged",
        "new source-plan production caller            = 0",
        "Direct `pub use` additions to the MIR root vocabulary",
    ):
        require(input_task, fragment, f"INPUT0 task contract {fragment}")

    definitions = (
        "struct PreparedNormalSourcePlanInputV1",
        "struct NormalSourceSurfaceInventoryV1",
        "struct NormalSourcePlanClassifierV1",
        "enum SealedNormalSourcePlanV1",
        "enum SealedNormalScalarRootV1",
        "struct SealedNormalCallableModuleSourceV1",
        "struct RejectedNormalSourcePlanV1",
    )
    for definition in definitions:
        require_count(production, definition, 1, f"sole definition {definition}")
    require_count(
        classifier,
        "pub(crate) fn seal(",
        1,
        "sole source-plan classifier terminal",
    )
    require_count(
        compiler_mod,
        "pub(crate) mod normal_source_plan;",
        1,
        "compiler module declaration",
    )
    require_count(
        mir_root,
        "pub(crate) use compiler::normal_source_plan;",
        1,
        "owner-module facade",
    )

    for fragment in (
        "ScalarRoot(SealedNormalScalarRootV1)",
        "CallableModule(SealedNormalCallableModuleSourceV1)",
        "Script(SealedNormalScriptSourceV1)",
        "Main0(SealedNormalMainSourceV1)",
        "NormalSourcePlanStageV1::RootSurface",
        "NormalSourcePlanStageV1::SourceEntry",
        "NormalSourcePlanStageV1::FamilyClosure",
        "method_entries.sort_by",
    ):
        require(production, fragment, f"source-family law {fragment}")

    for test_name in (
        "empty_and_scalar_programs_are_scripts",
        "main_zero_only_is_a_scalar_main_root",
        "top_level_or_main_box_helpers_make_callable_modules",
        "function_only_program_has_no_source_entry",
        "script_mixed_with_main_or_function_is_rejected_in_either_order",
        "duplicate_main_is_rejected_in_either_order",
        "main_must_be_static_and_define_static_main_zero",
        "unsupported_declaration_is_rejected_before_family_selection",
        "non_program_root_is_rejected_at_root_surface",
    ):
        require(tests, f"fn {test_name}(", f"fixture {test_name}")

    input_definitions = (
        "struct PreparedNormalFileSourcePlanRequestV1",
        "struct ClassifiedNormalFileSourcePlanV1",
        "struct RejectedNormalFileSourcePlanningV1",
    )
    for definition in input_definitions:
        require_count(frontdoor_input, definition, 1, f"sole INPUT0 definition {definition}")
    for fragment in (
        "fn prepare_source_plan_request(",
        "PreparedNormalSourcePlanInputV1::new(ast, display_identity)",
        "fn classify(",
        "NormalSourcePlanClassifierV1::seal(input)",
        "profile: SealedNormalEntryProfileV1",
        "receipt: NormalFileSourceReceiptV1",
        "fn stage(&self)",
        "fn error(&self)",
        "fn discard(self)",
    ):
        require(frontdoor_input, fragment, f"INPUT0 boundary {fragment}")
    require_count(
        frontdoor,
        "mod source_plan_input;",
        1,
        "front-door child-module declaration",
    )
    require_count(
        frontdoor,
        "fn prepare_raw_vm_handoff(self)",
        1,
        "unchanged narrow handoff terminal",
    )

    for test_name in (
        "parsed_empty_and_scalar_sources_become_script_plans_once",
        "parsed_main_zero_becomes_scalar_main_plan_once",
        "parsed_main_with_top_level_or_box_helper_becomes_callable_module",
        "parsed_function_only_retains_missing_entry_rejection",
        "parsed_script_plus_main_retains_mixed_family_rejection",
        "parse_and_using_rejections_never_issue_source_plan_requests",
    ):
        require(
            frontdoor_input_tests,
            f"fn {test_name}(",
            f"INPUT0 fixture {test_name}",
        )

    forbidden_classifier_authority = (
        "SealedNormalEntryProfileV1",
        "NormalFileNoImportVmReferenceV1",
        "RawVmReference",
        "MirBuilder",
        "MirInstruction",
        "ValueId",
        "MirType",
        "crate::runner",
        "crate::runtime",
        "NYASH_ENTRY",
        "module.functions",
        "compile_with_source",
        "build_module",
    )
    for forbidden in forbidden_classifier_authority:
        if forbidden in production:
            raise AssertionError(
                f"source classifier gained non-source authority: {forbidden}"
            )

    for forbidden in (
        "retry",
        "fallback",
        "reclassify",
        "into_ast",
        ".clone()",
        "ASTNode::Program { statements:",
    ):
        if forbidden in production:
            raise AssertionError(f"source owner gained forbidden operation: {forbidden}")

    for forbidden in (
        "prepare_raw_vm_handoff",
        "RawVmReference",
        "into_downstream",
        "match profile",
        "std::fs::",
        "NyashParser",
        "parse_from_string",
        ".clone()",
        "retry",
        "fallback",
        "compile_with_source",
        "build_module",
    ):
        if forbidden in frontdoor_input:
            raise AssertionError(
                f"INPUT0 gained forbidden route/profile/I/O authority: {forbidden}"
            )

    for marker in ("#[derive(Debug, Clone", "#[derive(Clone", "#[derive(Debug, Copy"):
        if marker in production:
            raise AssertionError(f"move-only source product became duplicable: {marker}")

    watched_symbols = (
        "NormalSourcePlanClassifierV1",
        "SealedNormalSourcePlanV1",
        "PreparedNormalSourcePlanInputV1",
    )
    allowed = set(ALL_FILES) | {
        COMPILER_MOD,
        MIR_ROOT,
        FRONTDOOR,
        FRONTDOOR_INPUT,
        FRONTDOOR_INPUT_TESTS,
    }
    for path in (ROOT / "src").rglob("*.rs"):
        if path in allowed:
            continue
        text = path.read_text()
        if any(symbol in text for symbol in watched_symbols):
            raise AssertionError(
                f"disconnected source-plan authority escaped S0: {path.relative_to(ROOT)}"
            )

    for path in (
        *ALL_FILES,
        FRONTDOOR,
        FRONTDOOR_INPUT,
        FRONTDOOR_INPUT_TESTS,
        Path(__file__),
    ):
        if len(path.read_text().splitlines()) >= 800:
            raise AssertionError(
                f"file must remain below 800 lines: {path.relative_to(ROOT)}"
            )

    print(
        "[normal-source-plan0-guard] ok "
        "classifier=1 script=1 main0=1 callable=1 profile=0 "
        "frontdoor_input=1 disconnected_consumer=1 production_consumer=0 "
        "second_read_parse=0 raw_route_delta=0 rewrite=0 below_800=1"
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
