#!/usr/bin/env python3
"""Private IF0-S0 structural checks for the public EXPR0-SPINE0 guard."""

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


def check_if0_s0(root: Path) -> str:
    driver_path = "src/mir/builder/stmts/if_statement_descent.rs"
    tests_path = "src/mir/builder/stmts/if_statement_descent_tests.rs"
    stmts_root_path = "src/mir/builder/stmts/mod.rs"
    readme_path = "src/mir/builder/stmts/README.md"
    if_form_path = "src/mir/builder/if_form.rs"
    phi_path = "src/mir/builder/phi.rs"
    helper_path = (
        "tools/checks/lib/"
        "callable_result_i0_site0_r0_expr0_spine0_if0.py"
    )

    driver = _read(root, driver_path)
    tests = _read(root, tests_path)
    stmts_root = _read(root, stmts_root_path)
    readme = _read(root, readme_path)
    if_form = _read(root, if_form_path)
    phi = _read(root, phi_path)

    _require_count(driver, "trait IfStatementDescentPortV1", 1, "If port owner")
    _require_count(driver, "type IfInput;", 1, "associated If input")
    _require_count(driver, "struct IfStatementSyntaxViewV1", 1, "If syntax view")
    _require_count(driver, "fn if_syntax", 2, "syntax declaration plus raw impl")
    _require_count(
        driver,
        "fn if_condition_expression_input",
        2,
        "condition input declaration plus raw impl",
    )
    _require_count(
        driver,
        "fn if_then_body_input",
        2,
        "then input declaration plus raw impl",
    )
    _require_count(
        driver,
        "fn if_else_body_input",
        2,
        "else input declaration plus raw impl",
    )
    _require_count(driver, "fn drive_if_statement_v1", 1, "If driver")
    _require_count(
        driver,
        "drive_legacy_expression_v1(builder, port, condition_input)?",
        1,
        "one condition E0 descent",
    )
    _require_count(
        driver,
        "drive_legacy_body_v1(builder, port, body)",
        1,
        "one branch body descent site",
    )
    _require_count(
        driver,
        "prepare_if_statement_condition_value_v1",
        2,
        "FastMem preparation definition plus consumer",
    )
    _require_count(
        driver + stmts_root,
        "prepare_if_statement_condition_value_v1",
        3,
        "one FastMem helper definition plus disconnected and legacy consumers",
    )
    _require_count(
        driver,
        "ensure_fastmem_owner_eq_condition(",
        1,
        "one FastMem condition verifier owner",
    )
    _require_count(
        driver,
        "add_fastmem_branch_condition_fact(",
        1,
        "one FastMem condition-fact owner",
    )
    if "ensure_fastmem_owner_eq_condition(" in stmts_root:
        _fail("legacy statement If duplicates FastMem condition verification")
    if "add_fastmem_branch_condition_fact(" in stmts_root:
        _fail("legacy statement If duplicates FastMem fact publication")
    _require_count(
        driver,
        "lower_if_form_with_condition_value_and_branch_lowerer(",
        1,
        "one existing IfForm completion consumer",
    )

    driver_at = driver.index("fn drive_if_statement_v1<Port>")
    driver_end = driver.index("fn drive_raw_if_statement_v1", driver_at)
    driver_body = driver[driver_at:driver_end]
    syntax_at = driver_body.index("port.if_syntax(input)?")
    condition_input_at = driver_body.index("port.if_condition_expression_input(input)?")
    condition_lower_at = driver_body.index(
        "drive_legacy_expression_v1(builder, port, condition_input)?"
    )
    prepare_at = driver_body.index("prepare_if_statement_condition_value_v1(")
    then_input_at = driver_body.index("port.if_then_body_input(input)?")
    else_input_at = driver_body.index("port.if_else_body_input(input)?")
    if_form_at = driver_body.index(
        "lower_if_form_with_condition_value_and_branch_lowerer("
    )
    body_lower_at = driver_body.index("drive_legacy_body_v1(builder, port, body)")
    if not (
        syntax_at
        < condition_input_at
        < condition_lower_at
        < prepare_at
        < if_form_at
        < then_input_at
        < else_input_at
        < body_lower_at
    ):
        _fail("IF0 order must be condition -> FastMem -> IfForm lazy branch demand")

    for forbidden in (
        "SourcePathSegmentV1",
        "LegacyExprInputV1",
        "LegacyBodyInputV1",
        "ledger",
        "claim(",
        "MatchExpr",
        "LoopBody",
        "suffix_route_input",
        "cf_if(",
        "build_expression(",
        "build_statement(",
        "MirInstruction",
        "define_phi",
        "emit_conditional",
        "retry",
        "fallback",
    ):
        if forbidden in driver:
            _fail(f"IF0 driver owns forbidden authority: {forbidden}")

    _require_count(if_form, "enum IfBranchKindV1", 1, "one branch-demand vocabulary")
    _require_count(
        if_form,
        "fn lower_if_form_with_condition_value_and_branch_lowerer",
        1,
        "one callback-based IfForm core",
    )
    core_at = if_form.index("fn lower_if_form_with_condition_value_and_branch_lowerer")
    core = if_form[core_at:]
    _require_count(core, "lower_branch(self, IfBranchKindV1::Then)?", 1, "then demand")
    _require_count(core, "lower_branch(self, IfBranchKindV1::Else)?", 1, "else demand")
    if "self.build_expression(" in core:
        _fail("callback-based IfForm core must not raw-lower branch syntax")
    then_at = core.index("lower_branch(self, IfBranchKindV1::Then)?")
    else_at = core.index("lower_branch(self, IfBranchKindV1::Else)?")
    edge_at = core.index(
        "crate::mir::builder::emission::branch::emit_conditional_edgecfg("
    )
    if not then_at < else_at < edge_at:
        _fail("IfForm must lower then -> optional else before existing EdgeCFG commit")

    _require_count(
        if_form,
        "lower_if_form_with_condition_value_and_branch_lowerer(",
        1,
        "one legacy raw wrapper consumer",
    )
    _require_count(
        phi,
        "fn normalize_if_else_phi(",
        1,
        "existing If result-PHI owner",
    )
    if "_then_ast_for_analysis" in phi or "_else_ast_for_analysis" in phi:
        _fail("retired unused If analysis syntax parameters remain")

    _require_count(
        stmts_root,
        "mod if_statement_descent;",
        1,
        "private IF0 substrate module",
    )
    _require_count(
        stmts_root,
        "mod if_statement_descent_tests;",
        1,
        "focused IF0 fixture module",
    )

    for fixture in (
        "if_driver_demands_condition_then_and_else_in_exact_order",
        "if_driver_condition_failure_precedes_ifform_effects",
        "if_driver_fastmem_failure_requests_no_branch_or_cfg",
        "if_driver_fastmem_success_publishes_one_existing_condition_fact",
        "if_driver_branch_failures_preserve_exact_demand_boundary",
        "if_driver_implicit_false_never_requests_else",
        "if_driver_preserves_termination_and_variable_phi_shapes",
    ):
        if fixture not in tests:
            _fail(f"missing IF0-S0 fixture: {fixture}")
    for evidence in (
        "block.terminator.clone()",
        "block.predecessors",
        '"then-lower"',
        '"else-lower"',
        "FastMemRegionId",
        "MirInstruction::Branch",
        "MirInstruction::Return",
        "MirInstruction::Phi",
        "builder.recursion_depth",
        "IfPartialStateSnapshotV1",
        "assert_simple_if_termination_shape",
        "merge.predecessors",
        "FastMemBranchConditionProofKind::SourceAssumeOwnerEq",
    ):
        if evidence not in tests:
            _fail(f"missing IF0-S0 fixture evidence: {evidence}")

    production_driver_callers = 0
    raw_driver_callers = 0
    ignored = {(root / driver_path).resolve(), (root / tests_path).resolve()}
    generic_call = re.compile(r"\bdrive_if_statement_v1(?:\s*::\s*<[^>]*>)?\s*\(")
    raw_call = re.compile(r"\bdrive_raw_if_statement_v1(?:\s*::\s*<[^>]*>)?\s*\(")
    for path in (root / "src").rglob("*.rs"):
        if path.resolve() in ignored:
            continue
        text = path.read_text(encoding="utf-8")
        production_driver_callers += len(generic_call.findall(text))
        raw_driver_callers += len(raw_call.findall(text))
    if production_driver_callers != 0:
        _fail(f"IF0-S0 production generic driver callers: {production_driver_callers}")
    if raw_driver_callers != 0:
        _fail(f"IF0-S0 production raw driver callers: {raw_driver_callers}")

    for phrase in (
        "disconnected statement-If child-demand",
        "existing FastMem post-condition verification",
        "IfForm remains the sole block",
        "one callback invoked at the existing then",
        "Statement Void publication remains",
        "source paths or caller ledger",
        "It owns no Match, Loop, suffix routing",
    ):
        if phrase not in readme:
            _fail(f"missing IF0-S0 README boundary: {phrase}")

    touched = (
        driver_path,
        tests_path,
        stmts_root_path,
        readme_path,
        if_form_path,
        phi_path,
        helper_path,
    )
    oversized = [relative for relative in touched if len(_read(root, relative).splitlines()) >= 800]
    if oversized:
        _fail(f"IF0-S0 source/check files reached 800 lines: {oversized}")

    return "if_driver=1 if_branch_core=1 if_production_callers=0"
