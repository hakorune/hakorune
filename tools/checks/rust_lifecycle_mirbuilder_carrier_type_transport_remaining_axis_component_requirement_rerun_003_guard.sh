#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
FIXTURE="$ROOT/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-carrier-type-transport-remaining-axis-component-requirement-rerun-003-v0.json"
TOOL="$ROOT/tools/rust_lifecycle/mirbuilder_carrier_type_transport_remaining_axis_component_requirement_rerun_003.py"
CARD="$ROOT/docs/development/current/main/phases/phase-296x/2099-MIRBUILDER-CARRIER-TYPE-TRANSPORT-REMAINING-AXIS-COMPONENT-REQUIREMENT-RERUN-003.md"
TASK_ORDER="$ROOT/docs/development/current/main/design/mirbuilder-rust-to-hako-converter-task-order-ssot.md"
MANIFEST="$ROOT/docs/development/current/main/design/fixtures/rust-lifecycle/source-selfhost-family-guard-manifest-v0.json"

python3 "$TOOL" --check

python3 - "$FIXTURE" "$CARD" "$TASK_ORDER" "$MANIFEST" <<'PY'
import json
import sys
from pathlib import Path

fixture = json.load(open(sys.argv[1], encoding="utf-8"))
card = Path(sys.argv[2]).read_text(encoding="utf-8")
task_order = Path(sys.argv[3]).read_text(encoding="utf-8")
manifest = json.load(open(sys.argv[4], encoding="utf-8"))


def need(cond, msg):
    if not cond:
        raise SystemExit(msg)


token = "MIRBUILDER-CARRIER-TYPE-TRANSPORT-REMAINING-AXIS-COMPONENT-REQUIREMENT-RERUN-003"
selected_next = "MIRBUILDER-CARRIER-TYPE-SCALAR-KNOWN-TRANSPORT-CLOSEOUT-BASIS-001"
design_stop = "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001"

need(fixture.get("kind") == "MirBuilderCarrierTypeTransportRemainingAxisComponentRequirementRerunV3", "bad kind")
need(fixture.get("token") == token, "bad token")
need(token in card, "card missing token")

inputs = fixture.get("input_state") or {}
need(inputs.get("current_blocker") == design_stop, "blocker drift")
need(inputs.get("previous_rerun", "").endswith("mirbuilder-carrier-type-transport-remaining-axis-component-requirement-rerun-002-v0.json"), "previous rerun drift")
need(inputs.get("scalar_known_map_load_i64_typed_direct_closeout_contract_basis", "").endswith("mirbuilder-carrier-type-scalar-known-map-load-i64-typed-direct-closeout-contract-basis-v0.json"), "closeout basis input drift")

rule = fixture.get("selector_rule") or {}
need(rule.get("name") == "CarrierTypeRemainingAxisComponentRequirementSelectorV3", "selector drift")
need(rule.get("selection_requires_exactly_one_root_component_requirement") is True, "exactly-one rule drift")
need(rule.get("component_specific_card_selection_allowed_if_exactly_one_root_requirement") is True, "component selection rule drift")
need(rule.get("concrete_carrier_type_axis_selection") is False, "axis selection rule drift")
need(rule.get("tie_breaking_forbidden") is True, "tie-break rule drift")
need(rule.get("basis_source_materialization_required") is True, "basis materialization rule drift")

rows = fixture.get("component_requirement_rows") or []
need(len(rows) == 7, "requirement count drift")
by_req = {row.get("requirement_id"): row for row in rows}
scalar = by_req.get("ScalarKnownCloseoutAuthority") or {}
need(scalar.get("candidate_axis") == "ScalarKnownTransportAxis", "scalar axis drift")
need(scalar.get("root_authority_status") == "Proven", "scalar root status drift")
need(scalar.get("root_authority_reason_token") == "MapLoadScalarI64TypedDirectCloseoutContractAccepted", "scalar reason drift")
need(scalar.get("proof_tuple_complete") is True, "scalar proof tuple drift")
need(scalar.get("selection_eligible") is True, "scalar selection drift")
sources = scalar.get("accepted_sources") or []
need(len(sources) == 1, "scalar accepted source count drift")
source = sources[0]
need(source.get("source_kind") == "TypedDirectCloseoutContract", "source kind drift")
need(source.get("closeout_contract_id") == "MapLoadScalarI64ScalarKnownTypedDirectCloseoutContract", "contract id drift")
need(source.get("route_kind") == "MapLoadScalarI64", "route kind drift")
need(source.get("return_shape") == "ScalarI64OrMissingZero", "return shape drift")
need(source.get("proof_function") == "prove_scalar_i64_map_get_store_fact", "proof function drift")
need(source.get("value_demand") == "ScalarI64", "value demand drift")
need(source.get("publication_policy") == "NoPublication", "publication policy drift")
need(source.get("all_rows_join_contract") is True, "all rows join drift")
need(source.get("no_carrier_boundary_required_or_already_covered") is True, "carrier boundary drift")
need(isinstance(source.get("proof_source_hash"), str) and len(source["proof_source_hash"]) == 64, "proof hash drift")

for requirement_id, row in by_req.items():
    if requirement_id == "ScalarKnownCloseoutAuthority":
        continue
    need(row.get("accepted_sources") == [], f"unexpected accepted source: {requirement_id}")
    need(row.get("root_authority_status") == "Unproven", f"unexpected root status: {requirement_id}")
    need(row.get("proof_tuple_complete") is False, f"unexpected proof tuple: {requirement_id}")
    need(row.get("selection_eligible") is False, f"unexpected selection: {requirement_id}")

summary = fixture.get("summary") or {}
need(summary.get("component_requirement_count") == 7, "summary requirement count drift")
need(summary.get("accepted_component_evidence_source_count") == 1, "summary accepted source drift")
need(summary.get("component_authority_source_count") == 1, "summary authority source drift")
need(summary.get("root_component_requirement_count") == 1, "summary root count drift")
need(summary.get("selection_eligible_component_requirement_count") == 1, "summary eligible drift")
need(summary.get("component_specific_card_selection_eligible_count") == 1, "summary component selection drift")
need(summary.get("concrete_carrier_type_axis_selection") == 0, "summary axis selection drift")

decision = fixture.get("decision") or {}
need(decision.get("kind") == "SelectComponentSpecificCard", "decision kind drift")
need(decision.get("reason_token") == "ExactlyOneCarrierTypeComponentRequirementRoot", "reason drift")
need(decision.get("selected_carrier_type_axis") is None, "axis must not be selected")
need(decision.get("selected_component_requirement") == "ScalarKnownCloseoutAuthority", "selected component drift")
need(decision.get("selected_next_card") == selected_next, "selected next drift")

claims = fixture.get("claims") or {}
for key in [
    "accepted_typed_direct_closeout_contract_materialized",
    "scalar_known_closeout_authority_accepted_root",
    "component_specific_card_selection",
]:
    need(claims.get(key) == 1, f"missing positive claim: {key}")
for key in [
    "concrete_carrier_type_axis_selection",
    "scalar_known_transport_axis_closeout",
    "source_selfhost_claim",
    "native_seed_materialization",
    "hako_generation",
    "hako_adopted_decision",
    "runtime_fallback",
    "new_backend_route",
    "new_abi",
    "new_python_semantic_projector",
    "manual_axis_selection",
    "manual_carrier_selection",
    "hardcoded_carrier_axis_priority",
    "row_count_as_proof",
    "source_path_as_authority",
    "owner_name_as_proof",
    "route_membership_alone_as_proof",
    "return_type_string_mapping_as_proof",
    "observed_subaxis_set_as_proof",
]:
    need(claims.get(key) == 0, f"forbidden claim drift: {key}")

rows_by_token = {row.get("token"): row for row in manifest.get("rows") or []}
manifest_row = rows_by_token.get(token) or {}
need(manifest_row.get("card", "").endswith("2099-MIRBUILDER-CARRIER-TYPE-TRANSPORT-REMAINING-AXIS-COMPONENT-REQUIREMENT-RERUN-003.md"), "manifest card drift")
need(manifest_row.get("fixture", "").endswith("mirbuilder-carrier-type-transport-remaining-axis-component-requirement-rerun-003-v0.json"), "manifest fixture drift")
need(manifest_row.get("legacy_guard", "").endswith("rust_lifecycle_mirbuilder_carrier_type_transport_remaining_axis_component_requirement_rerun_003_guard.sh"), "manifest guard drift")

need(token in task_order, "task order missing token")
need(f"selected_next_card={selected_next}" in task_order, "task order next drift")

print("output_contract=rust-lifecycle-mirbuilder-carrier-type-transport-remaining-axis-component-requirement-rerun-003")
print("accepted_component_evidence_source_count=1")
print("root_component_requirement_count=1")
print("selected_component_requirement=ScalarKnownCloseoutAuthority")
print("selected_next_card=" + selected_next)
print("concrete_carrier_type_axis_selection=0")
print("source_selfhost_claim=0")
print("summary=ok")
PY
