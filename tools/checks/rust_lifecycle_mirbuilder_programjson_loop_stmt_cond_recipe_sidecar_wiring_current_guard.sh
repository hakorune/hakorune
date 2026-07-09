#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
STATE="$ROOT/docs/development/current/main/CURRENT_STATE.toml"
CARD="$ROOT/docs/development/current/main/phases/phase-296x/3356-MIRBUILDER-PROGRAMJSON-LOOP-STMT-COND-RECIPE-SIDECAR-WIRING-CURRENT-GATE-001.md"
BASE_GUARD="$ROOT/tools/checks/rust_lifecycle_mirbuilder_programjson_loop_stmt_cond_recipe_sidecar_wiring_gate.sh"
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


token = "MIRBUILDER-PROGRAMJSON-LOOP-STMT-COND-RECIPE-SIDECAR-WIRING-CURRENT-GATE-001"
base_token = "MIRBUILDER-PROGRAMJSON-LOOP-STMT-COND-RECIPE-SIDECAR-WIRING-001"
next_card = "MIRBUILDER-RECIPEITEM-COND-RECIPE-OBSERVATION-BOUNDARY-SELECTION-001"

need(state.get("latest_card") == token, "latest card drift")
need(state.get("latest_card_path", "").endswith(card_path.name), "latest path drift")
need(state.get("current_blocker_token") == next_card, "current blocker drift")
need(token in card, "card token missing")
need(next_card in card, "selected next missing")
need(base_guard_path.exists(), "base wiring guard missing")
need(token in task_order, "task-order missing current token")
need(base_token in task_order, "task-order missing base token")
need(next_card in task_order, "task-order missing selected next")

for claim in [
    "loop_stmt_cond_recipe_sidecar_wiring_current_gate = 1",
    "loop_stmt_cond_recipe_sidecar_wiring = 1",
    "recipe_item_attachment_implementation = 1",
    "legacy_cond_facts_required = 1",
    "current_wrapper = 1",
]:
    need(claim in card, f"card claim missing: {claim}")

for non_claim in [
    "cond_recipe_deep_observation = 0",
    "if_stmt_cond_recipe_wiring = 0",
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

grep -q '^loop_stmt_cond_recipe_sidecar_wiring=1$' <<<"$BASE_OUT"
grep -q '^recipe_item_attachment_implementation=1$' <<<"$BASE_OUT"
grep -q '^legacy_cond_facts_required=1$' <<<"$BASE_OUT"
grep -q '^cond_recipe_deep_observation=0$' <<<"$BASE_OUT"
grep -q '^if_stmt_cond_recipe_wiring=0$' <<<"$BASE_OUT"
grep -q '^recipe_matcher_input_authority=0$' <<<"$BASE_OUT"
grep -q '^bool_recipe_lowering=0$' <<<"$BASE_OUT"
grep -q '^mir_cmp_emission=0$' <<<"$BASE_OUT"
grep -q '^branch_emission=0$' <<<"$BASE_OUT"
grep -q '^route_selection=0$' <<<"$BASE_OUT"
grep -q '^runtime_route_switch=0$' <<<"$BASE_OUT"
grep -q '^programjson_runtime_route_authority=0$' <<<"$BASE_OUT"
grep -q '^runtime_fallback=0$' <<<"$BASE_OUT"
grep -q '^source_selfhost_claim=0$' <<<"$BASE_OUT"
grep -q '^summary=ok$' <<<"$BASE_OUT"

cat <<'EOF'
output_contract=rust-lifecycle-mirbuilder-programjson-loop-stmt-cond-recipe-sidecar-wiring-current-gate
token=MIRBUILDER-PROGRAMJSON-LOOP-STMT-COND-RECIPE-SIDECAR-WIRING-CURRENT-GATE-001
loop_stmt_cond_recipe_sidecar_wiring_current_gate=1
loop_stmt_cond_recipe_sidecar_wiring=1
recipe_item_attachment_implementation=1
legacy_cond_facts_required=1
current_wrapper=1
cond_recipe_deep_observation=0
if_stmt_cond_recipe_wiring=0
recipe_matcher_input_authority=0
bool_recipe_lowering=0
mir_cmp_emission=0
branch_emission=0
route_selection=0
runtime_route_switch=0
programjson_runtime_route_authority=0
runtime_fallback=0
source_selfhost_claim=0
selected_next_card=MIRBUILDER-RECIPEITEM-COND-RECIPE-OBSERVATION-BOUNDARY-SELECTION-001
summary=ok
EOF
