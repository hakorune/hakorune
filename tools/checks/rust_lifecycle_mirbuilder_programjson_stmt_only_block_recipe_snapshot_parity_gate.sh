#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="rust-lifecycle-mirbuilder-programjson-stmt-only-block-recipe-snapshot-parity-gate"

source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

FIXTURE="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-programjson-stmt-only-block-recipe-snapshot-parity-v0.json"
SNAPSHOT_IMPL="$ROOT_DIR/lang/src/compiler/mirbuilder/program_json_stmt_only_block_recipe_snapshot.hako"
SELECTION_GUARD="$ROOT_DIR/tools/checks/rust_lifecycle_mirbuilder_programjson_block_recipe_next_capability_selection_guard.sh"
HAKO_BIN="$ROOT_DIR/tools/bin/hako"

guard_require_command "$TAG" python3
guard_require_command "$TAG" timeout
guard_require_files "$TAG" "$FIXTURE" "$SNAPSHOT_IMPL" "$SELECTION_GUARD" "$HAKO_BIN"

SELECTION_OUT="$(guard_cached_run "$TAG" bash "$SELECTION_GUARD")"
if ! grep -q '^summary=ok$' <<<"$SELECTION_OUT"; then
  printf '%s\n' "$SELECTION_OUT" >&2
  guard_fail "$TAG" "ProgramJSON stmt-only block recipe selection is not green"
fi

TMP_DIR="$(mktemp -d /tmp/hakorune-programjson-stmt-only-block-recipe.XXXXXX)"
cleanup() { rm -rf "$TMP_DIR" >/dev/null 2>&1 || true; }
trap cleanup EXIT

APP="$TMP_DIR/programjson_stmt_only_block_recipe_probe.hako"
MIR_JSON="$TMP_DIR/programjson_stmt_only_block_recipe_probe.mir.json"
EXE="$TMP_DIR/programjson_stmt_only_block_recipe_probe.exe"
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

if fixture.get("kind") != "MirBuilderProgramJsonStmtOnlyBlockRecipeSnapshotParityV1":
    raise SystemExit("bad fixture kind")
if fixture.get("token") != "MIRBUILDER-PROGRAMJSON-STMT-ONLY-BLOCK-RECIPE-SNAPSHOT-PARITY-001":
    raise SystemExit("bad fixture token")

acceptance = fixture.get("acceptance") or {}
for key, value in {
    "programjson_traversal_used": 1,
    "recipe_root_traversal_used": 1,
    "stmt_only_reducer_called": 1,
    "prebuilt_token_snapshot_input": 0,
    "string_only_facade": 0,
    "mir_json_route_green": 1,
    "runtime_parity_green": 1,
    "row_count": 4,
    "selection_prerequisite_green": 1,
}.items():
    if acceptance.get(key) != value:
        raise SystemExit(f"bad acceptance field: {key}")

claims = fixture.get("claims") or {}
for key, value in claims.items():
    if value != 0:
        raise SystemExit(f"forbidden claim drift: {key}")

rows = fixture.get("rows") or []
if len(rows) != 4:
    raise SystemExit("StmtOnly ProgramJSON parity requires exactly 4 rows")

calls = []
expected_rows = []
for row in rows:
    row_id = row["row_id"]
    summary = row["rust_oracle_expected_summary"]
    if not summary.startswith("snapshot_kind=ProgramJsonStmtOnlyBlockRecipeSnapshotV1;"):
        raise SystemExit(f"bad summary prefix: {row_id}")
    program_json = json.dumps(row["program_json"], separators=(",", ":"), ensure_ascii=False)
    calls.append(
        "    print("
        + json.dumps(f"dto:{row_id}:")
        + " + ProgramJsonStmtOnlyBlockRecipeSnapshotBox.build_summary("
        + json.dumps(program_json)
        + "))"
    )
    expected_rows.append("dto:" + row_id + ":" + summary)

source = "\n".join([
    "using lang.compiler.mirbuilder.program_json_stmt_only_block_recipe_snapshot as ProgramJsonStmtOnlyBlockRecipeSnapshotBox",
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

python3 - "$SNAPSHOT_IMPL" <<'PY'
import sys
from pathlib import Path

text = Path(sys.argv[1]).read_text(encoding="utf-8")
for needle in [
    "ProgramJsonV0PhaseStateBox.parse(program_json",
    "BoxHelpers.map_get(root, \"items\")",
    "StmtOnlyBlockRecipeBox.build_summary(stmt0, stmt1)",
]:
    if needle not in text:
        raise SystemExit(f"missing ProgramJSON StmtOnly bridge token: {needle}")
for forbidden in ["RecipeMatcher", "emit_mir", "new_backend_route", "ASTNode", "RecipeBodies"]:
    if forbidden in text:
        raise SystemExit(f"forbidden implementation token: {forbidden}")
PY

bash "$HAKO_BIN" --backend mir --verify "$SNAPSHOT_IMPL" >/dev/null

if ! timeout 90s bash "$HAKO_BIN" --backend mir --emit-mir-json "$MIR_JSON" "$APP" >"$BUILD_LOG" 2>&1; then
  tail -n 160 "$BUILD_LOG" || true
  guard_fail "$TAG" "failed to emit MIR JSON for ProgramJSON StmtOnly block recipe probe"
fi

python3 - "$MIR_JSON" <<'PY'
import json
import sys
from pathlib import Path

data = json.loads(Path(sys.argv[1]).read_text(encoding="utf-8"))
functions = data.get("functions") or []
main = next((fn for fn in functions if fn.get("name") == "main"), None)
snapshot = next((fn for fn in functions if fn.get("name") == "ProgramJsonStmtOnlyBlockRecipeSnapshotBox.build_summary/1"), None)
if main is None:
    raise SystemExit("main function missing")
if snapshot is None:
    raise SystemExit("snapshot implementation function missing")

def route_rows(fn):
    metadata = fn.get("metadata") or {}
    rows = []
    for key in ("global_call_routes", "lowering_plan"):
        value = metadata.get(key) or []
        if isinstance(value, list):
            rows.extend(row for row in value if isinstance(row, dict))
    return rows

main_routes = [row for row in route_rows(main) if row.get("symbol") == "ProgramJsonStmtOnlyBlockRecipeSnapshotBox.build_summary/1"]
if len(main_routes) < 4:
    raise SystemExit("main does not call snapshot once per fixture row")
for row in main_routes:
    if row.get("tier") != "DirectAbi" or row.get("return_shape") != "string_handle":
        raise SystemExit(f"snapshot route is not DirectAbi/string_handle: {row}")

bad = [row for row in route_rows(snapshot) if row.get("tier") == "Unsupported" or row.get("reason")]
if bad:
    raise SystemExit(f"snapshot function has unsupported routes: {bad[:5]}")

symbols = {row.get("symbol") for row in route_rows(snapshot)}
for symbol in [
    "ProgramJsonV0PhaseStateBox.parse/2",
    "StmtOnlyBlockRecipeBox.build_summary/2",
]:
    if symbol not in symbols:
        raise SystemExit(f"missing snapshot route: {symbol}")
PY

if ! timeout --kill-after=2s 120s env \
    NYASH_LLVM_OPT_LEVEL=0 \
    HAKO_LLVM_OPT_LEVEL=0 \
    bash "$HAKO_BIN" --backend mir --emit-exe "$EXE" "$APP" >"$BUILD_LOG" 2>&1; then
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
output_contract=rust-lifecycle-mirbuilder-programjson-stmt-only-block-recipe-snapshot-parity-gate-v0
token=MIRBUILDER-PROGRAMJSON-STMT-ONLY-BLOCK-RECIPE-SNAPSHOT-PARITY-001
owner=ProgramJsonStmtOnlyBlockRecipeSnapshotV1
row_count=4
programjson_traversal_used=1
recipe_root_traversal_used=1
stmt_only_reducer_called=1
prebuilt_token_snapshot_input=0
string_only_facade=0
mir_json_route_green=1
runtime_parity_green=1
runtime_route_switch=0
recipe_bodies_materialization=0
full_recipe_matcher_execution=0
route_selection=0
mir_mutation=0
id_allocation=0
backend_lowering_claim=0
source_selfhost_claim=0
summary=ok
REPORT
