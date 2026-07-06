#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="rust-lifecycle-mirbuilder-programjson-recipebodies-runtime-publication-bridge-gate"

source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

FIXTURE="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-programjson-recipebodies-runtime-publication-bridge-v0.json"
EXPANDED_FIXTURE="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-programjson-recipebodies-verifier-boundary-expanded-dto-coverage-parity-v0.json"
SELECTION_GUARD="$ROOT_DIR/tools/checks/rust_lifecycle_mirbuilder_programjson_recipebodies_after_expanded_dto_coverage_next_contract_selection_guard.sh"
IMPL="$ROOT_DIR/lang/src/compiler/mirbuilder/program_json_recipebodies_runtime_publication_bridge.hako"
HAKO_BIN="$ROOT_DIR/tools/bin/hako"

guard_require_command "$TAG" python3
guard_require_command "$TAG" timeout
guard_require_files "$TAG" "$FIXTURE" "$EXPANDED_FIXTURE" "$SELECTION_GUARD" "$IMPL" "$HAKO_BIN"

SELECTION_OUT="$(guard_cached_run "$TAG" bash "$SELECTION_GUARD")"
if ! grep -q '^selected_next_card=MIRBUILDER-PROGRAMJSON-RECIPEBODIES-RUNTIME-PUBLICATION-BRIDGE-001$' <<<"$SELECTION_OUT"; then
  printf '%s\n' "$SELECTION_OUT" >&2
  guard_fail "$TAG" "selection guard does not select runtime publication bridge"
fi

TMP_DIR="$(mktemp -d /tmp/hakorune-recipebodies-publication-bridge.XXXXXX)"
cleanup() { rm -rf "$TMP_DIR" >/dev/null 2>&1 || true; }
trap cleanup EXIT

APP="$TMP_DIR/recipebodies_publication_bridge_probe.hako"
MIR_JSON="$TMP_DIR/recipebodies_publication_bridge_probe.mir.json"
EXE="$TMP_DIR/recipebodies_publication_bridge_probe.exe"
EXPECTED="$TMP_DIR/expected.txt"
BUILD_LOG="$TMP_DIR/build.log"
RUN_OUT="$TMP_DIR/run.out"
RUN_FILTERED="$TMP_DIR/run.filtered"
RUN_ERR="$TMP_DIR/run.err"

python3 - "$FIXTURE" "$EXPANDED_FIXTURE" "$APP" "$EXPECTED" <<'PY'
import json
import sys
from pathlib import Path

fixture = json.loads(Path(sys.argv[1]).read_text(encoding="utf-8"))
expanded = json.loads(Path(sys.argv[2]).read_text(encoding="utf-8"))
app = Path(sys.argv[3])
expected = Path(sys.argv[4])

if fixture.get("kind") != "MirBuilderProgramJsonRecipeBodiesRuntimePublicationBridgeV1":
    raise SystemExit("bad fixture kind")
if fixture.get("token") != "MIRBUILDER-PROGRAMJSON-RECIPEBODIES-RUNTIME-PUBLICATION-BRIDGE-001":
    raise SystemExit("bad fixture token")

contract = fixture.get("publication_contract") or {}
for key, value in {
    "output": "RecipeBodiesPublicationSnapshotV1",
    "publication_mode": "read_only_snapshot",
    "result_map_required": True,
    "directabi_allowed": True,
    "runtime_route_switch": False,
}.items():
    if contract.get(key) != value:
        raise SystemExit(f"publication contract drift: {key}")

claims = fixture.get("claims") or {}
for key, expected_value in {
    "runtime_recipe_bodies_publication_bridge": 1,
    "read_only_publication_snapshot": 1,
}.items():
    if claims.get(key) != expected_value:
        raise SystemExit(f"missing positive claim: {key}")
for key, value in claims.items():
    if key in {"runtime_recipe_bodies_publication_bridge", "read_only_publication_snapshot"}:
        continue
    if value != 0:
        raise SystemExit(f"forbidden claim drift: {key}")

rows = fixture.get("rows") or []
expanded_rows = expanded.get("rows") or []
if [row.get("row_id") for row in rows] != [row.get("row_id") for row in expanded_rows]:
    raise SystemExit("publication rows must match expanded DTO coverage rows")

program_json_by_id = {row["row_id"]: row["program_json"] for row in expanded_rows}
lines = [
    "using lang.compiler.mirbuilder.program_json_recipebodies_runtime_publication_bridge as ProgramJsonRecipeBodiesRuntimePublicationBridgeBox",
    "",
    "static box Main {",
    "  main() {",
]
expected_lines = []
for index, row in enumerate(rows):
    row_id = row["row_id"]
    summary = row["expected_summary"]
    for token in [
        "snapshot_kind=RecipeBodiesPublicationSnapshotV1;",
        ";ok=1",
        ";verified_recipe_present=1",
        ";verifier_boundary_used=1",
        ";body_count=4",
        ";def_count=1",
        ";update_count=2",
        ";readonly=1",
        ";recipe_matcher_executed=0",
    ]:
        if token not in summary:
            raise SystemExit(f"missing expected summary token {token}: {row_id}")
    program_json = json.dumps(program_json_by_id[row_id], separators=(",", ":"), ensure_ascii=False)
    var_name = f"publication{index}"
    lines.append(
        "    local "
        + var_name
        + " = ProgramJsonRecipeBodiesRuntimePublicationBridgeBox.build_publication("
        + json.dumps(program_json)
        + ")"
    )
    lines.append(
        "    print(\"publication:"
        + row_id
        + ":\" + ProgramJsonRecipeBodiesRuntimePublicationBridgeBox.publication_summary("
        + var_name
        + "))"
    )
    expected_lines.append("publication:" + row_id + ":" + summary + "\n")
lines.extend(["    return 0", "  }", "}", ""])
app.write_text("\n".join(lines), encoding="utf-8")
expected.write_text("".join(expected_lines), encoding="utf-8")
PY

python3 - "$IMPL" <<'PY'
import sys
from pathlib import Path

text = Path(sys.argv[1]).read_text(encoding="utf-8")
for needle in [
    "build_publication(program_json): MapBox",
    "ProgramJsonRecipeBodiesRecursiveNestedArenaBuilderBox.build_arena(program_json)",
    "RecipeVerifierBox.verify(root",
    "RecipePortSigBox.snapshot(port_sig)",
    "\"readonly\" => 1",
    "\"recipe_matcher_executed\" => 0",
]:
    if needle not in text:
        raise SystemExit(f"missing publication implementation token: {needle}")
for forbidden in ["RecipeMatcherBox", "emit_mir", "new_backend_route", "ASTNode"]:
    if forbidden in text:
        raise SystemExit(f"forbidden implementation token: {forbidden}")
PY

bash "$HAKO_BIN" --backend mir --verify "$IMPL" >/dev/null

if ! timeout 90s bash "$HAKO_BIN" --backend mir --emit-mir-json "$MIR_JSON" "$APP" >"$BUILD_LOG" 2>&1; then
  tail -n 160 "$BUILD_LOG" || true
  guard_fail "$TAG" "failed to emit MIR JSON for runtime publication bridge probe"
fi

python3 - "$MIR_JSON" <<'PY'
import json
import sys
from pathlib import Path

data = json.loads(Path(sys.argv[1]).read_text(encoding="utf-8"))
functions = data.get("functions") or []
main = next((fn for fn in functions if fn.get("name") == "main"), None)
publication = next((fn for fn in functions if fn.get("name") == "ProgramJsonRecipeBodiesRuntimePublicationBridgeBox.build_publication/1"), None)
summary = next((fn for fn in functions if fn.get("name") == "ProgramJsonRecipeBodiesRuntimePublicationBridgeBox.publication_summary/1"), None)
if main is None or publication is None or summary is None:
    raise SystemExit("required functions missing")

def route_rows(fn):
    metadata = fn.get("metadata") or {}
    rows = []
    for key in ("global_call_routes", "lowering_plan"):
        value = metadata.get(key) or []
        if isinstance(value, list):
            rows.extend(row for row in value if isinstance(row, dict))
    return rows

main_publication_routes = [
    row for row in route_rows(main)
    if row.get("symbol") == "ProgramJsonRecipeBodiesRuntimePublicationBridgeBox.build_publication/1"
]
if len(main_publication_routes) < 2:
    raise SystemExit("main must call build_publication once per row")
for row in main_publication_routes:
    if row.get("tier") != "DirectAbi" or row.get("return_shape") != "map_handle":
        raise SystemExit(f"publication route is not DirectAbi/map_handle: {row}")

main_summary_routes = [
    row for row in route_rows(main)
    if row.get("symbol") == "ProgramJsonRecipeBodiesRuntimePublicationBridgeBox.publication_summary/1"
]
if len(main_summary_routes) < 2:
    raise SystemExit("main must summarize each publication row")

for fn in [publication, summary]:
    bad = [row for row in route_rows(fn) if row.get("tier") == "Unsupported" or row.get("reason")]
    if bad:
        raise SystemExit(f"{fn.get('name')} has unsupported routes: {bad[:5]}")

symbols = {row.get("symbol") for row in route_rows(publication)}
for symbol in [
    "ProgramJsonRecipeBodiesRecursiveNestedArenaBuilderBox.build_arena/1",
    "ProgramJsonV0PhaseStateBox.parse/2",
    "RecipeVerifierBox.verify/2",
    "RecipePortSigBox.snapshot/1",
]:
    if symbol not in symbols:
        raise SystemExit(f"missing publication route: {symbol}")
if any("RecipeMatcher" in str(row.get("symbol")) for row in route_rows(publication) + route_rows(summary)):
    raise SystemExit("RecipeMatcher must not be called by publication bridge")
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
  guard_fail "$TAG" "runtime publication parity mismatch"
fi

cat <<'REPORT'
output_contract=rust-lifecycle-mirbuilder-programjson-recipebodies-runtime-publication-bridge-gate-v0
token=MIRBUILDER-PROGRAMJSON-RECIPEBODIES-RUNTIME-PUBLICATION-BRIDGE-001
owner=ProgramJsonRecipeBodiesRuntimePublicationBridgeBox
row_count=2
runtime_recipe_bodies_publication_bridge=1
read_only_publication_snapshot=1
directabi_map_handle_publication=1
verified_recipe_present=1
verifier_boundary_used=1
readonly=1
recipe_matcher_executed=0
runtime_recipe_bodies_authority=0
full_recipe_matcher_execution=0
route_selection=0
mir_lowering=0
mir_mutation=0
id_allocation=0
runtime_route_switch=0
runtime_fallback=0
source_selfhost_claim=0
summary=ok
REPORT
