#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="rust-lifecycle-mirbuilder-programjson-loop-stmt-cond-recipe-sidecar-wiring-gate"

source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

FIXTURE="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-programjson-loop-stmt-cond-recipe-sidecar-wiring-v0.json"
EXPANDED_FIXTURE="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-programjson-recipebodies-verifier-boundary-expanded-dto-coverage-parity-v0.json"
LOOP_HANDLER="$ROOT_DIR/lang/src/compiler/mirbuilder/stmt_handlers/loop_stmt_handler.hako"
RECIPE_ITEM="$ROOT_DIR/lang/src/compiler/mirbuilder/recipe/recipe_item_box.hako"
BOOL_RECIPE="$ROOT_DIR/lang/src/compiler/mirbuilder/recipe/bool_recipe_box.hako"
SELECTION_GATE="$ROOT_DIR/tools/checks/rust_lifecycle_mirbuilder_programjson_recipeitem_cond_recipe_producer_wiring_selection_guard.sh"
TASK_ORDER="$ROOT_DIR/docs/development/current/main/design/mirbuilder-rust-to-hako-converter-task-order-ssot.md"
HAKO_BIN="$ROOT_DIR/tools/bin/hako"

guard_require_command "$TAG" python3
guard_require_files "$TAG" "$FIXTURE" "$EXPANDED_FIXTURE" "$LOOP_HANDLER" "$RECIPE_ITEM" "$BOOL_RECIPE" "$SELECTION_GATE" "$TASK_ORDER" "$HAKO_BIN"

SELECTION_OUT="$(guard_cached_run "$TAG" bash "$SELECTION_GATE")"
if ! grep -q '^selected_producer=LoopStmtHandlerLoopConditionProducer$' <<<"$SELECTION_OUT"; then
  printf '%s\n' "$SELECTION_OUT" >&2
  guard_fail "$TAG" "LoopStmtHandler producer selection prerequisite is not green"
fi

python3 - "$FIXTURE" "$LOOP_HANDLER" "$TASK_ORDER" <<'PY'
import json
import sys
from pathlib import Path

fixture = json.loads(Path(sys.argv[1]).read_text(encoding="utf-8"))
impl = Path(sys.argv[2]).read_text(encoding="utf-8")
task_order = Path(sys.argv[3]).read_text(encoding="utf-8")

def need(condition, message):
    if not condition:
        raise SystemExit(message)

need(fixture.get("schema_version") == 0, "bad schema_version")
need(fixture.get("kind") == "MirBuilderProgramJsonLoopStmtCondRecipeSidecarWiringV1", "bad kind")
need(fixture.get("token") == "MIRBUILDER-PROGRAMJSON-LOOP-STMT-COND-RECIPE-SIDECAR-WIRING-001", "bad token")
need(fixture.get("owner") == "LoopStmtHandler", "bad owner")

contract = fixture.get("wiring_contract") or {}
need(contract.get("source_builder") == "BoolRecipeBox.from_numeric_compare_codes from LoopStmtHandler condition observation", "bad source builder")
need(contract.get("target_attachment") == "RecipeItem.cond_recipe sidecar field", "bad target attachment")
need(contract.get("legacy_cond_facts_required") is True, "legacy cond_facts must remain required")
need(contract.get("lowering_behavior_change") is False, "lowering must not change")
need(contract.get("verifier_behavior_change") is False, "verifier must not change")

claims = fixture.get("claims") or {}
for key in ["loop_stmt_cond_recipe_sidecar_wiring", "recipe_item_attachment_implementation", "legacy_cond_facts_required"]:
    need(claims.get(key) == 1, f"missing positive claim: {key}")
for key in [
    "if_stmt_cond_recipe_wiring",
    "cond_recipe_deep_observation",
    "recipe_matcher_input_authority",
    "bool_recipe_lowering",
    "mir_cmp_emission",
    "branch_emission",
    "route_selection",
    "runtime_route_switch",
    "programjson_runtime_route_authority",
    "runtime_fallback",
    "source_selfhost_claim",
]:
    need(claims.get(key) == 0, f"forbidden claim drift: {key}")

for needle in [
    "using lang.compiler.mirbuilder.recipe.bool_recipe_box as BoolRecipeBox",
    "BoolRecipeBox.from_numeric_compare_codes",
    "RecipeItemBox.loop_item(cond_facts, body_seq)",
    "loop_item.set(\"cond_recipe\", cond_recipe)",
]:
    need(needle in impl, f"LoopStmtHandler missing wiring token: {needle}")
for forbidden in ["RecipeMatcherBox", "emit_mir", "route_registry", "PlanLowerer"]:
    need(forbidden not in impl, f"forbidden implementation token: {forbidden}")
for needle in [
    "MIRBUILDER-PROGRAMJSON-LOOP-STMT-COND-RECIPE-SIDECAR-WIRING-001",
    "MIRBUILDER-RECIPEITEM-COND-RECIPE-OBSERVATION-BOUNDARY-SELECTION-001",
]:
    need(needle in task_order, f"task-order missing: {needle}")
PY

TMP_DIR="$(mktemp -d /tmp/hakorune-loop-cond-recipe-sidecar.XXXXXX)"
cleanup() { rm -rf "$TMP_DIR" >/dev/null 2>&1 || true; }
trap cleanup EXIT

APP="$TMP_DIR/loop_cond_recipe_sidecar.hako"
EXPECTED="$TMP_DIR/expected.txt"
ACTUAL="$TMP_DIR/actual.txt"
EXE="$TMP_DIR/loop_cond_recipe_sidecar.exe"
EMIT_LOG="$TMP_DIR/emit.log"

python3 - "$FIXTURE" "$EXPANDED_FIXTURE" "$APP" "$EXPECTED" <<'PY'
import json
import sys
from pathlib import Path

fixture = json.loads(Path(sys.argv[1]).read_text(encoding="utf-8"))
expanded = json.loads(Path(sys.argv[2]).read_text(encoding="utf-8"))
app = Path(sys.argv[3])
expected = Path(sys.argv[4])
expanded_by_id = {row["row_id"]: row["program_json"] for row in expanded.get("rows") or []}

lines = [
    "using selfhost.shared.common.string_helpers as StringHelpers",
    "using selfhost.shared.common.box_helpers as BoxHelpers",
    "using lang.compiler.mirbuilder.program_json_v0_phase_state_box as ProgramJsonV0PhaseStateBox",
    "using lang.compiler.mirbuilder.recipe.recipe_item_box as RecipeItemBox",
    "",
    "static box Main {",
    "  main() {",
]
expected_lines = []

for idx, row in enumerate(fixture["rows"]):
    program = json.loads(json.dumps(expanded_by_id[row["source_program_row"]]))
    program["body"][1]["cond"] = row["loop_condition_patch"]
    program_json = json.dumps(program, separators=(",", ":"))
    out = f"out_{idx}"
    root = f"root_{idx}"
    items = f"items_{idx}"
    loop_item = f"loop_item_{idx}"
    lines.extend([
        f"    local {out} = ProgramJsonV0PhaseStateBox.parse({json.dumps(program_json)}, \"[test]\")",
        f"    local {root} = BoxHelpers.map_get({out}, \"recipe_root\")",
        f"    local {items} = BoxHelpers.map_get({root}, \"items\")",
        f"    local {loop_item} = BoxHelpers.array_get({items}, 1)",
        f"    print(\"{row['row_id']}:cond_recipe_map=\" + StringHelpers.int_to_str(BoxHelpers.is_map(BoxHelpers.map_get({loop_item}, \"cond_recipe\"))))",
        f"    print(\"{row['row_id']}:legacy_cond_facts_map=\" + StringHelpers.int_to_str(BoxHelpers.is_map(BoxHelpers.map_get({loop_item}, \"cond_facts\"))))",
    ])
    expected_lines.append(f"{row['row_id']}:cond_recipe_map={row['expected_cond_recipe_map']}")
    expected_lines.append(f"{row['row_id']}:legacy_cond_facts_map={row['expected_legacy_cond_facts_map']}")

lines.extend(["    return 0", "  }", "}", ""])
app.write_text("\n".join(lines), encoding="utf-8")
expected.write_text("\n".join(expected_lines) + "\n", encoding="utf-8")
PY

bash "$HAKO_BIN" --backend mir --verify "$LOOP_HANDLER" >/dev/null
if ! bash "$HAKO_BIN" --backend mir --emit-exe "$EXE" "$APP" >"$EMIT_LOG" 2>&1; then
  tail -n 160 "$EMIT_LOG" || true
  guard_fail "$TAG" "failed to emit LoopStmtHandler cond_recipe executable"
fi

chmod +x "$EXE"
"$EXE" >"$ACTUAL.raw"

python3 - "$EXPECTED" "$ACTUAL.raw" "$ACTUAL" <<'PY'
import sys
from pathlib import Path

expected = Path(sys.argv[1]).read_text(encoding="utf-8").splitlines()
raw = Path(sys.argv[2]).read_text(encoding="utf-8").splitlines()
actual_path = Path(sys.argv[3])
actual = [line.strip() for line in raw if line.strip() and not line.startswith("Result:")]
actual_path.write_text("\n".join(actual) + "\n", encoding="utf-8")
if actual != expected:
    print("[loop-stmt/cond-recipe-sidecar] mismatch")
    for idx in range(max(len(expected), len(actual))):
        exp = expected[idx] if idx < len(expected) else "<missing>"
        got = actual[idx] if idx < len(actual) else "<missing>"
        if exp != got:
            print(f"row={idx} expected={exp!r} actual={got!r}")
    raise SystemExit(1)
PY

cat <<'REPORT'
output_contract=rust-lifecycle-mirbuilder-programjson-loop-stmt-cond-recipe-sidecar-wiring-gate-v0
token=MIRBUILDER-PROGRAMJSON-LOOP-STMT-COND-RECIPE-SIDECAR-WIRING-001
owner=LoopStmtHandler
row_count=1
loop_stmt_cond_recipe_sidecar_wiring=1
recipe_item_attachment_implementation=1
legacy_cond_facts_required=1
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
summary=ok
REPORT
