#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="rust-lifecycle-mirbuilder-unconverted-surface-report-rerun-guard"
source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

TOOL="$ROOT_DIR/tools/rust_lifecycle/mirbuilder_crate_wide_unconverted_surface_report.py"
FIXTURE="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-crate-wide-unconverted-surface-report-v0.json"
MANIFEST="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/source-selfhost-family-guard-manifest-v0.json"
SURVEY="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-crate-wide-native-owner-seed-capability-survey-v0.json"
CARD="$ROOT_DIR/docs/development/current/main/phases/phase-296x/1949-MIRBUILDER-UNCONVERTED-SURFACE-REPORT-RERUN-001.md"

guard_require_command "$TAG" python3
guard_require_files "$TAG" "$TOOL" "$FIXTURE" "$MANIFEST" "$SURVEY" "$CARD"

python3 "$TOOL" --check

python3 - <<'PY'
import hashlib
import json
from pathlib import Path

def sha256(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()

fixture_path = Path("docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-crate-wide-unconverted-surface-report-v0.json")
manifest_path = Path("docs/development/current/main/design/fixtures/rust-lifecycle/source-selfhost-family-guard-manifest-v0.json")
survey_path = Path("docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-crate-wide-native-owner-seed-capability-survey-v0.json")
card = Path("docs/development/current/main/phases/phase-296x/1949-MIRBUILDER-UNCONVERTED-SURFACE-REPORT-RERUN-001.md").read_text()
fixture = json.loads(fixture_path.read_text())

token = "MIRBUILDER-UNCONVERTED-SURFACE-REPORT-RERUN-001"
if token not in card:
    raise SystemExit("card missing token")
if fixture.get("kind") != "MirBuilderCrateWideUnconvertedSurfaceReportV1":
    raise SystemExit("fixture kind mismatch")
if fixture.get("token") != "MIRBUILDER-CRATE-WIDE-UNCONVERTED-SURFACE-REPORT-001":
    raise SystemExit("source report token drift")

provenance = fixture.get("provenance") or {}
if provenance.get("source_selfhost_family_guard_manifest_hash") != sha256(manifest_path):
    raise SystemExit("report manifest hash is stale")
if provenance.get("native_owner_seed_capability_survey_hash") != sha256(survey_path):
    raise SystemExit("report native seed survey hash is stale")
if provenance.get("tool_version") != "regex_source_text_v0":
    raise SystemExit("tool version drift")

summary = fixture.get("summary") or {}
if summary.get("scanned_surface_count") != 1584:
    raise SystemExit("scanned surface count drift")
if summary.get("classified_once_count") != summary.get("scanned_surface_count"):
    raise SystemExit("classified count must match scanned count")
if summary.get("missing_projection_policy_count") != 1384:
    raise SystemExit("missing projection count drift")
if summary.get("unmapped_count") != 0:
    raise SystemExit("unmapped count drift")

decision = fixture.get("decision") or {}
if decision.get("kind") != "KeepStopped":
    raise SystemExit("decision kind drift")
if decision.get("reason_token") != "AmbiguousUnconvertedSurfaceCandidates":
    raise SystemExit("decision reason drift")
if decision.get("selected_next_card") != "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001":
    raise SystemExit("decision next card drift")
PY

cat <<'REPORT'
output_contract=rust-lifecycle-mirbuilder-unconverted-surface-report-rerun
source_report_fresh=1
scanned_surface_count=1584
missing_projection_policy_count=1384
selected_next_card=SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001
manual_family_selection=0
source_selfhost_claim=0
runtime_fallback=0
summary=ok
REPORT
