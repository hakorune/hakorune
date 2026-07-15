"""Focused P0c-F finite-call, DAG, typed-plan, and I1 activation guard."""

from __future__ import annotations

import pathlib
import re


def check_p0c_f(root: pathlib.Path, fail) -> None:
    compiler_mod = root / "src/mir/compiler/mod.rs"
    compiler_capability = root / "src/mir/compiler/capability.rs"
    finite_call_tests = root / "src/mir/compiler/finite_direct_call_tests.rs"
    profile_mod = root / "src/mir/resolved_value_profile/mod.rs"
    profile_analyzer = root / "src/mir/resolved_value_profile/analyzer.rs"
    profile_tests = root / "src/mir/resolved_value_profile/direct_call_tests.rs"
    graph = root / "src/mir/compiler/acyclic_callable_graph.rs"
    graph_tests = root / "src/mir/compiler/acyclic_callable_graph/tests.rs"
    plan = root / "src/mir/compiler/acyclic_callable_module_plan.rs"
    plan_tests = root / "src/mir/compiler/acyclic_callable_module_plan/tests.rs"
    activation_tests = root / "src/mir/compiler/acyclic_callable_module_activation_tests.rs"
    transaction = root / "src/mir/builder/resolved_lowering/callable_module_transaction.rs"
    transaction_tests = (
        root / "src/mir/builder/resolved_lowering/callable_module_transaction_tests.rs"
    )

    allowed_by_pattern = {
        r"analyze_trivial_canonical_owner_with_finite_direct_calls_v1": {
            compiler_capability,
            profile_mod,
            profile_tests,
        },
        r"verify_function_with_finite_direct_calls_v1": {
            compiler_capability,
            finite_call_tests,
            plan,
        },
        r"VerifiedAcyclicCallableGraphV1::verify": {graph_tests, plan},
        r"VerifiedAcyclicCallableModulePlanV1::verify": {
            plan_tests,
            compiler_mod,
            transaction_tests,
        },
        r"build_acyclic_callable_module_candidate\(": {
            transaction,
            transaction_tests,
            compiler_mod,
        },
    }
    for pattern, allowed in allowed_by_pattern.items():
        actual = {
            path
            for path in (root / "src").rglob("*.rs")
            if re.search(pattern, path.read_text())
        }
        unexpected = sorted(path.relative_to(root) for path in actual - allowed)
        missing = sorted(path.relative_to(root) for path in allowed - actual)
        if unexpected or missing:
            fail(
                f"P0c-F caller set drift for {pattern!r}: "
                f"unexpected={unexpected} missing={missing}"
            )

    for path in [
        compiler_mod,
        compiler_capability,
        finite_call_tests,
        profile_mod,
        profile_analyzer,
        profile_tests,
        graph,
        graph_tests,
        plan,
        plan_tests,
        activation_tests,
        transaction,
        transaction_tests,
        root / "tools/checks/lib/resolved_callable_l0.py",
        root / "tools/checks/lib/resolved_callable_p0c_f.py",
    ]:
        lines = len(path.read_text().splitlines())
        if lines >= 800:
            fail(f"source/check reached 800-line stop boundary: {path.relative_to(root)} ({lines})")

    expected_tests = {
        profile_tests: 7,
        finite_call_tests: 2,
        graph_tests: 4,
        plan_tests: 3,
        activation_tests: 4,
    }
    for path, expected in expected_tests.items():
        actual = path.read_text().count("#[test]")
        if actual != expected:
            fail(
                f"P0c-F fixture count drift: {path.relative_to(root)} "
                f"expected={expected} actual={actual}"
            )

    capability_text = compiler_capability.read_text()
    for marker in [
        "DirectCallAdmissionV1::ExistingExactOne",
        "DirectCallAdmissionV1::FiniteOneOrMore",
        "verify_expression(input, &argument, expression_policy)",
    ]:
        if marker not in capability_text:
            fail(f"P0c-F-DX0a preflight boundary missing: {marker}")
    if "DirectCallPolicyV1::OneOrMoreExact" not in profile_analyzer.read_text():
        fail("P0c-F-DX0a finite profile policy missing")

    graph_text = graph.read_text()
    for marker in [
        "VerifiedAcyclicCallableGraphV1",
        "VerifiedCallableGraphSiteV1",
        "VerifiedCallableGraphEdgeV1",
        "header_for_callable(target.callable())",
        "Deterministic Kahn",
    ]:
        if marker not in graph_text:
            fail(f"P0c-F-S0 graph contract missing: {marker}")
    for forbidden in [
        "VerifiedTrivialDirectCallV1",
        "MirInstruction",
        "CanonicalCallableSymbolV1",
        "ConservativeBarrier",
        "VerifiedCallableModulePreflightV1",
    ]:
        if forbidden in graph_text:
            fail(f"P0c-F-S0 topology product owns a forbidden authority: {forbidden}")

    plan_text = plan.read_text()
    for marker in [
        "VerifiedAcyclicCallableModulePlanV1",
        "CanonicalTrivialBindingSsaPlanV1",
        "verify_function_with_finite_direct_calls_v1",
        "FunctionCallSiteCountMismatch",
    ]:
        if marker not in plan_text:
            fail(f"P0c-F-V0 typed plan contract missing: {marker}")
    for forbidden in [
        "MirBuilder",
        "MirInstruction",
        "MirFunction",
        "MirModule",
        "CanonicalCallableSymbolV1",
        "ConservativeBarrier",
        "build_resolved_callable_module_candidate",
        "try_add_functions_atomic",
    ]:
        if forbidden in plan_text:
            fail(f"P0c-F-V0 typed plan owns a forbidden effect authority: {forbidden}")

    compiler_text = compiler_mod.read_text()
    for marker in [
        'callable_program_stage_error("acyclic_activation"',
        "build_acyclic_callable_module_candidate(plan)",
        "canonical_callable_module/{stage}",
    ]:
        if marker not in compiler_text:
            fail(f"P0c-F-I1 compiler ingress missing: {marker}")
    for retired in ["VerifiedSiblingCallModulePlanV1", '"sibling_activation"']:
        if retired in compiler_text:
            fail(f"P0c-F-I1 retained retired B1 activation authority: {retired}")

    transaction_text = transaction.read_text()
    for marker in [
        "collect_acyclic_with",
        "collect_typed_with",
        "build_acyclic_callable_module_candidate",
        "publish_callable_drafts",
    ]:
        if marker not in transaction_text:
            fail(f"P0c-F-I1 typed transaction seam missing: {marker}")
    if (root / "src/mir/compiler/sibling_call_activation.rs").exists():
        fail("P0c-F-I1 must retire the superseded B1 activation witness")
