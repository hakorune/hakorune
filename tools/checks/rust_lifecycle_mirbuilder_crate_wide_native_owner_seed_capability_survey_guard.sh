#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="rust-lifecycle-mirbuilder-crate-wide-native-owner-seed-capability-survey-guard"
source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

TOOL="$ROOT_DIR/tools/rust_lifecycle/mirbuilder_crate_wide_native_owner_seed_capability_survey.py"
FIXTURE="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-crate-wide-native-owner-seed-capability-survey-v0.json"
CARD="$ROOT_DIR/docs/development/current/main/phases/phase-296x/1825-MIRBUILDER-CRATE-WIDE-NATIVE-OWNER-SEED-CAPABILITY-SURVEY-001.md"

guard_require_command "$TAG" python3
guard_require_files "$TAG" "$TOOL" "$FIXTURE" "$CARD"

python3 "$TOOL" --check

python3 - <<'PY'
import json
from pathlib import Path

fixture = json.loads(Path("docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-crate-wide-native-owner-seed-capability-survey-v0.json").read_text())
card = Path("docs/development/current/main/phases/phase-296x/1825-MIRBUILDER-CRATE-WIDE-NATIVE-OWNER-SEED-CAPABILITY-SURVEY-001.md").read_text()

token = "MIRBUILDER-CRATE-WIDE-NATIVE-OWNER-SEED-CAPABILITY-SURVEY-001"
if fixture.get("kind") != "MirBuilderCrateWideNativeOwnerSeedCapabilitySurveyV1":
    raise SystemExit("fixture kind mismatch")
if fixture.get("token") != token:
    raise SystemExit("fixture token mismatch")
if token not in card:
    raise SystemExit("card missing token")

scope = fixture.get("input_scope") or {}
if scope.get("survey_unit") != "semantic_owner_edge":
    raise SystemExit("survey unit must be semantic_owner_edge")
if "src/mir/builder" not in scope.get("source_roots", []):
    raise SystemExit("survey source root drift")

items = fixture.get("scanned_items") or []
if not items:
    raise SystemExit("survey scanned items missing")
seen = set()
for item in items:
    owner = item.get("owner_edge_id")
    if not owner:
        raise SystemExit("survey item missing owner_edge_id")
    if owner in seen:
        raise SystemExit(f"duplicate owner_edge_id: {owner}")
    seen.add(owner)
    if not item.get("classification"):
        raise SystemExit(f"missing classification: {owner}")
    if not item.get("evidence_refs"):
        raise SystemExit(f"missing evidence refs: {owner}")
    if item.get("eligible_for_native_owner_seed") is not None:
        raise SystemExit("legacy eligible flag should not replace classification")

summary = fixture.get("summary") or {}
if summary.get("scanned_item_count") != len(items):
    raise SystemExit("summary scanned item count drift")

decision = fixture.get("decision") or {}
if decision.get("kind") != "KeepStopped":
    raise SystemExit("current survey should keep Source Selfhost stopped")
if decision.get("selected_next_card") != "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001":
    raise SystemExit("selected next card drift")

claims = fixture.get("claims") or {}
for key in [
    "survey_scope_explicit",
    "survey_unit_semantic_owner_edge",
    "selected_source_surfaces_partitioned_exactly_once",
    "each_item_has_stable_classification",
    "each_non_convertible_item_has_blocker_token",
    "each_item_has_evidence_refs",
]:
    if claims.get(key) != 1:
        raise SystemExit(f"positive claim must be 1: {key}")
for key in [
    "support_lane_projector_as_hako_adoption_candidate",
    "generated_artifact_as_edit_authority",
    "composition_owner_as_semantic_owner",
    "manual_family_selection",
    "route_membership_alone_as_proof",
    "coverage_percentage_as_proof",
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
output_contract=rust-lifecycle-mirbuilder-crate-wide-native-owner-seed-capability-survey-v0
survey_unit=semantic_owner_edge
decision=KeepStopped
manual_family_selection=0
source_selfhost_claim=0
runtime_fallback=0
summary=ok
REPORT
