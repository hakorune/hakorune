#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="rust-lifecycle-mirbuilder-programjson-layer4-recipe-stmt-seq-dto-parity-gate"

source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

FIXTURE="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-programjson-layer4-recipe-stmt-seq-dto-parity-v0.json"
SNAPSHOT_IMPL="$ROOT_DIR/lang/src/compiler/mirbuilder/program_json_recipe_stmt_seq_dto_snapshot.hako"
SHAPE_SEQ_IMPL="$ROOT_DIR/lang/src/compiler/mirbuilder/mir_json_v0_shape_box_recipe_seq.hako"
PORT_SIG_CONTRACT="$ROOT_DIR/tools/checks/hako_aot_programjson_recipe_verifier_port_sig_result_contract_guard.sh"
HAKO_BIN="$ROOT_DIR/tools/bin/hako"

guard_require_command "$TAG" python3
guard_require_command "$TAG" timeout
guard_require_files "$TAG" "$FIXTURE" "$SNAPSHOT_IMPL" "$SHAPE_SEQ_IMPL" "$PORT_SIG_CONTRACT" "$HAKO_BIN"

CONTRACT_OUT="$(guard_cached_run "$TAG" bash "$PORT_SIG_CONTRACT")"
if ! grep -q '^recipe_verifier_port_sig_aot_call_fixed=1$' <<<"$CONTRACT_OUT"; then
  printf '%s\n' "$CONTRACT_OUT" >&2
  guard_fail "$TAG" "RecipeVerifier / PortSig AOT route contract is not green"
fi

TMP_DIR="$(mktemp -d /tmp/hakorune-layer4-recipe-stmt-seq-dto.XXXXXX)"
cleanup() { rm -rf "$TMP_DIR" >/dev/null 2>&1 || true; }
trap cleanup EXIT

APP="$TMP_DIR/layer4_recipe_stmt_seq_dto_probe.hako"
MIR_JSON="$TMP_DIR/layer4_recipe_stmt_seq_dto_probe.mir.json"
EXE="$TMP_DIR/layer4_recipe_stmt_seq_dto_probe.exe"
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

if fixture.get("kind") != "MirBuilderProgramJsonLayer4RecipeStmtSeqDtoParityV1":
    raise SystemExit("bad fixture kind")
if fixture.get("token") != "MIRBUILDER-PROGRAMJSON-LAYER4-RECIPE-STMT-SEQ-DTO-PARITY-001":
    raise SystemExit("bad fixture token")

acceptance = fixture.get("acceptance") or {}
for key, value in {
    "programjson_traversal_used": 1,
    "structured_recipe_dto_constructed": 1,
    "recipe_verifier_used": 1,
    "recipe_stmt_seq_scanner_used": 1,
    "shape_kind_selection": 0,
    "prebuilt_token_snapshot_input": 0,
    "string_only_facade": 0,
    "mir_json_route_green": 1,
    "runtime_parity_green": 1,
}.items():
    if acceptance.get(key) != value:
        raise SystemExit(f"bad acceptance field: {key}")

claims = fixture.get("claims") or {}
for key, value in claims.items():
    if value != 0:
        raise SystemExit(f"forbidden claim drift: {key}")

rows = fixture.get("rows") or []
if len(rows) < 7:
    raise SystemExit("Recipe stmt-seq DTO parity requires at least 7 rows")

calls = []
expected_rows = []
for row in rows:
    row_id = row["row_id"]
    summary = row["rust_oracle_expected_summary"]
    if not summary.startswith("snapshot_kind=RecipeStmtSeqDtoSnapshotV1;"):
        raise SystemExit(f"bad summary prefix: {row_id}")
    for token in [";err=0", ";matched=1", ";item_count=", ";seq_sig="]:
        if token not in summary:
            raise SystemExit(f"missing expected DTO token {token}: {row_id}")
    program_json = json.dumps(row["program_json"], separators=(",", ":"), ensure_ascii=False)
    calls.append(
        "    print("
        + json.dumps(f"dto:{row_id}:")
        + " + ProgramJsonRecipeStmtSeqDtoSnapshotBox.build_summary("
        + json.dumps(program_json)
        + "))"
    )
    expected_rows.append("dto:" + row_id + ":" + summary)

source = "\n".join([
    "using lang.compiler.mirbuilder.program_json_recipe_stmt_seq_dto_snapshot as ProgramJsonRecipeStmtSeqDtoSnapshotBox",
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

python3 - "$SNAPSHOT_IMPL" "$SHAPE_SEQ_IMPL" <<'PY'
import sys
from pathlib import Path

snapshot = Path(sys.argv[1]).read_text(encoding="utf-8")
shape_seq = Path(sys.argv[2]).read_text(encoding="utf-8")
for needle in [
    "ProgramJsonV0PhaseStateBox.parse(program_json",
    "RecipeVerifierBox.verify(root",
    "MirJsonV0ShapeRecipeSeq._recipe_stmt_seq_summary(root",
]:
    if needle not in snapshot:
        raise SystemExit(f"missing Recipe stmt-seq DTO implementation token: {needle}")
for needle in [
    "_recipe_stmt_seq_summary(recipe_root",
    "seq_sig=",
    "BoxHelpers.array_get(items, i)",
    "BoxHelpers.same_token(leaf_kind, \"Stmt\")",
]:
    if needle not in shape_seq:
        raise SystemExit(f"missing Recipe stmt-seq helper token: {needle}")
for forbidden in ["_shape_kind_from_recipe", "emit_mir", "new_backend_route", "RecipeMatcher", "ASTNode"]:
    if forbidden in snapshot:
        raise SystemExit(f"forbidden implementation token: {forbidden}")
PY

bash "$HAKO_BIN" --backend mir --verify "$SNAPSHOT_IMPL" >/dev/null
bash "$HAKO_BIN" --backend mir --verify "$SHAPE_SEQ_IMPL" >/dev/null

if ! timeout 90s bash "$HAKO_BIN" --backend mir --emit-mir-json "$MIR_JSON" "$APP" >"$BUILD_LOG" 2>&1; then
  tail -n 160 "$BUILD_LOG" || true
  guard_fail "$TAG" "failed to emit MIR JSON for Layer4 Recipe stmt-seq DTO probe"
fi

python3 - "$MIR_JSON" <<'PY'
import json
import sys
from pathlib import Path

data = json.loads(Path(sys.argv[1]).read_text(encoding="utf-8"))
functions = data.get("functions") or []
main = next((fn for fn in functions if fn.get("name") == "main"), None)
snapshot = next((fn for fn in functions if fn.get("name") == "ProgramJsonRecipeStmtSeqDtoSnapshotBox.build_summary/1"), None)
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

main_routes = [row for row in routes(main) if row.get("symbol") == "ProgramJsonRecipeStmtSeqDtoSnapshotBox.build_summary/1"]
if len(main_routes) < 7:
    raise SystemExit("main does not call snapshot once per fixture row")
for row in main_routes:
    if row.get("tier") != "DirectAbi" or row.get("return_shape") != "string_handle":
        raise SystemExit(f"snapshot route is not DirectAbi/string_handle: {row}")

bad = [row for row in routes(snapshot) if row.get("tier") == "Unsupported" or row.get("reason")]
if bad:
    raise SystemExit(f"snapshot function has unsupported routes: {bad[:5]}")

symbols = {row.get("symbol") for row in routes(snapshot)}
for symbol in [
    "ProgramJsonV0PhaseStateBox.parse/2",
    "RecipeVerifierBox.verify/2",
    "MirJsonV0ShapeRecipeSeq._recipe_stmt_seq_summary/2",
]:
    if symbol not in symbols:
        raise SystemExit(f"missing snapshot route: {symbol}")
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
output_contract=rust-lifecycle-mirbuilder-programjson-layer4-recipe-stmt-seq-dto-parity-gate-v0
token=MIRBUILDER-PROGRAMJSON-LAYER4-RECIPE-STMT-SEQ-DTO-PARITY-001
owner=ProgramJsonRecipeStmtSeqDtoSnapshotV1
programjson_traversal_used=1
structured_recipe_dto_constructed=1
recipe_verifier_used=1
recipe_stmt_seq_scanner_used=1
shape_kind_selection=0
mir_json_route_green=1
runtime_parity_green=1
runtime_route_switch=0
full_recipe_matcher_execution=0
route_selection=0
mir_mutation=0
id_allocation=0
backend_lowering_claim=0
source_selfhost_claim=0
summary=ok
REPORT
