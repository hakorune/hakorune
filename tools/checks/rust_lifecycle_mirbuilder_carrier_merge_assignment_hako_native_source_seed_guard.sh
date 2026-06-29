#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="rust-lifecycle-mirbuilder-carrier-merge-assignment-hako-native-source-seed-guard"
source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

FIXTURE="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-carrier-merge-assignment-hako-native-source-seed-v0.json"
CARD="$ROOT_DIR/docs/development/current/main/phases/phase-296x/1871-MIRBUILDER-CARRIER-MERGE-ASSIGNMENT-STATEMENT-HAKO-NATIVE-SOURCE-SEED-001.md"
NATIVE_SEED="$ROOT_DIR/lang/src/compiler/lib/carrier_merge_assignment_native_seed.hako"
MODULE="$ROOT_DIR/lang/src/compiler/hako_module.toml"
PROMOTION="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/carrier-merge-assignment-hako-shadow-promotion-decision-v0.json"
CONTRACT="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-carrier-merge-assignment-statement-mutation-frame-contract-v0.json"

guard_require_command "$TAG" python3
guard_require_command "$TAG" rg
guard_require_files "$TAG" "$FIXTURE" "$CARD" "$NATIVE_SEED" "$MODULE" "$PROMOTION" "$CONTRACT"

python3 - <<'PY'
import json
from pathlib import Path

fixture = json.loads(Path("docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-carrier-merge-assignment-hako-native-source-seed-v0.json").read_text())
if fixture.get("kind") != "MirBuilderCarrierMergeAssignmentHakoNativeSourceSeedV1":
    raise SystemExit("fixture kind mismatch")
if fixture.get("token") != "MIRBUILDER-CARRIER-MERGE-ASSIGNMENT-STATEMENT-HAKO-NATIVE-SOURCE-SEED-001":
    raise SystemExit("fixture token mismatch")
if fixture.get("family_id") != "hakorune_mir_builder::carrier_merge_assignment":
    raise SystemExit("family id mismatch")

promotion = json.loads(Path("docs/development/current/main/design/fixtures/rust-lifecycle/carrier-merge-assignment-hako-shadow-promotion-decision-v0.json").read_text())
if promotion["decision"]["kind"] != "Promote":
    raise SystemExit("carrier-merge assignment promotion must be Promote")
if promotion["selected_stage"] != "HakoMainline":
    raise SystemExit("carrier-merge assignment selected stage must be HakoMainline")

contract = json.loads(Path("docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-carrier-merge-assignment-statement-mutation-frame-contract-v0.json").read_text())
if contract["kind"] != "MirBuilderCarrierMergeAssignmentStatementMutationFrameContractV1":
    raise SystemExit("mutation-frame contract kind mismatch")
frame = contract["mutation_frame_contract"]
if frame["state_outputs"] != ["current_bindings", "carrier_updates", "builder.variable_ctx.variable_map"]:
    raise SystemExit("mutation-frame state outputs mismatch")
if frame["read_only_inputs"] != ["carrier_phis"]:
    raise SystemExit("mutation-frame read-only inputs mismatch")
if frame["mutation_order"] != [
    "ResealBuilderVariableMapFromCurrentBindings",
    "DelegateLoopBodyAssignmentLowering",
    "ReturnEffectsOnlyWhenNoBinding",
    "UpdateCarrierUpdatesWhenCarrierPhiExists",
    "UpdateCurrentBindingsWhenCarrierOrCurrentBindingExists",
    "PublishReturnedBindingToBuilderVariableMap",
]:
    raise SystemExit("mutation-frame order mismatch")

native_path = Path(fixture["native_source_seed"]["path"])
if not native_path.exists():
    raise SystemExit("native source seed missing")
if "lang/generated/" in native_path.as_posix():
    raise SystemExit("native seed must not live under generated tree")

native = native_path.read_text()
for needle in [
    "MIRBUILDER-CARRIER-MERGE-ASSIGNMENT-STATEMENT-HAKO-NATIVE-SOURCE-SEED-001",
    "source-family: hakorune_mir_builder::carrier_merge_assignment",
    "source-selfhost-claim: 0",
    "box CarrierMergeAssignmentStateShellBox",
    "static box CarrierMergeAssignmentNativeSeedApi",
    "static box CarrierMergeAssignmentApi",
    "apply(state): i64",
    "state.resealed_builder_map = 1",
    "state.effects_returned = 1",
    "state.carrier_update_written = 1",
    "state.current_binding_written = 1",
    "state.builder_variable_map_written = 1",
    "state.binding_returned == 0",
    "state.carrier_phi_present != 0",
    "state.current_binding_present != 0",
]:
    if needle not in native:
        raise SystemExit(f"native seed missing expected text: {needle}")
for forbidden in ["@generated", "manual-edit: forbidden", "runtime_fallback"]:
    if forbidden in native:
        raise SystemExit(f"native seed contains forbidden text: {forbidden}")

module = Path("lang/src/compiler/hako_module.toml").read_text()
if 'lib.carrier_merge_assignment_native_seed = "lib/carrier_merge_assignment_native_seed.hako"' not in module:
    raise SystemExit("module export missing carrier_merge_assignment_native_seed")

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

if [[ -x "$ROOT_DIR/tools/bin/hako" ]]; then
  "$ROOT_DIR/tools/bin/hako" --backend mir --verify "$NATIVE_SEED" >/dev/null
fi

cat <<'REPORT'
output_contract=rust-lifecycle-mirbuilder-carrier-merge-assignment-hako-native-source-seed-v0
family_id=hakorune_mir_builder::carrier_merge_assignment
native_source_seed=lang/src/compiler/lib/carrier_merge_assignment_native_seed.hako
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
