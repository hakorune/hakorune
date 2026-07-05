#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="rust-lifecycle-mirbuilder-programjson-loop-cond-continue-with-return-snapshot-parity-gate"

source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

FIXTURE="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-programjson-loop-cond-continue-with-return-snapshot-parity-v0.json"
SNAPSHOT_IMPL="$ROOT_DIR/lang/src/compiler/mirbuilder/program_json_loop_cond_continue_with_return_snapshot.hako"
SCANNER_IMPL="$ROOT_DIR/lang/src/compiler/mirbuilder/program_json_v0_scanner_box.hako"
PLAN_RULE_IMPL="$ROOT_DIR/lang/src/compiler/lib/loop_cond_continue_with_return_plan_rule.hako"
LABEL_IMPL="$ROOT_DIR/lang/src/compiler/lib/planner_rule_label_formatter.hako"
HAKO_BIN="$ROOT_DIR/tools/bin/hako"

guard_require_command "$TAG" python3
guard_require_files "$TAG" "$FIXTURE" "$SNAPSHOT_IMPL" "$SCANNER_IMPL" "$PLAN_RULE_IMPL" "$LABEL_IMPL" "$HAKO_BIN"

TMP_DIR="$(mktemp -d /tmp/hakorune-programjson-loop-cond-continue-snapshot-parity.XXXXXX)"
cleanup() { rm -rf "$TMP_DIR" >/dev/null 2>&1 || true; }
trap cleanup EXIT

APP="$TMP_DIR/programjson_loop_cond_continue_with_return_snapshot_parity.hako"
EXPECTED="$TMP_DIR/expected.json"
RUN_LOG="$TMP_DIR/run.log"
EXE="$TMP_DIR/programjson_loop_cond_continue_with_return_snapshot_parity.exe"
EMIT_LOG="$TMP_DIR/emit.log"

python3 - "$FIXTURE" "$APP" "$EXPECTED" <<'PY'
import json
import sys
from pathlib import Path

fixture = json.loads(Path(sys.argv[1]).read_text(encoding="utf-8"))
app = Path(sys.argv[2])
expected = Path(sys.argv[3])

calls = []
expected_rows = []
for row in fixture["rows"]:
    row_id = row["row_id"]
    program_json = json.dumps(row["program_json"], separators=(",", ":"), ensure_ascii=False)
    calls.append(
        "    print("
        + json.dumps(f"snapshot:{row_id}:")
        + " + ProgramJsonLoopCondContinueWithReturnSnapshotBox.build_summary("
        + json.dumps(program_json)
        + "))"
    )
    expected_rows.append(
        {
            "kind": "snapshot",
            "row_id": row_id,
            "expected": row["rust_astnode_token_oracle"]["expected_summary"],
        }
    )
    facade_tokens = row.get("facade_tokens")
    if facade_tokens:
        calls.append(
            "    print("
            + json.dumps(f"facade:{row_id}:")
            + " + LoopCondContinueWithReturnPlanRuleBox.build_summary("
            + json.dumps(facade_tokens["rule_order_token"])
            + ", "
            + json.dumps(facade_tokens["planner_present_token"])
            + ", "
            + json.dumps(facade_tokens["candidate_rule_token"])
            + ", "
            + json.dumps(facade_tokens["recipe_only_token"])
            + "))"
        )
        expected_rows.append(
            {
                "kind": "facade",
                "row_id": row_id,
                "expected": row["rust_astnode_facade_oracle"]["expected_summary"],
            }
        )

source = "\n".join(
    [
        "using lang.compiler.mirbuilder.program_json_loop_cond_continue_with_return_snapshot as ProgramJsonLoopCondContinueWithReturnSnapshotBox",
        "using lang.compiler.lib.loop_cond_continue_with_return_plan_rule as LoopCondContinueWithReturnPlanRuleBox",
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
expected.write_text(json.dumps(expected_rows, indent=2, sort_keys=True) + "\n", encoding="utf-8")
PY

bash "$HAKO_BIN" --backend mir --verify "$SCANNER_IMPL" >/dev/null
bash "$HAKO_BIN" --backend mir --verify "$SNAPSHOT_IMPL" >/dev/null
bash "$HAKO_BIN" --backend mir --verify "$LABEL_IMPL" >/dev/null
bash "$HAKO_BIN" --backend mir --verify "$PLAN_RULE_IMPL" >/dev/null

if ! bash "$HAKO_BIN" --backend mir --emit-exe "$EXE" "$APP" >"$EMIT_LOG" 2>&1; then
  tail -n 160 "$EMIT_LOG" || true
  guard_fail "$TAG" "failed to emit ProgramJSON snapshot parity executable"
fi

if ! "$EXE" >"$RUN_LOG" 2>&1; then
  tail -n 160 "$RUN_LOG" || true
  guard_fail "$TAG" "failed to run ProgramJSON snapshot parity executable"
fi

python3 - "$EXPECTED" "$RUN_LOG" <<'PY'
import json
import sys
from pathlib import Path

expected_rows = json.loads(Path(sys.argv[1]).read_text(encoding="utf-8"))
expected = {(row["kind"], row["row_id"]): row["expected"] for row in expected_rows}
actual = {}
for raw in Path(sys.argv[2]).read_text(encoding="utf-8").splitlines():
    line = raw.strip()
    if not line or line.startswith("Result:"):
        continue
    parts = line.split(":", 2)
    if len(parts) != 3:
        print(f"bad output line: {line!r}", file=sys.stderr)
        raise SystemExit(1)
    actual[(parts[0], parts[1])] = parts[2]

if set(actual) != set(expected):
    print(f"expected keys={sorted(expected)}", file=sys.stderr)
    print(f"actual keys={sorted(actual)}", file=sys.stderr)
    raise SystemExit(1)

for key, exp in expected.items():
    got = actual[key]
    if got != exp:
        print(f"mismatch key={key} expected={exp!r} actual={got!r}", file=sys.stderr)
        raise SystemExit(1)
PY

cat <<'REPORT'
output_contract=rust-lifecycle-mirbuilder-programjson-loop-cond-continue-with-return-snapshot-parity-gate-v0
owner=ProgramJsonLoopCondContinueWithReturnSnapshotV1
input_contract=LoopCondContinueThenReturnMinimalV1
fixture=mirbuilder-programjson-loop-cond-continue-with-return-snapshot-parity-v0.json
hako_snapshot_implementation=lang/src/compiler/mirbuilder/program_json_loop_cond_continue_with_return_snapshot.hako
same_facade=lang/src/compiler/lib/loop_cond_continue_with_return_plan_rule.hako
parity_rows=10
facade_parity_rows=1
execution_backend=aot
aot_execution_status=green
token_snapshot_parity=green
same_facade_output_parity=green
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
