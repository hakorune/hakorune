#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
cd "$ROOT_DIR"

TAG="rust-lifecycle-mirbuilder-projection-descriptor-coverage-reclassification"

source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

CARD="docs/development/current/main/phases/phase-296x/2005-MIRBUILDER-PROJECTION-DESCRIPTOR-COVERAGE-RECLASSIFICATION-001.md"
REPORT="docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-crate-wide-unconverted-surface-report-v0.json"
V2="docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-missing-projection-policy-cluster-resolution-v2-v0.json"
TOOL="tools/rust_lifecycle/mirbuilder_crate_wide_unconverted_surface_report.py"
TASK_ORDER="docs/development/current/main/design/mirbuilder-rust-to-hako-converter-task-order-ssot.md"
STATE="docs/development/current/main/CURRENT_STATE.toml"

guard_require_files "$TAG" "$CARD" "$REPORT" "$V2" "$TOOL" "$TASK_ORDER" "$STATE"

python3 "$TOOL" --check

python3 - "$CARD" "$REPORT" "$V2" "$TASK_ORDER" "$STATE" <<'PY'
from __future__ import annotations

import json
import sys
import tomllib
from pathlib import Path

card_path = Path(sys.argv[1])
report_path = Path(sys.argv[2])
v2_path = Path(sys.argv[3])
task_order_path = Path(sys.argv[4])
state_path = Path(sys.argv[5])

card = card_path.read_text(encoding="utf-8")
report = json.loads(report_path.read_text(encoding="utf-8"))
v2 = json.loads(v2_path.read_text(encoding="utf-8"))
task_order = task_order_path.read_text(encoding="utf-8")
state = tomllib.loads(state_path.read_text(encoding="utf-8"))


def require(condition: bool, message: str) -> None:
    if not condition:
        raise SystemExit(message)


token = "MIRBUILDER-PROJECTION-DESCRIPTOR-COVERAGE-RECLASSIFICATION-001"
next_card = "MIRBUILDER-CRATE-WIDE-NATIVE-OWNER-SEED-CAPABILITY-SURVEY-RERUN-008"
design_stop = "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001"

require(token in card, "card missing token")
for needle in [
    "projection_descriptor_coverage_reclassified_count = 380",
    "missing_projection_policy_count = 1004",
    "mapped_to_known_owner_count = 398",
    next_card,
    "source_selfhost_claim = 0",
]:
    require(needle in card, f"card missing {needle}")

require((v2.get("decision") or {}).get("selected_next_card") == token, "V2 must select reclassification")
require((v2.get("cluster_state") or {}).get("excluded_existing_decision_cluster_count") == 41, "V2 excluded count drift")

summary = report.get("summary") or {}
require(summary.get("projection_descriptor_coverage_reclassified_count") == 380, "reclassified count drift")
require(summary.get("missing_projection_policy_count") == 1004, "missing projection count drift")
require(summary.get("mapped_to_known_owner_count") == 398, "mapped owner count drift")
require(summary.get("borrow_policy_needed_count") == 112, "borrow count drift")
require(summary.get("scanned_surface_count") == 1584, "surface count drift")

covered_items = [
    item for item in report.get("items", [])
    if item.get("reason_token") == "ProjectionDescriptorCoverageLanded"
]
require(len(covered_items) == 380, "covered item count drift")
for item in covered_items:
    require(item.get("classification") == "MappedToKnownOwner", "covered item classification drift")
    require(item.get("blockers") == [], "covered item must have no blockers")
    require(item.get("next_card") is None, "covered item must not select next card")
    require(item.get("covered_projection_cluster_id"), "covered item missing cluster id")

claims = report.get("claims") or {}
require(claims.get("projection_descriptor_coverage_reclassification") == 1, "coverage reclassification claim drift")
for key in [
    "manual_family_selection",
    "source_selfhost_claim",
    "runtime_fallback",
    "new_backend_route",
    "new_abi",
    "new_python_semantic_projector",
]:
    require(claims.get(key) == 0, f"forbidden claim drift: {key}")

decision = report.get("decision") or {}
require(decision.get("kind") == "KeepStopped", "report decision kind drift")
require(decision.get("selected_next_card") == design_stop, "report decision next drift")

require(state.get("latest_card") == token, "CURRENT_STATE latest card drift")
require(state.get("latest_card_path") == str(card_path), "CURRENT_STATE latest path drift")
require(state.get("current_blocker_token") == design_stop, "CURRENT_STATE blocker drift")

for needle in [
    token,
    next_card,
    "projection_descriptor_coverage_reclassified_count = 380",
    "missing_projection_policy_count = 1004",
]:
    require(needle in task_order, f"task-order missing {needle}")

print("output_contract=rust-lifecycle-mirbuilder-projection-descriptor-coverage-reclassification")
print("projection_descriptor_coverage_reclassified_count=380")
print("missing_projection_policy_count=1004")
print("mapped_to_known_owner_count=398")
print(f"selected_next_card={next_card}")
print("source_selfhost_claim=0")
print("summary=ok")
PY
