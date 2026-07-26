#!/usr/bin/env python3
"""Validate the behavior-neutral D-prime SSA-P0 seam inventory."""

from __future__ import annotations

from collections import Counter
import json
from pathlib import Path
import sys


EXPECTED_IDS = set(
    """
binding.flat-environment-owner
binding.flat-environment-construction
binding.flat-publish
binding.flat-read
binding.flat-rebind
binding.flat-remove
binding.flat-active-bindings
binding.flat-direct-membership
binding.parameter-declaration
binding.local-declaration
binding.outbox-declaration
binding.variable-read
binding.assignment-target-resolution
binding.assignment-old-value-read
binding.assignment-new-definition
binding.scope-success-retirement
binding.scope-error-retirement
binding.scope-materialized-values
cfg.canonical-conditional
cfg.shared-conditional-wrapper
cfg.canonical-then-jump
cfg.shared-jump-wrapper
cfg.shared-branch-setter
cfg.shared-jump-setter
cfg.generic-emission-writer
cfg.raw-terminator-writer
cfg.raw-seal-state
cfg.phi-analysis-repair
cfg.if-predecessor-check
cfg.successor-derived-recompute
phi.txn-owner
phi.provisional-define
phi.final-define
phi.batch-prepend
phi.function-final-define
phi.patch
phi.rollback
phi.canonical-if-final
phi.canonical-if-reserve
phi.canonical-if-expose
phi.plan-provisional
phi.plan-patch-rollback
phi.joinir-exit-transaction
phi.joinir-loop-batch
phi.edgecfg-final
phi.bridge-conditional-method
phi.bridge-block-converter
phi.legacy-builder-wrappers
phi.legacy-if-form
phi.legacy-loop-api
phi.peek-expression
phi.postpass-repair
phi.module-postpass-call
rc.assignment-previous-read
rc.assignment-release
rc.self-assignment-gap
rc.scope-success-gap
rc.scope-error-discard
rc.module-insertion-pass
rc.blockexpr-tail-escape-gap
publication.lowerer-finish
publication.old-effect-empty
publication.resolved-binding-finish
publication.function-draft-finalize
publication.function-session-commit
publication.module-finalize
publication.module-verifier-result
publication.module-session-commit
publication.module-session-owner
publication.duplicate-function-overwrite
return.resolver-exit-record
return.target-verifier
return.source-projection
return.final-only-preflight
return.region-flow-duplicate-policy
return.canonical-lower-arm
return.exit-coverage-claim
return.shared-emitter
return.cleanup-contract-gap
return.implicit-void-finalization
if.effect-vocabulary
if.flow-effect-fields
if.effect-analysis
if.effect-verifier
if.effect-consumption
if.branch-snapshot-owner
if.active-effect-stack
if.lower-effect-queries
if.manual-join-materializer
if.identity-store-adapters
if.topology-cfg-session
if.legacy-if-form
""".split()
)

EXPECTED_CATEGORIES = {
    "binding_value",
    "cfg_predecessor",
    "phi_lifecycle",
    "rc_lifetime",
    "publication",
    "terminal_return",
    "old_if_authority",
}
EXPECTED_DISPOSITIONS = {
    "move to Binding SSA",
    "control-only retain",
    "legacy isolate",
    "caller-zero delete",
}
ROW_FIELDS = {
    "id",
    "category",
    "current_path",
    "anchors",
    "current_role",
    "disposition",
    "target",
    "note",
}


def fail(message: str) -> None:
    raise SystemExit(f"SSA-P0 inventory: {message}")


def production_rs(root: Path):
    for path in (root / "src").rglob("*.rs"):
        if path.name == "tests.rs" or path.name.endswith("_tests.rs"):
            continue
        yield path


def count_in(paths, literal: str) -> int:
    return sum(path.read_text(errors="ignore").count(literal) for path in paths)


def main() -> None:
    if len(sys.argv) != 3:
        fail("usage: resolved_binding_ssa_inventory.py ROOT INVENTORY")
    root = Path(sys.argv[1]).resolve()
    inventory = Path(sys.argv[2]).resolve()
    data = json.loads(inventory.read_text())

    if set(data) != {"schema", "decision", "classifications", "rows"}:
        fail("top-level schema drifted")
    if data["schema"] != "CanonicalSsaSeamInventoryV1":
        fail("schema name drifted")
    if data["decision"] != "dprime_function_owned_binding_ssa":
        fail("D-prime decision drifted")
    if set(data["classifications"]) != EXPECTED_DISPOSITIONS:
        fail("classification vocabulary drifted")

    rows = data["rows"]
    ids = [row.get("id") for row in rows]
    if len(ids) != len(set(ids)):
        fail("duplicate row id")
    if set(ids) != EXPECTED_IDS:
        fail(
            "required row set drifted: "
            f"missing={sorted(EXPECTED_IDS - set(ids))} "
            f"extra={sorted(set(ids) - EXPECTED_IDS)}"
        )

    for row in rows:
        row_id = row["id"]
        if set(row) != ROW_FIELDS:
            fail(f"{row_id}: row schema drifted")
        if row["category"] not in EXPECTED_CATEGORIES:
            fail(f"{row_id}: unknown category")
        if row["disposition"] not in EXPECTED_DISPOSITIONS:
            fail(f"{row_id}: unknown disposition")
        path = root / row["current_path"]
        if not path.is_file():
            fail(f"{row_id}: missing current_path {path}")
        anchors = row["anchors"]
        if not isinstance(anchors, list) or not anchors:
            fail(f"{row_id}: anchors must be a non-empty list")
        text = path.read_text(errors="ignore")
        for anchor in anchors:
            if not isinstance(anchor, str) or not anchor or anchor not in text:
                fail(f"{row_id}: stale anchor {anchor!r}")
        for field in ("current_role", "target", "note"):
            if not isinstance(row[field], str) or not row[field]:
                fail(f"{row_id}: {field} must be non-empty")

    categories = Counter(row["category"] for row in rows)
    if set(categories) != EXPECTED_CATEGORIES:
        fail(f"category coverage drifted: {categories}")
    dispositions = Counter(row["disposition"] for row in rows)
    if set(dispositions) != EXPECTED_DISPOSITIONS:
        fail(f"disposition coverage drifted: {dispositions}")

    lowerer = (root / "src/mir/builder/resolved_lowering/lowerer.rs").read_text()
    identity = (root / "src/mir/builder/resolved_lowering/identity.rs").read_text()
    identity_ledger = (
        root / "src/mir/builder/resolved_lowering/identity/ledger.rs"
    ).read_text()
    value_environment = (
        root / "src/mir/builder/resolved_lowering/identity/value_environment.rs"
    ).read_text()
    if_materialization = (
        root / "src/mir/builder/resolved_lowering/if_materialization.rs"
    ).read_text()
    located_if = (root / "src/mir/builder/resolved_lowering/located_if.rs").read_text()
    module_lifecycle = (root / "src/mir/builder/module_lifecycle.rs").read_text()

    exact_counts = {
        "canonical declaration publications": (
            lowerer.count("self.identity.publish_declaration("),
            3,
        ),
        "canonical variable reads": (lowerer.count("self.identity.variable_value("), 1),
        "canonical assignment old-value reads": (
            lowerer.count("self.identity.current_value(binding)?"),
            1,
        ),
        "canonical assignment releases": (lowerer.count("MirInstruction::ReleaseStrong"), 1),
        "canonical conditional edges": (
            if_materialization.count("branch::emit_conditional("),
            1,
        ),
        "canonical merge jumps": (if_materialization.count("branch::emit_jump("), 2),
        "canonical old If snapshots": (
            located_if.count("ResolvedBranchTransactionV1::snapshot("),
            2,
        ),
        "canonical old If manual PHI calls": (located_if.count("define_join_phis("), 1),
        "canonical old If join publications": (
            located_if.count("publish_join_values("),
            1,
        ),
    }
    for label, (actual, expected) in exact_counts.items():
        if actual != expected:
            fail(f"{label}: expected {expected}, got {actual}")

    if value_environment.count("values: BTreeMap<BindingRefV1, ValueId>") != 1:
        fail("pre-SSA flat environment owner drifted")
    if identity.count("self.values.contains(*binding)") != 1:
        fail("pre-SSA environment membership seam drifted")
    if identity.count("self.values.remove(*binding)") != 2:
        fail("flat environment scope removal seam drifted")
    if "ValueId" in identity_ledger or "BasicBlockId" in identity_ledger:
        fail("SSA-S2 identity ledger regained MIR value/block ownership")
    if "materialize_all_phi_inputs" not in module_lifecycle:
        fail("module PHI repair seam disappeared without SSA-I1 reclassification")

    production = list(production_rs(root))
    binding_ssa_path = root / "src/mir/builder/ssa/binding/mod.rs"
    if binding_ssa_path not in production:
        fail("SSA-S1 disconnected BindingSsaBuilderV1 owner is missing")
    if count_in(production, "struct BindingSsaBuilderV1") != 1:
        fail("SSA-S1 must retain exactly one BindingSsaBuilderV1 declaration")
    non_test_resolved_callers = [
        path
        for path in production
        if path != root / "src/mir/compiler/lowering_input.rs"
    ]
    if count_in(non_test_resolved_callers, ".compile_resolved(") != 0:
        fail("default/non-test resolved caller activated during SSA-P0")
    lowering_input = (root / "src/mir/compiler/lowering_input.rs").read_text()
    if lowering_input.count(".compile_resolved(") != 2 or "#[cfg(test)]" not in lowering_input:
        fail("inline compile_resolved test caller boundary drifted")

    facade = root / "src/mir/builder/ssa/phi_input_materializer.rs"
    split_dir = root / "src/mir/builder/ssa/phi_input_materializer"
    expected_split_manifest = {
        "edge_rematerialization.rs",
        "edge_rematerialization_tests.rs",
        "edge_verifier.rs",
        "edge_verifier_p0_tests.rs",
        "function_repair.rs",
        "function_repair_tests.rs",
        "legacy_candidate.rs",
        "legacy_candidate_cfg.rs",
        "legacy_candidate_tests.rs",
        "remat_fact.rs",
        "remat_fact_tests.rs",
        "test_support.rs",
    }
    actual_split_manifest = {path.name for path in split_dir.iterdir() if path.is_file()}
    if actual_split_manifest != expected_split_manifest:
        fail(
            "SSA-L0 split manifest drifted: "
            f"expected={sorted(expected_split_manifest)} "
            f"actual={sorted(actual_split_manifest)}"
        )
    facade_text = facade.read_text()
    for anchor in (
        "mod edge_rematerialization;",
        "pub(in crate::mir::builder) mod edge_verifier;",
        "mod function_repair;",
        "pub(in crate::mir::builder) mod legacy_candidate;",
        "pub(in crate::mir::builder) mod remat_fact;",
        "use edge_rematerialization::for_pred;",
        "use function_repair::materialize_all_phi_inputs;",
    ):
        if anchor not in facade_text:
            fail(f"SSA-L0 facade anchor missing: {anchor}")
    for forbidden in (
        "struct PhiInputMaterializationAnalysis",
        "fn rematerialize_for_pred",
        "fn prune_unused_phi_instructions",
        "fn complete_missing_self_carried_phi_inputs",
    ):
        if forbidden in facade_text:
            fail(f"SSA-L0 facade regained implementation: {forbidden}")
    for path in [facade, *(split_dir / name for name in expected_split_manifest)]:
        lines = len(path.read_text().splitlines())
        if lines >= 800:
            fail(f"SSA-L0 source reached the 800-line stop boundary: {path} ({lines})")

    print(f"canonical_ssa_p0_rows={len(rows)}")
    for category in sorted(categories):
        print(f"canonical_ssa_p0_{category}={categories[category]}")
    for disposition in sorted(dispositions):
        key = disposition.replace(" ", "_").replace("-", "_")
        print(f"canonical_ssa_p0_{key}={dispositions[disposition]}")
    print("canonical_ssa_p0_behavior_delta=0")
    print("canonical_ssa_l0_facade=thin")
    print("canonical_ssa_l0_edge_rematerialization=isolated")
    print("canonical_ssa_l0_function_repair=legacy-isolated")
    print("canonical_ssa_l0_behavior_delta=0")
    print("canonical_ssa_p0_follow_on=SSA-C1")


if __name__ == "__main__":
    main()
