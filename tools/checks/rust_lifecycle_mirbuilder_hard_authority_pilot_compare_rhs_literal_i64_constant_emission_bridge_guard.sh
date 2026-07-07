#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="rust-lifecycle-mirbuilder-hard-authority-pilot-compare-rhs-literal-i64-constant-emission-bridge-guard"

source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

CARD="$ROOT_DIR/docs/development/current/main/phases/phase-296x/3335-MIRBUILDER-HARD-AUTHORITY-PILOT-COMPARE-RHS-LITERAL-I64-CONSTANT-EMISSION-BRIDGE-001.md"
FIXTURE="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-hard-authority-pilot-compare-rhs-literal-i64-constant-emission-bridge-v0.json"
SELECTION_FIXTURE="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-post-rhs-valueid-request-abi-next-seam-selection-v0.json"
LITERAL_FIXTURE="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-compare-rhs-literal-i64-constant-emission-bridge-v0.json"
IMPL="$ROOT_DIR/src/mir/builder/compare_rhs_valueid_resolution_bridge.rs"
SELECTION_GUARD="$ROOT_DIR/tools/checks/rust_lifecycle_mirbuilder_post_rhs_valueid_request_abi_next_seam_selection_guard.sh"
LITERAL_GUARD="$ROOT_DIR/tools/checks/rust_lifecycle_mirbuilder_compare_rhs_literal_i64_constant_emission_bridge_gate.sh"
STATE="$ROOT_DIR/docs/development/current/main/CURRENT_STATE.toml"
TASK_ORDER="$ROOT_DIR/docs/development/current/main/design/mirbuilder-rust-to-hako-converter-task-order-ssot.md"
INDEX="$ROOT_DIR/docs/tools/check-scripts-index.md"

guard_require_command "$TAG" python3
guard_require_files "$TAG" "$CARD" "$FIXTURE" "$SELECTION_FIXTURE" "$LITERAL_FIXTURE" "$IMPL" "$SELECTION_GUARD" "$LITERAL_GUARD" "$STATE" "$TASK_ORDER" "$INDEX"

SELECTION_OUT="$(guard_cached_run "$TAG" bash "$SELECTION_GUARD")"
if ! grep -q '^post_rhs_valueid_request_abi_next_seam_selected=1$' <<<"$SELECTION_OUT"; then
  printf '%s\n' "$SELECTION_OUT" >&2
  guard_fail "$TAG" "post request-ABI seam selection prerequisite is not green"
fi

LITERAL_OUT="$(guard_cached_run "$TAG" bash "$LITERAL_GUARD")"
if ! grep -q '^literal_i64_constant_emission_bridge=1$' <<<"$LITERAL_OUT"; then
  printf '%s\n' "$LITERAL_OUT" >&2
  guard_fail "$TAG" "LiteralI64 bridge evidence is not green"
fi

python3 - "$CARD" "$FIXTURE" "$SELECTION_FIXTURE" "$LITERAL_FIXTURE" "$IMPL" "$STATE" "$TASK_ORDER" "$INDEX" <<'PY'
import json
import sys
from pathlib import Path

card_path = Path(sys.argv[1])
fixture = json.loads(Path(sys.argv[2]).read_text(encoding="utf-8"))
selection = json.loads(Path(sys.argv[3]).read_text(encoding="utf-8"))
literal = json.loads(Path(sys.argv[4]).read_text(encoding="utf-8"))
impl = Path(sys.argv[5]).read_text(encoding="utf-8")
state = Path(sys.argv[6]).read_text(encoding="utf-8")
task_order = Path(sys.argv[7]).read_text(encoding="utf-8")
index = Path(sys.argv[8]).read_text(encoding="utf-8")
card = card_path.read_text(encoding="utf-8")

def need(condition: bool, message: str) -> None:
    if not condition:
        raise SystemExit(message)

token = "MIRBUILDER-HARD-AUTHORITY-PILOT-COMPARE-RHS-LITERAL-I64-CONSTANT-EMISSION-BRIDGE-001"
output_contract = "rust-lifecycle-mirbuilder-hard-authority-pilot-compare-rhs-literal-i64-constant-emission-bridge-v0"
candidate = "CompareRhsLiteralI64ConstantEmissionBridgeBoundary"
blocker = "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001"
card_rel_path = "docs/development/current/main/phases/phase-296x/3335-MIRBUILDER-HARD-AUTHORITY-PILOT-COMPARE-RHS-LITERAL-I64-CONSTANT-EMISSION-BRIDGE-001.md"

need(f"# 3335 - {token}" in card, "card token drift")
need(output_contract in card, "card output contract drift")
need(candidate in card, "card candidate drift")

need(fixture.get("kind") == "MirBuilderHardAuthorityPilotCompareRhsLiteralI64ConstantEmissionBridgeV1", "fixture kind drift")
need(fixture.get("token") == token, "fixture token drift")
need(fixture.get("output_contract") == output_contract, "fixture output contract drift")
need((fixture.get("current_state") or {}).get("current_blocker_token") == blocker, "fixture blocker drift")

need((selection.get("selected_next_seam") or {}).get("candidate_id") == candidate, "selection candidate drift")
need((selection.get("claims") or {}).get("post_rhs_valueid_request_abi_next_seam_selected") == 1, "selection claim drift")
need((selection.get("claims") or {}).get("next_seam_implemented") == 0, "selection must not implement seam")

need(literal.get("owner") == "CompareRhsConstantEmissionBridge", "literal fixture owner drift")
literal_claims = literal.get("claims") or {}
for key in ["actual_rhs_valueid_resolution_literal_i64", "literal_i64_constant_emission_bridge", "literal_constant_valueid_allocation", "constant_mir_emission", "integer_type_publication", "mutation_performed_const_only"]:
    need(literal_claims.get(key) == 1, f"literal evidence missing {key}")
for key in ["symbol_ref_valueid_resolution", "symbol_lookup_execution", "local_ssa_finalize_compare", "mir_compare_emission", "mir_branch_emission", "basicblock_control_flow_mutation", "source_selfhost_claim"]:
    need(literal_claims.get(key) == 0, f"literal forbidden drift {key}")

pilot = fixture.get("pilot") or {}
need(pilot.get("candidate_id") == candidate, "pilot candidate drift")
need(pilot.get("owner_id") == "CompareRhsConstantEmissionBridge", "pilot owner drift")
need(pilot.get("mutation_scope") == "ConstInstructionOnly", "pilot mutation scope drift")
need(pilot.get("claim_ceiling") == "scoped_mutation_hard_authority_pilot", "claim ceiling drift")

need(len(impl.splitlines()) < 800, "source exceeds 800-line source limit")
for needle in [
    "CompareRhsConstantEmissionBridge",
    "resolve_literal_i64",
    "emission::constant::emit_integer",
    "MUTATION_KIND_CONST_INSTRUCTION_ONLY",
    "used_symbol_lookup: false",
    "local_ssa_finalize_compare_executed: false",
    "mir_compare_emitted: false",
    "mir_branch_emitted: false",
    "runtime_route_switch: false",
    "programjson_runtime_authority: false",
]:
    need(needle in impl, f"implementation token missing: {needle}")

claims = fixture.get("claims") or {}
for key in ["hard_authority_pilot_implemented", "compare_rhs_literal_i64_const_emission_bridge_owner", "actual_rhs_valueid_resolution_literal_i64", "literal_constant_valueid_allocation", "constant_mir_emission", "integer_type_publication", "mutation_performed_const_only"]:
    need(claims.get(key) == 1, f"positive claim drift: {key}")
for key in ["hako_adopted_decision", "source_selfhost_claim", "native_seed_materialization", "actual_rhs_valueid_resolution_general", "symbol_ref_valueid_resolution", "symbol_lookup_execution", "local_ssa_finalize_compare_execution", "mir_cmp_emission", "branch_emission", "basic_block_control_flow_mutation", "route_selection", "runtime_route_switch", "programjson_runtime_route_authority", "runtime_fallback", "new_backend_route", "new_abi"]:
    need(claims.get(key) == 0, f"forbidden claim drift: {key}")

need(f'latest_card = "{token}"' in state, "CURRENT_STATE latest card drift")
need(f'latest_card_path = "{card_rel_path}"' in state, "CURRENT_STATE latest path drift")
need(f'current_blocker_token = "{blocker}"' in state, "CURRENT_STATE blocker drift")

for needle in [token, output_contract, candidate, "compare_rhs_literal_i64_const_emission_bridge_owner = 1", "actual_rhs_valueid_resolution_literal_i64 = 1", "mutation_performed_const_only = 1", "source_selfhost_claim = 0", blocker]:
    need(needle in task_order, f"task-order missing {needle}")

need("tools/checks/rust_lifecycle_mirbuilder_hard_authority_pilot_compare_rhs_literal_i64_constant_emission_bridge_guard.sh" in index, "check index missing guard")

print(f"output_contract={output_contract}")
print(f"token={token}")
print(f"candidate={candidate}")
print("owner=CompareRhsConstantEmissionBridge")
print("hard_authority_pilot_implemented=1")
print("compare_rhs_literal_i64_const_emission_bridge_owner=1")
print("actual_rhs_valueid_resolution_literal_i64=1")
print("literal_constant_valueid_allocation=1")
print("constant_mir_emission=1")
print("integer_type_publication=1")
print("mutation_performed_const_only=1")
print("actual_rhs_valueid_resolution_general=0")
print("symbol_ref_valueid_resolution=0")
print("symbol_lookup_execution=0")
print("local_ssa_finalize_compare_execution=0")
print("mir_cmp_emission=0")
print("branch_emission=0")
print("basic_block_control_flow_mutation=0")
print("route_selection=0")
print("runtime_route_switch=0")
print("programjson_runtime_route_authority=0")
print("runtime_fallback=0")
print("source_selfhost_claim=0")
print("new_backend_route=0")
print("new_abi=0")
print("selected_next_card=MIRBUILDER-COMPARE-RHS-SYMBOLREF-LOOKUP-CONTRACT-CONSULTATION-001")
print("summary=ok")
PY

