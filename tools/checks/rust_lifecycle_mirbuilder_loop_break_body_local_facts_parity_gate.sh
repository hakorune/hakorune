#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="rust-lifecycle-mirbuilder-loop-break-body-local-facts-parity-gate"

source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

FIXTURE="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-loop-break-body-local-facts-rust-oracle-v0.json"
HAKO_IMPL="$ROOT_DIR/lang/src/compiler/lib/loop_break_body_local_facts.hako"
HAKO_BIN="$ROOT_DIR/tools/bin/hako"

guard_require_command "$TAG" python3
guard_require_files "$TAG" "$FIXTURE" "$HAKO_IMPL" "$HAKO_BIN"

TMP_DIR="$(mktemp -d /tmp/hakorune-loop-break-body-local-facts-parity.XXXXXX)"
cleanup() { rm -rf "$TMP_DIR" >/dev/null 2>&1 || true; }
trap cleanup EXIT

APP="$TMP_DIR/loop_break_body_local_facts_parity.hako"
EXPECTED="$TMP_DIR/expected.txt"
ACTUAL="$TMP_DIR/actual.txt"
EXE="$TMP_DIR/loop_break_body_local_facts_parity.exe"
EMIT_LOG="$TMP_DIR/emit.log"

python3 - "$FIXTURE" "$APP" "$EXPECTED" <<'PY'
import json
import sys
from pathlib import Path

fixture = json.loads(Path(sys.argv[1]).read_text(encoding="utf-8"))
app = Path(sys.argv[2])
expected = Path(sys.argv[3])
rows = fixture["rows"]
expected.write_text("\n".join(row["expected_summary"] for row in rows) + "\n", encoding="utf-8")

calls = []
for row in rows:
    calls.append(
        "    print(LoopBreakBodyLocalFactsBox.build_summary("
        f"{json.dumps(row['condition_token'])}, {json.dumps(row['body_local_token'])}))"
    )

source = "\n".join([
    "using lang.compiler.lib.loop_break_body_local_facts as LoopBreakBodyLocalFactsBox",
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

bash "$HAKO_BIN" --backend mir --verify "$HAKO_IMPL" >/dev/null

if ! bash "$HAKO_BIN" --backend mir --emit-exe "$EXE" "$APP" >"$EMIT_LOG" 2>&1; then
  tail -n 120 "$EMIT_LOG" || true
  guard_fail "$TAG" "failed to emit parity executable"
fi

chmod +x "$EXE"
"$EXE" >"$ACTUAL.raw"

python3 - "$EXPECTED" "$ACTUAL.raw" "$ACTUAL" <<'PY'
import sys
from pathlib import Path

expected = Path(sys.argv[1]).read_text(encoding="utf-8").splitlines()
raw = Path(sys.argv[2]).read_text(encoding="utf-8").splitlines()
actual_path = Path(sys.argv[3])
actual = [line.strip() for line in raw if line.strip() and not line.startswith("Result:")]
actual_path.write_text("\n".join(actual) + "\n", encoding="utf-8")
if actual != expected:
    print("[loop-break-body-local-facts/parity] mismatch")
    for idx in range(max(len(expected), len(actual))):
        exp = expected[idx] if idx < len(expected) else "<missing>"
        got = actual[idx] if idx < len(actual) else "<missing>"
        if exp != got:
            print(f"row={idx} expected={exp!r} actual={got!r}")
    raise SystemExit(1)
PY

cat <<'REPORT'
output_contract=rust-lifecycle-mirbuilder-loop-break-body-local-facts-parity-gate-v0
owner=loop_break_body_local_facts
input_contract=BackendSafeLoopBreakBodyLocalFactsTokenSnapshotV1
rust_oracle_fixture=mirbuilder-loop-break-body-local-facts-rust-oracle-v0.json
hako_implementation=lang/src/compiler/lib/loop_break_body_local_facts.hako
parity_rows=6
parity_status=green
source_selfhost_claim=0
hako_adopted_decision=0
full_ast_traversal_adopted=0
loop_break_subset_dispatch_migrated=0
break_if_analysis_migrated=0
loop_increment_extraction_migrated=0
synthetic_break_condition_construction_migrated=0
route_selection_migration=0
backend_lowering_migration=0
mir_mutation_migration=0
id_allocation_migration=0
summary=ok
REPORT
