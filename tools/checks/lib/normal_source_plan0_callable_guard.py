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
    callable_source_path = source_dir / "callable_source.rs"
    callable_source_tests_path = source_dir / "callable_source_tests.rs"
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
        callable_source_path,
        callable_source_tests_path,
        header_source_path,
        header_source_tests_path,
        header_view_path,
        Path(__file__),
    )

    task = task_path.read_text()
    callable_source = callable_source_path.read_text()
    callable_source_tests = callable_source_tests_path.read_text()
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
