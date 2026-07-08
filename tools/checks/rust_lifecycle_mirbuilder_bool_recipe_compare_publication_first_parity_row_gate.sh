#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
STATE="$ROOT/docs/development/current/main/CURRENT_STATE.toml"
CARD="$ROOT/docs/development/current/main/phases/phase-296x/3351-MIRBUILDER-BOOL-RECIPE-COMPARE-PUBLICATION-FIRST-PARITY-ROW-GATE-001.md"
FIXTURE="$ROOT/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-bool-recipe-compare-publication-parity-v0.json"
BASE_GATE="$ROOT/tools/checks/rust_lifecycle_mirbuilder_bool_recipe_compare_publication_parity_gate.sh"

python3 - "$STATE" "$CARD" "$FIXTURE" "$BASE_GATE" <<'PY'
import json
import sys
import tomllib
from pathlib import Path

state_path, card_path, fixture_path, base_gate_path = map(Path, sys.argv[1:])
state = tomllib.loads(state_path.read_text())
card = card_path.read_text()
fixture = json.loads(fixture_path.read_text())

def need(cond, msg):
    if not cond:
        raise SystemExit(msg)

token = "MIRBUILDER-BOOL-RECIPE-COMPARE-PUBLICATION-FIRST-PARITY-ROW-GATE-001"
next_card = "MIRBUILDER-RECIPEITEM-CONDITION-SLOT-BOOL-RECIPE-BRIDGE-SELECTION-001"

need(state.get("latest_card") == token, "latest card drift")
need(state.get("latest_card_path", "").endswith(card_path.name), "latest path drift")
need(state.get("current_blocker_token") == next_card, "current blocker drift")
need(token in card, "card token missing")
need(next_card in card, "selected next missing")
need(base_gate_path.exists(), "base BoolRecipe publication gate missing")

rows = fixture.get("rows") or []
need(len(rows) == 1, "first-row gate widened")
need(rows[0].get("row_id") == "var_le_literal", "first-row id drift")

for claim in [
    "bool_recipe_compare_publication_first_parity_row_gate = 1",
    "publication_rows = 1",
    "selected_row_id = var_le_literal",
    "read_only_bool_recipe_compare_publication = 1",
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

print("bool_recipe_compare_publication_first_parity_row_gate=1")
print("selected_row_id=var_le_literal")
print("publication_rows=1")
print("source_selfhost_claim=0")
PY

BASE_OUT="$(bash "$BASE_GATE")"
printf '%s\n' "$BASE_OUT"

grep -q '^publication_rows=1$' <<<"$BASE_OUT"
grep -q '^bool_recipe_compare_publication_parity=1$' <<<"$BASE_OUT"
grep -q '^read_only_bool_recipe_compare_publication=1$' <<<"$BASE_OUT"
grep -q '^source_selfhost_claim=0$' <<<"$BASE_OUT"
grep -q '^summary=ok$' <<<"$BASE_OUT"

cat <<'EOF'
output_contract=rust-lifecycle-mirbuilder-bool-recipe-compare-publication-first-parity-row-gate
token=MIRBUILDER-BOOL-RECIPE-COMPARE-PUBLICATION-FIRST-PARITY-ROW-GATE-001
bool_recipe_compare_publication_first_parity_row_gate=1
selected_row_id=var_le_literal
publication_rows=1
read_only_bool_recipe_compare_publication=1
analysis_only=1
recipe_item_attachment=0
recipe_matcher_input_authority=0
bool_recipe_lowering=0
mir_cmp_emission=0
branch_emission=0
route_selection=0
runtime_route_switch=0
source_selfhost_claim=0
selected_next_card=MIRBUILDER-RECIPEITEM-CONDITION-SLOT-BOOL-RECIPE-BRIDGE-SELECTION-001
summary=ok
EOF
