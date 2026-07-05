#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="rust-lifecycle-mirbuilder-programjson-block-expr-shape-scan-parity-gate"

source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

FIXTURE="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-programjson-block-expr-shape-scan-parity-v0.json"
SCAN_IMPL="$ROOT_DIR/lang/src/compiler/mirbuilder/program_json_block_expr_shape_scan.hako"
SCANNER_IMPL="$ROOT_DIR/lang/src/compiler/mirbuilder/program_json_v0_scanner_box.hako"
HAKO_BIN="$ROOT_DIR/tools/bin/hako"

guard_require_command "$TAG" python3
guard_require_files "$TAG" "$FIXTURE" "$SCAN_IMPL" "$SCANNER_IMPL" "$HAKO_BIN"

TMP_DIR="$(mktemp -d /tmp/hakorune-programjson-block-expr-shape-scan.XXXXXX)"
cleanup() { rm -rf "$TMP_DIR" >/dev/null 2>&1 || true; }
trap cleanup EXIT

APP="$TMP_DIR/programjson_block_expr_shape_scan_parity.hako"
EXPECTED="$TMP_DIR/expected.json"
RUN_LOG="$TMP_DIR/run.log"
EXE="$TMP_DIR/programjson_block_expr_shape_scan_parity.exe"
EMIT_LOG="$TMP_DIR/emit.log"

python3 - "$FIXTURE" "$APP" "$EXPECTED" <<'PY'
import json
import sys
from pathlib import Path

fixture = json.loads(Path(sys.argv[1]).read_text(encoding="utf-8"))
app = Path(sys.argv[2])
expected = Path(sys.argv[3])

if fixture.get("kind") != "MirBuilderProgramJsonBlockExprShapeScanParityV1":
    raise SystemExit("bad fixture kind")
if fixture.get("token") != "MIRBUILDER-PROGRAMJSON-BLOCK-EXPR-SHAPE-SCAN-CAPABILITY-001":
    raise SystemExit("bad fixture token")

emit = fixture.get("programjson_emit_evidence") or {}
if emit.get("source") != "src/macro/ast_json/joinir_compat.rs":
    raise SystemExit("missing ProgramJSON emit source evidence")
if emit.get("node_kind") != "BlockExpr":
    raise SystemExit("bad ProgramJSON node kind evidence")
if emit.get("node_type") is not None:
    raise SystemExit("BlockExpr ProgramJSON node_type must stay null")
if emit.get("fields") != ["prelude_stmts", "tail_expr"]:
    raise SystemExit("bad ProgramJSON field evidence")

rows = fixture.get("rows") or []
if len(rows) != 10:
    raise SystemExit("block-expr capability must cover exactly 10 rows")

summary = fixture.get("summary") or {}
if summary.get("programjson_traversal_used") != 1:
    raise SystemExit("programjson traversal claim missing")
if summary.get("string_only_facade") != 0:
    raise SystemExit("string-only facade must remain 0")
if summary.get("rust_astnode_projector_retire_candidate") != 1:
    raise SystemExit("retire-candidate marker missing")

claims = fixture.get("claims") or {}
for key in [
    "source_selfhost_claim",
    "mir_mutation",
    "id_allocation",
    "backend_lowering",
    "full_recipe_matcher_execution",
    "route_selection",
    "runtime_fallback",
    "new_backend_route",
    "new_abi",
    "programjson_full_parser_claim",
    "hako_adopted_for_full_owner",
    "block_expr_lowering",
    "prelude_execution_semantics",
]:
    if claims.get(key) != 0:
        raise SystemExit(f"forbidden claim drift: {key}")

calls = []
expected_rows = []
for row in rows:
    row_id = row["row_id"]
    program_json = json.dumps(row["program_json"], separators=(",", ":"), ensure_ascii=False)
    calls.append(
        "    print("
        + json.dumps(f"snapshot:{row_id}:")
        + " + ProgramJsonBlockExprShapeScanBox.build_summary("
        + json.dumps(program_json)
        + "))"
    )
    expected_rows.append({"row_id": row_id, "expected": row["rust_astnode_token_oracle"]["expected_summary"]})

source = "\n".join([
    "using lang.compiler.mirbuilder.program_json_block_expr_shape_scan as ProgramJsonBlockExprShapeScanBox",
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
expected.write_text(json.dumps(expected_rows, indent=2, sort_keys=True) + "\n", encoding="utf-8")
PY

bash "$HAKO_BIN" --backend mir --verify "$SCANNER_IMPL" >/dev/null
bash "$HAKO_BIN" --backend mir --verify "$SCAN_IMPL" >/dev/null

if ! bash "$HAKO_BIN" --backend mir --emit-exe "$EXE" "$APP" >"$EMIT_LOG" 2>&1; then
  tail -n 160 "$EMIT_LOG" || true
  guard_fail "$TAG" "failed to emit ProgramJSON block-expr shape scan parity executable"
fi

if ! "$EXE" >"$RUN_LOG" 2>&1; then
  tail -n 160 "$RUN_LOG" || true
  guard_fail "$TAG" "failed to run ProgramJSON block-expr shape scan parity executable"
fi

python3 - "$EXPECTED" "$RUN_LOG" <<'PY'
import json
import sys
from pathlib import Path

expected_rows = json.loads(Path(sys.argv[1]).read_text(encoding="utf-8"))
expected = {row["row_id"]: row["expected"] for row in expected_rows}
actual = {}
for raw in Path(sys.argv[2]).read_text(encoding="utf-8").splitlines():
    line = raw.strip()
    if not line or line.startswith("Result:"):
        continue
    parts = line.split(":", 2)
    if len(parts) != 3 or parts[0] != "snapshot":
        print(f"bad output line: {line!r}", file=sys.stderr)
        raise SystemExit(1)
    actual[parts[1]] = parts[2]

if set(actual) != set(expected):
    print(f"expected rows={sorted(expected)}", file=sys.stderr)
    print(f"actual rows={sorted(actual)}", file=sys.stderr)
    raise SystemExit(1)

for row_id, exp in expected.items():
    got = actual[row_id]
    if got != exp:
        print(f"mismatch row={row_id} expected={exp!r} actual={got!r}", file=sys.stderr)
        raise SystemExit(1)
PY

cat <<'REPORT'
output_contract=rust-lifecycle-mirbuilder-programjson-block-expr-shape-scan-parity-gate-v0
owner=ProgramJsonBlockExprShapeScanV1
input_contract=ProgramJSON-v0
output_contract=BlockExprShapeSnapshotV1
fixture=mirbuilder-programjson-block-expr-shape-scan-parity-v0.json
hako_implementation=lang/src/compiler/mirbuilder/program_json_block_expr_shape_scan.hako
scanner=lang/src/compiler/mirbuilder/program_json_v0_scanner_box.hako
parity_rows=10
covered_rows=10
execution_backend=aot
aot_execution_status=green
token_snapshot_parity=green
programjson_traversal_used=1
string_only_facade=0
rust_astnode_projector_retire_candidate=1
block_expr_lowering=0
prelude_execution_semantics=0
source_selfhost_claim=0
mir_mutation=0
id_allocation=0
backend_lowering=0
full_recipe_matcher_execution=0
route_selection=0
runtime_fallback=0
new_backend_route=0
new_abi=0
summary=ok
REPORT
