#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="rust-lifecycle-mirbuilder-crate-wide-shape-signature-inventory-guard"
source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

TOOL="$ROOT_DIR/tools/rust_lifecycle/mirbuilder_crate_wide_shape_signature_inventory.py"
FIXTURE="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-crate-wide-shape-signature-inventory-v0.json"
REPORT="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-crate-wide-unconverted-surface-report-v0.json"
CARD="$ROOT_DIR/docs/development/current/main/phases/phase-296x/1878-MIRBUILDER-CRATE-WIDE-SHAPE-SIGNATURE-INVENTORY-001.md"

guard_require_command "$TAG" python3
guard_require_files "$TAG" "$TOOL" "$FIXTURE" "$REPORT" "$CARD"

python3 "$TOOL" --check

python3 - <<'PY'
import json
from collections import Counter
from pathlib import Path

fixture = json.loads(Path("docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-crate-wide-shape-signature-inventory-v0.json").read_text())
report = json.loads(Path("docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-crate-wide-unconverted-surface-report-v0.json").read_text())
card = Path("docs/development/current/main/phases/phase-296x/1878-MIRBUILDER-CRATE-WIDE-SHAPE-SIGNATURE-INVENTORY-001.md").read_text()

token = "MIRBUILDER-CRATE-WIDE-SHAPE-SIGNATURE-INVENTORY-001"
if fixture.get("kind") != "MirBuilderCrateWideShapeSignatureInventoryV1":
    raise SystemExit("fixture kind mismatch")
if fixture.get("token") != token:
    raise SystemExit("fixture token mismatch")
if token not in card:
    raise SystemExit("card missing token")

summary = fixture["summary"]
if summary["input_candidate_count"] != 1396:
    raise SystemExit("input candidate count drift")
if summary["shape_signature_count"] != 54:
    raise SystemExit("shape signature count drift")
if summary["shape_signature_candidate_count"] != 1211:
    raise SystemExit("shape signature candidate count drift")
if summary["unknown_shape_candidate_count_after_inventory"] != 185:
    raise SystemExit("unknown shape count drift")

missing = [item for item in report["items"] if item["classification"] == "MissingProjectionPolicy"]
with_shape = [item for item in missing if item.get("shape_signature")]
without_shape = [item for item in missing if not item.get("shape_signature")]
if len(with_shape) != 1211:
    raise SystemExit("report shape-signature count drift")
if len(without_shape) != 185:
    raise SystemExit("report unknown-shape count drift")
for item in with_shape:
    if item["owner_edge_confidence"] != "FixtureMapped":
        raise SystemExit(f"shape assigned to non-FixtureMapped item: {item['source_id']}")
    if "docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-crate-wide-shape-signature-inventory-v0.json" not in item.get("evidence_refs", []):
        raise SystemExit(f"shape item lacks evidence ref: {item['source_id']}")
for item in without_shape:
    if item["owner_edge_confidence"] != "None":
        raise SystemExit(f"FixtureMapped item lacks shape signature: {item['source_id']}")

decision = fixture["decision"]
if decision["kind"] != "ApplyShapeSignatureInventory":
    raise SystemExit("decision kind drift")
if decision["selected_next_card"] != "MIRBUILDER-PROJECTION-POLICY-CLUSTER-PRIORITY-RESOLUTION-001":
    raise SystemExit("selected next card drift")

claims = fixture["claims"]
if claims.get("shape_signature_inventory_defined") != 1:
    raise SystemExit("shape signature inventory claim missing")
if claims.get("shape_signature_candidate_count_after_inventory") != 1211:
    raise SystemExit("shape signature candidate claim drift")
if claims.get("unknown_shape_candidate_count_after_inventory") != 185:
    raise SystemExit("unknown shape candidate claim drift")
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
output_contract=rust-lifecycle-mirbuilder-crate-wide-shape-signature-inventory-v0
shape_signature_count=54
shape_signature_candidate_count=1211
unknown_shape_candidate_count=185
decision=ApplyShapeSignatureInventory
next_card=MIRBUILDER-PROJECTION-POLICY-CLUSTER-PRIORITY-RESOLUTION-001
manual_family_selection=0
source_selfhost_claim=0
runtime_fallback=0
new_backend_route=0
new_abi=0
summary=ok
REPORT
