#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="rust-lifecycle-mirbuilder-programjson-recipebodies-array-helper-total-map-contract-guard"

source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

FIXTURE="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/hako-programjson-recipebodies-array-helper-total-map-contract-v0.json"
IMPL="$ROOT_DIR/lang/src/compiler/mirbuilder/program_json_recipebodies_one_shape_arena_builder.hako"
CARD="$ROOT_DIR/docs/development/current/main/phases/phase-296x/3210-HAKO-PROGRAMJSON-RECIPEBODIES-ARRAY-HELPER-TOTAL-MAP-CONTRACT-001.md"
TASK_ORDER="$ROOT_DIR/docs/development/current/main/design/mirbuilder-rust-to-hako-converter-task-order-ssot.md"
HAKO_BIN="$ROOT_DIR/tools/bin/hako"

guard_require_command "$TAG" python3
guard_require_files "$TAG" "$FIXTURE" "$IMPL" "$CARD" "$TASK_ORDER" "$HAKO_BIN"

python3 - "$FIXTURE" "$IMPL" "$CARD" "$TASK_ORDER" <<'PY'
import json
import re
import sys
from pathlib import Path

fixture_path, impl_path, card_path, task_order_path = sys.argv[1:]
fixture = json.loads(Path(fixture_path).read_text(encoding="utf-8"))
impl = Path(impl_path).read_text(encoding="utf-8")
card = Path(card_path).read_text(encoding="utf-8")
task_order = Path(task_order_path).read_text(encoding="utf-8")

token = "HAKO-PROGRAMJSON-RECIPEBODIES-ARRAY-HELPER-TOTAL-MAP-CONTRACT-001"
next_card = "MIRBUILDER-PROGRAMJSON-RECIPEBODIES-AFTER-LOOP-BODY-ARENA-NEXT-CONTRACT-SELECTION-001"

if fixture.get("kind") != "HakoProgramJsonRecipeBodiesArrayHelperTotalMapContractV1":
    raise SystemExit("bad fixture kind")
if fixture.get("token") != token:
    raise SystemExit("bad fixture token")

rule = fixture.get("contract_rule") or {}
for key in [
    "raw_array_helper_return_removed",
    "body_builder_returns_mapbox",
    "one_shape_arena_contract_unchanged",
]:
    if rule.get(key) is not True:
        raise SystemExit(f"bad contract rule: {key}")
if rule.get("aot_array_return_widening") != "Forbidden":
    raise SystemExit("AOT array return widening must stay forbidden")
if rule.get("by_name_aot_exception") != "Forbidden":
    raise SystemExit("by-name AOT exception must stay forbidden")

for removed in fixture.get("removed_helpers") or []:
    if removed in impl:
        raise SystemExit(f"raw array helper remains: {removed}")
if "_body_map(items, n): MapBox" not in impl:
    raise SystemExit("body builder must return MapBox")
if not re.search(r"_body_map\(items, n\): MapBox \{.*local out = \[\].*out\.push\(me\._item_ref", impl, re.S):
    raise SystemExit("body item array must be local to MapBox body builder")
if re.search(r"_build_body_items\s*\(", impl):
    raise SystemExit("_build_body_items helper must be removed")

claims = fixture.get("claims") or {}
if claims.get("recipebodies_array_helper_contract") != 1:
    raise SystemExit("contract claim missing")
for key, value in claims.items():
    if key == "recipebodies_array_helper_contract":
        continue
    if value != 0:
        raise SystemExit(f"forbidden claim drift: {key}")

if fixture.get("decision", {}).get("selected_next_card") != next_card:
    raise SystemExit("bad next-card decision")
for needle in [token, next_card, "aot_array_return_widening=0"]:
    if needle not in card:
        raise SystemExit(f"card missing: {needle}")
if token not in task_order:
    raise SystemExit("task-order RecipeBodies array helper marker missing")
PY

bash "$HAKO_BIN" --backend mir --verify "$IMPL" >/dev/null

cat <<'REPORT'
output_contract=rust-lifecycle-mirbuilder-programjson-recipebodies-array-helper-total-map-contract-guard-v0
token=HAKO-PROGRAMJSON-RECIPEBODIES-ARRAY-HELPER-TOTAL-MAP-CONTRACT-001
owner=ProgramJsonRecipeBodiesOneShapeArenaBuilderBox
raw_array_helper_return_removed=1
body_builder_returns_mapbox=1
one_shape_arena_contract_unchanged=1
aot_array_return_widening=0
by_name_aot_exception=0
programjson_new_shape=0
recipe_bodies_materialization=0
runtime_recipe_bodies_arena=0
full_recipe_matcher_execution=0
route_selection=0
mir_mutation=0
id_allocation=0
backend_lowering_claim=0
runtime_route_switch=0
source_selfhost_claim=0
selected_next_card=MIRBUILDER-PROGRAMJSON-RECIPEBODIES-AFTER-LOOP-BODY-ARENA-NEXT-CONTRACT-SELECTION-001
summary=ok
REPORT
