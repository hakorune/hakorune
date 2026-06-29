#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
FIXTURE="$ROOT/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-minimal-path-composed-closure-native-slice-decomposition-v0.json"
CARD="$ROOT/docs/development/current/main/phases/phase-296x/1811-MIRBUILDER-MINIMAL-PATH-COMPOSED-CLOSURE-NATIVE-SLICE-DECOMPOSITION-001.md"
STATE="$ROOT/docs/development/current/main/CURRENT_STATE.toml"
TASK_ORDER="$ROOT/docs/development/current/main/design/mirbuilder-rust-to-hako-converter-task-order-ssot.md"

python3 - "$ROOT" "$FIXTURE" "$CARD" "$STATE" "$TASK_ORDER" <<'PY'
import json
import sys
from pathlib import Path
import tomllib

root, fixture_path, card_path, state_path, task_order_path = map(Path, sys.argv[1:])

TOKEN = "MIRBUILDER-MINIMAL-PATH-COMPOSED-CLOSURE-NATIVE-SLICE-DECOMPOSITION-001"
BLOCKER = "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001"

def die(message: str) -> None:
    print(f"[minimal-path-native-slice-decomposition] {message}", file=sys.stderr)
    raise SystemExit(1)

def read(path: Path) -> str:
    if not path.exists():
        die(f"missing path: {path.relative_to(root)}")
    return path.read_text()

fixture = json.loads(read(fixture_path))
card = read(card_path)
state = tomllib.loads(read(state_path))
task_order = read(task_order_path)

if fixture.get("kind") != "MirBuilderMinimalPathComposedClosureNativeSliceDecompositionV1":
    die("fixture kind mismatch")
if fixture.get("token") != TOKEN:
    die("fixture token mismatch")
if TOKEN not in card:
    die("card missing token")
if state.get("current_blocker_token") != BLOCKER:
    die("CURRENT_STATE blocker drift")
latest_card = state.get("latest_card") or ""
if latest_card not in state.get("latest_card_path", ""):
    die("CURRENT_STATE latest path drift")

for rel in (fixture.get("input_authority") or {}).values():
    path = root / rel
    if not path.exists():
        die(f"input authority missing: {rel}")

resolution = json.loads(read(root / fixture["input_authority"]["wider_route_selection_resolution"]))
if resolution.get("basis", {}).get("kind") != "KeepSourceSelfhostStopped":
    die("wider route resolution basis drift")
if "MIRBUILDER-MINIMAL-PATH-COMPOSED-CLOSURE-NATIVE-SLICE-DECOMPOSITION-001" not in resolution.get("resolution", {}).get("planned_follow_up_task_packs", []):
    die("wider route resolution no longer names decomposition follow-up")

mainline = json.loads(read(root / fixture["input_authority"]["minimal_path_mainline_pilot"]))
if mainline.get("route_state") != "DerivedMainline":
    die("mainline pilot route state drift")
if mainline.get("claims", {}).get("source_selfhost_claim") != 0:
    die("mainline pilot must not claim Source Selfhost")

adoption = json.loads(read(root / fixture["input_authority"]["allocation_policy_adoption_recheck"]))
if adoption.get("decision") != "Adopt":
    die("allocation policy adoption recheck drift")

slices = fixture.get("slices") or []
classifications = {row.get("candidate_classification") for row in slices}
for required in [
    "AlreadyAdopted",
    "BoundedSurfaceAdopted",
    "ConsultationGated",
    "SupportLaneOnly",
]:
    if required not in classifications:
        die(f"missing slice classification: {required}")

pool = fixture.get("candidate_pool") or {}
if pool.get("candidate_eligible_count") != 0:
    die("candidate_eligible_count must remain zero")
if pool.get("repairable_inconsistency_count") != 0:
    die("repairable_inconsistency_count must remain zero")
if pool.get("consultation_gated_count") != 1:
    die("consultation_gated_count drift")

decision = fixture.get("decision") or {}
if decision.get("kind") != "KeepStopped":
    die("decision kind must be KeepStopped")
if decision.get("selected_next_card") != BLOCKER:
    die("selected next card must remain design stop")
if decision.get("selected_slice_id") is not None:
    die("selected_slice_id must be null")

for key, expected in (fixture.get("claims") or {}).items():
    if expected != 0:
        die(f"claim must be zero: {key}")

for needle in [
    TOKEN,
    BLOCKER,
    "NoCandidateAfterNativeSliceDecomposition",
    "manual_family_selection = 0",
    "source_selfhost_claim",
]:
    if needle not in task_order:
        die(f"task-order missing: {needle}")

print("[minimal-path-native-slice-decomposition] OK")
PY
