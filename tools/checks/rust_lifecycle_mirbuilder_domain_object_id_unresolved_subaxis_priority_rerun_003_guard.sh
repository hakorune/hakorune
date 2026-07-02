#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
FIXTURE="$ROOT/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-domain-object-id-unresolved-subaxis-priority-rerun-003-v0.json"
TOOL="$ROOT/tools/rust_lifecycle/mirbuilder_domain_object_id_unresolved_subaxis_priority_rerun_003.py"
CARD="$ROOT/docs/development/current/main/phases/phase-296x/2068-MIRBUILDER-DOMAIN-OBJECT-ID-UNRESOLVED-SUBAXIS-PRIORITY-RERUN-003.md"
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


token = "MIRBUILDER-DOMAIN-OBJECT-ID-UNRESOLVED-SUBAXIS-PRIORITY-RERUN-003"
basis_token = "MIRBUILDER-DOMAIN-OBJECT-ID-TYPED-DEPENDENCY-ROOT-AUTHORITY-BASIS-001"
design_stop = "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001"

need(fixture.get("kind") == "MirBuilderDomainObjectIdUnresolvedSubaxisPriorityRerunV3", "bad kind")
need(fixture.get("token") == token, "bad token")
need(token in card, "card missing token")
need(token in task_order, "task-order missing token")
need(basis_token in task_order, "task-order missing basis token")

inputs = fixture.get("input_state") or {}
need(inputs.get("current_blocker") == design_stop, "blocker drift")
need(inputs.get("typed_dependency_root_authority_basis", "").endswith("mirbuilder-domain-object-id-typed-dependency-root-authority-basis-v0.json"), "basis input drift")

graph = fixture.get("graph_summary") or {}
need(graph.get("candidate_subaxis_count") == 5, "candidate count drift")
need(graph.get("accepted_typed_dependency_edge_count") == 0, "accepted edge count drift")
need(graph.get("rejected_edge_count") == 0, "rejected edge count drift")
need(graph.get("ambiguous_edge_count") == 0, "ambiguous edge count drift")
need(graph.get("forbidden_edge_source_count") == 0, "forbidden edge drift")
need(graph.get("cycle_count") == 0, "cycle count drift")
need(graph.get("dependency_root_candidate_count") == 0, "root candidate count drift")
need(graph.get("guard_clean_candidate_count") == 5, "guard clean count drift")
need(graph.get("proof_tuple_complete_candidate_count") == 0, "proof tuple drift")
need(graph.get("selection_eligible_subaxis_count") == 0, "selection drift")

edges = fixture.get("typed_dependency_edges") or {}
need(edges.get("accepted") == [], "accepted edge list should be empty")
need(edges.get("rejected") == [], "rejected edge list should be empty")

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
    need(candidate.get("dependency_participation") == "Isolated", "candidate should be isolated")
    need(candidate.get("proof_tuple_complete") is False, "proof tuple should be false")
    need(candidate.get("selection_eligible") is False, "candidate should not be selected")

decision = fixture.get("decision") or {}
need(decision.get("kind") == "KeepStopped", "decision kind drift")
need(decision.get("reason_token") == "NoMachineDerivedDomainObjectIdTypedDependencyRootAuthority", "reason drift")
need(decision.get("selected_domain_subaxis") is None, "subaxis must not be selected")
need(decision.get("selected_next_card") == design_stop, "next drift")

claims = fixture.get("claims") or {}
need(claims.get("typed_dependency_root_authority_basis_consumed") == 1, "basis consumed claim drift")
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

print("output_contract=rust-lifecycle-mirbuilder-domain-object-id-unresolved-subaxis-priority-rerun-003")
print("accepted_typed_dependency_edge_count=0")
print("dependency_root_candidate_count=0")
print("decision=KeepStopped")
print("reason=NoMachineDerivedDomainObjectIdTypedDependencyRootAuthority")
print("source_selfhost_claim=0")
print("summary=ok")
PY
