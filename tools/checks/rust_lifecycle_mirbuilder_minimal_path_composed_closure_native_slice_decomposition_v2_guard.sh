#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
cd "$ROOT_DIR"

TAG="rust-lifecycle-mirbuilder-minimal-path-composed-closure-native-slice-decomposition-v2"

source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

CARD="docs/development/current/main/phases/phase-296x/3325-MIRBUILDER-MINIMAL-PATH-COMPOSED-CLOSURE-NATIVE-SLICE-DECOMPOSITION-001.md"
FIXTURE="docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-minimal-path-composed-closure-native-slice-decomposition-v2.json"
REGISTRY="docs/development/current/main/design/fixtures/rust-lifecycle/source-selfhost-explicit-authority-registry-basis-v0.json"
STATE="docs/development/current/main/CURRENT_STATE.toml"
TASK_ORDER="docs/development/current/main/design/mirbuilder-rust-to-hako-converter-task-order-ssot.md"
INDEX="docs/tools/check-scripts-index.md"

guard_require_files "$TAG" \
  "$CARD" \
  "$FIXTURE" \
  "$REGISTRY" \
  "$STATE" \
  "$TASK_ORDER" \
  "$INDEX"

python3 - "$CARD" "$FIXTURE" "$REGISTRY" "$STATE" "$TASK_ORDER" "$INDEX" <<'PY'
from __future__ import annotations

import json
import sys
import tomllib
from pathlib import Path

card_path = Path(sys.argv[1])
fixture_path = Path(sys.argv[2])
registry_path = Path(sys.argv[3])
state_path = Path(sys.argv[4])
task_order_path = Path(sys.argv[5])
index_path = Path(sys.argv[6])

card = card_path.read_text(encoding="utf-8")
fixture = json.loads(fixture_path.read_text(encoding="utf-8"))
registry = json.loads(registry_path.read_text(encoding="utf-8"))
state = tomllib.loads(state_path.read_text(encoding="utf-8"))
task_order = task_order_path.read_text(encoding="utf-8")
index = index_path.read_text(encoding="utf-8")


def need(condition: bool, message: str) -> None:
    if not condition:
        raise SystemExit(message)


token = "MIRBUILDER-MINIMAL-PATH-COMPOSED-CLOSURE-NATIVE-SLICE-DECOMPOSITION-001"
blocker = "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001"
next_card = "MIRBUILDER-AUTHORITY-FACADE-HARD-AUTHORITY-PILOT-POLICY-001"
output_contract = "rust-lifecycle-mirbuilder-minimal-path-composed-closure-native-slice-decomposition-v2"

need(f"# 3325 - {token}" in card, "card token drift")
need(output_contract in card, "card output contract drift")
need(next_card in card, "card selected next drift")

need(fixture.get("kind") == "MirBuilderMinimalPathComposedClosureNativeSliceDecompositionV2", "fixture kind drift")
need(fixture.get("token") == token, "fixture token drift")
need(fixture.get("output_contract") == output_contract, "fixture output contract drift")
need((fixture.get("current_state") or {}).get("latest_card") == token, "fixture latest card drift")
need((fixture.get("current_state") or {}).get("current_blocker_token") == blocker, "fixture blocker drift")

need((registry.get("decision") or {}).get("selected_axis") == "HardAuthoritySeamProofAxis", "registry axis drift")
need((registry.get("claims") or {}).get("new_proof_axis_registered") == 1, "registry proof-axis claim drift")

for section in [
    "authority_seams",
    "owner_dependency_graph",
    "minimal_path_required_owner_set",
    "first_hard_authority_candidate_selector_input",
]:
    need(fixture.get(section), f"missing decomposition section: {section}")

rows = fixture.get("first_hard_authority_candidate_selector_input") or []
summary = fixture.get("selector_summary") or {}
need(summary.get("candidate_row_count") == len(rows), "candidate row count drift")
eligible = [row for row in rows if row.get("eligible_as_hard_authority_candidate") == 1]
rejected = [row for row in rows if row.get("eligible_as_hard_authority_candidate") == 0]
need(len(eligible) == 1, "must have exactly one eligible hard authority candidate")
need(len(rejected) == summary.get("rejected_candidate_count"), "rejected candidate count drift")
need(summary.get("eligible_hard_authority_candidate_count") == 1, "eligible summary drift")
need(summary.get("selected_candidate_for_policy") == "BoolRecipeCompareSemanticCommandBoundary", "selected candidate drift")

selected = eligible[0]
need(selected.get("candidate_id") == "BoolRecipeCompareSemanticCommandBoundary", "eligible candidate id drift")
need(selected.get("seam_kind") == "LoweringFacingSemanticCommand", "eligible seam kind drift")
need(selected.get("rust_oracle_available") == 1, "rust oracle availability drift")
need(selected.get("hako_impl_available") == 1, "hako impl availability drift")
need(selected.get("aot_guard_available") == 1, "aot guard availability drift")
for key in [
    "mutation_required",
    "route_selection_required",
    "runtime_switch_required",
    "source_selfhost_claim_required",
    "support_lane_projector",
    "string_only_facade",
    "manual_family_selection",
]:
    need(selected.get(key) == 0, f"eligible candidate forbidden flag drift: {key}")

for row in rejected:
    need(row.get("rejection_reason"), f"rejected row missing reason: {row.get('candidate_id')}")

decision = fixture.get("decision") or {}
need(decision.get("kind") == "SelectHardAuthorityPilotPolicy", "decision kind drift")
need(decision.get("reason_token") == "ExactlyOneHardAuthoritySeamCandidateFromRegistryDecomposition", "reason token drift")
need(decision.get("selected_candidate_for_policy") == "BoolRecipeCompareSemanticCommandBoundary", "decision selected candidate drift")
need(decision.get("selected_next_card") == next_card, "decision next drift")

claims = fixture.get("claims") or {}
for key in [
    "minimal_path_native_slice_decomposition",
    "selector_ready_decomposition",
    "exactly_one_hard_authority_candidate_for_policy",
]:
    need(claims.get(key) == 1, f"positive claim drift: {key}")
for key in [
    "hard_authority_pilot_selected",
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

need(state.get("current_blocker_token") == blocker, "CURRENT_STATE blocker drift")
latest_card = state.get("latest_card")
latest_path = state.get("latest_card_path")
need(isinstance(latest_card, str) and latest_card, "CURRENT_STATE latest card missing")
need(isinstance(latest_path, str) and Path(latest_path).exists(), "CURRENT_STATE latest path missing")

for needle in [
    token,
    output_contract,
    "BoolRecipeCompareSemanticCommandBoundary",
    "eligible_hard_authority_candidate_count = 1",
    "hard_authority_pilot_selected = 0",
    "source_selfhost_claim = 0",
    next_card,
]:
    need(needle in task_order, f"task-order missing {needle}")

need("tools/checks/rust_lifecycle_mirbuilder_minimal_path_composed_closure_native_slice_decomposition_v2_guard.sh" in index, "check index missing guard")

print(f"output_contract={output_contract}")
print("decision=SelectHardAuthorityPilotPolicy")
print("reason_token=ExactlyOneHardAuthoritySeamCandidateFromRegistryDecomposition")
print("eligible_hard_authority_candidate_count=1")
print("selected_candidate_for_policy=BoolRecipeCompareSemanticCommandBoundary")
print(f"selected_next_card={next_card}")
print("hard_authority_pilot_selected=0")
print("source_selfhost_claim=0")
print("runtime_route_switch=0")
print("summary=ok")
PY
