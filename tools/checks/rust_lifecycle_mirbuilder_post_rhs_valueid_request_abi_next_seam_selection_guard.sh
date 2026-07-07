#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="rust-lifecycle-mirbuilder-post-rhs-valueid-request-abi-next-seam-selection-guard"

source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

CARD="$ROOT_DIR/docs/development/current/main/phases/phase-296x/3334-MIRBUILDER-POST-RHS-VALUEID-REQUEST-ABI-NEXT-SEAM-SELECTION-001.md"
FIXTURE="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-post-rhs-valueid-request-abi-next-seam-selection-v0.json"
REQUEST_ABI_FIXTURE="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-hard-authority-pilot-compare-rhs-valueid-request-abi-v0.json"
LITERAL_FIXTURE="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-compare-rhs-literal-i64-constant-emission-bridge-v0.json"
REQUEST_ABI_GUARD="$ROOT_DIR/tools/checks/rust_lifecycle_mirbuilder_hard_authority_pilot_compare_rhs_valueid_request_abi_guard.sh"
LITERAL_GUARD="$ROOT_DIR/tools/checks/rust_lifecycle_mirbuilder_compare_rhs_literal_i64_constant_emission_bridge_gate.sh"
STATE="$ROOT_DIR/docs/development/current/main/CURRENT_STATE.toml"
TASK_ORDER="$ROOT_DIR/docs/development/current/main/design/mirbuilder-rust-to-hako-converter-task-order-ssot.md"
INDEX="$ROOT_DIR/docs/tools/check-scripts-index.md"

guard_require_command "$TAG" python3
guard_require_files "$TAG" "$CARD" "$FIXTURE" "$REQUEST_ABI_FIXTURE" "$LITERAL_FIXTURE" "$REQUEST_ABI_GUARD" "$LITERAL_GUARD" "$STATE" "$TASK_ORDER" "$INDEX"

REQUEST_ABI_OUT="$(guard_cached_run "$TAG" bash "$REQUEST_ABI_GUARD")"
if ! grep -q '^compare_rhs_valueid_resolution_request_abi_owner=1$' <<<"$REQUEST_ABI_OUT"; then
  printf '%s\n' "$REQUEST_ABI_OUT" >&2
  guard_fail "$TAG" "request ABI hard-authority prerequisite is not green"
fi

LITERAL_OUT="$(guard_cached_run "$TAG" bash "$LITERAL_GUARD")"
if ! grep -q '^literal_i64_constant_emission_bridge=1$' <<<"$LITERAL_OUT"; then
  printf '%s\n' "$LITERAL_OUT" >&2
  guard_fail "$TAG" "LiteralI64 bridge evidence is not green"
fi

python3 - "$CARD" "$FIXTURE" "$REQUEST_ABI_FIXTURE" "$LITERAL_FIXTURE" "$STATE" "$TASK_ORDER" "$INDEX" <<'PY'
import json
import sys
from pathlib import Path

card_path = Path(sys.argv[1])
fixture = json.loads(Path(sys.argv[2]).read_text(encoding="utf-8"))
request_abi = json.loads(Path(sys.argv[3]).read_text(encoding="utf-8"))
literal = json.loads(Path(sys.argv[4]).read_text(encoding="utf-8"))
state = Path(sys.argv[5]).read_text(encoding="utf-8")
task_order = Path(sys.argv[6]).read_text(encoding="utf-8")
index = Path(sys.argv[7]).read_text(encoding="utf-8")
card = card_path.read_text(encoding="utf-8")

def need(condition: bool, message: str) -> None:
    if not condition:
        raise SystemExit(message)

token = "MIRBUILDER-POST-RHS-VALUEID-REQUEST-ABI-NEXT-SEAM-SELECTION-001"
output_contract = "rust-lifecycle-mirbuilder-post-rhs-valueid-request-abi-next-seam-selection-v0"
selected_card = "MIRBUILDER-HARD-AUTHORITY-PILOT-COMPARE-RHS-LITERAL-I64-CONSTANT-EMISSION-BRIDGE-001"
selected_candidate = "CompareRhsLiteralI64ConstantEmissionBridgeBoundary"
blocker = "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001"
follow_on_path = "docs/development/current/main/phases/phase-296x/3335-MIRBUILDER-HARD-AUTHORITY-PILOT-COMPARE-RHS-LITERAL-I64-CONSTANT-EMISSION-BRIDGE-001.md"
second_follow_on_card = "MIRBUILDER-HARD-AUTHORITY-PILOT-COMPARE-RHS-SYMBOLREF-LOOKUP-CONTRACT-PARITY-001"
second_follow_on_path = "docs/development/current/main/phases/phase-296x/3336-MIRBUILDER-HARD-AUTHORITY-PILOT-COMPARE-RHS-SYMBOLREF-LOOKUP-CONTRACT-PARITY-001.md"
third_follow_on_card = "MIRBUILDER-HARD-AUTHORITY-PILOT-COMPARE-RHS-SYMBOLREF-LOOKUP-BRIDGE-001"
third_follow_on_path = "docs/development/current/main/phases/phase-296x/3337-MIRBUILDER-HARD-AUTHORITY-PILOT-COMPARE-RHS-SYMBOLREF-LOOKUP-BRIDGE-001.md"

need(f"# 3334 - {token}" in card, "card token drift")
need(output_contract in card, "card output contract drift")
need(selected_candidate in card, "card selected candidate drift")
need(selected_card in card, "card selected next drift")

need(fixture.get("kind") == "MirBuilderPostRhsValueIdRequestAbiNextSeamSelectionV1", "fixture kind drift")
need(fixture.get("token") == token, "fixture token drift")
need(fixture.get("output_contract") == output_contract, "fixture output contract drift")
need((fixture.get("current_state") or {}).get("current_blocker_token") == blocker, "fixture blocker drift")

need((request_abi.get("claims") or {}).get("compare_rhs_valueid_resolution_request_abi_owner") == 1, "request ABI evidence missing")
need((request_abi.get("claims") or {}).get("actual_rhs_valueid_resolution") == 0, "request ABI must be read-only")
need((literal.get("claims") or {}).get("literal_i64_constant_emission_bridge") == 1, "literal bridge evidence missing")
need((literal.get("claims") or {}).get("mutation_performed_const_only") == 1, "literal bridge mutation scope drift")
need((literal.get("claims") or {}).get("symbol_lookup_execution") == 0, "literal bridge symbol lookup drift")

selected = fixture.get("selected_next_seam") or {}
need(selected.get("candidate_id") == selected_candidate, "selected candidate drift")
need(selected.get("owner_id") == "CompareRhsConstantEmissionBridge", "selected owner drift")
need(selected.get("input_surface") == "CompareRhsValueIdResolutionRequestSnapshotV1", "input surface drift")
need(selected.get("output_surface") == "CompareRhsValueIdResolutionResponseV1", "output surface drift")
need(selected.get("mutation_scope") == "ConstInstructionOnly", "mutation scope drift")
for key in ["rust_oracle_available", "hako_request_abi_available", "rust_bridge_impl_available", "guard_available", "eligible_as_next_hard_authority_seam", "actual_mutation_required"]:
    need(selected.get(key) == 1, f"selected positive drift: {key}")
for key in ["route_selection_required", "runtime_switch_required", "source_selfhost_claim_required", "support_lane_projector", "string_only_facade"]:
    need(selected.get(key) == 0, f"selected forbidden drift: {key}")

decision = fixture.get("decision") or {}
need(decision.get("kind") == "SelectNextScopedMutationSeam", "decision kind drift")
need(decision.get("reason_token") == "LiteralI64ConstEmissionIsFirstScopedRequestAbiConsumer", "reason drift")
need(decision.get("selected_next_card") == selected_card, "selected next drift")

claims = fixture.get("claims") or {}
for key in ["post_rhs_valueid_request_abi_next_seam_selected", "request_abi_pilot_evidence_consumed", "literal_i64_const_emission_bridge_selected", "existing_literal_i64_bridge_guard_green"]:
    need(claims.get(key) == 1, f"positive claim drift: {key}")
for key in ["next_seam_implemented", "hako_adopted_decision", "source_selfhost_claim", "native_seed_materialization", "actual_rhs_valueid_resolution_literal_i64", "literal_constant_valueid_allocation", "constant_mir_emission", "symbol_lookup_execution", "local_ssa_finalize_compare_execution", "mir_cmp_emission", "branch_emission", "basic_block_control_flow_mutation", "route_selection", "runtime_route_switch", "programjson_runtime_route_authority", "runtime_fallback", "new_backend_route", "new_abi"]:
    need(claims.get(key) == 0, f"forbidden claim drift: {key}")

need(f'latest_card = "{token}"' in state or f'latest_card = "{selected_card}"' in state or f'latest_card = "{second_follow_on_card}"' in state or f'latest_card = "{third_follow_on_card}"' in state, "CURRENT_STATE latest card drift")
need(f'latest_card_path = "{card_path.as_posix()}"' in state or f'latest_card_path = "{follow_on_path}"' in state or f'latest_card_path = "{second_follow_on_path}"' in state or f'latest_card_path = "{third_follow_on_path}"' in state, "CURRENT_STATE latest path drift")
need(f'current_blocker_token = "{blocker}"' in state, "CURRENT_STATE blocker drift")

for needle in [token, output_contract, selected_candidate, "post_rhs_valueid_request_abi_next_seam_selected = 1", "next_seam_implemented = 0", "source_selfhost_claim = 0", selected_card]:
    need(needle in task_order, f"task-order missing {needle}")

need("tools/checks/rust_lifecycle_mirbuilder_post_rhs_valueid_request_abi_next_seam_selection_guard.sh" in index, "check index missing guard")

print(f"output_contract={output_contract}")
print("decision=SelectNextScopedMutationSeam")
print("reason_token=LiteralI64ConstEmissionIsFirstScopedRequestAbiConsumer")
print(f"selected_candidate={selected_candidate}")
print(f"selected_next_card={selected_card}")
print("post_rhs_valueid_request_abi_next_seam_selected=1")
print("next_seam_implemented=0")
print("source_selfhost_claim=0")
print("runtime_route_switch=0")
print("summary=ok")
PY
