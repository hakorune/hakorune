#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="rust-lifecycle-mirbuilder-crate-wide-unconverted-surface-report-guard"
source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

TOOL="$ROOT_DIR/tools/rust_lifecycle/mirbuilder_crate_wide_unconverted_surface_report.py"
FIXTURE="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-crate-wide-unconverted-surface-report-v0.json"
CARD="$ROOT_DIR/docs/development/current/main/phases/phase-296x/1826-MIRBUILDER-CRATE-WIDE-UNCONVERTED-SURFACE-REPORT-001.md"

guard_require_command "$TAG" python3
guard_require_files "$TAG" "$TOOL" "$FIXTURE" "$CARD"

python3 "$TOOL" --check

python3 - <<'PY'
import json
from pathlib import Path

fixture = json.loads(Path("docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-crate-wide-unconverted-surface-report-v0.json").read_text())
card = Path("docs/development/current/main/phases/phase-296x/1826-MIRBUILDER-CRATE-WIDE-UNCONVERTED-SURFACE-REPORT-001.md").read_text()

token = "MIRBUILDER-CRATE-WIDE-UNCONVERTED-SURFACE-REPORT-001"
if fixture.get("kind") != "MirBuilderCrateWideUnconvertedSurfaceReportV1":
    raise SystemExit("fixture kind mismatch")
if fixture.get("token") != token:
    raise SystemExit("fixture token mismatch")
if token not in card:
    raise SystemExit("card missing token")

input_info = fixture.get("input") or {}
if input_info.get("scan_unit") != "rust_function_or_method":
    raise SystemExit("scan unit drift")
if input_info.get("join_unit") != "semantic_owner_edge":
    raise SystemExit("join unit drift")
if input_info.get("scan_method") != "regex_source_text_v0":
    raise SystemExit("scan method drift")

items = fixture.get("items") or []
if not items:
    raise SystemExit("items missing")
seen = set()
for item in items:
    source_id = item.get("source_id")
    if not source_id:
        raise SystemExit("item missing source_id")
    if source_id in seen:
        raise SystemExit(f"duplicate source_id: {source_id}")
    seen.add(source_id)
    if not item.get("classification"):
        raise SystemExit(f"item missing classification: {source_id}")
    if not item.get("reason_token"):
        raise SystemExit(f"item missing reason_token: {source_id}")

summary = fixture.get("summary") or {}
if summary.get("scanned_surface_count") != len(items):
    raise SystemExit("summary scanned surface count drift")
if summary.get("classified_once_count") != len(items):
    raise SystemExit("classified count drift")

decision = fixture.get("decision") or {}
if decision.get("kind") != "KeepStopped":
    raise SystemExit("current report should keep Source Selfhost stopped")
if decision.get("selected_next_card") != "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001":
    raise SystemExit("selected next card drift")

claims = fixture.get("claims") or {}
for key in [
    "tool_output_matches_checked_in_fixture",
    "scan_unit_rust_function_or_method",
    "join_unit_semantic_owner_edge",
    "scan_method_regex_source_text_v0",
    "every_scanned_public_method_classified_exactly_once",
    "every_unconverted_item_has_reason_token",
    "multiple_candidates_keep_stopped",
    "borrow_policy_known_does_not_select_owner",
    "composite_suspected_is_not_decomposition_proof",
    "generated_artifact_only_is_not_native_edit_authority",
    "support_lane_only_is_not_hako_adoption_candidate",
]:
    if claims.get(key) != 1:
        raise SystemExit(f"positive claim must be 1: {key}")
for key in [
    "rust_ast_parser_required",
    "rustc_adapter_required",
    "semantic_inference_beyond_existing_ssot",
    "manual_family_selection",
    "source_selfhost_claim",
    "runtime_fallback",
    "new_backend_route",
    "new_abi",
    "new_python_semantic_projector",
    "runner_semantic_owner",
]:
    if claims.get(key) != 0:
        raise SystemExit(f"non-claim must be 0: {key}")
PY

cat <<'REPORT'
output_contract=rust-lifecycle-mirbuilder-crate-wide-unconverted-surface-report-v0
scan_unit=rust_function_or_method
join_unit=semantic_owner_edge
decision=KeepStopped
manual_family_selection=0
source_selfhost_claim=0
runtime_fallback=0
summary=ok
REPORT
