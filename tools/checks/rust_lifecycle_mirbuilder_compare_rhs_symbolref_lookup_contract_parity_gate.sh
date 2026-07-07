#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="rust-lifecycle-mirbuilder-compare-rhs-symbolref-lookup-contract-parity-gate"

source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

FIXTURE="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-compare-rhs-symbolref-lookup-contract-parity-v0.json"
IMPL="$ROOT_DIR/src/mir/builder/compare_rhs_symbolref_contract.rs"
CONSULT_GATE="$ROOT_DIR/tools/checks/rust_lifecycle_mirbuilder_compare_rhs_symbolref_lookup_contract_consultation_guard.sh"

guard_require_command "$TAG" python3
guard_require_command "$TAG" cargo
guard_require_files "$TAG" "$FIXTURE" "$IMPL" "$CONSULT_GATE"

CONSULT_OUT="$(guard_cached_run "$TAG" bash "$CONSULT_GATE")"
if ! grep -q '^symbol_table_contract_first_selected=1$' <<<"$CONSULT_OUT"; then
  printf '%s\n' "$CONSULT_OUT" >&2
  guard_fail "$TAG" "SymbolRef contract consultation prerequisite is not green"
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
need(fixture.get("kind") == "MirBuilderCompareRhsSymbolRefLookupContractParityV1", "bad kind")
need(fixture.get("token") == "MIRBUILDER-COMPARE-RHS-SYMBOLREF-LOOKUP-CONTRACT-PARITY-001", "bad token")
need(fixture.get("owner") == "CompareRhsSymbolRefContractObserver", "bad owner")
need(fixture.get("contract") == "SymbolRefResolutionContractV1", "bad contract")

rows = fixture.get("rows") or []
need([row.get("row_id") for row in rows] == [
    "simple_local_i",
    "renamed_local_count",
    "shadowed_name_not_claimed",
], "row set drift")
need(rows[0].get("symbol_id") == 1 and rows[0].get("expected_rust_variable_key") == "i", "bad simple row")
need(rows[1].get("symbol_id") == 3 and rows[1].get("expected_rust_variable_key") == "count", "bad renamed row")
need(rows[2].get("scope_contract_kind") == "shadowed_name_not_claimed", "bad shadow row")

claims = fixture.get("claims") or {}
for key in [
    "symbol_ref_resolution_contract_v1",
    "symbol_id_to_source_name_mapping",
    "source_name_to_expected_rust_variable_key_mapping",
    "rust_variable_key_readonly_observed",
    "rust_current_valueid_readonly_observed",
    "simple_local_row",
    "renamed_local_row",
    "shadowed_name_not_claimed_row",
    "rust_oracle_current_valueid_observed",
    "programjson_symbol_contract_matches_rust_observation",
]:
    need(claims.get(key) == 1, f"missing positive claim: {key}")
need(claims.get("contract_verified_rows") == 2, "contract_verified_rows must stay 2")
for key in [
    "symbol_lookup_execution",
    "symbol_ref_valueid_resolution",
    "existing_valueid_returned_as_bridge_response",
    "valueid_allocated",
    "constant_mir_emission",
    "local_ssa_finalize_compare",
    "mir_compare_emission",
    "mir_branch_emission",
    "basicblock_mutation",
    "route_selection",
    "runtime_route_switch",
    "programjson_runtime_authority",
    "source_selfhost_claim",
]:
    need(claims.get(key) == 0, f"forbidden claim drift: {key}")

for needle in [
    "SymbolRefResolutionContract",
    "CompareRhsSymbolRefContractObserver",
    "observe_no_shadow_local",
    "shadowed_name_not_claimed",
    "SCOPE_SAME_FUNCTION_LOCAL_NO_SHADOW",
    "SCOPE_RENAMED_LOCAL_NO_SHADOW",
    "SCOPE_SHADOWED_NAME_NOT_CLAIMED",
    "rust_current_valueid_nonzero",
    "local_ssa_materialization_claimed: false",
    "readonly: true",
]:
    need(needle in impl, f"implementation missing token: {needle}")
for forbidden in [
    "emit_integer",
    "emit_compare",
    "emit_branch",
    "finalize_compare(",
    "CompareRhsValueIdResolutionResponse",
    "mutation_performed",
]:
    need(forbidden not in impl, f"forbidden implementation token: {forbidden}")
PY

cargo test -q --lib symbolref_contract_observes_simple_and_renamed_locals_readonly -- --nocapture

cat <<'REPORT'
output_contract=rust-lifecycle-mirbuilder-compare-rhs-symbolref-lookup-contract-parity-v0
token=MIRBUILDER-COMPARE-RHS-SYMBOLREF-LOOKUP-CONTRACT-PARITY-001
owner=CompareRhsSymbolRefContractObserver
symbol_ref_resolution_contract_v1=1
symbol_id_to_source_name_mapping=1
source_name_to_expected_rust_variable_key_mapping=1
rust_variable_key_readonly_observed=1
rust_current_valueid_readonly_observed=1
contract_verified_rows=2
simple_local_row=1
renamed_local_row=1
shadowed_name_not_claimed_row=1
rust_oracle_current_valueid_observed=1
programjson_symbol_contract_matches_rust_observation=1
symbol_lookup_execution=0
symbol_ref_valueid_resolution=0
existing_valueid_returned_as_bridge_response=0
valueid_allocated=0
constant_mir_emission=0
local_ssa_finalize_compare=0
mir_compare_emission=0
mir_branch_emission=0
basicblock_mutation=0
route_selection=0
runtime_route_switch=0
programjson_runtime_authority=0
source_selfhost_claim=0
selected_next_card=MIRBUILDER-COMPARE-RHS-SYMBOLREF-LOOKUP-BRIDGE-001
summary=ok
REPORT
