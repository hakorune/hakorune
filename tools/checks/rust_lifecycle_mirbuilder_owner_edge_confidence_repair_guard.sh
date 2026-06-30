#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="rust-lifecycle-mirbuilder-owner-edge-confidence-repair-guard"
source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

TOOL="$ROOT_DIR/tools/rust_lifecycle/mirbuilder_owner_edge_confidence_repair.py"
FIXTURE="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-owner-edge-confidence-repair-v0.json"
REPORT="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-crate-wide-unconverted-surface-report-v0.json"
CARD="$ROOT_DIR/docs/development/current/main/phases/phase-296x/1876-MIRBUILDER-OWNER-EDGE-CONFIDENCE-REPAIR-001.md"

guard_require_command "$TAG" python3
guard_require_files "$TAG" "$TOOL" "$FIXTURE" "$REPORT" "$CARD"

python3 "$TOOL" --check

python3 - <<'PY'
import json
from pathlib import Path

fixture = json.loads(Path("docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-owner-edge-confidence-repair-v0.json").read_text())
report = json.loads(Path("docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-crate-wide-unconverted-surface-report-v0.json").read_text())
card = Path("docs/development/current/main/phases/phase-296x/1876-MIRBUILDER-OWNER-EDGE-CONFIDENCE-REPAIR-001.md").read_text()

token = "MIRBUILDER-OWNER-EDGE-CONFIDENCE-REPAIR-001"
if fixture.get("kind") != "MirBuilderOwnerEdgeConfidenceRepairV1":
    raise SystemExit("fixture kind mismatch")
if fixture.get("token") != token:
    raise SystemExit("fixture token mismatch")
if token not in card:
    raise SystemExit("card missing token")

summary = fixture["summary"]
if summary["input_candidate_count"] != 1396:
    raise SystemExit("input candidate count drift")
if summary["mapped_cluster_count"] != 8:
    raise SystemExit("mapped cluster count drift")
if summary["mapped_candidate_count"] != 1211:
    raise SystemExit("mapped candidate count drift")
if summary["denied_cluster_count"] != 1:
    raise SystemExit("denied cluster count drift")
if summary["denied_candidate_count"] != 185:
    raise SystemExit("denied candidate count drift")

for item in fixture["cluster_mappings"]:
    if item["owner_edge_confidence"] != "FixtureMapped":
        raise SystemExit("mapped cluster did not assign FixtureMapped confidence")
    if item["selected"] is not True:
        raise SystemExit("mapped cluster must be selected")
    if item["likely_owner_cluster"].startswith("Other"):
        raise SystemExit("Other cluster must not be mapped")
for item in fixture["denied_clusters"]:
    if item["owner_edge_confidence"] != "None":
        raise SystemExit("denied cluster confidence drift")
    if item["selected"] is not False:
        raise SystemExit("denied cluster selected drift")

missing = [item for item in report["items"] if item["classification"] == "MissingProjectionPolicy"]
fixture_mapped = [item for item in missing if item["owner_edge_confidence"] == "FixtureMapped"]
none = [item for item in missing if item["owner_edge_confidence"] == "None"]
if len(fixture_mapped) != 1211:
    raise SystemExit("report FixtureMapped count drift")
if len(none) != 185:
    raise SystemExit("report None confidence count drift")
for item in fixture_mapped:
    if not item.get("known_owner_edge"):
        raise SystemExit(f"FixtureMapped item lacks known owner edge: {item['source_id']}")
    if "docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-owner-edge-confidence-repair-v0.json" not in item.get("evidence_refs", []):
        raise SystemExit(f"FixtureMapped item lacks repair evidence ref: {item['source_id']}")
for item in none:
    if item.get("likely_owner_cluster") != "OtherMissingProjectionPolicyCluster":
        raise SystemExit(f"non-Other cluster left unmapped: {item['source_id']}")

decision = fixture["decision"]
if decision["kind"] != "ApplyOwnerEdgeConfidenceRepair":
    raise SystemExit("decision kind drift")
if decision["selected_next_card"] != "MIRBUILDER-MISSING-PROJECTION-STABLE-DENY-REASON-REPAIR-001":
    raise SystemExit("selected next card drift")

claims = fixture["claims"]
for key in [
    "owner_edge_confidence_repair_defined",
    "other_cluster_not_selectable",
]:
    if claims.get(key) != 1:
        raise SystemExit(f"positive claim must be 1: {key}")
for key in [
    "manual_family_selection",
    "cluster_size_as_proof",
    "coverage_percentage_as_proof",
    "route_membership_alone_as_proof",
    "generated_artifact_as_edit_authority",
    "source_selfhost_claim",
    "runtime_fallback",
    "new_backend_route",
    "new_abi",
    "new_python_semantic_projector",
    "runner_semantic_owner",
    "family_name_based_policy",
    "hako_emission",
    "hako_adopted_decision",
    "native_source_seed_materialization",
]:
    if claims.get(key) != 0:
        raise SystemExit(f"non-claim must be 0: {key}")
PY

cat <<'REPORT'
output_contract=rust-lifecycle-mirbuilder-owner-edge-confidence-repair-v0
mapped_candidate_count=1211
denied_candidate_count=185
decision=ApplyOwnerEdgeConfidenceRepair
next_card=MIRBUILDER-MISSING-PROJECTION-STABLE-DENY-REASON-REPAIR-001
manual_family_selection=0
source_selfhost_claim=0
runtime_fallback=0
new_backend_route=0
new_abi=0
summary=ok
REPORT
