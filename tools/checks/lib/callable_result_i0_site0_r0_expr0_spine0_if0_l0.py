#!/usr/bin/env python3
"""Structural checks for the disconnected located statement-If adapter."""

from __future__ import annotations

from pathlib import Path


def _fail(message: str) -> None:
    raise RuntimeError(message)


def _read(root: Path, relative: str) -> str:
    path = root / relative
    if not path.is_file():
        _fail(f"missing {relative}")
    return path.read_text(encoding="utf-8")


def _count(text: str, needle: str, expected: int, label: str) -> None:
    actual = text.count(needle)
    if actual != expected:
        _fail(f"{label}: expected={expected} actual={actual}")


def check_if0_l0(root: Path) -> str:
    adapter_path = "src/mir/builder/located_legacy_if.rs"
    tests_path = "src/mir/builder/located_legacy_if_tests.rs"
    session_path = "src/mir/builder/located_legacy_lowering.rs"
    driver_path = "src/mir/builder/stmts/if_statement_descent.rs"
    block_path = "src/mir/builder/stmts/block_stmt.rs"
    helper_path = (
        "tools/checks/lib/"
        "callable_result_i0_site0_r0_expr0_spine0_if0_l0.py"
    )

    adapter = _read(root, adapter_path)
    tests = _read(root, tests_path)
    session = _read(root, session_path)
    driver = _read(root, driver_path)
    block = _read(root, block_path)

    _count(adapter, "struct LocatedStatementIfInputV1", 1, "located If input")
    _count(adapter, "fn select_exact_statement_if_v1", 1, "located If selector")
    _count(adapter, "ASTNode::If {", 1, "exact If syntax selector")
    _count(adapter, "_ => return Err(input)", 1, "carrier-preserving rejection")
    _count(adapter, "impl<'plan> IfStatementDescentPortV1", 1, "located If port")
    _count(adapter, "ExprChildRoleV1::IfCondition", 1, "condition PATH0 role")
    _count(adapter, "BodyChildRoleV1::IfThen", 1, "then PATH0 role")
    _count(adapter, "BodyChildRoleV1::IfElse", 1, "else PATH0 role")
    _count(adapter, "drive_if_statement_v1(builder, session, selected)", 1, "shared driver")
    _count(adapter, "complete_if_statement_v1(builder, lowering)", 1, "shared completion")
    _count(adapter, "emit_void", 0, "adapter-owned Void")
    _count(
        session,
        "if_adapter::select_exact_statement_if_v1(input)",
        1,
        "one located selector consumer",
    )
    _count(driver, "fn complete_if_statement_v1", 1, "completion owner")
    _count(driver, "lowering?;", 1, "completion success preflight")
    _count(driver, "emission::constant::emit_void(builder)", 1, "existing Void owner")
    _count(
        adapter + block,
        "complete_if_statement_v1(builder, lowering)",
        2,
        "raw and located completion consumers",
    )

    for forbidden in (
        "SourcePathSegmentV1",
        "SourcePathV1",
        ".child(",
        "LoopBody",
        "MatchExpr",
        "suffix_route_input",
        "ASTNode::Program",
        "build_if_statement",
        "cf_if",
        "drive_short_circuit",
        "FastMem",
        "IfForm",
        "lower_if_form",
        "MirInstruction::Phi",
        "BasicBlock",
        ".ledger",
        ".claim(",
        "prove_body_inactive",
        "prove_stmt_inactive",
        "drive_raw_legacy_body",
        "retry",
        "fallback",
    ):
        if forbidden in adapter:
            _fail(f"located If adapter owns forbidden authority: {forbidden}")

    selector_site = session.index("if_adapter::select_exact_statement_if_v1(input)")
    inactive_site = session.index(".prove_stmt_inactive(&input)", selector_site)
    if selector_site >= inactive_site:
        _fail("located If selector must precede inactive-statement delegation")

    completion_callers = 0
    for path in (root / "src").rglob("*.rs"):
        completion_callers += path.read_text(encoding="utf-8").count(
            "complete_if_statement_v1(builder, lowering)"
        )
    if completion_callers != 2:
        _fail(f"shared If completion callers: expected=2 actual={completion_callers}")

    for fixture in (
        "located_statement_if_claims_actual_top_level_condition_lhs",
        "located_statement_if_reuses_short_circuit_condition_descent",
        "located_statement_if_orders_condition_rows_and_lowers_inactive_branches",
        "active_then_row_fails_before_raw_branch_effects_and_poisons_session",
        "active_else_row_fails_without_else_call_effects_and_poisons_session",
        "condition_wrong_order_has_no_control_effects_then_fresh_session_succeeds",
        "root_loop_keeps_nested_if_row_parked_for_loop0",
        "located_statement_if_selector_rejects_non_if_without_rebuilding_carrier",
    ):
        if fixture not in tests:
            _fail(f"missing IF0-L0 fixture: {fixture}")
    for evidence in (
        'contains("WrongOrder")',
        "CallableResultCallerLedgerErrorV1::RowsUnderPrefix",
        "LocatedIfBoundarySnapshotV1",
        "core_next_value",
        "core_next_block",
        "all_void_const_count(&builder), 0",
        "all_void_const_count(&builder), 1",
        "instructions(builder)\n        .into_iter()",
        "LocatedLegacyLoweringErrorV1::Poisoned",
        "boundary_snapshot(&builder), before",
        "Helpers.left",
        "Helpers.right",
        "MirInstruction::BinOp",
        "MirInstruction::Phi",
        "MirInstruction::Branch",
        "expected_present_marker",
        "expected_absent_marker",
        "ConstValue::Integer(value)",
    ):
        if evidence not in tests:
            _fail(f"missing IF0-L0 exact evidence: {evidence}")

    production_verify_callers = 0
    for path in (root / "src").rglob("*.rs"):
        relative = path.relative_to(root).as_posix()
        if path.name.endswith("_tests.rs") or "/tests/" in relative:
            continue
        text = path.read_text(encoding="utf-8")
        production_verify_callers += text.count("LocatedLegacyLoweringSessionV1::verify(")
    if production_verify_callers != 0:
        _fail(f"located session production roots: {production_verify_callers}")

    touched = (adapter_path, tests_path, session_path, driver_path, block_path, helper_path)
    oversized = [path for path in touched if len(_read(root, path).splitlines()) >= 800]
    if oversized:
        _fail(f"IF0-L0 source/check files reached 800 lines: {oversized}")
    return "if_located_adapter=1 if_located_roots=0"
