#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
FIXTURE="$ROOT/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-missing-projection-policy-cluster-resolution-v3-v0.json"
TOOL="$ROOT/tools/rust_lifecycle/mirbuilder_missing_projection_policy_cluster_resolution_v3.py"
CARD="$ROOT/docs/development/current/main/phases/phase-296x/2008-MIRBUILDER-MISSING-PROJECTION-POLICY-CLUSTER-RESOLUTION-V3.md"
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


token = "MIRBUILDER-MISSING-PROJECTION-POLICY-CLUSTER-RESOLUTION-V3"
next_card = "MIRBUILDER-CARRIER-TYPE-TRANSPORT-POLICY-INVENTORY-RERUN-002"
design_stop = "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001"

need(fixture.get("kind") == "MirBuilderMissingProjectionPolicyClusterResolutionV3", "bad kind")
need(fixture.get("token") == token, "bad token")
need(token in card, "card missing token")

inputs = fixture.get("input_state") or {}
need(inputs.get("current_blocker") == design_stop, "blocker drift")
need(inputs.get("native_owner_checkpoint_rerun", "").endswith("source-selfhost-native-owner-checkpoint-rerun-v0.json"), "checkpoint input drift")

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

decision = fixture.get("decision") or {}
need(decision.get("kind") == "SelectCarrierTypeTransportPolicyInventoryRerun002", "decision kind drift")
need(decision.get("reason_token") == "TypeTransportMissingBlocksProjectionPolicyClusters", "reason drift")
need(decision.get("selected_next_card") == next_card, "decision next drift")

claims = fixture.get("claims") or {}
need(claims.get("all_missing_projection_policy_items_clustered_exactly_once") == 1, "cluster completeness drift")
for key in [
    "manual_family_selection",
    "manual_shape_selection",
    "manual_axis_selection",
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

need(state.get("latest_card") == token, "CURRENT_STATE latest card drift")
need(state.get("current_blocker_token") == design_stop, "CURRENT_STATE blocker drift")
for needle in [
    token,
    next_card,
    "input_candidate_count = 1004",
    "type_transport_missing_cluster_count = 76",
    "source_selfhost_claim = 0",
]:
    need(needle in task_order, f"task-order missing {needle}")

print("output_contract=rust-lifecycle-mirbuilder-missing-projection-policy-cluster-resolution-v3")
print("input_candidate_count=1004")
print("cluster_count=78")
print("selection_eligible_cluster_count=0")
print("type_transport_missing_cluster_count=76")
print(f"selected_next_card={next_card}")
print("source_selfhost_claim=0")
print("summary=ok")
PY
