#!/usr/bin/env python3
"""Private Return source-partition checks for the public EXPR0-SPINE0 guard."""

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


def _require_order(text: str, needles: tuple[str, ...], label: str) -> None:
    positions = [text.index(needle) for needle in needles]
    if positions != sorted(positions) or len(set(positions)) != len(positions):
        _fail(f"{label} order drifted")


def _production_calls(
    root: Path,
    symbol: str,
    *,
    excluded: set[str],
) -> int:
    pattern = re.compile(rf"\b{re.escape(symbol)}\s*\(")
    count = 0
    for path in (root / "src").rglob("*.rs"):
        relative = path.relative_to(root).as_posix()
        if (
            relative in excluded
            or relative.endswith("_tests.rs")
            or "/tests/" in relative
        ):
            continue
        count += len(pattern.findall(path.read_text(encoding="utf-8")))
    return count


def check_ret0_s0(root: Path) -> str:
    descent_path = "src/mir/builder/stmts/return_statement_descent.rs"
    tests_path = "src/mir/builder/stmts/return_statement_descent_tests.rs"
    raw_tests_path = "src/mir/builder/stmts/return_statement_raw_tests.rs"
    parity_tests_path = "src/mir/builder/stmts/return_statement_parity_tests.rs"
    surface_path = "src/mir/builder/raw_expression_dispatch/statement_surface.rs"
    located_path = "src/mir/builder/located_legacy_return.rs"
    located_tests_path = "src/mir/builder/located_legacy_return_tests.rs"
    located_session_path = "src/mir/builder/located_legacy_lowering.rs"
    owner_path = "src/mir/builder/stmts/return_stmt.rs"
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
    surface = _read(root, surface_path)
    located = _read(root, located_path)
    located_tests = _read(root, located_tests_path)
    located_session = _read(root, located_session_path)
    owner = _read(root, owner_path)
    stmts_root = _read(root, stmts_root_path)
    readme = _read(root, readme_path)

    # The value owner has one mandatory source value and one ordered child demand.
    for needle, expected, label in (
        ("trait ReturnStatementDescentPortV1", 1, "Return port owner"),
        ("type ReturnInput;", 1, "associated Return input"),
        ("fn return_value_syntax", 2, "syntax port"),
        ("fn try_match_return_optimization", 2, "Match port"),
        ("fn return_value_expression_input", 2, "child-input port"),
        ("fn drive_value_return_statement_v1", 1, "value Return driver"),
        ("ensure_return_allowed(builder)?;", 1, "value cleanup preflight"),
        (
            "try_apply_match_return_optimization(builder, Some(value), true)",
            1,
            "existing Match owner delegation",
        ),
        (
            "drive_legacy_expression_v1(builder, port, expression_input)?",
            1,
            "one value child descent",
        ),
        (
            "emit_return_from_value(builder, return_value)",
            1,
            "shared completion",
        ),
    ):
        _require_count(descent, needle, expected, label)

    _require_order(
        descent,
        (
            "ensure_return_allowed(builder)?;",
            "port.return_value_syntax(input)?",
            "port.try_match_return_optimization(builder, input, value)?",
            "port.return_value_expression_input(input)?",
            "drive_legacy_expression_v1(builder, port, expression_input)?",
            "emit_return_from_value(builder, return_value)",
        ),
        "value Return cleanup/syntax/Match/input/child/completion",
    )

    raw_input = descent[
        descent.index("struct RawLegacyValueReturnInputV1") :
        descent.index("struct ReturnStatementSyntaxViewV1")
    ]
    syntax_view = descent[
        descent.index("struct ReturnStatementSyntaxViewV1") :
        descent.index("trait ReturnStatementDescentPortV1")
    ]
    if "Option" in raw_input or "Option" in syntax_view:
        _fail("value Return input and syntax must be structurally mandatory")

    for forbidden in (
        "drive_raw_value_return_statement_v1",
        "RawLegacyChildLoweringPortV1",
        "MirInstruction::Return",
        "build_expression(",
        "retry",
        "fallback",
    ):
        if forbidden in descent:
            _fail(f"value Return owner gained forbidden authority: {forbidden}")

    # The raw/default statement surface is the sole Some/None selector.
    _require_count(
        surface,
        "node @ ASTNode::Return { .. }",
        1,
        "AST Return selector",
    )
    _require_count(
        surface,
        "RawLegacyValueReturnInputV1::new(*value)",
        1,
        "raw value input",
    )
    _require_count(
        surface,
        "drive_value_return_statement_v1(builder, port, &input)",
        1,
        "raw value owner caller",
    )
    _require_count(
        surface,
        "build_void_return_statement(builder)",
        1,
        "raw Void owner caller",
    )
    return_surface = surface[surface.index("fn build_return_with_port_v1") :]
    _require_order(
        return_surface,
        (
            "Some(value) =>",
            "RawLegacyValueReturnInputV1::new(*value)",
            "drive_value_return_statement_v1(builder, port, &input)",
            "None =>",
            "build_void_return_statement(builder)",
        ),
        "raw/default Return source partition",
    )

    # The exact Void leaf owns no AST, Option, child, or Match observation.
    _require_count(owner, "fn ensure_return_allowed", 1, "cleanup owner")
    _require_count(owner, "fn build_void_return_statement", 1, "Void owner")
    void_owner = owner[owner.index("fn build_void_return_statement") :]
    _require_order(
        void_owner,
        (
            "ensure_return_allowed(builder)?;",
            "emission::constant::emit_void(builder)?",
            "emit_return_from_value(builder, return_value)",
        ),
        "Void cleanup/emission/completion",
    )
    for forbidden in (
        "Option<",
        "ASTNode",
        "try_apply_match_return_optimization",
        "drive_value_return_statement_v1(",
        "build_expression(",
    ):
        if forbidden in void_owner:
            _fail(f"exact Void owner gained forbidden input/authority: {forbidden}")

    all_src = "\n".join(
        path.read_text(encoding="utf-8") for path in (root / "src").rglob("*.rs")
    )
    for retired in (
        "build_return_statement(",
        "drive_raw_value_return_statement_v1(",
        "try_apply_match_return_optimization(builder, None",
    ):
        if retired in all_src:
            _fail(f"retired Return authority remains: {retired}")

    external_driver_calls = _production_calls(
        root,
        "drive_value_return_statement_v1",
        excluded={descent_path},
    )
    if external_driver_calls != 2:
        _fail(
            "value Return driver must have raw/default plus detached callers: "
            f"actual={external_driver_calls}"
        )

    # The detached caller remains exact-value-only and root-inactive.
    for needle, expected, label in (
        ("struct LocatedValueReturnInputV1", 1, "located carrier"),
        ("fn select_exact_value_return_v1", 1, "located selector"),
        (
            "impl<'plan> ReturnStatementDescentPortV1 for "
            "LocatedLegacyLoweringSessionV1<'plan>",
            1,
            "located port",
        ),
        (
            "drive_value_return_statement_v1(builder, session, selected)",
            1,
            "located driver caller",
        ),
        ("ExprChildRoleV1::ReturnValue", 1, "ReturnValue role"),
        (".prove_expr_inactive(&expression)", 1, "located Match inactive proof"),
    ):
        _require_count(located, needle, expected, label)
    selector = located[
        located.index("fn select_exact_value_return_v1") :
        located.index("fn lower_selected_value_return_v1")
    ]
    if not re.search(
        r"ASTNode::Return\s*\{\s*value: Some\(value\), \.\.\s*\}",
        selector,
    ):
        _fail("located selector must admit exact Return Some only")
    if _production_calls(
        root,
        "LocatedLegacyLoweringSessionV1::verify",
        excluded=set(),
    ):
        _fail("located Return session gained production root ingress")

    # Historical parity remains test-only and preserves both source shapes.
    _require_count(
        parity_tests,
        "fn lower_pre_i0_return_reference(",
        1,
        "one historical Return reference",
    )
    reference = parity_tests[
        parity_tests.index("fn lower_pre_i0_return_reference(") :
        parity_tests.index("fn snapshot(")
    ]
    for required in (
        "ASTNode::Return { value, .. }",
        "ensure_return_allowed(builder)?;",
        "try_apply_match_return_optimization(builder, value.as_deref(), true)?",
        "drive_raw_legacy_expression_v1(builder, *expr)?",
        "emit_void(builder)?",
        "emit_return_from_value(builder, return_value)",
    ):
        if required not in reference:
            _fail(f"historical Return parity step missing: {required}")
    if _production_calls(
        root,
        "lower_pre_i0_return_reference",
        excluded={parity_tests_path},
    ):
        _fail("historical Return reference gained a production caller")
    selected = parity_tests[
        parity_tests.index("fn lower_selected(") :
        parity_tests.index("fn lower_pre_i0_return_reference(")
    ]
    _require_count(
        selected,
        "drive_value_return_statement_v1(builder, &mut port, &input)",
        1,
        "selected value Return owner",
    )
    _require_count(
        selected,
        "build_void_return_statement(builder)",
        1,
        "selected void Return owner",
    )
    _require_count(
        reference,
        "drive_raw_legacy_expression_v1(builder, *expr)?",
        1,
        "historical Return raw child oracle",
    )
    _require_count(
        raw_tests,
        "drive_raw_legacy_expression_v1(",
        9,
        "raw Return oracle calls",
    )
    for forbidden in (
        "drive_value_return_statement_v1(",
        "RawLegacyValueReturnInputV1",
        "RawLegacyChildLoweringPortV1",
    ):
        if forbidden in reference:
            _fail(f"historical Return reference reused selected owner: {forbidden}")
    for source, label in (
        (raw_tests, "raw Return tests"),
        (parity_tests, "Return parity tests"),
    ):
        if ".build_expression(" in source:
            _fail(f"{label} retained retired test facade")

    for fixture in (
        "cleanup_precedes_match_child_and_return_effects",
        "ordinary_value_probes_then_descends_once_and_completes_once",
        "selected_match_bypasses_value_demand_and_ordinary_completion",
        "syntax_match_input_and_child_failures_emit_no_return_completion",
        "configured_defer_reuses_copy_and_jump_completion_without_direct_return",
        "value_return_input_excludes_void",
    ):
        if fixture not in tests:
            _fail(f"missing Return owner fixture: {fixture}")
    for retired_fixture in (
        "raw_value_return_reuses_binary_and_short_circuit_child_spines",
        "raw_value_return_reuses_actual_method_call_child_spine",
        "legacy_void_return_remains",
    ):
        if retired_fixture in tests:
            _fail(f"retired facade-only fixture remains: {retired_fixture}")

    for fixture in (
        "raw_value_return_selects_owned_descent_for_actual_method_call",
        "raw_void_return_selects_void_source_partition",
        "raw_match_return_keeps_existing_selection_owner_without_second_completion",
        "raw_configured_defer_keeps_exact_copy_jump_completion",
        "raw_cleanup_and_child_failures_leave_no_terminator_then_reuse",
    ):
        if fixture not in raw_tests:
            _fail(f"missing raw Return ingress fixture: {fixture}")
    for fixture in (
        "literal_binary_short_circuit_and_method_call_have_exact_pre_i0_parity",
        "void_return_has_exact_pre_i0_parity",
        "selected_match_return_has_exact_pre_i0_parity",
        "configured_defer_has_exact_pre_i0_parity",
        "cleanup_and_child_failures_plus_same_builder_reuse_have_exact_pre_i0_parity",
    ):
        if fixture not in parity_tests:
            _fail(f"missing Return parity fixture: {fixture}")
    for fixture in (
        "located_return_claims_actual_body_value_last_in_exact_order",
        "located_return_claims_nested_argument_before_parent",
        "located_return_reuses_binary_and_deferred_short_circuit_spines",
        "located_return_wrong_order_poisons_without_call_or_completion",
        "located_return_cleanup_and_child_failures_require_fresh_sessions",
        "located_return_selector_excludes_void_and_non_return_statements",
    ):
        if fixture not in located_tests:
            _fail(f"missing located Return fixture: {fixture}")

    for module in (
        "return_statement_descent_tests",
        "return_statement_raw_tests",
        "return_statement_parity_tests",
    ):
        if not re.search(rf"#\[cfg\(test\)\]\s*mod {module};", stmts_root):
            _fail(f"Return fixture module must remain cfg(test)-scoped: {module}")
    if not re.search(
        r"#\[cfg\(test\)\]\s*#\[path = \"located_legacy_return_tests.rs\"\]\s*"
        r"mod return_tests;",
        located_session,
    ):
        _fail("located Return fixtures must remain cfg(test)-scoped")

    normalized_readme = " ".join(readme.split())
    for phrase in (
        "live associated-input owner",
        "one exact source partition",
        "`Some` constructs one mandatory",
        "`None` calls `build_void_return_statement`",
        "Match observation is value-bearing only",
        "old mixed Return facade and raw value facade are retired",
        "one `cfg(test)` pre-I0 Return orchestration reference",
        "disconnected exact `Return { value: Some(_) }` adapter",
        "production located root",
    ):
        if phrase not in normalized_readme:
            _fail(f"missing Return README boundary: {phrase}")

    touched = (
        descent_path,
        tests_path,
        raw_tests_path,
        parity_tests_path,
        surface_path,
        located_path,
        located_tests_path,
        located_session_path,
        owner_path,
        stmts_root_path,
        readme_path,
        helper_path,
    )
    oversized = [
        relative for relative in touched if len(_read(root, relative).splitlines()) >= 800
    ]
    if oversized:
        _fail(f"Return source/check files reached 800 lines: {oversized}")
    if re.search(r"Arc<|Rc<|thread_local!|static mut", descent):
        _fail("Return owner must remain stack-scoped and non-persistent")

    return (
        "ret_value_driver=1 ret_raw_value_callers=1 ret_void_callers=1 "
        "ret_detached_callers=1 ret_old_facades=0 ret_parity_reference=1"
    )
