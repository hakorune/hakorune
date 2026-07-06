#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="rust-lifecycle-mirbuilder-programjson-canonical-loop-facts-input-snapshot-mapbox-publication-bridge-gate"

source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

FIXTURE="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-programjson-canonical-loop-facts-input-snapshot-mapbox-publication-bridge-v0.json"
EXPANDED_FIXTURE="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-programjson-recipebodies-verifier-boundary-expanded-dto-coverage-parity-v0.json"
DESIGN_STOP_GUARD="$ROOT_DIR/tools/checks/rust_lifecycle_mirbuilder_programjson_canonical_loop_facts_input_snapshot_aot_boundary_design_stop_guard.sh"
PUBLICATION_GUARD="$ROOT_DIR/tools/checks/rust_lifecycle_mirbuilder_programjson_recipebodies_runtime_publication_bridge_gate.sh"
IMPL="$ROOT_DIR/lang/src/compiler/mirbuilder/program_json_canonical_loop_facts_input_snapshot.hako"
HAKO_BIN="$ROOT_DIR/tools/bin/hako"

guard_require_command "$TAG" python3
guard_require_command "$TAG" timeout
guard_require_files "$TAG" "$FIXTURE" "$EXPANDED_FIXTURE" "$DESIGN_STOP_GUARD" "$PUBLICATION_GUARD" "$IMPL" "$HAKO_BIN"

DESIGN_OUT="$(guard_cached_run "$TAG" bash "$DESIGN_STOP_GUARD")"
if ! grep -q '^recommended_default=A_MAPBOX_SNAPSHOT_PUBLICATION_BRIDGE$' <<<"$DESIGN_OUT"; then
  printf '%s\n' "$DESIGN_OUT" >&2
  guard_fail "$TAG" "design stop does not recommend MapBox publication bridge"
fi

PUBLICATION_OUT="$(guard_cached_run "$TAG" bash "$PUBLICATION_GUARD")"
if ! grep -q '^directabi_map_handle_publication=1$' <<<"$PUBLICATION_OUT"; then
  printf '%s\n' "$PUBLICATION_OUT" >&2
  guard_fail "$TAG" "RecipeBodies publication precedent is not DirectAbi/map_handle green"
fi

TMP_DIR="$(mktemp -d /tmp/hakorune-canonical-loop-facts-publication.XXXXXX)"
cleanup() { rm -rf "$TMP_DIR" >/dev/null 2>&1 || true; }
trap cleanup EXIT

APP="$TMP_DIR/canonical_loop_facts_publication_probe.hako"
MIR_JSON="$TMP_DIR/canonical_loop_facts_publication_probe.mir.json"
EXE="$TMP_DIR/canonical_loop_facts_publication_probe.exe"
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

if fixture.get("kind") != "MirBuilderProgramJsonCanonicalLoopFactsInputSnapshotMapboxPublicationBridgeV1":
    raise SystemExit("bad fixture kind")
if fixture.get("token") != "MIRBUILDER-PROGRAMJSON-CANONICAL-LOOP-FACTS-INPUT-SNAPSHOT-MAPBOX-PUBLICATION-BRIDGE-001":
    raise SystemExit("bad fixture token")

contract = fixture.get("publication_contract") or {}
required_contract = {
    "base_contract": "ProgramJsonReadOnlyMapSnapshotPublicationContractV1",
    "output": "ProgramJsonCanonicalLoopFactsInputSnapshotV1",
    "output_container": "MapBox",
    "publication_mode": "read_only_snapshot",
    "source": "verified_recipe",
    "result_map_required": True,
    "directabi_allowed": True,
    "runtime_route_switch": False,
    "recipe_matcher_execution": False,
}
for key, value in required_contract.items():
    if contract.get(key) != value:
        raise SystemExit(f"publication contract drift: {key}")

guard_contract = fixture.get("guard_contract") or {}
for key in [
    "aot_required",
    "mapbox_result_contract_required",
    "directabi_map_handle_required",
    "no_object_or_void_widening",
    "no_complex_string_summary_boundary",
    "no_runtime_fallback",
    "no_recipe_matcher_execution",
    "no_route_selection",
    "no_mir_lowering",
    "no_mir_mutation",
    "no_id_allocation",
]:
    if guard_contract.get(key) is not True:
        raise SystemExit(f"guard contract missing true: {key}")
if guard_contract.get("vm_only_main_acceptance") is not False:
    raise SystemExit("VM-only main acceptance must stay false")

claims = fixture.get("claims") or {}
for key in [
    "canonical_loop_facts_input_snapshot_publication_bridge",
    "read_only_canonical_loop_facts_input_snapshot",
    "directabi_map_handle_publication",
]:
    if claims.get(key) != 1:
        raise SystemExit(f"missing positive claim: {key}")
for key, value in claims.items():
    if key in {
        "canonical_loop_facts_input_snapshot_publication_bridge",
        "read_only_canonical_loop_facts_input_snapshot",
        "directabi_map_handle_publication",
    }:
        continue
    if value != 0:
        raise SystemExit(f"forbidden claim drift: {key}")

expanded_rows = expanded.get("rows") or []
program_json_by_id = {row["row_id"]: row["program_json"] for row in expanded_rows}
rows = fixture.get("rows") or []
if [row.get("row_id") for row in rows] != [
    "local_loop_body_if_branch_return",
    "local_loop_body_if_branch_return_alt_names",
]:
    raise SystemExit("unexpected row order")

lines = [
    "using lang.compiler.mirbuilder.program_json_canonical_loop_facts_input_snapshot as ProgramJsonCanonicalLoopFactsInputSnapshotBox",
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
        "snapshot_kind=ProgramJsonCanonicalLoopFactsInputSnapshotV1;",
        ";ok=1",
        ";source=verified_recipe",
        ";matcher_input_present=1",
        ";exit_has_return=1",
        ";loop_cond_return_in_body_present=1",
        ";cond_kind=VarLtInt",
        ";update_kind=AddVarInt",
        ";recipe_matcher_executed=0",
    ]:
        if token not in summary:
            raise SystemExit(f"missing expected summary token {token}: {row_id}")
    program_json = json.dumps(program_json_by_id[row_id], separators=(",", ":"), ensure_ascii=False)
    var_name = f"snapshot{index}"
    lines.append(
        "    local "
        + var_name
        + " = ProgramJsonCanonicalLoopFactsInputSnapshotBox.build_snapshot("
        + json.dumps(program_json)
        + ")"
    )
    lines.append(
        "    print(\"snapshot:"
        + row_id
        + ":\" + ProgramJsonCanonicalLoopFactsInputSnapshotBox.snapshot_summary("
        + var_name
        + "))"
    )
    expected_lines.append("snapshot:" + row_id + ":" + summary + "\n")
lines.extend(["    return 0", "  }", "}", ""])
app.write_text("\n".join(lines), encoding="utf-8")
expected.write_text("".join(expected_lines), encoding="utf-8")
PY

python3 - "$IMPL" <<'PY'
import sys
from pathlib import Path

text = Path(sys.argv[1]).read_text(encoding="utf-8")
for needle in [
    "build_snapshot(program_json): MapBox",
    "ProgramJsonV0PhaseStateBox.parse(program_json",
    "RecipeVerifierBox.verify(root",
    "\"source_code\" => 1",
    "\"matcher_input_present\" => 1",
    "\"readonly\" => 1",
    "\"recipe_matcher_executed\" => 0",
    "snapshot_summary(snapshot)",
]:
    if needle not in text:
        raise SystemExit(f"missing implementation token: {needle}")
for forbidden in [
    "RecipeMatcherBox",
    "build_summary(program_json)",
    "emit_mir",
    "new_backend_route",
    "ASTNode",
]:
    if forbidden in text:
        raise SystemExit(f"forbidden implementation token: {forbidden}")
PY

bash "$HAKO_BIN" --backend mir --verify "$IMPL" >/dev/null

if ! timeout 90s bash "$HAKO_BIN" --backend mir --emit-mir-json "$MIR_JSON" "$APP" >"$BUILD_LOG" 2>&1; then
  tail -n 160 "$BUILD_LOG" || true
  guard_fail "$TAG" "failed to emit MIR JSON for canonical loop facts snapshot probe"
fi

python3 - "$MIR_JSON" <<'PY'
import json
import sys
from pathlib import Path

data = json.loads(Path(sys.argv[1]).read_text(encoding="utf-8"))
functions = data.get("functions") or []
main = next((fn for fn in functions if fn.get("name") == "main"), None)
snapshot = next((fn for fn in functions if fn.get("name") == "ProgramJsonCanonicalLoopFactsInputSnapshotBox.build_snapshot/1"), None)
summary = next((fn for fn in functions if fn.get("name") == "ProgramJsonCanonicalLoopFactsInputSnapshotBox.snapshot_summary/1"), None)
verifier = next((fn for fn in functions if fn.get("name") == "ProgramJsonCanonicalLoopFactsInputSnapshotBox._verified_recipe_present/1"), None)
if main is None or snapshot is None or summary is None or verifier is None:
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
    raise SystemExit("main must call build_snapshot once per row")
for row in main_snapshot_routes:
    if row.get("tier") != "DirectAbi" or row.get("return_shape") != "map_handle":
        raise SystemExit(f"snapshot route is not DirectAbi/map_handle: {row}")

main_summary_routes = [
    row for row in route_rows(main)
    if row.get("symbol") == "ProgramJsonCanonicalLoopFactsInputSnapshotBox.snapshot_summary/1"
]
if len(main_summary_routes) < 2:
    raise SystemExit("main must summarize each snapshot row")

owner_functions = [
    fn for fn in functions
    if str(fn.get("name", "")).startswith("ProgramJsonCanonicalLoopFactsInputSnapshotBox.")
]
for fn in owner_functions:
    bad = [row for row in route_rows(fn) if row.get("tier") == "Unsupported" or row.get("reason")]
    if bad:
        raise SystemExit(f"{fn.get('name')} has unsupported routes: {bad[:5]}")

snapshot_symbols = {row.get("symbol") for row in route_rows(snapshot)}
if "ProgramJsonCanonicalLoopFactsInputSnapshotBox._verified_recipe_present/1" not in snapshot_symbols:
    raise SystemExit("build_snapshot must call verifier-boundary helper")
symbols = {row.get("symbol") for row in route_rows(verifier)}
for symbol in [
    "ProgramJsonV0PhaseStateBox.parse/2",
    "RecipeVerifierBox.verify/2",
]:
    if symbol not in symbols:
        raise SystemExit(f"missing verifier-boundary route: {symbol}")
all_rows = []
for fn in owner_functions:
    all_rows.extend(route_rows(fn))
if any("RecipeMatcher" in str(row.get("symbol")) for row in all_rows):
    raise SystemExit("RecipeMatcher must not be called by input snapshot bridge")
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
  guard_fail "$TAG" "runtime canonical loop facts snapshot mismatch"
fi

cat <<'REPORT'
output_contract=rust-lifecycle-mirbuilder-programjson-canonical-loop-facts-input-snapshot-mapbox-publication-bridge-gate-v0
token=MIRBUILDER-PROGRAMJSON-CANONICAL-LOOP-FACTS-INPUT-SNAPSHOT-MAPBOX-PUBLICATION-BRIDGE-001
owner=ProgramJsonCanonicalLoopFactsInputSnapshotBox
row_count=2
canonical_loop_facts_input_snapshot_publication_bridge=1
read_only_canonical_loop_facts_input_snapshot=1
directabi_map_handle_publication=1
source=verified_recipe
matcher_input_present=1
recipe_matcher_executed=0
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
