#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="rust-lifecycle-mirbuilder-unconverted-surface-report-rerun-004"
source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

TOOL="$ROOT_DIR/tools/rust_lifecycle/mirbuilder_crate_wide_unconverted_surface_report.py"
FIXTURE="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-crate-wide-unconverted-surface-report-v0.json"
MANIFEST="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/source-selfhost-family-guard-manifest-v0.json"
SURVEY="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-crate-wide-native-owner-seed-capability-survey-v0.json"
BASIS_007="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/source-selfhost-wider-route-selection-basis-007-v0.json"
ADOPTION="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-emission-ssa-phi-hako-adoption-decision-v0.json"
CARD="$ROOT_DIR/docs/development/current/main/phases/phase-296x/2057-MIRBUILDER-UNCONVERTED-SURFACE-REPORT-RERUN-004.md"
TASK_ORDER="$ROOT_DIR/docs/development/current/main/design/mirbuilder-rust-to-hako-converter-task-order-ssot.md"
STATE="$ROOT_DIR/docs/development/current/main/CURRENT_STATE.toml"

guard_require_command "$TAG" python3
guard_require_files "$TAG" "$TOOL" "$FIXTURE" "$MANIFEST" "$SURVEY" "$BASIS_007" "$ADOPTION" "$CARD" "$TASK_ORDER" "$STATE"

cd "$ROOT_DIR"
python3 "$TOOL" --check

python3 - "$CARD" "$FIXTURE" "$MANIFEST" "$SURVEY" "$BASIS_007" "$ADOPTION" "$TASK_ORDER" "$STATE" <<'PY'
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
basis_path = Path(sys.argv[5])
adoption_path = Path(sys.argv[6])
task_order_path = Path(sys.argv[7])
state_path = Path(sys.argv[8])

card = card_path.read_text(encoding="utf-8")
fixture = json.loads(fixture_path.read_text(encoding="utf-8"))
manifest = json.loads(manifest_path.read_text(encoding="utf-8"))
basis = json.loads(basis_path.read_text(encoding="utf-8"))
adoption = json.loads(adoption_path.read_text(encoding="utf-8"))
_task_order = task_order_path.read_text(encoding="utf-8")
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


def native_owner_adoption_ledger_hash(manifest_obj: dict) -> str:
    rows = [
        {
            "token": row.get("token"),
            "fixture": row.get("fixture"),
        }
        for row in manifest_obj.get("rows", [])
        if str(row.get("token") or "").endswith("-HAKO-ADOPTION-DECISION-001")
    ]
    payload = json.dumps(rows, sort_keys=True, separators=(",", ":")).encode("utf-8")
    return hashlib.sha256(payload).hexdigest()


def adoption_delta_after(manifest_obj: dict, token: str) -> list[str]:
    rows = manifest_obj.get("rows", [])
    start = 0
    for index, row in enumerate(rows):
        if row.get("token") == token:
            start = index + 1
            break
    return [
        str(row.get("token"))
        for row in rows[start:]
        if str(row.get("token") or "").endswith("-HAKO-ADOPTION-DECISION-001")
    ]


token = "MIRBUILDER-UNCONVERTED-SURFACE-REPORT-RERUN-004"
report_token = "MIRBUILDER-CRATE-WIDE-UNCONVERTED-SURFACE-REPORT-001"
design_stop = "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001"
next_task = "SOURCE-SELFHOST-NATIVE-OWNER-CHECKPOINT-RERUN-002"
previous_report = "MIRBUILDER-UNCONVERTED-SURFACE-REPORT-RERUN-003"
emission_adoption = "MIRBUILDER-EMISSION_SSA_PHI-HAKO-ADOPTION-DECISION-001"

require(token in card, "card missing token")
for needle in [
    "projection_descriptor_ledger_hash_fresh = 1",
    "native_owner_adoption_ledger_hash_fresh = 1",
    "native_owner_adoption_delta_after_rerun_003_count = 1",
    emission_adoption,
    next_task,
    "source_selfhost_claim = 0",
]:
    require(needle in card, f"card missing {needle}")

basis_decision = basis.get("decision") or {}
require(basis.get("token") == "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-BASIS-007", "basis token drift")
require(basis_decision.get("selected_next_card") == token, "basis selected next drift")
require(basis_decision.get("reason_token") == "SourceSurfaceReportStaleAfterEmissionSsaPhiAdoption", "basis reason drift")

adoption_claims = adoption.get("claims") or {}
require(adoption.get("token") == emission_adoption, "adoption token drift")
require(adoption_claims.get("hako_adopted") == 1, "emission adoption must be accepted")
require(adoption_claims.get("source_selfhost_claim") == 0, "emission adoption must not claim Source Selfhost")

require(fixture.get("kind") == "MirBuilderCrateWideUnconvertedSurfaceReportV1", "fixture kind drift")
require(fixture.get("token") == report_token, "source report token drift")

provenance = fixture.get("provenance") or {}
require(provenance.get("projection_descriptor_ledger_hash") == projection_ledger_hash(manifest), "projection ledger hash stale")
require(provenance.get("source_selfhost_family_guard_manifest_hash") == projection_ledger_hash(manifest), "manifest projection hash stale")
require(provenance.get("native_owner_adoption_ledger_hash") == native_owner_adoption_ledger_hash(manifest), "native owner adoption ledger hash stale")
require(provenance.get("native_owner_seed_capability_survey_hash") == sha256_file(survey_path), "native seed survey hash stale")
require(provenance.get("tool_version") == "regex_source_text_v0", "tool version drift")
require(adoption_delta_after(manifest, previous_report) == [emission_adoption], "native owner adoption delta drift")

summary = fixture.get("summary") or {}
require(summary.get("scanned_surface_count") == 1584, "scanned surface count drift")
require(summary.get("classified_once_count") == 1584, "classified count drift")
require(summary.get("missing_projection_policy_count") == 1004, "missing projection count drift")
require(summary.get("projection_descriptor_coverage_reclassified_count") == 380, "descriptor coverage count drift")
require(summary.get("borrow_policy_needed_count") == 112, "borrow policy count drift")
require(summary.get("unmapped_count") == 0, "unmapped count drift")

decision = fixture.get("decision") or {}
require(decision.get("kind") == "KeepStopped", "report decision kind drift")
require(decision.get("reason_token") == "AmbiguousUnconvertedSurfaceCandidates", "report decision reason drift")
require(decision.get("selected_next_card") == design_stop, "report decision next card drift")

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

require(state.get("current_blocker_token") == design_stop, "CURRENT_STATE blocker drift")

print("output_contract=rust-lifecycle-mirbuilder-unconverted-surface-report-rerun-004")
print("projection_descriptor_ledger_hash_fresh=1")
print("native_owner_adoption_ledger_hash_fresh=1")
print("native_owner_adoption_delta_after_rerun_003_count=1")
print("scanned_surface_count=1584")
print("missing_projection_policy_count=1004")
print("borrow_policy_needed_count=112")
print(f"recommended_next_task={next_task}")
print("manual_family_selection=0")
print("source_selfhost_claim=0")
print("summary=ok")
PY
