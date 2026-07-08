#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
STATE="$ROOT/docs/development/current/main/CURRENT_STATE.toml"
CARD="$ROOT/docs/development/current/main/phases/phase-296x/3350-MIRBUILDER-BOOL-RECIPE-COMPARE-PUBLICATION-FIRST-PARITY-ROW-SELECTION-001.md"
FIXTURE="$ROOT/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-bool-recipe-compare-publication-parity-v0.json"
PUBLICATION_IMPL="$ROOT/lang/src/compiler/mirbuilder/program_json_bool_recipe_compare_publication.hako"
PREREQ_CARD="$ROOT/docs/development/current/main/phases/phase-296x/3349-MIRBUILDER-SCALAR-KNOWN-FASTPATH-HAKO-SHADOW-CONSUME-MISMATCH-GUARD-EXPANSION-001.md"

python3 - "$STATE" "$CARD" "$FIXTURE" "$PUBLICATION_IMPL" "$PREREQ_CARD" <<'PY'
import json
import sys
import tomllib
from pathlib import Path

state_path, card_path, fixture_path, impl_path, prereq_card_path = map(Path, sys.argv[1:])
state = tomllib.loads(state_path.read_text())
card = card_path.read_text()
fixture = json.loads(fixture_path.read_text())
impl = impl_path.read_text()

def need(cond, msg):
    if not cond:
        raise SystemExit(msg)

token = "MIRBUILDER-BOOL-RECIPE-COMPARE-PUBLICATION-FIRST-PARITY-ROW-SELECTION-001"
next_card = "MIRBUILDER-BOOL-RECIPE-COMPARE-PUBLICATION-FIRST-PARITY-ROW-GATE-001"

need(state.get("latest_card") == token, "latest card drift")
need(state.get("latest_card_path", "").endswith(card_path.name), "latest path drift")
need(state.get("current_blocker_token") == next_card, "current blocker drift")
need(token in card, "card token missing")
need(next_card in card, "selected next missing")
need(prereq_card_path.exists(), "prerequisite card missing")
need(
    "hako_shadow_mismatch_guard_expanded = 1" in prereq_card_path.read_text(),
    "prerequisite mismatch guard expansion not landed",
)

rows = fixture.get("rows") or []
need(len(rows) == 1, "publication fixture must remain first-row scoped")
row = rows[0]
need(row.get("row_id") == "var_le_literal", "first row id drift")
need(row.get("source_program_row") == "local_loop_body_if_branch_return", "source row drift")
need(row.get("loop_condition_patch", {}).get("op") == "<=", "condition op drift")
need("BoolRecipeComparePublicationV1" in row.get("expected_publication_summary", ""), "summary contract drift")

claims = fixture.get("claims") or {}
need(claims.get("read_only_bool_recipe_compare_publication") == 1, "read-only claim missing")
for key in [
    "recipe_item_attachment",
    "recipe_matcher_input_authority",
    "bool_recipe_lowering",
    "mir_cmp_emission",
    "branch_emission",
    "route_selection",
    "runtime_route_switch",
    "source_selfhost_claim",
]:
    need(claims.get(key) == 0, f"forbidden fixture claim drift: {key}")

for needle in [
    "static box ProgramJsonBoolRecipeComparePublicationBox",
    "build_publication(program_json): MapBox",
    "publication_summary(publication)",
]:
    need(needle in impl, f"implementation token missing: {needle}")

for claim in [
    "bool_recipe_compare_publication_first_parity_row_selected = 1",
    "selected_row_id = var_le_literal",
    "selection_only = 1",
]:
    need(claim in card, f"card claim missing: {claim}")

for non_claim in [
    "parity_executed = 0",
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

print("bool_recipe_compare_publication_first_parity_row_selected=1")
print("selected_row_id=var_le_literal")
print("selection_only=1")
print("source_selfhost_claim=0")
PY

cat <<'EOF'
output_contract=rust-lifecycle-mirbuilder-bool-recipe-compare-publication-first-parity-row-selection
token=MIRBUILDER-BOOL-RECIPE-COMPARE-PUBLICATION-FIRST-PARITY-ROW-SELECTION-001
bool_recipe_compare_publication_first_parity_row_selected=1
selected_row_id=var_le_literal
source_fixture_rows=1
selection_only=1
parity_executed=0
recipe_item_attachment=0
recipe_matcher_input_authority=0
bool_recipe_lowering=0
mir_cmp_emission=0
branch_emission=0
route_selection=0
runtime_route_switch=0
source_selfhost_claim=0
selected_next_card=MIRBUILDER-BOOL-RECIPE-COMPARE-PUBLICATION-FIRST-PARITY-ROW-GATE-001
summary=ok
EOF
