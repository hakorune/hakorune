#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
CARD="$ROOT/docs/development/current/main/phases/phase-296x/1803-SOURCE-SELFHOST-DOCS-GUARD-MAINTENANCE-REDUCTION-001.md"
FIXTURE="$ROOT/docs/development/current/main/design/fixtures/rust-lifecycle/source-selfhost-docs-guard-maintenance-reduction-v0.json"
STATE="$ROOT/docs/development/current/main/CURRENT_STATE.toml"
TASK_ORDER="$ROOT/docs/development/current/main/design/mirbuilder-rust-to-hako-converter-task-order-ssot.md"
POLICY="$ROOT/docs/development/current/main/design/current-docs-update-policy-ssot.md"
INDEX="$ROOT/docs/tools/check-scripts-index.md"
MANIFEST="$ROOT/docs/development/current/main/design/fixtures/rust-lifecycle/source-selfhost-family-guard-manifest-v0.json"

python3 - "$ROOT" "$CARD" "$FIXTURE" "$STATE" "$TASK_ORDER" "$POLICY" "$INDEX" "$MANIFEST" <<'PY'
import json
import sys
from pathlib import Path
import tomllib

root, card, fixture, state, task_order, policy, index, manifest = map(Path, sys.argv[1:])

TOKEN = "SOURCE-SELFHOST-DOCS-GUARD-MAINTENANCE-REDUCTION-001"
BLOCKER = "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001"
NEXT = "MIRBUILDER-MINIMAL-PATH-COMPOSED-CLOSURE-NATIVE-SLICE-DECOMPOSITION-001"
REQUIRED_TASKS = {
    "MIRBUILDER-MINIMAL-PATH-MAINLINE-READINESS-GUARD-REALIGNMENT-001",
    "GUARD-SOURCE-SELFHOST-CURRENT-POINTER-DECOUPLE-001",
    "GUARD-SOURCE-SELFHOST-MANIFEST-FAMILY-001",
    "DOCS-SOURCE-SELFHOST-COMPACT-CURRENT-STATE-001",
    "DOCS-SOURCE-SELFHOST-TASK-ORDER-THINNING-001",
    "DOCS-CHECK-INDEX-FAMILY-VIEW-001",
}
ALLOWED_POST_MAINTENANCE = {
    "SOURCE-SELFHOST-POST-MAINTENANCE-TASK-INVENTORY-001",
    "MIRBUILDER-MINIMAL-PATH-COMPOSED-CLOSURE-NATIVE-SLICE-DECOMPOSITION-001",
    "MIRBUILDER-MINIMAL-PATH-COMPOSED-CLOSURE-NATIVE-OWNER-SEED-INVENTORY-001",
    "MIRBUILDER-GENERATED-ARTIFACT-TO-NATIVE-OWNER-SEED-POLICY-001",
    "MIRBUILDER-NATIVE-OWNER-SEED-PILOT-TARGET-SELECTION-001",
}

def die(msg: str) -> None:
    print(f"[source-selfhost-docs-guard-maintenance-reduction] {msg}", file=sys.stderr)
    raise SystemExit(1)

def read(path: Path) -> str:
    if not path.exists():
        die(f"missing path: {path.relative_to(root)}")
    return path.read_text()

card_text = read(card)
fixture_data = json.loads(read(fixture))
manifest_data = json.loads(read(manifest))
state_text = read(state)
task_order_text = read(task_order)
policy_text = read(policy)
index_text = read(index)

if TOKEN not in card_text:
    die("card missing token")
if "code_or_guard_delta_required = 1" not in card_text:
    die("card missing code_or_guard_delta_required acceptance")
if BLOCKER not in card_text:
    die("card missing design-stop blocker preservation")

if fixture_data.get("kind") != "SourceSelfhostDocsGuardMaintenanceReductionV1":
    die("fixture kind mismatch")
if fixture_data.get("token") != TOKEN:
    die("fixture token mismatch")
if fixture_data.get("input_state", {}).get("current_blocker_token") != BLOCKER:
    die("fixture does not preserve current design-stop blocker")
if fixture_data.get("deferred_semantic_next", {}).get("task") != NEXT:
    die("fixture missing deferred semantic next task")

tasks = {row.get("task") for row in fixture_data.get("maintenance_sequence", [])}
missing = REQUIRED_TASKS - tasks
if missing:
    die(f"fixture missing maintenance tasks: {sorted(missing)}")

claims = fixture_data.get("claims", {})
for key in [
    "manual_family_selection",
    "source_selfhost_claim",
    "runtime_fallback",
    "new_backend_route",
    "new_abi",
    "new_python_semantic_projector",
    "runner_semantic_owner",
    "route_repair",
    "family_adoption_decision",
]:
    if claims.get(key) != 0:
        die(f"claim must be zero: {key}")

state_data = tomllib.loads(state_text)
manifest_tokens = {row.get("token") for row in manifest_data.get("rows", []) if row.get("token")}
allowed_latest_cards = {TOKEN} | REQUIRED_TASKS | ALLOWED_POST_MAINTENANCE | manifest_tokens
latest_card = state_data.get("latest_card")
latest_card_path = state_data.get("latest_card_path") or ""
if latest_card not in allowed_latest_cards:
    die(f"CURRENT_STATE latest_card is outside maintenance sequence: {latest_card}")
if latest_card not in latest_card_path:
    die("CURRENT_STATE latest_card_path does not reference latest_card")
if state_data.get("current_blocker_token") != BLOCKER:
    die("CURRENT_STATE current blocker drifted from design stop")

for needle in [TOKEN, "GUARD-SOURCE-SELFHOST-MANIFEST-FAMILY-001", NEXT]:
    if needle not in task_order_text:
        die(f"task-order missing: {needle}")

for needle in [
    "do not create a dedicated `.sh` guard for every row",
    "prefer one reusable lane guard per bucket",
    "CURRENT_STATE.toml",
]:
    if needle not in policy_text:
        die(f"current docs update policy missing expected row/guard-cost rule: {needle}")

if "rust_lifecycle_source_selfhost_family_guard.sh" not in index_text:
    die("check-scripts index missing Source Selfhost family guard")

print("[source-selfhost-docs-guard-maintenance-reduction] OK")
PY
