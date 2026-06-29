#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
cd "$ROOT_DIR"

TAG="rust-lifecycle-source-selfhost-wider-route-selection-basis"

source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

CARD="docs/development/current/main/phases/phase-296x/1801-SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-BASIS-001.md"
SSOT="docs/development/current/main/design/source-selfhost-wider-route-selection-basis-ssot.md"
FIXTURE="docs/development/current/main/design/fixtures/rust-lifecycle/source-selfhost-wider-route-selection-basis-v0.json"
DESIGN_STOP="docs/development/current/main/phases/phase-296x/1799-SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001.md"
RUNNER_BREAKDOWN="docs/development/current/main/phases/phase-296x/1800-SOURCE-SELFHOST-RUNNER-AND-ROUTE-TASK-BREAKDOWN-001.md"
ARTIFACT_POLICY="docs/development/current/main/design/artifact-policy-ssot.md"
VM_RETIREMENT="docs/development/current/main/design/vm-active-lane-retirement-ssot.md"
ROLE_SSOT="docs/development/current/main/design/rust-to-hako-converter-implementation-role-ssot.md"
TASK_ORDER="docs/development/current/main/design/mirbuilder-rust-to-hako-converter-task-order-ssot.md"
STATE="docs/development/current/main/CURRENT_STATE.toml"
INDEX="docs/tools/check-scripts-index.md"

guard_require_files "$TAG" \
  "$CARD" \
  "$SSOT" \
  "$FIXTURE" \
  "$DESIGN_STOP" \
  "$RUNNER_BREAKDOWN" \
  "$ARTIFACT_POLICY" \
  "$VM_RETIREMENT" \
  "$ROLE_SSOT" \
  "$TASK_ORDER" \
  "$STATE" \
  "$INDEX"

python3 - "$CARD" "$SSOT" "$FIXTURE" "$DESIGN_STOP" "$RUNNER_BREAKDOWN" "$ARTIFACT_POLICY" "$VM_RETIREMENT" "$ROLE_SSOT" "$TASK_ORDER" "$STATE" "$INDEX" <<'PY'
from __future__ import annotations

import json
import sys
import tomllib
from pathlib import Path

card_path = Path(sys.argv[1])
ssot_path = Path(sys.argv[2])
fixture_path = Path(sys.argv[3])
design_stop_path = Path(sys.argv[4])
runner_breakdown_path = Path(sys.argv[5])
artifact_policy_path = Path(sys.argv[6])
vm_retirement_path = Path(sys.argv[7])
role_ssot_path = Path(sys.argv[8])
task_order_path = Path(sys.argv[9])
state_path = Path(sys.argv[10])
index_path = Path(sys.argv[11])

card = card_path.read_text(encoding="utf-8")
ssot = ssot_path.read_text(encoding="utf-8")
fixture = json.loads(fixture_path.read_text(encoding="utf-8"))
design_stop = design_stop_path.read_text(encoding="utf-8")
runner_breakdown = runner_breakdown_path.read_text(encoding="utf-8")
artifact_policy = artifact_policy_path.read_text(encoding="utf-8")
vm_retirement = vm_retirement_path.read_text(encoding="utf-8")
role_ssot = role_ssot_path.read_text(encoding="utf-8")
task_order = task_order_path.read_text(encoding="utf-8")
state = tomllib.loads(state_path.read_text(encoding="utf-8"))
index = index_path.read_text(encoding="utf-8")

def require(condition: bool, message: str) -> None:
    if not condition:
        raise SystemExit(message)

token = "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-BASIS-001"
contract = "rust-lifecycle-source-selfhost-wider-route-selection-basis-v0"
design_stop_token = "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001"

require(f"# {token}" in card, "card token drift")
require(contract in card, "card contract drift")
require(design_stop_token in card, "card must preserve design stop")
require("KeepSourceSelfhostStopped" in card, "card missing keep-stopped basis")
require("runner_semantic_owner = 0" in card, "card runner contract drift")

require("# Source Selfhost Wider Route-Selection Basis" in ssot, "SSOT title drift")
for needle in [
    "ConsultationGatedWiderRouteSelection",
    "MachineDerivedRouteRepair",
    "KeepSourceSelfhostStopped",
    "current vm-hako",
    "future interpreter",
    "one .hako source/projector",
]:
    require(needle in ssot, f"SSOT missing {needle}")

require(fixture.get("kind") == "SourceSelfhostWiderRouteSelectionBasisV1", "fixture kind drift")
require(fixture.get("output_contract") == contract, "fixture output contract drift")
require((fixture.get("current_state") or {}).get("latest_card") == token, "fixture latest card drift")
require((fixture.get("current_state") or {}).get("current_blocker_token") == design_stop_token, "fixture blocker drift")

basis = fixture.get("basis") or {}
require(basis.get("kind") == "KeepSourceSelfhostStopped", "fixture basis drift")
require(basis.get("reason_token") == "NoEligibleNativeAdoptionCandidate", "fixture reason drift")
require(basis.get("allowed_resume") == [
    "ConsultationGatedWiderRouteSelection",
    "MachineDerivedRouteRepair",
], "fixture resume drift")

runner = fixture.get("runner_vocabulary") or {}
require((runner.get("current_vm_hako") or {}).get("co_mainline_product_lane") is False, "vm-hako co-mainline drift")
require((runner.get("exe_aot") or {}).get("semantic_owner") is False, "exe/aot semantic-owner drift")
require((runner.get("future_interpreter") or {}).get("active") is False, "future interpreter active drift")
require((runner.get("future_interpreter") or {}).get("required_for_python_to_hako_projector_migration") is False, "future interpreter prerequisite drift")

claims = fixture.get("claims") or {}
for key in [
    "runner_semantic_owner",
    "single_hako_meaning_source",
    "future_interpreter_required_for_projector_migration",
    "exe_aot_gate_is_semantic_owner",
    "vm_hako_co_mainline_claim",
    "consultation_gated_wider_route_selection",
    "machine_derived_route_repair_allowed",
]:
    require(key in claims, f"missing claim key {key}")
require(claims.get("runner_semantic_owner") == 0, "runner semantic-owner claim drift")
require(claims.get("single_hako_meaning_source") == 1, "single meaning source drift")
require(claims.get("future_interpreter_required_for_projector_migration") == 0, "future interpreter claim drift")
require(claims.get("exe_aot_gate_is_semantic_owner") == 0, "exe/aot claim drift")
require(claims.get("vm_hako_co_mainline_claim") == 0, "vm-hako claim drift")
require(claims.get("consultation_gated_wider_route_selection") == 1, "consultation claim drift")
require(claims.get("machine_derived_route_repair_allowed") == 1, "repair claim drift")

require(design_stop_token in runner_breakdown, "runner breakdown provenance missing")
require("SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-BASIS-001" in runner_breakdown, "runner breakdown next pack missing")
require("current `vm-hako` is not a co-mainline candidate" in artifact_policy, "artifact policy provenance missing")
require("A future interpreter artifact is reserved" in artifact_policy, "artifact policy future-interpreter boundary drift")
require("hako_vm_active_product_target=0" in vm_retirement, "vm retirement boundary drift")
require("interactive_interpreter_active_product_target=0" in vm_retirement, "vm retirement interpreter boundary drift")
require("backend/interpreter directly interprets compiler policy facts as a second" in role_ssot, "role SSOT semantic consumer drift")

for needle in [
    token,
    contract,
    design_stop_token,
    "ConsultationGatedWiderRouteSelection",
    "MachineDerivedRouteRepair",
    "runner_semantic_owner = 0",
    "future_interpreter_required_for_projector_migration = 0",
]:
    require(needle in task_order, f"task-order missing {needle}")

require(state.get("latest_card") == token, "CURRENT_STATE latest card drift")
require(state.get("current_blocker_token") == design_stop_token, "CURRENT_STATE blocker drift")
require("tools/checks/rust_lifecycle_source_selfhost_wider_route_selection_basis_guard.sh" in index, "check index missing guard")

print(f"output_contract={contract}")
print(f"current_blocker_preserved={design_stop_token}")
print("basis_kind=KeepSourceSelfhostStopped")
print("reason_token=NoEligibleNativeAdoptionCandidate")
print("next_action=SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001")
print("single_hako_meaning_source=1")
print("runner_semantic_owner=0")
print("future_interpreter_required_for_projector_migration=0")
print("manual_family_selection=0")
print("runtime_fallback=0")
print("new_backend_route=0")
print("new_abi=0")
print("source_selfhost_claim=0")
print("summary=ok")
PY
