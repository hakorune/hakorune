"""Focused P0c-MR graph-inventory and disconnected SCC authority guard."""

from __future__ import annotations

import pathlib
import re


def check_p0c_mr(root: pathlib.Path, fail) -> None:
    inventory = root / "src/mir/compiler/callable_graph_inventory.rs"
    inventory_tests = root / "src/mir/compiler/callable_graph_inventory/tests.rs"
    acyclic = root / "src/mir/compiler/acyclic_callable_graph.rs"
    scc = root / "src/mir/compiler/callable_scc_partition.rs"
    scc_algorithm = root / "src/mir/compiler/callable_scc_partition/algorithm.rs"
    scc_tests = root / "src/mir/compiler/callable_scc_partition/tests.rs"
    recursive_plan = root / "src/mir/compiler/recursive_callable_module_plan.rs"
    recursive_plan_tests = (
        root / "src/mir/compiler/recursive_callable_module_plan/tests.rs"
    )
    recursive_capability = (
        root / "src/mir/canonical_recursive_callable_module_capability.rs"
    )
    recursive_backend_gate = (
        root / "src/mir/canonical_recursive_callable_module_backend_capability.rs"
    )
    recursive_activation_tests = (
        root / "src/mir/compiler/recursive_callable_module_activation_tests.rs"
    )
    transaction = (
        root / "src/mir/builder/resolved_lowering/callable_module_transaction.rs"
    )
    compiler_mod = root / "src/mir/compiler/mod.rs"
    this_guard = root / "tools/checks/lib/resolved_callable_p0c_mr.py"

    for path in [
        inventory,
        inventory_tests,
        acyclic,
        scc,
        scc_algorithm,
        scc_tests,
        recursive_plan,
        recursive_plan_tests,
        recursive_capability,
        recursive_backend_gate,
        recursive_activation_tests,
        transaction,
        compiler_mod,
        this_guard,
    ]:
        lines = len(path.read_text().splitlines())
        if lines >= 800:
            fail(
                f"P0c-MR source/check reached 800-line stop boundary: "
                f"{path.relative_to(root)} ({lines})"
            )

    if scc_tests.read_text().count("#[test]") != 4:
        fail("P0c-MR-S0 focused SCC fixture count must remain exactly four")

    actual_scc_callers = {
        path
        for path in (root / "src").rglob("*.rs")
        if "VerifiedCallableSccPartitionV1::verify" in path.read_text()
    }
    if actual_scc_callers != {scc_tests, recursive_plan}:
        fail(
            "P0c-MR-S0/V0 SCC consumer drift: "
            f"{sorted(path.relative_to(root) for path in actual_scc_callers)}"
        )

    inventory_text = inventory.read_text()
    if re.search(
        r"#\[derive\([^]]*Clone[^]]*\)\]\s*pub\(crate\) struct "
        r"VerifiedCallableGraphInventoryV1",
        inventory_text,
    ):
        fail("P0c-MR-G0 sealed inventory must remain non-Clone")

    scc_text = scc.read_text()
    for marker in [
        "VerifiedCallableSccPartitionV1",
        "CallableSccIdV1",
        "CallableSccRecursionKindV1",
        "component_by_callable",
        "condensation_edges",
        "condensation_order",
        "seal_partition(inventory, drafts)",
    ]:
        if marker not in scc_text:
            fail(f"P0c-MR-S0 partition contract missing: {marker}")
    if re.search(
        r"#\[derive\([^]]*Clone[^]]*\)\]\s*pub\(crate\) struct "
        r"VerifiedCallableSccPartitionV1",
        scc_text,
    ):
        fail("P0c-MR-S0 sealed partition must remain non-Clone")
    for forbidden in [
        "VerifiedTrivialDirectCallV1",
        "CanonicalCallableSymbolV1",
        "ConservativeBarrier",
        "MirInstruction",
        "MirBuilder",
        "MirFunction",
        "MirModule",
        "Backend",
        "Runtime",
        "Arc<",
        "Mutex<",
        "RwLock<",
    ]:
        if forbidden in scc_text:
            fail(f"P0c-MR-S0 partition owns a forbidden authority: {forbidden}")

    algorithm_text = scc_algorithm.read_text()
    for marker in [
        "iterative_finish_order",
        "let mut stack",
        "while let Some",
        "reachable_within",
    ]:
        if marker not in algorithm_text:
            fail(f"P0c-MR-S0 explicit-stack traversal missing: {marker}")
    for forbidden in ["fn dfs", "fn recursive", "Tarjan index"]:
        if forbidden in algorithm_text:
            fail(f"P0c-MR-S0 host-stack/discovery authority drift: {forbidden}")

    if compiler_mod.read_text().count("mod callable_scc_partition;") != 1:
        fail("P0c-MR-S0 callable SCC module declaration drift")

    if recursive_plan_tests.read_text().count("#[test]") != 3:
        fail("P0c-MR-V0 focused recursive-plan fixture count must remain exactly three")
    actual_plan_callers = {
        path
        for path in (root / "src").rglob("*.rs")
        if "VerifiedRecursiveCallableModulePlanV1::verify" in path.read_text()
    }
    if actual_plan_callers != {recursive_plan_tests, compiler_mod}:
        fail(
            "P0c-MR-V0 production caller drift: "
            f"{sorted(path.relative_to(root) for path in actual_plan_callers)}"
        )
    recursive_plan_text = recursive_plan.read_text()
    for marker in [
        "VerifiedRecursiveCallableModulePlanV1",
        "VerifiedCallableSccPartitionV1",
        "CanonicalTrivialBindingSsaPlanV1",
        "recursive_component_count() == 0",
        "FunctionCallSiteCountMismatch",
        "CardinalityMismatch",
    ]:
        if marker not in recursive_plan_text:
            fail(f"P0c-MR-V0 recursive plan contract missing: {marker}")
    for forbidden in [
        "MirInstruction",
        "MirBuilder",
        "MirFunction",
        "MirModule",
        "FunctionReturnContract",
        "ConservativeBarrier",
        "Backend",
        "Runtime",
        "CopyOwned",
        "DestroyOwned",
    ]:
        if forbidden in recursive_plan_text:
            fail(f"P0c-MR-V0 plan owns a forbidden authority: {forbidden}")
    if compiler_mod.read_text().count("mod recursive_callable_module_plan;") != 1:
        fail("P0c-MR-V0 recursive module plan declaration drift")

    install_callers = {
        path
        for path in (root / "src").rglob("*.rs")
        if "CanonicalRecursiveCallableModuleCapabilityV1::install_for_module" in path.read_text()
    }
    if install_callers != {recursive_backend_gate, transaction}:
        fail(
            "P0c-MR-C0 production capability producer drift: "
            f"{sorted(path.relative_to(root) for path in install_callers)}"
        )
    capability_text = recursive_capability.read_text()
    for marker in [
        "canonical_recursive_callable_module_v1",
        "schema_version: u8",
        "install_for_module",
        "verify_required",
        "capability_missing",
        "capability_preexisting",
        "capability_schema_drift",
    ]:
        if marker not in capability_text:
            fail(f"P0c-MR-C0 capability contract missing: {marker}")
    backend_text = recursive_backend_gate.read_text()
    for marker in [
        "[backend/canonical_recursive_callable_module_v1_unsupported]",
        'backend == "mir-interpreter"',
        "silent_fallback_allowed=false",
    ]:
        if marker not in backend_text:
            fail(f"P0c-MR-C0 backend fail-fast contract missing: {marker}")
    for forbidden in [
        "VerifiedCallableSccPartitionV1",
        "VerifiedCallableGraphInventoryV1",
        "MirInstruction",
        "FunctionCall",
    ]:
        if forbidden in backend_text:
            fail(f"P0c-MR-C0 backend gate infers topology: {forbidden}")

    if recursive_activation_tests.read_text().count("#[test]") != 7:
        fail("P0c-MR-I1 focused activation/proof fixture count must remain exactly seven")
    compiler_text = compiler_mod.read_text()
    if len(re.findall(r"pub fn compile_resolved_recursive_callable_module\s*\(", compiler_text)) != 1:
        fail("P0c-MR-I1 explicit recursive compiler ingress count drift")
    transaction_text = transaction.read_text()
    for marker in [
        "collect_recursive_with",
        "build_recursive_callable_module_candidate",
        "publish_recursive_callable_drafts",
        "drafts.publish_into(module)?",
        "canonical_recursive_callable_module_capability",
    ]:
        if marker not in transaction_text:
            fail(f"P0c-MR-I1 atomic transaction contract missing: {marker}")
    for forbidden in [
        "compile_resolved_callable_module(",
        "compile_resolved(",
        "compile_legacy(",
        "retry",
        "fallback",
    ]:
        method_start = compiler_text.index("pub fn compile_resolved_recursive_callable_module")
        method_end = compiler_text.index("/// Compile an explicitly non-canonical", method_start)
        if forbidden in compiler_text[method_start:method_end]:
            fail(f"P0c-MR-I1 recursive ingress route retry drift: {forbidden}")
