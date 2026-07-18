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
    local_raw_tests_path = "src/mir/builder/stmts/local_statement_raw_tests.rs"
    local_parity_tests_path = (
        "src/mir/builder/stmts/local_statement_parity_tests.rs"
    )
    located_local_tests_path = (
        "src/mir/builder/located_legacy_local_tests.rs"
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
    local_raw_tests = _read(root, local_raw_tests_path)
    local_parity_tests = _read(root, local_parity_tests_path)
    located_local_tests = _read(root, located_local_tests_path)
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
    _require_count(
        local_descent,
        "fn local_syntax",
        2,
        "Local syntax query declaration plus raw implementation",
    )
    _require_count(
        local_descent,
        "fn local_initializer_expression_input",
        2,
        "ordinary initializer input declaration plus raw implementation",
    )
    _require_count(
        local_descent,
        "fn lower_typed_array_literal_initializer",
        2,
        "typed-array hook declaration plus raw implementation",
    )
    _require_count(
        local_descent,
        "fn lower_record_constructor_initializer",
        2,
        "record hook declaration plus raw implementation",
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

    parity_resolved = (root / local_parity_tests_path).resolve()
    production_rust_source = "\n".join(
        path.read_text(encoding="utf-8")
        for path in (root / "src").rglob("*.rs")
        if path.resolve() != parity_resolved
    )
    normalized_rust_source = re.sub(r"\s+", " ", production_rust_source)
    _require_count(
        normalized_rust_source,
        "impl LocalStatementDescentPortV1 for RawLegacyChildLoweringPortV1",
        1,
        "LCL0-I0 raw implementation",
    )
    _require_count(
        normalized_rust_source,
        "observe_preflighted_local_statement(",
        2,
        "debug observation declaration plus sole post-preflight consumer",
    )
    _require_count(
        located,
        "impl<'plan> LocalStatementDescentPortV1 for LocatedLegacyLoweringSessionV1<'plan>",
        1,
        "LCL0-L0 located implementation",
    )
    _require_count(
        located,
        "if matches!(input.node(), ASTNode::Local { .. })",
        1,
        "LCL0-L0 syntax selector",
    )
    _require_count(
        located,
        "ExprChildRoleV1::LocalInitializer(index)",
        1,
        "LCL0-L0 exact initializer source role",
    )
    _require_count(
        located,
        "fn prove_local_initializer_inactive(",
        1,
        "LCL0-L0 specialized inactive-proof owner",
    )
    _require_count(
        located,
        "self.prove_local_initializer_inactive(input, index)?;",
        2,
        "LCL0-L0 specialized hook proof consumers",
    )
    proof_at = located.index("fn prove_local_initializer_inactive(")
    inactive_at = located.index(".prove_expr_inactive(&expression)", proof_at)
    typed_hook_at = located.index("fn lower_typed_array_literal_initializer(", proof_at)
    typed_proof_at = located.index(
        "self.prove_local_initializer_inactive(input, index)?;", typed_hook_at
    )
    typed_effect_at = located.index(
        "builder.build_typed_array_literal(elements.to_vec())", typed_hook_at
    )
    record_hook_at = located.index("fn lower_record_constructor_initializer(", typed_hook_at)
    record_proof_at = located.index(
        "self.prove_local_initializer_inactive(input, index)?;", record_hook_at
    )
    record_effect_at = located.index(
        "builder.build_record_constructor_value(class.to_string(), arguments.to_vec())",
        record_hook_at,
    )
    if not (
        proof_at < inactive_at
        and typed_hook_at < typed_proof_at < typed_effect_at
        and record_hook_at < record_proof_at < record_effect_at
    ):
        _fail("LCL0-L0 inactive proof must precede every specialized effect")
    for forbidden in (
        "RowsUnderPrefix {",
        "value_origin_newbox.insert",
        "current_static_box",
        "retry",
        "fallback",
    ):
        if forbidden in located:
            _fail(f"LCL0-L0 located adapter owns forbidden authority: {forbidden}")

    local_driver_callers = 0
    local_ignored = {
        (root / local_descent_path).resolve(),
        (root / local_descent_tests_path).resolve(),
        (root / local_raw_tests_path).resolve(),
        parity_resolved,
        (root / located_local_tests_path).resolve(),
    }
    for path in (root / "src").rglob("*.rs"):
        if path.resolve() in local_ignored:
            continue
        local_driver_callers += path.read_text(encoding="utf-8").count(
            "drive_local_statement_v1("
        )
    if local_driver_callers != 1:
        _fail(
            "LCL0-L0 generic driver must have one located caller outside raw/test owners: "
            f"actual={local_driver_callers}"
        )

    _require_count(
        local_descent,
        "struct RawLegacyLocalInputV1",
        1,
        "owned raw Local input",
    )
    _require_count(
        local_descent,
        "fn drive_raw_local_statement_v1",
        1,
        "thin raw Local facade",
    )
    _require_count(
        local_descent,
        "builder.build_typed_array_literal(elements.to_vec())",
        1,
        "existing typed-array owner reuse",
    )
    _require_count(
        local_descent,
        "builder.build_record_constructor_value(class.to_string(), arguments.to_vec())",
        1,
        "existing record-constructor owner reuse",
    )

    raw_selector_callers = 0
    for path in (root / "src").rglob("*.rs"):
        if path.resolve() in local_ignored:
            continue
        raw_selector_callers += path.read_text(encoding="utf-8").count(
            "drive_raw_local_statement_v1("
        )
    if raw_selector_callers != 1:
        _fail(
            "LCL0-I0 raw production selectors: "
            f"expected=1 actual={raw_selector_callers}"
        )
    _require_count(
        variable_stmt,
        "super::local_statement_descent::drive_raw_local_statement_v1(",
        1,
        "existing build_local_statement facade selector",
    )
    for retired in (
        "for (i, _var_name) in variables.iter().enumerate()",
        "builder.build_typed_array_literal(elements.clone())",
        "builder.build_record_constructor_value(class.clone(), arguments.clone())",
        "builder.build_expression(*init_expr.clone())",
    ):
        if retired in variable_stmt:
            _fail(f"LCL0-I0 old raw orchestration remains selected: {retired}")

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
        "drive_local_statement_v1, LocalStatementDescentPortV1, LocalStatementSyntaxViewV1,",
        1,
        "private located Local adapter facade",
    )
    _require_count(
        stmts_root,
        "mod local_statement_descent_tests;",
        1,
        "focused Local descent fixture module",
    )
    _require_count(
        stmts_root,
        "mod local_statement_raw_tests;",
        1,
        "focused raw Local fixture module",
    )
    _require_count(
        stmts_root,
        "mod local_statement_parity_tests;",
        1,
        "focused Local parity fixture module",
    )
    if not re.search(
        r"#\[cfg\(test\)\]\s*mod local_statement_parity_tests;",
        stmts_root,
    ):
        _fail("LCL0-P0 parity module must remain cfg(test)-scoped")
    if not re.search(
        r'#\[cfg\(test\)\]\s*#\[path = "located_legacy_local_tests\.rs"\]\s*mod local_tests;',
        located,
    ):
        _fail("LCL0-L0 located fixture module must remain exactly cfg(test)-scoped")

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
    _require_count(
        local_descent_tests,
        "EventV1::Input(1)",
        1,
        "generic Local second-initializer associated-input fixture",
    )
    if "canonical source grammar currently admits at most one initialized binding" not in stmts_readme:
        _fail("missing LCL0-L0 source-cardinality boundary in statement README")

    for fixture in (
        "raw_local_selector_preserves_initializer_order_and_binding_completion",
        "raw_local_preflight_rejects_before_first_initializer_effect",
        "raw_local_child_failure_stops_later_initializer_and_binding_publication",
        "raw_local_initializers_reuse_binary_and_short_circuit_spines",
        "raw_local_typed_array_reuses_specialized_claim_before_appends",
        "raw_local_record_initializer_reuses_existing_constructor_owner",
        "raw_local_untyped_missing_initializer_keeps_existing_null_sugar",
    ):
        if fixture not in local_raw_tests:
            _fail(f"missing LCL0-I0 fixture: {fixture}")

    _require_count(
        local_parity_tests,
        "fn lower_pre_i0_local_reference(",
        1,
        "cfg(test) pre-I0 Local reference",
    )
    _require_count(
        local_parity_tests,
        "fn build_pre_i0_local_reference(",
        1,
        "cfg(test) pre-I0 Local orchestration",
    )
    for owner_call, label in (
        ("preflight_exact_numeric_local_initializers(", "whole preflight"),
        ("observe_preflighted_local_statement(", "debug observation"),
        ("builder.build_typed_array_literal(elements.to_vec())", "typed-array owner"),
        (
            "builder.build_record_constructor_value(class.to_string(), arguments.to_vec())",
            "record owner",
        ),
        ("Some(initializer) => builder.build_expression(initializer.clone())?", "ordinary child"),
        ("crate::mir::builder::emission::constant::emit_null(builder)?", "Null sugar"),
        (
            "build_local_statement_from_values_with_types_and_preclaims(",
            "from-values completion",
        ),
    ):
        _require_count(local_parity_tests, owner_call, 1, f"LCL0-P0 {label}")
    for forbidden in (
        "drive_local_statement_v1(",
        "drive_raw_local_statement_v1(",
        "LocalStatementDescentPortV1",
        "RawLegacyChildLoweringPortV1",
    ):
        if forbidden in local_parity_tests:
            _fail(f"LCL0-P0 reference must not reuse selected descent: {forbidden}")
    for fixture in (
        "ordinary_exact_numeric_and_null_locals_have_exact_pre_i0_snapshot_parity",
        "typed_array_local_has_exact_pre_i0_snapshot_parity",
        "record_constructor_local_has_exact_pre_i0_snapshot_parity",
        "binary_and_short_circuit_initializers_have_exact_pre_i0_snapshot_parity",
        "preflight_and_child_failures_plus_reuse_have_exact_pre_i0_snapshot_parity",
        "specialized_and_completion_failures_plus_reuse_have_exact_pre_i0_snapshot_parity",
    ):
        if fixture not in local_parity_tests:
            _fail(f"missing LCL0-P0 fixture: {fixture}")
    for fixture in (
        "located_local_claims_exact_initializers_in_statement_and_expression_order",
        "located_local_short_circuit_keeps_deferred_rhs_site_and_completion",
        "located_local_special_hooks_require_exact_inactive_initializer_subtrees",
        "active_row_below_typed_array_hook_rejects_before_builder_effects_and_poisons_session",
        "active_row_below_record_hook_rejects_before_constructor_effects",
        "wrong_statement_order_fails_before_local_initializer_or_binding_effects",
    ):
        if fixture not in located_local_tests:
            _fail(f"missing LCL0-L0 fixture: {fixture}")
    for snapshot_fact in (
        "blocks:",
        "value_types:",
        "value_kinds:",
        "value_origins:",
        "string_literals:",
        "map_value_types:",
        "map_literal_value_types:",
        "variable_map:",
        "bindings:",
        "scope_frames:",
        "pin_slots:",
        "local_slot_contracts:",
        "local_identity_evidence:",
        "record_value_contracts:",
        "record_local_values:",
        "array_element_write_witnesses:",
        "array_state_terms:",
        "typed_array_contract_sources:",
        "typed_array_element_contracts:",
        "exact_numeric_const_facts:",
        "exact_numeric_value_facts:",
        "slot_registry:",
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
    ):
        if snapshot_fact not in local_parity_tests:
            _fail(f"missing LCL0-P0 exact snapshot fact: {snapshot_fact}")
    for path in (root / "src").rglob("*.rs"):
        if path.resolve() == parity_resolved:
            continue
        if "lower_pre_i0_local_reference" in path.read_text(encoding="utf-8"):
            _fail(
                "LCL0-P0 pre-I0 reference escaped cfg(test) parity module: "
                f"{path.relative_to(root)}"
            )

    for phrase in (
        "Local declaration preflight",
        "existing whole-declaration exact-numeric/typed-array preflight",
        "request ordinary initializer expressions in declaration order",
        "prove the exact `LocalInitializer(index)` subtree",
        "must not reconstruct source sites",
        "LCL0-I0 selects the owned raw Local input",
        "LCL0-P0 keeps one `cfg(test)` pre-I0 orchestration reference",
        "LCL0-L0 adds one disconnected located Local selector",
        "exact inactive-subtree proof before specialized effects",
    ):
        if phrase not in stmts_readme:
            _fail(f"missing LCL0 README boundary: {phrase}")

    touched = (
        local_descent_path,
        local_descent_tests_path,
        local_raw_tests_path,
        local_parity_tests_path,
        located_local_tests_path,
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

    return (
        "lcl_driver=1 lcl_e0_descents=1 lcl_raw_impl=1 "
        "lcl_raw_selector=1 lcl_parity_reference=1 lcl_located_impl=1 "
        "lcl_located_selector=1 lcl_special_inactive_proof=1"
    )
