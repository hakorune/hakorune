#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
cd "$ROOT_DIR"

TAG="rust-lifecycle-mirbuilder-missing-projection-policy-cluster-resolution-v2"

source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

CARD="docs/development/current/main/phases/phase-296x/2004-MIRBUILDER-MISSING-PROJECTION-POLICY-CLUSTER-RESOLUTION-V2.md"
FIXTURE="docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-missing-projection-policy-cluster-resolution-v2-v0.json"
CHECKPOINT="docs/development/current/main/design/fixtures/rust-lifecycle/source-selfhost-native-owner-checkpoint-v0.json"
TOOL="tools/rust_lifecycle/mirbuilder_missing_projection_policy_cluster_resolution_v2.py"
TASK_ORDER="docs/development/current/main/design/mirbuilder-rust-to-hako-converter-task-order-ssot.md"
STATE="docs/development/current/main/CURRENT_STATE.toml"

guard_require_files "$TAG" "$CARD" "$FIXTURE" "$CHECKPOINT" "$TOOL" "$TASK_ORDER" "$STATE"

python3 "$TOOL" --check

python3 - "$CARD" "$FIXTURE" "$CHECKPOINT" "$TASK_ORDER" "$STATE" <<'PY'
from __future__ import annotations

import json
import sys
import tomllib
from pathlib import Path

card_path = Path(sys.argv[1])
fixture_path = Path(sys.argv[2])
checkpoint_path = Path(sys.argv[3])
task_order_path = Path(sys.argv[4])
state_path = Path(sys.argv[5])

card = card_path.read_text(encoding="utf-8")
fixture = json.loads(fixture_path.read_text(encoding="utf-8"))
checkpoint = json.loads(checkpoint_path.read_text(encoding="utf-8"))
task_order = task_order_path.read_text(encoding="utf-8")
state = tomllib.loads(state_path.read_text(encoding="utf-8"))


def require(condition: bool, message: str) -> None:
    if not condition:
        raise SystemExit(message)


token = "MIRBUILDER-MISSING-PROJECTION-POLICY-CLUSTER-RESOLUTION-V2"
next_card = "MIRBUILDER-PROJECTION-DESCRIPTOR-COVERAGE-RECLASSIFICATION-001"
design_stop = "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001"

require(token in card, "card missing token")
for needle in [
    "input_candidate_count = 1384",
    "selection_eligible_cluster_count = 41",
    "excluded_existing_decision_cluster_count = 41",
    "selectable_cluster_count = 0",
    next_card,
    "source_selfhost_claim = 0",
]:
    require(needle in card, f"card missing {needle}")

require(fixture.get("kind") == "MirBuilderMissingProjectionPolicyClusterResolutionV2", "fixture kind drift")
require(fixture.get("token") == token, "fixture token drift")
require((fixture.get("input_state") or {}).get("current_blocker") == design_stop, "blocker drift")
require((checkpoint.get("decision") or {}).get("selected_next_card") == token, "checkpoint must select V2")

cluster_state = fixture.get("cluster_state") or {}
require(cluster_state.get("input_candidate_count") == 1384, "input candidate count drift")
require(cluster_state.get("selection_eligible_cluster_count") == 41, "eligible cluster count drift")
require(cluster_state.get("priority_eligible_cluster_count") == 41, "priority eligible count drift")
require(cluster_state.get("excluded_existing_decision_cluster_count") == 41, "excluded existing count drift")
require(cluster_state.get("selectable_cluster_count") == 0, "selectable count drift")

resolution = fixture.get("resolution") or {}
require(resolution.get("eligible_projection_policy_clusters_already_landed") is True, "landed cluster resolution drift")
require(resolution.get("report_reclassification_required") is True, "reclassification flag drift")
require(resolution.get("new_projection_policy_selection_allowed") is False, "new projection policy must not be selected")
require(resolution.get("candidate_count_as_proof") is False, "candidate count must not be proof")

decision = fixture.get("decision") or {}
require(decision.get("kind") == "SelectProjectionDescriptorCoverageReclassification", "decision kind drift")
require(decision.get("reason_token") == "ProjectionPolicyClustersAlreadyLandedButReportStillMissing", "decision reason drift")
require(decision.get("selected_next_card") == next_card, "decision next drift")

claims = fixture.get("claims") or {}
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
    require(claims.get(key) == 0, f"forbidden claim drift: {key}")

require(state.get("latest_card") == token, "CURRENT_STATE latest card drift")
require(state.get("latest_card_path") == str(card_path), "CURRENT_STATE latest path drift")
require(state.get("current_blocker_token") == design_stop, "CURRENT_STATE blocker drift")

for needle in [
    token,
    next_card,
    "excluded_existing_decision_cluster_count = 41",
]:
    require(needle in task_order, f"task-order missing {needle}")

print("output_contract=rust-lifecycle-mirbuilder-missing-projection-policy-cluster-resolution-v2")
print("input_candidate_count=1384")
print("selection_eligible_cluster_count=41")
print("excluded_existing_decision_cluster_count=41")
print("selectable_cluster_count=0")
print(f"selected_next_card={next_card}")
print("source_selfhost_claim=0")
print("summary=ok")
PY
