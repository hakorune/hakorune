#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
cd "$ROOT_DIR"

TAG="rust-lifecycle-source-selfhost-wider-route-selection-resolution"

source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

CARD="docs/development/current/main/phases/phase-296x/1802-SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-RESOLUTION-001.md"
FIXTURE="docs/development/current/main/design/fixtures/rust-lifecycle/source-selfhost-wider-route-selection-resolution-v0.json"
DESIGN_STOP="docs/development/current/main/phases/phase-296x/1799-SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001.md"
RUNNER_BREAKDOWN="docs/development/current/main/phases/phase-296x/1800-SOURCE-SELFHOST-RUNNER-AND-ROUTE-TASK-BREAKDOWN-001.md"
BASIS="docs/development/current/main/phases/phase-296x/1801-SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-BASIS-001.md"
TASK_ORDER="docs/development/current/main/design/mirbuilder-rust-to-hako-converter-task-order-ssot.md"
INDEX="docs/tools/check-scripts-index.md"
STATE="docs/development/current/main/CURRENT_STATE.toml"

guard_require_files "$TAG" \
  "$CARD" \
  "$FIXTURE" \
  "$DESIGN_STOP" \
  "$RUNNER_BREAKDOWN" \
  "$BASIS" \
  "$TASK_ORDER" \
  "$INDEX" \
  "$STATE"

python3 - "$CARD" "$FIXTURE" "$DESIGN_STOP" "$RUNNER_BREAKDOWN" "$BASIS" "$TASK_ORDER" "$INDEX" "$STATE" <<'PY'
from __future__ import annotations

import json
import sys
import tomllib
from pathlib import Path

card_path = Path(sys.argv[1])
fixture_path = Path(sys.argv[2])
design_stop_path = Path(sys.argv[3])
runner_breakdown_path = Path(sys.argv[4])
basis_path = Path(sys.argv[5])
task_order_path = Path(sys.argv[6])
index_path = Path(sys.argv[7])
state_path = Path(sys.argv[8])

card = card_path.read_text(encoding="utf-8")
fixture = json.loads(fixture_path.read_text(encoding="utf-8"))
design_stop = design_stop_path.read_text(encoding="utf-8")
runner_breakdown = runner_breakdown_path.read_text(encoding="utf-8")
basis = basis_path.read_text(encoding="utf-8")
task_order = task_order_path.read_text(encoding="utf-8")
index = index_path.read_text(encoding="utf-8")
state = tomllib.loads(state_path.read_text(encoding="utf-8"))

def require(condition: bool, message: str) -> None:
    if not condition:
        raise SystemExit(message)

token = "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-RESOLUTION-001"
contract = "rust-lifecycle-source-selfhost-wider-route-selection-resolution-v0"
design_stop_token = "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001"

require(f"# {token}" in card, "card token drift")
require(f"output_contract:\n  {contract}" in card, "card output contract drift")
require("KeepSourceSelfhostStopped" in card, "card resolution drift")
require("NoEligibleNativeAdoptionCandidate" in card, "card reason drift")
require("MIRBUILDER-MINIMAL-PATH-COMPOSED-CLOSURE-NATIVE-SLICE-DECOMPOSITION-001" in card, "card missing decomposition follow-up")
require("<ROUTE-FAMILY>-ROUTE-MATRIX-REPAIR-001" in card, "card missing repair follow-up")

require(fixture.get("kind") == "SourceSelfhostWiderRouteSelectionResolutionV1", "fixture kind drift")
require(fixture.get("output_contract") == contract, "fixture output contract drift")
require((fixture.get("current_state") or {}).get("latest_card") == token, "fixture latest card drift")
require((fixture.get("current_state") or {}).get("current_blocker_token") == design_stop_token, "fixture blocker drift")

basis_obj = fixture.get("basis") or {}
require(basis_obj.get("kind") == "KeepSourceSelfhostStopped", "fixture basis kind drift")
require(basis_obj.get("reason_token") == "NoEligibleNativeAdoptionCandidate", "fixture basis reason drift")
require(basis_obj.get("allowed_resume") == [
    "ConsultationGatedWiderRouteSelection",
    "MachineDerivedRouteRepair",
], "fixture allowed resume drift")

candidate_pool = fixture.get("candidate_pool") or {}
require(candidate_pool.get("state") == "Blocked", "candidate pool state drift")
require(candidate_pool.get("eligible_count") == 0, "candidate count drift")
require(candidate_pool.get("repairable_inconsistency_count") == 0, "repairable count drift")
require(candidate_pool.get("consultation_gated_count") == 1, "consultation gated count drift")
require(candidate_pool.get("ambiguous_candidate_count") == 0, "ambiguous count drift")

resolution = fixture.get("resolution") or {}
require(resolution.get("kind") == "KeepSourceSelfhostStopped", "resolution kind drift")
require(resolution.get("reason_token") == "NoEligibleNativeAdoptionCandidate", "resolution reason drift")
require(resolution.get("next_action") == design_stop_token, "resolution next action drift")
require(resolution.get("planned_follow_up_task_packs") == [
    "MIRBUILDER-MINIMAL-PATH-COMPOSED-CLOSURE-NATIVE-SLICE-DECOMPOSITION-001",
    "<ROUTE-FAMILY>-ROUTE-MATRIX-REPAIR-001",
], "planned follow-up packs drift")

recovery = fixture.get("recovery") or {}
for key in [
    "manual_family_selection",
    "route_membership_alone_as_proof",
    "coverage_percentage_as_proof",
    "bundle_size_as_proof",
    "support_lane_projector_as_adoption_candidate",
    "runtime_fallback",
    "new_backend_route",
    "new_abi",
    "new_python_semantic_projector",
    "source_selfhost_claim",
]:
    require(recovery.get(key) == 0, f"recovery claim drift: {key}")

require(design_stop_token in runner_breakdown, "runner breakdown provenance missing")

require(state.get("latest_card") == token, "CURRENT_STATE latest card drift")
require(state.get("current_blocker_token") == design_stop_token, "CURRENT_STATE blocker drift")
require(state.get("latest_card_path") == "docs/development/current/main/phases/phase-296x/1802-SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-RESOLUTION-001.md", "CURRENT_STATE latest card path drift")

for needle in [
    token,
    contract,
    "MIRBUILDER-MINIMAL-PATH-COMPOSED-CLOSURE-NATIVE-SLICE-DECOMPOSITION-001",
    "<ROUTE-FAMILY>-ROUTE-MATRIX-REPAIR-001",
    "KeepSourceSelfhostStopped",
    "NoEligibleNativeAdoptionCandidate",
]:
    require(needle in task_order, f"task-order missing {needle}")

require("tools/checks/rust_lifecycle_source_selfhost_wider_route_selection_resolution_guard.sh" in index, "check index missing guard")

print(f"output_contract={contract}")
print(f"current_blocker_preserved={design_stop_token}")
print("basis_kind=KeepSourceSelfhostStopped")
print("reason_token=NoEligibleNativeAdoptionCandidate")
print("next_action=SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001")
print("planned_follow_up_task_packs_named=1")
print("manual_family_selection=0")
print("runtime_fallback=0")
print("new_backend_route=0")
print("new_abi=0")
print("source_selfhost_claim=0")
print("summary=ok")
PY
