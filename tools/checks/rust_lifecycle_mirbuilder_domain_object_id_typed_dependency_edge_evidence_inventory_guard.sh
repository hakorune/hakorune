#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
FIXTURE="$ROOT/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-domain-object-id-typed-dependency-edge-evidence-inventory-v0.json"
TOOL="$ROOT/tools/rust_lifecycle/mirbuilder_domain_object_id_typed_dependency_edge_evidence_inventory.py"
CARD="$ROOT/docs/development/current/main/phases/phase-296x/2069-MIRBUILDER-DOMAIN-OBJECT-ID-TYPED-DEPENDENCY-EDGE-EVIDENCE-INVENTORY-001.md"
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


token = "MIRBUILDER-DOMAIN-OBJECT-ID-TYPED-DEPENDENCY-EDGE-EVIDENCE-INVENTORY-001"
next_card = "MIRBUILDER-DOMAIN-OBJECT-ID-RETURN-TYPE-REFERENCE-EDGE-DERIVATION-BASIS-001"
design_stop = "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001"

need(fixture.get("kind") == "MirBuilderDomainObjectIdTypedDependencyEdgeEvidenceInventoryV1", "bad kind")
need(fixture.get("token") == token, "bad token")
need(token in card, "card missing token")
need(token in task_order, "task-order missing token")
need(next_card in task_order, "task-order missing next card")

inputs = fixture.get("input_state") or {}
need(inputs.get("current_blocker") == design_stop, "blocker drift")
need(inputs.get("unresolved_subaxis_priority_rerun_003", "").endswith("mirbuilder-domain-object-id-unresolved-subaxis-priority-rerun-003-v0.json"), "rerun 003 input drift")

previous = fixture.get("previous_state") or {}
need(previous.get("rerun_003_decision") == "KeepStopped", "previous decision drift")
need(previous.get("rerun_003_reason_token") == "NoMachineDerivedDomainObjectIdTypedDependencyRootAuthority", "previous reason drift")
need(previous.get("accepted_typed_dependency_edge_count") == 0, "previous edge count drift")
need(previous.get("dependency_root_candidate_count") == 0, "previous root count drift")

summary = fixture.get("summary") or {}
need(summary.get("unresolved_non_id_domain_row_count") == 85, "unresolved row count drift")
need(summary.get("evidence_kind_count") == 6, "evidence kind count drift")
need(summary.get("direct_source_field_evidence_kind_count") == 1, "direct source field count drift")
need(summary.get("selected_evidence_kind_count") == 1, "selected evidence kind count drift")
need(summary.get("return_type_field_reference_candidate_count") == 85, "return type count drift")
need(summary.get("policy_decision_payload_pattern_count") == 1, "policy payload pattern count drift")
need(summary.get("accepted_edge_ready_count") == 0, "accepted edge ready count drift")
need(summary.get("selected_evidence_kind") == "ReturnTypeFieldReference", "selected evidence kind drift")

sources = {row.get("evidence_kind"): row for row in fixture.get("evidence_source_inventory") or []}
expected = {
    "ReturnTypeFieldReference",
    "ParameterTypeReference",
    "ConstructedDomainObjectReference",
    "VerifierInputContractReference",
    "PolicyDecisionPayloadReference",
    "FixtureDeclaredSemanticResourceDependency",
}
need(set(sources) == expected, "evidence source set drift")
rt = sources["ReturnTypeFieldReference"]
need(rt.get("direct_source_field_present") is True, "return_type field should be direct")
need(rt.get("source_field") == "return_type", "return_type source field drift")
need(rt.get("candidate_reference_count") == 85, "return_type candidate drift")
need(rt.get("concrete_typed_reference_count") == 85, "return_type concrete ref drift")
need(rt.get("accepted_edge_ready_count") == 0, "return_type must not be edge-ready")
need(rt.get("selected_for_derivation_basis") is True, "return_type should be selected for derivation basis")
need("ReturnTypeReferenceIsNotDependencyEdgeByItself" in (rt.get("blocked_by") or []), "missing return type blocker")

for kind, row in sources.items():
    if kind != "ReturnTypeFieldReference":
        need(row.get("selected_for_derivation_basis") is False, f"unexpected selected kind: {kind}")

decision = fixture.get("decision") or {}
need(decision.get("kind") == "SelectReturnTypeReferenceEdgeDerivationBasis", "decision kind drift")
need(decision.get("reason_token") == "ReturnTypeFieldReferenceIsOnlyConcreteLedgerEvidenceSource", "reason drift")
need(decision.get("selected_evidence_kind") == "ReturnTypeFieldReference", "selected evidence drift")
need(decision.get("selected_domain_subaxis") is None, "subaxis must not be selected")
need(decision.get("selected_next_card") == next_card, "next card drift")

claims = fixture.get("claims") or {}
need(claims.get("typed_dependency_edge_evidence_inventory") == 1, "inventory claim drift")
for key in [
    "return_type_field_as_edge_by_itself",
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

print("output_contract=rust-lifecycle-mirbuilder-domain-object-id-typed-dependency-edge-evidence-inventory")
print("return_type_field_reference_candidate_count=85")
print("accepted_edge_ready_count=0")
print("selected_next_card=" + next_card)
print("source_selfhost_claim=0")
print("summary=ok")
PY
