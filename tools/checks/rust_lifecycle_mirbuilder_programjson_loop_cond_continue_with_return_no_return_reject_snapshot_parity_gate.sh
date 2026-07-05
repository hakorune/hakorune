#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="rust-lifecycle-mirbuilder-programjson-loop-cond-continue-with-return-no-return-reject-snapshot-parity-gate"

source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

FIXTURE="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-programjson-loop-cond-continue-with-return-no-return-reject-snapshot-parity-v0.json"
SNAPSHOT_IMPL="$ROOT_DIR/lang/src/compiler/mirbuilder/program_json_loop_cond_continue_with_return_snapshot.hako"
SCANNER_IMPL="$ROOT_DIR/lang/src/compiler/mirbuilder/program_json_v0_scanner_box.hako"
HAKO_BIN="$ROOT_DIR/tools/bin/hako"

guard_require_command "$TAG" python3
guard_require_files "$TAG" "$FIXTURE" "$SNAPSHOT_IMPL" "$SCANNER_IMPL" "$HAKO_BIN"

TMP_DIR="$(mktemp -d /tmp/hakorune-programjson-loop-cond-continue-no-return-parity.XXXXXX)"
cleanup() { rm -rf "$TMP_DIR" >/dev/null 2>&1 || true; }
trap cleanup EXIT

APP="$TMP_DIR/programjson_loop_cond_continue_no_return_reject_snapshot_parity.hako"
EXPECTED="$TMP_DIR/expected.txt"
RUN_LOG="$TMP_DIR/run.log"
EXE="$TMP_DIR/programjson_loop_cond_continue_no_return_reject_snapshot_parity.exe"
EMIT_LOG="$TMP_DIR/emit.log"

python3 - "$FIXTURE" "$APP" "$EXPECTED" <<'PY'
import json
import sys
from pathlib import Path

fixture = json.loads(Path(sys.argv[1]).read_text(encoding="utf-8"))
app = Path(sys.argv[2])
expected = Path(sys.argv[3])

calls = []
expected_lines = []
for row in fixture["rows"]:
    program_json = json.dumps(row["program_json"], separators=(",", ":"), ensure_ascii=False)
    calls.append(
        "    print(ProgramJsonLoopCondContinueWithReturnSnapshotBox.build_summary("
        + json.dumps(program_json)
        + "))"
    )
    expected_lines.append(row["rust_astnode_token_oracle"]["expected_summary"])

source = "\n".join(
    [
        "using lang.compiler.mirbuilder.program_json_loop_cond_continue_with_return_snapshot as ProgramJsonLoopCondContinueWithReturnSnapshotBox",
        "",
        "static box Main {",
        "  main() {",
        *calls,
        "    return 0",
        "  }",
        "}",
        "",
    ]
)
app.write_text(source, encoding="utf-8")
expected.write_text("\n".join(expected_lines) + "\n", encoding="utf-8")
PY

bash "$HAKO_BIN" --backend mir --verify "$SCANNER_IMPL" >/dev/null
bash "$HAKO_BIN" --backend mir --verify "$SNAPSHOT_IMPL" >/dev/null

if ! bash "$HAKO_BIN" --backend mir --emit-exe "$EXE" "$APP" >"$EMIT_LOG" 2>&1; then
  tail -n 160 "$EMIT_LOG" || true
  guard_fail "$TAG" "failed to emit no-return snapshot parity executable"
fi

if ! "$EXE" >"$RUN_LOG" 2>&1; then
  tail -n 160 "$RUN_LOG" || true
  guard_fail "$TAG" "failed to run no-return snapshot parity executable"
fi

python3 - "$EXPECTED" "$RUN_LOG" <<'PY'
import sys
from pathlib import Path

expected = Path(sys.argv[1]).read_text(encoding="utf-8").splitlines()
actual = []
for raw in Path(sys.argv[2]).read_text(encoding="utf-8").splitlines():
    line = raw.strip()
    if line and not line.startswith("Result:"):
        actual.append(line)
if actual != expected:
    print(f"expected={expected!r}", file=sys.stderr)
    print(f"actual={actual!r}", file=sys.stderr)
    raise SystemExit(1)
PY

cat <<'REPORT'
output_contract=rust-lifecycle-mirbuilder-programjson-loop-cond-continue-with-return-no-return-reject-snapshot-parity-gate-v0
owner=ProgramJsonLoopCondContinueWithReturnSnapshotV1
shape_scope=LoopCondContinueNoReturnRejectV1
fixture=mirbuilder-programjson-loop-cond-continue-with-return-no-return-reject-snapshot-parity-v0.json
parity_rows=1
execution_backend=aot
aot_execution_status=green
token_snapshot_parity=green
programjson_snapshot_matches_rust_astnode_oracle=1
source_selfhost_claim=0
hako_adopted_decision=0
rust_astnode_projector_retired=0
rust_astnode_projector_fully_retired=0
programjson_full_parser_claim=0
recipe_matching_migrated=0
route_selection_migration=0
backend_lowering_migration=0
mir_mutation_migration=0
id_allocation_migration=0
summary=ok
REPORT
