#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
STATE="$ROOT/docs/development/current/main/CURRENT_STATE.toml"
CARD="$ROOT/docs/development/current/main/phases/phase-296x/3352-MIRBUILDER-RECIPEITEM-CONDITION-SLOT-BOOL-RECIPE-BRIDGE-SELECTION-CURRENT-001.md"
BASE_GUARD="$ROOT/tools/checks/rust_lifecycle_mirbuilder_recipeitem_condition_slot_bool_recipe_bridge_selection_guard.sh"
TASK_ORDER="$ROOT/docs/development/current/main/design/mirbuilder-rust-to-hako-converter-task-order-ssot.md"

python3 - "$STATE" "$CARD" "$BASE_GUARD" "$TASK_ORDER" <<'PY'
import sys
import tomllib
from pathlib import Path

state_path, card_path, base_guard_path, task_order_path = map(Path, sys.argv[1:])
state = tomllib.loads(state_path.read_text())
card = card_path.read_text()
task_order = task_order_path.read_text()

def need(cond, msg):
    if not cond:
        raise SystemExit(msg)

token = "MIRBUILDER-RECIPEITEM-CONDITION-SLOT-BOOL-RECIPE-BRIDGE-SELECTION-CURRENT-001"
next_card = "MIRBUILDER-RECIPEITEM-CONDITION-SLOT-BOOL-RECIPE-SIDECAR-BRIDGE-CURRENT-GATE-001"

need(state.get("latest_card") == token, "latest card drift")
need(state.get("latest_card_path", "").endswith(card_path.name), "latest path drift")
need(state.get("current_blocker_token") == next_card, "current blocker drift")
need(token in card, "card token missing")
need(next_card in card, "selected next missing")
need(base_guard_path.exists(), "base selection guard missing")

for needle in [
    "MIRBUILDER-RECIPEITEM-CONDITION-SLOT-BOOL-RECIPE-BRIDGE-SELECTION-001",
    "MIRBUILDER-RECIPEITEM-CONDITION-SLOT-BOOL-RECIPE-SIDECAR-BRIDGE-001",
]:
    need(needle in task_order, f"task-order missing: {needle}")

for claim in [
    "condition_slot_bridge_selection = 1",
    "selected_optional_cond_recipe_sidecar = 1",
    "current_wrapper = 1",
]:
    need(claim in card, f"card claim missing: {claim}")

for non_claim in [
    "recipe_item_attachment = 0",
    "recipe_matcher_input_authority = 0",
    "bool_recipe_lowering = 0",
    "mir_cmp_emission = 0",
    "branch_emission = 0",
    "route_selection = 0",
    "runtime_route_switch = 0",
    "source_selfhost_claim = 0",
]:
    need(non_claim in card, f"card non-claim missing: {non_claim}")
PY

BASE_OUT="$(bash "$BASE_GUARD")"
printf '%s\n' "$BASE_OUT"

grep -q '^selected_bridge=OptionalCondRecipeSidecar$' <<<"$BASE_OUT"
grep -q '^selected_next_card=MIRBUILDER-RECIPEITEM-CONDITION-SLOT-BOOL-RECIPE-SIDECAR-BRIDGE-001$' <<<"$BASE_OUT"
grep -q '^legacy_cond_facts_required=1$' <<<"$BASE_OUT"
grep -q '^cond_recipe_optional=1$' <<<"$BASE_OUT"
grep -q '^source_selfhost_claim=0$' <<<"$BASE_OUT"
grep -q '^summary=ok$' <<<"$BASE_OUT"

cat <<'EOF'
output_contract=rust-lifecycle-mirbuilder-recipeitem-condition-slot-bool-recipe-bridge-selection-current
token=MIRBUILDER-RECIPEITEM-CONDITION-SLOT-BOOL-RECIPE-BRIDGE-SELECTION-CURRENT-001
condition_slot_bridge_selection=1
selected_optional_cond_recipe_sidecar=1
selected_bridge=OptionalCondRecipeSidecar
legacy_cond_facts_required=1
cond_recipe_optional=1
current_wrapper=1
recipe_item_attachment=0
recipe_matcher_input_authority=0
bool_recipe_lowering=0
mir_cmp_emission=0
branch_emission=0
route_selection=0
runtime_route_switch=0
source_selfhost_claim=0
selected_next_card=MIRBUILDER-RECIPEITEM-CONDITION-SLOT-BOOL-RECIPE-SIDECAR-BRIDGE-CURRENT-GATE-001
summary=ok
EOF
