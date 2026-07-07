#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="rust-lifecycle-mirbuilder-hard-authority-pilot-compare-rhs-symbolref-lookup-bridge-guard"

source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

CARD="$ROOT_DIR/docs/development/current/main/phases/phase-296x/3337-MIRBUILDER-HARD-AUTHORITY-PILOT-COMPARE-RHS-SYMBOLREF-LOOKUP-BRIDGE-001.md"
FIXTURE="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-hard-authority-pilot-compare-rhs-symbolref-lookup-bridge-v0.json"
PREV_GUARD="$ROOT_DIR/tools/checks/rust_lifecycle_mirbuilder_hard_authority_pilot_compare_rhs_symbolref_lookup_contract_parity_guard.sh"
BRIDGE_GUARD="$ROOT_DIR/tools/checks/rust_lifecycle_mirbuilder_compare_rhs_symbolref_lookup_bridge_gate.sh"
STATE="$ROOT_DIR/docs/development/current/main/CURRENT_STATE.toml"
TASK_ORDER="$ROOT_DIR/docs/development/current/main/design/mirbuilder-rust-to-hako-converter-task-order-ssot.md"
INDEX="$ROOT_DIR/docs/tools/check-scripts-index.md"

guard_require_command "$TAG" python3
guard_require_files "$TAG" "$CARD" "$FIXTURE" "$PREV_GUARD" "$BRIDGE_GUARD" "$STATE" "$TASK_ORDER" "$INDEX"

PREV_OUT="$(guard_cached_run "$TAG" bash "$PREV_GUARD")"
if ! grep -q '^compare_rhs_symbolref_lookup_contract_parity_owner=1$' <<<"$PREV_OUT"; then
  printf '%s\n' "$PREV_OUT" >&2
  guard_fail "$TAG" "SymbolRef contract parity hard-authority prerequisite is not green"
fi

BRIDGE_OUT="$(guard_cached_run "$TAG" bash "$BRIDGE_GUARD")"
if ! grep -q '^symbol_ref_valueid_resolution_no_shadow_local=1$' <<<"$BRIDGE_OUT"; then
  printf '%s\n' "$BRIDGE_OUT" >&2
  guard_fail "$TAG" "SymbolRef lookup bridge evidence is not green"
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

token = "MIRBUILDER-HARD-AUTHORITY-PILOT-COMPARE-RHS-SYMBOLREF-LOOKUP-BRIDGE-001"
output_contract = "rust-lifecycle-mirbuilder-hard-authority-pilot-compare-rhs-symbolref-lookup-bridge-v0"
blocker = "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001"
card_path = "docs/development/current/main/phases/phase-296x/3337-MIRBUILDER-HARD-AUTHORITY-PILOT-COMPARE-RHS-SYMBOLREF-LOOKUP-BRIDGE-001.md"

need(f"# 3337 - {token}" in card, "card token drift")
need(fixture.get("kind") == "MirBuilderHardAuthorityPilotCompareRhsSymbolRefLookupBridgeV1", "fixture kind drift")
need(fixture.get("token") == token, "fixture token drift")
need(fixture.get("output_contract") == output_contract, "fixture output contract drift")
need((fixture.get("current_state") or {}).get("current_blocker_token") == blocker, "fixture blocker drift")
need((fixture.get("pilot") or {}).get("candidate_id") == "CompareRhsSymbolRefLookupBridgeBoundary", "candidate drift")
need((fixture.get("pilot") or {}).get("lookup_scope") == "NoShadowLocalOnly", "lookup scope drift")
need((fixture.get("pilot") or {}).get("mutation_allowed") is False, "mutation must stay forbidden")

claims = fixture.get("claims") or {}
for key in [
    "hard_authority_pilot_implemented",
    "compare_rhs_symbolref_lookup_bridge_owner",
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
    need(claims.get(key) == 1, f"positive claim drift: {key}")
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
    "new_backend_route",
    "new_abi",
]:
    need(claims.get(key) == 0, f"forbidden claim drift: {key}")

need(f'latest_card = "{token}"' in state, "CURRENT_STATE latest card drift")
need(f'latest_card_path = "{card_path}"' in state, "CURRENT_STATE latest path drift")
need(f'current_blocker_token = "{blocker}"' in state, "CURRENT_STATE blocker drift")

for needle in [
    token,
    output_contract,
    "compare_rhs_symbolref_lookup_bridge_owner = 1",
    "symbol_ref_valueid_resolution_no_shadow_local = 1",
    "symbol_ref_valueid_resolution_general = 0",
    "local_ssa_finalize_compare = 0",
    "source_selfhost_claim = 0",
    "MIRBUILDER-COMPARE-LOCALSSA-FINALIZE-COMPARE-DESIGN-STOP-001",
]:
    need(needle in task_order, f"task-order missing {needle}")

need("tools/checks/rust_lifecycle_mirbuilder_hard_authority_pilot_compare_rhs_symbolref_lookup_bridge_guard.sh" in index, "check index missing guard")
PY

cat <<'REPORT'
output_contract=rust-lifecycle-mirbuilder-hard-authority-pilot-compare-rhs-symbolref-lookup-bridge-v0
token=MIRBUILDER-HARD-AUTHORITY-PILOT-COMPARE-RHS-SYMBOLREF-LOOKUP-BRIDGE-001
owner=CompareRhsSymbolRefLookupBridge
hard_authority_pilot_implemented=1
compare_rhs_symbolref_lookup_bridge_owner=1
symbol_ref_valueid_resolution_no_shadow_local=1
symbol_lookup_execution=1
existing_valueid_returned=1
rhs_value_id_present=1
rhs_value_id_nonzero=1
contract_verified_symbol_lookup=1
symbol_ref_valueid_resolution_general=0
shadowing_symbol_lookup=0
current_ssa_authority=0
local_ssa_finalize_compare=0
valueid_allocated=0
constant_mir_emission=0
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
selected_next_card=MIRBUILDER-COMPARE-LOCALSSA-FINALIZE-COMPARE-DESIGN-STOP-001
summary=ok
REPORT
