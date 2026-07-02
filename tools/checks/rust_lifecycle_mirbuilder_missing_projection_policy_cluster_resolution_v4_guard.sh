#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
FIXTURE="$ROOT/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-missing-projection-policy-cluster-resolution-v4-v0.json"
TOOL="$ROOT/tools/rust_lifecycle/mirbuilder_missing_projection_policy_cluster_resolution_v4.py"
CARD="$ROOT/docs/development/current/main/phases/phase-296x/2059-MIRBUILDER-MISSING-PROJECTION-POLICY-CLUSTER-RESOLUTION-V4.md"
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
_task_order = Path(sys.argv[4]).read_text(encoding="utf-8")


def need(cond, msg):
    if not cond:
        raise SystemExit(msg)


token = "MIRBUILDER-MISSING-PROJECTION-POLICY-CLUSTER-RESOLUTION-V4"
next_card = "MIRBUILDER-CARRIER-TYPE-TRANSPORT-POLICY-INVENTORY-RERUN-003"
design_stop = "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001"

need(fixture.get("kind") == "MirBuilderMissingProjectionPolicyClusterResolutionV4", "bad kind")
need(fixture.get("token") == token, "bad token")
need(token in card, "card missing token")
need("LocalMechanicalSelectorAuthorityV1" in card, "card missing local authority")
need("worker_inventory = consumed" in card, "card missing worker inventory")

inputs = fixture.get("input_state") or {}
need(inputs.get("current_blocker") == design_stop, "blocker drift")
need(
    inputs.get("native_owner_checkpoint_rerun", "").endswith(
        "source-selfhost-native-owner-checkpoint-rerun-002-v0.json"
    ),
    "checkpoint input drift",
)

local = fixture.get("local_authority") or {}
need(local.get("local_selection_authority") == "LocalMechanicalSelectorAuthorityV1", "local authority drift")
need(local.get("worker_inventory") == "consumed", "worker inventory drift")
need(local.get("worker_inventory_scope") == "read_only_current_fixtures_cards_ledgers", "worker scope drift")

checkpoint = fixture.get("checkpoint_decision") or {}
need(checkpoint.get("kind") == "SelectMissingProjectionPolicyClusterResolutionV4", "checkpoint decision drift")
need(checkpoint.get("selected_next_card") == token, "checkpoint next drift")

report = fixture.get("report_state") or {}
need(report.get("missing_projection_policy_count") == 1004, "report missing count drift")
need(report.get("projection_descriptor_coverage_reclassified_count") == 380, "report coverage count drift")
need(report.get("borrow_policy_needed_count") == 112, "report borrow count drift")

clusters = fixture.get("cluster_state") or {}
need(clusters.get("input_candidate_count") == 1004, "input count drift")
need(clusters.get("cluster_count") == 78, "cluster count drift")
need(clusters.get("selection_eligible_cluster_count") == 0, "eligible cluster drift")
need(clusters.get("fixture_mapped_count") == 819, "fixture mapped count drift")
need(clusters.get("heuristic_or_unmapped_count") == 185, "unmapped count drift")
need(clusters.get("type_transport_missing_cluster_count") == 76, "type transport missing drift")
need(clusters.get("owner_confidence_missing_cluster_count") == 5, "owner confidence missing drift")
need(clusters.get("missing_shape_signature_cluster_count") == 5, "shape signature missing drift")

selection_rule = fixture.get("selection_rule") or {}
for key in [
    "type_transport_missing_blocks_projection_policy",
    "exactly_one_next_lane_or_keep_stopped",
    "worker_inventory_required_or_waived",
]:
    need(selection_rule.get(key) is True, f"selection rule drift: {key}")
for key in [
    "manual_projection_policy_selection",
    "manual_lane_selection",
    "cluster_size_as_proof",
    "coverage_percentage_as_proof",
]:
    need(selection_rule.get(key) is False, f"forbidden selection rule drift: {key}")

decision = fixture.get("decision") or {}
need(decision.get("kind") == "SelectCarrierTypeTransportPolicyInventoryRerun003", "decision kind drift")
need(decision.get("reason_token") == "TypeTransportMissingBlocksProjectionPolicyClusters", "reason drift")
need(decision.get("selected_next_card") == next_card, "decision next drift")
need(decision.get("selected_cluster_id") is None, "must not select projection cluster")

claims = fixture.get("claims") or {}
need(claims.get("all_missing_projection_policy_items_clustered_exactly_once") == 1, "cluster completeness drift")
for key in [
    "manual_family_selection",
    "manual_shape_selection",
    "manual_axis_selection",
    "manual_lane_selection",
    "candidate_count_as_proof",
    "cluster_size_as_proof",
    "coverage_percentage_as_proof",
    "new_projection_policy_selected",
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
print("output_contract=rust-lifecycle-mirbuilder-missing-projection-policy-cluster-resolution-v4")
print("input_candidate_count=1004")
print("cluster_count=78")
print("selection_eligible_cluster_count=0")
print("type_transport_missing_cluster_count=76")
print(f"selected_next_card={next_card}")
print("source_selfhost_claim=0")
print("summary=ok")
PY
