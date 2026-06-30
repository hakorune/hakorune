#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="rust-lifecycle-mirbuilder-missing-projection-policy-other-shape-signature-inventory-guard"
source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

TOOL="$ROOT_DIR/tools/rust_lifecycle/mirbuilder_missing_projection_policy_other_shape_signature_inventory.py"
FIXTURE="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-missing-projection-policy-other-shape-signature-inventory-v0.json"
CARD="$ROOT_DIR/docs/development/current/main/phases/phase-296x/1944-MIRBUILDER-MISSING-PROJECTION-POLICY-OTHER-SHAPE-SIGNATURE-INVENTORY-001.md"

guard_require_command "$TAG" python3
guard_require_files "$TAG" "$TOOL" "$FIXTURE" "$CARD"

python3 "$TOOL" --check

python3 - <<'PY'
import json
from pathlib import Path

fixture = json.loads(Path("docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-missing-projection-policy-other-shape-signature-inventory-v0.json").read_text())
card = Path("docs/development/current/main/phases/phase-296x/1944-MIRBUILDER-MISSING-PROJECTION-POLICY-OTHER-SHAPE-SIGNATURE-INVENTORY-001.md").read_text()

token = "MIRBUILDER-MISSING-PROJECTION-POLICY-OTHER-SHAPE-SIGNATURE-INVENTORY-001"
if fixture.get("kind") != "MirBuilderMissingProjectionPolicyOtherShapeSignatureInventoryV1":
    raise SystemExit("fixture kind mismatch")
if fixture.get("token") != token:
    raise SystemExit("fixture token mismatch")
if token not in card:
    raise SystemExit("card missing token")

summary = fixture["summary"]
if summary["input_other_owner_cluster_count"] != 185:
    raise SystemExit("input row count drift")
if summary["input_subcluster_count"] != 123:
    raise SystemExit("input subcluster count drift")
if summary["assigned_subcluster_count"] != 123:
    raise SystemExit("assigned subcluster count drift")
if summary["assigned_row_count"] != 185:
    raise SystemExit("assigned row count drift")
if summary["shape_signature_count"] != 11:
    raise SystemExit("shape signature count drift")
if summary["unknown_shape_count_after_inventory"] != 0:
    raise SystemExit("unknown shape count must be zero")
expected = {
    "shape.other_mutating_result_surface": 20,
    "shape.other_optional_read_surface": 25,
    "shape.other_unit_observer_surface": 17,
    "shape.other_custom_carrier_surface": 14,
}
for key, value in expected.items():
    if summary["shape_signature_counts"].get(key) != value:
        raise SystemExit(f"shape signature count drift: {key}")

if len(fixture["assignments"]) != 123:
    raise SystemExit("assignment list count drift")
if any(row["prior_shape_signature"] != "unknown_shape" for row in fixture["assignments"]):
    raise SystemExit("all prior shapes must be unknown_shape")
if any(row["selected_as_projection_policy"] for row in fixture["assignments"]):
    raise SystemExit("inventory must not select projection policy")

decision = fixture["decision"]
if decision["kind"] != "SelectOtherShapeSignatureClusterResolution":
    raise SystemExit("decision kind drift")
if decision["reason_token"] != "MultipleOtherShapeSignatureCandidates":
    raise SystemExit("decision reason drift")
if decision["selected_next_card"] != "MIRBUILDER-MISSING-PROJECTION-POLICY-OTHER-SHAPE-SIGNATURE-CLUSTER-RESOLUTION-001":
    raise SystemExit("selected next card drift")

claims = fixture["claims"]
for key in [
    "other_owner_cluster_rerun_consumed",
    "all_other_owner_subclusters_assigned_shape_candidate",
]:
    if claims.get(key) != 1:
        raise SystemExit(f"required claim must be 1: {key}")
if claims.get("input_other_owner_cluster_count") != 185:
    raise SystemExit("claim input count drift")
if claims.get("unknown_shape_count_after_inventory") != 0:
    raise SystemExit("claim unknown shape count drift")
for key in [
    "semantic_projection_inference",
    "family_name_based_policy",
    "manual_family_selection",
    "cluster_size_as_proof",
    "coverage_percentage_as_proof",
    "route_membership_alone_as_proof",
    "generated_artifact_as_edit_authority",
    "hako_generation",
    "hako_adopted_decision",
    "native_source_seed_materialization",
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
output_contract=rust-lifecycle-mirbuilder-missing-projection-policy-other-shape-signature-inventory-v0
input_other_owner_cluster_count=185
assigned_subcluster_count=123
shape_signature_count=11
unknown_shape_count_after_inventory=0
selected_next_card=MIRBUILDER-MISSING-PROJECTION-POLICY-OTHER-SHAPE-SIGNATURE-CLUSTER-RESOLUTION-001
manual_family_selection=0
source_selfhost_claim=0
runtime_fallback=0
new_backend_route=0
new_abi=0
summary=ok
REPORT
