#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="rust-lifecycle-mirbuilder-slot-registry-release-hako-native-source-seed-guard"
source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

FIXTURE="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-slot-registry-release-hako-native-source-seed-v0.json"
CARD="$ROOT_DIR/docs/development/current/main/phases/phase-296x/1821-MIRBUILDER-SLOT-REGISTRY-RELEASE-HAKO-NATIVE-SOURCE-SEED-001.md"
NATIVE_SEED="$ROOT_DIR/lang/src/compiler/lib/slot_registry_release_native_seed.hako"
MODULE="$ROOT_DIR/lang/src/compiler/hako_module.toml"
GENERATED="$ROOT_DIR/lang/generated/rust_derived/hakorune_mir_builder/mirbuilder_slot_registry_release.hako"
GENERATED_MANIFEST="$ROOT_DIR/lang/generated/rust_derived/hakorune_mir_builder/mirbuilder_slot_registry_release.artifact.json"
PROMOTION="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/slot-registry-release-hako-shadow-promotion-decision-v0.json"
SELECTION="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-native-owner-seed-pilot-target-selection-v2.json"
VERIFIER="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-slot-registry-release-derived-hako-verifier-result-v0.json"

guard_require_command "$TAG" python3
guard_require_command "$TAG" rg
guard_require_files "$TAG" "$FIXTURE" "$CARD" "$NATIVE_SEED" "$MODULE" "$GENERATED" "$GENERATED_MANIFEST" "$PROMOTION" "$SELECTION" "$VERIFIER"

python3 - <<'PY'
import json
from pathlib import Path

fixture = json.loads(Path("docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-slot-registry-release-hako-native-source-seed-v0.json").read_text())
if fixture.get("kind") != "MirBuilderSlotRegistryReleaseHakoNativeSourceSeedV1":
    raise SystemExit("fixture kind mismatch")
if fixture.get("token") != "MIRBUILDER-SLOT-REGISTRY-RELEASE-HAKO-NATIVE-SOURCE-SEED-001":
    raise SystemExit("fixture token mismatch")
if fixture.get("family_id") != "hakorune_mir_builder::slot_registry_release":
    raise SystemExit("family id mismatch")

selection = json.loads(Path("docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-native-owner-seed-pilot-target-selection-v2.json").read_text())
if selection["decision"]["selected_target"] != "SlotRegistryRelease":
    raise SystemExit("seed target selection did not select SlotRegistryRelease")
if selection["decision"]["next_card"] != "MIRBUILDER-SLOT-REGISTRY-RELEASE-HAKO-NATIVE-SOURCE-SEED-001":
    raise SystemExit("seed target selection next card mismatch")

promotion = json.loads(Path("docs/development/current/main/design/fixtures/rust-lifecycle/slot-registry-release-hako-shadow-promotion-decision-v0.json").read_text())
if promotion["decision"]["kind"] != "Promote":
    raise SystemExit("SlotRegistryRelease promotion must be Promote")
if promotion["selected_stage"] != "HakoMainline":
    raise SystemExit("SlotRegistryRelease selected stage must be HakoMainline")

verifier = json.loads(Path("docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-slot-registry-release-derived-hako-verifier-result-v0.json").read_text())
checks = verifier["checks"]
if checks["slot_registry_release_only"] != 1:
    raise SystemExit("verifier must be limited to SlotRegistryRelease")
for key in ["module_metadata_publication", "metadata_publication", "semantic_refresh", "runtime_fallback"]:
    if checks.get(key) != 0:
        raise SystemExit(f"forbidden verifier boundary opened: {key}")

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
    "MIRBUILDER-SLOT-REGISTRY-RELEASE-HAKO-NATIVE-SOURCE-SEED-001",
    "source-family: hakorune_mir_builder::slot_registry_release",
    "hako-adopted: 0",
    "source-selfhost-claim: 0",
    "static box SlotRegistryReleaseNativeSeedApi",
    "static box SlotRegistryReleaseApi",
    "release(state): FunctionSlotRegistryPreparedBox",
    "apply(state): i64",
    "state.current_slot_registry = null",
    "state.current_slot_registry_present = 0",
    "state.released_registry_present = 1",
    "state.slot_registry_released = 1",
]:
    if needle not in native:
        raise SystemExit(f"native seed missing expected text: {needle}")
for forbidden in ["@generated", "manual-edit: forbidden", "runtime_fallback"]:
    if forbidden in native:
        raise SystemExit(f"native seed contains forbidden text: {forbidden}")

module = Path("lang/src/compiler/hako_module.toml").read_text()
if 'lib.slot_registry_release_native_seed = "lib/slot_registry_release_native_seed.hako"' not in module:
    raise SystemExit("module export missing slot_registry_release_native_seed")

claims = fixture["claims"]
for key in [
    "manual_family_selection",
    "family_adoption_decision",
    "source_selfhost_claim",
    "generated_artifact_as_edit_authority",
    "runtime_fallback",
    "new_backend_route",
    "new_abi",
    "new_python_semantic_projector",
    "runner_semantic_owner",
    "rust_deletion",
]:
    if claims.get(key) != 0:
        raise SystemExit(f"claim must be 0: {key}")
if fixture["seed_status"]["native_source_owner_seed_present"] != 1:
    raise SystemExit("native seed presence claim must be 1")
if fixture["native_source_seed"]["generator_overwrite_guard"] is not True:
    raise SystemExit("generator overwrite guard must be true")

native_rel = native_path.as_posix()
for script in Path("tools/rust_lifecycle").glob("*.py"):
    if native_rel in script.read_text():
        raise SystemExit(f"generator/tool mentions native seed path directly: {script}")
PY

cat <<'REPORT'
output_contract=rust-lifecycle-mirbuilder-slot-registry-release-hako-native-source-seed-v0
family_id=hakorune_mir_builder::slot_registry_release
native_source_seed=lang/src/compiler/lib/slot_registry_release_native_seed.hako
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
