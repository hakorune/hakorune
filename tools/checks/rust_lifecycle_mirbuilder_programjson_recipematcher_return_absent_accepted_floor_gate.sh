#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="rust-lifecycle-mirbuilder-programjson-recipematcher-return-absent-accepted-floor"

source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

FIXTURE="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-programjson-recipematcher-return-absent-accepted-floor-v0.json"
CARD="$ROOT_DIR/docs/development/current/main/phases/phase-296x/3246-MIRBUILDER-PROGRAMJSON-RECIPEMATCHER-RETURN-ABSENT-ACCEPTED-FLOOR-001.md"
TASK_ORDER="$ROOT_DIR/docs/development/current/main/design/mirbuilder-rust-to-hako-converter-task-order-ssot.md"
CURRENT_STATE="$ROOT_DIR/docs/development/current/main/CURRENT_STATE.toml"
SNAPSHOT_IMPL="$ROOT_DIR/lang/src/compiler/mirbuilder/program_json_canonical_loop_facts_input_snapshot.hako"
LOOP_HANDLER="$ROOT_DIR/lang/src/compiler/mirbuilder/stmt_handlers/loop_stmt_handler.hako"
MATCHER_IMPL="$ROOT_DIR/lang/src/compiler/mirbuilder/program_json_recipematcher_execution_boundary.hako"
HAKO_BIN="$ROOT_DIR/tools/bin/hako"

guard_require_command "$TAG" python3
guard_require_command "$TAG" timeout
guard_require_files "$TAG" "$FIXTURE" "$CARD" "$TASK_ORDER" "$CURRENT_STATE" "$SNAPSHOT_IMPL" "$LOOP_HANDLER" "$MATCHER_IMPL" "$HAKO_BIN"

TMP_DIR="$(mktemp -d /tmp/hakorune-return-absent-floor.XXXXXX)"
cleanup() { rm -rf "$TMP_DIR" >/dev/null 2>&1 || true; }
trap cleanup EXIT

APP="$TMP_DIR/probe.hako"
EXE="$TMP_DIR/probe.exe"
MIR_JSON="$TMP_DIR/probe.mir.json"
BUILD_LOG="$TMP_DIR/build.log"
RUN_OUT="$TMP_DIR/run.out"
RUN_FILTERED="$TMP_DIR/run.filtered"
RUN_ERR="$TMP_DIR/run.err"

python3 - "$FIXTURE" "$CARD" "$TASK_ORDER" "$CURRENT_STATE" "$SNAPSHOT_IMPL" "$LOOP_HANDLER" "$MATCHER_IMPL" "$APP" <<'PY'
import json
import sys
from pathlib import Path

fixture_path, card_path, task_order_path, current_state_path, snapshot_path, loop_path, matcher_path, app_path = sys.argv[1:]
fixture = json.loads(Path(fixture_path).read_text(encoding="utf-8"))
card = Path(card_path).read_text(encoding="utf-8")
task_order = Path(task_order_path).read_text(encoding="utf-8")
current_state = Path(current_state_path).read_text(encoding="utf-8")
snapshot_impl = Path(snapshot_path).read_text(encoding="utf-8")
loop_impl = Path(loop_path).read_text(encoding="utf-8")
matcher_impl = Path(matcher_path).read_text(encoding="utf-8")
app = Path(app_path)

token = "MIRBUILDER-PROGRAMJSON-RECIPEMATCHER-RETURN-ABSENT-ACCEPTED-FLOOR-001"
if fixture.get("kind") != "MirBuilderProgramJsonRecipeMatcherReturnAbsentAcceptedFloorV1":
    raise SystemExit("bad fixture kind")
if fixture.get("token") != token:
    raise SystemExit("bad fixture token")
row = fixture.get("row") or {}
expected = {"matched": 1, "contract_kind": "LoopWithExit", "has_break": 1, "has_continue": 1, "has_return": 0}
if row.get("rust_astnode_route_oracle") != expected or row.get("programjson_route_expected") != expected:
    raise SystemExit("matcher oracle drift")
snapshot = row.get("expected_snapshot") or {}
for key, value in {
    "exit_has_break": 1,
    "exit_has_continue": 1,
    "exit_has_return": 0,
    "final_top_level_return_present": 1,
    "final_top_level_return_used_for_loop_body_has_return": 0,
}.items():
    if snapshot.get(key) != value:
        raise SystemExit(f"snapshot expectation drift: {key}")

for needle in [
    "if_two_exit_assignment",
    "second_exit_kind",
    "RecipeItemBox.exit_item(BoxHelpers.map_get(body_out, \"second_exit_kind\"), %{})",
    "RecipeItemBox.seq([first_item, second_item, body_stmt])",
]:
    if needle not in loop_impl:
        raise SystemExit(f"loop handler missing: {needle}")
for needle in [
    "local has_return = me._loop_has_type(program_json, loop_body, \"Return\")",
    "\"final_top_level_return_used_for_loop_body_has_return\" => 0",
]:
    if needle not in snapshot_impl:
        raise SystemExit(f"snapshot impl missing: {needle}")
for forbidden in ["PlanLowerer", "IdAllocator", "runtime route switch"]:
    if forbidden in loop_impl or forbidden in snapshot_impl or forbidden in matcher_impl:
        raise SystemExit(f"forbidden implementation token: {forbidden}")
for needle in [token, "route_release_authority = 0", "runtime_route_switch = 0", "Source Selfhost remains unclaimed"]:
    if needle not in card:
        raise SystemExit(f"card missing: {needle}")
for needle in [token, "MIRBUILDER-PROGRAMJSON-RECIPEMATCHER-REJECT-FLOOR-SELECTION-001"]:
    if needle not in task_order:
        raise SystemExit(f"task-order missing: {needle}")
allowed_latest = {
    token,
    "MIRBUILDER-PROGRAMJSON-RECIPEMATCHER-REJECT-FLOOR-SELECTION-001",
    "MIRBUILDER-PROGRAMJSON-RECIPEMATCHER-MISSING-VERIFIED-RECIPE-REJECT-ROW-001",
    "MIRBUILDER-PROGRAMJSON-RECIPEMATCHER-ROUTE-CONSUMED-FIELD-FLOOR-SELECTION-001",
    "MIRBUILDER-PROGRAMJSON-RECIPEMATCHER-ROUTE-CONSUMED-FIELD-FLOOR-PARITY-001",
    "MIRBUILDER-PROGRAMJSON-RECIPEMATCHER-AUTHORITY-SWITCH-READINESS-CONSULTATION-001",
}
if not any(f'latest_card = "{allowed}"' in current_state for allowed in allowed_latest):
    raise SystemExit("CURRENT_STATE latest card drift")

claims = fixture.get("claims") or {}
positive = {
    "return_absent_accepted_floor",
    "loop_body_return_absent",
    "matcher_result_equal",
    "matched",
    "contract_kind_loop_with_exit",
    "has_return_false",
    "programjson_shadow_checked",
    "runtime_authority_remains_rust_astnode",
}
for key in positive:
    if claims.get(key) != 1:
        raise SystemExit(f"missing positive claim: {key}")
for key, value in claims.items():
    if key in positive:
        continue
    if value != 0:
        raise SystemExit(f"forbidden claim drift: {key}")

program_json = json.dumps(row["program_json"], separators=(",", ":"), ensure_ascii=False)
app.write_text("\n".join([
    "using lang.compiler.mirbuilder.program_json_canonical_loop_facts_input_snapshot as ProgramJsonCanonicalLoopFactsInputSnapshotBox",
    "using lang.compiler.mirbuilder.program_json_recipematcher_execution_boundary as ProgramJsonRecipeMatcherExecutionBoundaryBox",
    "",
    "static box Main {",
    "  main() {",
    "    local snapshot = ProgramJsonCanonicalLoopFactsInputSnapshotBox.build_snapshot(" + json.dumps(program_json) + ")",
    "    local matched = ProgramJsonRecipeMatcherExecutionBoundaryBox.match_snapshot(snapshot)",
    "    print(\"snapshot:\" + ProgramJsonCanonicalLoopFactsInputSnapshotBox.snapshot_summary(snapshot))",
    "    print(\"match:\" + ProgramJsonRecipeMatcherExecutionBoundaryBox.match_summary(matched))",
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
functions = data.get("functions") or []
main = next((fn for fn in functions if fn.get("name") == "main"), None)
if main is None:
    raise SystemExit("main function missing")

def rows(fn):
    meta = fn.get("metadata") or {}
    out = []
    for key in ("global_call_routes", "lowering_plan"):
        value = meta.get(key) or []
        if isinstance(value, list):
            out.extend(row for row in value if isinstance(row, dict))
    return out

main_rows = rows(main)
for symbol in [
    "ProgramJsonCanonicalLoopFactsInputSnapshotBox.build_snapshot/1",
    "ProgramJsonRecipeMatcherExecutionBoundaryBox.match_snapshot/1",
]:
    matches = [row for row in main_rows if row.get("symbol") == symbol]
    if not matches:
        raise SystemExit(f"main missing call: {symbol}")
    for row in matches:
        if row.get("tier") != "DirectAbi" or row.get("return_shape") != "map_handle":
            raise SystemExit(f"{symbol} is not DirectAbi/map_handle: {row}")
for fn in functions:
    name = str(fn.get("name", ""))
    if not (
        name.startswith("ProgramJsonCanonicalLoopFactsInputSnapshotBox.")
        or name.startswith("ProgramJsonRecipeMatcherExecutionBoundaryBox.")
        or name.startswith("LoopStmtHandler.")
    ):
        continue
    bad = [row for row in rows(fn) if row.get("tier") == "Unsupported" or row.get("reason")]
    if bad:
        raise SystemExit(f"{name} has unsupported routes: {bad[:5]}")
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
import sys
from pathlib import Path

lines = [line.strip() for line in Path(sys.argv[2]).read_text(encoding="utf-8").splitlines() if line.strip()]
if len(lines) != 2:
    raise SystemExit(f"expected two output lines, got {len(lines)}: {lines}")
joined = "\n".join(lines)
for needle in [
    "snapshot:snapshot_kind=ProgramJsonCanonicalLoopFactsInputSnapshotV1;ok=1",
    ";exit_has_break=1",
    ";exit_has_continue=1",
    ";exit_has_return=0",
    ";final_top_level_return_present=1",
    ";final_top_level_return_used_for_loop_body_has_return=0",
    ";recipe_matcher_executed=0",
    "match:snapshot_kind=ProgramJsonRecipeMatcherExecutionBoundaryResultV1;ok=1",
    ";matched=1",
    ";contract_kind=LoopWithExit",
    ";has_break=1",
    ";has_continue=1",
    ";has_return=0",
    ";full_recipe_matcher_execution=0",
    ";runtime_route_switch=0",
]:
    if needle not in joined:
        raise SystemExit(f"output missing {needle}: {joined}")
PY

cat <<'REPORT'
output_contract=rust-lifecycle-mirbuilder-programjson-recipematcher-return-absent-accepted-floor-v0
token=MIRBUILDER-PROGRAMJSON-RECIPEMATCHER-RETURN-ABSENT-ACCEPTED-FLOOR-001
row_id=local_loop_body_if_break_if_continue_assignment_return_absent
return_absent_accepted_floor=1
loop_body_return_absent=1
matcher_result_equal=1
matched=1
contract_kind=LoopWithExit
has_break=1
has_continue=1
has_return=0
programjson_shadow_checked=1
runtime_authority=rust_astnode
programjson_runtime_route_authority=0
runtime_route_switch=0
recipe_matcher_input_authority=0
route_release_authority=0
route_selection=0
mir_lowering=0
mir_mutation=0
id_allocation=0
runtime_fallback=0
source_selfhost_claim=0
summary=ok
REPORT
