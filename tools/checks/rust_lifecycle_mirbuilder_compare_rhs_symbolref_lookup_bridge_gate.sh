#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="rust-lifecycle-mirbuilder-compare-rhs-symbolref-lookup-bridge-gate"

source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

FIXTURE="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-compare-rhs-symbolref-lookup-bridge-v0.json"
IMPL="$ROOT_DIR/src/mir/builder/compare_rhs_symbolref_lookup_bridge.rs"
PARITY_GATE="$ROOT_DIR/tools/checks/rust_lifecycle_mirbuilder_compare_rhs_symbolref_lookup_contract_parity_gate.sh"

guard_require_command "$TAG" python3
guard_require_command "$TAG" cargo
guard_require_files "$TAG" "$FIXTURE" "$IMPL" "$PARITY_GATE"

PARITY_OUT="$(guard_cached_run "$TAG" bash "$PARITY_GATE")"
if ! grep -q '^symbol_ref_resolution_contract_v1=1$' <<<"$PARITY_OUT"; then
  printf '%s\n' "$PARITY_OUT" >&2
  guard_fail "$TAG" "SymbolRef contract parity prerequisite is not green"
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
need(fixture.get("kind") == "MirBuilderCompareRhsSymbolRefLookupBridgeV1", "bad kind")
need(fixture.get("token") == "MIRBUILDER-COMPARE-RHS-SYMBOLREF-LOOKUP-BRIDGE-001", "bad token")
need(fixture.get("owner") == "CompareRhsSymbolRefLookupBridge", "bad owner")
need(fixture.get("input_contract") == "SymbolRefResolutionContractV1", "bad input contract")
need(fixture.get("response_contract") == "CompareRhsValueIdResolutionResponseV1", "bad response contract")

rows = fixture.get("rows") or []
need([row.get("row_id") for row in rows] == [
    "simple_local_i_lookup",
    "renamed_local_count_lookup",
    "unmapped_symbol_id_rejects",
], "row set drift")

claims = fixture.get("claims") or {}
for key in [
    "symbol_ref_valueid_resolution_no_shadow_local",
    "symbol_lookup_execution",
    "existing_valueid_returned",
    "rhs_value_id_present",
    "rhs_value_id_nonzero",
    "contract_verified_symbol_lookup",
    "simple_local_i_lookup",
    "renamed_local_count_lookup",
    "unmapped_symbol_id_rejects",
]:
    need(claims.get(key) == 1, f"missing positive claim: {key}")
for key in [
    "symbol_ref_valueid_resolution_general",
    "shadowing_symbol_lookup",
    "current_ssa_authority",
    "local_ssa_finalize_compare",
    "valueid_allocated",
    "literal_constant_valueid_allocation",
    "constant_mir_emission",
    "mir_compare_emission",
    "mir_branch_emission",
    "basicblock_mutation",
    "route_selection",
    "runtime_route_switch",
    "programjson_runtime_authority",
    "runtime_fallback",
    "source_selfhost_claim",
]:
    need(claims.get(key) == 0, f"forbidden claim drift: {key}")

for needle in [
    "CompareRhsSymbolRefLookupBridge",
    "resolve_symbol_ref_no_shadow_local",
    "SymbolRefResolutionContract",
    "CompareRhsValueIdResolutionResponse",
    "SCOPE_SAME_FUNCTION_LOCAL_NO_SHADOW",
    "SCOPE_RENAMED_LOCAL_NO_SHADOW",
    "used_symbol_lookup: resolved",
    "valueid_allocated: false",
    "mutation_performed: false",
    "mutation_kind_code: 0",
    "local_ssa_finalize_compare_executed: false",
    "mir_compare_emitted: false",
    "mir_branch_emitted: false",
    "runtime_route_switch: false",
    "programjson_runtime_authority: false",
]:
    need(needle in impl, f"implementation missing token: {needle}")
for forbidden in [
    "emit_integer",
    "emit_compare",
    "emit_branch",
    "finalize_compare(",
    "next_value_id(",
    "alloc_typed",
    "emit_instruction",
]:
    need(forbidden not in impl, f"forbidden implementation token: {forbidden}")
PY

cargo test -q --lib symbolref_lookup_bridge_returns_existing_no_shadow_valueids_only -- --nocapture

cat <<'REPORT'
output_contract=rust-lifecycle-mirbuilder-compare-rhs-symbolref-lookup-bridge-v0
token=MIRBUILDER-COMPARE-RHS-SYMBOLREF-LOOKUP-BRIDGE-001
owner=CompareRhsSymbolRefLookupBridge
symbol_ref_valueid_resolution_no_shadow_local=1
symbol_lookup_execution=1
existing_valueid_returned=1
rhs_value_id_present=1
rhs_value_id_nonzero=1
contract_verified_symbol_lookup=1
simple_local_i_lookup=1
renamed_local_count_lookup=1
unmapped_symbol_id_rejects=1
symbol_ref_valueid_resolution_general=0
shadowing_symbol_lookup=0
current_ssa_authority=0
local_ssa_finalize_compare=0
valueid_allocated=0
literal_constant_valueid_allocation=0
constant_mir_emission=0
mir_compare_emission=0
mir_branch_emission=0
basicblock_mutation=0
route_selection=0
runtime_route_switch=0
programjson_runtime_authority=0
runtime_fallback=0
source_selfhost_claim=0
selected_next_card=MIRBUILDER-COMPARE-LOCALSSA-FINALIZE-COMPARE-DESIGN-STOP-001
summary=ok
REPORT
