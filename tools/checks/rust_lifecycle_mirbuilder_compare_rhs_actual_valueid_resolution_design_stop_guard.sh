#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="rust-lifecycle-mirbuilder-compare-rhs-actual-valueid-resolution-design-stop-guard"

source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

FIXTURE="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-compare-rhs-actual-valueid-resolution-design-stop-v0.json"
CARD="$ROOT_DIR/docs/development/current/main/phases/phase-296x/3309-MIRBUILDER-COMPARE-RHS-ACTUAL-VALUEID-RESOLUTION-DESIGN-STOP-001.md"
CURRENT_STATE="$ROOT_DIR/docs/development/current/main/CURRENT_STATE.toml"
TASK_ORDER="$ROOT_DIR/docs/development/current/main/design/mirbuilder-rust-to-hako-converter-task-order-ssot.md"
PREV_GATE="$ROOT_DIR/tools/checks/rust_lifecycle_mirbuilder_compare_rhs_materialization_readonly_resolution_parity_gate.sh"

guard_require_command "$TAG" python3
guard_require_files "$TAG" "$FIXTURE" "$CARD" "$CURRENT_STATE" "$TASK_ORDER" "$PREV_GATE"

PREV_OUT="$(guard_cached_run "$TAG" bash "$PREV_GATE")"
if ! grep -q '^compare_rhs_materialization_readonly_resolution_parity=1$' <<<"$PREV_OUT"; then
  printf '%s\n' "$PREV_OUT" >&2
  guard_fail "$TAG" "RHS read-only resolution parity prerequisite is not green"
fi

python3 - "$FIXTURE" "$CARD" "$CURRENT_STATE" "$TASK_ORDER" <<'PY'
import json
import sys
from pathlib import Path

fixture = json.loads(Path(sys.argv[1]).read_text(encoding="utf-8"))
card = Path(sys.argv[2]).read_text(encoding="utf-8")
current_state = Path(sys.argv[3]).read_text(encoding="utf-8")
task_order = Path(sys.argv[4]).read_text(encoding="utf-8")

def need(condition, message):
    if not condition:
        raise SystemExit(message)

need(fixture.get("schema_version") == 0, "bad schema_version")
need(fixture.get("kind") == "MirBuilderCompareRhsActualValueIdResolutionDesignStopV1", "bad kind")
need(fixture.get("token") == "MIRBUILDER-COMPARE-RHS-ACTUAL-VALUEID-RESOLUTION-DESIGN-STOP-001", "bad token")
need(fixture.get("prerequisite") == "MIRBUILDER-COMPARE-RHS-MATERIALIZATION-READONLY-RESOLUTION-PARITY-001", "bad prerequisite")

candidates = {row.get("name"): row for row in fixture.get("candidates") or []}
need(candidates["RequestResponseAbiFirst"].get("selected") is True, "request/response ABI must be selected")
need(candidates["RequestResponseAbiFirst"].get("selected_next_card") == "MIRBUILDER-COMPARE-RHS-VALUEID-RESOLUTION-REQUEST-RESPONSE-ABI-001", "bad selected next")
for name in [
    "LiteralI64ConstantEmissionNow",
    "SymbolRefLookupNow",
    "RustAuthorityShadowResolutionGuard",
    "MoveToAnotherLayer4Owner",
]:
    need(candidates[name].get("selected") is False, f"{name} must not be selected")

sequence = fixture.get("selected_sequence") or []
need(sequence[:2] == [
    "MIRBUILDER-COMPARE-RHS-VALUEID-RESOLUTION-REQUEST-RESPONSE-ABI-001",
    "MIRBUILDER-COMPARE-RHS-LITERAL-I64-CONSTANT-EMISSION-BRIDGE-001",
], "bad selected sequence prefix")
need("MIRBUILDER-COMPARE-LOCALSSA-FINALIZE-COMPARE-DESIGN-STOP-001" in sequence, "LocalSSA design-stop must remain later")

claims = fixture.get("claims") or {}
for key in [
    "compare_rhs_actual_valueid_resolution_design_stop",
    "request_response_abi_selected",
]:
    need(claims.get(key) == 1, f"missing positive claim: {key}")
for key in [
    "literal_i64_constant_emission_selected_now",
    "symbol_ref_lookup_selected_now",
    "rust_authority_shadow_selected_now",
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

allowed_latest = [
    'latest_card = "MIRBUILDER-COMPARE-RHS-MATERIALIZATION-READONLY-RESOLUTION-PARITY-001"',
    'latest_card = "MIRBUILDER-COMPARE-RHS-ACTUAL-VALUEID-RESOLUTION-DESIGN-STOP-001"',
    'latest_card = "MIRBUILDER-COMPARE-RHS-VALUEID-RESOLUTION-REQUEST-RESPONSE-ABI-001"',
]
need(any(entry in current_state for entry in allowed_latest), "CURRENT_STATE latest card must point to prerequisite or design-stop")
need(
    "MIRBUILDER-COMPARE-RHS-ACTUAL-VALUEID-RESOLUTION-DESIGN-STOP-001" in task_order
    or "actual RHS ValueId resolution design-stop" in task_order,
    "task-order must retain design stop evidence",
)
need("MIRBUILDER-COMPARE-RHS-VALUEID-RESOLUTION-REQUEST-RESPONSE-ABI-001" in card, "card must name selected ABI next")
need("actual RHS `ValueId` resolution: `0`" in card, "card must keep actual resolution unclaimed")
PY

cat <<'REPORT'
output_contract=rust-lifecycle-mirbuilder-compare-rhs-actual-valueid-resolution-design-stop-guard-v0
token=MIRBUILDER-COMPARE-RHS-ACTUAL-VALUEID-RESOLUTION-DESIGN-STOP-001
decision=SelectCompareRhsValueIdResolutionRequestResponseAbi
compare_rhs_actual_valueid_resolution_design_stop=1
request_response_abi_selected=1
literal_i64_constant_emission_selected_now=0
symbol_ref_lookup_selected_now=0
rust_authority_shadow_selected_now=0
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
selected_next_card=MIRBUILDER-COMPARE-RHS-VALUEID-RESOLUTION-REQUEST-RESPONSE-ABI-001
summary=ok
REPORT
