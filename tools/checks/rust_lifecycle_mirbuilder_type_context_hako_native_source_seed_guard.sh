#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="rust-lifecycle-mirbuilder-type-context-hako-native-source-seed-guard"
source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

FIXTURE="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-type-context-hako-native-source-seed-v0.json"
CARD="$ROOT_DIR/docs/development/current/main/phases/phase-296x/1971-MIRBUILDER-TYPE-CONTEXT-HAKO-NATIVE-SOURCE-SEED-001.md"
NATIVE_SEED="$ROOT_DIR/lang/src/compiler/lib/type_context_native_seed.hako"
MODULE="$ROOT_DIR/lang/src/compiler/hako_module.toml"
RERUN="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-crate-wide-native-owner-seed-capability-survey-rerun-005-v0.json"
SELECTION="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-strict-converter-emission-native-seed-candidate-selection-v0.json"
BRIDGE="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-strict-converter-emission-to-native-seed-bridge-policy-v0.json"

guard_require_command "$TAG" python3
guard_require_files "$TAG" "$FIXTURE" "$CARD" "$NATIVE_SEED" "$MODULE" "$RERUN" "$SELECTION" "$BRIDGE"

python3 - <<'PY'
import json
from pathlib import Path

TOKEN = "MIRBUILDER-TYPE-CONTEXT-HAKO-NATIVE-SOURCE-SEED-001"
OWNER = "hakorune_mir_builder::type_context"
FIXTURE = Path("docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-type-context-hako-native-source-seed-v0.json")

fixture = json.loads(FIXTURE.read_text())
if fixture.get("kind") != "MirBuilderTypeContextHakoNativeSourceSeedV1":
    raise SystemExit("fixture kind mismatch")
if fixture.get("token") != TOKEN:
    raise SystemExit("fixture token mismatch")

rerun = json.loads(Path("docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-crate-wide-native-owner-seed-capability-survey-rerun-005-v0.json").read_text())
if rerun["decision"]["selected_owner_edge_id"] != OWNER:
    raise SystemExit("rerun-005 selected owner mismatch")
if rerun["decision"]["selected_next_card"] != TOKEN:
    raise SystemExit("rerun-005 next card mismatch")

bridge = json.loads(Path("docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-strict-converter-emission-to-native-seed-bridge-policy-v0.json").read_text())
if bridge["policy"]["seed_draft_input_state_name"] != "DerivedArtifactSeedDraftInput":
    raise SystemExit("bridge policy seed draft state drift")
if bridge["policy"]["generated_artifact_as_native_edit_authority"] is not False:
    raise SystemExit("generated artifact must not be native authority")

selection = json.loads(Path("docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-strict-converter-emission-native-seed-candidate-selection-v0.json").read_text())
selected_candidates = [
    c for c in selection.get("candidates", [])
    if c.get("owner_edge_id") == OWNER and c.get("next_card") == TOKEN
]
if len(selected_candidates) != 5:
    raise SystemExit(f"expected 5 bridge-eligible type_context surfaces, got {len(selected_candidates)}")
for c in selected_candidates:
    for key, expected in [
        ("bridge_state", "BridgeEligible"),
        ("deterministic_regeneration", True),
        ("provenance_manifest_present", True),
        ("verifier_or_oracle_or_guard_present", True),
        ("borrow_policy_gap", False),
        ("carrier_type_transport_gap", False),
        ("composite_owner", False),
        ("already_hako_adopted", False),
    ]:
        if c.get(key) != expected:
            raise SystemExit(f"candidate field mismatch for {c.get('verifier_result_fixture')}: {key}")

scope = fixture["surface_scope"]
if scope.get("collation_rule") != "FamilySeedSurfaceCollationV1":
    raise SystemExit("collation rule mismatch")
if scope.get("selected_surface_count") != 5:
    raise SystemExit("selected surface count mismatch")
surfaces = {s["surface_id"]: s for s in scope["surfaces"]}
expected_surfaces = {
    "type_context.origin_map": "OwnerMapProjectionSurface",
    "type_context.snapshot_restore": "OwnedMapSnapshotRestoreSurface",
    "type_context.string_literal": "OwnerMapProjectionSurface",
    "type_context.value_kind": "OwnerMapProjectionSurface",
    "type_context.value_type": "OwnerMapProjectionSurface",
}
if set(surfaces) != set(expected_surfaces):
    raise SystemExit("surface set mismatch")
for surface_id, role in expected_surfaces.items():
    surface = surfaces[surface_id]
    if surface.get("role") != role:
        raise SystemExit(f"surface role mismatch: {surface_id}")
    if surface.get("bridge_state") != "BridgeEligible":
        raise SystemExit(f"surface bridge state mismatch: {surface_id}")
    for key in ["generated_artifact", "generated_artifact_manifest", "verifier_result", "source_plan", "source_recipe"]:
        path = Path(surface[key])
        if not path.exists():
            raise SystemExit(f"surface evidence missing: {path}")
    verifier = json.loads(Path(surface["verifier_result"]).read_text())
    if verifier.get("family_id") != OWNER:
        raise SystemExit(f"verifier family mismatch: {surface_id}")
    if verifier.get("result") != "VerifiedHakoFamilyIR":
        raise SystemExit(f"verifier result mismatch: {surface_id}")

snapshot = surfaces["type_context.snapshot_restore"]["snapshot_notes"]
if snapshot.get("field_transport") != "OpaqueOwnedMapStorage":
    raise SystemExit("snapshot field transport mismatch")
if snapshot.get("container_cloned") is not False:
    raise SystemExit("snapshot must not claim cloned containers")
if snapshot.get("move_reset_contract") is not True:
    raise SystemExit("snapshot move/reset contract missing")

native_path = Path(fixture["native_seed"]["native_source_seed_path"])
if "lang/generated/" in native_path.as_posix():
    raise SystemExit("native seed must not live under generated tree")
native = native_path.read_text()
for needle in [
    "native-source-seed: MIRBUILDER-TYPE-CONTEXT-HAKO-NATIVE-SOURCE-SEED-001",
    "source-family: hakorune_mir_builder::type_context",
    "source-input-state: DerivedArtifactSeedDraftInput",
    "surface-collation: FamilySeedSurfaceCollationV1",
    "source-selfhost-claim: 0",
    "enum MirType",
    "enum MirValueKind",
    "box TypeContext",
    "box TypeContextSnapshot",
    "static box TypeContextApi",
    "get_origin_box(ctx, value_id): Option<StringBox>",
    "set_origin_box(ctx, value_id, class_name): i64",
    "clear_origin_boxes(ctx): i64",
    "string_literal(ctx, value_id): Option<StringBox>",
    "try_get_kind(ctx, value_id): Option<MirValueKind>",
    "get_kind(ctx, value_id): MirValueKind",
    "set_kind(ctx, value_id, kind): i64",
    "get_type(ctx, value_id): Option<MirType>",
    "set_type(ctx, value_id, ty): i64",
    "take_snapshot(ctx): TypeContextSnapshot",
    "restore_snapshot(ctx, snapshot: TypeContextSnapshot): i64",
]:
    if needle not in native:
        raise SystemExit(f"native seed missing expected text: {needle}")
if "@generated" in native or "manual-edit: forbidden" in native:
    raise SystemExit("native seed must not carry generated manual-edit markers")
if "static box Main" in native:
    raise SystemExit("native seed must not include generated smoke Main")

module = Path("lang/src/compiler/hako_module.toml").read_text()
if 'lib.type_context_native_seed = "lib/type_context_native_seed.hako"' not in module:
    raise SystemExit("module export missing type_context_native_seed")

claims = fixture["claims"]
for key in [
    "surface_selection_by_hand",
    "manual_family_selection",
    "generated_artifact_as_native_edit_authority",
    "generated_artifact_as_edit_authority",
    "hako_generation",
    "hako_adopted_decision",
    "source_selfhost_claim",
    "runtime_fallback",
    "new_backend_route",
    "new_abi",
    "new_python_semantic_projector",
    "runner_semantic_owner",
    "rust_deletion",
]:
    if claims.get(key) != 0:
        raise SystemExit(f"claim must be 0: {key}")
if claims.get("native_seed_materialization") != 1:
    raise SystemExit("native seed materialization claim must be 1")
if claims.get("family_seed_surface_collation") != 1:
    raise SystemExit("family seed collation claim must be 1")
if fixture["native_seed"]["generator_overwrite_guard"] is not True:
    raise SystemExit("generator overwrite guard must be true")
if fixture["decision"]["selected_next_card"] != "MIRBUILDER-TYPE-CONTEXT-HAKO-ADOPTION-DECISION-001":
    raise SystemExit("next card mismatch")
PY

cat <<'REPORT'
output_contract=rust-lifecycle-mirbuilder-type-context-hako-native-source-seed-v0
family_id=hakorune_mir_builder::type_context
surface_collation=FamilySeedSurfaceCollationV1
selected_surface_count=5
native_source_seed=lang/src/compiler/lib/type_context_native_seed.hako
native_source_owner_seed_present=1
generator_overwrite_guard=1
hako_adopted_decision=0
generated_artifact_as_edit_authority=0
source_selfhost_claim=0
runtime_fallback=0
new_backend_route=0
new_abi=0
new_python_semantic_projector=0
summary=ok
REPORT
