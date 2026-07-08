#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
STATE="$ROOT/docs/development/current/main/CURRENT_STATE.toml"
CARD="$ROOT/docs/development/current/main/phases/phase-296x/3354-MIRBUILDER-RECIPEITEM-CONDITION-SLOT-BOOL-RECIPE-SIDECAR-BRIDGE-CURRENT-GATE-001.md"
BASE_GUARD="$ROOT/tools/checks/rust_lifecycle_mirbuilder_recipeitem_condition_slot_bool_recipe_sidecar_bridge_gate.sh"
TASK_ORDER="$ROOT/docs/development/current/main/design/mirbuilder-rust-to-hako-converter-task-order-ssot.md"

python3 - "$STATE" "$CARD" "$BASE_GUARD" "$TASK_ORDER" <<'PY'
import sys
import tomllib
from pathlib import Path

state_path, card_path, base_guard_path, task_order_path = map(Path, sys.argv[1:])
state = tomllib.loads(state_path.read_text(encoding="utf-8"))
card = card_path.read_text(encoding="utf-8")
task_order = task_order_path.read_text(encoding="utf-8")


def need(cond, msg):
    if not cond:
        raise SystemExit(msg)


token = "MIRBUILDER-RECIPEITEM-CONDITION-SLOT-BOOL-RECIPE-SIDECAR-BRIDGE-CURRENT-GATE-001"
next_card = "MIRBUILDER-PROGRAMJSON-RECIPEITEM-COND-RECIPE-PRODUCER-WIRING-SELECTION-001"

need(state.get("latest_card") == token, "latest card drift")
need(state.get("latest_card_path", "").endswith(card_path.name), "latest path drift")
need(state.get("current_blocker_token") == next_card, "current blocker drift")
need(token in card, "card token missing")
need(next_card in card, "selected next missing")
need(base_guard_path.exists(), "base sidecar bridge guard missing")
need(token in task_order, "task-order missing current token")
need(f"selected_next_card={next_card}" in task_order, "task-order selected next drift")

for claim in [
    "recipeitem_cond_recipe_sidecar_bridge_current_gate = 1",
    "recipeitem_cond_recipe_sidecar_bridge = 1",
    "optional_cond_recipe_sidecar = 1",
    "legacy_cond_facts_required = 1",
    "recipe_item_attachment = 1",
    "current_wrapper = 1",
]:
    need(claim in card, f"card claim missing: {claim}")

for non_claim in [
    "verifier_behavior_change = 0",
    "lowering_behavior_change = 0",
    "recipe_matcher_input_authority = 0",
    "bool_recipe_lowering = 0",
    "mir_cmp_emission = 0",
    "branch_emission = 0",
    "route_selection = 0",
    "runtime_route_switch = 0",
    "programjson_runtime_route_authority = 0",
    "runtime_fallback = 0",
    "source_selfhost_claim = 0",
]:
    need(non_claim in card, f"card non-claim missing: {non_claim}")
PY

BASE_OUT="$(bash "$BASE_GUARD")"
printf '%s\n' "$BASE_OUT"

grep -q '^recipeitem_cond_recipe_sidecar_bridge=1$' <<<"$BASE_OUT"
grep -q '^optional_cond_recipe_sidecar=1$' <<<"$BASE_OUT"
grep -q '^legacy_cond_facts_required=1$' <<<"$BASE_OUT"
grep -q '^recipe_item_attachment=1$' <<<"$BASE_OUT"
grep -q '^verifier_behavior_change=0$' <<<"$BASE_OUT"
grep -q '^lowering_behavior_change=0$' <<<"$BASE_OUT"
grep -q '^recipe_matcher_input_authority=0$' <<<"$BASE_OUT"
grep -q '^source_selfhost_claim=0$' <<<"$BASE_OUT"
grep -q '^summary=ok$' <<<"$BASE_OUT"

cat <<'EOF'
output_contract=rust-lifecycle-mirbuilder-recipeitem-condition-slot-bool-recipe-sidecar-bridge-current-gate
token=MIRBUILDER-RECIPEITEM-CONDITION-SLOT-BOOL-RECIPE-SIDECAR-BRIDGE-CURRENT-GATE-001
recipeitem_cond_recipe_sidecar_bridge_current_gate=1
recipeitem_cond_recipe_sidecar_bridge=1
optional_cond_recipe_sidecar=1
legacy_cond_facts_required=1
recipe_item_attachment=1
current_wrapper=1
verifier_behavior_change=0
lowering_behavior_change=0
recipe_matcher_input_authority=0
bool_recipe_lowering=0
mir_cmp_emission=0
branch_emission=0
route_selection=0
runtime_route_switch=0
programjson_runtime_route_authority=0
runtime_fallback=0
source_selfhost_claim=0
selected_next_card=MIRBUILDER-PROGRAMJSON-RECIPEITEM-COND-RECIPE-PRODUCER-WIRING-SELECTION-001
summary=ok
EOF
