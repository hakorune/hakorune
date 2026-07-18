#!/usr/bin/env python3
"""Private RET0-S0 structural checks for the public EXPR0-SPINE0 guard."""

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


def check_ret0_s0(root: Path) -> str:
    descent_path = "src/mir/builder/stmts/return_statement_descent.rs"
    tests_path = "src/mir/builder/stmts/return_statement_descent_tests.rs"
    return_owner_path = "src/mir/builder/stmts/return_stmt.rs"
    stmts_root_path = "src/mir/builder/stmts/mod.rs"
    readme_path = "src/mir/builder/stmts/README.md"
    helper_path = (
        "tools/checks/lib/"
        "callable_result_i0_site0_r0_expr0_spine0_stmt0_return.py"
    )

    descent = _read(root, descent_path)
    tests = _read(root, tests_path)
    return_owner = _read(root, return_owner_path)
    stmts_root = _read(root, stmts_root_path)
    readme = _read(root, readme_path)

    _require_count(
        descent,
        "trait ReturnStatementDescentPortV1",
        1,
        "value-bearing Return port owner",
    )
    _require_count(descent, "type ReturnInput;", 1, "associated Return input")
    _require_count(
        descent,
        "fn return_value_syntax",
        2,
        "Return syntax declaration plus raw implementation",
    )
    _require_count(
        descent,
        "fn try_match_return_optimization",
        2,
        "match hook declaration plus raw implementation",
    )
    _require_count(
        descent,
        "fn return_value_expression_input",
        2,
        "ReturnValue input declaration plus raw implementation",
    )
    _require_count(
        descent,
        "fn drive_value_return_statement_v1",
        1,
        "value-bearing Return driver",
    )
    _require_count(
        descent,
        "ensure_return_allowed(builder)?;",
        1,
        "existing cleanup prohibition consumer",
    )
    _require_count(
        descent,
        "try_apply_match_return_optimization(builder, Some(value), true)",
        1,
        "existing raw match-return owner delegation",
    )
    _require_count(
        descent,
        "drive_legacy_expression_v1(builder, port, expression_input)?",
        1,
        "one ReturnValue E0 descent",
    )
    _require_count(
        descent,
        "emit_return_from_value(builder, return_value)",
        1,
        "existing Return completion consumer",
    )

    cleanup_at = descent.index("ensure_return_allowed(builder)?;")
    syntax_at = descent.index("port.return_value_syntax(input)?")
    match_at = descent.index("port.try_match_return_optimization(builder, input, value)?")
    input_at = descent.index("port.return_value_expression_input(input)?")
    child_at = descent.index(
        "drive_legacy_expression_v1(builder, port, expression_input)?"
    )
    completion_at = descent.index("emit_return_from_value(builder, return_value)")
    if not cleanup_at < syntax_at < match_at < input_at < child_at < completion_at:
        _fail(
            "RET0 order must be cleanup -> syntax -> match -> input -> child -> completion"
        )

    raw_input_start = descent.index("struct RawLegacyValueReturnInputV1")
    syntax_view_start = descent.index("struct ReturnStatementSyntaxViewV1")
    trait_start = descent.index("trait ReturnStatementDescentPortV1")
    if "Option" in descent[raw_input_start:syntax_view_start]:
        _fail("RET0 raw input must make one Return value structurally mandatory")
    if "Option" in descent[syntax_view_start:trait_start]:
        _fail("RET0 syntax view must not admit void Return")

    for forbidden in (
        "MatchReturnFacts",
        "compose_match_return",
        "PlanVerifier",
        "PlanLowerer",
        "ExprChildRoleV1",
        "LegacyStmtInputV1",
        "LocatedLegacy",
        "CallableResult",
        "SourcePath",
        "ledger",
        "return_defer_active =",
        "MirInstruction::Return",
        "build_expression(",
        "retry",
        "fallback",
    ):
        if forbidden in descent:
            _fail(f"RET0 substrate owns forbidden authority: {forbidden}")

    _require_count(
        return_owner,
        "fn ensure_return_allowed",
        1,
        "single cleanup prohibition owner",
    )
    _require_count(
        return_owner,
        "ensure_return_allowed(builder)?;",
        1,
        "legacy Return facade cleanup consumer",
    )
    _require_count(
        return_owner,
        "try_apply_match_return_optimization(builder, value.as_deref(), true)?",
        1,
        "legacy match-return selection preserved",
    )
    _require_count(
        return_owner,
        "builder.build_expression(*expr)?",
        1,
        "legacy value lowering preserved",
    )
    _require_count(
        return_owner,
        "crate::mir::builder::emission::constant::emit_void(builder)?",
        1,
        "legacy Void Return lowering preserved",
    )
    _require_count(
        return_owner,
        "emit_return_from_value(builder, return_value)",
        1,
        "legacy Return completion preserved",
    )

    ignored = {
        (root / descent_path).resolve(),
        (root / tests_path).resolve(),
    }
    driver_callers = 0
    raw_callers = 0
    for path in (root / "src").rglob("*.rs"):
        if path.resolve() in ignored:
            continue
        source = path.read_text(encoding="utf-8")
        driver_callers += source.count("drive_value_return_statement_v1(")
        raw_callers += source.count("drive_raw_value_return_statement_v1(")
    if driver_callers != 0:
        _fail(f"RET0-S0 production driver callers must be zero: actual={driver_callers}")
    if raw_callers != 0:
        _fail(f"RET0-S0 raw production selectors must be zero: actual={raw_callers}")

    _require_count(
        stmts_root,
        "pub(in crate::mir::builder) mod return_statement_descent;",
        1,
        "Builder-private Return descent module",
    )
    if not re.search(
        r"#\[cfg\(test\)\]\s*mod return_statement_descent_tests;",
        stmts_root,
    ):
        _fail("RET0-S0 fixture module must remain cfg(test)-scoped")

    for fixture in (
        "cleanup_precedes_match_child_and_return_effects",
        "ordinary_value_probes_then_descends_once_and_completes_once",
        "selected_match_bypasses_value_demand_and_ordinary_completion",
        "syntax_match_input_and_child_failures_emit_no_return_completion",
        "configured_defer_reuses_copy_and_jump_completion_without_direct_return",
        "raw_value_return_reuses_binary_and_short_circuit_child_spines",
        "raw_value_return_reuses_actual_method_call_child_spine",
        "value_return_input_excludes_void_while_legacy_void_return_remains",
    ):
        if fixture not in tests:
            _fail(f"missing RET0-S0 fixture: {fixture}")

    for phrase in (
        "one disconnected orchestration boundary for",
        "runs the existing cleanup prohibition first",
        "delegates the existing match-return probe",
        "requests `ReturnValue` once",
        "existing `emit_return_from_value` owner",
        "does not admit `return;`",
        "must not reconstruct",
    ):
        if phrase not in readme:
            _fail(f"missing RET0-S0 README boundary: {phrase}")

    touched = (
        descent_path,
        tests_path,
        return_owner_path,
        stmts_root_path,
        readme_path,
        helper_path,
    )
    oversized = [
        relative for relative in touched if len(_read(root, relative).splitlines()) >= 800
    ]
    if oversized:
        _fail(f"RET0-S0 source/check files reached 800 lines: {oversized}")
    if re.search(r"Arc<|Rc<|thread_local!|static mut", descent):
        _fail("RET0-S0 substrate must remain stack-scoped and non-persistent")

    return "ret_driver=1 ret_e0_descents=1 ret_match_hook=1 ret_production_callers=0"
