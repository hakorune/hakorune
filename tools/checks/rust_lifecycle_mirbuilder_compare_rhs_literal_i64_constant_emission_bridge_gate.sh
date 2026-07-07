#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="rust-lifecycle-mirbuilder-compare-rhs-literal-i64-constant-emission-bridge-gate"

source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

FIXTURE="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-compare-rhs-literal-i64-constant-emission-bridge-v0.json"
IMPL="$ROOT_DIR/src/mir/builder/compare_rhs_valueid_resolution_bridge.rs"
ABI_GATE="$ROOT_DIR/tools/checks/rust_lifecycle_mirbuilder_compare_rhs_valueid_resolution_request_response_abi_gate.sh"

guard_require_command "$TAG" python3
guard_require_command "$TAG" cargo
guard_require_files "$TAG" "$FIXTURE" "$IMPL" "$ABI_GATE"

ABI_OUT="$(guard_cached_run "$TAG" bash "$ABI_GATE")"
if ! grep -q '^compare_rhs_valueid_resolution_request_response_abi=1$' <<<"$ABI_OUT"; then
  printf '%s\n' "$ABI_OUT" >&2
  guard_fail "$TAG" "RHS ValueId resolution ABI prerequisite is not green"
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
need(fixture.get("kind") == "MirBuilderCompareRhsLiteralI64ConstantEmissionBridgeV1", "bad kind")
need(fixture.get("token") == "MIRBUILDER-COMPARE-RHS-LITERAL-I64-CONSTANT-EMISSION-BRIDGE-001", "bad token")
need(fixture.get("owner") == "CompareRhsConstantEmissionBridge", "bad owner")
need(fixture.get("response_contract") == "CompareRhsValueIdResolutionResponseV1", "bad response contract")

row = fixture.get("row") or {}
expected = row.get("expected_response") or {}
need(row.get("request_kind") == "LiteralConstantResolution", "bad request kind")
need(row.get("bound_kind") == "LiteralI64", "bad bound kind")
need(expected.get("allocated_valueid_delta") == 1, "missing ValueId delta expectation")
need(expected.get("const_instruction_count_delta") == 1, "missing Const delta expectation")
need(expected.get("const_kind") == "Integer", "missing Integer const expectation")
need(expected.get("mutation_performed_const_only") == 1, "missing const-only mutation expectation")

claims = fixture.get("claims") or {}
for key in [
    "actual_rhs_valueid_resolution_literal_i64",
    "literal_i64_constant_emission_bridge",
    "literal_constant_valueid_allocation",
    "constant_mir_emission",
    "integer_type_publication",
    "rhs_value_id_present",
    "mutation_performed_const_only",
]:
    need(claims.get(key) == 1, f"missing positive claim: {key}")
for key in [
    "actual_rhs_valueid_resolution_general",
    "symbol_ref_valueid_resolution",
    "symbol_lookup_execution",
    "local_ssa_finalize_compare",
    "mir_compare_emission",
    "mir_branch_emission",
    "basicblock_control_flow_mutation",
    "route_selection",
    "runtime_route_switch",
    "programjson_runtime_route_authority",
    "runtime_fallback",
    "source_selfhost_claim",
]:
    need(claims.get(key) == 0, f"forbidden claim drift: {key}")

for needle in [
    "CompareRhsConstantEmissionBridge",
    "resolve_literal_i64",
    "emission::constant::emit_integer",
    "CompareRhsValueIdResolutionResponse",
    "rhs_value_id_present: true",
    "emitted_constant: true",
    "valueid_allocated: true",
    "mutation_performed: true",
    "MUTATION_KIND_CONST_INSTRUCTION_ONLY",
    "used_symbol_lookup: false",
    "local_ssa_finalize_compare_executed: false",
    "mir_compare_emitted: false",
    "mir_branch_emitted: false",
    "runtime_route_switch: false",
    "programjson_runtime_authority: false",
]:
    need(needle in impl, f"implementation missing token: {needle}")
for forbidden in [
    "finalize_compare(",
    "emit_to(",
    "emit_branch",
    "build_comparison_op",
    "variable_ctx",
    "declare_local",
    "route_loop",
]:
    need(forbidden not in impl, f"forbidden implementation token: {forbidden}")
PY

cargo test -q --lib compare_rhs_literal_i64_bridge_emits_const_only -- --nocapture

cat <<'REPORT'
output_contract=rust-lifecycle-mirbuilder-compare-rhs-literal-i64-constant-emission-bridge-v0
token=MIRBUILDER-COMPARE-RHS-LITERAL-I64-CONSTANT-EMISSION-BRIDGE-001
owner=CompareRhsConstantEmissionBridge
literal_i64_constant_emission_bridge=1
actual_rhs_valueid_resolution_literal_i64=1
rhs_value_id_present=1
rhs_value_id_nonzero=1
valueid_allocated=1
allocated_valueid_delta=1
constant_mir_emission=1
const_instruction_count_delta=1
const_kind=Integer
type_ctx_integer=1
mutation_performed_const_only=1
actual_rhs_valueid_resolution_general=0
symbol_ref_valueid_resolution=0
symbol_lookup_execution=0
local_ssa_finalize_compare=0
mir_compare_emission=0
mir_branch_emission=0
basicblock_control_flow_mutation=0
route_selection=0
runtime_route_switch=0
programjson_runtime_route_authority=0
runtime_fallback=0
source_selfhost_claim=0
selected_next_card=MIRBUILDER-COMPARE-RHS-SYMBOLREF-LOOKUP-CONTRACT-CONSULTATION-001
summary=ok
REPORT
