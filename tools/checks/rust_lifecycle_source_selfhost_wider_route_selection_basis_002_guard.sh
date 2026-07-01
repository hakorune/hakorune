#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
cd "$ROOT_DIR"

TAG="rust-lifecycle-source-selfhost-wider-route-selection-basis-002"

source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

CARD="docs/development/current/main/phases/phase-296x/1974-SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-BASIS-002.md"
FIXTURE="docs/development/current/main/design/fixtures/rust-lifecycle/source-selfhost-wider-route-selection-basis-002-v0.json"
RERUN="docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-crate-wide-native-owner-seed-capability-survey-rerun-006-v0.json"
TASK_ORDER="docs/development/current/main/design/mirbuilder-rust-to-hako-converter-task-order-ssot.md"
STATE="docs/development/current/main/CURRENT_STATE.toml"

guard_require_files "$TAG" "$CARD" "$FIXTURE" "$RERUN" "$TASK_ORDER" "$STATE"

python3 - "$CARD" "$FIXTURE" "$RERUN" "$TASK_ORDER" "$STATE" <<'PY'
from __future__ import annotations

import json
import sys
import tomllib
from pathlib import Path

card_path = Path(sys.argv[1])
fixture_path = Path(sys.argv[2])
rerun_path = Path(sys.argv[3])
task_order_path = Path(sys.argv[4])
state_path = Path(sys.argv[5])

card = card_path.read_text(encoding="utf-8")
fixture = json.loads(fixture_path.read_text(encoding="utf-8"))
rerun = json.loads(rerun_path.read_text(encoding="utf-8"))
task_order = task_order_path.read_text(encoding="utf-8")
state = tomllib.loads(state_path.read_text(encoding="utf-8"))

def require(condition: bool, message: str) -> None:
    if not condition:
        raise SystemExit(message)

token = "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-BASIS-002"
contract = "rust-lifecycle-source-selfhost-wider-route-selection-basis-002-v0"
design_stop = "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001"
rerun_token = "MIRBUILDER-CRATE-WIDE-NATIVE-OWNER-SEED-CAPABILITY-SURVEY-RERUN-006"
reason = "NoBridgeEligibleStrictEmissionNativeSeedCandidateAfterTypeContextAdoption"

require(f"# {token}" in card, "card token drift")
for needle in [
    contract,
    rerun_token,
    "bridge_eligible_count = 0",
    "PostStrictEmissionBridgeExhaustionSelectorBasis",
    "SelectUnconvertedSurfaceReportRerun",
    "SelectStrictDenyGapClusterResolution",
    "SelectGeneratedArtifactToNativeSeedBridgePolicyV2",
    "SelectNativeOwnerCoverageCheckpoint",
    "SelectRouteRepair",
    "KeepStopped",
]:
    require(needle in card, f"card missing {needle}")

require(fixture.get("kind") == "SourceSelfhostWiderRouteSelectionBasisV2", "fixture kind drift")
require(fixture.get("token") == token, "fixture token drift")
require(fixture.get("output_contract") == contract, "fixture contract drift")

evidence = fixture.get("rerun_006_evidence") or {}
require(evidence.get("decision") == "KeepStopped", "fixture evidence decision drift")
require(evidence.get("reason_token") == reason, "fixture evidence reason drift")
require(evidence.get("bridge_eligible_count") == 0, "fixture bridge count drift")
require(evidence.get("selected_next_card") == design_stop, "fixture next card drift")

rerun_decision = rerun.get("decision") or {}
rerun_pool = rerun.get("candidate_pool") or {}
require(rerun.get("token") == rerun_token, "rerun token drift")
require(rerun_decision.get("kind") == "KeepStopped", "rerun decision drift")
require(rerun_decision.get("reason_token") == reason, "rerun reason drift")
require(rerun_decision.get("selected_next_card") == design_stop, "rerun next drift")
require(rerun_pool.get("bridge_eligible_count") == 0, "rerun bridge count drift")

basis = fixture.get("basis") or {}
require(basis.get("kind") == "PostStrictEmissionBridgeExhaustionSelectorBasis", "basis kind drift")
require(basis.get("allowed_decisions") == [
    "SelectUnconvertedSurfaceReportRerun",
    "SelectStrictDenyGapClusterResolution",
    "SelectGeneratedArtifactToNativeSeedBridgePolicyV2",
    "SelectNativeOwnerCoverageCheckpoint",
    "SelectRouteRepair",
    "KeepStopped",
], "allowed decisions drift")

contract_obj = fixture.get("selection_contract") or {}
for key in [
    "consume_rerun_006",
    "bridge_eligible_count_must_be_zero",
    "exactly_one_lane_or_keep_stopped",
    "selector_fixture_required_before_implementation",
    "native_seed_requires_selected_owner_edge_id",
    "route_repair_requires_concrete_target",
]:
    require(contract_obj.get(key) is True, f"selection contract drift: {key}")

decision = fixture.get("decision") or {}
require(decision.get("kind") == "BasisDefined", "decision kind drift")
require(decision.get("selected_next_card") == design_stop, "decision next drift")
require(decision.get("selected_owner_edge_id") is None, "owner edge must not be selected")

claims = fixture.get("claims") or {}
for key in [
    "manual_family_selection",
    "manual_shape_selection",
    "manual_axis_selection",
    "cluster_size_as_proof",
    "coverage_percentage_as_proof",
    "route_membership_alone_as_proof",
    "generated_artifact_as_native_edit_authority",
    "native_seed_materialization",
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
    contract,
    "post_rerun_006_task_order",
    "PostStrictEmissionBridgeExhaustionSelectorBasis",
    "selector fixture",
]:
    require(needle in task_order, f"task-order missing {needle}")

print(f"output_contract={contract}")
print(f"current_blocker_preserved={design_stop}")
print(f"input_card={rerun_token}")
print("bridge_eligible_count=0")
print("basis_kind=PostStrictEmissionBridgeExhaustionSelectorBasis")
print("manual_family_selection=0")
print("manual_shape_selection=0")
print("manual_axis_selection=0")
print("source_selfhost_claim=0")
print("summary=ok")
PY
