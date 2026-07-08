#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
FIXTURE="$ROOT/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-carrier-type-scalar-known-map-load-i64-typed-direct-closeout-contract-basis-v0.json"
TOOL="$ROOT/tools/rust_lifecycle/mirbuilder_carrier_type_scalar_known_map_load_i64_typed_direct_closeout_contract_basis.py"
CARD="$ROOT/docs/development/current/main/phases/phase-296x/2098-MIRBUILDER-CARRIER-TYPE-SCALAR-KNOWN-MAP-LOAD-I64-TYPED-DIRECT-CLOSEOUT-CONTRACT-BASIS-001.md"
TASK_ORDER="$ROOT/docs/development/current/main/design/mirbuilder-rust-to-hako-converter-task-order-ssot.md"
MANIFEST="$ROOT/docs/development/current/main/design/fixtures/rust-lifecycle/source-selfhost-family-guard-manifest-v0.json"

python3 "$TOOL" --check

python3 - "$FIXTURE" "$CARD" "$TASK_ORDER" "$MANIFEST" "$ROOT" <<'PY'
import json
import sys
from pathlib import Path

fixture = json.load(open(sys.argv[1], encoding="utf-8"))
card = Path(sys.argv[2]).read_text(encoding="utf-8")
task_order = Path(sys.argv[3]).read_text(encoding="utf-8")
manifest = json.load(open(sys.argv[4], encoding="utf-8"))
root = Path(sys.argv[5])


def need(cond, msg):
    if not cond:
        raise SystemExit(msg)


token = "MIRBUILDER-CARRIER-TYPE-SCALAR-KNOWN-MAP-LOAD-I64-TYPED-DIRECT-CLOSEOUT-CONTRACT-BASIS-001"
next_card = "MIRBUILDER-CARRIER-TYPE-TRANSPORT-REMAINING-AXIS-COMPONENT-REQUIREMENT-RERUN-003"
design_stop = "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001"

need(fixture.get("kind") == "MirBuilderCarrierTypeScalarKnownMapLoadI64TypedDirectCloseoutContractBasisV1", "bad kind")
need(fixture.get("token") == token, "bad token")
need(token in card, "card missing token")

inputs = fixture.get("input_state") or {}
need(inputs.get("current_blocker") == design_stop, "blocker drift")
need(inputs.get("previous_rerun_token") == "MIRBUILDER-CARRIER-TYPE-TRANSPORT-REMAINING-AXIS-COMPONENT-REQUIREMENT-RERUN-002", "previous rerun token drift")

previous = fixture.get("previous_state") or {}
need(previous.get("accepted_component_evidence_source_count") == 0, "previous evidence count drift")
need(previous.get("root_component_requirement_count") == 0, "previous root count drift")
need(previous.get("selection_eligible_component_requirement_count") == 0, "previous eligible count drift")
need(previous.get("decision") == "KeepStopped", "previous decision drift")
need(previous.get("selected_next_card") == design_stop, "previous next drift")
need(previous.get("source_inventory_typed_direct_closeout_contract_count") == 0, "source inventory count drift")

target = fixture.get("target") or {}
need(target.get("component_requirement") == "ScalarKnownCloseoutAuthority", "target requirement drift")
need(target.get("candidate_axis") == "ScalarKnownTransportAxis", "candidate axis drift")
need(target.get("accepted_source_kind") == "TypedDirectCloseoutContract", "source kind drift")
need(target.get("target_requirement_acceptance_claim") == 0, "target acceptance must not be claimed")

contract = fixture.get("contract") or {}
need(contract.get("contract_id") == "MapLoadScalarI64ScalarKnownTypedDirectCloseoutContract", "contract id drift")
need(contract.get("route_kind") == "MapLoadScalarI64", "route kind drift")
need(contract.get("return_shape") == "ScalarI64OrMissingZero", "return shape drift")
need(contract.get("proof_function") == "prove_scalar_i64_map_get_store_fact", "proof function drift")
need(contract.get("value_demand") == "ScalarI64", "value demand drift")
need(contract.get("publication_policy") == "NoPublication", "publication policy drift")
need(contract.get("all_rows_join_contract") is True, "narrow contract rows must join")
need(contract.get("no_carrier_boundary_required_or_already_covered") is True, "carrier boundary contract drift")

rule = fixture.get("selection_rule") or {}
for key in [
    "basis_only",
    "rerun_required_before_selection",
    "direct_component_selection_from_zero_root_forbidden",
]:
    need(rule.get(key) is True, f"rule drift: {key}")
for key in [
    "component_specific_card_selection",
    "concrete_carrier_type_axis_selection",
    "source_path_as_authority",
    "owner_name_as_proof",
    "row_count_as_proof",
    "route_membership_alone_as_proof",
]:
    need(rule.get(key) is False, f"forbidden rule drift: {key}")

claims = fixture.get("claims") or {}
for key in [
    "typed_direct_closeout_contract_basis",
    "map_load_scalar_i64_existing_rust_owner_evidence",
    "scalar_i64_or_missing_zero_return_shape_evidence",
    "scalar_i64_value_demand_evidence",
    "no_publication_policy_evidence",
    "basis_only",
    "rerun_required_before_component_selection",
]:
    need(claims.get(key) == 1, f"missing positive claim: {key}")
for key in [
    "scalar_known_transport_axis_closeout",
    "scalar_known_closeout_authority_accepted_root",
    "target_requirement_acceptance_claim",
    "root_component_requirement_selected",
    "component_specific_card_selection",
    "concrete_carrier_type_axis_selection",
    "source_selfhost_claim",
    "native_seed_materialization",
    "hako_generation",
    "hako_adopted_decision",
    "runtime_fallback",
    "new_backend_route",
    "new_abi",
    "new_python_semantic_projector",
    "source_path_as_authority",
    "owner_name_as_proof",
    "row_count_as_proof",
    "route_membership_alone_as_proof",
    "return_type_string_mapping_as_proof",
    "observed_subaxis_set_as_proof",
    "hardcoded_carrier_axis_priority",
    "manual_axis_selection",
    "manual_carrier_selection",
]:
    need(claims.get(key) == 0, f"forbidden claim drift: {key}")

decision = fixture.get("decision") or {}
need(decision.get("kind") == "SelectCarrierTypeRemainingAxisComponentRequirementRerun003", "decision kind drift")
need(decision.get("selected_next_card") == next_card, "next card drift")
need(decision.get("selected_carrier_type_axis") is None, "axis must not be selected")
need(decision.get("selected_component_requirement") is None, "component must not be selected")

rows_by_token = {row.get("token"): row for row in manifest.get("rows") or []}
manifest_row = rows_by_token.get(token) or {}
need(manifest_row.get("card", "").endswith("2098-MIRBUILDER-CARRIER-TYPE-SCALAR-KNOWN-MAP-LOAD-I64-TYPED-DIRECT-CLOSEOUT-CONTRACT-BASIS-001.md"), "manifest card drift")
need(manifest_row.get("fixture", "").endswith("mirbuilder-carrier-type-scalar-known-map-load-i64-typed-direct-closeout-contract-basis-v0.json"), "manifest fixture drift")
need(manifest_row.get("legacy_guard", "").endswith("rust_lifecycle_mirbuilder_carrier_type_scalar_known_map_load_i64_typed_direct_closeout_contract_basis_guard.sh"), "manifest guard drift")

source_expectations = {
    "src/mir/generic_method_route_plan/map_set_scalar_proof.rs": [
        "prove_scalar_i64_map_get_store_fact",
        "MapSetScalarI64DominatesNoEscape",
    ],
    "src/mir/generic_method_route_plan/collection_read_routes.rs": [
        "MapLoadScalarI64",
        "ScalarI64OrMissingZero",
        "GenericMethodValueDemand::ScalarI64",
        "GenericMethodPublicationPolicy::NoPublication",
    ],
    "src/mir/generic_method_route_plan/tests/scalar_proof.rs": [
        "MapLoadScalarI64",
        "ScalarI64OrMissingZero",
        "GenericMethodValueDemand::ScalarI64",
        "GenericMethodPublicationPolicy::NoPublication",
    ],
}
for rel_path, tokens in source_expectations.items():
    text = (root / rel_path).read_text(encoding="utf-8")
    for expected in tokens:
        need(expected in text, f"missing source token {expected} in {rel_path}")

need(token in task_order, "task order missing token")
need(f"selected_next_card={next_card}" in task_order, "task order next drift")

print("output_contract=rust-lifecycle-mirbuilder-carrier-type-scalar-known-map-load-i64-typed-direct-closeout-contract-basis")
print("typed_direct_closeout_contract_basis=1")
print("basis_only=1")
print("component_specific_card_selection=0")
print("concrete_carrier_type_axis_selection=0")
print("selected_next_card=" + next_card)
print("source_selfhost_claim=0")
print("summary=ok")
PY
