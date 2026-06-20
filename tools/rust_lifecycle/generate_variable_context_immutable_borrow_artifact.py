#!/usr/bin/env python3
"""Generate the focused VariableContext immutable BorrowView Hako artifact.

This 1522 pilot is narrower than full VariableContext. It emits only the
read-only owner-carrying BorrowView surface for VariableContext::variable_map
and keeps returned mutable borrow, snapshot/restore, and carrier-sensitive
behavior excluded.
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

FACTS = FIXTURES / "variable-context-immutable-borrow-facts-v0.json"
PLAN = FIXTURES / "variable-context-immutable-borrow-plan-v0.json"
ORACLE = FIXTURES / "variable-context-immutable-borrow-oracle-vectors-v0.json"
HAKO = OUT_DIR / "variable_context_immutable_borrow.hako"
MANIFEST = OUT_DIR / "variable_context_immutable_borrow.artifact.json"

SUBJECT = "hakorune_mir_builder::variable_context::VariableContext.immutable_map_borrow"
FAMILY_ID = "hakorune_mir_builder::variable_context"
SCOPE = "VariableContext_immutable_borrow_only"
METHOD = "VariableContext::variable_map"
EXCLUDED = [
    "VariableContext::variable_map_mut",
    "VariableContext::snapshot",
    "VariableContext::restore",
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

    method_facts = facts.get("method_facts", [])
    if len(method_facts) != 1:
        raise SystemExit("expected one immutable borrow method fact")

    method = method_facts[0]
    if method.get("id") != METHOD:
        raise SystemExit("unexpected method fact id")
    borrow = method.get("receiver_borrow", {})
    if borrow.get("kind") != "SharedRead":
        raise SystemExit("expected SharedRead borrow")
    if borrow.get("scope") != "ReturnedBorrow":
        raise SystemExit("expected ReturnedBorrow scope")
    if borrow.get("escapes") is not False:
        raise SystemExit("borrow escape must be denied")
    if borrow.get("owner_carrying_required") is not True:
        raise SystemExit("owner-carrying borrow required")

    returns = method.get("returns", {})
    if returns.get("borrow_view") != "OwnerCarryingBorrowView":
        raise SystemExit("unexpected borrow_view spelling")
    if returns.get("access") != "read":
        raise SystemExit("unexpected borrow access")

    denied = {row["id"]: row for row in facts.get("denied_methods", [])}
    for name, reason in {
        "VariableContext::variable_map_mut": "ReturnedMutableBorrow",
        "VariableContext::snapshot": "SnapshotOwnedMapOutOfScope",
        "VariableContext::restore": "ReplaceOwnedOutOfScope",
    }.items():
        if denied.get(name, {}).get("deny_reason") != reason:
            raise SystemExit(f"missing denied method fact: {name}")

    excluded_consumers = {row["id"] for row in facts.get("excluded_consumers", [])}
    for name in ["CarrierInfo::from_variable_map", "CarrierInfo::with_explicit_carriers"]:
        if name not in excluded_consumers:
            raise SystemExit(f"missing excluded consumer: {name}")

    plans = {row["id"]: row for row in plan.get("plans", [])}
    borrow_plan = plans.get(METHOD)
    if borrow_plan is None:
        raise SystemExit("missing borrow plan")
    if borrow_plan.get("plan_kind") != "BorrowView":
        raise SystemExit("borrow plan kind must be BorrowView")
    if borrow_plan.get("access") != "read":
        raise SystemExit("borrow plan access must be read")
    if borrow_plan.get("owner_carrying") is not True:
        raise SystemExit("borrow plan must be owner_carrying")
    if borrow_plan.get("escape_policy") != "deny_if_escapes":
        raise SystemExit("borrow plan must deny escape")
    if borrow_plan.get("return_alias_policy") != "owner_carrying_view_only":
        raise SystemExit("borrow plan return alias policy mismatch")

    required = set(borrow_plan.get("required_facts", []))
    for fact in [
        "receiver_borrow.kind=SharedRead",
        "receiver_borrow.scope=ReturnedBorrow",
        "receiver_borrow.escapes=false",
        "receiver_borrow.owner_carrying_required=true",
    ]:
        if fact not in required:
            raise SystemExit(f"missing required fact: {fact}")

    denied_plan = set(plan.get("denied", []))
    for name in EXCLUDED + ["CarrierInfo::from_variable_map", "CarrierInfo::with_explicit_carriers", "PHI planner integration"]:
        if name not in denied_plan:
            raise SystemExit(f"missing denied plan boundary: {name}")

    behavior = plan.get("behavior", {})
    if behavior.get("general_resolver_implemented") is not False:
        raise SystemExit("general resolver must remain disabled")
    if behavior.get("converter_emission_added") is not False:
        raise SystemExit("converter emission must remain disabled")
    if behavior.get("rust_lifetime_syntax_added") is not False:
        raise SystemExit("rust lifetime syntax must remain disabled")
    if behavior.get("carrier_phi_claim") is not False:
        raise SystemExit("carrier PHI claim must remain false")
    if behavior.get("full_variable_context_claim") is not False:
        raise SystemExit("full VariableContext claim must remain false")

    oracle_vectors = oracle.get("vectors", [])
    if not any(
        op.get("op") == "borrow_view" and op.get("method") in {"variable_map", METHOD}
        for vector in oracle_vectors
        for op in vector.get("operations", [])
    ):
        raise SystemExit("missing borrow_view oracle op")
    if not any(op.get("op") == "borrow_get" for vector in oracle_vectors for op in vector.get("operations", [])):
        raise SystemExit("missing borrow_get oracle op")
    if not any(op.get("op") == "borrow_len" for vector in oracle_vectors for op in vector.get("operations", [])):
        raise SystemExit("missing borrow_len oracle op")
    if not any(op.get("op") == "borrow_iteration_order" for vector in oracle_vectors for op in vector.get("operations", [])):
        raise SystemExit("missing borrow_iteration_order oracle op")

    denied_vectors = set(oracle.get("denied_vectors", []))
    for name in ["variable_map_mut_returned_borrow", "snapshot", "restore", "carrier_extraction", "phi_planner_integration"]:
        if name not in denied_vectors:
            raise SystemExit(f"missing denied oracle vector: {name}")

    scope = oracle.get("promotion_scope", {})
    if scope.get("hako_authority") != "VariableContext immutable map borrow only":
        raise SystemExit("unexpected oracle promotion scope")
    if scope.get("carrier_phi_claim") is not False:
        raise SystemExit("oracle carrier phi claim must be false")
    if scope.get("full_variable_context_claim") is not False:
        raise SystemExit("oracle full variable context claim must be false")
    if scope.get("mirbuilder_wide_claim") is not False:
        raise SystemExit("oracle mirbuilder-wide claim must be false")


def build_hako() -> str:
    verified_ir = {
        "generated_by": "tools/rust_lifecycle/generate_variable_context_immutable_borrow_artifact.py",
        "artifact_manifest": "lang/generated/rust_derived/hakorune_mir_builder/variable_context_immutable_borrow.artifact.json",
        "family_comment": "hakorune_mir_builder::variable_context",
        "pilot_scope": SCOPE,
        "using_module": "apps.lib.collections.ordered_map",
        "box": {
            "name": "VariableContext",
            "field_name": "variable_map",
            "field_type": "OrderedMapBox",
            "initializer": "OrderedMap.create()",
        },
        "static_boxes": [
            {
                "name": "VariableContextApi",
                "methods": [
                    {
                        "signature": "variable_map(ctx)",
                        "body_lines": ["return ctx.variable_map"],
                    }
                ],
            },
            {
                "name": "VariableMapViewApi",
                "methods": [
                    {
                        "signature": "is_empty(view): i64",
                        "body_lines": dedent(
                            """
                            if view.length() == 0 {
                                return 1
                            }
                            return 0
                            """
                        ).strip("\n").splitlines(),
                    },
                    {
                        "signature": "len(view): i64",
                        "body_lines": ["return view.length()"],
                    },
                    {
                        "signature": "contains(view, name): i64",
                        "body_lines": dedent(
                            """
                            if view.has(name) == true {
                                return 1
                            }
                            return 0
                            """
                        ).strip("\n").splitlines(),
                    },
                    {
                        "signature": "lookup(view, name)",
                        "body_lines": ["return view.get(name)"],
                    },
                ],
            },
        ],
        "main": {
            "lines": dedent(
                """
                local ctx = new VariableContext()
                local view = VariableContextApi.variable_map(ctx)
                if VariableMapViewApi.is_empty(view) != 1 {
                    print("variable_context_borrow_view_empty=fail")
                    return 1
                }
                if VariableMapViewApi.len(view) != 0 {
                    print("variable_context_borrow_view_len=fail")
                    return 2
                }
                if VariableMapViewApi.contains(view, "x") != 0 {
                    print("variable_context_borrow_view_contains=fail")
                    return 3
                }
                if VariableMapViewApi.lookup(view, "x") != null {
                    print("variable_context_borrow_view_lookup=fail")
                    return 4
                }

                print("variable_context_immutable_borrow_derived_artifact=ok")
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
        generator_tool="tools/rust_lifecycle/generate_variable_context_immutable_borrow_artifact.py",
        generator_version="variable-context-immutable-borrow-derived-artifact-v0",
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
        unchanged_label="generated_variable_context_immutable_borrow_artifact=unchanged",
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
