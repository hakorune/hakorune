#!/usr/bin/env python3
"""Private STMT0 structural checks for the public EXPR0-SPINE0 guard."""

from __future__ import annotations

import re
from pathlib import Path


def _fail(message: str) -> None:
    raise RuntimeError(message)


def _read(root: Path, relative: str) -> str:
    path = root / relative
    if not path.is_file():
        _fail(f"missing {relative}")
    return path.read_text(encoding="utf-8")


def _require_count(text: str, needle: str, expected: int, label: str) -> None:
    actual = text.count(needle)
    if actual != expected:
        _fail(f"{label}: expected={expected} actual={actual}")


def check_lcl0_s0(root: Path, located: str) -> str:
    local_descent_path = "src/mir/builder/stmts/local_statement_descent.rs"
    local_descent_tests_path = (
        "src/mir/builder/stmts/local_statement_descent_tests.rs"
    )
    stmts_root_path = "src/mir/builder/stmts/mod.rs"
    stmts_readme_path = "src/mir/builder/stmts/README.md"
    variable_stmt_path = "src/mir/builder/stmts/variable_stmt.rs"
    helper_path = (
        "tools/checks/lib/"
        "callable_result_i0_site0_r0_expr0_spine0_stmt0.py"
    )

    local_descent = _read(root, local_descent_path)
    local_descent_tests = _read(root, local_descent_tests_path)
    stmts_root = _read(root, stmts_root_path)
    stmts_readme = _read(root, stmts_readme_path)
    variable_stmt = _read(root, variable_stmt_path)

    _require_count(
        local_descent,
        "trait LocalStatementDescentPortV1",
        1,
        "Local initializer child-demand port owner",
    )
    _require_count(local_descent, "type LocalInput;", 1, "associated Local input")
    _require_count(local_descent, "fn local_syntax", 1, "Local syntax query owner")
    _require_count(
        local_descent,
        "fn local_initializer_expression_input",
        1,
        "ordinary initializer input query owner",
    )
    _require_count(
        local_descent,
        "fn lower_typed_array_literal_initializer",
        1,
        "typed-array specialized hook owner",
    )
    _require_count(
        local_descent,
        "fn lower_record_constructor_initializer",
        1,
        "record specialized hook owner",
    )
    _require_count(
        local_descent,
        "fn drive_local_statement_v1",
        1,
        "Local associated-input driver",
    )
    _require_count(
        local_descent,
        "preflight_exact_numeric_local_initializers(",
        1,
        "existing whole-Local preflight consumer",
    )
    _require_count(
        local_descent,
        "drive_legacy_expression_v1(builder, port, expression_input)?",
        1,
        "ordinary initializer E0 descent",
    )
    _require_count(
        local_descent,
        "build_local_statement_from_values_with_types_and_preclaims(",
        1,
        "existing Local completion consumer",
    )

    syntax_at = local_descent.index("let syntax = port.local_syntax(input)?")
    preflight_at = local_descent.index("preflight_exact_numeric_local_initializers(")
    loop_at = local_descent.index("for index in 0..variables.len()")
    typed_array_at = local_descent.index("port.lower_typed_array_literal_initializer(")
    record_at = local_descent.index(".lower_record_constructor_initializer(")
    input_at = local_descent.index(
        "port.local_initializer_expression_input(input, index)?"
    )
    descent_at = local_descent.index(
        "drive_legacy_expression_v1(builder, port, expression_input)?"
    )
    completion_at = local_descent.rindex(
        "build_local_statement_from_values_with_types_and_preclaims("
    )
    if not (
        syntax_at
        < preflight_at
        < loop_at
        < typed_array_at
        < record_at
        < input_at
        < descent_at
        < completion_at
    ):
        _fail(
            "LCL0 order must be syntax -> whole preflight -> ordinal routes -> completion"
        )

    for forbidden in (
        "build_expression(",
        "build_expression_impl(",
        "ExprChildRoleV1",
        "LegacyStmtInputV1",
        "LocatedLegacy",
        "CallableResult",
        "SourcePath",
        "ledger",
        "recursion_depth",
        "variable_map",
        "binding_ctx",
        "type_ctx",
        "MirInstruction",
        "retry",
        "fallback",
    ):
        if forbidden in local_descent:
            _fail(f"LCL0 substrate owns forbidden authority: {forbidden}")

    all_rust_source = "\n".join(
        path.read_text(encoding="utf-8") for path in (root / "src").rglob("*.rs")
    )
    normalized_rust_source = re.sub(r"\s+", " ", all_rust_source)
    if (
        "impl LocalStatementDescentPortV1 for RawLegacyChildLoweringPortV1"
        in normalized_rust_source
    ):
        _fail("LCL0-S0 raw implementation must remain zero")
    if (
        "LocalStatementDescentPortV1 for LocatedLegacyLoweringSessionV1"
        in normalized_rust_source
    ):
        _fail("LCL0-S0 located implementation must remain zero")

    local_driver_callers = 0
    local_ignored = {
        (root / local_descent_path).resolve(),
        (root / local_descent_tests_path).resolve(),
    }
    for path in (root / "src").rglob("*.rs"):
        if path.resolve() in local_ignored:
            continue
        local_driver_callers += path.read_text(encoding="utf-8").count(
            "drive_local_statement_v1("
        )
    if local_driver_callers != 0:
        _fail(
            "LCL0-S0 production/raw/located driver callers must remain zero: "
            f"actual={local_driver_callers}"
        )

    _require_count(
        variable_stmt,
        "pub(in crate::mir::builder) fn preflight_exact_numeric_local_initializers",
        1,
        "existing Local preflight exposure",
    )
    _require_count(
        variable_stmt,
        "pub(in crate::mir::builder) fn "
        "build_local_statement_from_values_with_types_and_preclaims",
        1,
        "existing Local completion exposure",
    )
    _require_count(
        stmts_root,
        "mod local_statement_descent;",
        1,
        "private Local descent module",
    )
    _require_count(
        stmts_root,
        "mod local_statement_descent_tests;",
        1,
        "focused Local descent fixture module",
    )

    for fixture in (
        "ordinary_initializers_preflight_then_descend_in_index_order_and_complete_once",
        "later_exact_numeric_missing_initializer_rejects_before_first_child_effect",
        "later_typed_array_declaration_rejects_before_first_child_effect",
        "untyped_missing_initializer_uses_null_without_child_demand",
        "syntax_failure_precedes_preflight_or_initializer_effects",
        "initializer_input_and_child_failures_publish_no_binding_or_later_initializer",
        "typed_array_special_hook_precedes_direct_builder_effects_and_preclaim_reaches_local",
        "record_special_hook_precedes_constructor_effects",
    ):
        if fixture not in local_descent_tests:
            _fail(f"missing LCL0-S0 fixture: {fixture}")

    for phrase in (
        "Local declaration preflight",
        "existing whole-declaration exact-numeric/typed-array preflight",
        "request ordinary initializer expressions in declaration order",
        "prove the exact `LocalInitializer(index)` subtree",
        "must not reconstruct source sites",
        "first LCL0-S0 slice is disconnected",
    ):
        if phrase not in stmts_readme:
            _fail(f"missing LCL0 README boundary: {phrase}")

    touched = (
        local_descent_path,
        local_descent_tests_path,
        stmts_root_path,
        stmts_readme_path,
        variable_stmt_path,
        helper_path,
    )
    oversized = [
        relative for relative in touched if len(_read(root, relative).splitlines()) >= 800
    ]
    if oversized:
        _fail(f"LCL0 source/check files reached 800 lines: {oversized}")
    if re.search(r"Arc<|Rc<|thread_local!|static mut", local_descent):
        _fail("LCL0 substrate must remain stack-scoped and immutable")

    return "lcl_driver=1 lcl_e0_descents=1 lcl_production_callers=0"
