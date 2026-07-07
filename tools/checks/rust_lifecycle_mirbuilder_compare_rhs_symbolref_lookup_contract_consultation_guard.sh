#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="rust-lifecycle-mirbuilder-compare-rhs-symbolref-lookup-contract-consultation-guard"

source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

FIXTURE="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-compare-rhs-symbolref-lookup-contract-consultation-v0.json"
CARD="$ROOT_DIR/docs/development/current/main/phases/phase-296x/3312-MIRBUILDER-COMPARE-RHS-SYMBOLREF-LOOKUP-CONTRACT-CONSULTATION-001.md"
CURRENT_STATE="$ROOT_DIR/docs/development/current/main/CURRENT_STATE.toml"
TASK_ORDER="$ROOT_DIR/docs/development/current/main/design/mirbuilder-rust-to-hako-converter-task-order-ssot.md"
PREV_GATE="$ROOT_DIR/tools/checks/rust_lifecycle_mirbuilder_compare_rhs_literal_i64_constant_emission_bridge_gate.sh"

guard_require_command "$TAG" python3
guard_require_files "$TAG" "$FIXTURE" "$CARD" "$CURRENT_STATE" "$TASK_ORDER" "$PREV_GATE"

PREV_OUT="$(guard_cached_run "$TAG" bash "$PREV_GATE")"
if ! grep -q '^literal_i64_constant_emission_bridge=1$' <<<"$PREV_OUT"; then
  printf '%s\n' "$PREV_OUT" >&2
  guard_fail "$TAG" "LiteralI64 constant emission bridge prerequisite is not green"
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
need(fixture.get("kind") == "MirBuilderCompareRhsSymbolRefLookupContractConsultationV1", "bad kind")
need(fixture.get("token") == "MIRBUILDER-COMPARE-RHS-SYMBOLREF-LOOKUP-CONTRACT-CONSULTATION-001", "bad token")
need(fixture.get("prerequisite") == "MIRBUILDER-COMPARE-RHS-LITERAL-I64-CONSTANT-EMISSION-BRIDGE-001", "bad prerequisite")

candidates = {row.get("name"): row for row in fixture.get("candidates") or []}
need(candidates["SymbolTableContractFirst"].get("selected") is True, "B must be selected")
need(
    candidates["SymbolTableContractFirst"].get("selected_next_card")
    == "MIRBUILDER-COMPARE-RHS-SYMBOLREF-LOOKUP-CONTRACT-PARITY-001",
    "bad selected next",
)
for name in [
    "NameBasedNarrowBridge",
    "RuntimeAdjacentRustOracleShadow",
    "OpenSymbolRefLookupImmediately",
]:
    need(candidates[name].get("selected") is False, f"{name} must not be selected")

contract = fixture.get("required_contract") or {}
need(contract.get("name") == "SymbolRefResolutionContractV1", "bad contract name")
fields = set(contract.get("fields") or [])
for field in [
    "symbol_id",
    "source_name",
    "expected_rust_variable_key",
    "rust_variable_key_present",
    "rust_current_valueid_present",
    "rust_current_valueid_nonzero",
    "scope_contract_kind",
    "shadowing_claimed",
    "current_ssa_claimed",
    "local_ssa_materialization_claimed",
    "readonly",
]:
    need(field in fields, f"missing contract field: {field}")
need(contract.get("scope_contract_kinds", {}).get("1") == "same_function_local_no_shadow", "missing no-shadow local scope kind")
need(contract.get("scope_contract_kinds", {}).get("3") == "shadowed_name_not_claimed", "missing shadowed non-claim scope kind")

sequence = fixture.get("selected_sequence") or []
need(sequence == [
    "MIRBUILDER-COMPARE-RHS-SYMBOLREF-LOOKUP-CONTRACT-PARITY-001",
    "MIRBUILDER-COMPARE-RHS-SYMBOLREF-LOOKUP-BRIDGE-001",
    "MIRBUILDER-COMPARE-LOCALSSA-FINALIZE-COMPARE-DESIGN-STOP-001",
], "bad selected sequence")

claims = fixture.get("claims") or {}
for key in [
    "symbolref_lookup_contract_consultation",
    "symbol_table_contract_first_selected",
    "c_style_oracle_evidence_folded_into_3313",
]:
    need(claims.get(key) == 1, f"missing positive claim: {key}")
for key in [
    "symbolref_actual_lookup",
    "symbol_ref_valueid_resolution",
    "symbol_lookup_execution",
    "name_fallback_authority",
    "shadowing_symbol_lookup",
    "current_ssa_authority",
    "local_ssa_finalize_compare",
    "mir_compare_emission",
    "mir_branch_emission",
    "route_selection",
    "runtime_route_switch",
    "programjson_runtime_authority",
    "runtime_fallback",
    "source_selfhost_claim",
]:
    need(claims.get(key) == 0, f"forbidden claim drift: {key}")

allowed_latest = [
    'latest_card = "MIRBUILDER-COMPARE-RHS-LITERAL-I64-CONSTANT-EMISSION-BRIDGE-001"',
    'latest_card = "MIRBUILDER-HARD-AUTHORITY-PILOT-COMPARE-RHS-LITERAL-I64-CONSTANT-EMISSION-BRIDGE-001"',
    'latest_card = "MIRBUILDER-COMPARE-RHS-SYMBOLREF-LOOKUP-CONTRACT-CONSULTATION-001"',
    'latest_card = "MIRBUILDER-COMPARE-RHS-SYMBOLREF-LOOKUP-CONTRACT-PARITY-001"',
    'latest_card = "MIRBUILDER-HARD-AUTHORITY-PILOT-COMPARE-RHS-SYMBOLREF-LOOKUP-CONTRACT-PARITY-001"',
    'latest_card = "MIRBUILDER-COMPARE-RHS-SYMBOLREF-LOOKUP-BRIDGE-001"',
    'latest_card = "MIRBUILDER-COMPARE-LOCALSSA-FINALIZE-COMPARE-DESIGN-STOP-001"',
    'latest_card = "MIRBUILDER-COMPARE-LOCALSSA-FINALIZE-COMPARE-BRIDGE-001"',
    'latest_card = "MIRBUILDER-COMPARE-MIR-COMPARE-EMISSION-BRIDGE-001"',
    'latest_card = "MIRBUILDER-COMPARE-BRANCH-EMISSION-DESIGN-STOP-001"',
    'latest_card = "MIRBUILDER-COMPARE-BRANCH-EMISSION-BRIDGE-001"',
    'latest_card = "MIRBUILDER-COMPARE-BOOLRECIPE-TO-MIR-COMPARE-BRANCH-CLOSEOUT-001"',
    'latest_card = "MIRBUILDER-COMPARE-RUNTIME-ROUTE-AUTHORITY-DESIGN-STOP-001"',
]
need(any(entry in current_state for entry in allowed_latest), "CURRENT_STATE latest card must point to prerequisite or 3312")
need("B_SYMBOL_TABLE_CONTRACT_FIRST" in card, "card must record selected B")
need("actual SymbolRef lookup: `0`" in card, "card must keep actual lookup unclaimed")
need("MIRBUILDER-COMPARE-RHS-SYMBOLREF-LOOKUP-CONTRACT-PARITY-001" in task_order, "task-order must name 3313")
need("MIRBUILDER-COMPARE-RHS-SYMBOLREF-LOOKUP-BRIDGE-001" in task_order, "task-order must name 3314")
PY

cat <<'REPORT'
output_contract=rust-lifecycle-mirbuilder-compare-rhs-symbolref-lookup-contract-consultation-v0
token=MIRBUILDER-COMPARE-RHS-SYMBOLREF-LOOKUP-CONTRACT-CONSULTATION-001
decision=SelectSymbolTableContractFirst
symbolref_lookup_contract_consultation=1
symbol_table_contract_first_selected=1
c_style_oracle_evidence_folded_into_3313=1
symbolref_actual_lookup=0
symbol_ref_valueid_resolution=0
symbol_lookup_execution=0
name_fallback_authority=0
shadowing_symbol_lookup=0
current_ssa_authority=0
local_ssa_finalize_compare=0
mir_compare_emission=0
mir_branch_emission=0
route_selection=0
runtime_route_switch=0
programjson_runtime_authority=0
runtime_fallback=0
source_selfhost_claim=0
selected_next_card=MIRBUILDER-COMPARE-RHS-SYMBOLREF-LOOKUP-CONTRACT-PARITY-001
summary=ok
REPORT
