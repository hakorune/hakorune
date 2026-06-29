#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="rust-lifecycle-mirbuilder-function-region-stack-pop-hako-native-source-seed-guard"
source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

FIXTURE="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-function-region-stack-pop-hako-native-source-seed-v0.json"
CARD="$ROOT_DIR/docs/development/current/main/phases/phase-296x/1818-MIRBUILDER-FUNCTION-REGION-STACK-POP-HAKO-NATIVE-SOURCE-SEED-001.md"
NATIVE_SEED="$ROOT_DIR/lang/src/compiler/lib/function_region_stack_pop_native_seed.hako"
MODULE="$ROOT_DIR/lang/src/compiler/hako_module.toml"
GENERATED="$ROOT_DIR/lang/generated/rust_derived/hakorune_mir_builder/mirbuilder_function_region_stack_pop.hako"
GENERATED_MANIFEST="$ROOT_DIR/lang/generated/rust_derived/hakorune_mir_builder/mirbuilder_function_region_stack_pop.artifact.json"
PROMOTION="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/function-region-stack-pop-hako-shadow-promotion-decision-v0.json"
SELECTION="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-native-owner-seed-pilot-target-selection-v1.json"
VERIFIER="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-function-region-stack-pop-derived-hako-verifier-result-v0.json"

guard_require_command "$TAG" python3
guard_require_command "$TAG" rg
guard_require_files "$TAG" "$FIXTURE" "$CARD" "$NATIVE_SEED" "$MODULE" "$GENERATED" "$GENERATED_MANIFEST" "$PROMOTION" "$SELECTION" "$VERIFIER"

python3 - <<'PY'
import json
from pathlib import Path

fixture = json.loads(Path("docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-function-region-stack-pop-hako-native-source-seed-v0.json").read_text())
if fixture.get("kind") != "MirBuilderFunctionRegionStackPopHakoNativeSourceSeedV1":
    raise SystemExit("fixture kind mismatch")
if fixture.get("token") != "MIRBUILDER-FUNCTION-REGION-STACK-POP-HAKO-NATIVE-SOURCE-SEED-001":
    raise SystemExit("fixture token mismatch")
if fixture.get("family_id") != "hakorune_mir_builder::function_region_stack_pop":
    raise SystemExit("family id mismatch")

selection = json.loads(Path("docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-native-owner-seed-pilot-target-selection-v1.json").read_text())
if selection["decision"]["selected_target"] != "FunctionRegionStackPop":
    raise SystemExit("seed target selection did not select FunctionRegionStackPop")
if selection["decision"]["next_card"] != "MIRBUILDER-FUNCTION-REGION-STACK-POP-HAKO-NATIVE-SOURCE-SEED-001":
    raise SystemExit("seed target selection next card mismatch")

promotion = json.loads(Path("docs/development/current/main/design/fixtures/rust-lifecycle/function-region-stack-pop-hako-shadow-promotion-decision-v0.json").read_text())
if promotion["decision"]["kind"] != "Promote":
    raise SystemExit("FunctionRegionStackPop promotion must be Promote")
if promotion["selected_stage"] != "HakoMainline":
    raise SystemExit("FunctionRegionStackPop selected stage must be HakoMainline")

verifier = json.loads(Path("docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-function-region-stack-pop-derived-hako-verifier-result-v0.json").read_text())
checks = verifier["checks"]
if checks["function_region_stack_pop_only"] != 1:
    raise SystemExit("verifier must be limited to FunctionRegionStackPop")
for key in ["host_env_lookup", "slot_registry_release", "metadata_publication", "semantic_refresh", "runtime_fallback"]:
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
    "MIRBUILDER-FUNCTION-REGION-STACK-POP-HAKO-NATIVE-SOURCE-SEED-001",
    "source-family: hakorune_mir_builder::function_region_stack_pop",
    "hako-adopted: 0",
    "source-selfhost-claim: 0",
    "static box FunctionRegionStackPopNativeSeedApi",
    "static box FunctionRegionStackPopApi",
    "pop_option(stack): Option<i64>",
    "apply(state): i64",
    "state.stack_size_before = before",
    "state.stack_size_after = after",
    "state.pop_attempted = 1",
]:
    if needle not in native:
        raise SystemExit(f"native seed missing expected text: {needle}")
for forbidden in ["@generated", "manual-edit: forbidden", "host_env_lookup", "runtime_fallback"]:
    if forbidden in native:
        raise SystemExit(f"native seed contains forbidden text: {forbidden}")

module = Path("lang/src/compiler/hako_module.toml").read_text()
if 'lib.function_region_stack_pop_native_seed = "lib/function_region_stack_pop_native_seed.hako"' not in module:
    raise SystemExit("module export missing function_region_stack_pop_native_seed")

claims = fixture["claims"]
for key in [
    "manual_family_selection",
    "family_adoption_decision",
    "source_selfhost_claim",
    "generated_artifact_as_edit_authority",
    "host_env_lookup",
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
output_contract=rust-lifecycle-mirbuilder-function-region-stack-pop-hako-native-source-seed-v0
family_id=hakorune_mir_builder::function_region_stack_pop
native_source_seed=lang/src/compiler/lib/function_region_stack_pop_native_seed.hako
native_source_owner_seed_present=1
generator_overwrite_guard=1
hako_adopted_decision=0
generated_artifact_as_edit_authority=0
source_selfhost_claim=0
host_env_lookup=0
runtime_fallback=0
new_backend_route=0
new_abi=0
new_python_semantic_projector=0
summary=ok
REPORT
