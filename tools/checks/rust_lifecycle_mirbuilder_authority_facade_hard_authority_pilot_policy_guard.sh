#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
cd "$ROOT_DIR"

TAG="rust-lifecycle-mirbuilder-authority-facade-hard-authority-pilot-policy"

source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

CARD="docs/development/current/main/phases/phase-296x/3326-MIRBUILDER-AUTHORITY-FACADE-HARD-AUTHORITY-PILOT-POLICY-001.md"
FIXTURE="docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-authority-facade-hard-authority-pilot-policy-v0.json"
REGISTRY="docs/development/current/main/design/fixtures/rust-lifecycle/source-selfhost-explicit-authority-registry-basis-v0.json"
DECOMPOSITION="docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-minimal-path-composed-closure-native-slice-decomposition-v2.json"
STATE="docs/development/current/main/CURRENT_STATE.toml"
TASK_ORDER="docs/development/current/main/design/mirbuilder-rust-to-hako-converter-task-order-ssot.md"
INDEX="docs/tools/check-scripts-index.md"

guard_require_files "$TAG" \
  "$CARD" \
  "$FIXTURE" \
  "$REGISTRY" \
  "$DECOMPOSITION" \
  "$STATE" \
  "$TASK_ORDER" \
  "$INDEX"

python3 - "$CARD" "$FIXTURE" "$REGISTRY" "$DECOMPOSITION" "$STATE" "$TASK_ORDER" "$INDEX" <<'PY'
from __future__ import annotations

import json
import sys
import tomllib
from pathlib import Path

card_path = Path(sys.argv[1])
fixture_path = Path(sys.argv[2])
registry_path = Path(sys.argv[3])
decomposition_path = Path(sys.argv[4])
state_path = Path(sys.argv[5])
task_order_path = Path(sys.argv[6])
index_path = Path(sys.argv[7])

card = card_path.read_text(encoding="utf-8")
fixture = json.loads(fixture_path.read_text(encoding="utf-8"))
registry = json.loads(registry_path.read_text(encoding="utf-8"))
decomposition = json.loads(decomposition_path.read_text(encoding="utf-8"))
state = tomllib.loads(state_path.read_text(encoding="utf-8"))
task_order = task_order_path.read_text(encoding="utf-8")
index = index_path.read_text(encoding="utf-8")


def need(condition: bool, message: str) -> None:
    if not condition:
        raise SystemExit(message)


token = "MIRBUILDER-AUTHORITY-FACADE-HARD-AUTHORITY-PILOT-POLICY-001"
blocker = "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001"
candidate = "BoolRecipeCompareSemanticCommandBoundary"
next_card = "MIRBUILDER-HARD-AUTHORITY-PILOT-BOOLRECIPE-COMPARE-SEMANTIC-COMMAND-001"
next_card_path = "docs/development/current/main/phases/phase-296x/3327-MIRBUILDER-HARD-AUTHORITY-PILOT-BOOLRECIPE-COMPARE-SEMANTIC-COMMAND-001.md"
follow_on_card = "MIRBUILDER-POST-HARD-AUTHORITY-PILOT-NEXT-SEAM-SELECTION-001"
follow_on_card_path = "docs/development/current/main/phases/phase-296x/3328-MIRBUILDER-POST-HARD-AUTHORITY-PILOT-NEXT-SEAM-SELECTION-001.md"
second_follow_on_card = "MIRBUILDER-HARD-AUTHORITY-PILOT-COMPARE-RHS-MATERIALIZATION-INTENT-001"
second_follow_on_card_path = "docs/development/current/main/phases/phase-296x/3329-MIRBUILDER-HARD-AUTHORITY-PILOT-COMPARE-RHS-MATERIALIZATION-INTENT-001.md"
third_follow_on_card = "MIRBUILDER-HARD-AUTHORITY-PILOT-COMPARE-RHS-VALUEID-RESOLUTION-PLAN-001"
third_follow_on_card_path = "docs/development/current/main/phases/phase-296x/3331-MIRBUILDER-HARD-AUTHORITY-PILOT-COMPARE-RHS-VALUEID-RESOLUTION-PLAN-001.md"
output_contract = "rust-lifecycle-mirbuilder-authority-facade-hard-authority-pilot-policy-v0"

need(f"# 3326 - {token}" in card, "card token drift")
need(output_contract in card, "card output contract drift")
need(candidate in card, "card missing selected candidate")
need(next_card in card, "card selected next drift")

need(fixture.get("kind") == "MirBuilderAuthorityFacadeHardAuthorityPilotPolicyV1", "fixture kind drift")
need(fixture.get("token") == token, "fixture token drift")
need(fixture.get("output_contract") == output_contract, "fixture output contract drift")
need((fixture.get("current_state") or {}).get("latest_card") == token, "fixture latest card drift")
need((fixture.get("current_state") or {}).get("current_blocker_token") == blocker, "fixture blocker drift")

need((registry.get("decision") or {}).get("selected_axis") == "HardAuthoritySeamProofAxis", "registry axis drift")
need((decomposition.get("selector_summary") or {}).get("eligible_hard_authority_candidate_count") == 1, "decomposition eligible count drift")
need((decomposition.get("decision") or {}).get("selected_candidate_for_policy") == candidate, "decomposition selected candidate drift")

selected = fixture.get("selected_pilot") or {}
need(selected.get("candidate_id") == candidate, "selected candidate drift")
need(selected.get("owner_id") == "BoolRecipeCompareLoweringIntentSnapshotBox", "owner drift")
need(selected.get("seam_kind") == "LoweringFacingSemanticCommand", "seam kind drift")
need(selected.get("input_surface") == "BoolRecipeComparePublicationV1", "input surface drift")
need(selected.get("output_surface") == "BoolRecipeCompareLoweringIntentSnapshotV1", "output surface drift")
need(selected.get("downstream_consumer") == "CompareLoweringSymbolicCommandSnapshotV1", "consumer drift")
need(selected.get("claim_ceiling") == "scoped_hard_authority_pilot", "claim ceiling drift")
for key in ["rust_oracle_available", "hako_impl_available", "aot_guard_available"]:
    need(selected.get(key) == 1, f"selected positive field drift: {key}")
for key in [
    "mutation_required",
    "route_selection_required",
    "runtime_switch_required",
    "source_selfhost_claim_required",
    "support_lane_projector",
    "string_only_facade",
    "manual_family_selection",
]:
    need(selected.get(key) == 0, f"selected forbidden field drift: {key}")

basis = fixture.get("selection_basis") or {}
need(basis.get("registry_axis") == "HardAuthoritySeamProofAxis", "basis axis drift")
need(basis.get("eligible_hard_authority_candidate_count") == 1, "basis eligible count drift")
for key in [
    "manual_family_selection",
    "route_membership_alone_as_proof",
    "coverage_percentage_as_proof",
    "support_lane_projector_as_adoption_candidate",
]:
    need(basis.get(key) == 0, f"basis forbidden axis drift: {key}")

policy = fixture.get("pilot_policy") or {}
for key in [
    "authority_facade_required",
    "rust_oracle_fixture_required",
    "hako_owner_required",
    "aot_exe_parity_gate_required",
    "downstream_consumer_required",
    "implementation_card_required",
    "hako_adopted_decision_forbidden_in_policy_card",
    "source_selfhost_claim_forbidden",
]:
    need(policy.get(key) is True, f"policy requirement drift: {key}")

decision = fixture.get("decision") or {}
need(decision.get("kind") == "SelectHardAuthorityPilot", "decision kind drift")
need(decision.get("reason_token") == "BoolRecipeCompareSemanticCommandIsExactlyOneEligibleHardAuthorityCandidate", "reason token drift")
need(decision.get("selected_candidate") == candidate, "decision candidate drift")
need(decision.get("selected_next_card") == next_card, "decision next drift")

claims = fixture.get("claims") or {}
for key in [
    "hard_authority_pilot_policy_selected",
    "hard_authority_pilot_selected",
    "boolrecipe_compare_semantic_command_selected",
]:
    need(claims.get(key) == 1, f"positive claim drift: {key}")
for key in [
    "hard_authority_pilot_implemented",
    "source_selfhost_claim",
    "hako_adopted_decision",
    "native_seed_materialization",
    "manual_family_selection",
    "route_membership_alone_as_proof",
    "coverage_percentage_as_proof",
    "support_lane_projector_as_adoption_candidate",
    "route_selection",
    "runtime_route_switch",
    "programjson_runtime_route_authority",
    "runtime_fallback",
    "mir_mutation",
    "id_allocation",
    "new_backend_route",
    "new_abi",
]:
    need(claims.get(key) == 0, f"forbidden claim drift: {key}")

need(state.get("latest_card") in [token, next_card, follow_on_card, second_follow_on_card, third_follow_on_card], "CURRENT_STATE latest card drift")
need(state.get("latest_card_path") in [str(card_path), next_card_path, follow_on_card_path, second_follow_on_card_path, third_follow_on_card_path], "CURRENT_STATE latest path drift")
need(state.get("current_blocker_token") == blocker, "CURRENT_STATE blocker drift")

for needle in [
    token,
    output_contract,
    candidate,
    "hard_authority_pilot_selected = 1",
    "hard_authority_pilot_implemented = 0",
    "source_selfhost_claim = 0",
    next_card,
]:
    need(needle in task_order, f"task-order missing {needle}")

need("tools/checks/rust_lifecycle_mirbuilder_authority_facade_hard_authority_pilot_policy_guard.sh" in index, "check index missing guard")

print(f"output_contract={output_contract}")
print("decision=SelectHardAuthorityPilot")
print("reason_token=BoolRecipeCompareSemanticCommandIsExactlyOneEligibleHardAuthorityCandidate")
print(f"selected_candidate={candidate}")
print(f"selected_next_card={next_card}")
print("hard_authority_pilot_selected=1")
print("hard_authority_pilot_implemented=0")
print("source_selfhost_claim=0")
print("runtime_route_switch=0")
print("summary=ok")
PY
