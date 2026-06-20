#!/usr/bin/env python3
"""Fixture-only verifier for Rust-to-Hako lifecycle JSON records.

This tool intentionally reads checked-in fixtures only. It does not invoke
rustc, choose Hako lifecycle policy, emit .hako, or touch backend behavior.
"""

from __future__ import annotations

import argparse
import json
from pathlib import Path
from typing import Any


ROOT = Path(__file__).resolve().parents[2]
FIXTURES = ROOT / "docs/development/current/main/design/fixtures/rust-lifecycle"


class FixtureError(AssertionError):
    pass


def load_json(name: str) -> dict[str, Any]:
    path = FIXTURES / name
    if not path.exists():
        raise FixtureError(f"missing fixture: {path}")
    return json.loads(path.read_text())


def require(condition: bool, message: str) -> None:
    if not condition:
        raise FixtureError(message)


def by_id(rows: list[dict[str, Any]]) -> dict[str, dict[str, Any]]:
    return {row["id"]: row for row in rows}


def require_no_hako_policy_spelling(facts_name: str) -> None:
    text = (FIXTURES / facts_name).read_text()
    forbidden = [
        "OrderedMapBox",
        "BorrowView",
        "ReturnedMutableBorrow",
        "HakoLifecyclePlan",
        "LocalBox",
        "TransferOwned",
    ]
    for word in forbidden:
        require(word not in text, f"adapter facts contain Hako policy spelling: {word}")


def verify_common_result(
    result: dict[str, Any],
    subject: str,
    source_facts: str,
) -> None:
    require(result["schema_version"] == 0, "result schema_version")
    require(result["kind"] == "HakoLifecycleVerifierResult", "result kind")
    require(result["mode"] == "passive_adapter_fixture", "result mode")
    require(result["subject"] == subject, "result subject")
    require(result["source_facts"] == source_facts, "result source_facts")
    require(result["result"] == "VerifiedPlan", "result must be VerifiedPlan")

    claims = result["claims"]
    for flag in [
        "emission_allowed",
        "verifier_implementation_started",
        "emitter_implementation_started",
        "converter_core_changed",
        "backend_behavior_changed",
        "mirbuilder_wide_lifecycle",
    ]:
        require(claims[flag] is False, f"claim must stay false: {flag}")


def verify_target_neutral(facts: dict[str, Any]) -> None:
    target = facts["target_neutral"]
    require(target["hako_policy_owner"] is False, "hako_policy_owner")
    require(target["hako_plan_kind_spelling_allowed"] is False, "plan spelling")
    require(target["rendering_instruction_allowed"] is False, "rendering")
    require(target["rustc_toolchain_invoked"] is False, "rustc invoked")


def verify_binding_context() -> None:
    facts_name = "binding-context-adapter-facts-v0.json"
    result_name = "binding-context-adapter-verifier-result-v0.json"
    facts = load_json(facts_name)
    plan = load_json("binding-context-plan-v0.json")
    result = load_json(result_name)

    require_no_hako_policy_spelling(facts_name)
    require(facts["kind"] == "RustLifecycleAdapterFacts", "facts kind")
    require(facts["subject"] == plan["subject"], "facts/plan subject")
    verify_target_neutral(facts)
    verify_common_result(result, facts["subject"], facts_name)
    require(result["source_plan"] == "binding-context-plan-v0.json", "source plan")

    fields = by_id(facts["fields"])
    field = fields["BindingContext.binding_map"]
    require(field["deterministic_order_required"] is True, "deterministic order")
    require(field["drop_class"] == "TrivialMemory", "map trivial drop")

    plans = by_id(plan["plans"])
    map_plan = plans["BindingContext.binding_map"]
    require(map_plan["plan_kind"] == "OrderedMapBox", "existing map plan")
    require(
        "BindingContext.binding_map.deterministic_order_required=true"
        in map_plan["required_facts"],
        "plan requires deterministic order",
    )

    verified = set(result["verified_facts"])
    for item in [
        "BindingContext.binding_map.deterministic_order_required=true",
        "BindingContext::is_empty.receiver.borrow_kind=SharedRead",
        "BindingContext::insert.receiver.borrow_kind=UniqueWrite",
        "BindingContext::clear_for_function_entry.receiver.borrow_escape=CallOnly",
    ]:
        require(item in verified, f"missing verified fact: {item}")

    denied = set(result["denied_boundaries"])
    require("lifecycle-aware converter emission" in denied, "emission denied")
    require("general verifier implementation" in denied, "impl denied")


def verify_variable_context() -> None:
    facts_name = "variable-context-adapter-facts-v0.json"
    result_name = "variable-context-adapter-verifier-result-v0.json"
    facts = load_json(facts_name)
    result = load_json(result_name)
    plans = {
        name: load_json(name)
        for name in [
            "variable-context-simple-map-plan-v0.json",
            "variable-context-immutable-borrow-plan-v0.json",
            "variable-context-snapshot-restore-plan-v0.json",
            "variable-context-carrier-snapshot-plan-v0.json",
        ]
    }

    require_no_hako_policy_spelling(facts_name)
    require(facts["kind"] == "RustLifecycleAdapterFacts", "facts kind")
    verify_target_neutral(facts)
    verify_common_result(result, facts["subject"], facts_name)
    require(set(result["source_plans"]) == set(plans), "source plans")

    fields = by_id(facts["fields"])
    field = fields["VariableContext.variable_map"]
    require(field["deterministic_order_required"] is True, "deterministic order")
    require(field["drop_class"] == "TrivialMemory", "map trivial drop")

    methods = by_id(facts["methods"])
    require(methods["VariableContext::variable_map"]["receiver"]["borrow_escape"] == "Returned", "immutable returned borrow")
    require(methods["VariableContext::variable_map"]["returned_reference"]["mutation_allowed"] is False, "immutable mutation")
    require(methods["VariableContext::variable_map_mut"]["receiver"]["borrow_kind"] == "UniqueWrite", "mutable borrow kind")
    require(methods["VariableContext::variable_map_mut"]["returned_reference"]["mutation_allowed"] is True, "mutable mutation")
    require(methods["VariableContext::snapshot"]["ownership_effect"] == "CloneOwnedMap", "snapshot ownership")
    require(methods["VariableContext::restore"]["ownership_effect"] == "ReplaceOwned", "restore ownership")

    verified = set(result["verified_facts"])
    for item in [
        "VariableContext.variable_map.deterministic_order_required=true",
        "VariableContext::variable_map.returned_reference.mutation_allowed=false",
        "VariableContext::variable_map_mut.returned_reference.mutation_allowed=true",
        "VariableContext::snapshot.ownership_effect=CloneOwnedMap",
        "VariableContext::restore.ownership_effect=ReplaceOwned",
        "CarrierInfo::from_variable_map.required_access=ReadOnly",
    ]:
        require(item in verified, f"missing verified fact: {item}")

    surfaces = {row["surface"] for row in result["verified_plan_surfaces"]}
    for surface in ["simple_map", "immutable_map_borrow", "snapshot_restore", "carrier_snapshot"]:
        require(surface in surfaces, f"missing surface: {surface}")

    denied = set(result["denied_boundaries"])
    require("VariableContext::variable_map_mut emitted as naked alias" in denied, "mutable alias denied")
    require("full VariableContext parity" in denied, "full parity denied")
    require("MirBuilder-wide lifecycle parity" in denied, "wide parity denied")


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument(
        "--case",
        choices=["binding-context", "variable-context", "all"],
        default="all",
    )
    args = parser.parse_args()

    if args.case in ("binding-context", "all"):
        verify_binding_context()
    if args.case in ("variable-context", "all"):
        verify_variable_context()

    print("summary=ok")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
