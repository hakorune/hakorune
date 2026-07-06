#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="rust-lifecycle-mirbuilder-programjson-layer4-loop-recipe-dto-parity-gate"

source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

FIXTURE="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-programjson-layer4-loop-recipe-dto-parity-v0.json"
SNAPSHOT_IMPL="$ROOT_DIR/lang/src/compiler/mirbuilder/program_json_loop_recipe_dto_snapshot.hako"
PHASE_STATE_IMPL="$ROOT_DIR/lang/src/compiler/mirbuilder/program_json_v0_phase_state_box.hako"
HAKO_BIN="$ROOT_DIR/tools/bin/hako"

guard_require_command "$TAG" python3
guard_require_command "$TAG" timeout
guard_require_files "$TAG" "$FIXTURE" "$SNAPSHOT_IMPL" "$PHASE_STATE_IMPL" "$HAKO_BIN"

TMP_DIR="$(mktemp -d /tmp/hakorune-layer4-loop-recipe-dto.XXXXXX)"
cleanup() { rm -rf "$TMP_DIR" >/dev/null 2>&1 || true; }
trap cleanup EXIT

APP="$TMP_DIR/layer4_loop_recipe_dto_probe.hako"
MIR_JSON="$TMP_DIR/layer4_loop_recipe_dto_probe.mir.json"
EMIT_LOG="$TMP_DIR/emit.log"
EXPECTED="$TMP_DIR/expected.json"

python3 - "$FIXTURE" "$APP" "$EXPECTED" <<'PY'
import json
import sys
from pathlib import Path

fixture = json.loads(Path(sys.argv[1]).read_text(encoding="utf-8"))
app = Path(sys.argv[2])
expected = Path(sys.argv[3])

if fixture.get("kind") != "MirBuilderProgramJsonLayer4LoopRecipeDtoParityV1":
    raise SystemExit("bad fixture kind")
if fixture.get("token") != "MIRBUILDER-PROGRAMJSON-LAYER4-LOOP-RECIPE-DTO-PARITY-001":
    raise SystemExit("bad fixture token")

rows = fixture.get("rows") or []
if len(rows) < 4:
    raise SystemExit("Layer4 loop Recipe DTO parity requires at least 4 rows")

acceptance = fixture.get("acceptance") or {}
required_acceptance = {
    "programjson_traversal_used": 1,
    "structured_recipe_dto_constructed": 1,
    "prebuilt_token_snapshot_input": 0,
    "string_only_facade": 0,
    "mir_json_route_green": 1,
    "runtime_parity_green": 0,
}
for key, value in required_acceptance.items():
    if acceptance.get(key) != value:
        raise SystemExit(f"bad acceptance field: {key}")

claims = fixture.get("claims") or {}
for key, value in claims.items():
    if value != 0:
        raise SystemExit(f"forbidden claim drift: {key}")

for row in rows:
    summary = row.get("rust_oracle_expected_summary") or ""
    if not summary.startswith("snapshot_kind=LoopRecipeDtoSnapshotV1;"):
        raise SystemExit(f"bad expected summary prefix: {row.get('row_id')}")
    if "loop_without_local" not in row.get("row_id", ""):
        for token in [
            ";err=0",
            ";matched=1",
            ";root_len=3",
            ";loop_cond_kind=VarLtInt",
            ";loop_body_kind=",
            ";loop_step_int=",
        ]:
            if token not in summary:
                raise SystemExit(f"missing expected DTO token {token}: {row.get('row_id')}")

calls = []
expected_rows = []
for row in rows:
    program_json = json.dumps(row["program_json"], separators=(",", ":"), ensure_ascii=False)
    row_id = row["row_id"]
    calls.append(
        "    print("
        + json.dumps(f"dto:{row_id}:")
        + " + ProgramJsonLoopRecipeDtoSnapshotBox.build_summary("
        + json.dumps(program_json)
        + "))"
    )
    expected_rows.append(
        {
            "row_id": row_id,
            "expected_summary": row["rust_oracle_expected_summary"],
        }
    )

source = "\n".join(
    [
        "using lang.compiler.mirbuilder.program_json_loop_recipe_dto_snapshot as ProgramJsonLoopRecipeDtoSnapshotBox",
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

python3 - "$SNAPSHOT_IMPL" <<'PY'
import sys
from pathlib import Path

text = Path(sys.argv[1]).read_text(encoding="utf-8")
required = [
    "ProgramJsonV0PhaseStateBox.parse(program_json",
    "recipe_root",
    "RecipeItemBox.kind_is",
    "loop_cond_rhs_int",
    "loop_step_int",
    "loop_if_cond_rhs_int",
    "loop_if_then_retv",
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
        raise SystemExit(f"forbidden implementation token: {forbidden}")
if "BoxHelpers.array_len" in text:
    raise SystemExit("snapshot must not depend on BoxHelpers.array_len route")
PY

bash "$HAKO_BIN" --backend mir --verify "$SNAPSHOT_IMPL" >/dev/null
bash "$HAKO_BIN" --backend mir --verify "$PHASE_STATE_IMPL" >/dev/null

if ! timeout 90s bash "$HAKO_BIN" --backend mir --emit-mir-json "$MIR_JSON" "$APP" >"$EMIT_LOG" 2>&1; then
  tail -n 160 "$EMIT_LOG" || true
  guard_fail "$TAG" "failed to emit MIR JSON for Layer4 loop Recipe DTO probe"
fi

python3 - "$MIR_JSON" "$EXPECTED" <<'PY'
import json
import sys
from pathlib import Path

data = json.loads(Path(sys.argv[1]).read_text(encoding="utf-8"))
expected_rows = json.loads(Path(sys.argv[2]).read_text(encoding="utf-8"))

functions = data.get("functions") or []
main = next((fn for fn in functions if fn.get("name") == "main"), None)
snapshot = next((fn for fn in functions if fn.get("name") == "ProgramJsonLoopRecipeDtoSnapshotBox.build_summary/1"), None)
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
            rows.extend((key, row) for row in value if isinstance(row, dict))
    return rows

main_routes = [row for _, row in route_rows(main) if row.get("symbol") == "ProgramJsonLoopRecipeDtoSnapshotBox.build_summary/1"]
if len(main_routes) < len(expected_rows):
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

snapshot_bad = []
snapshot_routes = [row for _, row in route_rows(snapshot)]
for row in snapshot_routes:
    if row.get("tier") == "Unsupported" or row.get("reason"):
        snapshot_bad.append(row)
if snapshot_bad:
    raise SystemExit(f"snapshot function has unsupported routes: {snapshot_bad[:5]}")

phase_routes = [
    row for row in snapshot_routes
    if row.get("symbol") == "ProgramJsonV0PhaseStateBox.parse/2"
]
if not phase_routes:
    raise SystemExit("snapshot does not call PhaseState parse")
if not any(row.get("tier") == "DirectAbi" and row.get("return_shape") == "map_handle" for row in phase_routes):
    raise SystemExit(f"PhaseState parse route is not DirectAbi/map_handle: {phase_routes}")

for expected in expected_rows:
    summary = expected["expected_summary"]
    if ";err=0" in summary:
        for token in ["shape_kind=", "loop_cond_rhs_int=", "loop_step_int="]:
            if token not in summary:
                raise SystemExit(f"expected summary missing {token}: {expected['row_id']}")
PY

cat <<'REPORT'
output_contract=rust-lifecycle-mirbuilder-programjson-layer4-loop-recipe-dto-parity-gate-v0
token=MIRBUILDER-PROGRAMJSON-LAYER4-LOOP-RECIPE-DTO-PARITY-001
owner=ProgramJsonLoopRecipeDtoSnapshotV1
fixture=mirbuilder-programjson-layer4-loop-recipe-dto-parity-v0.json
hako_implementation=lang/src/compiler/mirbuilder/program_json_loop_recipe_dto_snapshot.hako
programjson_traversal_used=1
structured_recipe_dto_constructed=1
snapshot_route=DirectAbi
snapshot_return_shape=string_handle
phase_state_parse_route=DirectAbi
phase_state_parse_return_shape=map_handle
mir_json_route_green=1
runtime_parity_green=0
full_emit_exe_status=unclaimed_heavy_timeout_pending
full_recipe_matcher_execution=0
mir_mutation=0
id_allocation=0
backend_lowering_claim=0
runtime_route_switch=0
source_selfhost_claim=0
summary=ok
REPORT
