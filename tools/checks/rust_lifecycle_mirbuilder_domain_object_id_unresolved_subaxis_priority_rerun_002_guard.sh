#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
FIXTURE="$ROOT/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-domain-object-id-unresolved-subaxis-priority-rerun-002-v0.json"
TOOL="$ROOT/tools/rust_lifecycle/mirbuilder_domain_object_id_unresolved_subaxis_priority_rerun_002.py"
CARD="$ROOT/docs/development/current/main/phases/phase-296x/2066-MIRBUILDER-DOMAIN-OBJECT-ID-UNRESOLVED-SUBAXIS-PRIORITY-RERUN-002.md"
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


token = "MIRBUILDER-DOMAIN-OBJECT-ID-UNRESOLVED-SUBAXIS-PRIORITY-RERUN-002"
basis_token = "MIRBUILDER-DOMAIN-OBJECT-ID-SUBAXIS-MECHANICAL-SELECTION-BASIS-001"
design_stop = "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001"

need(fixture.get("kind") == "MirBuilderDomainObjectIdUnresolvedSubaxisPriorityRerunV2", "bad kind")
need(fixture.get("token") == token, "bad token")
need(token in card, "card missing token")
need(token in task_order, "task-order missing token")
need(basis_token in task_order, "task-order missing basis token")

inputs = fixture.get("input_state") or {}
need(inputs.get("current_blocker") == design_stop, "blocker drift")
need(inputs.get("subaxis_mechanical_selection_basis", "").endswith("mirbuilder-domain-object-id-subaxis-mechanical-selection-basis-v0.json"), "basis input drift")
need(inputs.get("domain_object_id_transport_policy_inventory_rerun_002", "").endswith("mirbuilder-domain-object-id-transport-policy-inventory-rerun-002-v0.json"), "inventory input drift")

local = fixture.get("local_authority") or {}
need(local.get("local_selection_authority") == "LocalMechanicalSelectorAuthorityV1", "local authority drift")
need(local.get("worker_inventory") == "consumed", "worker inventory drift")

summary = fixture.get("summary") or {}
need(summary.get("unresolved_non_id_domain_row_count") == 85, "unresolved row count drift")
need(summary.get("closed_id_scalar_row_count") == 31, "closed ID scalar count drift")
need(summary.get("candidate_subaxis_count") == 5, "candidate count drift")
need(summary.get("guard_clean_candidate_count") == 5, "guard-clean count drift")
need(summary.get("proof_tuple_complete_candidate_count") == 0, "proof tuple count drift")
need(summary.get("selection_eligible_subaxis_count") == 0, "selection eligible count drift")

rule = fixture.get("selector_rule") or {}
need(rule.get("name") == "DomainObjectIdSubaxisMechanicalSelectorV1", "selector rule drift")
need(rule.get("hardcoded_subaxis_priority") is False, "hardcoded priority drift")

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
    name = candidate.get("domain_subaxis")
    need(candidate.get("row_count_diagnostic_only") is True, f"row count proof drift: {name}")
    need(candidate.get("proof_tuple_complete") is False, f"proof tuple should be incomplete: {name}")
    need(candidate.get("selection_eligible") is False, f"candidate should not be selected: {name}")
    need(candidate.get("dependency_root_authority", {}).get("status") == "Unproven", f"dependency status drift: {name}")
    need(candidate.get("prior_closed_lane_continuation_authority", {}).get("status") == "Unproven", f"closed lane status drift: {name}")
    need(candidate.get("guard_clean_authority", {}).get("status") == "Proven", f"guard clean status drift: {name}")
    blocked = candidate.get("blocked_by") or []
    need("NoDependencyRootAuthority" in blocked, f"missing dependency blocker: {name}")
    need("NoPriorClosedLaneContinuationAuthority" in blocked, f"missing closed-lane blocker: {name}")
    need("ProofTupleIncomplete" in blocked, f"missing proof blocker: {name}")

decision = fixture.get("decision") or {}
need(decision.get("kind") == "KeepStopped", "decision kind drift")
need(decision.get("reason_token") == "NoExactlyOneDomainObjectIdSubaxisMechanicalCandidate", "reason drift")
need(decision.get("selected_domain_subaxis") is None, "subaxis must not be selected")
need(decision.get("selected_next_card") == design_stop, "next card drift")

claims = fixture.get("claims") or {}
need(claims.get("subaxis_mechanical_selection_basis_consumed") == 1, "basis consumed claim drift")
for key in [
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
    "generated_artifact_as_native_edit_authority",
    "native_seed_materialization",
    "hako_generation",
    "hako_adopted_decision",
    "source_selfhost_claim",
    "runtime_fallback",
    "new_backend_route",
    "new_abi",
    "new_python_semantic_projector",
    "runner_semantic_owner",
]:
    need(claims.get(key) == 0, f"forbidden claim drift: {key}")

need(state.get("current_blocker_token") == design_stop, "CURRENT_STATE blocker drift")

print("output_contract=rust-lifecycle-mirbuilder-domain-object-id-unresolved-subaxis-priority-rerun-002")
print("candidate_subaxis_count=5")
print("guard_clean_candidate_count=5")
print("proof_tuple_complete_candidate_count=0")
print("selection_eligible_subaxis_count=0")
print("decision=KeepStopped")
print("reason=NoExactlyOneDomainObjectIdSubaxisMechanicalCandidate")
print("source_selfhost_claim=0")
print("summary=ok")
PY
