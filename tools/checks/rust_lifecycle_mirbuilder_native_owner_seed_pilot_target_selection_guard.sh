#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
FIXTURE="$ROOT/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-native-owner-seed-pilot-target-selection-v0.json"
CARD="$ROOT/docs/development/current/main/phases/phase-296x/1814-MIRBUILDER-NATIVE-OWNER-SEED-PILOT-TARGET-SELECTION-001.md"
STATE="$ROOT/docs/development/current/main/CURRENT_STATE.toml"
TASK_ORDER="$ROOT/docs/development/current/main/design/mirbuilder-rust-to-hako-converter-task-order-ssot.md"

python3 - "$ROOT" "$FIXTURE" "$CARD" "$STATE" "$TASK_ORDER" <<'PY'
import json
import sys
from pathlib import Path
import tomllib

root, fixture_path, card_path, state_path, task_order_path = map(Path, sys.argv[1:])

TOKEN = "MIRBUILDER-NATIVE-OWNER-SEED-PILOT-TARGET-SELECTION-001"
BLOCKER = "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001"
NEXT = "MIRBUILDER-RETURN-EMISSION-HAKO-NATIVE-SOURCE-SEED-001"

def die(message: str) -> None:
    print(f"[native-owner-seed-pilot-target-selection] {message}", file=sys.stderr)
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

policy = json.loads(read(root / fixture["input_authority"]["generated_artifact_to_seed_policy"]))
if policy.get("decision", {}).get("kind") != "PolicyDefined":
    die("seed policy input drift")
if policy.get("claims", {}).get("generated_artifact_as_edit_authority") != 0:
    die("seed policy must deny generated artifact edit authority")

scope = fixture.get("selection_scope") or {}
if scope.get("kind") != "NativeSourceSeedPilotOnly":
    die("selection scope drift")
if scope.get("not_family_hako_adoption_candidate") is not True:
    die("support lane must not become HakoAdopted candidate here")
if scope.get("not_source_selfhost_claim") is not True:
    die("selection must not claim Source Selfhost")

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
]:
    if required not in criteria:
        die(f"missing target criterion: {required}")

targets = fixture.get("candidate_targets") or []
if len(targets) != 3:
    die("expected exactly three pilot candidates")
eligible = [row for row in targets if row.get("eligible_for_seed_pilot")]
if len(eligible) != 3:
    die("all pilot candidates should be eligible")
ordered = sorted(eligible, key=lambda row: row.get("priority"))
if ordered[0].get("target") != "ReturnEmission":
    die("stable priority should select ReturnEmission")
for expected_target, expected_priority in [
    ("ReturnEmission", 0),
    ("FunctionRegionStackPop", 1),
    ("SlotRegistryRelease", 2),
]:
    matches = [row for row in targets if row.get("target") == expected_target]
    if len(matches) != 1:
        die(f"missing candidate: {expected_target}")
    if matches[0].get("priority") != expected_priority:
        die(f"priority drift for {expected_target}")
    if matches[0].get("promotion_state") != "HakoMainline":
        die(f"promotion state drift for {expected_target}")

decision = fixture.get("decision") or {}
if decision.get("kind") != "SelectNativeOwnerSeedPilotTarget":
    die("decision kind drift")
if decision.get("selected_target") != "ReturnEmission":
    die("selected target must be ReturnEmission")
if decision.get("selection_rule") != "lowest_priority_eligible_target":
    die("selection rule must be stable priority")
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
    "ReturnEmission",
    "support_lane_projector_as_hako_adoption_candidate = 0",
    "source_selfhost_claim",
]:
    if needle not in task_order:
        die(f"task-order missing: {needle}")

print("[native-owner-seed-pilot-target-selection] OK")
PY
