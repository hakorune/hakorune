#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="rust-lifecycle-mirbuilder-compare-rhs-materialization-readonly-resolution-pilot-gate"

source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

FIXTURE="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-compare-rhs-materialization-readonly-resolution-pilot-v0.json"
IMPL="$ROOT_DIR/lang/src/compiler/mirbuilder/compare_rhs_valueid_resolution_plan_snapshot.hako"
INTENT_IMPL="$ROOT_DIR/lang/src/compiler/mirbuilder/compare_rhs_materialization_intent_snapshot.hako"
SELECTION_GATE="$ROOT_DIR/tools/checks/rust_lifecycle_mirbuilder_compare_rhs_valueid_resolution_owner_selection_guard.sh"
HAKO_BIN="$ROOT_DIR/tools/bin/hako"

guard_require_command "$TAG" python3
guard_require_files "$TAG" "$FIXTURE" "$IMPL" "$INTENT_IMPL" "$SELECTION_GATE" "$HAKO_BIN"

SELECTION_OUT="$(guard_cached_run "$TAG" bash "$SELECTION_GATE")"
if ! grep -q '^readonly_rhs_valueid_resolution_plan_pilot_selected=1$' <<<"$SELECTION_OUT"; then
  printf '%s\n' "$SELECTION_OUT" >&2
  guard_fail "$TAG" "RHS ValueId resolution plan selection prerequisite is not green"
fi

python3 - "$FIXTURE" "$IMPL" <<'PY'
import json
import sys
from pathlib import Path

fixture = json.loads(Path(sys.argv[1]).read_text(encoding="utf-8"))
impl = Path(sys.argv[2]).read_text(encoding="utf-8")

def need(condition, message):
    if not condition:
        raise SystemExit(message)

need(fixture.get("schema_version") == 0, "bad schema_version")
need(fixture.get("kind") == "MirBuilderCompareRhsMaterializationReadonlyResolutionPilotV1", "bad kind")
need(fixture.get("token") == "MIRBUILDER-COMPARE-RHS-MATERIALIZATION-READONLY-RESOLUTION-PILOT-001", "bad token")
need(fixture.get("owner") == "CompareRhsValueIdResolutionPlanSnapshotBox", "bad owner")
need(fixture.get("output_contract") == "CompareRhsValueIdResolutionPlanSnapshotV1", "bad output contract")
need([row.get("row_id") for row in fixture.get("rows") or []] == ["intent_literal_i64", "intent_symbol_ref"], "row set drift")

claims = fixture.get("claims") or {}
for key in [
    "compare_rhs_materialization_readonly_resolution_pilot",
    "rhs_valueid_resolution_plan_snapshot",
    "literal_constant_plan_row",
    "symbol_lookup_plan_row",
    "analysis_only",
]:
    need(claims.get(key) == 1, f"missing positive claim: {key}")
for key in [
    "rhs_value_id_resolution",
    "literal_constant_value_id_allocation",
    "constant_mir_emission",
    "runtime_helper_emission",
    "local_ssa_finalize_compare_execution",
    "mir_cmp_emission",
    "branch_emission",
    "basic_block_mutation",
    "value_id_allocation",
    "route_selection",
    "runtime_route_switch",
    "programjson_runtime_route_authority",
    "programjson_var_rhs_full_dispatcher_authority",
    "runtime_fallback",
    "source_selfhost_claim",
]:
    need(claims.get(key) == 0, f"forbidden claim drift: {key}")

for needle in [
    "build_plan(program_json): MapBox",
    "CompareRhsMaterializationIntentSnapshotBox.build_intent",
    "build_plan_from_intent(intent): MapBox",
    '"rhs_valueid_resolution_plan_ready" => 1',
    '"literal_constant_required" => literal_required',
    '"symbol_lookup_required" => symbol_required',
    '"rhs_value_id_resolution" => 0',
    '"literal_constant_value_id_allocation" => 0',
    '"constant_mir_emission" => 0',
    '"local_ssa_finalize_compare_execution" => 0',
    '"mir_cmp_emission" => 0',
    '"value_id_allocation" => 0',
    "plan_summary(plan)",
]:
    need(needle in impl, f"implementation missing token: {needle}")
for forbidden in [
    "MirInstruction",
    "emit_mir",
    "emit_compare",
    "emit_branch",
    "route_registry",
    "RecipeMatcherBox",
    "next_value_id",
    "finalize_compare(",
]:
    need(forbidden not in impl, f"forbidden implementation token: {forbidden}")
PY

TMP_DIR="$(mktemp -d /tmp/hakorune-rhs-readonly-resolution.XXXXXX)"
cleanup() { rm -rf "$TMP_DIR" >/dev/null 2>&1 || true; }
trap cleanup EXIT

APP="$TMP_DIR/rhs_readonly_resolution.hako"
EXPECTED="$TMP_DIR/expected.txt"
ACTUAL="$TMP_DIR/actual.txt"
EXE="$TMP_DIR/rhs_readonly_resolution.exe"
EMIT_LOG="$TMP_DIR/emit.log"

python3 - "$FIXTURE" "$APP" "$EXPECTED" <<'PY'
import json
import sys
from pathlib import Path

fixture = json.loads(Path(sys.argv[1]).read_text(encoding="utf-8"))
app = Path(sys.argv[2])
expected = Path(sys.argv[3])

lines = [
    "using lang.compiler.mirbuilder.compare_rhs_valueid_resolution_plan_snapshot as CompareRhsValueIdResolutionPlanSnapshotBox",
    "",
    "static box Main {",
    "  main() {",
]
expected_lines = []
for idx, row in enumerate(fixture["rows"]):
    if row.get("input_kind") != "intent_map":
        raise SystemExit(f"unknown input_kind for {row['row_id']}")
    intent_entries = ", ".join(
        json.dumps(key) + " => " + str(value)
        for key, value in row["intent"].items()
    )
    lines.append(f"    local intent_{idx} = %{{{intent_entries}}}")
    lines.append(f"    local plan_{idx} = CompareRhsValueIdResolutionPlanSnapshotBox.build_plan_from_intent(intent_{idx})")
    lines.append(
        f"    print(\"rhs_resolution_plan:{row['row_id']}:\" + CompareRhsValueIdResolutionPlanSnapshotBox.plan_summary(plan_{idx}))"
    )
    expected_lines.append(f"rhs_resolution_plan:{row['row_id']}:{row['expected_plan_summary']}")

lines.extend(["    return 0", "  }", "}", ""])
app.write_text("\n".join(lines), encoding="utf-8")
expected.write_text("\n".join(expected_lines) + "\n", encoding="utf-8")
PY

bash "$HAKO_BIN" --backend mir --verify "$IMPL" >/dev/null

if ! bash "$HAKO_BIN" --backend mir --emit-exe "$EXE" "$APP" >"$EMIT_LOG" 2>&1; then
  tail -n 160 "$EMIT_LOG" || true
  guard_fail "$TAG" "failed to emit RHS read-only resolution executable"
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
    print("[compare-rhs/readonly-resolution-pilot] mismatch")
    for idx in range(max(len(expected), len(actual))):
        exp = expected[idx] if idx < len(expected) else "<missing>"
        got = actual[idx] if idx < len(actual) else "<missing>"
        if exp != got:
            print(f"row={idx} expected={exp!r} actual={got!r}")
    raise SystemExit(1)
PY

cat <<'REPORT'
output_contract=rust-lifecycle-mirbuilder-compare-rhs-materialization-readonly-resolution-pilot-gate-v0
token=MIRBUILDER-COMPARE-RHS-MATERIALIZATION-READONLY-RESOLUTION-PILOT-001
owner=CompareRhsValueIdResolutionPlanSnapshotBox
plan_rows=2
compare_rhs_materialization_readonly_resolution_pilot=1
rhs_valueid_resolution_plan_snapshot=1
literal_constant_plan_row=1
symbol_lookup_plan_row=1
analysis_only=1
rhs_value_id_resolution=0
literal_constant_value_id_allocation=0
constant_mir_emission=0
runtime_helper_emission=0
local_ssa_finalize_compare_execution=0
mir_cmp_emission=0
branch_emission=0
basic_block_mutation=0
value_id_allocation=0
route_selection=0
runtime_route_switch=0
programjson_runtime_route_authority=0
programjson_var_rhs_full_dispatcher_authority=0
runtime_fallback=0
source_selfhost_claim=0
selected_next_card=MIRBUILDER-COMPARE-RHS-MATERIALIZATION-READONLY-RESOLUTION-PARITY-001
summary=ok
REPORT
