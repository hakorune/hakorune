#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="rust-lifecycle-mirbuilder-typed-object-plan-refresh-hako-native-source-seed-guard"
source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

TOOL="$ROOT_DIR/tools/rust_lifecycle/mirbuilder_typed_object_plan_refresh_hako_native_source_seed.py"
FIXTURE="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-typed-object-plan-refresh-hako-native-source-seed-v0.json"
CARD="$ROOT_DIR/docs/development/current/main/phases/phase-296x/1998-MIRBUILDER-TYPED-OBJECT-PLAN-REFRESH-HAKO-NATIVE-SOURCE-SEED-001.md"
NATIVE_SEED="$ROOT_DIR/lang/src/compiler/lib/typed_object_plan_refresh_native_seed.hako"
MODULE="$ROOT_DIR/lang/src/compiler/hako_module.toml"
GENERATED="$ROOT_DIR/lang/generated/rust_derived/hakorune_mir_builder/mirbuilder_typed_object_plan_refresh.hako"
GENERATED_MANIFEST="$ROOT_DIR/lang/generated/rust_derived/hakorune_mir_builder/mirbuilder_typed_object_plan_refresh.artifact.json"
SELECTION="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-strict-converter-emission-native-seed-candidate-selection-rerun-004-v0.json"

guard_require_command "$TAG" python3
guard_require_files "$TAG" "$TOOL" "$FIXTURE" "$CARD" "$NATIVE_SEED" "$MODULE" "$GENERATED" "$GENERATED_MANIFEST" "$SELECTION"

python3 "$TOOL" --check

python3 - <<'PY'
import json
from pathlib import Path

fixture = json.loads(Path("docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-typed-object-plan-refresh-hako-native-source-seed-v0.json").read_text())
if fixture.get("kind") != "MirBuilderTypedObjectPlanRefreshHakoNativeSourceSeedV1":
    raise SystemExit("fixture kind mismatch")
if fixture.get("token") != "MIRBUILDER-TYPED-OBJECT-PLAN-REFRESH-HAKO-NATIVE-SOURCE-SEED-001":
    raise SystemExit("fixture token mismatch")
if fixture.get("family_id") != "hakorune_mir_builder::typed_object_plan_refresh":
    raise SystemExit("family id mismatch")

selection = json.loads(Path("docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-strict-converter-emission-native-seed-candidate-selection-rerun-004-v0.json").read_text())
if selection["decision"]["selected_owner_edge_id"] != "hakorune_mir_builder::typed_object_plan_refresh":
    raise SystemExit("selection did not select typed_object_plan_refresh")
if selection["decision"]["selected_next_card"] != "MIRBUILDER-TYPED-OBJECT-PLAN-REFRESH-HAKO-NATIVE-SOURCE-SEED-001":
    raise SystemExit("selection next card mismatch")

native_path = Path(fixture["native_source_seed"]["path"])
generated_path = Path(fixture["input_authority"]["generated_artifact"])
if not native_path.exists():
    raise SystemExit("native source seed missing")
if not generated_path.exists():
    raise SystemExit("generated artifact missing")
if "lang/generated/" in native_path.as_posix():
    raise SystemExit("native seed must not live under generated tree")
if native_path == generated_path:
    raise SystemExit("native seed must be distinct from generated artifact")

native = native_path.read_text()
for needle in [
    "native-source-seed: MIRBUILDER-TYPED-OBJECT-PLAN-REFRESH-HAKO-NATIVE-SOURCE-SEED-001",
    "source-family: hakorune_mir_builder::typed_object_plan_refresh",
    "source-input-state: DerivedArtifactSeedDraftInput",
    "source-selfhost-claim: 0",
    "box TypedObjectPlanRefreshPayloadBox",
    "box TypedObjectPlanRefreshResultBox",
    "static box TypedObjectPlanRefreshFixtureApi",
    "static box TypedObjectPlanRefreshApi",
    "project_shadow_record(plan, python_oracle, hako_shadow, parity_gate, promotion_token, retirement_token): TypedObjectPlanRefreshResultBox",
]:
    if needle not in native:
        raise SystemExit(f"native seed missing expected text: {needle}")
for forbidden in ["@generated", "manual-edit: forbidden", "static box Main"]:
    if forbidden in native:
        raise SystemExit(f"native seed contains forbidden text: {forbidden}")

module = Path("lang/src/compiler/hako_module.toml").read_text()
if 'lib.typed_object_plan_refresh_native_seed = "lib/typed_object_plan_refresh_native_seed.hako"' not in module:
    raise SystemExit("module export missing typed_object_plan_refresh_native_seed")

if fixture["seed_status"]["native_source_owner_seed_present"] != 1:
    raise SystemExit("native seed presence claim must be 1")
if fixture["seed_status"]["hako_adopted_decision"] != 0:
    raise SystemExit("seed card must not run HakoAdopted decision")
if fixture["native_source_seed"]["generator_overwrite_guard"] is not True:
    raise SystemExit("generator overwrite guard must be true")
claims = fixture["claims"]
for key in [
    "manual_family_selection",
    "family_adoption_decision",
    "source_selfhost_claim",
    "generated_artifact_as_edit_authority",
    "runtime_fallback",
    "new_backend_route",
    "new_abi",
    "new_canonical_mir_instruction",
    "new_python_semantic_projector",
    "runner_semantic_owner",
    "rust_deletion",
]:
    if claims.get(key) != 0:
        raise SystemExit(f"claim must be 0: {key}")
if claims.get("native_seed_materialization") != 1:
    raise SystemExit("native_seed_materialization must be 1")
PY

cat <<'REPORT'
output_contract=rust-lifecycle-mirbuilder-typed-object-plan-refresh-hako-native-source-seed-v0
family_id=hakorune_mir_builder::typed_object_plan_refresh
native_source_seed=lang/src/compiler/lib/typed_object_plan_refresh_native_seed.hako
native_source_owner_seed_present=1
generator_overwrite_guard=1
hako_adopted_decision=0
generated_artifact_as_edit_authority=0
source_selfhost_claim=0
runtime_fallback=0
new_backend_route=0
new_abi=0
new_canonical_mir_instruction=0
new_python_semantic_projector=0
summary=ok
REPORT
