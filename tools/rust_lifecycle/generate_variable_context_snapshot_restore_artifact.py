#!/usr/bin/env python3
"""Generate the focused VariableContext snapshot/restore Hako artifact.

This 1524 pilot is intentionally narrower than full VariableContext. It emits
only the snapshot()/restore() ownership transfer surface over the checked
snapshot/restore fixtures. Mutable map access and carrier-sensitive behavior
remain excluded.
"""

from __future__ import annotations

import argparse
from pathlib import Path
from textwrap import dedent
from typing import Any

from shared_family_generator import (
    build_common_rust_derived_inputs,
    build_common_rust_derived_manifest,
    read_json,
    run_validated_family_generator,
    stable_json,
)
from shared_mirbuilder_emitter import emit_verified_family_hako


ROOT = Path(__file__).resolve().parents[2]
FIXTURES = ROOT / "docs/development/current/main/design/fixtures/rust-lifecycle"
OUT_DIR = ROOT / "lang/generated/rust_derived/hakorune_mir_builder"

FACTS = FIXTURES / "variable-context-snapshot-restore-facts-v0.json"
PLAN = FIXTURES / "variable-context-snapshot-restore-plan-v0.json"
ORACLE = FIXTURES / "variable-context-snapshot-restore-oracle-vectors-v0.json"
HAKO = OUT_DIR / "variable_context_snapshot_restore.hako"
MANIFEST = OUT_DIR / "variable_context_snapshot_restore.artifact.json"

SUBJECT = "hakorune_mir_builder::variable_context::VariableContext.snapshot_restore"
FAMILY_ID = "hakorune_mir_builder::variable_context"
SCOPE = "VariableContext_snapshot_restore_only"
EXCLUDED = [
    "VariableContext::variable_map",
    "VariableContext::variable_map_mut",
    "VariableContext::lookup",
    "VariableContext::require",
    "VariableContext::insert",
    "VariableContext::remove",
    "VariableContext::contains",
    "VariableContext::len",
    "VariableContext::is_empty",
    "CarrierInfo::from_variable_map",
    "CarrierInfo::with_explicit_carriers",
    "PHI planner integration",
]


def validate_inputs(facts: dict[str, Any], plan: dict[str, Any], oracle: dict[str, Any]) -> None:
    if facts.get("kind") != "RustLifecycleFacts":
        raise SystemExit("unexpected facts kind")
    if plan.get("kind") != "HakoLifecyclePlan":
        raise SystemExit("unexpected plan kind")
    if oracle.get("kind") != "RustOracleVectors":
        raise SystemExit("unexpected oracle kind")
    if facts.get("subject") != SUBJECT or plan.get("subject") != SUBJECT or oracle.get("subject") != SUBJECT:
        raise SystemExit("subject mismatch")
    if facts.get("base_facts") != "variable-context-simple-map-facts-v0.json":
        raise SystemExit("unexpected base facts")

    method_facts = {row["id"]: row for row in facts.get("method_facts", [])}
    if set(method_facts) != {"VariableContext::snapshot", "VariableContext::restore"}:
        raise SystemExit("unexpected method facts")

    snapshot = method_facts["VariableContext::snapshot"]
    if snapshot.get("operation") != "CloneOwnedMap":
        raise SystemExit("snapshot operation must be CloneOwnedMap")
    snapshot_receiver = snapshot.get("receiver_borrow", {})
    if snapshot_receiver.get("kind") != "SharedRead" or snapshot_receiver.get("escapes") is not False:
        raise SystemExit("snapshot receiver borrow mismatch")
    snapshot_returns = snapshot.get("returns", {})
    if snapshot_returns.get("copy_kind") != "NonCopyOwned":
        raise SystemExit("snapshot copy kind mismatch")
    if snapshot_returns.get("deterministic_order_required") is not True:
        raise SystemExit("snapshot must require deterministic order")
    if snapshot_returns.get("drop_fact") != "TrivialMemory":
        raise SystemExit("snapshot drop fact must be TrivialMemory")
    if snapshot_returns.get("rust_type") != "BTreeMap<String, ValueId>":
        raise SystemExit("unexpected snapshot rust type")

    restore = method_facts["VariableContext::restore"]
    if restore.get("operation") != "ReplaceOwned":
        raise SystemExit("restore operation must be ReplaceOwned")
    restore_receiver = restore.get("receiver_borrow", {})
    if restore_receiver.get("kind") != "UniqueWrite" or restore_receiver.get("escapes") is not False:
        raise SystemExit("restore receiver borrow mismatch")
    moves = restore.get("argument_moves", [])
    if len(moves) != 1:
        raise SystemExit("restore must have one owned argument move")
    move = moves[0]
    if move.get("name") != "snapshot" or move.get("move_kind") != "ConsumeArgument":
        raise SystemExit("restore argument move mismatch")
    if move.get("deterministic_order_required") is not True:
        raise SystemExit("restore snapshot argument must require deterministic order")
    if move.get("drop_fact") != "TrivialMemory":
        raise SystemExit("restore snapshot argument drop fact mismatch")
    cleanup = restore.get("old_value_cleanup", {})
    if cleanup.get("required_fact") != "VariableContext.variable_map.drop_fact=TrivialMemory":
        raise SystemExit("restore cleanup fact mismatch")

    denied = {row["id"]: row for row in facts.get("denied_methods", [])}
    if denied.get("VariableContext::variable_map_mut", {}).get("deny_reason") != "ReturnedMutableBorrow":
        raise SystemExit("variable_map_mut must remain denied")

    excluded_consumers = set(facts.get("excluded_consumers", []))
    for name in ["CarrierInfo::from_variable_map", "CarrierInfo::with_explicit_carriers", "PHI planner integration"]:
        if name not in excluded_consumers:
            raise SystemExit(f"missing excluded consumer: {name}")

    plans = {row["id"]: row for row in plan.get("plans", [])}
    if set(plans) != {"VariableContext::snapshot", "VariableContext::restore"}:
        raise SystemExit("unexpected plan entries")
    snapshot_plan = plans["VariableContext::snapshot"]
    if snapshot_plan.get("plan_kind") != "CloneOwnedMap":
        raise SystemExit("snapshot plan kind mismatch")
    if snapshot_plan.get("result_plan") != "OwnedOrderedMap":
        raise SystemExit("snapshot result plan mismatch")
    for fact in [
        "receiver_borrow.kind=SharedRead",
        "receiver_borrow.escapes=false",
        "returns.deterministic_order_required=true",
        "returns.drop_fact=TrivialMemory",
    ]:
        if fact not in set(snapshot_plan.get("required_facts", [])):
            raise SystemExit(f"missing snapshot required fact: {fact}")

    restore_plan = plans["VariableContext::restore"]
    if restore_plan.get("plan_kind") != "ReplaceOwned":
        raise SystemExit("restore plan kind mismatch")
    if restore_plan.get("old_value_cleanup") != "erase":
        raise SystemExit("restore cleanup plan mismatch")
    for fact in [
        "receiver_borrow.kind=UniqueWrite",
        "receiver_borrow.escapes=false",
        "argument.move_kind=ConsumeArgument",
        "argument.deterministic_order_required=true",
        "VariableContext.variable_map.drop_fact=TrivialMemory",
    ]:
        if fact not in set(restore_plan.get("required_facts", [])):
            raise SystemExit(f"missing restore required fact: {fact}")

    behavior = plan.get("behavior", {})
    for name in [
        "general_resolver_implemented",
        "converter_emission_added",
        "rust_lifetime_syntax_added",
        "carrier_phi_claim",
        "full_variable_context_claim",
    ]:
        if behavior.get(name) is not False:
            raise SystemExit(f"unexpected behavior flag: {name}")

    oracle_vectors = oracle.get("vectors", [])
    ops = [op for vector in oracle_vectors for op in vector.get("operations", [])]
    for op in ["new", "insert", "snapshot", "restore", "len", "contains"]:
        if not any(item.get("op") == op for item in ops):
            raise SystemExit(f"missing oracle op: {op}")
    restore_ops = [item for item in ops if item.get("op") == "restore"]
    if not restore_ops:
        raise SystemExit("missing restore oracle op")
    restore_requires = set(restore_ops[0].get("requires", []))
    for requirement in ["ReplaceOwned", "old_map_cleanup=TrivialMemory"]:
        if requirement not in restore_requires:
            raise SystemExit(f"missing restore oracle requirement: {requirement}")

    scope = oracle.get("promotion_scope", {})
    if scope.get("hako_authority") != "VariableContext snapshot/restore only":
        raise SystemExit("unexpected promotion scope")
    for name in ["carrier_phi_claim", "full_variable_context_claim", "mirbuilder_wide_claim"]:
        if scope.get(name) is not False:
            raise SystemExit(f"unexpected oracle scope flag: {name}")

    denied_vectors = set(oracle.get("denied_vectors", []))
    for name in ["variable_map_mut_returned_borrow", "carrier_extraction", "phi_planner_integration"]:
        if name not in denied_vectors:
            raise SystemExit(f"missing denied oracle vector: {name}")


def build_hako() -> str:
    verified_ir = {
        "generated_by": "tools/rust_lifecycle/generate_variable_context_snapshot_restore_artifact.py",
        "artifact_manifest": "lang/generated/rust_derived/hakorune_mir_builder/variable_context_snapshot_restore.artifact.json",
        "family_comment": "hakorune_mir_builder::variable_context",
        "pilot_scope": SCOPE,
        "using_module": "apps.lib.collections.ordered_map",
        "box": {
            "name": "VariableContext",
            "field_name": "variable_map",
            "field_type": "OrderedMapBox",
            "initializer": "OrderedMap.create()",
        },
        "api": {
            "name": "VariableContextApi",
            "methods": [
                {
                    "signature": "snapshot(ctx)",
                    "body_lines": ["return ctx.variable_map.clone_owned()"],
                },
                {
                    "signature": "restore(ctx, snapshot)",
                    "body_lines": ["ctx.variable_map = snapshot.clone_owned()"],
                },
            ],
        },
        "main": {
            "lines": dedent(
                """
                local ctx = new VariableContext()
                local snapshot = VariableContextApi.snapshot(ctx)
                VariableContextApi.restore(ctx, snapshot)

                print("variable_context_snapshot_restore_derived_artifact=ok")
                return 0
                """
            ).strip("\n").splitlines(),
        },
    }
    return emit_verified_family_hako(verified_ir)


def build_manifest(hako_text: str) -> dict[str, Any]:
    return build_common_rust_derived_manifest(
        root=ROOT,
        family_id=FAMILY_ID,
        pilot_scope=SCOPE,
        state="DerivedShadow",
        source_rust_file=ROOT / "crates/hakorune_mir_builder/src/variable_context.rs",
        generator_tool="tools/rust_lifecycle/generate_variable_context_snapshot_restore_artifact.py",
        generator_version="variable-context-snapshot-restore-derived-artifact-v0",
        hako_path=HAKO,
        hako_text=hako_text,
        claims={
            "generated_hako_manual_edit": 0,
            "mainline_selected": 0,
            "full_variable_context_claim": 0,
            "rust_bootstrap_retained": 1,
            "backend_behavior_changed": 0,
            "source_selfhost_claim": 0,
        },
        inputs=build_common_rust_derived_inputs(
            root=ROOT,
            facts=FACTS,
            plan=PLAN,
            oracle=ORACLE,
        ),
        extra_fields={"excluded_methods": EXCLUDED},
    )


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--check", action="store_true", help="fail if generated files differ")
    args = parser.parse_args()

    hako_text = build_hako()
    manifest_text = stable_json(build_manifest(hako_text))

    run_validated_family_generator(
        check=args.check,
        root=ROOT,
        unchanged_label="generated_variable_context_snapshot_restore_artifact=unchanged",
        load_facts=lambda: read_json(FACTS),
        plan_path=PLAN,
        oracle_path=ORACLE,
        validate_inputs=validate_inputs,
        outputs_factory=lambda: [
            (HAKO, hako_text),
            (MANIFEST, manifest_text),
        ],
    )


if __name__ == "__main__":
    main()
