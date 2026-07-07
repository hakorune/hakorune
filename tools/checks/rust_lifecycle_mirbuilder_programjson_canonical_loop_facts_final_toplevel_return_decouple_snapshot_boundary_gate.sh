#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="rust-lifecycle-mirbuilder-programjson-canonical-loop-facts-final-toplevel-return-decouple-snapshot-boundary"

source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

FIXTURE="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-programjson-canonical-loop-facts-final-toplevel-return-decouple-snapshot-boundary-v0.json"
CARD="$ROOT_DIR/docs/development/current/main/phases/phase-296x/3245-MIRBUILDER-PROGRAMJSON-CANONICAL-LOOP-FACTS-FINAL-TOPLEVEL-RETURN-DECOUPLE-SNAPSHOT-BOUNDARY-001.md"
TASK_ORDER="$ROOT_DIR/docs/development/current/main/design/mirbuilder-rust-to-hako-converter-task-order-ssot.md"
CURRENT_STATE="$ROOT_DIR/docs/development/current/main/CURRENT_STATE.toml"
PREV_GUARD="$ROOT_DIR/tools/checks/rust_lifecycle_mirbuilder_programjson_loop_body_return_absent_scan_only_diagnostic_gate.sh"
SNAPSHOT_IMPL="$ROOT_DIR/lang/src/compiler/mirbuilder/program_json_canonical_loop_facts_input_snapshot.hako"
HAKO_BIN="$ROOT_DIR/tools/bin/hako"

guard_require_command "$TAG" python3
guard_require_command "$TAG" timeout
guard_require_files "$TAG" "$FIXTURE" "$CARD" "$TASK_ORDER" "$CURRENT_STATE" "$PREV_GUARD" "$SNAPSHOT_IMPL" "$HAKO_BIN"

PREV_OUT="$(HAKO_GUARD_RESULT_CACHE_ALLOW_DIRTY=1 guard_cached_run "$TAG-prev" bash "$PREV_GUARD")"
if ! grep -q '^return_absent_scan_only_diagnostic=1$' <<<"$PREV_OUT"; then
  printf '%s\n' "$PREV_OUT" >&2
  guard_fail "$TAG" "return-absent scan-only prerequisite is not green"
fi

TMP_DIR="$(mktemp -d /tmp/hakorune-final-return-decouple.XXXXXX)"
cleanup() { rm -rf "$TMP_DIR" >/dev/null 2>&1 || true; }
trap cleanup EXIT

APP="$TMP_DIR/probe.hako"
EXE="$TMP_DIR/probe.exe"
MIR_JSON="$TMP_DIR/probe.mir.json"
BUILD_LOG="$TMP_DIR/build.log"
RUN_OUT="$TMP_DIR/run.out"
RUN_FILTERED="$TMP_DIR/run.filtered"
RUN_ERR="$TMP_DIR/run.err"

python3 - "$FIXTURE" "$CARD" "$TASK_ORDER" "$CURRENT_STATE" "$SNAPSHOT_IMPL" "$APP" <<'PY'
import json
import sys
from pathlib import Path

fixture_path, card_path, task_order_path, current_state_path, snapshot_path, app_path = sys.argv[1:]
fixture = json.loads(Path(fixture_path).read_text(encoding="utf-8"))
card = Path(card_path).read_text(encoding="utf-8")
task_order = Path(task_order_path).read_text(encoding="utf-8")
current_state = Path(current_state_path).read_text(encoding="utf-8")
snapshot_impl = Path(snapshot_path).read_text(encoding="utf-8")
app = Path(app_path)

token = "MIRBUILDER-PROGRAMJSON-CANONICAL-LOOP-FACTS-FINAL-TOPLEVEL-RETURN-DECOUPLE-SNAPSHOT-BOUNDARY-001"
next_token = "MIRBUILDER-PROGRAMJSON-RECIPEMATCHER-RETURN-ABSENT-ACCEPTED-FLOOR-001"
if fixture.get("kind") != "MirBuilderProgramJsonCanonicalLoopFactsFinalTopLevelReturnDecoupleSnapshotBoundaryV1":
    raise SystemExit("bad fixture kind")
if fixture.get("token") != token:
    raise SystemExit("bad fixture token")
state = fixture.get("input_state") or {}
if state.get("producer_final_return_requirement_removed") is not False:
    raise SystemExit("producer final-return requirement claim drift")
row = fixture.get("row") or {}
expected = row.get("expected_snapshot") or {}
for key, value in {
    "exit_has_break": 1,
    "exit_has_continue": 1,
    "exit_has_return": 1,
    "final_top_level_return_present": 1,
    "final_top_level_return_used_for_loop_body_has_return": 0,
    "return_absent_accepted_floor": 0,
    "recipe_matcher_executed": 0,
}.items():
    if expected.get(key) != value:
        raise SystemExit(f"snapshot expectation drift: {key}")

for needle in [
    "\"final_top_level_return_present\" => final_return_present",
    "\"final_top_level_return_used_for_loop_body_has_return\" => 0",
    "local has_return = me._loop_has_type(program_json, loop_body, \"Return\")",
    "_final_top_level_return_present",
]:
    if needle not in snapshot_impl:
        raise SystemExit(f"snapshot impl missing: {needle}")
for forbidden in [
    "if third < 0 { return me._err(\"missing_final_return\") }",
    "if me._token_eq(me._node_type(program_json, third), \"Return\") != 1 { return me._err(\"final_stmt_not_return\") }",
]:
    if forbidden in snapshot_impl:
        raise SystemExit(f"snapshot boundary still directly rejects final return: {forbidden}")
for needle in [
    token,
    "producer_final_return_requirement_removed = 0",
    "return_absent_accepted_floor = 0",
    "runtime_route_switch = 0",
    "programjson_runtime_route_authority = 0",
    "Source Selfhost remains unclaimed",
]:
    if needle not in card:
        raise SystemExit(f"card missing: {needle}")
for needle in [token, next_token, "snapshot-boundary decouple"]:
    if needle not in task_order:
        raise SystemExit(f"task-order missing: {needle}")
allowed_latest = {
    token,
    next_token,
}
if not any(f'latest_card = "{allowed}"' in current_state for allowed in allowed_latest):
    raise SystemExit("CURRENT_STATE latest card drift")

claims = fixture.get("claims") or {}
positive = {
    "final_top_level_return_decoupled_from_loop_body_exit_usage",
    "snapshot_publishes_final_top_level_return_present",
    "loop_body_exit_has_return_uses_loop_body_scan_only",
    "snapshot_boundary_no_direct_final_return_reject",
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
    "",
    "static box Main {",
    "  main() {",
    "    local snapshot = ProgramJsonCanonicalLoopFactsInputSnapshotBox.build_snapshot(" + json.dumps(program_json) + ")",
    "    print(ProgramJsonCanonicalLoopFactsInputSnapshotBox.snapshot_summary(snapshot))",
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

symbol = "ProgramJsonCanonicalLoopFactsInputSnapshotBox.build_snapshot/1"
matches = [row for row in rows(main) if row.get("symbol") == symbol]
if not matches:
    raise SystemExit(f"main missing call: {symbol}")
for row in matches:
    if row.get("tier") != "DirectAbi" or row.get("return_shape") != "map_handle":
        raise SystemExit(f"{symbol} is not DirectAbi/map_handle: {row}")
for fn in functions:
    name = str(fn.get("name", ""))
    if not name.startswith("ProgramJsonCanonicalLoopFactsInputSnapshotBox."):
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
import json
import sys
from pathlib import Path

fixture = json.loads(Path(sys.argv[1]).read_text(encoding="utf-8"))
lines = [line.strip() for line in Path(sys.argv[2]).read_text(encoding="utf-8").splitlines() if line.strip()]
if len(lines) != 1:
    raise SystemExit(f"expected one output line, got {len(lines)}: {lines}")
line = lines[0]
if not line.startswith("snapshot_kind=ProgramJsonCanonicalLoopFactsInputSnapshotV1;"):
    raise SystemExit(f"bad snapshot summary: {line}")
expected = fixture["row"]["expected_snapshot"]
for key, value in {
    "ok": expected["ok"],
    "matcher_input_present": expected["matcher_input_present"],
    "exit_has_break": expected["exit_has_break"],
    "exit_has_continue": expected["exit_has_continue"],
    "exit_has_return": expected["exit_has_return"],
    "final_top_level_return_present": expected["final_top_level_return_present"],
    "final_top_level_return_used_for_loop_body_has_return": expected["final_top_level_return_used_for_loop_body_has_return"],
    "recipe_matcher_executed": expected["recipe_matcher_executed"],
}.items():
    needle = f";{key}={value}"
    if needle not in line:
        raise SystemExit(f"summary missing {needle}: {line}")
PY

cat <<'REPORT'
output_contract=rust-lifecycle-mirbuilder-programjson-canonical-loop-facts-final-toplevel-return-decouple-snapshot-boundary-v0
token=MIRBUILDER-PROGRAMJSON-CANONICAL-LOOP-FACTS-FINAL-TOPLEVEL-RETURN-DECOUPLE-SNAPSHOT-BOUNDARY-001
row_id=canonical_loop_facts_final_return_present_decoupled
final_top_level_return_decoupled_from_loop_body_exit_usage=1
snapshot_publishes_final_top_level_return_present=1
loop_body_exit_has_return_uses_loop_body_scan_only=1
snapshot_boundary_no_direct_final_return_reject=1
final_top_level_return_present=1
final_top_level_return_used_for_loop_body_has_return=0
return_absent_accepted_floor=0
matcher_result_equal=0
recipe_matcher_accepted_floor=0
producer_final_return_requirement_removed=0
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
