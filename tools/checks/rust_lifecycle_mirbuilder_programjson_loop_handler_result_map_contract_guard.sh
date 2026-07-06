#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="rust-lifecycle-mirbuilder-programjson-loop-handler-result-map-contract-guard"

source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

FIXTURE="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/hako-programjson-loop-handler-result-map-contract-v0.json"
LOOP_HANDLER="$ROOT_DIR/lang/src/compiler/mirbuilder/stmt_handlers/loop_stmt_handler.hako"
CARD="$ROOT_DIR/docs/development/current/main/phases/phase-296x/3209-HAKO-PROGRAMJSON-LOOP-HANDLER-RESULT-MAP-CONTRACT-001.md"
TASK_ORDER="$ROOT_DIR/docs/development/current/main/design/mirbuilder-rust-to-hako-converter-task-order-ssot.md"
HAKO_BIN="$ROOT_DIR/tools/bin/hako"

guard_require_command "$TAG" python3
guard_require_files "$TAG" "$FIXTURE" "$LOOP_HANDLER" "$CARD" "$TASK_ORDER" "$HAKO_BIN"

python3 - "$FIXTURE" "$LOOP_HANDLER" "$CARD" "$TASK_ORDER" <<'PY'
import json
import re
import sys
from pathlib import Path

fixture_path, handler_path, card_path, task_order_path = sys.argv[1:]
fixture = json.loads(Path(fixture_path).read_text(encoding="utf-8"))
handler = Path(handler_path).read_text(encoding="utf-8")
card = Path(card_path).read_text(encoding="utf-8")
task_order = Path(task_order_path).read_text(encoding="utf-8")

token = "HAKO-PROGRAMJSON-LOOP-HANDLER-RESULT-MAP-CONTRACT-001"
next_card = "HAKO-PROGRAMJSON-RECIPEBODIES-ARRAY-HELPER-TOTAL-MAP-CONTRACT-001"

if fixture.get("kind") != "HakoProgramJsonLoopHandlerResultMapContractV1":
    raise SystemExit("bad fixture kind")
if fixture.get("token") != token:
    raise SystemExit("bad fixture token")

rule = fixture.get("contract_rule") or {}
for key in [
    "handler_returns_total_result_map",
    "helper_result_maps_annotated",
    "dynamic_err_line_rewrap_forbidden",
]:
    if rule.get(key) is not True:
        raise SystemExit(f"bad contract rule: {key}")
if rule.get("aot_dynamic_string_return_widening") != "Forbidden":
    raise SystemExit("AOT dynamic string widening must stay forbidden")
if rule.get("by_name_aot_exception") != "Forbidden":
    raise SystemExit("by-name AOT exception must stay forbidden")

for required in fixture.get("required_annotations") or []:
    if required not in handler:
        raise SystemExit(f"missing annotation: {required}")
for required in [
    "handle_state_values(",
    "Loop If then Return payload scan failed",
]:
    if required not in handler:
        raise SystemExit(f"missing loop contract text: {required}")

for forbidden in [
    '"" + BoxHelpers.map_get(then_out, "err_line")',
    '"" + BoxHelpers.map_get(body_out, "err_line")',
    '"" + BoxHelpers.map_get(cond_out, "err_line")',
]:
    if forbidden in handler:
        raise SystemExit(f"dynamic err_line rewrap remains: {forbidden}")
if re.search(r"return\s+null\b", handler):
    raise SystemExit("LoopStmtHandler must not return null")

claims = fixture.get("claims") or {}
if claims.get("loop_handler_result_map_contract") != 1:
    raise SystemExit("contract claim missing")
for key, value in claims.items():
    if key == "loop_handler_result_map_contract":
        continue
    if value != 0:
        raise SystemExit(f"forbidden claim drift: {key}")

if fixture.get("decision", {}).get("selected_next_card") != next_card:
    raise SystemExit("bad next-card decision")
for needle in [token, next_card, "aot_dynamic_string_return_widening=0"]:
    if needle not in card:
        raise SystemExit(f"card missing: {needle}")
if token not in task_order:
    raise SystemExit("task-order loop handler cleanup marker missing")
PY

bash "$HAKO_BIN" --backend mir --verify "$LOOP_HANDLER" >/dev/null

cat <<'REPORT'
output_contract=rust-lifecycle-mirbuilder-programjson-loop-handler-result-map-contract-guard-v0
token=HAKO-PROGRAMJSON-LOOP-HANDLER-RESULT-MAP-CONTRACT-001
owner=LoopStmtHandler
handler_returns_total_result_map=1
helper_result_maps_annotated=1
dynamic_err_line_rewrap_forbidden=1
aot_dynamic_string_return_widening=0
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
selected_next_card=HAKO-PROGRAMJSON-RECIPEBODIES-ARRAY-HELPER-TOTAL-MAP-CONTRACT-001
summary=ok
REPORT
