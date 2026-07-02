#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
FIXTURE="$ROOT/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-domain-object-id-typed-dependency-root-authority-basis-v0.json"
TOOL="$ROOT/tools/rust_lifecycle/mirbuilder_domain_object_id_typed_dependency_root_authority_basis.py"
CARD="$ROOT/docs/development/current/main/phases/phase-296x/2067-MIRBUILDER-DOMAIN-OBJECT-ID-TYPED-DEPENDENCY-ROOT-AUTHORITY-BASIS-001.md"
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


token = "MIRBUILDER-DOMAIN-OBJECT-ID-TYPED-DEPENDENCY-ROOT-AUTHORITY-BASIS-001"
rerun_003 = "MIRBUILDER-DOMAIN-OBJECT-ID-UNRESOLVED-SUBAXIS-PRIORITY-RERUN-003"
design_stop = "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001"

need(fixture.get("kind") == "MirBuilderDomainObjectIdTypedDependencyRootAuthorityBasisV1", "bad kind")
need(fixture.get("token") == token, "bad token")
need(token in card, "card missing token")
need(token in task_order, "task-order missing token")
need(rerun_003 in task_order, "task-order missing rerun 003")

inputs = fixture.get("input_state") or {}
need(inputs.get("current_blocker") == design_stop, "blocker drift")
for key, suffix in [
    ("subaxis_mechanical_selection_basis", "mirbuilder-domain-object-id-subaxis-mechanical-selection-basis-v0.json"),
    ("unresolved_subaxis_priority_rerun_002", "mirbuilder-domain-object-id-unresolved-subaxis-priority-rerun-002-v0.json"),
    ("domain_object_id_transport_policy_inventory_rerun_002", "mirbuilder-domain-object-id-transport-policy-inventory-rerun-002-v0.json"),
]:
    need(inputs.get(key, "").endswith(suffix), f"input drift: {key}")

previous = fixture.get("previous_state") or {}
need(previous.get("candidate_subaxis_count") == 5, "previous candidate count drift")
need(previous.get("guard_clean_candidate_count") == 5, "previous guard-clean drift")
need(previous.get("proof_tuple_complete_candidate_count") == 0, "previous proof tuple drift")
need(previous.get("selection_eligible_subaxis_count") == 0, "previous selection drift")
need(previous.get("previous_reason_token") == "NoExactlyOneDomainObjectIdSubaxisMechanicalCandidate", "previous reason drift")

rule = fixture.get("selector_rule") or {}
need(rule.get("name") == "DomainObjectIdTypedDependencyRootAuthorityV1", "rule name drift")
need(rule.get("extends") == "DomainObjectIdSubaxisMechanicalSelectorV1.dependency_root_authority", "rule extension drift")
need(rule.get("edge_direction") == "dependent_subaxis_requires_prerequisite_subaxis", "edge direction drift")
need(rule.get("selection_requires_exactly_one_dependency_root") is True, "root selection drift")
need(rule.get("isolated_candidates_are_unranked") is True, "isolation rule drift")
for key in [
    "hardcoded_subaxis_priority",
    "owner_name_as_proof",
    "source_path_as_authority",
    "route_membership_alone_as_proof",
    "coverage_as_proof",
    "convenience_as_proof",
]:
    need(rule.get(key) is False, f"forbidden rule drift: {key}")
need(rule.get("row_count_is_diagnostic_only") is True, "row count diagnostic drift")

accepted = set(fixture.get("accepted_edge_evidence_kinds") or [])
for key in [
    "ReturnTypeFieldReference",
    "ParameterTypeReference",
    "ConstructedDomainObjectReference",
    "VerifierInputContractReference",
    "PolicyDecisionPayloadReference",
    "FixtureDeclaredSemanticResourceDependency",
]:
    need(key in accepted, f"missing accepted evidence kind: {key}")

forbidden = set(fixture.get("forbidden_edge_evidence_kinds") or [])
for key in [
    "RowCount",
    "OwnerName",
    "SourcePath",
    "RouteMembershipAlone",
    "CoveragePercentage",
    "ImplementationConvenience",
    "LexicalOrder",
    "HardcodedSubaxisPriority",
]:
    need(key in forbidden, f"missing forbidden evidence kind: {key}")

expected = {
    "AstNodeDomainTransportAxis",
    "ContextOrSpanDomainTransportAxis",
    "MirDomainTransportAxis",
    "OtherDomainObjectTransportAxis",
    "PlanRecipeDomainTransportAxis",
}
candidates = fixture.get("candidate_subaxes") or []
need({candidate.get("domain_subaxis") for candidate in candidates} == expected, "candidate set drift")
for candidate in candidates:
    need(candidate.get("guard_clean_authority") == "Proven", "guard clean drift")
    need(candidate.get("dependency_participation") == "NotEvaluatedByBasis", "participation drift")
    need(candidate.get("dependency_root_authority", {}).get("status") == "NotEvaluatedByBasis", "root status drift")
    need(candidate.get("proof_tuple_complete") is False, "proof tuple drift")
    need(candidate.get("selection_eligible") is False, "selection drift")

graph = fixture.get("graph_summary") or {}
need(graph.get("candidate_subaxis_count") == 5, "graph candidate count drift")
need(graph.get("accepted_typed_dependency_edge_count") == 0, "accepted edge count drift")
need(graph.get("rejected_edge_count") == 0, "rejected edge count drift")
need(graph.get("ambiguous_edge_count") == 0, "ambiguous edge count drift")
need(graph.get("forbidden_edge_source_count") == 0, "forbidden edge source drift")
need(graph.get("cycle_count") == 0, "cycle count drift")
need(graph.get("dependency_root_candidate_count") == 0, "root candidate count drift")
need(graph.get("guard_clean_candidate_count") == 5, "graph guard-clean drift")
need(graph.get("selection_eligible_subaxis_count") == 0, "graph selection drift")

decision = fixture.get("decision") or {}
need(decision.get("kind") == "SelectDomainObjectIdSubaxisPriorityRerun003", "decision kind drift")
need(decision.get("reason_token") == "DefineDomainObjectIdTypedDependencyRootAuthorityBeforeSubaxisSelection", "reason drift")
need(decision.get("selected_domain_subaxis") is None, "basis must not select subaxis")
need(decision.get("selected_next_card") == rerun_003, "next drift")

claims = fixture.get("claims") or {}
need(claims.get("typed_dependency_root_authority_basis_defined") == 1, "basis claim drift")
for key in [
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

print("output_contract=rust-lifecycle-mirbuilder-domain-object-id-typed-dependency-root-authority-basis")
print("selector=DomainObjectIdTypedDependencyRootAuthorityV1")
print("candidate_subaxis_count=5")
print("dependency_root_candidate_count=0")
print("decision=SelectDomainObjectIdSubaxisPriorityRerun003")
print(f"selected_next_card={rerun_003}")
print("source_selfhost_claim=0")
print("summary=ok")
PY
