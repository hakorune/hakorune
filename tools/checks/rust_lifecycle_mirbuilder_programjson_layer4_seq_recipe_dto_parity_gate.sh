#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="rust-lifecycle-mirbuilder-programjson-layer4-seq-recipe-dto-parity-gate"

source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

FIXTURE="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-programjson-layer4-seq-recipe-dto-parity-v0.json"
SNAPSHOT_IMPL="$ROOT_DIR/lang/src/compiler/mirbuilder/program_json_seq_recipe_dto_snapshot.hako"
PHASE_STATE_IMPL="$ROOT_DIR/lang/src/compiler/mirbuilder/program_json_v0_phase_state_box.hako"
HAKO_BIN="$ROOT_DIR/tools/bin/hako"

guard_require_command "$TAG" python3
guard_require_command "$TAG" timeout
guard_require_files "$TAG" "$FIXTURE" "$SNAPSHOT_IMPL" "$PHASE_STATE_IMPL" "$HAKO_BIN"

TMP_DIR="$(mktemp -d /tmp/hakorune-layer4-seq-recipe-dto.XXXXXX)"
cleanup() { rm -rf "$TMP_DIR" >/dev/null 2>&1 || true; }
trap cleanup EXIT

APP="$TMP_DIR/layer4_seq_recipe_dto_probe.hako"
MIR_JSON="$TMP_DIR/layer4_seq_recipe_dto_probe.mir.json"
EXE="$TMP_DIR/layer4_seq_recipe_dto_probe.exe"
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

if fixture.get("kind") != "MirBuilderProgramJsonLayer4SeqRecipeDtoParityV1":
    raise SystemExit("bad fixture kind")
if fixture.get("token") != "MIRBUILDER-PROGRAMJSON-LAYER4-SEQ-RECIPE-DTO-PARITY-001":
    raise SystemExit("bad fixture token")

rows = fixture.get("rows") or []
if len(rows) < 4:
    raise SystemExit("Layer4 Seq Recipe DTO parity requires at least 4 rows")

acceptance = fixture.get("acceptance") or {}
required_acceptance = {
    "programjson_traversal_used": 1,
    "structured_recipe_dto_constructed": 1,
    "prebuilt_token_snapshot_input": 0,
    "string_only_facade": 0,
    "mir_json_route_green": 1,
    "runtime_parity_green": 1,
}
for key, value in required_acceptance.items():
    if acceptance.get(key) != value:
        raise SystemExit(f"bad acceptance field: {key}")

selection = fixture.get("selection") or {}
if "assignment_print_deferred_reason" not in selection:
    raise SystemExit("Assignment/Print deferral reason must be explicit")

claims = fixture.get("claims") or {}
for key, value in claims.items():
    if value != 0:
        raise SystemExit(f"forbidden claim drift: {key}")

calls = []
expected_rows = []
for row in rows:
    row_id = row["row_id"]
    summary = row["rust_oracle_expected_summary"]
    if not summary.startswith("snapshot_kind=SeqRecipeDtoSnapshotV1;"):
        raise SystemExit(f"bad summary prefix: {row_id}")
    if "empty_body" not in row_id:
        for token in [";err=0", ";matched=1", ";item_count=", ";seq_sig=", ";shape_kind="]:
            if token not in summary:
                raise SystemExit(f"missing expected DTO token {token}: {row_id}")

    program_json = json.dumps(row["program_json"], separators=(",", ":"), ensure_ascii=False)
    calls.append(
        "    print("
        + json.dumps(f"dto:{row_id}:")
        + " + ProgramJsonSeqRecipeDtoSnapshotBox.build_summary("
        + json.dumps(program_json)
        + "))"
    )
    expected_rows.append("dto:" + row_id + ":" + summary)

source = "\n".join(
    [
        "using lang.compiler.mirbuilder.program_json_seq_recipe_dto_snapshot as ProgramJsonSeqRecipeDtoSnapshotBox",
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
expected.write_text("\n".join(expected_rows) + "\n", encoding="utf-8")
PY

python3 - "$SNAPSHOT_IMPL" <<'PY'
import sys
from pathlib import Path

text = Path(sys.argv[1]).read_text(encoding="utf-8")
required = [
    "ProgramJsonV0PhaseStateBox.parse(program_json",
    "recipe_root",
    "RecipeItemBox.kind_is(root, \"Seq\")",
    "seq_sig",
    "phase1_return_int",
    "phase12_return_newbox",
    "phase2_local_return_var",
]
for needle in required:
    if needle not in text:
        raise SystemExit(f"missing snapshot implementation token: {needle}")
for forbidden in [
    "source contains",
    "program_json contains",
    "rust_astnode",
    "ASTNode",
    "emit_mir",
    "new_backend_route",
]:
    if forbidden in text:
        raise SystemExit(f"forbidden snapshot implementation token: {forbidden}")
PY

bash "$HAKO_BIN" --backend mir --verify "$SNAPSHOT_IMPL" >/dev/null
bash "$HAKO_BIN" --backend mir --verify "$PHASE_STATE_IMPL" >/dev/null

if ! timeout 90s bash "$HAKO_BIN" --backend mir --emit-mir-json "$MIR_JSON" "$APP" >"$BUILD_LOG" 2>&1; then
  tail -n 160 "$BUILD_LOG" || true
  guard_fail "$TAG" "failed to emit MIR JSON for Layer4 Seq Recipe DTO probe"
fi

python3 - "$MIR_JSON" <<'PY'
import json
import sys
from pathlib import Path

data = json.loads(Path(sys.argv[1]).read_text(encoding="utf-8"))
functions = data.get("functions") or []
main = next((fn for fn in functions if fn.get("name") == "main"), None)
snapshot = next((fn for fn in functions if fn.get("name") == "ProgramJsonSeqRecipeDtoSnapshotBox.build_summary/1"), None)
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

main_routes = [row for row in route_rows(main) if row.get("symbol") == "ProgramJsonSeqRecipeDtoSnapshotBox.build_summary/1"]
if len(main_routes) < 4:
    raise SystemExit("main does not call snapshot once per fixture row")
for row in main_routes:
    if row.get("tier") != "DirectAbi":
        raise SystemExit(f"snapshot route is not DirectAbi: {row}")
    if row.get("return_shape") != "string_handle":
        raise SystemExit(f"snapshot route does not publish string_handle: {row}")
    if row.get("target_shape") != "generic_pure_string_body":
        raise SystemExit(f"snapshot target shape is not generic_pure_string_body: {row}")
    if row.get("reason"):
        raise SystemExit(f"snapshot route has reason: {row}")

bad = []
snapshot_routes = route_rows(snapshot)
for row in snapshot_routes:
    if row.get("tier") == "Unsupported" or row.get("reason"):
        bad.append(row)
if bad:
    raise SystemExit(f"snapshot function has unsupported routes: {bad[:5]}")

phase_routes = [row for row in snapshot_routes if row.get("symbol") == "ProgramJsonV0PhaseStateBox.parse/2"]
if not phase_routes:
    raise SystemExit("snapshot does not call PhaseState parse")
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
output_contract=rust-lifecycle-mirbuilder-programjson-layer4-seq-recipe-dto-parity-gate-v0
token=MIRBUILDER-PROGRAMJSON-LAYER4-SEQ-RECIPE-DTO-PARITY-001
owner=ProgramJsonSeqRecipeDtoSnapshotV1
programjson_traversal_used=1
structured_recipe_dto_constructed=1
mir_json_route_green=1
runtime_parity_green=1
runtime_route_switch=0
full_recipe_matcher_execution=0
mir_mutation=0
id_allocation=0
backend_lowering_claim=0
source_selfhost_claim=0
summary=ok
REPORT
