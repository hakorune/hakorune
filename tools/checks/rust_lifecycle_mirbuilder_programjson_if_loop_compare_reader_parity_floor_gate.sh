#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="rust-lifecycle-mirbuilder-programjson-if-loop-compare-reader-parity-floor-gate"

source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

FIXTURE="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-programjson-if-loop-compare-reader-parity-floor-v0.json"
PREV_GATE="$ROOT_DIR/tools/checks/rust_lifecycle_mirbuilder_bool_recipe_compare_publication_parity_gate.sh"
COMPARE_READER="$ROOT_DIR/lang/src/compiler/mirbuilder/program_json_compare_reader_box.hako"
PHASE_STATE="$ROOT_DIR/lang/src/compiler/mirbuilder/program_json_v0_phase_state_box.hako"
TASK_ORDER="$ROOT_DIR/docs/development/current/main/design/mirbuilder-rust-to-hako-converter-task-order-ssot.md"
HAKO_BIN="$ROOT_DIR/tools/bin/hako"

guard_require_command "$TAG" python3
guard_require_files "$TAG" "$FIXTURE" "$PREV_GATE" "$COMPARE_READER" "$PHASE_STATE" "$TASK_ORDER" "$HAKO_BIN"

PREV_OUT="$(guard_cached_run "$TAG" bash "$PREV_GATE")"
if ! grep -q '^bool_recipe_compare_publication_parity=1$' <<<"$PREV_OUT"; then
  printf '%s\n' "$PREV_OUT" >&2
  guard_fail "$TAG" "BoolRecipe publication prerequisite is not green"
fi

python3 - "$FIXTURE" "$COMPARE_READER" "$TASK_ORDER" <<'PY'
import json
import sys
from pathlib import Path

fixture = json.loads(Path(sys.argv[1]).read_text(encoding="utf-8"))
reader = Path(sys.argv[2]).read_text(encoding="utf-8")
task_order = Path(sys.argv[3]).read_text(encoding="utf-8")

def need(condition, message):
    if not condition:
        raise SystemExit(message)

need(fixture.get("schema_version") == 0, "bad schema_version")
need(fixture.get("kind") == "MirBuilderProgramJsonIfLoopCompareReaderParityFloorV1", "bad kind")
need(fixture.get("token") == "MIRBUILDER-PROGRAMJSON-IF-LOOP-COMPARE-READER-PARITY-FLOOR-001", "bad token")
need(fixture.get("prerequisite") == "MIRBUILDER-BOOL-RECIPE-COMPARE-PUBLICATION-PARITY-001", "bad prerequisite")
need(fixture.get("contract", {}).get("analysis_only") is True, "analysis_only must be true")
need(fixture.get("contract", {}).get("lowering") is False, "lowering must be false")

rows = fixture.get("rows") or []
need([row.get("target") for row in rows] == ["top_level_if", "top_level_loop", "loop_body_nested_if"], "bad row targets")
for row in rows:
    need("ProgramJsonCompareReaderCodeMapV1;ok=1" in row.get("expected_reader_summary", ""), "bad reader expectation")
    need("cond_recipe_present=1;bool_recipe_kind=Compare" in row.get("expected_recipe_summary", ""), "bad recipe expectation")

claims = fixture.get("claims") or {}
for key in ["if_loop_compare_reader_parity_floor", "direct_reader_matches_recipe_cond_recipe", "top_level_if_row", "top_level_loop_row", "loop_body_nested_if_row"]:
    need(claims.get(key) == 1, f"missing positive claim: {key}")
for key in [
    "if_accepts_all_6_compare_operators",
    "loop_nested_if_operator_expansion",
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

need("read_var_int_compare(program_json, compare_start): MapBox" in reader, "shared reader missing")
need("MIRBUILDER-PROGRAMJSON-IF-LOOP-COMPARE-READER-PARITY-FLOOR-001" in task_order, "task-order missing floor")
PY

TMP_DIR="$(mktemp -d /tmp/hakorune-if-loop-compare-reader-floor.XXXXXX)"
cleanup() { rm -rf "$TMP_DIR" >/dev/null 2>&1 || true; }
trap cleanup EXIT

APP="$TMP_DIR/if_loop_compare_reader_floor.hako"
EXPECTED="$TMP_DIR/expected.txt"
ACTUAL="$TMP_DIR/actual.txt"
EXE="$TMP_DIR/if_loop_compare_reader_floor.exe"
EMIT_LOG="$TMP_DIR/emit.log"

python3 - "$FIXTURE" "$APP" "$EXPECTED" <<'PY'
import json
import sys
from pathlib import Path

fixture = json.loads(Path(sys.argv[1]).read_text(encoding="utf-8"))
app = Path(sys.argv[2])
expected = Path(sys.argv[3])

lines = [
    "using selfhost.shared.common.box_helpers as BoxHelpers",
    "using lang.compiler.mirbuilder.program_json_compare_reader_box as ProgramJsonCompareReaderBox",
    "using lang.compiler.mirbuilder.program_json_v0_phase_state_box as ProgramJsonV0PhaseStateBox",
    "using lang.compiler.mirbuilder.recipe.recipe_item_box as RecipeItemBox",
    "",
    "static box Main {",
    "  main() {",
]
expected_lines = []
for idx, row in enumerate(fixture["rows"]):
    reader = f"reader_{idx}"
    out = f"out_{idx}"
    item = f"item_{idx}"
    lines.extend([
        f"    local {reader} = ProgramJsonCompareReaderBox.read_var_int_compare({json.dumps(row['compare_json'])}, 0)",
        f"    print(\"reader:{row['row_id']}:\" + ProgramJsonCompareReaderBox.code_map_summary({reader}))",
        f"    local {out} = ProgramJsonV0PhaseStateBox.parse({json.dumps(row['program_json'])}, \"[test]\")",
    ])
    root = f"root_{idx}"
    items = f"items_{idx}"
    lines.extend([
        f"    local {root} = BoxHelpers.map_get({out}, \"recipe_root\")",
        f"    local {items} = BoxHelpers.map_get({root}, \"items\")",
    ])
    if row["target"] in ("top_level_if", "top_level_loop"):
        lines.append(f"    local {item} = BoxHelpers.array_get({items}, 1)")
    else:
        loop_item = f"loop_item_{idx}"
        body = f"body_{idx}"
        body_items = f"body_items_{idx}"
        lines.extend([
            f"    local {loop_item} = BoxHelpers.array_get({items}, 1)",
            f"    local {body} = BoxHelpers.map_get({loop_item}, \"body_item\")",
            f"    local {body_items} = BoxHelpers.map_get({body}, \"items\")",
            f"    local {item} = BoxHelpers.array_get({body_items}, 0)",
        ])
    lines.append(f"    print(\"recipe:{row['row_id']}:\" + RecipeItemBox.cond_recipe_summary({item}))")
    expected_lines.append(f"reader:{row['row_id']}:{row['expected_reader_summary']}")
    expected_lines.append(f"recipe:{row['row_id']}:{row['expected_recipe_summary']}")
lines.extend(["    return 0", "  }", "}", ""])
app.write_text("\n".join(lines), encoding="utf-8")
expected.write_text("\n".join(expected_lines) + "\n", encoding="utf-8")
PY

bash "$HAKO_BIN" --backend mir --verify "$COMPARE_READER" >/dev/null
if ! bash "$HAKO_BIN" --backend mir --emit-exe "$EXE" "$APP" >"$EMIT_LOG" 2>&1; then
  tail -n 160 "$EMIT_LOG" || true
  guard_fail "$TAG" "failed to emit If/Loop compare reader parity floor executable"
fi

chmod +x "$EXE"
"$EXE" >"$ACTUAL.raw"

python3 - "$EXPECTED" "$ACTUAL.raw" "$ACTUAL" <<'PY'
import sys
from pathlib import Path

expected = Path(sys.argv[1]).read_text(encoding="utf-8").splitlines()
raw = Path(sys.argv[2]).read_text(encoding="utf-8").splitlines()
actual = [line.strip() for line in raw if line.strip() and not line.startswith("Result:")]
Path(sys.argv[3]).write_text("\n".join(actual) + "\n", encoding="utf-8")
if actual != expected:
    print("[if-loop/compare-reader-floor] mismatch")
    for idx in range(max(len(expected), len(actual))):
        exp = expected[idx] if idx < len(expected) else "<missing>"
        got = actual[idx] if idx < len(actual) else "<missing>"
        if exp != got:
            print(f"row={idx} expected={exp!r} actual={got!r}")
    raise SystemExit(1)
PY

cat <<'REPORT'
output_contract=rust-lifecycle-mirbuilder-programjson-if-loop-compare-reader-parity-floor-gate-v0
token=MIRBUILDER-PROGRAMJSON-IF-LOOP-COMPARE-READER-PARITY-FLOOR-001
row_count=3
if_loop_compare_reader_parity_floor=1
direct_reader_matches_recipe_cond_recipe=1
top_level_if_row=1
top_level_loop_row=1
loop_body_nested_if_row=1
if_accepts_all_6_compare_operators=0
loop_nested_if_operator_expansion=0
recipe_matcher_input_authority=0
bool_recipe_lowering=0
route_selection=0
runtime_route_switch=0
programjson_runtime_route_authority=0
runtime_fallback=0
source_selfhost_claim=0
selected_next_card=MIRBUILDER-BOOL-RECIPE-COMPARE-LOWERING-BOUNDARY-CONSULTATION-001
summary=ok
REPORT
