#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
FIXTURE="$ROOT/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-minimal-path-composed-closure-native-owner-seed-inventory-v0.json"
CARD="$ROOT/docs/development/current/main/phases/phase-296x/1812-MIRBUILDER-MINIMAL-PATH-COMPOSED-CLOSURE-NATIVE-OWNER-SEED-INVENTORY-001.md"
STATE="$ROOT/docs/development/current/main/CURRENT_STATE.toml"
TASK_ORDER="$ROOT/docs/development/current/main/design/mirbuilder-rust-to-hako-converter-task-order-ssot.md"

python3 - "$ROOT" "$FIXTURE" "$CARD" "$STATE" "$TASK_ORDER" <<'PY'
import json
import sys
from pathlib import Path
import tomllib

root, fixture_path, card_path, state_path, task_order_path = map(Path, sys.argv[1:])

TOKEN = "MIRBUILDER-MINIMAL-PATH-COMPOSED-CLOSURE-NATIVE-OWNER-SEED-INVENTORY-001"
BLOCKER = "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001"
DECOMPOSITION = "MIRBUILDER-MINIMAL-PATH-COMPOSED-CLOSURE-NATIVE-SLICE-DECOMPOSITION-001"

def die(message: str) -> None:
    print(f"[minimal-path-native-owner-seed-inventory] {message}", file=sys.stderr)
    raise SystemExit(1)

def read(path: Path) -> str:
    if not path.exists():
        die(f"missing path: {path.relative_to(root)}")
    return path.read_text()

fixture = json.loads(read(fixture_path))
card = read(card_path)
state = tomllib.loads(read(state_path))
task_order = read(task_order_path)

if fixture.get("kind") != "MirBuilderMinimalPathComposedClosureNativeOwnerSeedInventoryV1":
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
    if not (root / rel).exists():
        die(f"input authority missing: {rel}")

decomposition = json.loads(read(root / fixture["input_authority"]["native_slice_decomposition"]))
if decomposition.get("token") != DECOMPOSITION:
    die("input decomposition token mismatch")
if decomposition.get("decision", {}).get("kind") != "KeepStopped":
    die("input decomposition must remain KeepStopped")
if decomposition.get("candidate_pool", {}).get("candidate_eligible_count") != 0:
    die("input decomposition candidate count drift")
if decomposition.get("candidate_pool", {}).get("repairable_inconsistency_count") != 0:
    die("input decomposition repair count drift")

source = fixture.get("input_state") or {}
if source.get("source_slice") != "minimal_path_composed_execution_closure":
    die("source slice drift")
if source.get("source_reason_token") != "GeneratedArtifactIsNotNativeEditAuthority":
    die("generated artifact reason must be preserved")

inventory = fixture.get("leaf_owner_inventory") or []
if not inventory:
    die("leaf owner inventory is empty")
classifications = {row.get("classification") for row in inventory}
for required in [
    "NotSemanticOwner",
    "GeneratedArtifactOnly",
    "AlreadyAdopted",
    "BoundedSurfaceOnly",
]:
    if required not in classifications:
        die(f"missing leaf owner classification: {required}")
if any(row.get("eligible_for_native_owner_seed") for row in inventory):
    die("this inventory must not select a native owner seed")

reasons = {row.get("reason_token") for row in inventory}
for required in [
    "CompositionOwnerIsNotSemanticFamilyOwner",
    "GeneratedArtifactIsNotNativeEditAuthority",
    "AlreadyAdopted",
    "FullVariableContextClaimParked",
    "SupportLaneProjectorIsNotFamilyAdoptionCandidate",
]:
    if required not in reasons:
        die(f"missing reason token: {required}")

pool = fixture.get("candidate_pool") or {}
if pool.get("native_owner_seed_candidate_count") != 0:
    die("native_owner_seed_candidate_count must remain zero")
if pool.get("composite_needs_decomposition_count") != 0:
    die("composite_needs_decomposition_count must remain zero")
if pool.get("blocked_generated_artifact_only_count") != 1:
    die("blocked_generated_artifact_only_count drift")

decision = fixture.get("decision") or {}
if decision.get("kind") != "KeepStopped":
    die("decision must be KeepStopped")
if decision.get("reason_token") != "NoNativeOwnerSeedCandidate":
    die("decision reason drift")
if decision.get("selected_leaf_owner_id") is not None:
    die("selected_leaf_owner_id must remain null")
if decision.get("selected_next_card") != BLOCKER:
    die("selected next card must remain design stop")
if "exactly one leaf semantic owner" not in decision.get("recovery_message", ""):
    die("recovery message must describe the machine-derived resume condition")

claims = fixture.get("claims") or {}
for key in [
    "manual_family_selection",
    "composition_owner_as_semantic_owner",
    "generated_artifact_as_edit_authority",
    "native_source_owner_materialized",
    "source_selfhost_claim",
    "family_adoption_decision",
    "route_repair",
    "runtime_fallback",
    "new_backend_route",
    "new_abi",
    "new_python_semantic_projector",
    "runner_semantic_owner",
]:
    if claims.get(key) != 0:
        die(f"claim must be zero: {key}")

for needle in [
    TOKEN,
    BLOCKER,
    "NoNativeOwnerSeedCandidate",
    "composition_owner_as_semantic_owner = 0",
    "generated_artifact_as_edit_authority = 0",
    "source_selfhost_claim",
]:
    if needle not in task_order:
        die(f"task-order missing: {needle}")

print("[minimal-path-native-owner-seed-inventory] OK")
PY
