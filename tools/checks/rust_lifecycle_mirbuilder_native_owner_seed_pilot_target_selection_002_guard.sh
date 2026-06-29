#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
FIXTURE="$ROOT/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-native-owner-seed-pilot-target-selection-v1.json"
CARD="$ROOT/docs/development/current/main/phases/phase-296x/1817-MIRBUILDER-NATIVE-OWNER-SEED-PILOT-TARGET-SELECTION-002.md"
STATE="$ROOT/docs/development/current/main/CURRENT_STATE.toml"
TASK_ORDER="$ROOT/docs/development/current/main/design/mirbuilder-rust-to-hako-converter-task-order-ssot.md"

python3 - "$ROOT" "$FIXTURE" "$CARD" "$STATE" "$TASK_ORDER" <<'PY'
import json
import sys
from pathlib import Path
import tomllib

root, fixture_path, card_path, state_path, task_order_path = map(Path, sys.argv[1:])

TOKEN = "MIRBUILDER-NATIVE-OWNER-SEED-PILOT-TARGET-SELECTION-002"
BLOCKER = "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001"
NEXT = "MIRBUILDER-FUNCTION-REGION-STACK-POP-HAKO-NATIVE-SOURCE-SEED-001"

def die(message: str) -> None:
    print(f"[native-owner-seed-pilot-target-selection-002] {message}", file=sys.stderr)
    raise SystemExit(1)

def read(path: Path) -> str:
    if not path.exists():
        die(f"missing path: {path.relative_to(root)}")
    return path.read_text()

fixture = json.loads(read(fixture_path))
card = read(card_path)
state = tomllib.loads(read(state_path))
task_order = read(task_order_path)

if fixture.get("kind") != "MirBuilderNativeOwnerSeedPilotTargetSelectionV1":
    die("fixture kind mismatch")
if fixture.get("token") != TOKEN:
    die("fixture token mismatch")
if TOKEN not in card:
    die("card missing token")
if state.get("current_blocker_token") != BLOCKER:
    die("CURRENT_STATE blocker drift")
if state.get("latest_card") != TOKEN:
    die("CURRENT_STATE latest card drift")
if state.get("latest_card") not in state.get("latest_card_path", ""):
    die("CURRENT_STATE latest path drift")

for rel in (fixture.get("input_authority") or {}).values():
    if not (root / rel).exists():
        die(f"input authority missing: {rel}")

return_adoption = json.loads(read(root / fixture["input_authority"]["return_emission_adoption"]))
if return_adoption.get("decision", {}).get("value") != "Adopt":
    die("ReturnEmission adoption input must be Adopt")
if return_adoption.get("claims", {}).get("source_selfhost_claim") != 0:
    die("ReturnEmission adoption must not claim Source Selfhost")

for key, expected_family in [
    ("function_region_stack_pop_promotion", "hakorune_mir_builder::function_region_stack_pop"),
    ("slot_registry_release_promotion", "hakorune_mir_builder::slot_registry_release"),
]:
    promotion = json.loads(read(root / fixture["input_authority"][key]))
    if promotion.get("family_id") != expected_family:
        die(f"promotion family drift: {key}")
    if promotion.get("decision", {}).get("kind") != "Promote":
        die(f"promotion decision drift: {key}")
    if promotion.get("selected_stage") != "HakoMainline":
        die(f"promotion stage drift: {key}")

criteria = set(fixture.get("target_criteria") or [])
for required in [
    "LeafSemanticOwner",
    "HakoMainlinePromotionGreen",
    "SmallSurface",
    "NoCompositeOwner",
    "NoReturnedBorrow",
    "NoNewAbi",
    "NoNewBackendRoute",
    "NoRuntimeFallback",
    "OracleOrParityFixturePresent",
    "SeedMaterializationCanBeSeparateCard",
    "AlreadyAdoptedExcluded",
]:
    if required not in criteria:
        die(f"missing target criterion: {required}")

targets = fixture.get("candidate_targets") or []
if len(targets) != 3:
    die("expected exactly three pilot candidates")
by_target = {row.get("target"): row for row in targets}
if by_target["ReturnEmission"].get("classification") != "AlreadyAdopted":
    die("ReturnEmission must be excluded as AlreadyAdopted")
if by_target["ReturnEmission"].get("eligible_for_seed_pilot") is not False:
    die("ReturnEmission must not remain eligible")
eligible = sorted(
    [row for row in targets if row.get("eligible_for_seed_pilot")],
    key=lambda row: row.get("priority"),
)
if [row.get("target") for row in eligible] != ["FunctionRegionStackPop", "SlotRegistryRelease"]:
    die("eligible order drift")

decision = fixture.get("decision") or {}
if decision.get("kind") != "SelectNativeOwnerSeedPilotTarget":
    die("decision kind drift")
if decision.get("selected_target") != "FunctionRegionStackPop":
    die("selected target must be FunctionRegionStackPop")
if decision.get("selection_rule") != "lowest_priority_eligible_target_after_excluding_adopted":
    die("selection rule drift")
if decision.get("next_card") != NEXT:
    die("next card drift")

claims = fixture.get("claims") or {}
expected_claims = {
    "support_lane_projector_as_hako_adoption_candidate": 0,
    "support_lane_projector_as_seed_pilot_target": 1,
    "manual_family_selection": 0,
    "native_source_owner_materialized": 0,
    "family_adoption_decision": 0,
    "source_selfhost_claim": 0,
    "generated_artifact_as_edit_authority": 0,
    "runtime_fallback": 0,
    "new_backend_route": 0,
    "new_abi": 0,
    "new_python_semantic_projector": 0,
    "runner_semantic_owner": 0,
}
for key, expected in expected_claims.items():
    if claims.get(key) != expected:
        die(f"claim mismatch for {key}")

for needle in [
    TOKEN,
    NEXT,
    "FunctionRegionStackPop",
    "manual family selection",
    "Source Selfhost claim",
]:
    if needle not in task_order:
        die(f"task-order missing: {needle}")

print("[native-owner-seed-pilot-target-selection-002] OK")
PY
