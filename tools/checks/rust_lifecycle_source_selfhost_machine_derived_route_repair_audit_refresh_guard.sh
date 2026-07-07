#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
cd "$ROOT_DIR"

TAG="rust-lifecycle-source-selfhost-machine-derived-route-repair-audit-refresh"

source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

CARD="docs/development/current/main/phases/phase-296x/3323-SOURCE-SELFHOST-MACHINE-DERIVED-ROUTE-REPAIR-AUDIT-REFRESH-001.md"
FIXTURE="docs/development/current/main/design/fixtures/rust-lifecycle/source-selfhost-machine-derived-route-repair-audit-refresh-v0.json"
DESIGN_STOP_FIXTURE="docs/development/current/main/design/fixtures/rust-lifecycle/source-selfhost-wider-route-selection-design-stop-v0.json"
BASIS_011="docs/development/current/main/design/fixtures/rust-lifecycle/source-selfhost-wider-route-selection-basis-011-v0.json"
RESOLUTION="docs/development/current/main/design/fixtures/rust-lifecycle/source-selfhost-wider-route-selection-resolution-v0.json"
STATE="docs/development/current/main/CURRENT_STATE.toml"
TASK_ORDER="docs/development/current/main/design/mirbuilder-rust-to-hako-converter-task-order-ssot.md"
INDEX="docs/tools/check-scripts-index.md"

guard_require_files "$TAG" \
  "$CARD" \
  "$FIXTURE" \
  "$DESIGN_STOP_FIXTURE" \
  "$BASIS_011" \
  "$RESOLUTION" \
  "$STATE" \
  "$TASK_ORDER" \
  "$INDEX"

python3 - "$CARD" "$FIXTURE" "$DESIGN_STOP_FIXTURE" "$BASIS_011" "$RESOLUTION" "$STATE" "$TASK_ORDER" "$INDEX" <<'PY'
from __future__ import annotations

import json
import sys
import tomllib
from pathlib import Path

card_path = Path(sys.argv[1])
fixture_path = Path(sys.argv[2])
design_stop_path = Path(sys.argv[3])
basis_011_path = Path(sys.argv[4])
resolution_path = Path(sys.argv[5])
state_path = Path(sys.argv[6])
task_order_path = Path(sys.argv[7])
index_path = Path(sys.argv[8])

card = card_path.read_text(encoding="utf-8")
fixture = json.loads(fixture_path.read_text(encoding="utf-8"))
design_stop = json.loads(design_stop_path.read_text(encoding="utf-8"))
basis_011 = json.loads(basis_011_path.read_text(encoding="utf-8"))
resolution = json.loads(resolution_path.read_text(encoding="utf-8"))
state = tomllib.loads(state_path.read_text(encoding="utf-8"))
task_order = task_order_path.read_text(encoding="utf-8")
index = index_path.read_text(encoding="utf-8")


def need(condition: bool, message: str) -> None:
    if not condition:
        raise SystemExit(message)


token = "SOURCE-SELFHOST-MACHINE-DERIVED-ROUTE-REPAIR-AUDIT-REFRESH-001"
design_stop_token = "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001"
output_contract = "rust-lifecycle-source-selfhost-machine-derived-route-repair-audit-refresh-v0"

need(f"# 3323 - {token}" in card, "card token drift")
need(output_contract in card, "card output contract drift")
need("NoCurrentMachineDerivedRouteRepairCandidate" in card, "card reason drift")
need("current_unblock_repair_count:\n  0" in card, "card unblock count drift")

need(fixture.get("kind") == "SourceSelfhostMachineDerivedRouteRepairAuditRefreshV1", "fixture kind drift")
need(fixture.get("token") == token, "fixture token drift")
need(fixture.get("output_contract") == output_contract, "fixture output contract drift")
need((fixture.get("current_state") or {}).get("latest_card") == token, "fixture latest card drift")
need((fixture.get("current_state") or {}).get("current_blocker_token") == design_stop_token, "fixture blocker drift")

allowed = fixture.get("allowed_resume") or []
need(allowed == ["ConsultationGatedWiderRouteSelection", "MachineDerivedRouteRepair"], "allowed resume drift")

summary = fixture.get("audit_summary") or {}
need(summary.get("audited_repair_fixture_count") == 5, "audited repair fixture count drift")
need(summary.get("historical_or_consumed_repair_count") == 5, "historical repair count drift")
need(summary.get("current_unblock_repair_count") == 0, "current unblock repair count drift")
need(summary.get("route_matrix_concrete_inconsistency_count") == 0, "route inconsistency count drift")
need(summary.get("eligible_candidate_count") == 0, "eligible candidate count drift")
need(summary.get("candidate_pool_state") == "Blocked", "candidate pool state drift")
need(summary.get("machine_derived_route_repair_selected") == 0, "repair selection drift")

repairs = fixture.get("audited_repair_fixtures") or []
need(len(repairs) == summary.get("audited_repair_fixture_count"), "repair fixture list count drift")
for row in repairs:
    need(row.get("state") == "HistoricalConsumedRepair", f"unexpected repair state: {row}")
    need(row.get("current_unblock_repair") == 0, f"unexpected current unblock repair: {row}")
    need(Path(row.get("fixture", "")).exists(), f"repair fixture missing: {row}")

decision = fixture.get("decision") or {}
need(decision.get("kind") == "KeepSourceSelfhostStopped", "decision kind drift")
need(decision.get("reason_token") == "NoCurrentMachineDerivedRouteRepairCandidate", "reason token drift")
need(decision.get("selected_next_card") == design_stop_token, "selected next drift")
need(decision.get("next_action") == "DesignConsultationRequired", "next action drift")

claims = fixture.get("claims") or {}
for key in [
    "source_selfhost_claim",
    "consultation_gated_wider_route_selection",
    "machine_derived_route_repair_selected",
    "manual_family_selection",
    "manual_lane_selection",
    "route_membership_alone_as_proof",
    "coverage_percentage_as_proof",
    "route_selection",
    "runtime_route_switch",
    "programjson_runtime_route_authority",
    "runtime_fallback",
    "new_backend_route",
    "new_abi",
]:
    need(claims.get(key) == 0, f"forbidden claim drift: {key}")

need((design_stop.get("decision") or {}).get("kind") == "KeepSourceSelfhostStopped", "design-stop decision drift")
need((design_stop.get("recovery") or {}).get("resume_condition") == "ConsultationGatedWiderRouteSelectionOrMachineDerivedRouteRepair", "design-stop resume drift")
need((basis_011.get("decision") or {}).get("kind") == "KeepStopped", "basis-011 decision drift")
need((basis_011.get("decision") or {}).get("selected_next_card") == design_stop_token, "basis-011 next drift")
need((resolution.get("candidate_pool") or {}).get("eligible_count") == 0, "resolution eligible count drift")
need((resolution.get("candidate_pool") or {}).get("repairable_inconsistency_count") == 0, "resolution repairable count drift")

need(state.get("latest_card") == token, "CURRENT_STATE latest card drift")
need(state.get("latest_card_path") == str(card_path), "CURRENT_STATE latest path drift")
need(state.get("current_blocker_token") == design_stop_token, "CURRENT_STATE blocker drift")

for needle in [
    token,
    output_contract,
    "NoCurrentMachineDerivedRouteRepairCandidate",
    "current_unblock_repair_count = 0",
    "route_matrix_concrete_inconsistency_count = 0",
    design_stop_token,
]:
    need(needle in task_order, f"task-order missing {needle}")

need("tools/checks/rust_lifecycle_source_selfhost_machine_derived_route_repair_audit_refresh_guard.sh" in index, "check index missing guard")

print(f"output_contract={output_contract}")
print("decision=KeepSourceSelfhostStopped")
print("reason_token=NoCurrentMachineDerivedRouteRepairCandidate")
print("audited_repair_fixture_count=5")
print("current_unblock_repair_count=0")
print("route_matrix_concrete_inconsistency_count=0")
print("selected_next_card=SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001")
print("source_selfhost_claim=0")
print("runtime_route_switch=0")
print("summary=ok")
PY
