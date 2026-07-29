#!/usr/bin/env python3
"""Private IF0-S0/I0/P0 structural checks for the public EXPR0-SPINE0 guard."""

from __future__ import annotations

import re
from pathlib import Path

from callable_result_i0_site0_r0_expr0_spine0_if0_l0 import check_if0_l0


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


def _function_slice(text: str, signature: str) -> str:
    start = text.find(signature)
    if start < 0:
        _fail(f"missing function signature: {signature}")
    brace = text.find("{", start)
    if brace < 0:
        _fail(f"missing function body: {signature}")
    depth = 0
    for index in range(brace, len(text)):
        if text[index] == "{":
            depth += 1
        elif text[index] == "}":
            depth -= 1
            if depth == 0:
                return text[start : index + 1]
    _fail(f"unterminated function body: {signature}")
    raise AssertionError("unreachable")


def check_if0_s0(root: Path) -> str:
    driver_path = "src/mir/builder/stmts/if_statement_descent.rs"
    tests_path = "src/mir/builder/stmts/if_statement_descent_tests.rs"
    raw_tests_path = "src/mir/builder/stmts/if_statement_raw_tests.rs"
    parity_tests_path = "src/mir/builder/stmts/if_statement_parity_tests.rs"
    stmts_root_path = "src/mir/builder/stmts/mod.rs"
    block_stmt_path = "src/mir/builder/stmts/block_stmt.rs"
    exprs_path = "src/mir/builder/raw_expression_dispatch/statement_surface.rs"
    located_if_path = "src/mir/builder/resolved_lowering/located_if.rs"
    control_flow_path = "src/mir/builder/control_flow/mod.rs"
    readme_path = "src/mir/builder/stmts/README.md"
    if_form_path = "src/mir/builder/if_form.rs"
    phi_path = "src/mir/builder/phi.rs"
    helper_path = (
        "tools/checks/lib/"
        "callable_result_i0_site0_r0_expr0_spine0_if0.py"
    )

    driver = _read(root, driver_path)
    tests = _read(root, tests_path)
    raw_tests = _read(root, raw_tests_path)
    parity_tests = _read(root, parity_tests_path)
    stmts_root = _read(root, stmts_root_path)
    block_stmt = _read(root, block_stmt_path)
    exprs = _read(root, exprs_path)
    located_if = _read(root, located_if_path)
    control_flow = _read(root, control_flow_path)
    readme = _read(root, readme_path)
    if_form = _read(root, if_form_path)
    phi = _read(root, phi_path)

    _require_count(driver, "trait IfStatementDescentPortV1", 1, "If port owner")
    _require_count(driver, "type IfInput;", 1, "associated If input")
    _require_count(driver, "struct IfStatementSyntaxViewV1", 1, "If syntax view")
    _require_count(driver, "fn if_syntax", 2, "syntax declaration plus sole raw port")
    _require_count(
        driver,
        "fn if_condition_expression_input",
        2,
        "condition input declaration plus sole raw port",
    )
    _require_count(
        driver,
        "fn if_then_body_input",
        2,
        "then input declaration plus sole raw port",
    )
    _require_count(
        driver,
        "fn if_else_body_input",
        2,
        "else input declaration plus sole raw port",
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
        2,
        "one FastMem helper definition plus shared-driver consumer",
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
    _require_count(
        driver,
        "struct RawStatementIfPortV1<'port, Port>",
        1,
        "one production raw If port",
    )
    _require_count(
        driver,
        "RawLegacyChildLoweringPortV1",
        0,
        "retired alternate raw If port",
    )
    _require_count(
        driver,
        "ASTNode::Program {",
        1,
        "one legacy branch Program shell",
    )
    _require_count(driver, "Span::unknown()", 1, "one unknown branch Program span")
    _require_count(
        driver,
        "let mut port = RawStatementIfPortV1::new(child);",
        1,
        "raw wrapper selects production If port once",
    )
    _require_count(
        driver,
        "drive_if_statement_v1(builder, &mut port, &input)",
        1,
        "raw wrapper selects shared driver once",
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
    _require_count(if_form, "fn lower_if_form(", 0, "retired raw IfForm facade")
    _require_count(
        if_form,
        "fn lower_if_form_with_condition_value(",
        0,
        "retired condition-value IfForm facade",
    )
    _require_count(if_form, "build_expression(", 0, "retired IfForm raw facade edges")
    _require_count(
        control_flow,
        "pub(super) fn cf_if(",
        0,
        "retired raw control-flow If facade",
    )
    _require_count(
        control_flow,
        "pub(in crate::mir::builder) fn cf_if_with_port_v1<Port>(",
        1,
        "selected-port If owner",
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
    _require_count(
        stmts_root,
        "mod if_statement_raw_tests;",
        1,
        "production IF0 fixture module",
    )
    _require_count(
        stmts_root,
        "mod if_statement_parity_tests;",
        1,
        "cfg(test) IF0-P0 parity module",
    )
    _require_count(
        stmts_root,
        "#[cfg(test)]\nmod if_statement_parity_tests;",
        1,
        "IF0-P0 parity module remains test-only",
    )

    statement_dispatch = _function_slice(
        block_stmt,
        "pub(in crate::mir::builder) fn build_statement_with_port_v1<Port>(",
    )
    if_at = statement_dispatch.index("ASTNode::If {")
    next_arm_at = statement_dispatch.index("ASTNode::StaticConstTable", if_at)
    if_arm = statement_dispatch[if_at:next_arm_at]
    _require_count(
        if_arm,
        "if_statement_descent::drive_raw_if_statement_with_port_v1(",
        1,
        "one statement If selector",
    )
    _require_count(
        if_arm,
        "if_statement_descent::complete_if_statement_v1(builder, lowering)",
        1,
        "one shared facade completion",
    )
    _require_count(if_arm, "emit_void(builder)?", 0, "no duplicated facade Void")

    expression_dispatch = _function_slice(
        exprs, "pub(super) fn try_build_with_port_v1<Port>("
    )
    _require_count(
        expression_dispatch,
        "builder.cf_if_with_port_v1(",
        1,
        "expression If selected-port owner",
    )
    if "drive_raw_if_statement_v1" in expression_dispatch:
        _fail("expression-position If selects statement raw driver")
    if "drive_raw_if_statement_v1" in located_if:
        _fail("resolved located If selects legacy raw statement driver")

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

    for fixture in (
        "production_statement_if_explicit_else_publishes_merge_phis_then_facade_void",
        "production_statement_if_implicit_else_keeps_internal_and_facade_void_distinct",
        "production_statement_if_preserves_branch_termination_matrix",
        "production_statement_if_failures_emit_no_facade_void_and_do_not_retry",
        "production_statement_if_preserves_program_shell_recursion_boundary",
        "production_statement_if_preserves_branch_program_span_shell",
        "production_statement_if_fastmem_preserves_positive_and_negative_admission",
        "expression_if_remains_cf_if_value_route_without_statement_void",
    ):
        if fixture not in raw_tests:
            _fail(f"missing IF0-I0 fixture: {fixture}")
    for evidence in (
        "super::block_stmt::build_statement",
        "block.terminator",
        ".predecessors",
        "MirInstruction::Phi",
        "ConstValue::Void",
        ".recursion_depth = 198",
        ".recursion_depth = 199",
        ".recursion_depth = 200",
        "metadata_ctx.current_span()",
        "Span::unknown()",
        "FastMemBranchConditionProofKind::SourceAssumeOwnerEq",
        ".build_expression(statement_if(",
    ):
        if evidence not in raw_tests:
            _fail(f"missing IF0-I0 fixture evidence: {evidence}")
    for forbidden in ("drive_if_statement_v1(", "drive_raw_if_statement_v1("):
        if forbidden in raw_tests:
            _fail(f"IF0-I0 fixture bypasses production facade: {forbidden}")

    _require_count(
        parity_tests,
        "fn lower_pre_i0_statement_if_reference(",
        1,
        "one pre-I0 If orchestration reference",
    )
    _require_count(
        parity_tests,
        "fn lower_pre_i0_statement_surface_reference(",
        1,
        "one pre-I0 statement surface reference",
    )
    for symbol in (
        "fn lower_pre_i0_statement_if_reference(",
        "fn lower_pre_i0_statement_surface_reference(",
    ):
        outside = 0
        parity_path = (root / parity_tests_path).resolve()
        for path in (root / "src").rglob("*.rs"):
            if path.resolve() == parity_path:
                continue
            outside += path.read_text(encoding="utf-8").count(symbol)
        if outside != 0:
            _fail(f"IF0-P0 reference owner escaped cfg(test) module: {symbol}")
    _require_count(
        parity_tests,
        "super::block_stmt::build_statement(builder, statement)",
        1,
        "selected parity path uses production statement entry",
    )
    reference = _function_slice(
        parity_tests, "fn lower_pre_i0_statement_if_reference("
    )
    surface_reference = _function_slice(
        parity_tests, "fn lower_pre_i0_statement_surface_reference("
    )
    for forbidden in (
        "build_if_statement(",
        "drive_if_statement_v1(",
        "drive_raw_if_statement_v1(",
        "RawLegacyIfStatementPortV1",
        "prepare_if_statement_condition_value_v1(",
        "retry",
        "fallback",
    ):
        if forbidden in reference + surface_reference:
            _fail(f"IF0-P0 reference reuses selected authority: {forbidden}")
    for evidence, expected, label in (
        (
            "builder.cf_if_with_port_v1(",
            1,
            "explicit raw-port ordinary If reference",
        ),
        (
            "builder.lower_if_form_with_condition_value_and_branch_lowerer(",
            1,
            "shared FastMem IfForm core",
        ),
        ("RawLegacyChildLoweringPortV1", 1, "one explicit raw child port"),
        ("drive_legacy_expression_v1(", 2, "condition and branch raw demands"),
        (
            "ensure_fastmem_owner_eq_condition(",
            1,
            "retired direct FastMem verification",
        ),
        (
            "builder.add_fastmem_branch_condition_fact(",
            1,
            "retired direct FastMem fact publication",
        ),
        ("legacy_branch_program(", 2, "retired then Program branch shells"),
        ("map(legacy_branch_program)", 2, "retired optional Program branch shells"),
    ):
        _require_count(reference, evidence, expected, label)
    _require_count(
        surface_reference,
        "metadata_ctx.set_current_span(statement.span())",
        1,
        "outer statement span publication",
    )
    _require_count(
        surface_reference,
        "emission::constant::emit_void(builder)",
        1,
        "successful facade Void publication",
    )

    for fixture in (
        "if_statement_parity_explicit_else_phis_and_child_expression_families",
        "if_statement_parity_implicit_else_and_termination_matrix",
        "if_statement_parity_condition_then_else_failures_and_reuse",
        "if_statement_parity_fastmem_positive_negative_and_reuse",
        "if_statement_parity_recursion_boundaries_restore_exact_state",
        "if_statement_parity_preserves_outer_and_branch_program_spans",
    ):
        if fixture not in parity_tests:
            _fail(f"missing IF0-P0 parity fixture: {fixture}")
    for evidence in (
        "IfStatementParitySnapshotV1",
        "instruction_spans",
        "terminator_span",
        "predecessors",
        "successors",
        "variable_map",
        "scope_frames",
        "if_merge_stack",
        "debug_scope_stack",
        "fastmem_region_stack",
        "fastmem_branch_condition_facts",
        "pending_phis",
        "debug_join_counter",
        "recursion_depth",
        "current_span",
        "snapshot(&selected, selected_result)",
        "snapshot(&reference, reference_result)",
    ):
        if evidence not in parity_tests:
            _fail(f"missing IF0-P0 exact-state evidence: {evidence}")
    if "statement_if(" in reference or "statement_if_at(" in reference:
        _fail("IF0-P0 reference must not select itself through a fixture helper")

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
    if production_driver_callers != 1:
        _fail(f"IF0-L0 generic driver callers: {production_driver_callers}")
    if raw_driver_callers != 1:
        _fail(f"IF0-I0 production raw driver callers: {raw_driver_callers}")

    for phrase in (
        "disconnected statement-If child-demand",
        "existing FastMem post-condition verification",
        "IfForm remains the sole block",
        "one callback invoked at the existing then",
        "One thin success-only completion seam",
        "source paths or caller ledger",
        "It owns no Match, Loop, suffix routing",
        "production raw If port preserves the retired branch Program shell",
        "IF0-I0 selects the raw driver exactly once",
    ):
        if phrase not in readme:
            _fail(f"missing IF0-S0 README boundary: {phrase}")

    touched = (
        driver_path,
        tests_path,
        raw_tests_path,
        parity_tests_path,
        stmts_root_path,
        block_stmt_path,
        exprs_path,
        located_if_path,
        control_flow_path,
        readme_path,
        if_form_path,
        phi_path,
        helper_path,
    )
    oversized = [relative for relative in touched if len(_read(root, relative).splitlines()) >= 800]
    if oversized:
        _fail(f"IF0-S0 source/check files reached 800 lines: {oversized}")

    l0_summary = check_if0_l0(root)
    return f"if_driver=1 if_branch_core=1 if_raw_selectors=1 {l0_summary}"
