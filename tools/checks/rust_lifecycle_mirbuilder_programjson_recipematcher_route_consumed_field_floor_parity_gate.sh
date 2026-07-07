#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="rust-lifecycle-mirbuilder-programjson-recipematcher-route-consumed-field-floor-parity"

source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

FIXTURE="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-programjson-recipematcher-route-consumed-field-floor-parity-v0.json"
BREAK_FIXTURE="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-programjson-recipematcher-break-present-verified-recipe-support-v0.json"
BREAK_CONTINUE_FIXTURE="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-programjson-recipematcher-break-continue-present-verified-recipe-support-v0.json"
CARD="$ROOT_DIR/docs/development/current/main/phases/phase-296x/3250-MIRBUILDER-PROGRAMJSON-RECIPEMATCHER-ROUTE-CONSUMED-FIELD-FLOOR-PARITY-001.md"
TASK_ORDER="$ROOT_DIR/docs/development/current/main/design/mirbuilder-rust-to-hako-converter-task-order-ssot.md"
CURRENT_STATE="$ROOT_DIR/docs/development/current/main/CURRENT_STATE.toml"
PREV_GUARD="$ROOT_DIR/tools/checks/rust_lifecycle_mirbuilder_programjson_recipematcher_route_consumed_field_floor_selection_guard.sh"
HAKO_BIN="$ROOT_DIR/tools/bin/hako"

guard_require_command "$TAG" python3
guard_require_command "$TAG" timeout
guard_require_files "$TAG" "$FIXTURE" "$BREAK_FIXTURE" "$BREAK_CONTINUE_FIXTURE" "$CARD" "$TASK_ORDER" "$CURRENT_STATE" "$PREV_GUARD" "$HAKO_BIN"

PREV_OUT="$(HAKO_GUARD_RESULT_CACHE_ALLOW_DIRTY=1 guard_cached_run "$TAG-prev" bash "$PREV_GUARD")"
if ! grep -q '^route_consumed_field_floor_selection=1$' <<<"$PREV_OUT"; then
  printf '%s\n' "$PREV_OUT" >&2
  guard_fail "$TAG" "field-floor selection prerequisite is not green"
fi

TMP_DIR="$(mktemp -d /tmp/hakorune-route-field-floor-parity.XXXXXX)"
cleanup() { rm -rf "$TMP_DIR" >/dev/null 2>&1 || true; }
trap cleanup EXIT

APP="$TMP_DIR/probe.hako"
EXE="$TMP_DIR/probe.exe"
MIR_JSON="$TMP_DIR/probe.mir.json"
BUILD_LOG="$TMP_DIR/build.log"
RUN_OUT="$TMP_DIR/run.out"
RUN_FILTERED="$TMP_DIR/run.filtered"
RUN_ERR="$TMP_DIR/run.err"

python3 - "$FIXTURE" "$BREAK_FIXTURE" "$BREAK_CONTINUE_FIXTURE" "$CARD" "$TASK_ORDER" "$CURRENT_STATE" "$APP" <<'PY'
import json
import sys
from pathlib import Path

fixture_path, break_path, break_continue_path, card_path, task_order_path, current_state_path, app_path = sys.argv[1:]
fixture = json.loads(Path(fixture_path).read_text(encoding="utf-8"))
break_fixture = json.loads(Path(break_path).read_text(encoding="utf-8"))
break_continue_fixture = json.loads(Path(break_continue_path).read_text(encoding="utf-8"))
card = Path(card_path).read_text(encoding="utf-8")
task_order = Path(task_order_path).read_text(encoding="utf-8")
current_state = Path(current_state_path).read_text(encoding="utf-8")
app = Path(app_path)

token = "MIRBUILDER-PROGRAMJSON-RECIPEMATCHER-ROUTE-CONSUMED-FIELD-FLOOR-PARITY-001"
if fixture.get("kind") != "MirBuilderProgramJsonRecipeMatcherRouteConsumedFieldFloorParityV1":
    raise SystemExit("bad fixture kind")
if fixture.get("token") != token:
    raise SystemExit("bad fixture token")
rows = fixture.get("rows") or []
if [row.get("row_id") for row in rows] != ["break_present_fields", "break_continue_present_fields"]:
    raise SystemExit("row order drift")

source_by_id = {
    "break_present_fields": break_fixture["row"]["program_json"],
    "break_continue_present_fields": break_continue_fixture["row"]["program_json"],
}
for row in rows:
    source = source_by_id[row["row_id"]]
    if not isinstance(source, dict):
        raise SystemExit(f"missing source program json for {row['row_id']}")

claims = fixture.get("claims") or {}
for key in ["route_consumed_field_floor_parity_green", "accepted_rows_field_floor_checked", "programjson_shadow_checked"]:
    if claims.get(key) != 1:
        raise SystemExit(f"missing positive claim: {key}")
for key, value in claims.items():
    if key in {"route_consumed_field_floor_parity_green", "accepted_rows_field_floor_checked", "programjson_shadow_checked"}:
        continue
    if value != 0:
        raise SystemExit(f"forbidden claim drift: {key}")

for needle in [
    token,
    "break_present_fields",
    "break_continue_present_fields",
    "runtime_route_switch = 0",
    "programjson_runtime_route_authority = 0",
    "Source Selfhost remains unclaimed",
]:
    if needle not in card:
        raise SystemExit(f"card missing: {needle}")
for needle in [token, "MIRBUILDER-PROGRAMJSON-RECIPEMATCHER-AUTHORITY-SWITCH-READINESS-CONSULTATION-001"]:
    if needle not in task_order:
        raise SystemExit(f"task-order missing: {needle}")
if f'latest_card = "{token}"' not in current_state:
    raise SystemExit("CURRENT_STATE latest card drift")

program_json_0 = json.dumps(json.dumps(source_by_id["break_present_fields"], separators=(",", ":"), ensure_ascii=False))
program_json_1 = json.dumps(json.dumps(source_by_id["break_continue_present_fields"], separators=(",", ":"), ensure_ascii=False))
app.write_text("\n".join([
    "using lang.compiler.mirbuilder.program_json_canonical_loop_facts_input_snapshot as ProgramJsonCanonicalLoopFactsInputSnapshotBox",
    "using lang.compiler.mirbuilder.program_json_recipematcher_execution_boundary as ProgramJsonRecipeMatcherExecutionBoundaryBox",
    "",
    "static box Main {",
    "  main() {",
    f"    local program_json_0 = {program_json_0}",
    f"    local program_json_1 = {program_json_1}",
    "    local snapshot_0 = ProgramJsonCanonicalLoopFactsInputSnapshotBox.build_snapshot(program_json_0)",
    "    local matched_0 = ProgramJsonRecipeMatcherExecutionBoundaryBox.match_snapshot(snapshot_0)",
    "    print(\"row=break_present_fields;snapshot:\" + ProgramJsonCanonicalLoopFactsInputSnapshotBox.snapshot_summary(snapshot_0))",
    "    print(\"row=break_present_fields;match:\" + ProgramJsonRecipeMatcherExecutionBoundaryBox.match_summary(matched_0))",
    "    local snapshot_1 = ProgramJsonCanonicalLoopFactsInputSnapshotBox.build_snapshot(program_json_1)",
    "    local matched_1 = ProgramJsonRecipeMatcherExecutionBoundaryBox.match_snapshot(snapshot_1)",
    "    print(\"row=break_continue_present_fields;snapshot:\" + ProgramJsonCanonicalLoopFactsInputSnapshotBox.snapshot_summary(snapshot_1))",
    "    print(\"row=break_continue_present_fields;match:\" + ProgramJsonRecipeMatcherExecutionBoundaryBox.match_summary(matched_1))",
    "    return 0",
    "  }",
    "}",
    "",
]), encoding="utf-8")
PY

if ! timeout 90s bash "$HAKO_BIN" --backend mir --emit-mir-json "$MIR_JSON" "$APP" >"$BUILD_LOG" 2>&1; then
  tail -n 160 "$BUILD_LOG" >&2 || true
  guard_fail "$TAG" "failed to emit MIR JSON"
fi

python3 - "$MIR_JSON" <<'PY'
import json
import sys
from pathlib import Path

data = json.loads(Path(sys.argv[1]).read_text(encoding="utf-8"))
main = next((fn for fn in data.get("functions", []) if fn.get("name") == "main"), None)
if main is None:
    raise SystemExit("main function missing")
rows = []
for key in ("global_call_routes", "lowering_plan"):
    value = (main.get("metadata") or {}).get(key) or []
    rows.extend(row for row in value if isinstance(row, dict))
for symbol in [
    "ProgramJsonCanonicalLoopFactsInputSnapshotBox.build_snapshot/1",
    "ProgramJsonRecipeMatcherExecutionBoundaryBox.match_snapshot/1",
]:
    matches = [row for row in rows if row.get("symbol") == symbol]
    if not matches:
        raise SystemExit(f"missing main call: {symbol}")
    for row in matches:
        if row.get("tier") != "DirectAbi" or row.get("return_shape") != "map_handle":
            raise SystemExit(f"call is not DirectAbi/map_handle: {row}")
PY

if ! timeout --kill-after=2s 120s env \
    NYASH_LLVM_OPT_LEVEL=0 \
    HAKO_LLVM_OPT_LEVEL=0 \
    bash "$HAKO_BIN" --backend mir --emit-exe "$EXE" "$APP" >"$BUILD_LOG" 2>&1; then
  tail -n 160 "$BUILD_LOG" >&2 || true
  guard_fail "$TAG" "emit-exe failed or timed out"
fi
if ! "$EXE" >"$RUN_OUT" 2>"$RUN_ERR"; then
  cat "$RUN_ERR" >&2 || true
  guard_fail "$TAG" "executable failed"
fi
grep -v '^Result: 0$' "$RUN_OUT" >"$RUN_FILTERED" || true

python3 - "$FIXTURE" "$RUN_FILTERED" <<'PY'
import json
import sys
from pathlib import Path

fixture = json.loads(Path(sys.argv[1]).read_text(encoding="utf-8"))
lines = [line.strip() for line in Path(sys.argv[2]).read_text(encoding="utf-8").splitlines() if line.strip()]
if len(lines) != 4:
    raise SystemExit(f"expected four output lines, got {len(lines)}: {lines}")

def parse(row_id, kind):
    prefix = f"row={row_id};{kind}:"
    for line in lines:
        if line.startswith(prefix):
            parts = line[len(prefix):].split(";")
            return dict(part.split("=", 1) for part in parts if "=" in part)
    raise SystemExit(f"missing line: {prefix}")

for row in fixture["rows"]:
    row_id = row["row_id"]
    snap = parse(row_id, "snapshot")
    match = parse(row_id, "match")
    for key, value in row["expected_snapshot"].items():
        if snap.get(key) != str(value):
            raise SystemExit(f"{row_id} snapshot {key}: {snap.get(key)} != {value}")
    for key, value in row["expected_match"].items():
        if match.get(key) != str(value):
            raise SystemExit(f"{row_id} match {key}: {match.get(key)} != {value}")
    for key, value in {
        "full_recipe_matcher_execution": "0",
        "route_selection": "0",
        "mir_lowering": "0",
        "mir_mutation": "0",
        "id_allocation": "0",
        "runtime_route_switch": "0",
    }.items():
        if match.get(key) != value:
            raise SystemExit(f"{row_id} non-claim {key}: {match.get(key)} != {value}")
PY

cat <<'REPORT'
output_contract=rust-lifecycle-mirbuilder-programjson-recipematcher-route-consumed-field-floor-parity-v0
token=MIRBUILDER-PROGRAMJSON-RECIPEMATCHER-ROUTE-CONSUMED-FIELD-FLOOR-PARITY-001
route_consumed_field_floor_parity_green=1
accepted_rows_field_floor_checked=1
row_count=2
rows=break_present_fields,break_continue_present_fields
programjson_shadow_checked=1
programjson_runtime_route_authority=0
runtime_route_switch=0
recipe_matcher_input_authority=0
route_selection=0
mir_lowering=0
mir_mutation=0
id_allocation=0
runtime_fallback=0
source_selfhost_claim=0
summary=ok
REPORT
