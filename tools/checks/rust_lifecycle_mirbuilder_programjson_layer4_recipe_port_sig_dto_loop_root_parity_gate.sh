#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="rust-lifecycle-mirbuilder-programjson-layer4-recipe-port-sig-dto-loop-root-parity-gate"

source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

FIXTURE="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-programjson-layer4-recipe-port-sig-dto-loop-root-parity-v0.json"
SOURCE_FIXTURE="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-programjson-layer4-loop-recipe-dto-expanded-return-parity-v0.json"
SNAPSHOT_IMPL="$ROOT_DIR/lang/src/compiler/mirbuilder/program_json_recipe_port_sig_dto_snapshot.hako"
SELECTION_GUARD="$ROOT_DIR/tools/checks/rust_lifecycle_mirbuilder_programjson_layer4_next_recipe_dto_capability_selection_rerun_002_guard.sh"
PORT_SIG_CONTRACT="$ROOT_DIR/tools/checks/hako_aot_programjson_recipe_verifier_port_sig_result_contract_guard.sh"
HAKO_BIN="$ROOT_DIR/tools/bin/hako"

guard_require_command "$TAG" python3
guard_require_command "$TAG" timeout
guard_require_files "$TAG" "$FIXTURE" "$SOURCE_FIXTURE" "$SNAPSHOT_IMPL" "$SELECTION_GUARD" "$PORT_SIG_CONTRACT" "$HAKO_BIN"

SELECTION_OUT="$(bash "$SELECTION_GUARD")"
if ! grep -q '^summary=ok$' <<<"$SELECTION_OUT"; then
  printf '%s\n' "$SELECTION_OUT" >&2
  guard_fail "$TAG" "Recipe PortSig loop-root capability selection is not green"
fi

CONTRACT_OUT="$(bash "$PORT_SIG_CONTRACT")"
if ! grep -q '^recipe_verifier_port_sig_aot_call_fixed=1$' <<<"$CONTRACT_OUT"; then
  printf '%s\n' "$CONTRACT_OUT" >&2
  guard_fail "$TAG" "RecipeVerifier / PortSig AOT route contract is not green"
fi

TMP_DIR="$(mktemp -d /tmp/hakorune-layer4-recipe-port-sig-loop-root.XXXXXX)"
cleanup() { rm -rf "$TMP_DIR" >/dev/null 2>&1 || true; }
trap cleanup EXIT

APP="$TMP_DIR/layer4_recipe_port_sig_loop_root_probe.hako"
MIR_JSON="$TMP_DIR/layer4_recipe_port_sig_loop_root_probe.mir.json"
EXE="$TMP_DIR/layer4_recipe_port_sig_loop_root_probe.exe"
EXPECTED="$TMP_DIR/expected.txt"
BUILD_LOG="$TMP_DIR/build.log"
RUN_OUT="$TMP_DIR/run.out"
RUN_FILTERED="$TMP_DIR/run.filtered"
RUN_ERR="$TMP_DIR/run.err"

python3 - "$FIXTURE" "$SOURCE_FIXTURE" "$APP" "$EXPECTED" <<'PY'
import json
import sys
from pathlib import Path

fixture = json.loads(Path(sys.argv[1]).read_text(encoding="utf-8"))
source_fixture = json.loads(Path(sys.argv[2]).read_text(encoding="utf-8"))
app = Path(sys.argv[3])
expected = Path(sys.argv[4])

if fixture.get("kind") != "MirBuilderProgramJsonLayer4RecipePortSigDtoLoopRootParityV1":
    raise SystemExit("bad fixture kind")
if fixture.get("token") != "MIRBUILDER-PROGRAMJSON-LAYER4-RECIPE-PORT-SIG-DTO-LOOP-ROOT-PARITY-001":
    raise SystemExit("bad fixture token")
if source_fixture.get("kind") != "MirBuilderProgramJsonLayer4LoopRecipeDtoExpandedReturnParityV1":
    raise SystemExit("bad source fixture kind")

acceptance = fixture.get("acceptance") or {}
for key, value in {
    "programjson_traversal_used": 1,
    "structured_recipe_dto_constructed": 1,
    "recipe_verifier_used": 1,
    "recipe_port_sig_snapshot_used": 1,
    "loop_root_children_supported": 1,
    "route_selection": 0,
    "prebuilt_token_snapshot_input": 0,
    "string_only_facade": 0,
    "mir_json_route_green": 1,
    "runtime_parity_green": 1,
    "expanded_rows": 6,
    "selection_prerequisite_green": 1,
    "seq_loop_root_prerequisite_green": 1,
}.items():
    if acceptance.get(key) != value:
        raise SystemExit(f"bad acceptance field: {key}")

claims = fixture.get("claims") or {}
for key, value in claims.items():
    if value != 0:
        raise SystemExit(f"forbidden claim drift: {key}")

source_rows = fixture.get("source_rows") or {}
covered = source_rows.get("covered_rows") or []
if len(covered) != 6:
    raise SystemExit("Recipe PortSig loop-root parity requires exactly 6 covered rows")
expected_summary = source_rows.get("expected_summary")
if expected_summary != "snapshot_kind=RecipePortSigDtoSnapshotV1;err=0;matched=1;def_count=1;update_count=2":
    raise SystemExit("bad expected summary")

source_by_id = {row["row_id"]: row for row in source_fixture.get("rows") or []}
calls = []
expected_rows = []
for row_id in covered:
    row = source_by_id.get(row_id)
    if row is None:
        raise SystemExit(f"missing source row: {row_id}")
    program_json = json.dumps(row["program_json"], separators=(",", ":"), ensure_ascii=False)
    calls.append(
        "    print("
        + json.dumps(f"dto:{row_id}:")
        + " + ProgramJsonRecipePortSigDtoSnapshotBox.build_summary("
        + json.dumps(program_json)
        + "))"
    )
    expected_rows.append("dto:" + row_id + ":" + expected_summary)

source = "\n".join([
    "using lang.compiler.mirbuilder.program_json_recipe_port_sig_dto_snapshot as ProgramJsonRecipePortSigDtoSnapshotBox",
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
    "RecipeVerifierBox.verify(root",
    "RecipePortSigBox.snapshot(port_sig)",
    "def_count",
    "update_count",
]:
    if needle not in text:
        raise SystemExit(f"missing Recipe PortSig DTO implementation token: {needle}")
for forbidden in ["emit_mir", "new_backend_route", "RecipeMatcher", "ASTNode"]:
    if forbidden in text:
        raise SystemExit(f"forbidden implementation token: {forbidden}")
PY

bash "$HAKO_BIN" --backend mir --verify "$SNAPSHOT_IMPL" >/dev/null

if ! timeout 90s bash "$HAKO_BIN" --backend mir --emit-mir-json "$MIR_JSON" "$APP" >"$BUILD_LOG" 2>&1; then
  tail -n 160 "$BUILD_LOG" || true
  guard_fail "$TAG" "failed to emit MIR JSON for Recipe PortSig loop-root probe"
fi

python3 - "$MIR_JSON" <<'PY'
import json
import sys
from pathlib import Path

data = json.loads(Path(sys.argv[1]).read_text(encoding="utf-8"))
functions = data.get("functions") or []
main = next((fn for fn in functions if fn.get("name") == "main"), None)
snapshot = next((fn for fn in functions if fn.get("name") == "ProgramJsonRecipePortSigDtoSnapshotBox.build_summary/1"), None)
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

main_routes = [row for row in route_rows(main) if row.get("symbol") == "ProgramJsonRecipePortSigDtoSnapshotBox.build_summary/1"]
if len(main_routes) < 6:
    raise SystemExit("main does not call snapshot once per loop-root fixture row")
for row in main_routes:
    if row.get("tier") != "DirectAbi" or row.get("return_shape") != "string_handle":
        raise SystemExit(f"snapshot route is not DirectAbi/string_handle: {row}")

bad = [row for row in route_rows(snapshot) if row.get("tier") == "Unsupported" or row.get("reason")]
if bad:
    raise SystemExit(f"snapshot function has unsupported routes: {bad[:5]}")

symbols = {row.get("symbol") for row in route_rows(snapshot)}
for symbol in [
    "ProgramJsonV0PhaseStateBox.parse/2",
    "RecipeVerifierBox.verify/2",
    "RecipePortSigBox.snapshot/1",
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
output_contract=rust-lifecycle-mirbuilder-programjson-layer4-recipe-port-sig-dto-loop-root-parity-gate-v0
token=MIRBUILDER-PROGRAMJSON-LAYER4-RECIPE-PORT-SIG-DTO-LOOP-ROOT-PARITY-001
owner=ProgramJsonRecipePortSigDtoSnapshotV1
expanded_rows=6
programjson_traversal_used=1
structured_recipe_dto_constructed=1
recipe_verifier_used=1
recipe_port_sig_snapshot_used=1
loop_root_children_supported=1
route_selection=0
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
