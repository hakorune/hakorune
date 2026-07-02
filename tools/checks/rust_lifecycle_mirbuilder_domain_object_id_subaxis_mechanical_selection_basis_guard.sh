#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
FIXTURE="$ROOT/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-domain-object-id-subaxis-mechanical-selection-basis-v0.json"
TOOL="$ROOT/tools/rust_lifecycle/mirbuilder_domain_object_id_subaxis_mechanical_selection_basis.py"
CARD="$ROOT/docs/development/current/main/phases/phase-296x/2065-MIRBUILDER-DOMAIN-OBJECT-ID-SUBAXIS-MECHANICAL-SELECTION-BASIS-001.md"
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


token = "MIRBUILDER-DOMAIN-OBJECT-ID-SUBAXIS-MECHANICAL-SELECTION-BASIS-001"
previous_token = "MIRBUILDER-DOMAIN-OBJECT-ID-UNRESOLVED-SUBAXIS-PRIORITY-RESOLUTION-001"
next_rerun = "MIRBUILDER-DOMAIN-OBJECT-ID-UNRESOLVED-SUBAXIS-PRIORITY-RERUN-002"
design_stop = "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001"

need(fixture.get("kind") == "MirBuilderDomainObjectIdSubaxisMechanicalSelectionBasisV1", "bad kind")
need(fixture.get("token") == token, "bad token")
need(token in card, "card missing token")
need(token in task_order, "task-order missing token")
need(next_rerun in task_order, "task-order missing next rerun")
need("DomainObjectIdSubaxisMechanicalSelectorV1" in card, "card missing selector")

inputs = fixture.get("input_state") or {}
need(inputs.get("current_blocker") == design_stop, "blocker drift")
need(
    inputs.get("domain_object_id_transport_policy_inventory_rerun_002", "").endswith(
        "mirbuilder-domain-object-id-transport-policy-inventory-rerun-002-v0.json"
    ),
    "inventory input drift",
)
need(
    inputs.get("unresolved_subaxis_priority_resolution", "").endswith(
        "mirbuilder-domain-object-id-unresolved-subaxis-priority-resolution-v0.json"
    ),
    "priority input drift",
)

local = fixture.get("local_authority") or {}
need(local.get("local_selection_authority") == "LocalMechanicalSelectorAuthorityV1", "local authority drift")
need(local.get("worker_inventory") == "consumed", "worker inventory drift")
need(local.get("worker_inventory_scope") == "read_only_current_fixtures_cards_ledgers", "worker scope drift")

previous = fixture.get("previous_state") or {}
need(previous.get("unresolved_non_id_domain_row_count") == 85, "previous row count drift")
need(previous.get("candidate_subaxis_count") == 5, "previous candidate count drift")
need(previous.get("selection_eligible_subaxis_count") == 0, "previous eligible count drift")
need(
    previous.get("previous_reason_token") == "NoMachineDerivedDomainObjectIdUnresolvedSubaxisPriority",
    "previous reason drift",
)

rule = fixture.get("selector_rule") or {}
need(rule.get("name") == "DomainObjectIdSubaxisMechanicalSelectorV1", "selector name drift")
need(rule.get("selection_requires_exactly_one_guard_clean_candidate") is True, "exactly-one rule drift")
need(rule.get("dependency_root_authority_allowed") is True, "dependency-root rule drift")
need(rule.get("prior_closed_lane_continuation_authority_allowed") is True, "closed-lane rule drift")
need(rule.get("hardcoded_subaxis_priority") is False, "hardcoded priority drift")
need(rule.get("row_count_is_diagnostic_only") is True, "row-count rule drift")
for key in [
    "owner_name_as_proof",
    "source_path_as_authority",
    "route_membership_alone_as_proof",
    "manual_subaxis_selection",
]:
    need(rule.get(key) is False, f"forbidden selector rule drift: {key}")

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
    need(candidate.get("row_count_diagnostic_only") is True, f"row-count proof drift: {name}")
    need(candidate.get("proof_tuple_complete") is False, f"basis should not complete tuple: {name}")
    need(candidate.get("selection_eligible") is False, f"basis should not select candidate: {name}")
    need(candidate.get("dependency_root_authority", {}).get("status") == "NotEvaluatedAtBasis", "dependency status drift")
    need(
        candidate.get("prior_closed_lane_continuation_authority", {}).get("status") == "NotEvaluatedAtBasis",
        "closed-lane status drift",
    )
    need(candidate.get("guard_clean_authority", {}).get("status") == "NotEvaluatedAtBasis", "guard status drift")
    blocked_by = candidate.get("blocked_by") or []
    need("MechanicalSelectorBasisDefinedButNotEvaluated" in blocked_by, "blocked_by drift")

summary = fixture.get("summary") or {}
need(summary.get("candidate_subaxis_count") == 5, "summary candidate count drift")
need(summary.get("guard_clean_candidate_count") == 0, "guard-clean count drift")
need(summary.get("proof_tuple_complete_candidate_count") == 0, "proof tuple count drift")
need(summary.get("selection_eligible_subaxis_count") == 0, "selection count drift")

decision = fixture.get("decision") or {}
need(decision.get("kind") == "SelectDomainObjectIdSubaxisPriorityRerun", "decision kind drift")
need(decision.get("reason_token") == "DomainObjectIdSubaxisMechanicalSelectorBasisDefined", "reason drift")
need(decision.get("selected_domain_subaxis") is None, "basis must not select subaxis")
need(decision.get("selected_next_card") == next_rerun, "next drift")

recovery = fixture.get("recovery_if_rerun_fails") or {}
need(recovery.get("no_candidate_reason_token") == "NoExactlyOneDomainObjectIdSubaxisMechanicalCandidate", "recovery drift")
need(recovery.get("selected_next_card") == design_stop, "recovery next drift")

claims = fixture.get("claims") or {}
need(claims.get("domain_object_id_subaxis_mechanical_selection_basis_defined") == 1, "basis claim drift")
need(claims.get("local_mechanical_selector_authority_consumed") == 1, "local authority claim drift")
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
need(previous_token in task_order, "task-order missing previous token")

print("output_contract=rust-lifecycle-mirbuilder-domain-object-id-subaxis-mechanical-selection-basis")
print("selector=DomainObjectIdSubaxisMechanicalSelectorV1")
print("candidate_subaxis_count=5")
print("selection_eligible_subaxis_count=0")
print("decision=SelectDomainObjectIdSubaxisPriorityRerun")
print(f"selected_next_card={next_rerun}")
print("source_selfhost_claim=0")
print("summary=ok")
PY
