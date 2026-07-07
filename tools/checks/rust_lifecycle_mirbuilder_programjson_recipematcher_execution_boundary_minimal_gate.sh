#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="rust-lifecycle-mirbuilder-programjson-recipematcher-execution-boundary-minimal-gate"

source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

FIXTURE="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-programjson-recipematcher-execution-boundary-minimal-v0.json"
SNAPSHOT_FIXTURE="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-programjson-canonical-loop-facts-input-snapshot-mapbox-publication-bridge-v0.json"
EXPANDED_FIXTURE="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-programjson-recipebodies-verifier-boundary-expanded-dto-coverage-parity-v0.json"
SNAPSHOT_GATE="$ROOT_DIR/tools/checks/rust_lifecycle_mirbuilder_programjson_canonical_loop_facts_input_snapshot_mapbox_publication_bridge_gate.sh"
SNAPSHOT_IMPL="$ROOT_DIR/lang/src/compiler/mirbuilder/program_json_canonical_loop_facts_input_snapshot.hako"
MATCHER_IMPL="$ROOT_DIR/lang/src/compiler/mirbuilder/program_json_recipematcher_execution_boundary.hako"
HAKO_BIN="$ROOT_DIR/tools/bin/hako"

guard_require_command "$TAG" python3
guard_require_command "$TAG" timeout
guard_require_files "$TAG" "$FIXTURE" "$SNAPSHOT_FIXTURE" "$EXPANDED_FIXTURE" "$SNAPSHOT_GATE" "$SNAPSHOT_IMPL" "$MATCHER_IMPL" "$HAKO_BIN"

SNAPSHOT_OUT="$(guard_cached_run "$TAG" bash "$SNAPSHOT_GATE")"
if ! grep -q '^directabi_map_handle_publication=1$' <<<"$SNAPSHOT_OUT"; then
  printf '%s\n' "$SNAPSHOT_OUT" >&2
  guard_fail "$TAG" "CanonicalLoopFacts input snapshot publication bridge is not green"
fi

TMP_DIR="$(mktemp -d /tmp/hakorune-recipematcher-boundary.XXXXXX)"
cleanup() { rm -rf "$TMP_DIR" >/dev/null 2>&1 || true; }
trap cleanup EXIT

APP="$TMP_DIR/recipematcher_boundary_probe.hako"
MIR_JSON="$TMP_DIR/recipematcher_boundary_probe.mir.json"
EXE="$TMP_DIR/recipematcher_boundary_probe.exe"
EXPECTED="$TMP_DIR/expected.txt"
BUILD_LOG="$TMP_DIR/build.log"
RUN_OUT="$TMP_DIR/run.out"
RUN_FILTERED="$TMP_DIR/run.filtered"
RUN_ERR="$TMP_DIR/run.err"

python3 - "$FIXTURE" "$SNAPSHOT_FIXTURE" "$EXPANDED_FIXTURE" "$APP" "$EXPECTED" <<'PY'
import json
import sys
from pathlib import Path

fixture = json.loads(Path(sys.argv[1]).read_text(encoding="utf-8"))
snapshot_fixture = json.loads(Path(sys.argv[2]).read_text(encoding="utf-8"))
expanded = json.loads(Path(sys.argv[3]).read_text(encoding="utf-8"))
app = Path(sys.argv[4])
expected = Path(sys.argv[5])

if fixture.get("kind") != "MirBuilderProgramJsonRecipeMatcherExecutionBoundaryMinimalV1":
    raise SystemExit("bad fixture kind")
if fixture.get("token") != "MIRBUILDER-PROGRAMJSON-RECIPEMATCHER-EXECUTION-BOUNDARY-MINIMAL-001":
    raise SystemExit("bad fixture token")

contract = fixture.get("execution_contract") or {}
required_contract = {
    "input": "ProgramJsonCanonicalLoopFactsInputSnapshotV1 MapBox",
    "output": "ProgramJsonRecipeMatcherExecutionBoundaryResultV1 MapBox",
    "publication_mode": "read_only_observe_only",
    "result_map_required": True,
    "directabi_allowed": True,
    "recipe_matcher_input_authority": False,
    "runtime_route_switch": False,
    "route_selection": False,
    "mir_lowering": False,
}
for key, value in required_contract.items():
    if contract.get(key) != value:
        raise SystemExit(f"execution contract drift: {key}")

guard_contract = fixture.get("guard_contract") or {}
for key in [
    "aot_required",
    "mapbox_snapshot_input_required",
    "mapbox_result_contract_required",
    "directabi_map_handle_required",
    "no_object_or_void_widening",
    "no_runtime_fallback",
    "no_route_selection",
    "no_mir_lowering",
    "no_mir_mutation",
    "no_id_allocation",
    "no_runtime_route_switch",
]:
    if guard_contract.get(key) is not True:
        raise SystemExit(f"guard contract missing true: {key}")
if guard_contract.get("vm_only_main_acceptance") is not False:
    raise SystemExit("VM-only main acceptance must stay false")

claims = fixture.get("claims") or {}
for key in [
    "recipe_matcher_execution_boundary_minimal",
    "observe_only_recipe_matcher_execution",
    "recipe_matcher_input_snapshot_consumed",
]:
    if claims.get(key) != 1:
        raise SystemExit(f"missing positive claim: {key}")
for key, value in claims.items():
    if key in {
        "recipe_matcher_execution_boundary_minimal",
        "observe_only_recipe_matcher_execution",
        "recipe_matcher_input_snapshot_consumed",
    }:
        continue
    if value != 0:
        raise SystemExit(f"forbidden claim drift: {key}")

expanded_rows = expanded.get("rows") or []
program_json_by_id = {row["row_id"]: row["program_json"] for row in expanded_rows}
snapshot_rows = snapshot_fixture.get("rows") or []
snapshot_ids = [row.get("row_id") for row in snapshot_rows]
rows = fixture.get("rows") or []
row_ids = [row.get("row_id") for row in rows]
if row_ids != [
    "local_loop_body_if_branch_return",
    "local_loop_body_if_branch_return_alt_names",
]:
    raise SystemExit("unexpected row order")
if row_ids != snapshot_ids:
    raise SystemExit("matcher rows must match snapshot publication rows")

lines = [
    "using lang.compiler.mirbuilder.program_json_canonical_loop_facts_input_snapshot as ProgramJsonCanonicalLoopFactsInputSnapshotBox",
    "using lang.compiler.mirbuilder.program_json_recipematcher_execution_boundary as ProgramJsonRecipeMatcherExecutionBoundaryBox",
    "",
    "static box Main {",
    "  main() {",
]
expected_lines = []
for index, row in enumerate(rows):
    row_id = row["row_id"]
    if row_id not in program_json_by_id:
        raise SystemExit(f"expanded fixture missing row: {row_id}")
    summary = row["expected_summary"]
    for token in [
        "snapshot_kind=ProgramJsonRecipeMatcherExecutionBoundaryResultV1;",
        ";ok=1",
        ";matcher_input_consumed=1",
        ";matched=1",
        ";contract_kind=LoopWithExit",
        ";has_return=1",
        ";observe_only=1",
        ";recipe_matcher_executed=1",
        ";full_recipe_matcher_execution=0",
        ";route_selection=0",
        ";mir_lowering=0",
        ";id_allocation=0",
    ]:
        if token not in summary:
            raise SystemExit(f"missing expected summary token {token}: {row_id}")
    program_json = json.dumps(program_json_by_id[row_id], separators=(",", ":"), ensure_ascii=False)
    snap_var = f"snapshot{index}"
    match_var = f"match{index}"
    lines.append(
        "    local "
        + snap_var
        + " = ProgramJsonCanonicalLoopFactsInputSnapshotBox.build_snapshot("
        + json.dumps(program_json)
        + ")"
    )
    lines.append(
        "    local "
        + match_var
        + " = ProgramJsonRecipeMatcherExecutionBoundaryBox.match_snapshot("
        + snap_var
        + ")"
    )
    lines.append(
        "    print(\"match:"
        + row_id
        + ":\" + ProgramJsonRecipeMatcherExecutionBoundaryBox.match_summary("
        + match_var
        + "))"
    )
    expected_lines.append("match:" + row_id + ":" + summary + "\n")
lines.extend(["    return 0", "  }", "}", ""])
app.write_text("\n".join(lines), encoding="utf-8")
expected.write_text("".join(expected_lines), encoding="utf-8")
PY

python3 - "$MATCHER_IMPL" "$SNAPSHOT_IMPL" <<'PY'
import sys
from pathlib import Path

matcher = Path(sys.argv[1]).read_text(encoding="utf-8")
snapshot = Path(sys.argv[2]).read_text(encoding="utf-8")
for needle in [
    "match_snapshot(snapshot): MapBox",
    "\"matcher_input_consumed\" => 1",
    "\"contract_kind_code\" => 1",
    "\"recipe_matcher_executed\" => 1",
    "\"full_recipe_matcher_execution\" => 0",
    "\"route_selection\" => 0",
    "\"mir_lowering\" => 0",
    "\"id_allocation\" => 0",
    "match_summary(result)",
]:
    if needle not in matcher:
        raise SystemExit(f"missing matcher implementation token: {needle}")
for forbidden in [
    "emit_mir",
    "new_backend_route",
    "ASTNode",
    "RecipeComposer",
    "route switch",
]:
    if forbidden in matcher:
        raise SystemExit(f"forbidden matcher implementation token: {forbidden}")
if "build_snapshot(program_json): MapBox" not in snapshot:
    raise SystemExit("snapshot owner missing MapBox publication boundary")
PY

bash "$HAKO_BIN" --backend mir --verify "$MATCHER_IMPL" >/dev/null

if ! timeout 90s bash "$HAKO_BIN" --backend mir --emit-mir-json "$MIR_JSON" "$APP" >"$BUILD_LOG" 2>&1; then
  tail -n 160 "$BUILD_LOG" || true
  guard_fail "$TAG" "failed to emit MIR JSON for RecipeMatcher boundary probe"
fi

python3 - "$MIR_JSON" <<'PY'
import json
import sys
from pathlib import Path

data = json.loads(Path(sys.argv[1]).read_text(encoding="utf-8"))
functions = data.get("functions") or []
main = next((fn for fn in functions if fn.get("name") == "main"), None)
matcher = next((fn for fn in functions if fn.get("name") == "ProgramJsonRecipeMatcherExecutionBoundaryBox.match_snapshot/1"), None)
summary = next((fn for fn in functions if fn.get("name") == "ProgramJsonRecipeMatcherExecutionBoundaryBox.match_summary/1"), None)
if main is None or matcher is None or summary is None:
    raise SystemExit("required functions missing")

def route_rows(fn):
    metadata = fn.get("metadata") or {}
    rows = []
    for key in ("global_call_routes", "lowering_plan"):
        value = metadata.get(key) or []
        if isinstance(value, list):
            rows.extend(row for row in value if isinstance(row, dict))
    return rows

main_snapshot_routes = [
    row for row in route_rows(main)
    if row.get("symbol") == "ProgramJsonCanonicalLoopFactsInputSnapshotBox.build_snapshot/1"
]
if len(main_snapshot_routes) < 2:
    raise SystemExit("main must build a CanonicalLoopFacts input snapshot for each row")
for row in main_snapshot_routes:
    if row.get("tier") != "DirectAbi" or row.get("return_shape") != "map_handle":
        raise SystemExit(f"snapshot route is not DirectAbi/map_handle: {row}")

main_match_routes = [
    row for row in route_rows(main)
    if row.get("symbol") == "ProgramJsonRecipeMatcherExecutionBoundaryBox.match_snapshot/1"
]
if len(main_match_routes) < 2:
    raise SystemExit("main must call match_snapshot once per row")
for row in main_match_routes:
    if row.get("tier") != "DirectAbi" or row.get("return_shape") != "map_handle":
        raise SystemExit(f"matcher route is not DirectAbi/map_handle: {row}")

main_summary_routes = [
    row for row in route_rows(main)
    if row.get("symbol") == "ProgramJsonRecipeMatcherExecutionBoundaryBox.match_summary/1"
]
if len(main_summary_routes) < 2:
    raise SystemExit("main must summarize each matcher result")

owner_functions = [
    fn for fn in functions
    if str(fn.get("name", "")).startswith("ProgramJsonRecipeMatcherExecutionBoundaryBox.")
]
for fn in owner_functions:
    bad = [row for row in route_rows(fn) if row.get("tier") == "Unsupported" or row.get("reason")]
    if bad:
        raise SystemExit(f"{fn.get('name')} has unsupported routes: {bad[:5]}")

all_rows = []
for fn in owner_functions:
    all_rows.extend(route_rows(fn))
for forbidden in [
    "RecipeComposer",
    "lower_",
    "RouteSelection",
    "IdAllocator",
]:
    if any(forbidden in str(row.get("symbol")) for row in all_rows):
        raise SystemExit(f"forbidden downstream symbol leaked: {forbidden}")
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
  guard_fail "$TAG" "runtime RecipeMatcher boundary result mismatch"
fi

cat <<'REPORT'
output_contract=rust-lifecycle-mirbuilder-programjson-recipematcher-execution-boundary-minimal-gate-v0
token=MIRBUILDER-PROGRAMJSON-RECIPEMATCHER-EXECUTION-BOUNDARY-MINIMAL-001
owner=ProgramJsonRecipeMatcherExecutionBoundaryBox
row_count=2
recipe_matcher_execution_boundary_minimal=1
observe_only_recipe_matcher_execution=1
recipe_matcher_input_snapshot_consumed=1
recipe_matcher_input_authority=0
full_recipe_matcher_execution=0
route_selection=0
mir_lowering=0
mir_mutation=0
id_allocation=0
runtime_route_switch=0
runtime_fallback=0
source_selfhost_claim=0
vm_only_proof_as_main_acceptance=0
summary=ok
REPORT
