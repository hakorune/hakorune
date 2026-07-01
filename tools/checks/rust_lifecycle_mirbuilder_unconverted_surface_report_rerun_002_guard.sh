#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="rust-lifecycle-mirbuilder-unconverted-surface-report-rerun-002"
source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

TOOL="$ROOT_DIR/tools/rust_lifecycle/mirbuilder_crate_wide_unconverted_surface_report.py"
FIXTURE="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-crate-wide-unconverted-surface-report-v0.json"
MANIFEST="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/source-selfhost-family-guard-manifest-v0.json"
SURVEY="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-crate-wide-native-owner-seed-capability-survey-v0.json"
CARD="$ROOT_DIR/docs/development/current/main/phases/phase-296x/1976-MIRBUILDER-UNCONVERTED-SURFACE-REPORT-RERUN-002.md"
TASK_ORDER="$ROOT_DIR/docs/development/current/main/design/mirbuilder-rust-to-hako-converter-task-order-ssot.md"
STATE="$ROOT_DIR/docs/development/current/main/CURRENT_STATE.toml"

guard_require_command "$TAG" python3
guard_require_files "$TAG" "$TOOL" "$FIXTURE" "$MANIFEST" "$SURVEY" "$CARD" "$TASK_ORDER" "$STATE"

cd "$ROOT_DIR"
python3 "$TOOL" --check

python3 - "$CARD" "$FIXTURE" "$MANIFEST" "$SURVEY" "$TASK_ORDER" "$STATE" <<'PY'
from __future__ import annotations

import hashlib
import json
import sys
import tomllib
from pathlib import Path

card_path = Path(sys.argv[1])
fixture_path = Path(sys.argv[2])
manifest_path = Path(sys.argv[3])
survey_path = Path(sys.argv[4])
task_order_path = Path(sys.argv[5])
state_path = Path(sys.argv[6])

card = card_path.read_text(encoding="utf-8")
fixture = json.loads(fixture_path.read_text(encoding="utf-8"))
manifest = json.loads(manifest_path.read_text(encoding="utf-8"))
task_order = task_order_path.read_text(encoding="utf-8")
state = tomllib.loads(state_path.read_text(encoding="utf-8"))

def require(condition: bool, message: str) -> None:
    if not condition:
        raise SystemExit(message)

def sha256_file(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()

def projection_ledger_hash(manifest_obj: dict) -> str:
    rows = [
        {
            "token": row.get("token"),
            "fixture": row.get("fixture"),
        }
        for row in manifest_obj.get("rows", [])
        if "PROJECTION-POLICY" in (row.get("token") or "")
    ]
    payload = json.dumps(rows, sort_keys=True, separators=(",", ":")).encode("utf-8")
    return hashlib.sha256(payload).hexdigest()

token = "MIRBUILDER-UNCONVERTED-SURFACE-REPORT-RERUN-002"
next_card = "MIRBUILDER-CRATE-WIDE-NATIVE-OWNER-SEED-CAPABILITY-SURVEY-RERUN-007"
design_stop = "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001"

require(token in card, "card missing token")
for needle in [
    "projection_descriptor_ledger_hash_fresh = 1",
    next_card,
    "source_selfhost_claim = 0",
]:
    require(needle in card, f"card missing {needle}")

require(fixture.get("kind") == "MirBuilderCrateWideUnconvertedSurfaceReportV1", "fixture kind drift")
require(fixture.get("token") == "MIRBUILDER-CRATE-WIDE-UNCONVERTED-SURFACE-REPORT-001", "source report token drift")

provenance = fixture.get("provenance") or {}
require(provenance.get("projection_descriptor_ledger_hash") == projection_ledger_hash(manifest), "projection ledger hash is stale")
require(provenance.get("source_selfhost_family_guard_manifest_hash") == projection_ledger_hash(manifest), "manifest projection hash is stale")
require(provenance.get("native_owner_seed_capability_survey_hash") == sha256_file(survey_path), "native seed survey hash is stale")
require(provenance.get("tool_version") == "regex_source_text_v0", "tool version drift")

summary = fixture.get("summary") or {}
require(summary.get("scanned_surface_count") == 1584, "scanned surface count drift")
require(summary.get("classified_once_count") == summary.get("scanned_surface_count"), "classified count must match scanned count")
require(summary.get("missing_projection_policy_count") == 1384, "missing projection count drift")
require(summary.get("unmapped_count") == 0, "unmapped count drift")

decision = fixture.get("decision") or {}
require(decision.get("kind") == "KeepStopped", "decision kind drift")
require(decision.get("reason_token") == "AmbiguousUnconvertedSurfaceCandidates", "decision reason drift")
require(decision.get("selected_next_card") == design_stop, "decision next card drift")

claims = fixture.get("claims") or {}
for key in [
    "manual_family_selection",
    "source_selfhost_claim",
    "runtime_fallback",
    "new_backend_route",
    "new_abi",
    "new_python_semantic_projector",
]:
    require(claims.get(key) == 0, f"forbidden claim drift: {key}")

require(state.get("latest_card") == token, "CURRENT_STATE latest card drift")
expected_card_path = "docs/development/current/main/phases/phase-296x/1976-MIRBUILDER-UNCONVERTED-SURFACE-REPORT-RERUN-002.md"
require(state.get("latest_card_path") == expected_card_path, "CURRENT_STATE latest path drift")
require(state.get("current_blocker_token") == design_stop, "CURRENT_STATE blocker drift")

for needle in [
    token,
    next_card,
    "projection_descriptor_ledger_hash_fresh = 1",
]:
    require(needle in task_order, f"task-order missing {needle}")

print("output_contract=rust-lifecycle-mirbuilder-unconverted-surface-report-rerun-002")
print("projection_descriptor_ledger_hash_fresh=1")
print("scanned_surface_count=1584")
print("missing_projection_policy_count=1384")
print(f"selected_next_card={next_card}")
print("manual_family_selection=0")
print("source_selfhost_claim=0")
print("runtime_fallback=0")
print("summary=ok")
PY
