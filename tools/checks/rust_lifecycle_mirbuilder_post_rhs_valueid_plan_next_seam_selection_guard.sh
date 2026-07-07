#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="rust-lifecycle-mirbuilder-post-rhs-valueid-plan-next-seam-selection-guard"

source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

CARD="$ROOT_DIR/docs/development/current/main/phases/phase-296x/3332-MIRBUILDER-POST-RHS-VALUEID-PLAN-NEXT-SEAM-SELECTION-001.md"
FIXTURE="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-post-rhs-valueid-plan-next-seam-selection-v0.json"
PREV_FIXTURE="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-hard-authority-pilot-compare-rhs-valueid-resolution-plan-v0.json"
CANDIDATE_FIXTURE="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-compare-rhs-valueid-resolution-request-response-abi-v0.json"
PREV_GUARD="$ROOT_DIR/tools/checks/rust_lifecycle_mirbuilder_hard_authority_pilot_compare_rhs_valueid_resolution_plan_guard.sh"
CANDIDATE_GUARD="$ROOT_DIR/tools/checks/rust_lifecycle_mirbuilder_compare_rhs_valueid_resolution_request_response_abi_gate.sh"
STATE="$ROOT_DIR/docs/development/current/main/CURRENT_STATE.toml"
TASK_ORDER="$ROOT_DIR/docs/development/current/main/design/mirbuilder-rust-to-hako-converter-task-order-ssot.md"
INDEX="$ROOT_DIR/docs/tools/check-scripts-index.md"

guard_require_command "$TAG" python3
guard_require_files "$TAG" "$CARD" "$FIXTURE" "$PREV_FIXTURE" "$CANDIDATE_FIXTURE" "$PREV_GUARD" "$CANDIDATE_GUARD" "$STATE" "$TASK_ORDER" "$INDEX"

PREV_OUT="$(guard_cached_run "$TAG" bash "$PREV_GUARD")"
if ! grep -q '^compare_rhs_valueid_resolution_plan_owner=1$' <<<"$PREV_OUT"; then
  printf '%s\n' "$PREV_OUT" >&2
  guard_fail "$TAG" "RHS ValueId resolution plan pilot prerequisite is not green"
fi

CANDIDATE_OUT="$(guard_cached_run "$TAG" bash "$CANDIDATE_GUARD")"
if ! grep -q '^compare_rhs_valueid_resolution_request_response_abi=1$' <<<"$CANDIDATE_OUT"; then
  printf '%s\n' "$CANDIDATE_OUT" >&2
  guard_fail "$TAG" "RHS ValueId request ABI evidence is not green"
fi

python3 - "$CARD" "$FIXTURE" "$PREV_FIXTURE" "$CANDIDATE_FIXTURE" "$STATE" "$TASK_ORDER" "$INDEX" <<'PY'
import json
import sys
from pathlib import Path

card_path = Path(sys.argv[1])
fixture_path = Path(sys.argv[2])
prev_path = Path(sys.argv[3])
candidate_path = Path(sys.argv[4])
state_path = Path(sys.argv[5])
task_order_path = Path(sys.argv[6])
index_path = Path(sys.argv[7])

card = card_path.read_text(encoding="utf-8")
fixture = json.loads(fixture_path.read_text(encoding="utf-8"))
prev = json.loads(prev_path.read_text(encoding="utf-8"))
candidate_fixture = json.loads(candidate_path.read_text(encoding="utf-8"))
state = state_path.read_text(encoding="utf-8")
task_order = task_order_path.read_text(encoding="utf-8")
index = index_path.read_text(encoding="utf-8")

def need(condition: bool, message: str) -> None:
    if not condition:
        raise SystemExit(message)

token = "MIRBUILDER-POST-RHS-VALUEID-PLAN-NEXT-SEAM-SELECTION-001"
output_contract = "rust-lifecycle-mirbuilder-post-rhs-valueid-plan-next-seam-selection-v0"
blocker = "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001"
selected_card = "MIRBUILDER-HARD-AUTHORITY-PILOT-COMPARE-RHS-VALUEID-REQUEST-ABI-001"
selected_candidate = "CompareRhsValueIdResolutionRequestAbiBoundary"
follow_on_card = selected_card
follow_on_path = "docs/development/current/main/phases/phase-296x/3333-MIRBUILDER-HARD-AUTHORITY-PILOT-COMPARE-RHS-VALUEID-REQUEST-ABI-001.md"
second_follow_on_card = "MIRBUILDER-POST-RHS-VALUEID-REQUEST-ABI-NEXT-SEAM-SELECTION-001"
second_follow_on_path = "docs/development/current/main/phases/phase-296x/3334-MIRBUILDER-POST-RHS-VALUEID-REQUEST-ABI-NEXT-SEAM-SELECTION-001.md"
third_follow_on_card = "MIRBUILDER-HARD-AUTHORITY-PILOT-COMPARE-RHS-LITERAL-I64-CONSTANT-EMISSION-BRIDGE-001"
third_follow_on_path = "docs/development/current/main/phases/phase-296x/3335-MIRBUILDER-HARD-AUTHORITY-PILOT-COMPARE-RHS-LITERAL-I64-CONSTANT-EMISSION-BRIDGE-001.md"
fourth_follow_on_card = "MIRBUILDER-HARD-AUTHORITY-PILOT-COMPARE-RHS-SYMBOLREF-LOOKUP-CONTRACT-PARITY-001"
fourth_follow_on_path = "docs/development/current/main/phases/phase-296x/3336-MIRBUILDER-HARD-AUTHORITY-PILOT-COMPARE-RHS-SYMBOLREF-LOOKUP-CONTRACT-PARITY-001.md"

need(f"# 3332 - {token}" in card, "card token drift")
need(output_contract in card, "card output contract drift")
need(selected_candidate in card, "card selected candidate drift")
need(selected_card in card, "card selected next drift")

need(fixture.get("kind") == "MirBuilderPostRhsValueIdPlanNextSeamSelectionV1", "fixture kind drift")
need(fixture.get("token") == token, "fixture token drift")
need(fixture.get("output_contract") == output_contract, "fixture output contract drift")
need((fixture.get("current_state") or {}).get("current_blocker_token") == blocker, "fixture blocker drift")

need((prev.get("claims") or {}).get("compare_rhs_valueid_resolution_plan_owner") == 1, "previous pilot evidence missing")
need((prev.get("claims") or {}).get("source_selfhost_claim") == 0, "previous Source Selfhost claim drift")

need(candidate_fixture.get("owner") == "CompareRhsValueIdResolutionRequestSnapshotBox", "candidate owner drift")
need(candidate_fixture.get("request_contract") == "CompareRhsValueIdResolutionRequestSnapshotV1", "candidate request output drift")
need((candidate_fixture.get("claims") or {}).get("compare_rhs_valueid_resolution_request_response_abi") == 1, "candidate ABI not green")
need((candidate_fixture.get("claims") or {}).get("actual_rhs_valueid_resolution") == 0, "candidate actual resolution must remain 0")
need((candidate_fixture.get("claims") or {}).get("source_selfhost_claim") == 0, "candidate Source Selfhost claim drift")

selected = fixture.get("selected_next_seam") or {}
need(selected.get("candidate_id") == selected_candidate, "selected candidate drift")
need(selected.get("owner_id") == "CompareRhsValueIdResolutionRequestSnapshotBox", "selected owner drift")
need(selected.get("input_surface") == "CompareRhsValueIdResolutionPlanSnapshotV1", "input surface drift")
need(selected.get("output_surface") == "CompareRhsValueIdResolutionRequestSnapshotV1", "output surface drift")
need(selected.get("downstream_consumer") == "CompareRhsValueIdResolutionResponseV1", "consumer drift")
for key in ["rust_oracle_available", "hako_impl_available", "aot_guard_available", "downstream_consumer_available", "eligible_as_next_hard_authority_seam"]:
    need(selected.get(key) == 1, f"selected positive drift: {key}")
for key in ["mutation_required", "route_selection_required", "runtime_switch_required", "source_selfhost_claim_required", "support_lane_projector", "string_only_facade"]:
    need(selected.get(key) == 0, f"selected forbidden drift: {key}")

decision = fixture.get("decision") or {}
need(decision.get("kind") == "SelectNextHardAuthoritySeam", "decision kind drift")
need(decision.get("reason_token") == "CompareRhsValueIdResolutionRequestAbiIsDirectReadOnlyDownstreamSeam", "reason token drift")
need(decision.get("selected_next_card") == selected_card, "selected next drift")

claims = fixture.get("claims") or {}
for key in ["post_rhs_valueid_plan_next_seam_selected", "rhs_valueid_plan_pilot_evidence_consumed", "compare_rhs_valueid_resolution_request_abi_selected"]:
    need(claims.get(key) == 1, f"positive claim drift: {key}")
for key in ["next_seam_implemented", "hako_adopted_decision", "source_selfhost_claim", "native_seed_materialization", "actual_rhs_valueid_resolution", "literal_constant_valueid_allocation", "constant_mir_emission", "symbol_lookup_execution", "local_ssa_finalize_compare_execution", "mir_cmp_emission", "branch_emission", "basic_block_mutation", "value_id_allocation", "route_selection", "runtime_route_switch", "programjson_runtime_route_authority", "runtime_fallback", "new_backend_route", "new_abi"]:
    need(claims.get(key) == 0, f"forbidden claim drift: {key}")

need(
    f'latest_card = "{token}"' in state or f'latest_card = "{follow_on_card}"' in state or f'latest_card = "{second_follow_on_card}"' in state or f'latest_card = "{third_follow_on_card}"' in state or f'latest_card = "{fourth_follow_on_card}"' in state,
    "CURRENT_STATE latest card drift",
)
need(
    f'latest_card_path = "{card_path.as_posix()}"' in state or f'latest_card_path = "{follow_on_path}"' in state or f'latest_card_path = "{second_follow_on_path}"' in state or f'latest_card_path = "{third_follow_on_path}"' in state or f'latest_card_path = "{fourth_follow_on_path}"' in state,
    "CURRENT_STATE latest path drift",
)
need(f'current_blocker_token = "{blocker}"' in state, "CURRENT_STATE blocker drift")

for needle in [token, output_contract, selected_candidate, "post_rhs_valueid_plan_next_seam_selected = 1", "next_seam_implemented = 0", "source_selfhost_claim = 0", selected_card]:
    need(needle in task_order, f"task-order missing {needle}")

need("tools/checks/rust_lifecycle_mirbuilder_post_rhs_valueid_plan_next_seam_selection_guard.sh" in index, "check index missing guard")

print(f"output_contract={output_contract}")
print("decision=SelectNextHardAuthoritySeam")
print("reason_token=CompareRhsValueIdResolutionRequestAbiIsDirectReadOnlyDownstreamSeam")
print(f"selected_candidate={selected_candidate}")
print(f"selected_next_card={selected_card}")
print("post_rhs_valueid_plan_next_seam_selected=1")
print("next_seam_implemented=0")
print("source_selfhost_claim=0")
print("runtime_route_switch=0")
print("summary=ok")
PY
