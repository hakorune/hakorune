#!/usr/bin/env python3
"""Validate the disconnected D-prime SSA-M0 real-MIR adapter."""

from __future__ import annotations

from pathlib import Path
import sys


def fail(message: str) -> None:
    raise SystemExit(f"SSA-M0 real-MIR adapter: {message}")


def require(text: str, anchor: str, owner: str) -> None:
    if anchor not in text:
        fail(f"{owner}: missing anchor {anchor!r}")


def main() -> None:
    if len(sys.argv) != 2:
        fail("usage: resolved_binding_ssa_mir_adapter.py ROOT")
    root = Path(sys.argv[1]).resolve()
    box = root / "src/mir/builder/ssa/binding"
    paths = {
        "readme": box / "README.md",
        "module": box / "mod.rs",
        "adapter": box / "mir_adapter.rs",
        "tests": box / "mir_adapter_tests.rs",
        "phi": root / "src/mir/builder/emission/phi_lifecycle.rs",
        "taskboard": root
        / "docs/development/current/main/investigations/"
        "mirbuilder-dprime-binding-ssa-final-form-task-2026-07-14.md",
    }
    for path in paths.values():
        if not path.is_file():
            fail(f"missing required file {path}")
    text = {name: path.read_text() for name, path in paths.items()}

    for anchor in (
        "struct MirBindingSsaAdapterV1",
        "builder: &'a mut MirBuilder",
        "phis: &'a mut PhiTxn",
        "impl BindingSsaIrV1 for MirBindingSsaAdapterV1",
        ".define_provisional_phi(",
        "patch_phi_inputs(",
        "rollback_pending_phi(",
        "compute_def_blocks(function)",
        "compute_dominators(function)",
        "is_reachable_from_entry(function, predecessor)",
        "MirType::Unknown",
    ):
        require(text["adapter"], anchor, "real-MIR adapter")

    for forbidden in (
        "insert_phi_at_head",
        "update_phi_instruction",
        "rollback_provisional_phi",
        "update_cfg",
        "materialize_all_phi_inputs",
        "phi_input_materializer",
        "for_pred",
        "crate::ast",
        "ASTNode",
        "SourceStmtSite",
        "SourceExprSite",
        "ScopeId",
        "RegionId",
        "resolved_region_flow",
        "variable_map",
        "may_rebind",
        "carrier",
    ):
        if forbidden in text["adapter"]:
            fail(f"mechanical adapter regained forbidden policy/repair token: {forbidden}")

    test_count = text["tests"].count("#[test]")
    if test_count != 5:
        fail(f"focused real-MIR fixture count must remain 5, got {test_count}")
    for anchor in (
        "real_loop_phi_is_defined_before_exposure_and_stays_fact_unknown",
        "non_dominating_sibling_input_rolls_back_pending_phi_and_poison_ssa",
        "unreachable_predecessor_is_rejected_explicitly",
        "return_block_uses_the_same_cfg_witness_and_ssa_seal_path",
        "later_patch_failure_keeps_completed_peer_and_discards_draft",
        "CanonicalCfgSessionV1::new()",
        ".seal_block(",
        "ssa.finish().unwrap();",
        "phis.commit(&mut builder).unwrap();",
    ):
        require(text["tests"], anchor, "focused real-MIR fixtures")
    if "VerifiedPredecessorsV1::from_test_parts" in text["tests"]:
        fail("real-MIR fixtures bypassed CanonicalCfgSessionV1 witnesses")

    for anchor in (
        "fn rollback_pending_phi(",
        "if !self.pending.contains(&token)",
        "self.pending.retain(|pending| *pending != token)",
    ):
        require(text["phi"], anchor, "pending-only PHI transaction cleanup")

    callers = []
    for path in (root / "src").rglob("*.rs"):
        if path.parent == box:
            continue
        source = path.read_text()
        if "MirBindingSsaAdapterV1" in source or "BindingSsaBuilderV1::new" in source:
            callers.append(str(path.relative_to(root)))
    if callers:
        fail(f"production Binding SSA/adapter callers must remain zero: {callers}")

    for anchor in (
        "### SSA-M0 — disconnected real-MIR Binding SSA adapter — closed",
        "accepted fact refinement set = empty",
        "production Binding SSA and adapter callers = 0",
        "### SSA-RC0 — ownership and scope-escape law — active",
    ):
        require(text["taskboard"], anchor, "taskboard")

    bounded_paths = [path for name, path in paths.items() if name != "taskboard"]
    for path in (*bounded_paths, Path(__file__)):
        lines = len(path.read_text().splitlines())
        if lines >= 800:
            fail(f"source/check reached the 800-line stop boundary: {path} ({lines})")

    print("canonical_ssa_m0_adapter=real-mir-phi-txn")
    print("canonical_ssa_m0_witness=canonical-cfg-verified-predecessors-only")
    print("canonical_ssa_m0_open_phi_fact=conservative-unknown")
    print("canonical_ssa_m0_fact_refinements=0")
    print(f"canonical_ssa_m0_focused_fixtures={test_count}")
    print("canonical_ssa_m0_production_callers=0")
    print("canonical_ssa_m0_accepted_grammar_delta=0")


if __name__ == "__main__":
    main()
