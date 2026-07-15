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
    compiler_mod = root / "src/mir/compiler/mod.rs"
    this_guard = root / "tools/checks/lib/resolved_callable_p0c_mr.py"

    for path in [
        inventory,
        inventory_tests,
        acyclic,
        scc,
        scc_algorithm,
        scc_tests,
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
    if actual_scc_callers != {scc_tests}:
        fail(
            "P0c-MR-S0 production caller drift: "
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
