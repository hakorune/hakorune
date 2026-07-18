#!/usr/bin/env python3
"""Private RET0-S0/I0/P0 structural checks for the public EXPR0-SPINE0 guard."""

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
    raw_tests_path = "src/mir/builder/stmts/return_statement_raw_tests.rs"
    parity_tests_path = "src/mir/builder/stmts/return_statement_parity_tests.rs"
    located_path = "src/mir/builder/located_legacy_return.rs"
    located_tests_path = "src/mir/builder/located_legacy_return_tests.rs"
    located_session_path = "src/mir/builder/located_legacy_lowering.rs"
    return_owner_path = "src/mir/builder/stmts/return_stmt.rs"
    expression_owner_path = "src/mir/builder/exprs.rs"
    stmts_root_path = "src/mir/builder/stmts/mod.rs"
    readme_path = "src/mir/builder/stmts/README.md"
    helper_path = (
        "tools/checks/lib/"
        "callable_result_i0_site0_r0_expr0_spine0_stmt0_return.py"
    )

    descent = _read(root, descent_path)
    tests = _read(root, tests_path)
    raw_tests = _read(root, raw_tests_path)
    parity_tests = _read(root, parity_tests_path)
    located = _read(root, located_path)
    located_tests = _read(root, located_tests_path)
    located_session = _read(root, located_session_path)
    return_owner = _read(root, return_owner_path)
    expression_owner = _read(root, expression_owner_path)
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
        "try_apply_match_return_optimization(builder, None, true)?",
        1,
        "legacy Void match observation preserved",
    )
    _require_count(
        return_owner,
        "builder.build_expression(*expr)?",
        0,
        "retired inline value lowering",
    )
    _require_count(
        return_owner,
        "super::return_statement_descent::drive_raw_value_return_statement_v1(",
        1,
        "existing Return facade raw value selector",
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

    facade_start = return_owner.index(
        "pub(in crate::mir::builder) fn build_return_statement("
    )
    facade = return_owner[facade_start:]
    _require_count(
        facade,
        "if let Some(expr) = value {",
        1,
        "exact value-bearing Return selector",
    )
    if not re.search(
        r"if let Some\(expr\) = value \{\s*"
        r"return super::return_statement_descent::"
        r"drive_raw_value_return_statement_v1\(\s*builder, \*expr,\s*\);\s*\}",
        facade,
    ):
        _fail("RET0-I0 Some selector must early-return through the raw driver")

    selector_at = facade.index("if let Some(expr) = value {")
    raw_at = facade.index(
        "return super::return_statement_descent::drive_raw_value_return_statement_v1("
    )
    cleanup_at = facade.index("ensure_return_allowed(builder)?;")
    void_match_at = facade.index(
        "try_apply_match_return_optimization(builder, None, true)?"
    )
    void_at = facade.index(
        "crate::mir::builder::emission::constant::emit_void(builder)?"
    )
    facade_completion_at = facade.index(
        "emit_return_from_value(builder, return_value)"
    )
    if not (
        selector_at
        < raw_at
        < cleanup_at
        < void_match_at
        < void_at
        < facade_completion_at
    ):
        _fail(
            "RET0-I0 order must be Some/raw early return before "
            "Void cleanup -> match(None) -> emit_void -> completion"
        )

    ignored = {
        (root / descent_path).resolve(),
        (root / tests_path).resolve(),
        (root / raw_tests_path).resolve(),
        (root / located_path).resolve(),
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
    if raw_callers != 1:
        _fail(f"RET0-I0 raw production selectors must be one: actual={raw_callers}")

    _require_count(
        parity_tests,
        "fn lower_pre_i0_return_reference(",
        1,
        "one cfg(test) pre-I0 Return reference",
    )
    reference_start = parity_tests.index("fn lower_pre_i0_return_reference(")
    reference_end = parity_tests.index("fn snapshot(", reference_start)
    reference = parity_tests[reference_start:reference_end]
    for required in (
        "let span = expression.span();",
        "let node_kind = std::mem::discriminant(&expression);",
        "ASTNode::Return { value, .. }",
        "with_legacy_expression_recursion_guard_v1(builder, node_kind, move |builder| {",
        "builder.metadata_ctx.set_current_span(span);",
        "ensure_return_allowed(builder)?;",
        "try_apply_match_return_optimization(builder, value.as_deref(), true)?",
        "builder.build_expression(*expr)?",
        "emit_void(builder)?",
        "emit_return_from_value(builder, return_value)",
    ):
        if required not in reference:
            _fail(f"missing RET0-P0 reference step: {required}")

    span_extract_at = reference.index("let span = expression.span();")
    node_kind_at = reference.index(
        "let node_kind = std::mem::discriminant(&expression);"
    )
    destructure_at = reference.index("ASTNode::Return { value, .. }")
    guard_at = reference.index(
        "with_legacy_expression_recursion_guard_v1(builder, node_kind, move |builder| {"
    )
    span_at = reference.index("builder.metadata_ctx.set_current_span(span);")
    cleanup_at = reference.index("ensure_return_allowed(builder)?;")
    match_at = reference.index(
        "try_apply_match_return_optimization(builder, value.as_deref(), true)?"
    )
    child_at = reference.index("builder.build_expression(*expr)?")
    void_at = reference.index("emit_void(builder)?")
    completion_at = reference.index("emit_return_from_value(builder, return_value)")
    prefix_order = (
        span_extract_at < node_kind_at < destructure_at < guard_at < span_at < cleanup_at
    )
    if not prefix_order or not cleanup_at < match_at < child_at < completion_at:
        _fail("RET0-P0 value reference order drifted")
    if not prefix_order or not cleanup_at < match_at < void_at < completion_at:
        _fail("RET0-P0 Void reference order drifted")

    for forbidden in (
        "build_return_statement(",
        "drive_value_return_statement_v1",
        "drive_raw_value_return_statement_v1",
        "Located",
        "CallableResult",
        "SourcePath",
        "ledger",
        "MatchReturnFacts",
        "compose_match_return",
        "PlanVerifier",
        "PlanLowerer",
        "retry",
        "fallback",
    ):
        if forbidden in reference:
            _fail(f"RET0-P0 reference owns forbidden authority: {forbidden}")

    parity_reference_callers = 0
    for path in (root / "src").rglob("*.rs"):
        if path.resolve() == (root / parity_tests_path).resolve():
            continue
        parity_reference_callers += path.read_text(encoding="utf-8").count(
            "lower_pre_i0_return_reference("
        )
    if parity_reference_callers != 0:
        _fail(
            "RET0-P0 reference production callers must be zero: "
            f"actual={parity_reference_callers}"
        )

    _require_count(
        located,
        "struct LocatedValueReturnInputV1",
        1,
        "one located value-Return carrier",
    )
    _require_count(
        located,
        "fn select_exact_value_return_v1",
        1,
        "one exact located value-Return selector",
    )
    _require_count(
        located,
        "impl<'plan> ReturnStatementDescentPortV1 for LocatedLegacyLoweringSessionV1<'plan>",
        1,
        "one located Return port implementation",
    )
    _require_count(
        located,
        "drive_value_return_statement_v1(builder, session, selected)",
        1,
        "one located Return driver consumer",
    )
    _require_count(
        located,
        "ExprChildRoleV1::ReturnValue",
        1,
        "one canonical ReturnValue role consumer",
    )
    _require_count(
        located,
        ".prove_expr_inactive(&expression)",
        1,
        "one located Match inactive-subtree proof",
    )
    _require_count(
        located,
        "try_apply_match_return_optimization(builder, Some(value), true)",
        1,
        "one existing Match owner delegation",
    )

    carrier_start = located.index("struct LocatedValueReturnInputV1")
    selector_start = located.index("fn select_exact_value_return_v1", carrier_start)
    if "Option" in located[carrier_start:selector_start]:
        _fail("RET0-L0 carrier must make one Return value structurally mandatory")
    selector_end = located.index("fn lower_selected_value_return_v1", selector_start)
    selector = located[selector_start:selector_end]
    if not re.search(
        r"ASTNode::Return\s*\{\s*value: Some\(value\), \.\.\s*\}",
        selector,
    ):
        _fail("RET0-L0 selector must admit exact Return Some only")

    match_start = located.index("fn try_match_return_optimization(")
    child_start = located.index("fn return_value_expression_input(", match_start)
    match_hook = located[match_start:child_start]
    match_shape_at = match_hook.index("matches!(value, ASTNode::MatchExpr { .. })")
    location_at = match_hook.index("self.return_value_expression_input(input)?")
    inactive_at = match_hook.index(".prove_expr_inactive(&expression)")
    match_owner_at = match_hook.index(
        "try_apply_match_return_optimization(builder, Some(value), true)"
    )
    if not match_shape_at < location_at < inactive_at < match_owner_at:
        _fail(
            "RET0-L0 Match order must be shape -> exact location -> "
            "inactive proof -> existing owner"
        )

    for forbidden in (
        "SourceExprSiteV1",
        "SourcePathSegmentV1",
        "ledger.claim",
        ".claim(",
        "MirInstruction::Return",
        "drive_raw_value_return_statement_v1",
        "builder.build_expression(",
        "emit_return_from_value",
        "emit_void",
        "RowsUnderPrefix",
    ):
        if forbidden in located:
            _fail(f"RET0-L0 adapter owns forbidden authority: {forbidden}")

    _require_count(
        located_session,
        "return_adapter::select_exact_value_return_v1(input)",
        1,
        "one located Return statement selector",
    )
    _require_count(
        located_session,
        "return_adapter::lower_selected_value_return_v1(",
        1,
        "one located Return selector consumer",
    )
    if not re.search(
        r"#\[cfg\(test\)\]\s*#\[path = \"located_legacy_return_tests.rs\"\]\s*"
        r"mod return_tests;",
        located_session,
    ):
        _fail("RET0-L0 fixture module must remain cfg(test)-scoped")

    production_session_callers = 0
    for path in (root / "src").rglob("*.rs"):
        relative = path.relative_to(root).as_posix()
        if relative.endswith("_tests.rs") or "/tests/" in relative:
            continue
        production_session_callers += path.read_text(encoding="utf-8").count(
            "LocatedLegacyLoweringSessionV1::verify("
        )
    if production_session_callers != 0:
        _fail(
            "RET0-L0 located production session callers must be zero: "
            f"actual={production_session_callers}"
        )

    _require_count(
        expression_owner,
        "super::stmts::return_stmt::build_return_statement(self, stmt.value.clone())",
        1,
        "unchanged expression-to-Return facade entry",
    )
    if "drive_raw_value_return_statement_v1(" in expression_owner:
        _fail("RET0-I0 expression dispatch must not become a second Return policy owner")

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
    if not re.search(
        r"#\[cfg\(test\)\]\s*mod return_statement_raw_tests;",
        stmts_root,
    ):
        _fail("RET0-I0 raw fixture module must remain cfg(test)-scoped")
    if not re.search(
        r"#\[cfg\(test\)\]\s*mod return_statement_parity_tests;",
        stmts_root,
    ):
        _fail("RET0-P0 parity module must remain cfg(test)-scoped")

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

    for fixture in (
        "raw_value_return_selects_owned_descent_for_actual_method_call",
        "raw_void_return_stays_on_legacy_facade",
        "raw_match_return_keeps_existing_selection_owner_without_second_completion",
        "raw_configured_defer_keeps_exact_copy_jump_completion",
        "raw_cleanup_and_child_failures_leave_no_terminator_then_reuse",
    ):
        if fixture not in raw_tests:
            _fail(f"missing RET0-I0 raw fixture: {fixture}")

    for fixture in (
        "literal_binary_short_circuit_and_method_call_have_exact_pre_i0_parity",
        "void_return_has_exact_pre_i0_parity",
        "selected_match_return_has_exact_pre_i0_parity",
        "configured_defer_has_exact_pre_i0_parity",
        "cleanup_and_child_failures_plus_same_builder_reuse_have_exact_pre_i0_parity",
    ):
        if fixture not in parity_tests:
            _fail(f"missing RET0-P0 parity fixture: {fixture}")

    for snapshot_field in (
        "result:",
        "blocks:",
        "locals:",
        "value_types:",
        "value_kinds:",
        "value_origins:",
        "string_literals:",
        "exact_numeric_const_facts:",
        "exact_numeric_value_facts:",
        "variable_map:",
        "bindings:",
        "scope_frames:",
        "pin_slots:",
        "local_ssa_map:",
        "schedule_mat_map:",
        "current_block:",
        "next_value_id:",
        "next_core_value:",
        "next_core_block:",
        "next_binding_id:",
        "temp_slot_counter:",
        "recursion_depth:",
        "current_span:",
        "in_cleanup_block:",
        "cleanup_allow_return:",
        "return_defer_active:",
        "return_defer_slot:",
        "return_defer_target:",
        "return_deferred_emitted:",
    ):
        if snapshot_field not in parity_tests:
            _fail(f"missing RET0-P0 snapshot surface: {snapshot_field}")

    for fixture in (
        "located_return_claims_actual_body_value_last_in_exact_order",
        "located_return_claims_nested_argument_before_parent",
        "located_return_reuses_binary_and_deferred_short_circuit_spines",
        "located_return_wrong_order_poisons_without_call_or_completion",
        "located_return_cleanup_and_child_failures_require_fresh_sessions",
        "located_return_selector_excludes_void_and_non_return_statements",
    ):
        if fixture not in located_tests:
            _fail(f"missing RET0-L0 located fixture: {fixture}")

    actual_fixture_at = located_tests.index(
        "fn located_return_claims_actual_body_value_last_in_exact_order()"
    )
    actual_fixture_end = located_tests.index("#[test]", actual_fixture_at)
    actual_fixture = located_tests[actual_fixture_at:actual_fixture_end]
    _require_count(
        actual_fixture,
        "SourcePathSegmentV1::Body(5)",
        1,
        "actual final Return Body(5) row",
    )
    _require_count(
        actual_fixture,
        "SourcePathSegmentV1::Value",
        1,
        "actual final Return Value row",
    )
    _require_count(
        actual_fixture,
        "SourcePathSegmentV1::Argument(",
        0,
        "actual final Return has no nested call row",
    )

    for required in (
        "block.terminator.clone()",
        "SourcePathSegmentV1::Body(5)",
        "SourcePathSegmentV1::Value",
        "WrongOrder",
        "LocatedLegacyLoweringErrorV1::Poisoned",
        "builder.recursion_depth",
    ):
        if required not in located_tests:
            _fail(f"missing RET0-L0 fixture evidence: {required}")

    normalized_readme = " ".join(readme.split())
    for phrase in (
        "one disconnected orchestration boundary for",
        "runs the existing cleanup prohibition first",
        "delegates the existing match-return probe",
        "requests `ReturnValue` once",
        "existing `emit_return_from_value` owner",
        "does not admit `return;`",
        "must not reconstruct",
        "selects that driver exactly once inside the existing Return facade",
        "keeps the `None` branch on the legacy Void path",
        "one `cfg(test)` pre-I0 Return orchestration reference",
        "exact normalized parity",
        "has no production caller",
        "one disconnected exact `Return { value: Some(_) }` adapter",
        "existing `ReturnValue` source role",
        "active row below Match fails closed",
        "Void Return stays outside the adapter",
    ):
        if phrase not in normalized_readme:
            _fail(f"missing RET0-S0 README boundary: {phrase}")

    touched = (
        descent_path,
        tests_path,
        raw_tests_path,
        parity_tests_path,
        located_path,
        located_tests_path,
        located_session_path,
        return_owner_path,
        expression_owner_path,
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

    return (
        "ret_driver=1 ret_e0_descents=1 ret_match_hook=1 "
        "ret_raw_selectors=1 ret_parity_reference=1 "
        "ret_located_impl=1 ret_located_selectors=1"
    )
