#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="rust-lifecycle-mirbuilder-metadata-context-hako-adoption-decision-guard"
source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

FIXTURE="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-metadata-context-hako-adoption-decision-v0.json"
CARD="$ROOT_DIR/docs/development/current/main/phases/phase-296x/1969-MIRBUILDER-METADATA-CONTEXT-HAKO-ADOPTION-DECISION-001.md"
SEED_FIXTURE="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-metadata-context-hako-native-source-seed-v0.json"
SEED_GUARD="$ROOT_DIR/tools/checks/rust_lifecycle_mirbuilder_metadata_context_hako_native_source_seed_guard.sh"
SELECTION="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-strict-converter-emission-native-seed-candidate-selection-v0.json"
NATIVE_SOURCE="$ROOT_DIR/lang/src/compiler/lib/metadata_context_native_seed.hako"

guard_require_command "$TAG" python3
guard_require_files "$TAG" "$FIXTURE" "$CARD" "$SEED_FIXTURE" "$SEED_GUARD" "$SELECTION" "$NATIVE_SOURCE"

bash "$SEED_GUARD" >/tmp/metadata_context_native_seed_guard.out

python3 - <<'PY'
import json
from pathlib import Path

token = "MIRBUILDER-METADATA-CONTEXT-HAKO-ADOPTION-DECISION-001"
owner = "hakorune_mir_builder::metadata_context"

fixture = json.loads(Path("docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-metadata-context-hako-adoption-decision-v0.json").read_text())
seed = json.loads(Path("docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-metadata-context-hako-native-source-seed-v0.json").read_text())
selection = json.loads(Path("docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-strict-converter-emission-native-seed-candidate-selection-v0.json").read_text())
card = Path("docs/development/current/main/phases/phase-296x/1969-MIRBUILDER-METADATA-CONTEXT-HAKO-ADOPTION-DECISION-001.md").read_text()
native = Path("lang/src/compiler/lib/metadata_context_native_seed.hako").read_text()

if fixture.get("kind") != "MirBuilderMetadataContextHakoAdoptionDecisionV1":
    raise SystemExit("fixture kind mismatch")
if fixture.get("token") != token:
    raise SystemExit("fixture token mismatch")
if token not in card:
    raise SystemExit("card token mismatch")
if fixture.get("family_id") != owner:
    raise SystemExit("family id mismatch")

if seed["seed_status"]["native_source_owner_seed_present"] != 1:
    raise SystemExit("native source seed must be present")
if seed["native_seed"]["generator_overwrite_guard"] is not True:
    raise SystemExit("seed generator overwrite guard missing")
if seed["surface_scope"]["collation_rule"] != "FamilySeedSurfaceCollationV1":
    raise SystemExit("seed collation rule mismatch")
if seed["surface_scope"]["selected_surface_count"] != 3:
    raise SystemExit("seed surface count mismatch")

selected_candidates = [
    c for c in selection.get("candidates", [])
    if c.get("owner_edge_id") == owner
    and c.get("next_card") == "MIRBUILDER-METADATA-CONTEXT-HAKO-NATIVE-SOURCE-SEED-001"
]
if len(selected_candidates) != 3:
    raise SystemExit("strict candidate selection must include 3 metadata_context surfaces")
for c in selected_candidates:
    if c.get("bridge_state") != "BridgeEligible":
        raise SystemExit("all metadata_context candidates must be BridgeEligible")
    if c.get("borrow_policy_gap") is not False:
        raise SystemExit("borrow policy gap must be false")
    if c.get("carrier_type_transport_gap") is not False:
        raise SystemExit("carrier/type gap must be false")
    if c.get("already_hako_adopted") is not False:
        raise SystemExit("candidate must not have been adopted before this decision")

target = fixture["target"]
if target["family_scope"] != "BoundedFamilySeedSurfaceSet":
    raise SystemExit("target family scope mismatch")
if target["surface_collation_rule"] != "FamilySeedSurfaceCollationV1":
    raise SystemExit("target collation rule mismatch")
if target["selected_surface_count"] != 3:
    raise SystemExit("target surface count mismatch")
if target["native_source_owner_present"] != 1:
    raise SystemExit("native source owner must be present")
if target["strict_emission_bridge_candidate"] != 1:
    raise SystemExit("strict bridge candidate must be present")
if target["generated_artifact_as_edit_authority"] != 0:
    raise SystemExit("generated artifact must not be edit authority")
if target["region_parent_general_arraybox_policy"] != 0:
    raise SystemExit("region_parent must not become general ArrayBox policy")
if target["region_parent_returned_borrow_authority"] != 0:
    raise SystemExit("region_parent must not become returned borrow authority")

surfaces = {s["surface_id"]: s for s in fixture["selected_surfaces"]}
expected = {
    "metadata_context.scalar_source_file": "OwnerFieldSurface",
    "metadata_context.value_caller": "OwnerMapProjectionSurface",
    "metadata_context.region_parent": "OwnerScopedHelperSurface",
}
if set(surfaces) != set(expected):
    raise SystemExit("selected surface set mismatch")
for surface_id, role in expected.items():
    surface = surfaces[surface_id]
    if surface.get("role") != role:
        raise SystemExit(f"surface role mismatch: {surface_id}")
    verifier = json.loads(Path(surface["verifier_result"]).read_text())
    if verifier.get("family_id") != owner:
        raise SystemExit(f"verifier family mismatch: {surface_id}")
    if verifier.get("result") != "VerifiedHakoFamilyIR":
        raise SystemExit(f"verifier result mismatch: {surface_id}")
if surfaces["metadata_context.region_parent"].get("standalone_current_region_stack") != "Deny(ReturnedReadBorrow)":
    raise SystemExit("region_parent standalone stack policy mismatch")

decision = fixture["decision"]
if decision["value"] != "Adopt":
    raise SystemExit("decision must be Adopt")
if decision["selected_next_route"] != "native_hako_source_owner":
    raise SystemExit("selected next route mismatch")

for needle in [
    "hako-adopted: 1",
    "source-selfhost-claim: 0",
    "surface-collation: FamilySeedSurfaceCollationV1",
    "box MetadataContext",
    "static box MetadataContextApi",
]:
    if needle not in native:
        raise SystemExit(f"native source missing adoption marker: {needle}")
if "@generated" in native or "manual-edit: forbidden" in native:
    raise SystemExit("adopted native source must not be generated/manual-edit forbidden")
if "static box Main" in native:
    raise SystemExit("adopted native source must not include generated smoke Main")

claims = fixture["claims"]
for key in [
    "hako_adopted",
    "native_hako_source_owner_present",
    "generator_overwrite_guard",
    "family_seed_surface_collation",
    "rust_bootstrap_retained",
    "rust_oracle_retained",
]:
    if claims.get(key) != 1:
        raise SystemExit(f"positive claim must be 1: {key}")
for key in [
    "manual_family_selection",
    "surface_selection_by_hand",
    "source_selfhost_claim",
    "rust_deletion",
    "runtime_fallback",
    "new_backend_route",
    "new_abi",
    "new_python_semantic_projector",
    "runner_semantic_owner",
    "generated_artifact_as_edit_authority",
    "generated_artifact_as_native_edit_authority",
    "cluster_size_as_proof",
    "coverage_percentage_as_proof",
    "route_membership_alone_as_proof",
]:
    if claims.get(key) != 0:
        raise SystemExit(f"non-claim must be 0: {key}")
PY

cat <<'REPORT'
output_contract=rust-lifecycle-mirbuilder-metadata-context-hako-adoption-decision-v0
family_id=hakorune_mir_builder::metadata_context
decision=Adopt
surface_collation=FamilySeedSurfaceCollationV1
selected_surface_count=3
selected_next_route=native_hako_source_owner
native_hako_source_owner_present=1
generator_overwrite_guard=1
rust_bootstrap_retained=1
rust_oracle_retained=1
manual_family_selection=0
surface_selection_by_hand=0
source_selfhost_claim=0
rust_deletion=0
runtime_fallback=0
new_backend_route=0
new_abi=0
summary=ok
REPORT
