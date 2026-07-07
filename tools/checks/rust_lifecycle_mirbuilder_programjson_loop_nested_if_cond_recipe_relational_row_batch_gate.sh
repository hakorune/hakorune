#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="rust-lifecycle-mirbuilder-programjson-loop-nested-if-cond-recipe-relational-row-batch-gate"

source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

FIXTURE="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-programjson-loop-nested-if-cond-recipe-relational-row-batch-v0.json"
LOOP_HANDLER="$ROOT_DIR/lang/src/compiler/mirbuilder/stmt_handlers/loop_stmt_handler.hako"
BRIDGE="$ROOT_DIR/lang/src/compiler/mirbuilder/stmt_handlers/loop_nested_if_cond_recipe_bridge_box.hako"
PREV_GATE="$ROOT_DIR/tools/checks/rust_lifecycle_mirbuilder_programjson_if_cond_recipe_relational_row_batch_gate.sh"
HAKO_BIN="$ROOT_DIR/tools/bin/hako"

guard_require_command "$TAG" python3
guard_require_files "$TAG" "$FIXTURE" "$LOOP_HANDLER" "$BRIDGE" "$PREV_GATE" "$HAKO_BIN"

PREV_OUT="$(guard_cached_run "$TAG" bash "$PREV_GATE")"
if ! grep -q '^if_cond_recipe_relational_row_batch=1$' <<<"$PREV_OUT"; then
  printf '%s\n' "$PREV_OUT" >&2
  guard_fail "$TAG" "If relational row batch prerequisite is not green"
fi

python3 - "$FIXTURE" "$LOOP_HANDLER" "$BRIDGE" <<'PY'
import json
import sys
from pathlib import Path

fixture = json.loads(Path(sys.argv[1]).read_text(encoding="utf-8"))
loop_impl = Path(sys.argv[2]).read_text(encoding="utf-8")
bridge = Path(sys.argv[3]).read_text(encoding="utf-8")

def need(condition, message):
    if not condition:
        raise SystemExit(message)

need(fixture.get("schema_version") == 0, "bad schema_version")
need(fixture.get("kind") == "MirBuilderProgramJsonLoopNestedIfCondRecipeRelationalRowBatchV1", "bad kind")
need(fixture.get("token") == "MIRBUILDER-PROGRAMJSON-LOOP-NESTED-IF-COND-RECIPE-RELATIONAL-ROW-BATCH-001", "bad token")
need(fixture.get("prerequisite") == "MIRBUILDER-PROGRAMJSON-IF-COND-RECIPE-RELATIONAL-ROW-BATCH-001", "bad prerequisite")
need([row.get("row_id") for row in fixture.get("rows") or []] == [
    "loop_body_if_var_lt_int_then_return_assignment",
    "loop_body_if_var_le_int_then_return_assignment",
    "loop_body_if_var_gt_int_then_return_assignment",
    "loop_body_if_var_ge_int_then_return_assignment",
], "row set drift")

claims = fixture.get("claims") or {}
for key in ["loop_nested_if_cond_recipe_relational_row_batch", "shared_compare_reader_used", "legacy_cond_facts_relational"]:
    need(claims.get(key) == 1, f"missing positive claim: {key}")
need(claims.get("loop_nested_if_relational_rows") == 4, "bad relational row count")
for key in [
    "top_level_loop_route_semantics_changed",
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
    "LoopNestedIfCondRecipeBridgeBox.if_item(",
    "If cond Compare op is unsupported",
]:
    need(needle in loop_impl, f"LoopStmtHandler missing token: {needle}")
for needle in [
    'if cmp == 1 { cond_facts.set("cond_kind", "VarLtInt") }',
    'if cmp == 2 { cond_facts.set("cond_kind", "VarLeInt") }',
    'if cmp == 3 { cond_facts.set("cond_kind", "VarGtInt") }',
    'if cmp == 4 { cond_facts.set("cond_kind", "VarGeInt") }',
    "BoolRecipeBox.from_numeric_compare_code_map(cond_reader)",
]:
    need(needle in bridge, f"bridge missing token: {needle}")
PY

TMP_DIR="$(mktemp -d /tmp/hakorune-loop-nested-if-rel.XXXXXX)"
cleanup() { rm -rf "$TMP_DIR" >/dev/null 2>&1 || true; }
trap cleanup EXIT

APP="$TMP_DIR/loop_nested_if_rel.hako"
EXPECTED="$TMP_DIR/expected.txt"
ACTUAL="$TMP_DIR/actual.txt"
EXE="$TMP_DIR/loop_nested_if_rel.exe"
EMIT_LOG="$TMP_DIR/emit.log"

python3 - "$FIXTURE" "$APP" "$EXPECTED" <<'PY'
import json
import sys
from pathlib import Path

fixture = json.loads(Path(sys.argv[1]).read_text(encoding="utf-8"))
app = Path(sys.argv[2])
expected = Path(sys.argv[3])

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
    out = f"out_{idx}"
    root = f"root_{idx}"
    items = f"items_{idx}"
    loop_item = f"loop_item_{idx}"
    body = f"body_{idx}"
    body_items = f"body_items_{idx}"
    if_item = f"if_item_{idx}"
    facts = f"facts_{idx}"
    expected_kind = row["expected_cond_kind"]
    lines.extend([
        f"    local {out} = ProgramJsonV0PhaseStateBox.parse({json.dumps(row['program_json'])}, \"[test]\")",
        f"    local {root} = BoxHelpers.map_get({out}, \"recipe_root\")",
        f"    local {items} = BoxHelpers.map_get({root}, \"items\")",
        f"    local {loop_item} = BoxHelpers.array_get({items}, 1)",
        f"    local {body} = BoxHelpers.map_get({loop_item}, \"body_item\")",
        f"    local {body_items} = BoxHelpers.map_get({body}, \"items\")",
        f"    local {if_item} = BoxHelpers.array_get({body_items}, 0)",
        f"    local {facts} = BoxHelpers.map_get({if_item}, \"cond_facts\")",
        f"    local err_{idx} = BoxHelpers.map_get({out}, \"err\")",
        f"    local cond_kind_match_{idx} = BoxHelpers.same_token(BoxHelpers.map_get({facts}, \"cond_kind\"), {json.dumps(expected_kind)})",
        f"    local cond_rhs_int_{idx} = BoxHelpers.map_get({facts}, \"cond_rhs_int\")",
        f"    local cond_recipe_present_{idx} = RecipeItemBox.cond_recipe_present({if_item})",
        f"    local cond_recipe_summary_{idx} = RecipeItemBox.cond_recipe_summary({if_item})",
        f"    print(\"{row['row_id']}:err=\" + StringHelpers.int_to_str(err_{idx})",
        f"      + \";cond_kind_match=\" + StringHelpers.int_to_str(cond_kind_match_{idx})",
        f"      + \";cond_rhs_int=\" + StringHelpers.int_to_str(cond_rhs_int_{idx})",
        f"      + \";cond_recipe_present=\" + StringHelpers.int_to_str(cond_recipe_present_{idx})",
        f"      + \";\" + cond_recipe_summary_{idx})",
    ])
    expected_lines.append(row["expected_summary"])
lines.extend(["    return 0", "  }", "}", ""])
app.write_text("\n".join(lines), encoding="utf-8")
expected.write_text("\n".join(expected_lines) + "\n", encoding="utf-8")
PY

bash "$HAKO_BIN" --backend mir --verify "$LOOP_HANDLER" >/dev/null
bash "$HAKO_BIN" --backend mir --verify "$BRIDGE" >/dev/null
if ! bash "$HAKO_BIN" --backend mir --emit-exe "$EXE" "$APP" >"$EMIT_LOG" 2>&1; then
  tail -n 160 "$EMIT_LOG" || true
  guard_fail "$TAG" "failed to emit Loop nested If relational executable"
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
    print("[loop-nested-if/cond-recipe-relational-row-batch] mismatch")
    for idx in range(max(len(expected), len(actual))):
        exp = expected[idx] if idx < len(expected) else "<missing>"
        got = actual[idx] if idx < len(actual) else "<missing>"
        if exp != got:
            print(f"row={idx} expected={exp!r} actual={got!r}")
    raise SystemExit(1)
PY

cat <<'REPORT'
output_contract=rust-lifecycle-mirbuilder-programjson-loop-nested-if-cond-recipe-relational-row-batch-gate-v0
token=MIRBUILDER-PROGRAMJSON-LOOP-NESTED-IF-COND-RECIPE-RELATIONAL-ROW-BATCH-001
owner=LoopStmtHandler nested If producer
row_count=4
loop_nested_if_cond_recipe_relational_row_batch=1
loop_nested_if_relational_rows=4
shared_compare_reader_used=1
legacy_cond_facts_relational=1
top_level_loop_route_semantics_changed=0
recipe_matcher_input_authority=0
bool_recipe_lowering=0
mir_cmp_emission=0
branch_emission=0
route_selection=0
runtime_route_switch=0
programjson_runtime_route_authority=0
runtime_fallback=0
source_selfhost_claim=0
selected_next_card=MIRBUILDER-BOOL-RECIPE-COMPARE-LOWERING-EMISSION-CONSULTATION-001
summary=ok
REPORT
