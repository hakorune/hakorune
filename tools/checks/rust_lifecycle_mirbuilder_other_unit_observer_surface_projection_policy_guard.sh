#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="rust-lifecycle-mirbuilder-other-unit-observer-surface-projection-policy-guard"
source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

TOOL="$ROOT_DIR/tools/rust_lifecycle/mirbuilder_other_unit_observer_surface_projection_policy.py"
FIXTURE="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-other-unit-observer-surface-projection-policy-v0.json"
CARD="$ROOT_DIR/docs/development/current/main/phases/phase-296x/1946-MIRBUILDER-OTHER-UNIT-OBSERVER-SURFACE-PROJECTION-POLICY-001.md"

guard_require_command "$TAG" python3
guard_require_files "$TAG" "$TOOL" "$FIXTURE" "$CARD"

python3 "$TOOL" --check

python3 - <<'PY'
import json
from pathlib import Path

fixture = json.loads(Path("docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-other-unit-observer-surface-projection-policy-v0.json").read_text())
card = Path("docs/development/current/main/phases/phase-296x/1946-MIRBUILDER-OTHER-UNIT-OBSERVER-SURFACE-PROJECTION-POLICY-001.md").read_text()

token = "MIRBUILDER-OTHER-UNIT-OBSERVER-SURFACE-PROJECTION-POLICY-001"
if fixture.get("kind") != "MirBuilderOtherUnitObserverSurfaceProjectionPolicyV1":
    raise SystemExit("fixture kind mismatch")
if fixture.get("token") != token:
    raise SystemExit("fixture token mismatch")
if token not in card:
    raise SystemExit("card missing token")

state = fixture["input_state"]
if state["selected_shape_signature"] != "shape.other_unit_observer_surface":
    raise SystemExit("selected shape drift")
if state["source_count"] != 26:
    raise SystemExit("source count drift")
if state["current_blocker"] != "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001":
    raise SystemExit("current blocker drift")

axes = fixture["selection_axes"]
expected_axes = {
    "shape_signature": "shape.other_unit_observer_surface",
    "return_family": "unit",
    "borrow_axis": "NoBorrow",
    "type_transport_axis": "Known",
    "verifier_or_oracle_state": "Present",
    "owner_edge_confidence": "FileScoped",
}
if axes != expected_axes:
    raise SystemExit(f"selection axis drift: {axes}")

descriptor = fixture["unit_observer_descriptor"]
if descriptor["descriptor_id"] != "other_unit_observer_surface_v1":
    raise SystemExit("descriptor id drift")
if descriptor["return_contract"] != "unit":
    raise SystemExit("return contract drift")
if descriptor["mutation_frame"] != []:
    raise SystemExit("unit observer descriptor must not claim mutation frame")
for key in ["returned_borrow", "receiver_borrow"]:
    if descriptor[key] != 0:
        raise SystemExit(f"{key} must be 0")

surfaces = fixture["source_surfaces"]
if len(surfaces) != 26:
    raise SystemExit("source surface count drift")
if any(surface["return_type"] != "" for surface in surfaces):
    raise SystemExit("all surfaces must be unit-return")
if any(surface["receiver"] not in {"None", None, ""} for surface in surfaces):
    raise SystemExit("all surfaces must be receiver-free")
if any(surface["owner_edge_confidence"] != "FileScoped" for surface in surfaces):
    raise SystemExit("all surfaces must be FileScoped")

decision = fixture["decision"]
if decision["kind"] != "SelectProjectionPolicyDescriptor":
    raise SystemExit("decision kind drift")
if decision["reason_token"] != "OtherUnitObserverSurfaceDescriptorMaterialized":
    raise SystemExit("decision reason drift")
if decision["selected_next_card"] != "MIRBUILDER-PROJECTION-POLICY-CLUSTER-PRIORITY-RESOLUTION-001":
    raise SystemExit("selected next card drift")

claims = fixture["claims"]
for key in [
    "shape_signature_cluster_resolution_consumed",
    "shape_signature_inventory_consumed",
    "unconverted_surface_report_consumed",
    "descriptor_selected",
]:
    if claims.get(key) != 1:
        raise SystemExit(f"required claim must be 1: {key}")
if claims.get("source_count") != 26:
    raise SystemExit("claim source count drift")
for key in [
    "hako_projection_selected",
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
output_contract=rust-lifecycle-mirbuilder-other-unit-observer-surface-projection-policy-v0
selected_shape_signature=shape.other_unit_observer_surface
source_count=26
descriptor_selected=1
hako_projection_selected=0
selected_next_card=MIRBUILDER-PROJECTION-POLICY-CLUSTER-PRIORITY-RESOLUTION-001
manual_family_selection=0
source_selfhost_claim=0
runtime_fallback=0
new_backend_route=0
new_abi=0
summary=ok
REPORT
