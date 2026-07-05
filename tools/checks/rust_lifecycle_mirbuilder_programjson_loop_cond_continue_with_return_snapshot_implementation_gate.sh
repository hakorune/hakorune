#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="rust-lifecycle-mirbuilder-programjson-loop-cond-continue-with-return-snapshot-implementation-gate"

source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

FIXTURE="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-programjson-loop-cond-continue-with-return-snapshot-implementation-v0.json"
HAKO_IMPL="$ROOT_DIR/lang/src/compiler/mirbuilder/program_json_loop_cond_continue_with_return_snapshot.hako"
SCANNER_IMPL="$ROOT_DIR/lang/src/compiler/mirbuilder/program_json_v0_scanner_box.hako"
HAKO_BIN="$ROOT_DIR/tools/bin/hako"

guard_require_command "$TAG" python3
guard_require_files "$TAG" "$FIXTURE" "$HAKO_IMPL" "$SCANNER_IMPL" "$HAKO_BIN"

TMP_DIR="$(mktemp -d /tmp/hakorune-programjson-loop-cond-continue-snapshot.XXXXXX)"
cleanup() { rm -rf "$TMP_DIR" >/dev/null 2>&1 || true; }
trap cleanup EXIT

APP="$TMP_DIR/programjson_loop_cond_continue_with_return_snapshot.hako"
VERIFY_LOG="$TMP_DIR/verify.log"

python3 - "$FIXTURE" "$APP" <<'PY'
import json
import sys
from pathlib import Path

fixture = json.loads(Path(sys.argv[1]).read_text(encoding="utf-8"))
app = Path(sys.argv[2])
rows = fixture["rows"]

calls = []
for row in rows:
    if not row.get("expected_summary"):
        raise SystemExit(f"missing expected_summary for {row.get('case_id', '<unknown>')}")
    program_json = json.dumps(row["program_json"], separators=(",", ":"), ensure_ascii=False)
    calls.append(
        "    print(ProgramJsonLoopCondContinueWithReturnSnapshotBox.build_summary("
        f"{json.dumps(program_json)}))"
    )

source = "\n".join([
    "using lang.compiler.mirbuilder.program_json_loop_cond_continue_with_return_snapshot as ProgramJsonLoopCondContinueWithReturnSnapshotBox",
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
PY

bash "$HAKO_BIN" --backend mir --verify "$SCANNER_IMPL" >/dev/null
bash "$HAKO_BIN" --backend mir --verify "$HAKO_IMPL" >/dev/null

if ! bash "$HAKO_BIN" --backend mir --verify "$APP" >"$VERIFY_LOG" 2>&1; then
  tail -n 120 "$VERIFY_LOG" || true
  guard_fail "$TAG" "failed to verify generated ProgramJSON snapshot app"
fi

cat <<'REPORT'
output_contract=rust-lifecycle-mirbuilder-programjson-loop-cond-continue-with-return-snapshot-implementation-gate-v0
owner=ProgramJsonLoopCondContinueWithReturnSnapshotV1
input_contract=ProgramJsonLoopCondContinueThenReturnMinimalV1
fixture=mirbuilder-programjson-loop-cond-continue-with-return-snapshot-implementation-v0.json
hako_implementation=lang/src/compiler/mirbuilder/program_json_loop_cond_continue_with_return_snapshot.hako
scanner=lang/src/compiler/mirbuilder/program_json_v0_scanner_box.hako
implementation_rows=5
mir_verify_status=green
execution_backend=not_run
aot_execution_status=blocked_by_existing_program_json_v0_scanner_lowering
parity_status=not_claimed
source_selfhost_claim=0
hako_adopted_decision=0
rust_astnode_projector_retired=0
programjson_full_parser_claim=0
recipe_matching_migrated=0
route_selection_migration=0
backend_lowering_migration=0
mir_mutation_migration=0
id_allocation_migration=0
summary=ok
REPORT
