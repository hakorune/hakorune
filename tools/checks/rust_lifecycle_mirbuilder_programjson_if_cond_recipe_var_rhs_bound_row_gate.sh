#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="rust-lifecycle-mirbuilder-programjson-if-cond-recipe-var-rhs-bound-row-gate"

source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

FIXTURE="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-programjson-if-cond-recipe-var-rhs-bound-row-v0.json"
IF_HANDLER="$ROOT_DIR/lang/src/compiler/mirbuilder/stmt_handlers/if_stmt_handler.hako"
SELECTION_GATE="$ROOT_DIR/tools/checks/rust_lifecycle_mirbuilder_programjson_compare_reader_var_rhs_producer_row_selection_guard.sh"
HAKO_BIN="$ROOT_DIR/tools/bin/hako"

guard_require_command "$TAG" python3
guard_require_files "$TAG" "$FIXTURE" "$IF_HANDLER" "$SELECTION_GATE" "$HAKO_BIN"

SELECTION_OUT="$(guard_cached_run "$TAG" bash "$SELECTION_GATE")"
if ! grep -q '^top_level_if_var_rhs_row_selected=1$' <<<"$SELECTION_OUT"; then
  printf '%s\n' "$SELECTION_OUT" >&2
  guard_fail "$TAG" "Var rhs producer row selection prerequisite is not green"
fi

python3 - "$FIXTURE" "$IF_HANDLER" <<'PY'
import json
import sys
from pathlib import Path

fixture = json.loads(Path(sys.argv[1]).read_text(encoding="utf-8"))
impl = Path(sys.argv[2]).read_text(encoding="utf-8")

def need(condition, message):
    if not condition:
        raise SystemExit(message)

need(fixture.get("schema_version") == 0, "bad schema_version")
need(fixture.get("kind") == "MirBuilderProgramJsonIfCondRecipeVarRhsBoundRowV1", "bad kind")
need(fixture.get("token") == "MIRBUILDER-PROGRAMJSON-IF-COND-RECIPE-VAR-RHS-BOUND-ROW-001", "bad token")
need(fixture.get("prerequisite") == "MIRBUILDER-PROGRAMJSON-COMPARE-READER-VAR-RHS-PRODUCER-ROW-SELECTION-001", "bad prerequisite")
need([row.get("row_id") for row in fixture.get("rows") or []] == ["if_var_lt_var_then_return_else_null"], "row set drift")

scope = fixture.get("row_scope") or {}
need(scope.get("producer_scope") == "one exact top-level If row", "bad producer scope")
need(scope.get("analysis_only") is True, "row must remain analysis-only")

claims = fixture.get("claims") or {}
for key in [
    "if_cond_recipe_var_rhs_bound_row",
    "top_level_if_var_rhs_row_implemented",
    "shared_compare_reader_used",
    "cond_rhs_symbol_ref_published",
    "cond_rhs_kind_code_published",
    "cond_rhs_int_not_published_for_var_rhs",
]:
    need(claims.get(key) == 1, f"missing positive claim: {key}")
need(claims.get("row_count") == 1, "bad row count")
for key in [
    "loop_nested_if_var_rhs_row_selected",
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
    'if BoxHelpers.same_token(rhs_type, "Var") == 1',
    'cond_facts.set("cond_rhs_kind", rhs_kind)',
    'cond_facts.set("cond_rhs_kind_code", BoxHelpers.map_get(cond_reader, "bound_kind_code"))',
    'cond_facts.set("cond_rhs_var_name", rhs_var_name)',
    'cond_facts.set("cond_rhs_symbol_id", BoxHelpers.map_get(cond_reader, "bound_symbol_id"))',
    'ProgramJsonCompareReaderBox.read_var_int_compare(program_json, cond_start)',
    'BoolRecipeBox.from_numeric_compare_code_map(cond_reader)',
]:
    need(needle in impl, f"IfStmtHandler missing token: {needle}")
for forbidden in ["PlanLowerer", "route_registry", "emit_mir"]:
    need(forbidden not in impl, f"forbidden implementation token: {forbidden}")
PY

TMP_DIR="$(mktemp -d /tmp/hakorune-if-cond-recipe-var-rhs.XXXXXX)"
cleanup() { rm -rf "$TMP_DIR" >/dev/null 2>&1 || true; }
trap cleanup EXIT

APP="$TMP_DIR/if_cond_recipe_var_rhs.hako"
EXPECTED="$TMP_DIR/expected.txt"
ACTUAL="$TMP_DIR/actual.txt"
EXE="$TMP_DIR/if_cond_recipe_var_rhs.exe"
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
    if_item = f"if_item_{idx}"
    facts = f"facts_{idx}"
    missing = f"missing_{idx}"
    expected_kind = row["expected_cond_kind"]
    lines.extend([
        f"    local {out} = ProgramJsonV0PhaseStateBox.parse({json.dumps(row['program_json'])}, \"[test]\")",
        f"    local {root} = BoxHelpers.map_get({out}, \"recipe_root\")",
        f"    local {items} = BoxHelpers.map_get({root}, \"items\")",
        f"    local {if_item} = BoxHelpers.array_get({items}, 1)",
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

bash "$HAKO_BIN" --backend mir --verify "$IF_HANDLER" >/dev/null
if ! bash "$HAKO_BIN" --backend mir --emit-exe "$EXE" "$APP" >"$EMIT_LOG" 2>&1; then
  tail -n 160 "$EMIT_LOG" || true
  guard_fail "$TAG" "failed to emit If Var rhs cond_recipe executable"
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
    print("[if-stmt/cond-recipe-var-rhs] mismatch")
    for idx in range(max(len(expected), len(actual))):
        exp = expected[idx] if idx < len(expected) else "<missing>"
        got = actual[idx] if idx < len(actual) else "<missing>"
        if exp != got:
            print(f"row={idx} expected={exp!r} actual={got!r}")
    raise SystemExit(1)
PY

cat <<'REPORT'
output_contract=rust-lifecycle-mirbuilder-programjson-if-cond-recipe-var-rhs-bound-row-gate-v0
token=MIRBUILDER-PROGRAMJSON-IF-COND-RECIPE-VAR-RHS-BOUND-ROW-001
owner=IfStmtHandler
row_count=1
if_cond_recipe_var_rhs_bound_row=1
top_level_if_var_rhs_row_implemented=1
shared_compare_reader_used=1
cond_rhs_symbol_ref_published=1
cond_rhs_kind_code_published=1
cond_rhs_int_not_published_for_var_rhs=1
loop_nested_if_var_rhs_row_selected=0
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
selected_next_card=MIRBUILDER-PROGRAMJSON-VAR-RHS-PRODUCER-FOLLOWON-SELECTION-001
summary=ok
REPORT
