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
    raw_tests_path = "src/mir/builder/stmts/variable_assignment_raw_tests.rs"
    parity_tests_path = "src/mir/builder/stmts/variable_assignment_parity_tests.rs"
    stmts_root_path = "src/mir/builder/stmts/mod.rs"
    selector_path = "src/mir/builder/exprs.rs"
    grouped_path = "src/mir/builder/builder_build.rs"
    located_path = "src/mir/builder/located_legacy_lowering.rs"
    located_adapter_path = "src/mir/builder/located_legacy_assignment.rs"
    located_tests_path = "src/mir/builder/located_legacy_assignment_tests.rs"
    readme_path = "src/mir/builder/stmts/README.md"
    helper_path = (
        "tools/checks/lib/"
        "callable_result_i0_site0_r0_expr0_spine0_stmt0_assignment.py"
    )

    module = _read(root, module_path)
    tests = _read(root, tests_path)
    raw_tests = _read(root, raw_tests_path)
    parity_tests = _read(root, parity_tests_path)
    stmts_root = _read(root, stmts_root_path)
    selector = _read(root, selector_path)
    grouped = _read(root, grouped_path)
    located = _read(root, located_path)
    located_adapter = _read(root, located_adapter_path)
    located_tests = _read(root, located_tests_path)
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
    if driver_callers != 1:
        _fail(f"ASN0-L0 generic external callers must be one: actual={driver_callers}")
    if raw_callers != 1:
        _fail(f"ASN0-I0 raw production callers must be one: actual={raw_callers}")

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
    if not re.search(
        r"#\[cfg\(test\)\]\s*mod variable_assignment_raw_tests;",
        stmts_root,
    ):
        _fail("ASN0-I0 raw fixture module must remain cfg(test)-scoped")
    if not re.search(
        r"#\[cfg\(test\)\]\s*mod variable_assignment_parity_tests;",
        stmts_root,
    ):
        _fail("ASN0-P0 parity fixture module must remain cfg(test)-scoped")
    for export in (
        "drive_raw_variable_assignment_v1",
        "drive_variable_assignment_v1",
        "VariableAssignmentDescentPortV1",
        "VariableAssignmentSyntaxViewV1",
    ):
        _require_count(stmts_root, export, 1, f"narrow Assignment export {export}")

    variable_branch = re.search(
        r"else if let ASTNode::Variable \{ name, \.\. \} = stmt\.target\.as_ref\(\) \{(?P<body>.*?)\n\s*\} else \{",
        selector,
        re.DOTALL,
    )
    if variable_branch is None:
        _fail("missing exact Variable-target selector branch")
    _require_count(
        variable_branch.group("body"),
        "drive_raw_variable_assignment_v1(",
        1,
        "exact Variable selector raw delegation",
    )
    for owner in ("build_field_assignment(", "build_index_assignment("):
        _require_count(selector, owner, 1, f"unchanged {owner} selector")
    _require_count(
        selector,
        "build_compound_assignment_statement(",
        1,
        "unchanged compound selector",
    )
    grouped_branch = re.search(
        r"ASTNode::GroupedAssignmentExpr \{ lhs, rhs, \.\. \} => \{(?P<body>.*?)\n\s*\}",
        selector,
        re.DOTALL,
    )
    if grouped_branch is None:
        _fail("missing grouped Assignment selector")
    _require_count(
        grouped_branch.group("body"),
        "build_grouped_assignment(",
        1,
        "parked grouped Assignment facade",
    )
    if "drive_raw_variable_assignment_v1(" in grouped_branch.group("body"):
        _fail("grouped Assignment must not select ASN0 raw descent")
    _require_count(
        grouped,
        "fn build_grouped_assignment(",
        1,
        "dedicated grouped Assignment facade",
    )
    grouped_body = grouped.split("fn build_grouped_assignment(", 1)[1].split(
        "fn build_assignment_from_value(", 1
    )[0]
    for legacy_step in (
        "AssignmentResolverBox::ensure_declared(self, &var_name)?;",
        "self.build_expression(value)?",
        "self.build_assignment_from_value(var_name, value_id)",
    ):
        if legacy_step not in grouped_body:
            _fail(f"grouped Assignment lost legacy orchestration: {legacy_step}")
    if "drive_raw_variable_assignment_v1(" in grouped_body:
        _fail("grouped Assignment facade must remain outside ASN0")

    _require_count(
        parity_tests,
        "fn lower_pre_i0_assignment_reference(",
        1,
        "test-only pre-I0 Assignment reference",
    )
    reference_body = parity_tests.split(
        "fn lower_pre_i0_assignment_reference(", 1
    )[1].split("fn snapshot(", 1)[0]
    for required in (
        "with_legacy_expression_recursion_guard_v1",
        "ASTNode::Assignment { target, value, .. }",
        "ASTNode::Variable { name, .. }",
        "builder.metadata_ctx.set_current_span(span);",
        "AssignmentResolverBox::ensure_declared(builder, &name)?;",
        "builder.build_expression(*value)?",
        "builder.build_assignment_from_value(name, value)",
    ):
        if required not in reference_body:
            _fail(f"ASN0-P0 reference lost pre-I0 step: {required}")
    span_at = reference_body.index("builder.metadata_ctx.set_current_span(span);")
    preflight_at = reference_body.index(
        "AssignmentResolverBox::ensure_declared(builder, &name)?;"
    )
    rhs_at = reference_body.index("builder.build_expression(*value)?")
    completion_at = reference_body.index("builder.build_assignment_from_value(name, value)")
    if not span_at < preflight_at < rhs_at < completion_at:
        _fail("ASN0-P0 reference order must be span -> preflight -> RHS -> completion")
    for forbidden in (
        "drive_variable_assignment_v1(",
        "drive_raw_variable_assignment_v1(",
        "VariableAssignmentDescentPortV1",
        "ASTNode::FieldAccess",
        "ASTNode::Index",
        "ASTNode::CompoundAssignment",
    ):
        if forbidden in reference_body:
            _fail(f"ASN0-P0 reference owns forbidden surface: {forbidden}")
    for path in (root / "src").rglob("*.rs"):
        if path.resolve() == (root / parity_tests_path).resolve():
            continue
        if "lower_pre_i0_assignment_reference(" in path.read_text(encoding="utf-8"):
            _fail(f"test-only Assignment reference escaped parity module: {path}")

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

    for fixture in (
        "raw_variable_assignment_selects_owned_descent_and_recursive_rhs",
        "raw_undeclared_target_rejects_before_rhs_effects",
        "raw_rhs_failure_keeps_old_binding_and_fresh_retry_succeeds",
        "field_target_stays_on_field_owner_before_rhs_descent",
        "grouped_assignment_remains_on_its_legacy_facade",
    ):
        if fixture not in raw_tests:
            _fail(f"missing ASN0-I0 raw fixture: {fixture}")

    for fixture in (
        "literal_binary_and_short_circuit_rhs_have_exact_pre_i0_snapshot_parity",
        "exact_local_contract_reassignment_has_exact_pre_i0_snapshot_parity",
        "typed_array_reassignment_has_exact_pre_i0_snapshot_parity",
        "undeclared_and_rhs_failures_plus_reuse_have_exact_pre_i0_snapshot_parity",
        "pre_i0_reference_rejects_grouped_assignment_before_effects",
    ):
        if fixture not in parity_tests:
            _fail(f"missing ASN0-P0 parity fixture: {fixture}")

    for needle, expected, label in (
        ("struct LocatedVariableAssignmentInputV1", 1, "located selected input"),
        ("fn select_exact_variable_assignment_v1", 1, "located exact selector"),
        ("fn lower_selected_variable_assignment_v1", 1, "located selected lowering"),
        (
            "impl<'plan> VariableAssignmentDescentPortV1 for LocatedLegacyLoweringSessionV1<'plan>",
            1,
            "located Assignment port",
        ),
        ("drive_variable_assignment_v1(builder, session, selected)", 1, "shared driver use"),
        ("ExprChildRoleV1::AssignmentValue", 1, "AssignmentValue navigation"),
        ("with_legacy_expression_recursion_guard_v1", 2, "import plus outer guard"),
    ):
        _require_count(located_adapter, needle, expected, label)
    for forbidden in (
        "SourceExprSiteV1",
        "SourcePath",
        "AssignmentTarget",
        "ASTNode::FieldAccess",
        "ASTNode::Index",
        "CompoundAssignment",
        "GroupedAssignment",
        "RowsUnderPrefix",
        ".claim(",
        ".ledger",
        "build_expression(",
        "build_expression_impl(",
        "fallback",
        "retry",
    ):
        if forbidden in located_adapter:
            _fail(f"ASN0-L0 located adapter owns forbidden authority: {forbidden}")
    _require_count(
        located,
        "assignment_adapter::select_exact_variable_assignment_v1(input)",
        1,
        "located Assignment selector consumer",
    )
    selector_at = located.index("assignment_adapter::select_exact_variable_assignment_v1(input)")
    inactive_at = located.index(".prove_stmt_inactive(&input)")
    if not selector_at < inactive_at:
        _fail("ASN0-L0 selector must precede whole-statement inactive proof")
    for fixture in (
        "located_assignment_claims_outer_rhs_before_nested_argument_and_completes_once",
        "located_assignment_reuses_binary_and_deferred_short_circuit_children",
        "wrong_assignment_order_has_no_rhs_effect_and_fresh_session_succeeds",
        "undeclared_target_and_rhs_failure_publish_no_assignment",
        "loop_body_assignment_path_seam_fails_closed_until_loop0",
        "non_variable_targets_and_active_loop_controls_fail_closed",
    ):
        if fixture not in located_tests:
            _fail(f"missing ASN0-L0 fixture: {fixture}")
    for snapshot_fact in (
        "blocks:",
        "value_types:",
        "value_kinds:",
        "value_origins:",
        "variable_map:",
        "bindings:",
        "scope_frames:",
        "local_slot_contracts:",
        "typed_array_contract_sources:",
        "slot_registry:",
        "local_ssa_map:",
        "schedule_mat_map:",
        "next_value_id:",
        "recursion_depth:",
    ):
        if snapshot_fact not in parity_tests:
            _fail(f"ASN0-P0 snapshot misses fact: {snapshot_fact}")

    for phrase in (
        "exact Variable-target",
        "field/index target syntax is structurally absent",
        "declared-binding preflight before requesting the",
        "existing `build_assignment_from_value` owner",
        "second completion-time declaration check is retained",
        "selects this raw driver exactly once",
        "`GroupedAssignmentExpr` remains",
        "ASN0-P0 retains the pre-I0 exact Variable orchestration",
        "reference rejects Grouped, field, index, and compound surfaces",
        "parity reference and located `AssignmentValue` navigation",
        "ASN0-L0 adds one disconnected located adapter",
        "derives the RHS only",
        "through the existing `AssignmentValue` role",
        "Field/index/compound and",
        "LoopBodyRoot",
        "Exact Loop body carriage remains LOOP0",
    ):
        if phrase not in readme:
            _fail(f"missing ASN0-S0 README boundary: {phrase}")

    touched = (
        module_path,
        tests_path,
        raw_tests_path,
        parity_tests_path,
        stmts_root_path,
        selector_path,
        grouped_path,
        located_path,
        located_adapter_path,
        located_tests_path,
        readme_path,
        helper_path,
    )
    oversized = [
        relative for relative in touched if len(_read(root, relative).splitlines()) >= 800
    ]
    if oversized:
        _fail(f"ASN0 source/check files reached 800 lines: {oversized}")
    if re.search(r"Arc<|Rc<|thread_local!|static mut", module):
        _fail("ASN0 substrate must remain stack-scoped and immutable")

    return "asn_driver=1 asn_e0_descents=1 asn_raw_impl=1 asn_raw_selectors=1 asn_parity_reference=1 asn_located_impl=1 asn_located_selectors=1 asn_grouped_selectors=0"
