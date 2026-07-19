#!/usr/bin/env python3
"""Guard source-proof-gated callable-result activation rows."""

from __future__ import annotations

import re
import sys
from pathlib import Path


def fail(message: str) -> None:
    raise SystemExit(f"[callable-result-i0-a0] {message}")


def read(root: Path, relative: str) -> str:
    path = root / relative
    if not path.is_file():
        fail(f"missing {relative}")
    return path.read_text(encoding="utf-8")


def require_count(text: str, needle: str, expected: int, label: str) -> None:
    actual = text.count(needle)
    if actual != expected:
        fail(f"{label}: expected={expected} actual={actual}")


def main() -> None:
    root = Path(sys.argv[1] if len(sys.argv) > 1 else ".").resolve()
    activation = read(root, "src/mir/callable_result_representation/activation.rs")
    source_gate = read(
        root, "src/mir/callable_result_representation/activation_source_gate.rs"
    )
    emission = read(
        root, "src/mir/builder/control_flow/plan/lowerer/emission_port.rs"
    )
    located_loop = read(root, "src/mir/builder/control_flow/plan/located_loop.rs")
    solver = read(root, "src/mir/callable_result_representation/solver.rs")
    tests = read(root, "src/mir/callable_result_representation/tests/activation.rs")

    require_count(
        activation,
        "struct VerifiedCallableResultActivationPlanV1",
        1,
        "owned activation plan",
    )
    require_count(
        activation,
        "struct VerifiedCallableResultActivationRowsV1",
        1,
        "opaque activation rows",
    )
    require_count(
        activation,
        "CallableResultActivationDispositionV1::SelectedExactI64",
        1,
        "selected disposition producer",
    )
    require_count(
        activation,
        "CallableResultActivationDispositionV1::Unselected",
        1,
        "source-gate unselected projection",
    )
    require_count(
        activation,
        "classify_activation_source_site_v1(",
        1,
        "sole activation source-gate consumer",
    )
    require_count(activation, "results.disposition(target)", 0, "target-only selection")
    require_count(activation, "results.call_result(", 0, "activation result-row bypass")
    require_count(source_gate, "results.disposition(target)", 1, "source-gate target join")
    require_count(source_gate, "results.call_result(", 1, "source-gate required-proof join")
    require_count(
        emission,
        "CallableResultActivationDispositionV1::Unselected => {\n                        PlanLowerer::emit_raw_effect(builder, effect)",
        1,
        "planned Unselected raw-primary emission",
    )
    require_count(
        emission,
        "fn selected_terminal_uses_claim_target_not_raw_global_spelling()",
        1,
        "generic selected-terminal success fixture",
    )
    require_count(
        emission,
        "fn selected_terminal_rejects_unknown_required_argument_before_call_or_result_publication()",
        1,
        "generic selected-terminal fail-fast fixture",
    )
    require_count(
        activation,
        "observe_method_calls_shadow_view_v0(view)",
        1,
        "PATH0 inventory consumer",
    )
    require_count(
        located_loop,
        "struct LocatedCoreLoopExecutionSessionV1",
        1,
        "claimed CorePlan execution session owner",
    )
    require_count(
        located_loop,
        "enum LocatedCoreLoopExecutionStateV1",
        1,
        "claimed CorePlan execution state owner",
    )
    require_count(
        located_loop,
        "into_claimed_execution",
        0,
        "claimed bundle production bypass",
    )
    if "LocatedLegacyLoweringSessionV1" in located_loop:
        fail("CorePlan session must not reuse the legacy AST lowering session")

    for product in (
        "VerifiedCallableResultActivationRowsV1",
        "VerifiedCallableResultActivationPlanV1",
    ):
        if re.search(rf"#\[derive\([^]]*Clone[^]]*\)\]\s*pub\(crate\) struct {product}", activation):
            fail(f"{product} must remain non-Clone")
    for forbidden in ("Arc<", "Rc<", "ASTNode", "VerifiedSourceStaticCallTargetCatalogV1<'static>"):
        if forbidden in activation:
            fail(f"activation product contains forbidden authority: {forbidden}")

    if "if !all_keys.contains_key(caller)" not in solver:
        fail("instance caller membership is not checked against the complete catalog")
    if "if !static_keys.contains_key(row.target())" not in solver:
        fail("target membership is not restricted to static result keys")
    if re.search(r"for \([^)]*\) in all_declarations \{[\s\S]{0,300}prove_function", solver):
        fail("instance declarations entered result inference")

    for evidence in (
        "assert_eq!(rows.len(), 15)",
        "actual_parser_add_inventory_keeps_every_source_row_unselected",
        "activation_rows_preserve_the_generic_literal_selected_disposition",
        "source_gate_selects_direct_formal_required_argument",
        "declaration_reorder_preserves_owned_activation_rows",
        "activation_rows_cannot_pair_with_an_equal_foreign_catalog",
    ):
        if evidence not in tests:
            fail(f"missing fixture evidence: {evidence}")

    production_consumers = 0
    for path in (root / "src").rglob("*.rs"):
        relative = path.relative_to(root).as_posix()
        if "/tests/" in relative or path.name == "tests.rs" or path.name.endswith("_tests.rs"):
            continue
        production_consumers += path.read_text(encoding="utf-8").count(
            "VerifiedCallableResultActivationPlanV1::seal("
        )
    if production_consumers != 0:
        fail(f"production activation consumers: expected=0 actual={production_consumers}")

    production_session_starts = 0
    for path in (root / "src").rglob("*.rs"):
        relative = path.relative_to(root).as_posix()
        if "/tests/" in relative or path.name == "tests.rs" or path.name.endswith("_tests.rs"):
            continue
        count = path.read_text(encoding="utf-8").count(".start_execution(")
        production_session_starts += count
    if production_session_starts != 0:
        fail(
            "production claimed CorePlan execution-session callers: "
            f"expected=0 actual={production_session_starts}"
        )

    touched = [
        "src/mir/builder/callable_declaration_catalog/catalog.rs",
        "src/mir/callable_result_representation/activation.rs",
        "src/mir/callable_result_representation/activation_source_gate.rs",
        "src/mir/callable_result_representation/activation_error.rs",
        "src/mir/builder/control_flow/plan/lowerer/emission_port.rs",
        "src/mir/builder/control_flow/plan/located_loop.rs",
        "src/mir/builder/control_flow/plan/located_loop_tests.rs",
        "src/mir/callable_result_representation/solver.rs",
        "src/mir/callable_result_representation/tests/activation.rs",
        "src/mir/callable_result_representation/tests/generic_selected_activation_fixture.rs",
        "src/mir/callable_result_representation/tests/support.rs",
        "tools/checks/lib/callable_result_i0_a0.py",
    ]
    oversized = [relative for relative in touched if len(read(root, relative).splitlines()) >= 800]
    if oversized:
        fail(f"source/check files reached 800 lines: {oversized}")

    print(
        "[callable-result-i0-a0] ok: plan=1 rows=owned actual=15/2/0/15 "
        "generic_selected=1 source_gate_consumers=1 production_consumers=0 "
        "core_loop_session_consumers=0"
    )


if __name__ == "__main__":
    main()
