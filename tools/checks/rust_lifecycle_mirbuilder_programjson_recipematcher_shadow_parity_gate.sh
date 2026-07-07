#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="rust-lifecycle-mirbuilder-programjson-recipematcher-shadow-parity-gate"

source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

FIXTURE="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-programjson-recipematcher-shadow-parity-v0.json"
BOUNDARY_FIXTURE="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-programjson-recipematcher-execution-boundary-minimal-v0.json"
EXPANDED_FIXTURE="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-programjson-recipebodies-verifier-boundary-expanded-dto-coverage-parity-v0.json"
BOUNDARY_GATE="$ROOT_DIR/tools/checks/rust_lifecycle_mirbuilder_programjson_recipematcher_execution_boundary_minimal_gate.sh"
RUST_MATCHER="$ROOT_DIR/src/mir/builder/control_flow/plan/recipe_tree/matcher/mod.rs"
RUST_CONTRACT="$ROOT_DIR/src/mir/builder/control_flow/plan/recipe_tree/contracts.rs"
SNAPSHOT_IMPL="$ROOT_DIR/lang/src/compiler/mirbuilder/program_json_canonical_loop_facts_input_snapshot.hako"
MATCHER_IMPL="$ROOT_DIR/lang/src/compiler/mirbuilder/program_json_recipematcher_execution_boundary.hako"
HAKO_BIN="$ROOT_DIR/tools/bin/hako"

guard_require_command "$TAG" python3
guard_require_command "$TAG" timeout
guard_require_files "$TAG" "$FIXTURE" "$BOUNDARY_FIXTURE" "$EXPANDED_FIXTURE" "$BOUNDARY_GATE" "$RUST_MATCHER" "$RUST_CONTRACT" "$SNAPSHOT_IMPL" "$MATCHER_IMPL" "$HAKO_BIN"

BOUNDARY_OUT="$(guard_cached_run "$TAG" bash "$BOUNDARY_GATE")"
if ! grep -q '^observe_only_recipe_matcher_execution=1$' <<<"$BOUNDARY_OUT"; then
  printf '%s\n' "$BOUNDARY_OUT" >&2
  guard_fail "$TAG" "RecipeMatcher execution boundary prerequisite is not green"
fi

TMP_DIR="$(mktemp -d /tmp/hakorune-recipematcher-shadow-parity.XXXXXX)"
cleanup() { rm -rf "$TMP_DIR" >/dev/null 2>&1 || true; }
trap cleanup EXIT

APP="$TMP_DIR/recipematcher_shadow_parity_probe.hako"
EXE="$TMP_DIR/recipematcher_shadow_parity_probe.exe"
MIR_JSON="$TMP_DIR/recipematcher_shadow_parity_probe.mir.json"
BUILD_LOG="$TMP_DIR/build.log"
RUN_OUT="$TMP_DIR/run.out"
RUN_FILTERED="$TMP_DIR/run.filtered"
RUN_ERR="$TMP_DIR/run.err"

python3 - "$FIXTURE" "$BOUNDARY_FIXTURE" "$EXPANDED_FIXTURE" "$RUST_MATCHER" "$RUST_CONTRACT" "$APP" <<'PY'
import json
import sys
from pathlib import Path

fixture = json.loads(Path(sys.argv[1]).read_text(encoding="utf-8"))
boundary = json.loads(Path(sys.argv[2]).read_text(encoding="utf-8"))
expanded = json.loads(Path(sys.argv[3]).read_text(encoding="utf-8"))
rust_matcher = Path(sys.argv[4]).read_text(encoding="utf-8")
rust_contract = Path(sys.argv[5]).read_text(encoding="utf-8")
app = Path(sys.argv[6])

if fixture.get("kind") != "MirBuilderProgramJsonRecipeMatcherShadowParityV1":
    raise SystemExit("bad fixture kind")
if fixture.get("token") != "MIRBUILDER-PROGRAMJSON-RECIPEMATCHER-SHADOW-PARITY-001":
    raise SystemExit("bad fixture token")

contract = fixture.get("parity_contract") or {}
required = {
    "comparison": "canonical matcher result fields",
    "shadow_mode": True,
    "runtime_route_switch": False,
    "route_selection": False,
    "mir_lowering": False,
}
for key, value in required.items():
    if contract.get(key) != value:
        raise SystemExit(f"parity contract drift: {key}")

guard_contract = fixture.get("guard_contract") or {}
for key in [
    "aot_required",
    "rust_contract_source_checked",
    "programjson_matcher_boundary_gate_required",
    "canonical_field_compare_required",
    "no_runtime_route_switch",
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
    "recipe_matcher_shadow_parity",
    "matcher_result_equal",
    "rust_astnode_route_oracle_checked",
    "programjson_route_shadow_checked",
]:
    if claims.get(key) != 1:
        raise SystemExit(f"missing positive claim: {key}")
for key, value in claims.items():
    if key in {
        "recipe_matcher_shadow_parity",
        "matcher_result_equal",
        "rust_astnode_route_oracle_checked",
        "programjson_route_shadow_checked",
    }:
        continue
    if value != 0:
        raise SystemExit(f"forbidden claim drift: {key}")

for needle in [
    "pub fn try_match_loop(facts: &CanonicalLoopFacts)",
    "RecipeContractKind::LoopWithExit",
    "has_break",
    "has_continue",
    "has_return",
]:
    if needle not in rust_matcher and needle not in rust_contract:
        raise SystemExit(f"missing Rust oracle token: {needle}")

expanded_rows = expanded.get("rows") or []
program_json_by_id = {row["row_id"]: row["program_json"] for row in expanded_rows}
boundary_by_id = {row["row_id"]: row["expected_summary"] for row in boundary.get("rows") or []}
rows = fixture.get("rows") or []
if [row.get("row_id") for row in rows] != [
    "local_loop_body_if_branch_return",
    "local_loop_body_if_branch_return_alt_names",
]:
    raise SystemExit("unexpected row order")

for row in rows:
    row_id = row["row_id"]
    rust_oracle = row.get("rust_astnode_route_oracle") or {}
    programjson = row.get("programjson_route_expected") or {}
    if rust_oracle != programjson:
        raise SystemExit(f"fixture parity mismatch: {row_id}")
    if row.get("parity", {}).get("matcher_result_equal") != 1:
        raise SystemExit(f"missing parity marker: {row_id}")
    for key, expected in {
        "matched": 1,
        "contract_kind": "LoopWithExit",
        "has_break": 0,
        "has_continue": 0,
        "has_return": 1,
    }.items():
        if rust_oracle.get(key) != expected:
            raise SystemExit(f"unexpected Rust oracle {key}: {row_id}")
    if row_id not in program_json_by_id or row_id not in boundary_by_id:
        raise SystemExit(f"missing prerequisite row: {row_id}")

lines = [
    "using lang.compiler.mirbuilder.program_json_canonical_loop_facts_input_snapshot as ProgramJsonCanonicalLoopFactsInputSnapshotBox",
    "using lang.compiler.mirbuilder.program_json_recipematcher_execution_boundary as ProgramJsonRecipeMatcherExecutionBoundaryBox",
    "",
    "static box Main {",
    "  main() {",
]
for index, row in enumerate(rows):
    row_id = row["row_id"]
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
        "    print(\"shadow:"
        + row_id
        + ":\" + ProgramJsonRecipeMatcherExecutionBoundaryBox.match_summary("
        + match_var
        + "))"
    )
lines.extend(["    return 0", "  }", "}", ""])
app.write_text("\n".join(lines), encoding="utf-8")
PY

if ! timeout 90s bash "$HAKO_BIN" --backend mir --emit-mir-json "$MIR_JSON" "$APP" >"$BUILD_LOG" 2>&1; then
  tail -n 160 "$BUILD_LOG" || true
  guard_fail "$TAG" "failed to emit MIR JSON for RecipeMatcher shadow parity probe"
fi

python3 - "$MIR_JSON" <<'PY'
import json
import sys
from pathlib import Path

data = json.loads(Path(sys.argv[1]).read_text(encoding="utf-8"))
functions = data.get("functions") or []
main = next((fn for fn in functions if fn.get("name") == "main"), None)
if main is None:
    raise SystemExit("main function missing")

def route_rows(fn):
    metadata = fn.get("metadata") or {}
    rows = []
    for key in ("global_call_routes", "lowering_plan"):
        value = metadata.get(key) or []
        if isinstance(value, list):
            rows.extend(row for row in value if isinstance(row, dict))
    return rows

main_rows = route_rows(main)
for symbol in [
    "ProgramJsonCanonicalLoopFactsInputSnapshotBox.build_snapshot/1",
    "ProgramJsonRecipeMatcherExecutionBoundaryBox.match_snapshot/1",
]:
    rows = [row for row in main_rows if row.get("symbol") == symbol]
    if len(rows) < 2:
        raise SystemExit(f"main must call {symbol} once per row")
    for row in rows:
        if row.get("tier") != "DirectAbi" or row.get("return_shape") != "map_handle":
            raise SystemExit(f"{symbol} is not DirectAbi/map_handle: {row}")

owner_functions = [
    fn for fn in functions
    if str(fn.get("name", "")).startswith("ProgramJsonRecipeMatcherExecutionBoundaryBox.")
]
for fn in owner_functions:
    bad = [row for row in route_rows(fn) if row.get("tier") == "Unsupported" or row.get("reason")]
    if bad:
        raise SystemExit(f"{fn.get('name')} has unsupported routes: {bad[:5]}")
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

python3 - "$FIXTURE" "$RUN_FILTERED" <<'PY'
import sys
from pathlib import Path
import json

fixture = json.loads(Path(sys.argv[1]).read_text(encoding="utf-8"))
lines = [line.strip() for line in Path(sys.argv[2]).read_text(encoding="utf-8").splitlines() if line.strip()]
actual = {}
for line in lines:
    if not line.startswith("shadow:"):
        raise SystemExit(f"unexpected output line: {line}")
    _, row_id, summary = line.split(":", 2)
    fields = {}
    for part in summary.split(";"):
        if "=" in part:
            key, value = part.split("=", 1)
            fields[key] = value
    actual[row_id] = {
        "matched": int(fields.get("matched", "-1")),
        "contract_kind": fields.get("contract_kind"),
        "has_break": int(fields.get("has_break", "-1")),
        "has_continue": int(fields.get("has_continue", "-1")),
        "has_return": int(fields.get("has_return", "-1")),
    }
    for zero_key in [
        "full_recipe_matcher_execution",
        "route_selection",
        "mir_lowering",
        "mir_mutation",
        "id_allocation",
        "runtime_route_switch",
    ]:
        if fields.get(zero_key) != "0":
            raise SystemExit(f"forbidden claim output drift {zero_key}: {line}")

for row in fixture.get("rows") or []:
    row_id = row["row_id"]
    rust_oracle = row["rust_astnode_route_oracle"]
    if actual.get(row_id) != rust_oracle:
        raise SystemExit(f"shadow parity mismatch for {row_id}: {actual.get(row_id)} != {rust_oracle}")
PY

cat <<'REPORT'
output_contract=rust-lifecycle-mirbuilder-programjson-recipematcher-shadow-parity-gate-v0
token=MIRBUILDER-PROGRAMJSON-RECIPEMATCHER-SHADOW-PARITY-001
owner=ProgramJsonRecipeMatcherExecutionBoundaryBox
row_count=2
recipe_matcher_shadow_parity=1
matcher_result_equal=1
rust_astnode_route_oracle_checked=1
programjson_route_shadow_checked=1
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
