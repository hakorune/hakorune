#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
cd "$ROOT_DIR"

CARD="docs/development/current/main/phases/phase-296x/1779-ARTIFACT-SELFHOST-CHECKPOINT-001.md"
FIXTURE="docs/development/current/main/design/fixtures/rust-lifecycle/artifact-selfhost-checkpoint-v0.json"
STATE="docs/development/current/main/CURRENT_STATE.toml"
ROADMAP="docs/development/current/main/design/mirbuilder-selfhost-checkpoint-roadmap-ssot.md"
TASK_ORDER="docs/development/current/main/design/mirbuilder-rust-to-hako-converter-task-order-ssot.md"
INDEX="docs/tools/check-scripts-index.md"

python3 - "$CARD" "$FIXTURE" "$STATE" "$ROADMAP" "$TASK_ORDER" "$INDEX" <<'PY'
from __future__ import annotations

import json
import sys
import tomllib
from pathlib import Path

card_path = Path(sys.argv[1])
fixture_path = Path(sys.argv[2])
state_path = Path(sys.argv[3])
roadmap_path = Path(sys.argv[4])
task_order_path = Path(sys.argv[5])
index_path = Path(sys.argv[6])

card = card_path.read_text(encoding="utf-8")
fixture = json.loads(fixture_path.read_text(encoding="utf-8"))
state = tomllib.loads(state_path.read_text(encoding="utf-8"))
roadmap = roadmap_path.read_text(encoding="utf-8")
task_order = task_order_path.read_text(encoding="utf-8")
index = index_path.read_text(encoding="utf-8")

def require(condition: bool, message: str) -> None:
    if not condition:
        raise SystemExit(message)

token = "ARTIFACT-SELFHOST-CHECKPOINT-001"
output_contract = "rust-lifecycle-artifact-selfhost-checkpoint-v0"

require(f"# {token}" in card, "card token drift")
require(f"output_contract={output_contract}" in card, "card output contract drift")
require("docs_only_closeout = forbidden" in card, "card must keep docs-only closeout forbidden")
require("code_or_artifact_delta_required = 1" in card, "card must require a code/artifact delta")
require("artifact_selfhost_checkpoint_token = ARTIFACT-SELFHOST-CHECKPOINT-001" in card, "card checkpoint token drift")
require("candidate_pool_state = Blocked" in card, "card candidate pool drift")
require("composed_execution_evidence_consumed = 1" in card, "card composed evidence drift")
require("same_state_handoff_observed = 1" in card, "card same-state drift")
require("generated_hako_executable_closure = Closed" in card, "card executable closure drift")
require("next_queue_item_machine_derived = 1" in card, "card next-queue derivation drift")
require("checkpoint_guard_green = 1" in card, "card checkpoint guard claim drift")
require("MAINLINE-SELFHOST-PILOT-001" in card, "card next follow-on drift")
require("SOURCE-SELFHOST-ADOPTION-PLAN-001" in card, "card next follow-on drift")

require(fixture.get("kind") == "ArtifactSelfhostCheckpointV1", "fixture kind drift")
require(fixture.get("output_contract") == output_contract, "fixture output contract drift")
require(fixture.get("artifact_selfhost_checkpoint_token") == token, "fixture token drift")

fixture_state = fixture.get("current_state") or {}
require(fixture_state.get("latest_card") == token, "fixture latest card drift")
require(
    fixture_state.get("latest_card_path")
    == "docs/development/current/main/phases/phase-296x/1779-ARTIFACT-SELFHOST-CHECKPOINT-001.md",
    "fixture latest card path drift",
)
require(fixture_state.get("current_blocker_token") == token, "fixture current blocker drift")

checkpoint = fixture.get("checkpoint") or {}
require(checkpoint.get("candidate_pool_state") == "Blocked", "fixture candidate pool drift")
require(checkpoint.get("composed_execution_evidence_consumed") == 1, "fixture composed evidence drift")
require(checkpoint.get("same_state_handoff_observed") == 1, "fixture same-state drift")
require(checkpoint.get("generated_hako_executable_closure") == "Closed", "fixture executable closure drift")
require(checkpoint.get("next_queue_item_machine_derived") == 1, "fixture next-queue drift")

claims = fixture.get("claims") or {}
for key in [
    "manual_next_owner_selection",
    "runtime_fallback",
    "new_backend_route",
    "new_abi",
    "source_selfhost_claim",
]:
    require(claims.get(key) == 0, f"fixture claim drift: {key}")

require(state.get("latest_card") == token, "current-state latest card drift")
require(
    state.get("latest_card_path")
    == "docs/development/current/main/phases/phase-296x/1779-ARTIFACT-SELFHOST-CHECKPOINT-001.md",
    "current-state latest card path drift",
)
require(state.get("current_blocker_token") == token, "current-state blocker drift")

roadmap_requirements = [
    "ARTIFACT-SELFHOST-CHECKPOINT-001",
    "MAINLINE-SELFHOST-PILOT-001",
    "SOURCE-SELFHOST-ADOPTION-PLAN-001",
]
for needle in roadmap_requirements:
    require(needle in roadmap, f"roadmap missing {needle}")

task_order_requirements = [
    "active blocker:",
    "ARTIFACT-SELFHOST-CHECKPOINT-001",
    "selected next owner:",
    "Artifact selfhost checkpoint",
    "candidate_pool_state = Blocked",
]
for needle in task_order_requirements:
    require(needle in task_order, f"task-order missing {needle}")

require("tools/checks/rust_lifecycle_artifact_selfhost_checkpoint_guard.sh" in index, "check-scripts index missing guard entry")

print("output_contract=rust-lifecycle-artifact-selfhost-checkpoint-v0")
print("artifact_selfhost_checkpoint_token=ARTIFACT-SELFHOST-CHECKPOINT-001")
print("candidate_pool_state=Blocked")
print("composed_execution_evidence_consumed=1")
print("same_state_handoff_observed=1")
print("generated_hako_executable_closure=Closed")
print("next_queue_item_machine_derived=1")
print("current_state_pointer=green")
print("summary=ok")
PY
