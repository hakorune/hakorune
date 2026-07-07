#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="rust-lifecycle-mirbuilder-programjson-recipematcher-missing-verified-recipe-reject-row"

source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

FIXTURE="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-programjson-recipematcher-missing-verified-recipe-reject-row-v0.json"
CARD="$ROOT_DIR/docs/development/current/main/phases/phase-296x/3248-MIRBUILDER-PROGRAMJSON-RECIPEMATCHER-MISSING-VERIFIED-RECIPE-REJECT-ROW-001.md"
TASK_ORDER="$ROOT_DIR/docs/development/current/main/design/mirbuilder-rust-to-hako-converter-task-order-ssot.md"
CURRENT_STATE="$ROOT_DIR/docs/development/current/main/CURRENT_STATE.toml"
PREV_GUARD="$ROOT_DIR/tools/checks/rust_lifecycle_mirbuilder_programjson_recipematcher_reject_floor_selection_guard.sh"
SNAPSHOT_IMPL="$ROOT_DIR/lang/src/compiler/mirbuilder/program_json_canonical_loop_facts_input_snapshot.hako"
MATCHER_IMPL="$ROOT_DIR/lang/src/compiler/mirbuilder/program_json_recipematcher_execution_boundary.hako"
HAKO_BIN="$ROOT_DIR/tools/bin/hako"

guard_require_command "$TAG" python3
guard_require_command "$TAG" timeout
guard_require_files "$TAG" "$FIXTURE" "$CARD" "$TASK_ORDER" "$CURRENT_STATE" "$PREV_GUARD" "$SNAPSHOT_IMPL" "$MATCHER_IMPL" "$HAKO_BIN"

PREV_OUT="$(HAKO_GUARD_RESULT_CACHE_ALLOW_DIRTY=1 guard_cached_run "$TAG-prev" bash "$PREV_GUARD")"
if ! grep -q '^selected_next_card=MIRBUILDER-PROGRAMJSON-RECIPEMATCHER-MISSING-VERIFIED-RECIPE-REJECT-ROW-001$' <<<"$PREV_OUT"; then
  printf '%s\n' "$PREV_OUT" >&2
  guard_fail "$TAG" "missing-verified-recipe selection prerequisite is not green"
fi

TMP_DIR="$(mktemp -d /tmp/hakorune-missing-verified-recipe-reject.XXXXXX)"
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

token = "MIRBUILDER-PROGRAMJSON-RECIPEMATCHER-MISSING-VERIFIED-RECIPE-REJECT-ROW-001"
if fixture.get("kind") != "MirBuilderProgramJsonRecipeMatcherMissingVerifiedRecipeRejectRowV1":
    raise SystemExit("bad fixture kind")
if fixture.get("token") != token:
    raise SystemExit("bad fixture token")
row = fixture.get("row") or {}
if row.get("row_id") != "missing_verified_recipe_reject":
    raise SystemExit("bad row id")
if row.get("program_json") != {}:
    raise SystemExit("program_json drift")
if row.get("expected_snapshot", {}).get("reason") != "verified_recipe_missing":
    raise SystemExit("snapshot expectation drift")
if row.get("expected_matcher_result", {}).get("reason") != "snapshot_not_ok":
    raise SystemExit("matcher expectation drift")

for needle in [
    'return me._err("verified_recipe_missing")',
    'if reason == "verified_recipe_missing" { return 1 }',
    '"matcher_input_present" => 0',
]:
    if needle not in snapshot_impl:
        raise SystemExit(f"snapshot impl missing: {needle}")
for needle in [
    'return me._err("snapshot_not_ok")',
    'if reason == "snapshot_not_ok" { return 2 }',
    '"matched" => 0',
]:
    if needle not in matcher_impl:
        raise SystemExit(f"matcher impl missing: {needle}")

claims = fixture.get("claims") or {}
positive = {
    "missing_verified_recipe_reject_row_green",
    "reject_floor_row_green",
    "matcher_refuses_bad_snapshot",
    "programjson_shadow_checked",
}
for key in positive:
    if claims.get(key) != 1:
        raise SystemExit(f"missing positive claim: {key}")
for key, value in claims.items():
    if key in positive:
        continue
    if value != 0:
        raise SystemExit(f"forbidden claim drift: {key}")

for needle in [
    token,
    "verified_recipe_missing",
    "snapshot_not_ok",
    "runtime_route_switch = 0",
    "programjson_runtime_route_authority = 0",
    "Source Selfhost remains unclaimed",
]:
    if needle not in card:
        raise SystemExit(f"card missing: {needle}")
for needle in [token, "MIRBUILDER-PROGRAMJSON-RECIPEMATCHER-ROUTE-CONSUMED-FIELD-FLOOR-SELECTION-001"]:
    if needle not in task_order:
        raise SystemExit(f"task-order missing: {needle}")
allowed_latest = {
    token,
    "MIRBUILDER-PROGRAMJSON-RECIPEMATCHER-ROUTE-CONSUMED-FIELD-FLOOR-SELECTION-001",
    "MIRBUILDER-PROGRAMJSON-RECIPEMATCHER-ROUTE-CONSUMED-FIELD-FLOOR-PARITY-001",
    "MIRBUILDER-PROGRAMJSON-RECIPEMATCHER-AUTHORITY-SWITCH-READINESS-CONSULTATION-001",
    "MIRBUILDER-PROGRAMJSON-RECIPEMATCHER-NESTED-LOOP-DECISION-ROW-CONSULTATION-001",
    "MIRBUILDER-PROGRAMJSON-RECIPEMATCHER-NESTED-LOOP-REJECT-BOUNDARY-001",
    "MIRBUILDER-PROGRAMJSON-RECIPEMATCHER-REJECT-FLOOR-EXPANSION-SELECTION-001",
    "MIRBUILDER-PROGRAMJSON-RECIPEMATCHER-UNSUPPORTED-CONDITION-OPERATOR-REJECT-ROW-001",
}
if not any(f'latest_card = "{allowed}"' in current_state for allowed in allowed_latest):
    raise SystemExit("CURRENT_STATE latest card drift")

app.write_text("\n".join([
    "using lang.compiler.mirbuilder.program_json_canonical_loop_facts_input_snapshot as ProgramJsonCanonicalLoopFactsInputSnapshotBox",
    "using lang.compiler.mirbuilder.program_json_recipematcher_execution_boundary as ProgramJsonRecipeMatcherExecutionBoundaryBox",
    "",
    "static box Main {",
    "  main() {",
    "    local snapshot = ProgramJsonCanonicalLoopFactsInputSnapshotBox.build_snapshot(\"{}\")",
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
if len(lines) != 2:
    raise SystemExit(f"expected two output lines, got {len(lines)}: {lines}")
joined = "\n".join(lines)
for needle in [
    "snapshot:snapshot_kind=ProgramJsonCanonicalLoopFactsInputSnapshotV1;ok=0;reason=verified_recipe_missing",
    ";matcher_input_present=0",
    ";exit_has_continue=0",
    ";exit_has_return=0",
    ";exit_has_break=0",
    ";recipe_matcher_executed=0",
    "match:snapshot_kind=ProgramJsonRecipeMatcherExecutionBoundaryResultV1;ok=0;reason=snapshot_not_ok",
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
output_contract=rust-lifecycle-mirbuilder-programjson-recipematcher-missing-verified-recipe-reject-row-v0
token=MIRBUILDER-PROGRAMJSON-RECIPEMATCHER-MISSING-VERIFIED-RECIPE-REJECT-ROW-001
row_id=missing_verified_recipe_reject
missing_verified_recipe_reject_row_green=1
reject_floor_row_green=1
matcher_refuses_bad_snapshot=1
snapshot_reason=verified_recipe_missing
matcher_reason=snapshot_not_ok
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
