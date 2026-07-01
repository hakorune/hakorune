#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
cd "$ROOT_DIR"

TAG="rust-lifecycle-source-selfhost-post-rerun-006-next-lane-selector"

source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

CARD="docs/development/current/main/phases/phase-296x/1975-SOURCE-SELFHOST-POST-RERUN-006-NEXT-LANE-SELECTOR-001.md"
FIXTURE="docs/development/current/main/design/fixtures/rust-lifecycle/source-selfhost-post-rerun-006-next-lane-selector-v0.json"
BASIS="docs/development/current/main/design/fixtures/rust-lifecycle/source-selfhost-wider-route-selection-basis-002-v0.json"
RERUN="docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-crate-wide-native-owner-seed-capability-survey-rerun-006-v0.json"
REPORT="docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-crate-wide-unconverted-surface-report-v0.json"
MANIFEST="docs/development/current/main/design/fixtures/rust-lifecycle/source-selfhost-family-guard-manifest-v0.json"
TASK_ORDER="docs/development/current/main/design/mirbuilder-rust-to-hako-converter-task-order-ssot.md"
STATE="docs/development/current/main/CURRENT_STATE.toml"

guard_require_files "$TAG" "$CARD" "$FIXTURE" "$BASIS" "$RERUN" "$REPORT" "$MANIFEST" "$TASK_ORDER" "$STATE"

python3 - "$CARD" "$FIXTURE" "$BASIS" "$RERUN" "$REPORT" "$MANIFEST" "$TASK_ORDER" "$STATE" <<'PY'
from __future__ import annotations

import hashlib
import json
import sys
import tomllib
from pathlib import Path

card_path = Path(sys.argv[1])
fixture_path = Path(sys.argv[2])
basis_path = Path(sys.argv[3])
rerun_path = Path(sys.argv[4])
report_path = Path(sys.argv[5])
manifest_path = Path(sys.argv[6])
task_order_path = Path(sys.argv[7])
state_path = Path(sys.argv[8])

card = card_path.read_text(encoding="utf-8")
fixture = json.loads(fixture_path.read_text(encoding="utf-8"))
basis = json.loads(basis_path.read_text(encoding="utf-8"))
rerun = json.loads(rerun_path.read_text(encoding="utf-8"))
report = json.loads(report_path.read_text(encoding="utf-8"))
task_order = task_order_path.read_text(encoding="utf-8")
state = tomllib.loads(state_path.read_text(encoding="utf-8"))

def require(condition: bool, message: str) -> None:
    if not condition:
        raise SystemExit(message)

def sha256(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()

token = "SOURCE-SELFHOST-POST-RERUN-006-NEXT-LANE-SELECTOR-001"
contract = "rust-lifecycle-source-selfhost-post-rerun-006-next-lane-selector-v0"
design_stop = "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001"
next_card = "MIRBUILDER-UNCONVERTED-SURFACE-REPORT-RERUN-002"
reason = "UnconvertedSurfaceReportFreshnessCheckRequiredAfterBasis002"

require(f"# {token}" in card, "card token drift")
for needle in [
    contract,
    "SelectUnconvertedSurfaceReportRerun",
    reason,
    next_card,
    "manual_family_selection = 0",
    "source_selfhost_claim = 0",
]:
    require(needle in card, f"card missing {needle}")

require(fixture.get("kind") == "SourceSelfhostPostRerun006NextLaneSelectorV1", "fixture kind drift")
require(fixture.get("token") == token, "fixture token drift")
require(fixture.get("output_contract") == contract, "fixture contract drift")

require(basis.get("token") == "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-BASIS-002", "basis token drift")
require(basis.get("basis", {}).get("kind") == "PostStrictEmissionBridgeExhaustionSelectorBasis", "basis kind drift")
require(basis.get("selection_contract", {}).get("selector_fixture_required_before_implementation") is True, "basis selector contract drift")

rerun_decision = rerun.get("decision") or {}
rerun_pool = rerun.get("candidate_pool") or {}
require(rerun.get("token") == "MIRBUILDER-CRATE-WIDE-NATIVE-OWNER-SEED-CAPABILITY-SURVEY-RERUN-006", "rerun token drift")
require(rerun_decision.get("kind") == "KeepStopped", "rerun decision drift")
require(rerun_decision.get("selected_next_card") == design_stop, "rerun next card drift")
require(rerun_pool.get("bridge_eligible_count") == 0, "rerun bridge eligible count drift")

allowed = fixture.get("basis", {}).get("allowed_decisions")
require(allowed == basis.get("basis", {}).get("allowed_decisions"), "allowed decisions must match BASIS-002")

report_decision = report.get("decision") or {}
report_projection_hash = (
    (report.get("provenance") or {}).get("projection_descriptor_ledger_hash")
)
evidence = fixture.get("evidence") or {}
require(report_decision.get("kind") == "KeepStopped", "report decision drift")
require(report_decision.get("reason_token") == "AmbiguousUnconvertedSurfaceCandidates", "report reason drift")
require(evidence.get("report_projection_descriptor_ledger_hash") == report_projection_hash, "fixture projection ledger hash drift")
require(evidence.get("unconverted_surface_report_freshness_check_required") is True, "freshness-check flag drift")
require(evidence.get("rerun_006_bridge_eligible_count") == 0, "fixture rerun bridge count drift")
require(evidence.get("strict_deny_unclosed_near_miss_cluster_count") == 0, "strict-deny unclosed count drift")

decision = fixture.get("decision") or {}
require(decision.get("kind") == "SelectUnconvertedSurfaceReportRerun", "decision kind drift")
require(decision.get("reason_token") == reason, "decision reason drift")
require(decision.get("selected_next_card") == next_card, "decision next card drift")
require(decision.get("selected_owner_edge_id") is None, "selector must not choose owner")

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

require(state.get("current_blocker_token") == design_stop, "CURRENT_STATE blocker drift")

for needle in [
    token,
    contract,
    "post_rerun_006_task_order",
    next_card,
    reason,
]:
    require(needle in task_order, f"task-order missing {needle}")

print(f"output_contract={contract}")
print(f"current_blocker_preserved={design_stop}")
print("decision=SelectUnconvertedSurfaceReportRerun")
print(f"reason_token={reason}")
print(f"selected_next_card={next_card}")
print("unconverted_surface_report_freshness_check_required=1")
print("manual_family_selection=0")
print("manual_shape_selection=0")
print("manual_axis_selection=0")
print("source_selfhost_claim=0")
print("summary=ok")
PY
