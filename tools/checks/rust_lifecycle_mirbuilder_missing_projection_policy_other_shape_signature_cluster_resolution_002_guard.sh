#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="rust-lifecycle-mirbuilder-missing-projection-policy-other-shape-signature-cluster-resolution-002-guard"
source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

TOOL="$ROOT_DIR/tools/rust_lifecycle/mirbuilder_missing_projection_policy_other_shape_signature_cluster_resolution_002.py"
FIXTURE="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-missing-projection-policy-other-shape-signature-cluster-resolution-002-v0.json"
CARD="$ROOT_DIR/docs/development/current/main/phases/phase-296x/1947-MIRBUILDER-MISSING-PROJECTION-POLICY-OTHER-SHAPE-SIGNATURE-CLUSTER-RESOLUTION-002.md"

guard_require_command "$TAG" python3
guard_require_files "$TAG" "$TOOL" "$FIXTURE" "$CARD"

python3 "$TOOL" --check

python3 - <<'PY'
import json
from pathlib import Path

fixture = json.loads(Path("docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-missing-projection-policy-other-shape-signature-cluster-resolution-002-v0.json").read_text())
card = Path("docs/development/current/main/phases/phase-296x/1947-MIRBUILDER-MISSING-PROJECTION-POLICY-OTHER-SHAPE-SIGNATURE-CLUSTER-RESOLUTION-002.md").read_text()

token = "MIRBUILDER-MISSING-PROJECTION-POLICY-OTHER-SHAPE-SIGNATURE-CLUSTER-RESOLUTION-002"
if fixture.get("kind") != "MirBuilderMissingProjectionPolicyOtherShapeSignatureClusterResolutionV2":
    raise SystemExit("fixture kind mismatch")
if fixture.get("token") != token:
    raise SystemExit("fixture token mismatch")
if token not in card:
    raise SystemExit("card missing token")

input_state = fixture["input_state"]
if input_state["completed_other_shape_signatures"] != ["shape.other_unit_observer_surface"]:
    raise SystemExit("completed shape signature drift")
if input_state["current_blocker"] != "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001":
    raise SystemExit("current blocker drift")

summary = fixture["summary"]
if summary["input_shape_signature_count"] != 11:
    raise SystemExit("shape count drift")
if summary["input_other_owner_cluster_count"] != 185:
    raise SystemExit("input row count drift")
if summary["completed_shape_signature_count"] != 1:
    raise SystemExit("completed shape count drift")
if summary["selection_eligible_shape_count"] != 0:
    raise SystemExit("eligible shape count drift")
if summary["selected_shape_signature"] is not None:
    raise SystemExit("selected shape must be null")

clusters = fixture["clusters"]
completed = [cluster for cluster in clusters if cluster["shape_signature"] == "shape.other_unit_observer_surface"]
if len(completed) != 1:
    raise SystemExit("completed cluster missing")
if completed[0]["selection_eligible"] is not False:
    raise SystemExit("completed cluster must not be eligible")
if completed[0]["blocked_by"] != ["ProjectionPolicyDescriptorAlreadyLanded"]:
    raise SystemExit("completed cluster blocked_by drift")
if any(cluster["selection_eligible"] for cluster in clusters):
    raise SystemExit("no unclosed Other shape should be eligible")

decision = fixture["decision"]
if decision["kind"] != "KeepStopped":
    raise SystemExit("decision kind drift")
if decision["reason_token"] != "NoUnclosedOtherShapeSignatureClusterEligible":
    raise SystemExit("decision reason drift")
if decision["selected_shape_signature"] is not None:
    raise SystemExit("decision selected shape drift")
if decision["selected_next_card"] != "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001":
    raise SystemExit("decision next card drift")

claims = fixture["claims"]
for key in [
    "other_shape_signature_inventory_consumed",
    "family_manifest_consumed",
    "completed_other_shape_descriptors_excluded",
    "shape_clusters_evaluated_by_evidence_quality",
]:
    if claims.get(key) != 1:
        raise SystemExit(f"required claim must be 1: {key}")
for key in [
    "cluster_size_as_proof",
    "manual_family_selection",
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
output_contract=rust-lifecycle-mirbuilder-missing-projection-policy-other-shape-signature-cluster-resolution-002-v0
completed_other_shape_signatures=shape.other_unit_observer_surface
selection_eligible_shape_count=0
selected_next_card=SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001
manual_family_selection=0
source_selfhost_claim=0
runtime_fallback=0
new_backend_route=0
new_abi=0
summary=ok
REPORT
