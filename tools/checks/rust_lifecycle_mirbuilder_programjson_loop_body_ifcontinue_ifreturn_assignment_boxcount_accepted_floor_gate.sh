#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="rust-lifecycle-mirbuilder-programjson-loop-body-ifcontinue-ifreturn-assignment-boxcount-accepted-floor"

source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

FIXTURE="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-programjson-loop-body-ifcontinue-ifreturn-assignment-boxcount-accepted-floor-v0.json"
CARD="$ROOT_DIR/docs/development/current/main/phases/phase-296x/3239-MIRBUILDER-PROGRAMJSON-LOOP-BODY-IFCONTINUE-IFRETURN-ASSIGNMENT-BOXCOUNT-ACCEPTED-FLOOR-001.md"
TASK_ORDER="$ROOT_DIR/docs/development/current/main/design/mirbuilder-rust-to-hako-converter-task-order-ssot.md"
CURRENT_STATE="$ROOT_DIR/docs/development/current/main/CURRENT_STATE.toml"
DESIGN_GUARD="$ROOT_DIR/tools/checks/rust_lifecycle_mirbuilder_programjson_recipematcher_continue_present_row_shape_design_stop_guard.sh"
SNAPSHOT_IMPL="$ROOT_DIR/lang/src/compiler/mirbuilder/program_json_canonical_loop_facts_input_snapshot.hako"
LOOP_HANDLER="$ROOT_DIR/lang/src/compiler/mirbuilder/stmt_handlers/loop_stmt_handler.hako"
MATCHER_IMPL="$ROOT_DIR/lang/src/compiler/mirbuilder/program_json_recipematcher_execution_boundary.hako"
HAKO_BIN="$ROOT_DIR/tools/bin/hako"

guard_require_command "$TAG" python3
guard_require_command "$TAG" timeout
guard_require_files "$TAG" "$FIXTURE" "$CARD" "$TASK_ORDER" "$CURRENT_STATE" "$DESIGN_GUARD" "$SNAPSHOT_IMPL" "$LOOP_HANDLER" "$MATCHER_IMPL" "$HAKO_BIN"

DESIGN_OUT="$(HAKO_GUARD_RESULT_CACHE_ALLOW_DIRTY=1 guard_cached_run "$TAG-design" bash "$DESIGN_GUARD")"
if ! grep -q '^design_stop=1$' <<<"$DESIGN_OUT"; then
  printf '%s\n' "$DESIGN_OUT" >&2
  guard_fail "$TAG" "continue-present design-stop prerequisite is not green"
fi

TMP_DIR="$(mktemp -d /tmp/hakorune-ifcontinue-ifreturn.XXXXXX)"
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

token = "MIRBUILDER-PROGRAMJSON-LOOP-BODY-IFCONTINUE-IFRETURN-ASSIGNMENT-BOXCOUNT-ACCEPTED-FLOOR-001"
if fixture.get("kind") != "MirBuilderProgramJsonLoopBodyIfContinueIfReturnAssignmentBoxcountAcceptedFloorV1":
    raise SystemExit("bad fixture kind")
if fixture.get("token") != token:
    raise SystemExit("bad fixture token")
row = fixture.get("row") or {}
if row.get("row_id") != "local_loop_body_if_continue_if_return_assignment":
    raise SystemExit("bad row id")
expected = {"matched": 1, "contract_kind": "LoopWithExit", "has_break": 0, "has_continue": 1, "has_return": 1}
if row.get("rust_astnode_route_oracle") != expected or row.get("programjson_route_expected") != expected:
    raise SystemExit("matcher oracle drift")
snapshot = row.get("expected_snapshot") or {}
for key, value in {
    "exit_has_continue": 1,
    "exit_has_return": 1,
    "exit_has_break": 0,
    "loop_cond_continue_with_return_present": 1,
    "loop_cond_return_in_body_present": 0,
}.items():
    if snapshot.get(key) != value:
        raise SystemExit(f"snapshot expectation drift: {key}")

for needle in [
    "if_exit_if_return_assignment",
    "_read_loop_if_then_continue",
    "RecipeItemBox.exit_item(BoxHelpers.map_get(body_out, \"first_exit_kind\"), %{})",
    "RecipeItemBox.seq([first_exit_item, if_item, body_stmt])",
]:
    if needle not in loop_impl:
        raise SystemExit(f"loop handler missing: {needle}")
for needle in [
    "local has_return = me._loop_has_type(program_json, loop_body, \"Return\")",
    "\"exit_has_return\" => has_return",
    "\"loop_cond_continue_with_return_present\" => has_continue * has_return",
]:
    if needle not in snapshot_impl:
        raise SystemExit(f"snapshot impl missing: {needle}")
for forbidden in ["RecipeComposer", "PlanLowerer", "IdAllocator", "runtime route switch"]:
    if forbidden in loop_impl or forbidden in snapshot_impl or forbidden in matcher_impl:
        raise SystemExit(f"forbidden implementation token: {forbidden}")
for needle in [token, "general_loop_body_sequence_owner = 0", "runtime_route_switch = 0"]:
    if needle not in card:
        raise SystemExit(f"card missing: {needle}")
for needle in [token, "MIRBUILDER-PROGRAMJSON-RECIPEMATCHER-CONTINUE-PRESENT-ROW-SHAPE-DESIGN-STOP-001"]:
    if needle not in task_order:
        raise SystemExit(f"task-order missing: {needle}")
allowed_latest = {
    token,
    "MIRBUILDER-PROGRAMJSON-RECIPEMATCHER-BREAK-PRESENT-VERIFIED-RECIPE-SUPPORT-001",
    "MIRBUILDER-PROGRAMJSON-RECIPEMATCHER-BREAK-CONTINUE-PRESENT-VERIFIED-RECIPE-SUPPORT-001",
    "MIRBUILDER-PROGRAMJSON-RECIPEMATCHER-RETURN-ABSENT-DECISION-ROW-001",
    "MIRBUILDER-PROGRAMJSON-RECIPEMATCHER-RETURN-ABSENT-ROUTE-RELEASE-CONSULTATION-001",
    "MIRBUILDER-PROGRAMJSON-LOOP-BODY-RETURN-ABSENT-SCAN-ONLY-DIAGNOSTIC-001",
    "MIRBUILDER-PROGRAMJSON-CANONICAL-LOOP-FACTS-FINAL-TOPLEVEL-RETURN-DECOUPLE-SNAPSHOT-BOUNDARY-001",
    "MIRBUILDER-PROGRAMJSON-RECIPEMATCHER-RETURN-ABSENT-ACCEPTED-FLOOR-001",
}
if not any(f'latest_card = "{allowed}"' in current_state for allowed in allowed_latest):
    raise SystemExit("CURRENT_STATE latest card drift")

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

owners = [
    fn for fn in functions
    if str(fn.get("name", "")).startswith("ProgramJsonCanonicalLoopFactsInputSnapshotBox.")
    or str(fn.get("name", "")).startswith("ProgramJsonRecipeMatcherExecutionBoundaryBox.")
    or str(fn.get("name", "")).startswith("LoopStmtHandler.")
]
for fn in owners:
    bad = [row for row in rows(fn) if row.get("tier") == "Unsupported" or row.get("reason")]
    if bad:
        raise SystemExit(f"{fn.get('name')} has unsupported routes: {bad[:5]}")
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
if len(lines) != 2:
    raise SystemExit(f"expected two output lines, got {len(lines)}: {lines}")

def parse(prefix):
    for line in lines:
        if line.startswith(prefix):
            parts = line[len(prefix):].split(";")
            return dict(part.split("=", 1) for part in parts if "=" in part)
    raise SystemExit(f"missing line: {prefix}")

snap = parse("snapshot:")
match = parse("match:")
expected_snap = fixture["row"]["expected_snapshot"]
checks = {
    "ok": "1",
    "matcher_input_present": "1",
    "exit_has_continue": "1",
    "exit_has_return": "1",
    "exit_has_break": "0",
    "has_nested_loop": "0",
    "loop_cond_continue_with_return_present": "1",
    "loop_cond_return_in_body_present": "0",
    "cond_kind": expected_snap["cond_kind"],
    "loop_var": expected_snap["loop_var"],
    "loop_bound_int": str(expected_snap["loop_bound_int"]),
    "update_kind": expected_snap["update_kind"],
    "update_target": expected_snap["update_target"],
    "step_int": str(expected_snap["step_int"]),
}
for key, value in checks.items():
    if snap.get(key) != value:
        raise SystemExit(f"snapshot mismatch {key}: {snap.get(key)} != {value}")
expected_match = fixture["row"]["programjson_route_expected"]
for key, value in {
    "matched": str(expected_match["matched"]),
    "contract_kind": expected_match["contract_kind"],
    "has_break": str(expected_match["has_break"]),
    "has_continue": str(expected_match["has_continue"]),
    "has_return": str(expected_match["has_return"]),
    "full_recipe_matcher_execution": "0",
    "route_selection": "0",
    "mir_lowering": "0",
    "mir_mutation": "0",
    "id_allocation": "0",
    "runtime_route_switch": "0",
}.items():
    if match.get(key) != value:
        raise SystemExit(f"match mismatch {key}: {match.get(key)} != {value}")
PY

cat <<'REPORT'
output_contract=rust-lifecycle-mirbuilder-programjson-loop-body-ifcontinue-ifreturn-assignment-boxcount-accepted-floor-v0
token=MIRBUILDER-PROGRAMJSON-LOOP-BODY-IFCONTINUE-IFRETURN-ASSIGNMENT-BOXCOUNT-ACCEPTED-FLOOR-001
row_count=1
row_id=local_loop_body_if_continue_if_return_assignment
loop_body_three_stmt_boxcount=1
if_continue_if_return_assignment_supported=1
verified_recipe_present=1
canonical_loop_facts_snapshot_ok=1
loop_cond_continue_with_return_present=1
loop_cond_return_in_body_present=0
matcher_result_equal=1
matched=1
contract_kind=LoopWithExit
has_break=0
has_continue=1
has_return=1
runtime_authority=rust_astnode
programjson_shadow_checked=1
programjson_runtime_route_authority=0
runtime_route_switch=0
recipe_matcher_input_authority=0
full_recipe_matcher_execution=0
route_selection=0
mir_lowering=0
mir_mutation=0
id_allocation=0
runtime_fallback=0
source_selfhost_claim=0
new_backend_route=0
new_abi=0
vm_only_proof_as_main_acceptance=0
summary=ok
REPORT
