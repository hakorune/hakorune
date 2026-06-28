#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
cd "$ROOT_DIR"

CARD="docs/development/current/main/phases/phase-296x/1780-SOURCE-SELFHOST-ADOPTION-PLAN-001.md"
FIXTURE="docs/development/current/main/design/fixtures/rust-lifecycle/source-selfhost-adoption-plan-v0.json"
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

token = "SOURCE-SELFHOST-ADOPTION-PLAN-001"
output_contract = "rust-lifecycle-source-selfhost-adoption-plan-v0"

require(f"# {token}" in card, "card token drift")
require(f"output_contract={output_contract}" in card, "card output contract drift")
require("docs_only_closeout = forbidden" in card, "card must keep docs-only closeout forbidden")
require("code_or_artifact_delta_required = 1" in card, "card must require a code/artifact delta")
require("source_selfhost_adoption_plan_token = SOURCE-SELFHOST-ADOPTION-PLAN-001" in card, "card token field drift")
require("artifact_selfhost_checkpoint_provenance = 1" in card, "card checkpoint provenance drift")
require("mainline_pilot_provenance = 1" in card, "card mainline provenance drift")
require("candidate_pool_state = Blocked" in card, "card candidate pool drift")
require("manual_family_selection = 0" in card, "card manual family selection drift")
require("next_family_specific_hakoadopted_decision_machine_derived = 1" in card, "card machine-derived decision drift")
require("python_oracle_retained = 1" in card, "card python oracle drift")
require("rust_compat_reference_retained = 1" in card, "card rust compat drift")
require("Source Selfhost = 0" in card, "card non-claim drift")
require("HakoAdopted = 0" in card, "card non-claim drift")

require(fixture.get("kind") == "SourceSelfhostAdoptionPlanV1", "fixture kind drift")
require(fixture.get("output_contract") == output_contract, "fixture output contract drift")
require(fixture.get("source_selfhost_adoption_plan_token") == token, "fixture token drift")

fixture_state = fixture.get("current_state") or {}
require(fixture_state.get("latest_card") == token, "fixture latest card drift")
require(
    fixture_state.get("latest_card_path")
    == "docs/development/current/main/phases/phase-296x/1780-SOURCE-SELFHOST-ADOPTION-PLAN-001.md",
    "fixture latest card path drift",
)
require(fixture_state.get("current_blocker_token") == token, "fixture current blocker drift")

provenance = fixture.get("provenance") or {}
require(provenance.get("artifact_selfhost_checkpoint") == "ARTIFACT-SELFHOST-CHECKPOINT-001", "fixture checkpoint provenance drift")
require(provenance.get("mainline_pilot") == "MIRBUILDER-MINIMAL-PATH-MAINLINE-PILOT-001", "fixture mainline provenance drift")
require(provenance.get("route_matrix_closeout") == "MIRBUILDER-VARIABLE-CONTEXT-ROUTE-MATRIX-CLOSEOUT-001", "fixture route closeout provenance drift")
require(provenance.get("candidate_selection") == "MIRBUILDER-NEXT-HAKO-ADOPTION-CANDIDATE-SELECTION-001", "fixture candidate selection provenance drift")

plan = fixture.get("plan") or {}
require(plan.get("candidate_pool_state") == "Blocked", "fixture candidate pool drift")
require(plan.get("manual_family_selection") == 0, "fixture manual family selection drift")
require(plan.get("next_family_specific_hakoadopted_decision_machine_derived") == 1, "fixture machine-derived decision drift")
require(plan.get("python_oracle_retained") == 1, "fixture python oracle drift")
require(plan.get("rust_compat_reference_retained") == 1, "fixture rust compat drift")

claims = fixture.get("claims") or {}
for key in [
    "runtime_fallback",
    "new_backend_route",
    "new_abi",
    "source_selfhost_claim",
    "hako_adopted",
]:
    require(claims.get(key) == 0, f"fixture claim drift: {key}")

current_latest = state.get("latest_card")
current_blocker = state.get("current_blocker_token")
allowed_current_tokens = {
    token,
    "SOURCE-SELFHOST-BLOCKED-RECOVERY-DIAGNOSTIC-001",
    "VARIABLE-CONTEXT-NATIVE-SURFACE-ADOPTION-SELECTION-001",
    "VARIABLE-CONTEXT-NATIVE-SURFACE-HAKO-ADOPTION-DECISION-001",
    "SOURCE-SELFHOST-POST-VARIABLE-CONTEXT-SURFACE-RESOLUTION-001",
    "MIRBUILDER-VARIABLE-CONTEXT-RETURNED-READ-SNAPSHOT-ROUTE-001",
    "MIRBUILDER-VARIABLE-CONTEXT-OWNED-READ-SNAPSHOT-PROJECTION-001",
    "MIRBUILDER-VARIABLE-CONTEXT-ROUTE-MATRIX-RERUN-001",
}
require(current_latest in allowed_current_tokens, "current-state latest card drift")
require(current_blocker in allowed_current_tokens, "current-state blocker drift")
require(Path(state.get("latest_card_path", "")).exists(), "current-state latest card path missing")

roadmap_requirements = [
    "SOURCE-SELFHOST-ADOPTION-PLAN-001",
    "ARTIFACT-SELFHOST-CHECKPOINT-001",
    "MAINLINE-SELFHOST-PILOT-001",
]
for needle in roadmap_requirements:
    require(needle in roadmap, f"roadmap missing {needle}")

task_order_requirements = [
    "active blocker:",
    "SOURCE-SELFHOST-ADOPTION-PLAN-001",
    "selected next owner:",
    "candidate_pool_state = Blocked",
]
for needle in task_order_requirements:
    require(needle in task_order, f"task-order missing {needle}")

require("tools/checks/rust_lifecycle_source_selfhost_adoption_plan_guard.sh" in index, "check-scripts index missing guard entry")

print("output_contract=rust-lifecycle-source-selfhost-adoption-plan-v0")
print("source_selfhost_adoption_plan_token=SOURCE-SELFHOST-ADOPTION-PLAN-001")
print("candidate_pool_state=Blocked")
print("manual_family_selection=0")
print("next_family_specific_hakoadopted_decision_machine_derived=1")
print("python_oracle_retained=1")
print("rust_compat_reference_retained=1")
print("current_state_pointer=green")
print("summary=ok")
PY
