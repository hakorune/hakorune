#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="rust-lifecycle-mirbuilder-hard-authority-pilot-compare-rhs-symbolref-lookup-contract-parity-guard"

source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

CARD="$ROOT_DIR/docs/development/current/main/phases/phase-296x/3336-MIRBUILDER-HARD-AUTHORITY-PILOT-COMPARE-RHS-SYMBOLREF-LOOKUP-CONTRACT-PARITY-001.md"
FIXTURE="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-hard-authority-pilot-compare-rhs-symbolref-lookup-contract-parity-v0.json"
PREV_GUARD="$ROOT_DIR/tools/checks/rust_lifecycle_mirbuilder_hard_authority_pilot_compare_rhs_literal_i64_constant_emission_bridge_guard.sh"
CONSULT_GUARD="$ROOT_DIR/tools/checks/rust_lifecycle_mirbuilder_compare_rhs_symbolref_lookup_contract_consultation_guard.sh"
PARITY_GUARD="$ROOT_DIR/tools/checks/rust_lifecycle_mirbuilder_compare_rhs_symbolref_lookup_contract_parity_gate.sh"
STATE="$ROOT_DIR/docs/development/current/main/CURRENT_STATE.toml"
TASK_ORDER="$ROOT_DIR/docs/development/current/main/design/mirbuilder-rust-to-hako-converter-task-order-ssot.md"
INDEX="$ROOT_DIR/docs/tools/check-scripts-index.md"

guard_require_command "$TAG" python3
guard_require_files "$TAG" "$CARD" "$FIXTURE" "$PREV_GUARD" "$CONSULT_GUARD" "$PARITY_GUARD" "$STATE" "$TASK_ORDER" "$INDEX"

PREV_OUT="$(guard_cached_run "$TAG" bash "$PREV_GUARD")"
if ! grep -q '^compare_rhs_literal_i64_const_emission_bridge_owner=1$' <<<"$PREV_OUT"; then
  printf '%s\n' "$PREV_OUT" >&2
  guard_fail "$TAG" "LiteralI64 hard-authority prerequisite is not green"
fi

CONSULT_OUT="$(guard_cached_run "$TAG" bash "$CONSULT_GUARD")"
if ! grep -q '^symbol_table_contract_first_selected=1$' <<<"$CONSULT_OUT"; then
  printf '%s\n' "$CONSULT_OUT" >&2
  guard_fail "$TAG" "SymbolRef consultation prerequisite is not green"
fi

PARITY_OUT="$(guard_cached_run "$TAG" bash "$PARITY_GUARD")"
if ! grep -q '^symbol_ref_resolution_contract_v1=1$' <<<"$PARITY_OUT"; then
  printf '%s\n' "$PARITY_OUT" >&2
  guard_fail "$TAG" "SymbolRef contract parity evidence is not green"
fi

python3 - "$CARD" "$FIXTURE" "$STATE" "$TASK_ORDER" "$INDEX" <<'PY'
import json
import sys
from pathlib import Path

card = Path(sys.argv[1]).read_text(encoding="utf-8")
fixture = json.loads(Path(sys.argv[2]).read_text(encoding="utf-8"))
state = Path(sys.argv[3]).read_text(encoding="utf-8")
task_order = Path(sys.argv[4]).read_text(encoding="utf-8")
index = Path(sys.argv[5]).read_text(encoding="utf-8")

def need(condition: bool, message: str) -> None:
    if not condition:
        raise SystemExit(message)

token = "MIRBUILDER-HARD-AUTHORITY-PILOT-COMPARE-RHS-SYMBOLREF-LOOKUP-CONTRACT-PARITY-001"
output_contract = "rust-lifecycle-mirbuilder-hard-authority-pilot-compare-rhs-symbolref-lookup-contract-parity-v0"
blocker = "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001"
card_path = "docs/development/current/main/phases/phase-296x/3336-MIRBUILDER-HARD-AUTHORITY-PILOT-COMPARE-RHS-SYMBOLREF-LOOKUP-CONTRACT-PARITY-001.md"
follow_on_card = "MIRBUILDER-HARD-AUTHORITY-PILOT-COMPARE-RHS-SYMBOLREF-LOOKUP-BRIDGE-001"
follow_on_path = "docs/development/current/main/phases/phase-296x/3337-MIRBUILDER-HARD-AUTHORITY-PILOT-COMPARE-RHS-SYMBOLREF-LOOKUP-BRIDGE-001.md"

need(f"# 3336 - {token}" in card, "card token drift")
need(output_contract in str(fixture), "fixture output contract drift")
need(fixture.get("kind") == "MirBuilderHardAuthorityPilotCompareRhsSymbolRefLookupContractParityV1", "fixture kind drift")
need(fixture.get("token") == token, "fixture token drift")
need(fixture.get("output_contract") == output_contract, "fixture output contract drift")
need((fixture.get("current_state") or {}).get("current_blocker_token") == blocker, "fixture blocker drift")
need((fixture.get("pilot") or {}).get("candidate_id") == "CompareRhsSymbolRefLookupContractParityBoundary", "candidate drift")
need((fixture.get("pilot") or {}).get("actual_symbol_lookup") is False, "actual lookup must stay false")

claims = fixture.get("claims") or {}
for key in [
    "hard_authority_pilot_implemented",
    "compare_rhs_symbolref_lookup_contract_parity_owner",
    "symbol_ref_resolution_contract_v1",
    "symbol_id_to_source_name_mapping",
    "source_name_to_expected_rust_variable_key_mapping",
    "rust_variable_key_readonly_observed",
    "rust_current_valueid_readonly_observed",
    "rust_oracle_current_valueid_observed",
    "programjson_symbol_contract_matches_rust_observation",
]:
    need(claims.get(key) == 1, f"positive claim drift: {key}")
need(claims.get("contract_verified_rows") == 2, "contract row count drift")
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
    "runtime_fallback",
    "source_selfhost_claim",
    "new_backend_route",
    "new_abi",
]:
    need(claims.get(key) == 0, f"forbidden claim drift: {key}")

need(f'latest_card = "{token}"' in state or f'latest_card = "{follow_on_card}"' in state, "CURRENT_STATE latest card drift")
need(f'latest_card_path = "{card_path}"' in state or f'latest_card_path = "{follow_on_path}"' in state, "CURRENT_STATE latest path drift")
need(f'current_blocker_token = "{blocker}"' in state, "CURRENT_STATE blocker drift")

for needle in [
    token,
    output_contract,
    "compare_rhs_symbolref_lookup_contract_parity_owner = 1",
    "symbol_ref_resolution_contract_v1 = 1",
    "symbol_lookup_execution = 0",
    "symbol_ref_valueid_resolution = 0",
    "source_selfhost_claim = 0",
    "MIRBUILDER-COMPARE-RHS-SYMBOLREF-LOOKUP-BRIDGE-001",
]:
    need(needle in task_order, f"task-order missing {needle}")

need("tools/checks/rust_lifecycle_mirbuilder_hard_authority_pilot_compare_rhs_symbolref_lookup_contract_parity_guard.sh" in index, "check index missing guard")
PY

cat <<'REPORT'
output_contract=rust-lifecycle-mirbuilder-hard-authority-pilot-compare-rhs-symbolref-lookup-contract-parity-v0
token=MIRBUILDER-HARD-AUTHORITY-PILOT-COMPARE-RHS-SYMBOLREF-LOOKUP-CONTRACT-PARITY-001
owner=CompareRhsSymbolRefContractObserver
hard_authority_pilot_implemented=1
compare_rhs_symbolref_lookup_contract_parity_owner=1
symbol_ref_resolution_contract_v1=1
symbol_id_to_source_name_mapping=1
source_name_to_expected_rust_variable_key_mapping=1
rust_variable_key_readonly_observed=1
rust_current_valueid_readonly_observed=1
contract_verified_rows=2
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
runtime_fallback=0
source_selfhost_claim=0
new_backend_route=0
new_abi=0
selected_next_card=MIRBUILDER-COMPARE-RHS-SYMBOLREF-LOOKUP-BRIDGE-001
summary=ok
REPORT
