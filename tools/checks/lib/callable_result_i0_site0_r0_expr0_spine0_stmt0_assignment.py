#!/usr/bin/env python3
"""Private ASN0 structural checks for the public EXPR0-SPINE0 guard."""

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


def check_asn0_s0(root: Path) -> str:
    module_path = "src/mir/builder/stmts/variable_assignment_descent.rs"
    tests_path = "src/mir/builder/stmts/variable_assignment_descent_tests.rs"
    stmts_root_path = "src/mir/builder/stmts/mod.rs"
    readme_path = "src/mir/builder/stmts/README.md"
    helper_path = (
        "tools/checks/lib/"
        "callable_result_i0_site0_r0_expr0_spine0_stmt0_assignment.py"
    )

    module = _read(root, module_path)
    tests = _read(root, tests_path)
    stmts_root = _read(root, stmts_root_path)
    readme = _read(root, readme_path)

    for needle, expected, label in (
        ("trait VariableAssignmentDescentPortV1", 1, "Variable Assignment port"),
        ("type VariableAssignmentInput;", 1, "associated Assignment input"),
        ("fn variable_assignment_syntax", 2, "syntax query and raw impl"),
        ("fn assignment_rhs_expression_input", 2, "RHS query and raw impl"),
        ("fn drive_variable_assignment_v1", 1, "Assignment driver"),
        ("impl VariableAssignmentDescentPortV1 for RawLegacyChildLoweringPortV1", 1, "raw port impl"),
        ("fn drive_raw_variable_assignment_v1", 1, "raw facade"),
        (
            "drive_variable_assignment_v1(builder, &mut port, &input)",
            1,
            "raw facade delegation",
        ),
        ("AssignmentResolverBox::ensure_declared(builder, &variable_name)?;", 1, "pre-RHS declared check"),
        ("drive_legacy_expression_v1(builder, port, rhs_input)?", 1, "RHS E0 descent"),
        ("builder.build_assignment_from_value(variable_name, rhs)", 1, "existing completion"),
    ):
        _require_count(module, needle, expected, label)

    syntax_at = module.index(".variable_assignment_syntax(input)?")
    preflight_at = module.index(
        "AssignmentResolverBox::ensure_declared(builder, &variable_name)?;"
    )
    input_at = module.index("port.assignment_rhs_expression_input(input)?")
    descent_at = module.index("drive_legacy_expression_v1(builder, port, rhs_input)?")
    completion_at = module.index("builder.build_assignment_from_value(variable_name, rhs)")
    if not syntax_at < preflight_at < input_at < descent_at < completion_at:
        _fail("ASN0 order must be syntax -> declared preflight -> RHS input/descent -> completion")

    for forbidden in (
        "build_expression(",
        "build_expression_impl(",
        "ASTNode::FieldAccess",
        "ASTNode::Index",
        "CompoundAssignment",
        "GroupedAssignment",
        "AssignStmt",
        "ExprChildRoleV1",
        "LegacyStmtInputV1",
        "LocatedLegacy",
        "CallableResult",
        "SourcePath",
        "ledger",
        "ReleaseStrong",
        "LocalContractWrite",
        "variable_map",
        "binding_ctx",
        "type_ctx",
        "retry",
        "fallback",
    ):
        if forbidden in module:
            _fail(f"ASN0-S0 substrate owns forbidden authority: {forbidden}")

    ignored = {
        (root / module_path).resolve(),
        (root / tests_path).resolve(),
    }
    driver_callers = 0
    raw_callers = 0
    for path in (root / "src").rglob("*.rs"):
        if path.resolve() in ignored:
            continue
        source = path.read_text(encoding="utf-8")
        driver_callers += source.count("drive_variable_assignment_v1(")
        raw_callers += source.count("drive_raw_variable_assignment_v1(")
    if driver_callers != 0:
        _fail(f"ASN0-S0 generic production callers must be zero: actual={driver_callers}")
    if raw_callers != 0:
        _fail(f"ASN0-S0 raw production callers must be zero: actual={raw_callers}")

    _require_count(
        stmts_root,
        "mod variable_assignment_descent;",
        1,
        "private Assignment descent module",
    )
    if not re.search(
        r"#\[cfg\(test\)\]\s*mod variable_assignment_descent_tests;",
        stmts_root,
    ):
        _fail("ASN0-S0 fixture module must remain cfg(test)-scoped")

    for fixture in (
        "declared_target_preflights_then_descends_rhs_and_completes_once",
        "undeclared_binding_missing_and_pin_targets_reject_before_rhs_effects",
        "syntax_and_rhs_input_failures_publish_no_rhs_or_assignment_effects",
        "rhs_failure_keeps_old_assignment_and_emits_no_completion_effect",
        "completion_recheck_rejects_lost_binding_and_fresh_attempt_succeeds",
        "raw_facade_reuses_recursive_binary_rhs_and_existing_completion",
    ):
        if fixture not in tests:
            _fail(f"missing ASN0-S0 fixture: {fixture}")

    for phrase in (
        "exact Variable-target",
        "field/index target syntax is structurally absent",
        "declared-binding preflight before requesting the",
        "existing `build_assignment_from_value` owner",
        "second completion-time declaration check is retained",
        "parity reference, and located `AssignmentValue` navigation stay disconnected",
    ):
        if phrase not in readme:
            _fail(f"missing ASN0-S0 README boundary: {phrase}")

    touched = (module_path, tests_path, stmts_root_path, readme_path, helper_path)
    oversized = [
        relative for relative in touched if len(_read(root, relative).splitlines()) >= 800
    ]
    if oversized:
        _fail(f"ASN0 source/check files reached 800 lines: {oversized}")
    if re.search(r"Arc<|Rc<|thread_local!|static mut", module):
        _fail("ASN0 substrate must remain stack-scoped and immutable")

    return "asn_driver=1 asn_e0_descents=1 asn_raw_impl=1 asn_production_callers=0"
