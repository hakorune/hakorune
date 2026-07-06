#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="rust-lifecycle-mirbuilder-programjson-layer4-exit-recipe-dto-loop-body-parity-gate"

source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

FIXTURE="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-programjson-layer4-exit-recipe-dto-loop-body-parity-v0.json"
SNAPSHOT_IMPL="$ROOT_DIR/lang/src/compiler/mirbuilder/program_json_exit_recipe_dto_snapshot.hako"
LOOP_HANDLER="$ROOT_DIR/lang/src/compiler/mirbuilder/stmt_handlers/loop_stmt_handler.hako"
LEGACY_PARITY_GATE="$ROOT_DIR/tools/checks/rust_lifecycle_mirbuilder_programjson_layer4_exit_recipe_dto_parity_gate.sh"
HAKO_BIN="$ROOT_DIR/tools/bin/hako"

guard_require_command "$TAG" python3
guard_require_command "$TAG" timeout
guard_require_files "$TAG" "$FIXTURE" "$SNAPSHOT_IMPL" "$LOOP_HANDLER" "$LEGACY_PARITY_GATE" "$HAKO_BIN"

LEGACY_OUT="$(guard_cached_run "$TAG" bash "$LEGACY_PARITY_GATE")"
if ! grep -q '^runtime_parity_green=1$' <<<"$LEGACY_OUT"; then
  printf '%s\n' "$LEGACY_OUT" >&2
  guard_fail "$TAG" "legacy Exit Recipe DTO parity guard is not green"
fi

TMP_DIR="$(mktemp -d /tmp/hakorune-layer4-exit-loop-body-dto.XXXXXX)"
cleanup() { rm -rf "$TMP_DIR" >/dev/null 2>&1 || true; }
trap cleanup EXIT

APP="$TMP_DIR/layer4_exit_loop_body_dto_probe.hako"
MIR_JSON="$TMP_DIR/layer4_exit_loop_body_dto_probe.mir.json"
EXE="$TMP_DIR/layer4_exit_loop_body_dto_probe.exe"
EXPECTED="$TMP_DIR/expected.txt"
BUILD_LOG="$TMP_DIR/build.log"
RUN_OUT="$TMP_DIR/run.out"
RUN_FILTERED="$TMP_DIR/run.filtered"
RUN_ERR="$TMP_DIR/run.err"

python3 - "$FIXTURE" "$APP" "$EXPECTED" <<'PY'
import json
import sys
from pathlib import Path

fixture = json.loads(Path(sys.argv[1]).read_text(encoding="utf-8"))
app = Path(sys.argv[2])
expected = Path(sys.argv[3])

if fixture.get("kind") != "MirBuilderProgramJsonLayer4ExitRecipeDtoLoopBodyParityV1":
    raise SystemExit("bad fixture kind")
if fixture.get("token") != "MIRBUILDER-PROGRAMJSON-LAYER4-EXIT-RECIPE-DTO-LOOP-BODY-PARITY-001":
    raise SystemExit("bad fixture token")

acceptance = fixture.get("acceptance") or {}
for key, value in {
    "programjson_traversal_used": 1,
    "structured_recipe_dto_constructed": 1,
    "prebuilt_token_snapshot_input": 0,
    "string_only_facade": 0,
    "mir_json_route_green": 1,
    "runtime_parity_green": 1,
    "loop_exit_dto_green": 1,
    "legacy_root_exit_parity_guard_still_green": 1,
}.items():
    if acceptance.get(key) != value:
        raise SystemExit(f"bad acceptance field: {key}")

claims = fixture.get("claims") or {}
for key, value in claims.items():
    if value != 0:
        raise SystemExit(f"forbidden claim drift: {key}")

rows = fixture.get("rows") or []
if len(rows) != 1:
    raise SystemExit("loop-body Exit parity must stay one promoted row")

calls = []
expected_rows = []
for row in rows:
    row_id = row["row_id"]
    summary = row["rust_oracle_expected_summary"]
    if "exit_path=root.loop.body0.if.then" not in summary:
        raise SystemExit(f"row does not prove loop-body Exit: {row_id}")
    program_json = json.dumps(row["program_json"], separators=(",", ":"), ensure_ascii=False)
    calls.append(
        "    print("
        + json.dumps(f"dto:{row_id}:")
        + " + ProgramJsonExitRecipeDtoSnapshotBox.build_summary("
        + json.dumps(program_json)
        + "))"
    )
    expected_rows.append("dto:" + row_id + ":" + summary)

source = "\n".join([
    "using lang.compiler.mirbuilder.program_json_exit_recipe_dto_snapshot as ProgramJsonExitRecipeDtoSnapshotBox",
    "",
    "static box Main {",
    "  main() {",
    *calls,
    "    return 0",
    "  }",
    "}",
    "",
])
app.write_text(source, encoding="utf-8")
expected.write_text("\n".join(expected_rows) + "\n", encoding="utf-8")
PY

python3 - "$LOOP_HANDLER" <<'PY'
import sys
from pathlib import Path

text = Path(sys.argv[1]).read_text(encoding="utf-8")
for needle in [
    'BoxHelpers.same_token(after_first_ch, ",")',
    'BoxHelpers.same_token(body_kind, "if_then_assignment")',
    'RecipeItemBox.exit_item("Return", if_exit_payload)',
]:
    if needle not in text:
        raise SystemExit(f"missing LoopStmtHandler contract token: {needle}")
for forbidden in [
    'after_first_ch != ","',
    'body_kind == "if_then_assignment"',
]:
    if forbidden in text:
        raise SystemExit(f"forbidden dynamic string comparison remains: {forbidden}")
PY

bash "$HAKO_BIN" --backend mir --verify "$SNAPSHOT_IMPL" >/dev/null
bash "$HAKO_BIN" --backend mir --verify "$LOOP_HANDLER" >/dev/null

if ! timeout 90s bash "$HAKO_BIN" --backend mir --emit-mir-json "$MIR_JSON" "$APP" >"$BUILD_LOG" 2>&1; then
  tail -n 160 "$BUILD_LOG" || true
  guard_fail "$TAG" "failed to emit MIR JSON for Layer4 Exit loop-body DTO probe"
fi

python3 - "$MIR_JSON" <<'PY'
import json
import sys
from pathlib import Path

data = json.loads(Path(sys.argv[1]).read_text(encoding="utf-8"))
functions = data.get("functions") or []
main = next((fn for fn in functions if fn.get("name") == "main"), None)
snapshot = next((fn for fn in functions if fn.get("name") == "ProgramJsonExitRecipeDtoSnapshotBox.build_summary/1"), None)
if main is None:
    raise SystemExit("main function missing")
if snapshot is None:
    raise SystemExit("snapshot implementation function missing")

def routes(fn):
    rows = []
    metadata = fn.get("metadata") or {}
    for key in ("global_call_routes", "lowering_plan"):
        value = metadata.get(key) or []
        if isinstance(value, list):
            rows.extend(row for row in value if isinstance(row, dict))
    return rows

main_routes = [row for row in routes(main) if row.get("symbol") == "ProgramJsonExitRecipeDtoSnapshotBox.build_summary/1"]
if len(main_routes) < 1:
    raise SystemExit("main does not call snapshot for the promoted loop-body row")
for row in main_routes:
    if row.get("tier") != "DirectAbi" or row.get("return_shape") != "string_handle":
        raise SystemExit(f"snapshot route is not DirectAbi/string_handle: {row}")

bad = [row for row in routes(snapshot) if row.get("tier") == "Unsupported" or row.get("reason")]
if bad:
    raise SystemExit(f"snapshot function has unsupported routes: {bad[:5]}")

phase_routes = [row for row in routes(snapshot) if row.get("symbol") == "ProgramJsonV0PhaseStateBox.parse/2"]
if not any(row.get("tier") == "DirectAbi" and row.get("return_shape") == "map_handle" for row in phase_routes):
    raise SystemExit(f"PhaseState parse route is not DirectAbi/map_handle: {phase_routes}")
PY

if ! timeout --kill-after=2s 120s bash "$HAKO_BIN" --backend mir --emit-exe "$EXE" "$APP" >"$BUILD_LOG" 2>&1; then
  tail -n 160 "$BUILD_LOG" >&2 || true
  guard_fail "$TAG" "emit-exe probe failed or timed out"
fi
if ! "$EXE" >"$RUN_OUT" 2>"$RUN_ERR"; then
  cat "$RUN_ERR" >&2 || true
  guard_fail "$TAG" "executable failed at runtime"
fi
grep -v '^Result: 0$' "$RUN_OUT" >"$RUN_FILTERED" || true
if ! diff -u "$EXPECTED" "$RUN_FILTERED" >/dev/null; then
  echo "[${TAG}] expected:" >&2
  cat "$EXPECTED" >&2
  echo "[${TAG}] actual:" >&2
  cat "$RUN_FILTERED" >&2
  guard_fail "$TAG" "runtime parity mismatch"
fi

cat <<'REPORT'
output_contract=rust-lifecycle-mirbuilder-programjson-layer4-exit-recipe-dto-loop-body-parity-gate-v0
token=MIRBUILDER-PROGRAMJSON-LAYER4-EXIT-RECIPE-DTO-LOOP-BODY-PARITY-001
owner=ProgramJsonExitRecipeDtoSnapshotV1
programjson_traversal_used=1
structured_recipe_dto_constructed=1
mir_json_route_green=1
runtime_parity_green=1
loop_exit_dto_green=1
legacy_root_exit_parity_guard_still_green=1
runtime_route_switch=0
full_recipe_matcher_execution=0
mir_mutation=0
id_allocation=0
backend_lowering_claim=0
source_selfhost_claim=0
summary=ok
REPORT
