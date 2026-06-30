#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="rust-lifecycle-mirbuilder-missing-projection-policy-other-shape-signature-cluster-resolution-guard"
source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

TOOL="$ROOT_DIR/tools/rust_lifecycle/mirbuilder_missing_projection_policy_other_shape_signature_cluster_resolution.py"
FIXTURE="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-missing-projection-policy-other-shape-signature-cluster-resolution-v0.json"
CARD="$ROOT_DIR/docs/development/current/main/phases/phase-296x/1945-MIRBUILDER-MISSING-PROJECTION-POLICY-OTHER-SHAPE-SIGNATURE-CLUSTER-RESOLUTION-001.md"

guard_require_command "$TAG" python3
guard_require_files "$TAG" "$TOOL" "$FIXTURE" "$CARD"

python3 "$TOOL" --check

python3 - <<'PY'
import json
from pathlib import Path

fixture = json.loads(Path("docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-missing-projection-policy-other-shape-signature-cluster-resolution-v0.json").read_text())
card = Path("docs/development/current/main/phases/phase-296x/1945-MIRBUILDER-MISSING-PROJECTION-POLICY-OTHER-SHAPE-SIGNATURE-CLUSTER-RESOLUTION-001.md").read_text()

token = "MIRBUILDER-MISSING-PROJECTION-POLICY-OTHER-SHAPE-SIGNATURE-CLUSTER-RESOLUTION-001"
if fixture.get("kind") != "MirBuilderMissingProjectionPolicyOtherShapeSignatureClusterResolutionV1":
    raise SystemExit("fixture kind mismatch")
if fixture.get("token") != token:
    raise SystemExit("fixture token mismatch")
if token not in card:
    raise SystemExit("card missing token")

summary = fixture["summary"]
if summary["input_shape_signature_count"] != 11:
    raise SystemExit("shape count drift")
if summary["input_other_owner_cluster_count"] != 185:
    raise SystemExit("input row count drift")
if summary["selection_eligible_shape_count"] != 1:
    raise SystemExit("eligible shape count drift")
if summary["selected_shape_signature"] != "shape.other_unit_observer_surface":
    raise SystemExit("selected shape drift")

eligible = [cluster for cluster in fixture["clusters"] if cluster["selection_eligible"]]
if len(eligible) != 1:
    raise SystemExit("exactly one eligible shape required")
selected = eligible[0]
if selected["shape_signature"] != "shape.other_unit_observer_surface":
    raise SystemExit("eligible shape drift")
if selected["candidate_count"] != 26:
    raise SystemExit("selected candidate count drift")
if selected["subcluster_count"] != 17:
    raise SystemExit("selected subcluster count drift")
if selected["blocked_by"] != []:
    raise SystemExit("selected shape must not be blocked")

decision = fixture["decision"]
if decision["kind"] != "SelectOtherShapeSignatureProjectionPolicy":
    raise SystemExit("decision kind drift")
if decision["reason_token"] != "ExactlyOneOtherShapeSignatureClusterEligible":
    raise SystemExit("decision reason drift")
if decision["selected_shape_signature"] != "shape.other_unit_observer_surface":
    raise SystemExit("decision selected shape drift")
if decision["selected_next_card"] != "MIRBUILDER-OTHER-UNIT-OBSERVER-SURFACE-PROJECTION-POLICY-001":
    raise SystemExit("selected next card drift")

claims = fixture["claims"]
for key in [
    "other_shape_signature_inventory_consumed",
    "shape_clusters_evaluated_by_evidence_quality",
]:
    if claims.get(key) != 1:
        raise SystemExit(f"required claim must be 1: {key}")
if claims.get("input_shape_signature_count") != 11:
    raise SystemExit("claim shape count drift")
if claims.get("input_other_owner_cluster_count") != 185:
    raise SystemExit("claim row count drift")
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
output_contract=rust-lifecycle-mirbuilder-missing-projection-policy-other-shape-signature-cluster-resolution-v0
input_shape_signature_count=11
selection_eligible_shape_count=1
selected_shape_signature=shape.other_unit_observer_surface
selected_next_card=MIRBUILDER-OTHER-UNIT-OBSERVER-SURFACE-PROJECTION-POLICY-001
manual_family_selection=0
source_selfhost_claim=0
runtime_fallback=0
new_backend_route=0
new_abi=0
summary=ok
REPORT
