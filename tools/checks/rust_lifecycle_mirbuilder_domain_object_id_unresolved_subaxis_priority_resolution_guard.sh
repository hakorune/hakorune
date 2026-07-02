#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
FIXTURE="$ROOT/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-domain-object-id-unresolved-subaxis-priority-resolution-v0.json"
TOOL="$ROOT/tools/rust_lifecycle/mirbuilder_domain_object_id_unresolved_subaxis_priority_resolution.py"
CARD="$ROOT/docs/development/current/main/phases/phase-296x/2064-MIRBUILDER-DOMAIN-OBJECT-ID-UNRESOLVED-SUBAXIS-PRIORITY-RESOLUTION-001.md"
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


token = "MIRBUILDER-DOMAIN-OBJECT-ID-UNRESOLVED-SUBAXIS-PRIORITY-RESOLUTION-001"
input_token = "MIRBUILDER-DOMAIN-OBJECT-ID-TRANSPORT-POLICY-INVENTORY-RERUN-002"
design_stop = "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001"

need(fixture.get("kind") == "MirBuilderDomainObjectIdUnresolvedSubaxisPriorityResolutionV1", "bad kind")
need(fixture.get("token") == token, "bad token")
need(token in card, "card missing token")
need(token in task_order, "task-order missing token")
need("DesignConsultationRequired" in card, "card missing recovery")

inputs = fixture.get("input_state") or {}
need(inputs.get("current_blocker") == design_stop, "blocker drift")
need(
    inputs.get("domain_object_id_transport_policy_inventory_rerun_002", "").endswith(
        "mirbuilder-domain-object-id-transport-policy-inventory-rerun-002-v0.json"
    ),
    "input drift",
)

local = fixture.get("local_authority") or {}
need(local.get("local_selection_authority") == "LocalMechanicalSelectorAuthorityV1", "local authority drift")
need(local.get("worker_inventory") == "consumed", "worker inventory drift")
need(local.get("worker_inventory_scope") == "read_only_current_fixtures_cards_ledgers", "worker scope drift")

input_decision = fixture.get("input_decision") or {}
need(input_decision.get("kind") == "SelectUnresolvedSubaxisPriorityResolution", "input decision drift")
need(input_decision.get("selected_next_card") == token, "input next drift")

summary = fixture.get("summary") or {}
need(summary.get("unresolved_non_id_domain_row_count") == 85, "unresolved count drift")
need(summary.get("candidate_subaxis_count") == 5, "candidate count drift")
need(summary.get("selection_eligible_subaxis_count") == 0, "eligible count drift")
expected = {
    "PlanRecipeDomainTransportAxis": 48,
    "MirDomainTransportAxis": 19,
    "AstNodeDomainTransportAxis": 14,
    "ContextOrSpanDomainTransportAxis": 3,
    "OtherDomainObjectTransportAxis": 1,
}
counts = summary.get("domain_subaxis_counts") or {}
for key, value in expected.items():
    need(counts.get(key) == value, f"subaxis count drift: {key}")

candidates = fixture.get("candidate_subaxes") or []
need(len(candidates) == 5, "candidate rows drift")
for candidate in candidates:
    need(candidate.get("selection_eligible") is False, f"candidate unexpectedly eligible: {candidate.get('domain_subaxis')}")
    need(candidate.get("machine_priority_authority") == "Unproven", "machine authority drift")
    blocked_by = candidate.get("blocked_by") or []
    for token_ in [
        "NoDependencyRootAuthority",
        "NoPriorClosedLaneConsumptionAuthority",
        "NoExactlyOneGuardCleanCandidate",
        "RowCountIsDiagnosticOnly",
    ]:
        need(token_ in blocked_by, f"missing blocked_by {token_}")

selection_rule = fixture.get("selection_rule") or {}
need(selection_rule.get("subaxis_priority_requires_machine_authority") is True, "authority rule drift")
need(selection_rule.get("row_count_is_diagnostic_only") is True, "row-count diagnostic rule drift")
need(selection_rule.get("design_consultation_required_if_no_machine_authority") is True, "consultation rule drift")
for key in ["owner_name_as_proof", "route_membership_alone_as_proof", "manual_subaxis_selection"]:
    need(selection_rule.get(key) is False, f"forbidden rule drift: {key}")

decision = fixture.get("decision") or {}
need(decision.get("kind") == "KeepStopped", "decision kind drift")
need(decision.get("reason_token") == "NoMachineDerivedDomainObjectIdUnresolvedSubaxisPriority", "reason drift")
need(decision.get("selected_domain_subaxis") is None, "subaxis should not be selected")
need(decision.get("selected_next_card") == design_stop, "next drift")

recovery = fixture.get("recovery") or {}
need(recovery.get("kind") == "DesignConsultationRequired", "recovery drift")

claims = fixture.get("claims") or {}
need(claims.get("domain_object_id_transport_policy_inventory_rerun_002_consumed") == 1, "input consumed claim drift")
need(claims.get("unresolved_subaxis_priority_resolution_ready") == 1, "ready claim drift")
for key in [
    "manual_family_selection",
    "manual_shape_selection",
    "manual_axis_selection",
    "manual_carrier_selection",
    "manual_subaxis_selection",
    "row_count_as_proof",
    "owner_name_as_proof",
    "domain_object_count_as_proof",
    "cluster_size_as_proof",
    "coverage_percentage_as_proof",
    "route_membership_alone_as_proof",
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
need(input_token in task_order, "task-order missing input token")

print("output_contract=rust-lifecycle-mirbuilder-domain-object-id-unresolved-subaxis-priority-resolution")
print("unresolved_non_id_domain_row_count=85")
print("candidate_subaxis_count=5")
print("selection_eligible_subaxis_count=0")
print("decision=KeepStopped")
print("reason=NoMachineDerivedDomainObjectIdUnresolvedSubaxisPriority")
print("source_selfhost_claim=0")
print("summary=ok")
PY
