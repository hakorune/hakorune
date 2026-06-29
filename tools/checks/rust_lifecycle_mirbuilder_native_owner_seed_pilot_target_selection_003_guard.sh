#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
FIXTURE="$ROOT/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-native-owner-seed-pilot-target-selection-v2.json"
CARD="$ROOT/docs/development/current/main/phases/phase-296x/1820-MIRBUILDER-NATIVE-OWNER-SEED-PILOT-TARGET-SELECTION-003.md"
STATE="$ROOT/docs/development/current/main/CURRENT_STATE.toml"
TASK_ORDER="$ROOT/docs/development/current/main/design/mirbuilder-rust-to-hako-converter-task-order-ssot.md"

python3 - "$ROOT" "$FIXTURE" "$CARD" "$STATE" "$TASK_ORDER" <<'PY'
import json
import sys
from pathlib import Path
import tomllib

root, fixture_path, card_path, state_path, task_order_path = map(Path, sys.argv[1:])

TOKEN = "MIRBUILDER-NATIVE-OWNER-SEED-PILOT-TARGET-SELECTION-003"
NEXT = "MIRBUILDER-SLOT-REGISTRY-RELEASE-HAKO-NATIVE-SOURCE-SEED-001"
BLOCKER = "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001"

def die(message: str) -> None:
    print(f"[native-owner-seed-pilot-target-selection-003] {message}", file=sys.stderr)
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

for key in ["return_emission_adoption", "function_region_stack_pop_adoption"]:
    adoption = json.loads(read(root / fixture["input_authority"][key]))
    if adoption.get("decision", {}).get("value") != "Adopt":
        die(f"adoption input must be Adopt: {key}")
    if adoption.get("claims", {}).get("source_selfhost_claim") != 0:
        die(f"adoption input must not claim Source Selfhost: {key}")

promotion = json.loads(read(root / fixture["input_authority"]["slot_registry_release_promotion"]))
if promotion.get("family_id") != "hakorune_mir_builder::slot_registry_release":
    die("SlotRegistryRelease promotion family drift")
if promotion.get("decision", {}).get("kind") != "Promote":
    die("SlotRegistryRelease promotion decision drift")
if promotion.get("selected_stage") != "HakoMainline":
    die("SlotRegistryRelease promotion stage drift")

targets = fixture.get("candidate_targets") or []
by_target = {row.get("target"): row for row in targets}
for target in ["ReturnEmission", "FunctionRegionStackPop"]:
    if by_target[target].get("classification") != "AlreadyAdopted":
        die(f"{target} must be excluded as AlreadyAdopted")
    if by_target[target].get("eligible_for_seed_pilot") is not False:
        die(f"{target} must not remain eligible")
eligible = [row for row in targets if row.get("eligible_for_seed_pilot")]
if len(eligible) != 1 or eligible[0].get("target") != "SlotRegistryRelease":
    die("SlotRegistryRelease must be the only remaining eligible target")

decision = fixture.get("decision") or {}
if decision.get("selected_target") != "SlotRegistryRelease":
    die("selected target drift")
if decision.get("next_card") != NEXT:
    die("next card drift")

claims = fixture.get("claims") or {}
for key, expected in {
    "support_lane_projector_as_hako_adoption_candidate": 0,
    "support_lane_projector_as_seed_pilot_target": 1,
    "manual_family_selection": 0,
    "native_source_owner_materialized": 0,
    "family_adoption_decision": 0,
    "source_selfhost_claim": 0,
    "runtime_fallback": 0,
    "new_backend_route": 0,
    "new_abi": 0,
}.items():
    if claims.get(key) != expected:
        die(f"claim mismatch for {key}")

for needle in [TOKEN, NEXT, "SlotRegistryRelease", "manual family selection", "Source Selfhost claim"]:
    if needle not in task_order:
        die(f"task-order missing: {needle}")

print("[native-owner-seed-pilot-target-selection-003] OK")
PY
