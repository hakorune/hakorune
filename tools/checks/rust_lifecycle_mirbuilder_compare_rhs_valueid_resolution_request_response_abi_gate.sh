#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="rust-lifecycle-mirbuilder-compare-rhs-valueid-resolution-request-response-abi-gate"

source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

FIXTURE="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-compare-rhs-valueid-resolution-request-response-abi-v0.json"
IMPL="$ROOT_DIR/lang/src/compiler/mirbuilder/compare_rhs_valueid_resolution_request_snapshot.hako"
PLAN_IMPL="$ROOT_DIR/lang/src/compiler/mirbuilder/compare_rhs_valueid_resolution_plan_snapshot.hako"
DESIGN_STOP_GATE="$ROOT_DIR/tools/checks/rust_lifecycle_mirbuilder_compare_rhs_actual_valueid_resolution_design_stop_guard.sh"
HAKO_BIN="$ROOT_DIR/tools/bin/hako"

guard_require_command "$TAG" python3
guard_require_files "$TAG" "$FIXTURE" "$IMPL" "$PLAN_IMPL" "$DESIGN_STOP_GATE" "$HAKO_BIN"

DESIGN_OUT="$(guard_cached_run "$TAG" bash "$DESIGN_STOP_GATE")"
if ! grep -q '^request_response_abi_selected=1$' <<<"$DESIGN_OUT"; then
  printf '%s\n' "$DESIGN_OUT" >&2
  guard_fail "$TAG" "RHS ValueId request/response ABI prerequisite is not green"
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
need(fixture.get("kind") == "MirBuilderCompareRhsValueIdResolutionRequestResponseAbiV1", "bad kind")
need(fixture.get("token") == "MIRBUILDER-COMPARE-RHS-VALUEID-RESOLUTION-REQUEST-RESPONSE-ABI-001", "bad token")
need(fixture.get("owner") == "CompareRhsValueIdResolutionRequestSnapshotBox", "bad owner")
need(fixture.get("request_contract") == "CompareRhsValueIdResolutionRequestSnapshotV1", "bad request contract")
need(fixture.get("response_contract") == "CompareRhsValueIdResolutionResponseV1", "bad response contract")
need([row.get("row_id") for row in fixture.get("request_rows") or []] == ["request_literal_i64", "request_symbol_ref"], "row set drift")

response_schema = fixture.get("response_schema") or {}
required = set(response_schema.get("required_fields") or [])
for field in [
    "ok",
    "reason_code",
    "rhs_value_id_present",
    "rhs_value_id",
    "emitted_constant",
    "constant_kind_code",
    "constant_i64",
    "used_symbol_lookup",
    "symbol_id",
    "valueid_allocated",
    "mutation_performed",
    "mutation_kind_code",
    "local_ssa_finalize_compare_executed",
    "mir_compare_emitted",
    "mir_branch_emitted",
    "route_selection",
    "runtime_route_switch",
    "programjson_runtime_authority",
    "source_selfhost_claim",
]:
    need(field in required, f"missing response field: {field}")
need(response_schema.get("mutation_kind_codes", {}).get("1") == "ConstInstructionOnly", "missing const-only mutation code")

claims = fixture.get("claims") or {}
for key in [
    "compare_rhs_valueid_resolution_request_response_abi",
    "request_from_resolution_plan_snapshot",
    "response_schema_fixed",
    "literal_i64_request_shape",
    "symbol_ref_request_shape",
    "mutation_flags_explicit",
]:
    need(claims.get(key) == 1, f"missing positive claim: {key}")
for key in [
    "actual_rhs_valueid_resolution",
    "literal_constant_valueid_allocation",
    "constant_mir_emission",
    "symbol_lookup_execution",
    "local_ssa_finalize_compare_execution",
    "mir_cmp_emission",
    "branch_emission",
    "basic_block_mutation",
    "value_id_allocation",
    "route_selection",
    "runtime_route_switch",
    "programjson_runtime_route_authority",
    "runtime_fallback",
    "source_selfhost_claim",
]:
    need(claims.get(key) == 0, f"forbidden claim drift: {key}")

for needle in [
    "build_request(program_json): MapBox",
    "CompareRhsValueIdResolutionPlanSnapshotBox.build_plan",
    "build_request_from_plan(plan): MapBox",
    '"request_ready" => 1',
    '"actual_resolution_executed" => 0',
    '"rhs_value_id_resolution" => 0',
    '"literal_constant_value_id_allocation" => 0',
    '"constant_mir_emission" => 0',
    '"symbol_lookup_execution" => 0',
    '"local_ssa_finalize_compare_execution" => 0',
    '"mir_cmp_emission" => 0',
    '"value_id_allocation" => 0',
    "request_summary(request)",
]:
    need(needle in impl, f"implementation missing token: {needle}")
for forbidden in [
    "MirInstruction",
    "emit_mir",
    "emit_compare",
    "emit_branch",
    "next_value_id",
    "finalize_compare(",
]:
    need(forbidden not in impl, f"forbidden implementation token: {forbidden}")
PY

TMP_DIR="$(mktemp -d /tmp/hakorune-rhs-resolution-abi.XXXXXX)"
cleanup() { rm -rf "$TMP_DIR" >/dev/null 2>&1 || true; }
trap cleanup EXIT

APP="$TMP_DIR/rhs_resolution_abi.hako"
EXPECTED="$TMP_DIR/expected.txt"
ACTUAL="$TMP_DIR/actual.txt"
EXE="$TMP_DIR/rhs_resolution_abi.exe"
EMIT_LOG="$TMP_DIR/emit.log"

python3 - "$FIXTURE" "$APP" "$EXPECTED" <<'PY'
import json
import sys
from pathlib import Path

fixture = json.loads(Path(sys.argv[1]).read_text(encoding="utf-8"))
app = Path(sys.argv[2])
expected = Path(sys.argv[3])

lines = [
    "using lang.compiler.mirbuilder.compare_rhs_valueid_resolution_request_snapshot as CompareRhsValueIdResolutionRequestSnapshotBox",
    "",
    "static box Main {",
    "  main() {",
]
expected_lines = []
for idx, row in enumerate(fixture["request_rows"]):
    plan_entries = ", ".join(
        json.dumps(key) + " => " + str(value)
        for key, value in row["plan"].items()
    )
    lines.append(f"    local plan_{idx} = %{{{plan_entries}}}")
    lines.append(f"    local request_{idx} = CompareRhsValueIdResolutionRequestSnapshotBox.build_request_from_plan(plan_{idx})")
    lines.append(
        f"    print(\"rhs_resolution_request:{row['row_id']}:\" + CompareRhsValueIdResolutionRequestSnapshotBox.request_summary(request_{idx}))"
    )
    expected_lines.append(f"rhs_resolution_request:{row['row_id']}:{row['expected_request_summary']}")

lines.extend(["    return 0", "  }", "}", ""])
app.write_text("\n".join(lines), encoding="utf-8")
expected.write_text("\n".join(expected_lines) + "\n", encoding="utf-8")
PY

bash "$HAKO_BIN" --backend mir --verify "$IMPL" >/dev/null

if ! bash "$HAKO_BIN" --backend mir --emit-exe "$EXE" "$APP" >"$EMIT_LOG" 2>&1; then
  tail -n 160 "$EMIT_LOG" || true
  guard_fail "$TAG" "failed to emit RHS resolution ABI executable"
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
    print("[compare-rhs/valueid-resolution-abi] mismatch")
    for idx in range(max(len(expected), len(actual))):
        exp = expected[idx] if idx < len(expected) else "<missing>"
        got = actual[idx] if idx < len(actual) else "<missing>"
        if exp != got:
            print(f"row={idx} expected={exp!r} actual={got!r}")
    raise SystemExit(1)
PY

cat <<'REPORT'
output_contract=rust-lifecycle-mirbuilder-compare-rhs-valueid-resolution-request-response-abi-gate-v0
token=MIRBUILDER-COMPARE-RHS-VALUEID-RESOLUTION-REQUEST-RESPONSE-ABI-001
owner=CompareRhsValueIdResolutionRequestSnapshotBox
request_rows=2
compare_rhs_valueid_resolution_request_response_abi=1
request_from_resolution_plan_snapshot=1
response_schema_fixed=1
literal_i64_request_shape=1
symbol_ref_request_shape=1
mutation_flags_explicit=1
actual_rhs_valueid_resolution=0
literal_constant_valueid_allocation=0
constant_mir_emission=0
symbol_lookup_execution=0
local_ssa_finalize_compare_execution=0
mir_cmp_emission=0
branch_emission=0
basic_block_mutation=0
value_id_allocation=0
route_selection=0
runtime_route_switch=0
programjson_runtime_route_authority=0
runtime_fallback=0
source_selfhost_claim=0
selected_next_card=MIRBUILDER-COMPARE-RHS-LITERAL-I64-CONSTANT-EMISSION-BRIDGE-001
summary=ok
REPORT
