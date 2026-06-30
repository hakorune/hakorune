#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="rust-lifecycle-mirbuilder-missing-projection-stable-deny-reason-repair-guard"
source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

TOOL="$ROOT_DIR/tools/rust_lifecycle/mirbuilder_missing_projection_stable_deny_reason_repair.py"
FIXTURE="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-missing-projection-stable-deny-reason-repair-v0.json"
REPORT="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-crate-wide-unconverted-surface-report-v0.json"
CARD="$ROOT_DIR/docs/development/current/main/phases/phase-296x/1877-MIRBUILDER-MISSING-PROJECTION-STABLE-DENY-REASON-REPAIR-001.md"

guard_require_command "$TAG" python3
guard_require_files "$TAG" "$TOOL" "$FIXTURE" "$REPORT" "$CARD"

python3 "$TOOL" --check

python3 - <<'PY'
import json
from collections import Counter
from pathlib import Path

fixture = json.loads(Path("docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-missing-projection-stable-deny-reason-repair-v0.json").read_text())
report = json.loads(Path("docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-crate-wide-unconverted-surface-report-v0.json").read_text())
card = Path("docs/development/current/main/phases/phase-296x/1877-MIRBUILDER-MISSING-PROJECTION-STABLE-DENY-REASON-REPAIR-001.md").read_text()

token = "MIRBUILDER-MISSING-PROJECTION-STABLE-DENY-REASON-REPAIR-001"
if fixture.get("kind") != "MirBuilderMissingProjectionStableDenyReasonRepairV1":
    raise SystemExit("fixture kind mismatch")
if fixture.get("token") != token:
    raise SystemExit("fixture token mismatch")
if token not in card:
    raise SystemExit("card missing token")

summary = fixture["summary"]
expected_reasons = {
    "OwnerEdgeConfidenceMissing": 185,
    "UnsupportedDirectShape": 1211,
}
if summary["input_candidate_count"] != 1396:
    raise SystemExit("input candidate count drift")
if summary["stable_deny_reason_counts_after_repair"] != expected_reasons:
    raise SystemExit("stable deny reason counts drift")
if summary["selectable_stable_deny_reason_count"] != 1211:
    raise SystemExit("selectable stable deny reason count drift")
if summary["unselectable_stable_deny_reason_count"] != 185:
    raise SystemExit("unselectable stable deny reason count drift")

missing = [item for item in report["items"] if item["classification"] == "MissingProjectionPolicy"]
reasons = Counter(item.get("stable_deny_reason") for item in missing)
if dict(sorted(reasons.items())) != expected_reasons:
    raise SystemExit(f"report stable deny reason drift: {dict(reasons)}")
for item in missing:
    if item["owner_edge_confidence"] == "FixtureMapped" and item.get("stable_deny_reason") != "UnsupportedDirectShape":
        raise SystemExit(f"FixtureMapped item has wrong stable deny reason: {item['source_id']}")
    if item["owner_edge_confidence"] == "None" and item.get("stable_deny_reason") != "OwnerEdgeConfidenceMissing":
        raise SystemExit(f"None-confidence item has wrong stable deny reason: {item['source_id']}")
    if "docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-missing-projection-stable-deny-reason-repair-v0.json" not in item.get("evidence_refs", []):
        raise SystemExit(f"missing stable deny evidence ref: {item['source_id']}")

decision = fixture["decision"]
if decision["kind"] != "ApplyStableDenyReasonRepair":
    raise SystemExit("decision kind drift")
if decision["selected_next_card"] != "MIRBUILDER-CRATE-WIDE-SHAPE-SIGNATURE-INVENTORY-001":
    raise SystemExit("selected next card drift")

claims = fixture["claims"]
if claims.get("stable_deny_reason_repair_defined") != 1:
    raise SystemExit("stable deny reason repair claim missing")
if claims.get("unsupported_direct_shape_count_after_repair") != 1211:
    raise SystemExit("unsupported direct shape claim drift")
if claims.get("owner_edge_confidence_missing_count_after_repair") != 185:
    raise SystemExit("owner edge confidence missing claim drift")
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
output_contract=rust-lifecycle-mirbuilder-missing-projection-stable-deny-reason-repair-v0
unsupported_direct_shape_count=1211
owner_edge_confidence_missing_count=185
decision=ApplyStableDenyReasonRepair
next_card=MIRBUILDER-CRATE-WIDE-SHAPE-SIGNATURE-INVENTORY-001
manual_family_selection=0
source_selfhost_claim=0
runtime_fallback=0
new_backend_route=0
new_abi=0
summary=ok
REPORT
