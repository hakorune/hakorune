#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="rust-lifecycle-mirbuilder-programjson-loop-nested-if-var-rhs-bound-row-gate"

source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

FIXTURE="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-programjson-loop-nested-if-var-rhs-bound-row-v0.json"
LOOP_HANDLER="$ROOT_DIR/lang/src/compiler/mirbuilder/stmt_handlers/loop_stmt_handler.hako"
COND_SCAN="$ROOT_DIR/lang/src/compiler/mirbuilder/stmt_handlers/loop_nested_if_cond_scan_box.hako"
BRIDGE="$ROOT_DIR/lang/src/compiler/mirbuilder/stmt_handlers/loop_nested_if_cond_recipe_bridge_box.hako"
PREV_GATE="$ROOT_DIR/tools/checks/rust_lifecycle_mirbuilder_programjson_loop_nested_if_cond_scan_seam_guard.sh"
HAKO_BIN="$ROOT_DIR/tools/bin/hako"

guard_require_command "$TAG" python3
guard_require_files "$TAG" "$FIXTURE" "$LOOP_HANDLER" "$COND_SCAN" "$BRIDGE" "$PREV_GATE" "$HAKO_BIN"

PREV_OUT="$(guard_cached_run "$TAG" bash "$PREV_GATE")"
if ! grep -q '^loop_nested_if_cond_scan_seam=1$' <<<"$PREV_OUT"; then
  printf '%s\n' "$PREV_OUT" >&2
  guard_fail "$TAG" "Loop nested If scan seam prerequisite is not green"
fi

python3 - "$FIXTURE" "$LOOP_HANDLER" "$COND_SCAN" "$BRIDGE" <<'PY'
import json
import sys
from pathlib import Path

fixture = json.loads(Path(sys.argv[1]).read_text(encoding="utf-8"))
loop_impl = Path(sys.argv[2]).read_text(encoding="utf-8")
cond_scan = Path(sys.argv[3]).read_text(encoding="utf-8")
bridge = Path(sys.argv[4]).read_text(encoding="utf-8")

def need(condition, message):
    if not condition:
        raise SystemExit(message)

need(fixture.get("schema_version") == 0, "bad schema_version")
need(fixture.get("kind") == "MirBuilderProgramJsonLoopNestedIfVarRhsBoundRowV1", "bad kind")
need(fixture.get("token") == "MIRBUILDER-PROGRAMJSON-LOOP-NESTED-IF-VAR-RHS-BOUND-ROW-001", "bad token")
need(fixture.get("prerequisite") == "MIRBUILDER-PROGRAMJSON-LOOP-NESTED-IF-COND-SCAN-SEAM-001", "bad prerequisite")
need([row.get("row_id") for row in fixture.get("rows") or []] == ["loop_nested_if_var_lt_var_then_return_assignment"], "row set drift")

scope = fixture.get("row_scope") or {}
need(scope.get("producer_scope") == "one exact Loop-body nested If row", "bad producer scope")
need(scope.get("analysis_only") is True, "row must remain analysis-only")

claims = fixture.get("claims") or {}
for key in [
    "loop_nested_if_var_rhs_bound_row",
    "loop_nested_if_var_rhs_row_implemented",
    "shared_compare_reader_used",
    "loop_nested_if_cond_scan_seam_used",
    "cond_rhs_symbol_ref_published",
    "cond_rhs_kind_code_published",
    "cond_rhs_int_not_published_for_var_rhs",
]:
    need(claims.get(key) == 1, f"missing positive claim: {key}")
need(claims.get("row_count") == 1, "bad row count")
for key in [
    "top_level_loop_var_rhs_row_selected",
    "length_bound_producer_selected",
    "reversed_var_var_context_aware",
    "bool_recipe_lowering_executed",
    "mir_cmp_emission",
    "branch_emission",
    "basic_block_mutation",
    "value_id_allocation",
    "route_selection",
    "runtime_route_switch",
    "programjson_runtime_route_authority",
    "runtime_fallback",
    "source_selfhost_claim",
]:
    need(claims.get(key) == 0, f"forbidden claim drift: {key}")

for needle in [
    "LoopNestedIfCondScanBox.read_int_rhs(",
    "me._put_nested_if_cond_rhs(if_cond_facts, body_out, \"\")",
    'cond_facts.set("cond_rhs_kind_code", kind)',
    'cond_facts.set("cond_rhs_symbol_id", BoxHelpers.map_get(body_out, symbol_key))',
]:
    need(needle in loop_impl, f"LoopStmtHandler missing token: {needle}")
for needle in [
    'if BoxHelpers.same_token(rhs_type, "Var") == 1',
    "ProgramJsonCompareReaderBox.read_var_int_compare(program_json, cond_start)",
    '"if_cond_rhs_kind_code"',
    '"if_cond_rhs_symbol_id"',
]:
    need(needle in cond_scan, f"cond scan missing token: {needle}")
need("BoolRecipeBox.from_numeric_compare_code_map(cond_reader)" in bridge, "bridge must build cond_recipe from shared reader")
for forbidden in ["PlanLowerer", "route_registry", "emit_mir"]:
    need(forbidden not in loop_impl, f"forbidden implementation token: {forbidden}")
PY

TMP_DIR="$(mktemp -d /tmp/hakorune-loop-nested-if-var-rhs.XXXXXX)"
cleanup() { rm -rf "$TMP_DIR" >/dev/null 2>&1 || true; }
trap cleanup EXIT

APP="$TMP_DIR/loop_nested_if_var_rhs.hako"
EXPECTED="$TMP_DIR/expected.txt"
ACTUAL="$TMP_DIR/actual.txt"
EXE="$TMP_DIR/loop_nested_if_var_rhs.exe"
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
    "using lang.compiler.mirbuilder.stmt_handlers.loop_stmt_handler as LoopStmtHandler",
    "using lang.compiler.mirbuilder.recipe.recipe_item_box as RecipeItemBox",
    "",
    "static box Main {",
    "  main() {",
]
expected_lines = []
for idx, row in enumerate(fixture["rows"]):
    out = f"out_{idx}"
    loop_item = f"loop_item_{idx}"
    body = f"body_{idx}"
    body_items = f"body_items_{idx}"
    if_item = f"if_item_{idx}"
    facts = f"facts_{idx}"
    missing = f"missing_{idx}"
    expected_kind = row["expected_cond_kind"]
    lines.extend([
        f"    local program_json_{idx} = {json.dumps(row['program_json'])}",
        f"    local loop_start_{idx} = StringHelpers.index_of(program_json_{idx}, 0, \"{{\\\"type\\\":\\\"Loop\\\"\")",
        f"    local {out} = LoopStmtHandler.handle_state_values(program_json_{idx}, loop_start_{idx}, 0, \"[test]\", 0, 1, 0, 0, \"i\")",
        f"    local {loop_item} = BoxHelpers.map_get({out}, \"recipe_item\")",
        f"    local {body} = BoxHelpers.map_get({loop_item}, \"body_item\")",
        f"    local {body_items} = BoxHelpers.map_get({body}, \"items\")",
        f"    local {if_item} = BoxHelpers.array_get({body_items}, 0)",
        f"    local {facts} = BoxHelpers.map_get({if_item}, \"cond_facts\")",
        f"    local {missing} = 0",
        f"    if BoxHelpers.map_get({facts}, \"cond_rhs_int\") == null {{ {missing} = 1 }}",
        f"    print(\"{row['row_id']}:err=\" + StringHelpers.int_to_str(BoxHelpers.map_get({out}, \"err\"))",
        f"      + \";cond_kind_match=\" + StringHelpers.int_to_str(BoxHelpers.same_token(BoxHelpers.map_get({facts}, \"cond_kind\"), {json.dumps(expected_kind)}))",
        f"      + \";cond_rhs_kind_code=\" + StringHelpers.int_to_str(BoxHelpers.map_get({facts}, \"cond_rhs_kind_code\"))",
        f"      + \";cond_rhs_symbol_id=\" + StringHelpers.int_to_str(BoxHelpers.map_get({facts}, \"cond_rhs_symbol_id\"))",
        f"      + \";cond_rhs_int_missing=\" + StringHelpers.int_to_str({missing})",
        f"      + \";cond_recipe_present=\" + StringHelpers.int_to_str(RecipeItemBox.cond_recipe_present({if_item}))",
        f"      + \";\" + RecipeItemBox.cond_recipe_summary({if_item}))",
    ])
    expected_lines.append(row["expected_summary"])
lines.extend(["    return 0", "  }", "}", ""])
app.write_text("\n".join(lines), encoding="utf-8")
expected.write_text("\n".join(expected_lines) + "\n", encoding="utf-8")
PY

bash "$HAKO_BIN" --backend mir --verify "$LOOP_HANDLER" >/dev/null
bash "$HAKO_BIN" --backend mir --verify "$COND_SCAN" >/dev/null
if ! bash "$HAKO_BIN" --backend mir --emit-exe "$EXE" "$APP" >"$EMIT_LOG" 2>&1; then
  tail -n 160 "$EMIT_LOG" || true
  guard_fail "$TAG" "failed to emit Loop nested If Var rhs executable"
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
    print("[loop-nested-if/var-rhs-bound-row] mismatch")
    for idx in range(max(len(expected), len(actual))):
        exp = expected[idx] if idx < len(expected) else "<missing>"
        got = actual[idx] if idx < len(actual) else "<missing>"
        if exp != got:
            print(f"row={idx} expected={exp!r} actual={got!r}")
    raise SystemExit(1)
PY

cat <<'REPORT'
output_contract=rust-lifecycle-mirbuilder-programjson-loop-nested-if-var-rhs-bound-row-gate-v0
token=MIRBUILDER-PROGRAMJSON-LOOP-NESTED-IF-VAR-RHS-BOUND-ROW-001
owner=LoopStmtHandler+LoopNestedIfCondScanBox
row_count=1
loop_nested_if_var_rhs_bound_row=1
loop_nested_if_var_rhs_row_implemented=1
shared_compare_reader_used=1
loop_nested_if_cond_scan_seam_used=1
cond_rhs_symbol_ref_published=1
cond_rhs_kind_code_published=1
cond_rhs_int_not_published_for_var_rhs=1
top_level_loop_var_rhs_row_selected=0
length_bound_producer_selected=0
reversed_var_var_context_aware=0
bool_recipe_lowering_executed=0
mir_cmp_emission=0
branch_emission=0
basic_block_mutation=0
value_id_allocation=0
route_selection=0
runtime_route_switch=0
programjson_runtime_route_authority=0
runtime_fallback=0
source_selfhost_claim=0
selected_next_card=MIRBUILDER-PROGRAMJSON-VAR-RHS-PRODUCER-NEXT-SELECTION-001
summary=ok
REPORT
