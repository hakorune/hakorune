#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="rust-lifecycle-mirbuilder-programjson-loop-body-return-absent-scan-only-diagnostic"

source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

FIXTURE="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-programjson-loop-body-return-absent-scan-only-diagnostic-v0.json"
CARD="$ROOT_DIR/docs/development/current/main/phases/phase-296x/3244-MIRBUILDER-PROGRAMJSON-LOOP-BODY-RETURN-ABSENT-SCAN-ONLY-DIAGNOSTIC-001.md"
TASK_ORDER="$ROOT_DIR/docs/development/current/main/design/mirbuilder-rust-to-hako-converter-task-order-ssot.md"
CURRENT_STATE="$ROOT_DIR/docs/development/current/main/CURRENT_STATE.toml"
PREV_GUARD="$ROOT_DIR/tools/checks/rust_lifecycle_mirbuilder_programjson_recipematcher_return_absent_route_release_consultation_guard.sh"
SNAPSHOT_IMPL="$ROOT_DIR/lang/src/compiler/mirbuilder/program_json_canonical_loop_facts_input_snapshot.hako"
HAKO_BIN="$ROOT_DIR/tools/bin/hako"

guard_require_command "$TAG" python3
guard_require_command "$TAG" timeout
guard_require_files "$TAG" "$FIXTURE" "$CARD" "$TASK_ORDER" "$CURRENT_STATE" "$PREV_GUARD" "$SNAPSHOT_IMPL" "$HAKO_BIN"

PREV_OUT="$(HAKO_GUARD_RESULT_CACHE_ALLOW_DIRTY=1 guard_cached_run "$TAG-prev" bash "$PREV_GUARD")"
if ! grep -q '^selected_b_defer_return_absent=1$' <<<"$PREV_OUT"; then
  printf '%s\n' "$PREV_OUT" >&2
  guard_fail "$TAG" "return-absent route-release consultation prerequisite is not green"
fi

TMP_DIR="$(mktemp -d /tmp/hakorune-return-absent-diag.XXXXXX)"
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

token = "MIRBUILDER-PROGRAMJSON-LOOP-BODY-RETURN-ABSENT-SCAN-ONLY-DIAGNOSTIC-001"
if fixture.get("kind") != "MirBuilderProgramJsonLoopBodyReturnAbsentScanOnlyDiagnosticV1":
    raise SystemExit("bad fixture kind")
if fixture.get("token") != token:
    raise SystemExit("bad fixture token")
row = fixture.get("row") or {}
if row.get("row_id") != "loop_body_if_break_if_continue_assignment_final_return_present":
    raise SystemExit("bad row id")
expected = row.get("expected_diagnostic") or {}
for key, value in {
    "loop_body_has_break": 1,
    "loop_body_has_continue": 1,
    "loop_body_has_return": 0,
    "final_top_level_return_present": 1,
    "final_top_level_return_used_for_loop_body_has_return": 0,
    "return_absent_accepted_floor": 0,
    "matcher_result_equal": 0,
    "recipe_matcher_executed": 0,
}.items():
    if expected.get(key) != value:
        raise SystemExit(f"diagnostic expectation drift: {key}")

for needle in [
    "build_return_absent_scan_only_diagnostic",
    "return_absent_scan_only_diagnostic_summary",
    "\"final_top_level_return_used_for_loop_body_has_return\" => 0",
    "\"return_absent_accepted_floor\" => 0",
    "\"matcher_result_equal\" => 0",
]:
    if needle not in snapshot_impl:
        raise SystemExit(f"snapshot impl missing: {needle}")
for forbidden in ["RecipeMatcherBox", "PlanLowerer", "IdAllocator", "runtime route switch"]:
    if forbidden in snapshot_impl:
        raise SystemExit(f"forbidden diagnostic implementation token: {forbidden}")
for needle in [
    token,
    "This is not an accepted-floor row",
    "matcher_result_equal = 0",
    "runtime_route_switch = 0",
    "programjson_runtime_route_authority = 0",
    "Source Selfhost remains unclaimed",
]:
    if needle not in card:
        raise SystemExit(f"card missing: {needle}")
for needle in [token, "MIRBUILDER-PROGRAMJSON-CANONICAL-LOOP-FACTS-FINAL-TOPLEVEL-RETURN-DECOUPLE-SNAPSHOT-BOUNDARY-001"]:
    if needle not in task_order:
        raise SystemExit(f"task-order missing: {needle}")
allowed_latest = {
    token,
    "MIRBUILDER-PROGRAMJSON-CANONICAL-LOOP-FACTS-FINAL-TOPLEVEL-RETURN-DECOUPLE-SNAPSHOT-BOUNDARY-001",
    "MIRBUILDER-PROGRAMJSON-RECIPEMATCHER-RETURN-ABSENT-ACCEPTED-FLOOR-001",
    "MIRBUILDER-PROGRAMJSON-RECIPEMATCHER-REJECT-FLOOR-EXPANSION-SELECTION-001",
    "MIRBUILDER-PROGRAMJSON-RECIPEMATCHER-UNSUPPORTED-CONDITION-OPERATOR-REJECT-ROW-001",
}
if not any(f'latest_card = "{allowed}"' in current_state for allowed in allowed_latest):
    raise SystemExit("CURRENT_STATE latest card drift")

claims = fixture.get("claims") or {}
positive = {
    "return_absent_scan_only_diagnostic",
    "loop_body_control_flow_scan_used",
    "loop_body_has_break",
    "loop_body_has_continue",
    "final_top_level_return_present",
}
for key in positive:
    if claims.get(key) != 1:
        raise SystemExit(f"missing positive claim: {key}")
for key in [
    "loop_body_has_return",
    "final_top_level_return_used_for_loop_body_has_return",
    "return_absent_green",
    "return_absent_accepted_floor",
    "matcher_result_equal",
    "recipe_matcher_accepted_floor",
    "programjson_runtime_route_authority",
    "runtime_route_switch",
    "recipe_matcher_input_authority",
    "route_selection",
    "mir_lowering",
    "mir_mutation",
    "id_allocation",
    "runtime_fallback",
    "source_selfhost_claim",
]:
    if claims.get(key) != 0:
        raise SystemExit(f"forbidden claim drift: {key}")

program_json = json.dumps(row["program_json"], separators=(",", ":"), ensure_ascii=False)
app.write_text("\n".join([
    "using lang.compiler.mirbuilder.program_json_canonical_loop_facts_input_snapshot as ProgramJsonCanonicalLoopFactsInputSnapshotBox",
    "",
    "static box Main {",
    "  main() {",
    "    local diag = ProgramJsonCanonicalLoopFactsInputSnapshotBox.build_return_absent_scan_only_diagnostic(" + json.dumps(program_json) + ")",
    "    print(ProgramJsonCanonicalLoopFactsInputSnapshotBox.return_absent_scan_only_diagnostic_summary(diag))",
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
symbol = "ProgramJsonCanonicalLoopFactsInputSnapshotBox.build_return_absent_scan_only_diagnostic/1"
matches = [row for row in main_rows if row.get("symbol") == symbol]
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
if not line.startswith("snapshot_kind=ProgramJsonReturnAbsentScanOnlyDiagnosticV1;"):
    raise SystemExit(f"bad diagnostic summary: {line}")
expected = fixture["row"]["expected_diagnostic"]
checks = {
    "ok": expected["ok"],
    "loop_body_has_break": expected["loop_body_has_break"],
    "loop_body_has_continue": expected["loop_body_has_continue"],
    "loop_body_has_return": expected["loop_body_has_return"],
    "final_top_level_return_present": expected["final_top_level_return_present"],
    "final_top_level_return_used_for_loop_body_has_return": expected["final_top_level_return_used_for_loop_body_has_return"],
    "return_absent_scan_only_diagnostic": expected["return_absent_scan_only_diagnostic"],
    "return_absent_accepted_floor": expected["return_absent_accepted_floor"],
    "matcher_result_equal": expected["matcher_result_equal"],
    "recipe_matcher_executed": expected["recipe_matcher_executed"],
}
for key, value in checks.items():
    needle = f";{key}={value}"
    if needle not in line:
        raise SystemExit(f"summary missing {needle}: {line}")
PY

cat <<'REPORT'
output_contract=rust-lifecycle-mirbuilder-programjson-loop-body-return-absent-scan-only-diagnostic-v0
token=MIRBUILDER-PROGRAMJSON-LOOP-BODY-RETURN-ABSENT-SCAN-ONLY-DIAGNOSTIC-001
row_id=loop_body_if_break_if_continue_assignment_final_return_present
return_absent_scan_only_diagnostic=1
loop_body_control_flow_scan_used=1
loop_body_has_break=1
loop_body_has_continue=1
loop_body_has_return=0
final_top_level_return_present=1
final_top_level_return_used_for_loop_body_has_return=0
return_absent_green=0
return_absent_accepted_floor=0
matcher_result_equal=0
recipe_matcher_accepted_floor=0
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
