#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="rust-lifecycle-mirbuilder-programjson-recipematcher-shadow-parity-expanded-rows-gate"

source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

FIXTURE="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-programjson-recipematcher-shadow-parity-expanded-rows-v0.json"
CARD="$ROOT_DIR/docs/development/current/main/phases/phase-296x/3232-MIRBUILDER-PROGRAMJSON-RECIPEMATCHER-SHADOW-PARITY-EXPANDED-ROWS-001.md"
DUAL_RUN_GUARD="$ROOT_DIR/tools/checks/rust_lifecycle_mirbuilder_programjson_recipematcher_runtime_dual_run_shadow_guard.sh"
SNAPSHOT_IMPL="$ROOT_DIR/lang/src/compiler/mirbuilder/program_json_canonical_loop_facts_input_snapshot.hako"
MATCHER_IMPL="$ROOT_DIR/lang/src/compiler/mirbuilder/program_json_recipematcher_execution_boundary.hako"
HAKO_BIN="$ROOT_DIR/tools/bin/hako"

guard_require_command "$TAG" python3
guard_require_command "$TAG" timeout
guard_require_files "$TAG" "$FIXTURE" "$CARD" "$DUAL_RUN_GUARD" "$SNAPSHOT_IMPL" "$MATCHER_IMPL" "$HAKO_BIN"

DUAL_OUT="$(guard_cached_run "$TAG" bash "$DUAL_RUN_GUARD")"
if ! grep -q '^dual_run_shadow_guard=1$' <<<"$DUAL_OUT"; then
  printf '%s\n' "$DUAL_OUT" >&2
  guard_fail "$TAG" "dual-run shadow guard prerequisite is not green"
fi
if ! grep -q '^runtime_authority=rust_astnode$' <<<"$DUAL_OUT"; then
  printf '%s\n' "$DUAL_OUT" >&2
  guard_fail "$TAG" "dual-run guard did not keep Rust authority"
fi

TMP_DIR="$(mktemp -d /tmp/hakorune-recipematcher-expanded-rows.XXXXXX)"
cleanup() { rm -rf "$TMP_DIR" >/dev/null 2>&1 || true; }
trap cleanup EXIT

APP="$TMP_DIR/recipematcher_expanded_rows_probe.hako"
EXE="$TMP_DIR/recipematcher_expanded_rows_probe.exe"
MIR_JSON="$TMP_DIR/recipematcher_expanded_rows_probe.mir.json"
BUILD_LOG="$TMP_DIR/build.log"
RUN_OUT="$TMP_DIR/run.out"
RUN_FILTERED="$TMP_DIR/run.filtered"
RUN_ERR="$TMP_DIR/run.err"

python3 - "$FIXTURE" "$CARD" "$SNAPSHOT_IMPL" "$MATCHER_IMPL" "$APP" <<'PY'
import json
import sys
from pathlib import Path

fixture = json.loads(Path(sys.argv[1]).read_text(encoding="utf-8"))
card = Path(sys.argv[2]).read_text(encoding="utf-8")
snapshot_impl = Path(sys.argv[3]).read_text(encoding="utf-8")
matcher_impl = Path(sys.argv[4]).read_text(encoding="utf-8")
app = Path(sys.argv[5])

token = "MIRBUILDER-PROGRAMJSON-RECIPEMATCHER-SHADOW-PARITY-EXPANDED-ROWS-001"
if fixture.get("kind") != "MirBuilderProgramJsonRecipeMatcherShadowParityExpandedRowsV1":
    raise SystemExit("bad fixture kind")
if fixture.get("token") != token:
    raise SystemExit("bad fixture token")

contract = fixture.get("parity_contract") or {}
required = {
    "comparison": "canonical matcher result fields",
    "row_count": 4,
    "shadow_mode": True,
    "runtime_authority": "Rust ASTNode route",
    "runtime_route_switch": False,
    "programjson_runtime_route_authority": False,
    "route_selection": False,
    "mir_lowering": False,
}
for key, value in required.items():
    if contract.get(key) != value:
        raise SystemExit(f"parity contract drift: {key}")

guard_contract = fixture.get("guard_contract") or {}
for key in [
    "aot_required",
    "dual_run_guard_required",
    "canonical_field_compare_required",
    "no_runtime_route_switch",
    "no_programjson_runtime_authority",
    "no_recipe_matcher_input_authority",
    "no_route_selection",
    "no_mir_lowering",
    "no_mir_mutation",
    "no_id_allocation",
    "no_runtime_fallback",
]:
    if guard_contract.get(key) is not True:
        raise SystemExit(f"guard contract missing true: {key}")
if guard_contract.get("vm_only_main_acceptance") is not False:
    raise SystemExit("VM-only main acceptance must stay false")

claims = fixture.get("claims") or {}
for key in [
    "recipe_matcher_shadow_parity_expanded_rows",
    "matcher_result_equal",
    "runtime_authority_remains_rust_astnode",
    "programjson_shadow_checked",
]:
    if claims.get(key) != 1:
        raise SystemExit(f"missing positive claim: {key}")
if claims.get("expanded_row_count") != 4:
    raise SystemExit("expanded row count claim drift")
for key, value in claims.items():
    if key in {
        "recipe_matcher_shadow_parity_expanded_rows",
        "matcher_result_equal",
        "runtime_authority_remains_rust_astnode",
        "programjson_shadow_checked",
        "expanded_row_count",
    }:
        continue
    if value != 0:
        raise SystemExit(f"forbidden claim drift: {key}")

rows = fixture.get("rows") or []
if len(rows) != 4:
    raise SystemExit("fixture must contain four rows")
if len({row.get("row_id") for row in rows}) != len(rows):
    raise SystemExit("duplicate row ids")

expected = {
    "matched": 1,
    "contract_kind": "LoopWithExit",
    "has_break": 0,
    "has_continue": 0,
    "has_return": 1,
}
for row in rows:
    if row.get("rust_astnode_route_oracle") != expected:
        raise SystemExit(f"Rust oracle drift: {row.get('row_id')}")
    if row.get("programjson_route_expected") != expected:
        raise SystemExit(f"ProgramJSON expected drift: {row.get('row_id')}")
    if not isinstance(row.get("program_json"), dict):
        raise SystemExit(f"missing ProgramJSON object: {row.get('row_id')}")

for needle in [
    "build_snapshot(program_json): MapBox",
    "_name_code(name)",
    "if me._token_eq(name, \"i\") == 1",
    "if me._token_eq(name, \"count\") == 1",
]:
    if needle not in snapshot_impl:
        raise SystemExit(f"snapshot implementation missing token: {needle}")
for needle in [
    "match_snapshot(snapshot): MapBox",
    "\"contract_kind_code\" => 1",
    "\"runtime_route_switch\" => 0",
]:
    if needle not in matcher_impl:
        raise SystemExit(f"matcher implementation missing token: {needle}")
for forbidden in ["RecipeComposer", "PlanLowerer", "IdAllocator", "runtime route switch"]:
    if forbidden in snapshot_impl or forbidden in matcher_impl:
        raise SystemExit(f"forbidden implementation token: {forbidden}")

for needle in [
    token,
    "row_count=4",
    "runtime_authority=rust_astnode",
    "programjson_runtime_route_authority=0",
    "runtime_route_switch=0",
]:
    if needle not in card:
        raise SystemExit(f"card missing: {needle}")

lines = [
    "using lang.compiler.mirbuilder.program_json_canonical_loop_facts_input_snapshot as ProgramJsonCanonicalLoopFactsInputSnapshotBox",
    "using lang.compiler.mirbuilder.program_json_recipematcher_execution_boundary as ProgramJsonRecipeMatcherExecutionBoundaryBox",
    "",
    "static box Main {",
    "  main() {",
]
for index, row in enumerate(rows):
    program_json = json.dumps(row["program_json"], separators=(",", ":"), ensure_ascii=False)
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
        "    print(\"expanded:"
        + row["row_id"]
        + ":\" + ProgramJsonRecipeMatcherExecutionBoundaryBox.match_summary("
        + match_var
        + "))"
    )
lines.extend(["    return 0", "  }", "}", ""])
app.write_text("\n".join(lines), encoding="utf-8")
PY

if ! timeout 90s bash "$HAKO_BIN" --backend mir --emit-mir-json "$MIR_JSON" "$APP" >"$BUILD_LOG" 2>&1; then
  tail -n 160 "$BUILD_LOG" || true
  guard_fail "$TAG" "failed to emit MIR JSON for RecipeMatcher expanded rows probe"
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
    if len(rows) < 4:
        raise SystemExit(f"main must call {symbol} at least four times, got {len(rows)}")
    for row in rows:
        if row.get("tier") != "DirectAbi" or row.get("return_shape") != "map_handle":
            raise SystemExit(f"{symbol} is not DirectAbi/map_handle: {row}")

owner_functions = [
    fn for fn in functions
    if str(fn.get("name", "")).startswith("ProgramJsonRecipeMatcherExecutionBoundaryBox.")
    or str(fn.get("name", "")).startswith("ProgramJsonCanonicalLoopFactsInputSnapshotBox.")
]
for fn in owner_functions:
    bad = [row for row in route_rows(fn) if row.get("tier") == "Unsupported" or row.get("reason")]
    if bad:
        raise SystemExit(f"{fn.get('name')} has unsupported routes: {bad[:5]}")

all_symbols = {str(row.get("symbol")) for fn in owner_functions for row in route_rows(fn)}
for forbidden in ["RecipeComposer", "RouteSelection", "PlanLowerer", "IdAllocator"]:
    if any(forbidden in symbol for symbol in all_symbols):
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

python3 - "$FIXTURE" "$RUN_FILTERED" <<'PY'
import json
import sys
from pathlib import Path

fixture = json.loads(Path(sys.argv[1]).read_text(encoding="utf-8"))
lines = [line.strip() for line in Path(sys.argv[2]).read_text(encoding="utf-8").splitlines() if line.strip()]
if len(lines) != 4:
    raise SystemExit(f"expected four output rows, got {len(lines)}")

actual = {}
for line in lines:
    if not line.startswith("expanded:"):
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
            raise SystemExit(f"forbidden output drift {zero_key}: {line}")

for row in fixture.get("rows") or []:
    row_id = row["row_id"]
    if actual.get(row_id) != row["rust_astnode_route_oracle"]:
        raise SystemExit(f"expanded parity mismatch for {row_id}: {actual.get(row_id)}")
PY

cat <<'REPORT'
output_contract=rust-lifecycle-mirbuilder-programjson-recipematcher-shadow-parity-expanded-rows-gate-v0
token=MIRBUILDER-PROGRAMJSON-RECIPEMATCHER-SHADOW-PARITY-EXPANDED-ROWS-001
owner=ProgramJsonRecipeMatcherExecutionBoundaryBox
row_count=4
recipe_matcher_shadow_parity_expanded_rows=1
matcher_result_equal=1
runtime_authority=rust_astnode
programjson_shadow_checked=1
programjson_runtime_route_authority=0
runtime_route_switch=0
recipe_matcher_input_authority=0
full_recipe_matcher_execution=0
route_selection=0
mir_lowering=0
mir_mutation=0
id_allocation=0
runtime_fallback=0
source_selfhost_claim=0
new_backend_route=0
new_abi=0
vm_only_proof_as_main_acceptance=0
summary=ok
REPORT
