#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
FIXTURE="$ROOT/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-domain-object-id-return-type-reference-edge-derivation-basis-v0.json"
TOOL="$ROOT/tools/rust_lifecycle/mirbuilder_domain_object_id_return_type_reference_edge_derivation_basis.py"
CARD="$ROOT/docs/development/current/main/phases/phase-296x/2070-MIRBUILDER-DOMAIN-OBJECT-ID-RETURN-TYPE-REFERENCE-EDGE-DERIVATION-BASIS-001.md"
STATE="$ROOT/docs/development/current/main/CURRENT_STATE.toml"
TASK_ORDER="$ROOT/docs/development/current/main/design/mirbuilder-rust-to-hako-converter-task-order-ssot.md"

python3 "$TOOL" --check

python3 - "$FIXTURE" "$CARD" "$STATE" "$TASK_ORDER" <<'PY'
import json
import sys
import tomllib
from pathlib import Path

fixture = json.load(open(sys.argv[1], encoding="utf-8"))
card = Path(sys.argv[2]).read_text(encoding="utf-8")
state = tomllib.loads(Path(sys.argv[3]).read_text(encoding="utf-8"))
task_order = Path(sys.argv[4]).read_text(encoding="utf-8")


def need(cond, msg):
    if not cond:
        raise SystemExit(msg)


token = "MIRBUILDER-DOMAIN-OBJECT-ID-RETURN-TYPE-REFERENCE-EDGE-DERIVATION-BASIS-001"
next_card = "MIRBUILDER-DOMAIN-OBJECT-ID-RETURN-TYPE-RESOURCE-TAXONOMY-INVENTORY-001"
design_stop = "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001"

need(fixture.get("kind") == "MirBuilderDomainObjectIdReturnTypeReferenceEdgeDerivationBasisV1", "bad kind")
need(fixture.get("token") == token, "bad token")
need(token in card, "card missing token")
need(token in task_order, "task-order missing token")
need(next_card in task_order, "task-order missing next card")

inputs = fixture.get("input_state") or {}
need(inputs.get("current_blocker") == design_stop, "blocker drift")
need(inputs.get("typed_dependency_edge_evidence_inventory", "").endswith("mirbuilder-domain-object-id-typed-dependency-edge-evidence-inventory-v0.json"), "inventory input drift")

previous = fixture.get("previous_state") or {}
need(previous.get("selected_evidence_kind") == "ReturnTypeFieldReference", "previous selected evidence drift")
need(previous.get("return_type_field_reference_candidate_count") == 85, "previous return type count drift")
need(previous.get("accepted_edge_ready_count") == 0, "previous edge-ready drift")

basis = fixture.get("derivation_basis") or {}
need(basis.get("return_type_reference_is_dependency_edge_by_itself") is False, "return type edge proof drift")
need(basis.get("return_type_name_to_subaxis_map_allowed") is False, "name map drift")
need(basis.get("hardcoded_return_type_priority_allowed") is False, "hardcoded return type priority drift")
need(basis.get("derivation_requires_resource_taxonomy") is True, "taxonomy requirement drift")
need(basis.get("derivation_requires_dependent_and_prerequisite_roles") is True, "role requirement drift")
need(basis.get("derivation_requires_concrete_source_row_reference") is True, "source row requirement drift")

rule = fixture.get("accepted_derivation_rule") or {}
need(rule.get("name") == "ReturnTypeReferenceEdgeDerivationV1", "rule name drift")
need(rule.get("edge_direction") == "dependent_subaxis_requires_prerequisite_subaxis", "edge direction drift")
must_not = set(rule.get("must_not_use") or [])
for forbidden in ["return_type_name_prefix", "return_type_name_contains", "row_count", "owner_name", "source_path", "route_membership_alone"]:
    need(forbidden in must_not, f"missing forbidden derivation input: {forbidden}")

inventory = fixture.get("return_type_inventory") or {}
need(inventory.get("return_type_reference_count") == 85, "return type reference count drift")
need(inventory.get("distinct_return_type_count") == 44, "distinct return type count drift")
need(inventory.get("resource_taxonomy_entry_count") == 0, "taxonomy entry count drift")
need(inventory.get("edge_ready_return_type_count") == 0, "edge-ready return type count drift")

decision = fixture.get("decision") or {}
need(decision.get("kind") == "SelectReturnTypeResourceTaxonomyInventory", "decision kind drift")
need(decision.get("reason_token") == "ReturnTypeReferenceEdgeDerivationRequiresResourceTaxonomy", "reason drift")
need(decision.get("selected_domain_subaxis") is None, "subaxis must not be selected")
need(decision.get("selected_next_card") == next_card, "next card drift")

claims = fixture.get("claims") or {}
need(claims.get("return_type_reference_edge_derivation_basis_defined") == 1, "basis claim drift")
for key in [
    "return_type_reference_is_dependency_edge_by_itself",
    "return_type_name_to_subaxis_map_allowed",
    "hardcoded_return_type_priority_allowed",
    "source_selfhost_claim",
    "native_seed_materialization",
    "hako_generation",
    "hako_adopted_decision",
    "runtime_fallback",
    "new_backend_route",
    "new_abi",
    "new_python_semantic_projector",
    "runner_semantic_owner",
    "manual_family_selection",
    "manual_shape_selection",
    "manual_axis_selection",
    "manual_carrier_selection",
    "manual_subaxis_selection",
    "hardcoded_subaxis_priority",
    "row_count_as_proof",
    "domain_object_count_as_proof",
    "cluster_size_as_proof",
    "coverage_percentage_as_proof",
    "owner_name_as_proof",
    "source_path_as_authority",
    "route_membership_alone_as_proof",
    "convenience_as_proof",
]:
    need(claims.get(key) == 0, f"forbidden claim drift: {key}")

need(state.get("current_blocker_token") == design_stop, "CURRENT_STATE blocker drift")

print("output_contract=rust-lifecycle-mirbuilder-domain-object-id-return-type-reference-edge-derivation-basis")
print("return_type_reference_count=85")
print("distinct_return_type_count=44")
print("edge_ready_return_type_count=0")
print("selected_next_card=" + next_card)
print("source_selfhost_claim=0")
print("summary=ok")
PY
