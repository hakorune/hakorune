#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="rust-lifecycle-mirbuilder-programjson-recipematcher-nested-loop-reject-boundary"

source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

FIXTURE="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-programjson-recipematcher-nested-loop-reject-boundary-v0.json"
CARD="$ROOT_DIR/docs/development/current/main/phases/phase-296x/3253-MIRBUILDER-PROGRAMJSON-RECIPEMATCHER-NESTED-LOOP-REJECT-BOUNDARY-001.md"
TASK_ORDER="$ROOT_DIR/docs/development/current/main/design/mirbuilder-rust-to-hako-converter-task-order-ssot.md"
CURRENT_STATE="$ROOT_DIR/docs/development/current/main/CURRENT_STATE.toml"
PREV_GUARD="$ROOT_DIR/tools/checks/rust_lifecycle_mirbuilder_programjson_recipematcher_nested_loop_decision_row_consultation_guard.sh"
SNAPSHOT_IMPL="$ROOT_DIR/lang/src/compiler/mirbuilder/program_json_canonical_loop_facts_input_snapshot.hako"
MATCHER_IMPL="$ROOT_DIR/lang/src/compiler/mirbuilder/program_json_recipematcher_execution_boundary.hako"
HAKO_BIN="$ROOT_DIR/tools/bin/hako"

guard_require_command "$TAG" python3
guard_require_command "$TAG" timeout
guard_require_files "$TAG" "$FIXTURE" "$CARD" "$TASK_ORDER" "$CURRENT_STATE" "$PREV_GUARD" "$SNAPSHOT_IMPL" "$MATCHER_IMPL" "$HAKO_BIN"

PREV_OUT="$(HAKO_GUARD_RESULT_CACHE_ALLOW_DIRTY=1 guard_cached_run "$TAG-prev" bash "$PREV_GUARD")"
if ! grep -q '^selected_nested_loop_reject_boundary=1$' <<<"$PREV_OUT"; then
  printf '%s\n' "$PREV_OUT" >&2
  guard_fail "$TAG" "nested-loop decision prerequisite is not green"
fi

TMP_DIR="$(mktemp -d /tmp/hakorune-nested-loop-reject-boundary.XXXXXX)"
cleanup() { rm -rf "$TMP_DIR" >/dev/null 2>&1 || true; }
trap cleanup EXIT

APP="$TMP_DIR/probe.hako"
EXE="$TMP_DIR/probe.exe"
MIR_JSON="$TMP_DIR/probe.mir.json"
BUILD_LOG="$TMP_DIR/build.log"
RUN_OUT="$TMP_DIR/run.out"
RUN_FILTERED="$TMP_DIR/run.filtered"
RUN_ERR="$TMP_DIR/run.err"

python3 - "$FIXTURE" "$CARD" "$TASK_ORDER" "$CURRENT_STATE" "$SNAPSHOT_IMPL" "$MATCHER_IMPL" "$APP" <<'PY'
import json
import sys
from pathlib import Path

fixture_path, card_path, task_order_path, current_state_path, snapshot_path, matcher_path, app_path = sys.argv[1:]
fixture = json.loads(Path(fixture_path).read_text(encoding="utf-8"))
card = Path(card_path).read_text(encoding="utf-8")
task_order = Path(task_order_path).read_text(encoding="utf-8")
current_state = Path(current_state_path).read_text(encoding="utf-8")
snapshot_impl = Path(snapshot_path).read_text(encoding="utf-8")
matcher_impl = Path(matcher_path).read_text(encoding="utf-8")
app = Path(app_path)

token = "MIRBUILDER-PROGRAMJSON-RECIPEMATCHER-NESTED-LOOP-REJECT-BOUNDARY-001"
if fixture.get("kind") != "MirBuilderProgramJsonRecipeMatcherNestedLoopRejectBoundaryV1":
    raise SystemExit("bad fixture kind")
if fixture.get("token") != token:
    raise SystemExit("bad fixture token")
row = fixture.get("row") or {}
if row.get("row_id") != "nested_loop_reject_boundary":
    raise SystemExit("bad row id")
if row.get("input_snapshot", {}).get("has_nested_loop") != 1:
    raise SystemExit("input snapshot expectation must require has_nested_loop=1")
if row.get("expected_matcher_result", {}).get("reason") != "nested_loop_present":
    raise SystemExit("matcher reason expectation drift")

for needle in [
    '"has_nested_loop" => me._loop_has_type(program_json, loop_body, "Loop")',
    '";has_nested_loop="',
]:
    if needle not in snapshot_impl:
        raise SystemExit(f"snapshot impl missing: {needle}")
for needle in [
    'if me._i(snapshot, "has_nested_loop") == 1',
    'return me._err("nested_loop_present")',
    'if reason == "nested_loop_present" { return 5 }',
    'if code == 5 { return "nested_loop_present" }',
]:
    if needle not in matcher_impl:
        raise SystemExit(f"matcher impl missing: {needle}")

claims = fixture.get("claims") or {}
for key in ["nested_loop_reject_boundary_green", "programjson_shadow_checked"]:
    if claims.get(key) != 1:
        raise SystemExit(f"missing positive claim: {key}")
for key, value in claims.items():
    if key in {"nested_loop_reject_boundary_green", "programjson_shadow_checked"}:
        continue
    if value != 0:
        raise SystemExit(f"forbidden claim drift: {key}")

for needle in [
    token,
    "nested_loop_present",
    "nested_loop_accepted_floor = 0",
    "runtime_route_switch = 0",
    "programjson_runtime_route_authority = 0",
    "Source Selfhost remains unclaimed",
]:
    if needle not in card:
        raise SystemExit(f"card missing: {needle}")
for needle in [token, "MIRBUILDER-PROGRAMJSON-RECIPEMATCHER-REJECT-FLOOR-EXPANSION-SELECTION-001"]:
    if needle not in task_order:
        raise SystemExit(f"task-order missing: {needle}")
allowed_latest = {
    token,
    "MIRBUILDER-PROGRAMJSON-RECIPEMATCHER-REJECT-FLOOR-EXPANSION-SELECTION-001",
}
if not any(f'latest_card = "{allowed}"' in current_state for allowed in allowed_latest):
    raise SystemExit("CURRENT_STATE latest card drift")

app.write_text("\n".join([
    "using lang.compiler.mirbuilder.program_json_recipematcher_execution_boundary as ProgramJsonRecipeMatcherExecutionBoundaryBox",
    "",
    "static box Main {",
    "  main() {",
    "    local snapshot = %{",
    "      \"ok\" => 1,",
    "      \"reason_code\" => 0,",
    "      \"matcher_input_present\" => 1,",
    "      \"readonly\" => 1,",
    "      \"has_nested_loop\" => 1,",
    "      \"exit_has_break\" => 1,",
    "      \"exit_has_continue\" => 0,",
    "      \"exit_has_return\" => 1",
    "    }",
    "    local matched = ProgramJsonRecipeMatcherExecutionBoundaryBox.match_snapshot(snapshot)",
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
main = next((fn for fn in data.get("functions", []) if fn.get("name") == "main"), None)
if main is None:
    raise SystemExit("main function missing")
rows = []
for key in ("global_call_routes", "lowering_plan"):
    value = (main.get("metadata") or {}).get(key) or []
    rows.extend(row for row in value if isinstance(row, dict))
for symbol in [
    "ProgramJsonRecipeMatcherExecutionBoundaryBox.match_snapshot/1",
]:
    matches = [row for row in rows if row.get("symbol") == symbol]
    if not matches:
        raise SystemExit(f"main missing call: {symbol}")
    for row in matches:
        if row.get("tier") != "DirectAbi" or row.get("return_shape") != "map_handle":
            raise SystemExit(f"{symbol} is not DirectAbi/map_handle: {row}")
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
grep -v '^Result: 0$' "$RUN_OUT" | grep -v '^\[freeze:contract\]' >"$RUN_FILTERED" || true

python3 - "$RUN_FILTERED" <<'PY'
import sys
from pathlib import Path

lines = [line.strip() for line in Path(sys.argv[1]).read_text(encoding="utf-8").splitlines() if line.strip()]
if len(lines) != 1:
    raise SystemExit(f"expected one output line, got {len(lines)}: {lines}")
joined = "\n".join(lines)
for needle in [
    "match:snapshot_kind=ProgramJsonRecipeMatcherExecutionBoundaryResultV1;ok=0;reason=nested_loop_present",
    ";matcher_input_consumed=0",
    ";matched=0",
    ";contract_kind=Unsupported",
    ";full_recipe_matcher_execution=0",
    ";route_selection=0",
    ";runtime_route_switch=0",
]:
    if needle not in joined:
        raise SystemExit(f"output missing {needle}: {joined}")
PY

cat <<'REPORT'
output_contract=rust-lifecycle-mirbuilder-programjson-recipematcher-nested-loop-reject-boundary-v0
token=MIRBUILDER-PROGRAMJSON-RECIPEMATCHER-NESTED-LOOP-REJECT-BOUNDARY-001
row_id=nested_loop_reject_boundary
nested_loop_reject_boundary_green=1
nested_loop_accepted_floor=0
snapshot_has_nested_loop=1
matcher_reason=nested_loop_present
matched=0
contract_kind=Unsupported
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
