#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
cd "$ROOT_DIR"

TAG="rust-lifecycle-variable-context-explicit-mutation-api-hako-adoption-decision"

source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

guard_require_command "$TAG" python3

CARD="docs/development/current/main/phases/phase-296x/1793-VARIABLE-CONTEXT-EXPLICIT-MUTATION-API-HAKO-ADOPTION-DECISION-001.md"
FIXTURE="docs/development/current/main/design/fixtures/rust-lifecycle/variable-context-explicit-mutation-api-hako-adoption-decision-v0.json"
RERUN="docs/development/current/main/design/fixtures/rust-lifecycle/variable-context-route-matrix-rerun-002-v0.json"
PROJECTION="docs/development/current/main/design/fixtures/rust-lifecycle/variable-context-explicit-mutation-api-projection-v0.json"
NATIVE_SOURCE="$ROOT_DIR/apps/lib/hakorune_mir_builder/variable_context.hako"
NATIVE_CARRIER="$ROOT_DIR/apps/lib/hakorune_mir_builder/carrier_info.hako"
STATE="docs/development/current/main/CURRENT_STATE.toml"
TASK_ORDER="docs/development/current/main/design/mirbuilder-rust-to-hako-converter-task-order-ssot.md"
INDEX="docs/tools/check-scripts-index.md"

guard_require_files "$TAG" \
  "$CARD" \
  "$FIXTURE" \
  "$RERUN" \
  "$PROJECTION" \
  "$NATIVE_SOURCE" \
  "$NATIVE_CARRIER" \
  "$STATE" \
  "$TASK_ORDER" \
  "$INDEX"

bash tools/checks/rust_lifecycle_variable_context_route_matrix_rerun_002_guard.sh >/tmp/hako_variable_context_route_matrix_rerun_002_guard.out
bash tools/checks/rust_lifecycle_variable_context_explicit_mutation_api_projection_guard.sh >/tmp/hako_variable_context_explicit_mutation_api_projection_guard.out

python3 - "$CARD" "$FIXTURE" "$RERUN" "$PROJECTION" "$NATIVE_SOURCE" "$STATE" "$TASK_ORDER" "$INDEX" <<'PY'
from __future__ import annotations

import json
import sys
import tomllib
from pathlib import Path

card_path = Path(sys.argv[1])
fixture_path = Path(sys.argv[2])
rerun_path = Path(sys.argv[3])
projection_path = Path(sys.argv[4])
native_source_path = Path(sys.argv[5])
state_path = Path(sys.argv[6])
task_order_path = Path(sys.argv[7])
index_path = Path(sys.argv[8])

card = card_path.read_text(encoding="utf-8")
fixture = json.loads(fixture_path.read_text(encoding="utf-8"))
rerun = json.loads(rerun_path.read_text(encoding="utf-8"))
projection = json.loads(projection_path.read_text(encoding="utf-8"))
native_source = native_source_path.read_text(encoding="utf-8")
state = tomllib.loads(state_path.read_text(encoding="utf-8"))
task_order = task_order_path.read_text(encoding="utf-8")
index = index_path.read_text(encoding="utf-8")

def require(condition: bool, message: str) -> None:
    if not condition:
        raise SystemExit(message)

token = "VARIABLE-CONTEXT-EXPLICIT-MUTATION-API-HAKO-ADOPTION-DECISION-001"
output_contract = "rust-lifecycle-variable-context-explicit-mutation-api-hako-adoption-decision-v0"
surface_id = "VariableContextNativeSurfaceExplicitMutationApiOnlyV1"
included_scopes = [
    "VariableContext_simple_map_only",
    "VariableContext_snapshot_restore_only",
    "VariableContext_carrier_snapshot_only",
    "VariableContext_explicit_carrier_snapshot_only",
    "VariableContext_immutable_borrow_repaired_as_owned_snapshot",
    "VariableContext_mutable_returned_borrow_repaired_as_explicit_mutation",
]
native_guards = [
    "tools/checks/rust_mirbuilder_variable_context_native_simple_map_guard.sh",
    "tools/checks/rust_mirbuilder_variable_context_native_snapshot_restore_guard.sh",
    "tools/checks/rust_mirbuilder_carrier_info_native_snapshot_guard.sh",
    "tools/checks/rust_lifecycle_variable_context_explicit_mutation_api_projection_guard.sh",
]

require(f"# {token}" in card, "card token drift")
require(f"output_contract={output_contract}" in card, "card output contract drift")
require("decision = Adopt" in card, "card decision drift")
require(f"surface_id = {surface_id}" in card, "card surface drift")
require("native_behavior_guard_green = 1" in card, "card native guard drift")
require("explicit_mutation_api_projection_green = 1" in card, "card projection drift")

require(fixture.get("kind") == "VariableContextExplicitMutationApiHakoAdoptionDecisionV1", "fixture kind drift")
require(fixture.get("output_contract") == output_contract, "fixture output contract drift")

fixture_state = fixture.get("current_state") or {}
require(fixture_state.get("latest_card") == token, "fixture latest card drift")
require(fixture_state.get("current_blocker_token") == token, "fixture blocker drift")

selection = fixture.get("selection_evidence") or {}
require(selection.get("route_matrix_rerun") == "docs/development/current/main/design/fixtures/rust-lifecycle/variable-context-route-matrix-rerun-002-v0.json", "fixture rerun evidence drift")
require(selection.get("explicit_mutation_api_projection") == "docs/development/current/main/design/fixtures/rust-lifecycle/variable-context-explicit-mutation-api-projection-v0.json", "fixture projection evidence drift")
require(selection.get("surface_id") == surface_id, "fixture selected surface drift")
require(selection.get("candidate_pool_state") == "CandidateEligible", "fixture candidate state drift")

target = fixture.get("target") or {}
require(target.get("family_id") == "hakorune_mir_builder::variable_context", "fixture target family drift")
require(target.get("surface_id") == surface_id, "fixture target surface drift")
require(target.get("full_family_state") == "Parked", "fixture family state drift")
require(target.get("full_variable_context_claim") == 0, "fixture full claim drift")

require(fixture.get("included_scopes") == included_scopes, "fixture included scopes drift")
excluded = {item.get("scope"): item.get("reason") for item in fixture.get("excluded_scopes", [])}
require(excluded.get("VariableContext_mutable_returned_borrow") == "ReturnedMutableBorrow", "fixture mutable exclusion drift")

for path in [
    "apps/lib/hakorune_mir_builder/variable_context.hako",
    "apps/lib/hakorune_mir_builder/carrier_info.hako",
]:
    require(path in fixture.get("native_source_owners", []), f"fixture missing native source: {path}")
    require(Path(path).exists(), f"native source missing: {path}")

for guard in native_guards:
    require(guard in fixture.get("native_behavior_guards", []), f"fixture missing native guard: {guard}")
    require(Path(guard).exists(), f"native guard missing: {guard}")

decision = fixture.get("decision") or {}
require(decision.get("value") == "Adopt", "fixture decision drift")
require(decision.get("reason_token") == "NativeSurfaceOwnerPresentAndExplicitMutationGreen", "fixture reason drift")
require(decision.get("selected_next_route") == "native_hako_source_owner", "fixture next route drift")

claims = fixture.get("claims") or {}
for key in [
    "native_hako_source_owner_present",
    "native_behavior_guard_green",
    "explicit_mutation_api_projection_green",
    "replace_owned_map_native_api",
    "generator_overwrite_guard",
    "rust_bootstrap_retained",
    "rust_oracle_retained",
    "hako_adopted",
]:
    require(claims.get(key) == 1, f"fixture positive claim drift: {key}")
for key in [
    "generated_artifact_manual_edit",
    "full_variable_context_claim",
    "returned_mutable_borrow_selected",
    "raw_variable_map_alias_selected",
    "source_selfhost_claim",
    "runtime_fallback",
    "new_backend_route",
    "new_abi",
    "new_python_semantic_projector",
    "manual_family_selection",
]:
    require(claims.get(key) == 0, f"fixture non-claim drift: {key}")

rerun_result = rerun.get("rerun_result") or {}
require(rerun_result.get("candidate_pool_state") == "CandidateEligible", "rerun candidate drift")
require(rerun_result.get("selected_surface_id") == surface_id, "rerun surface drift")
require(rerun_result.get("next_action") == token, "rerun next action drift")

require(projection.get("kind") == "VariableContextExplicitMutationApiProjectionV1", "projection kind drift")
projection_state = projection.get("current_state") or {}
require(projection_state.get("latest_card") == "MIRBUILDER-VARIABLE-CONTEXT-EXPLICIT-MUTATION-API-PROJECTION-001", "projection latest card drift")
require((projection.get("projection") or {}).get("selected_policy") == "ExplicitMutationApiOnly", "projection policy drift")

require("replace_owned_map(ctx: VariableContextNative, owned_map: OrderedMapBox)" in native_source, "native source missing replace_owned_map")
require("variable_map_mut" not in native_source, "native source must not expose variable_map_mut")

allowed_current_tokens = {
    token,
    "SOURCE-SELFHOST-POST-VARIABLE-CONTEXT-EXPLICIT-MUTATION-RESOLUTION-001",
    "MIRBUILDER-VARIABLE-CONTEXT-REFERENCE-PROJECTION-CONTRACT-001",
}
allowed_current_paths = {
    "docs/development/current/main/phases/phase-296x/1793-VARIABLE-CONTEXT-EXPLICIT-MUTATION-API-HAKO-ADOPTION-DECISION-001.md",
    "docs/development/current/main/phases/phase-296x/1794-SOURCE-SELFHOST-POST-VARIABLE-CONTEXT-EXPLICIT-MUTATION-RESOLUTION-001.md",
    "docs/development/current/main/phases/phase-296x/1795-MIRBUILDER-VARIABLE-CONTEXT-REFERENCE-PROJECTION-CONTRACT-001.md",
}
require(state.get("latest_card") in allowed_current_tokens, "current-state latest card drift")
require(state.get("current_blocker_token") in allowed_current_tokens, "current-state blocker drift")
require(state.get("latest_card_path") in allowed_current_paths, "current-state latest card path drift")

for needle in [
    token,
    surface_id,
    "decision = Adopt",
]:
    require(needle in task_order, f"task-order missing {needle}")

require("tools/checks/rust_lifecycle_variable_context_explicit_mutation_api_hako_adoption_decision_guard.sh" in index, "check index missing guard")

print(f"output_contract={output_contract}")
print(f"surface_id={surface_id}")
print("decision=Adopt")
print("native_hako_source_owner_present=1")
print("native_behavior_guard_green=1")
print("explicit_mutation_api_projection_green=1")
print("replace_owned_map_native_api=1")
print("generator_overwrite_guard=1")
print("rust_bootstrap_retained=1")
print("rust_oracle_retained=1")
print("full_variable_context_claim=0")
print("returned_mutable_borrow_selected=0")
print("raw_variable_map_alias_selected=0")
print("source_selfhost_claim=0")
print("runtime_fallback=0")
print("new_backend_route=0")
print("new_abi=0")
print("summary=ok")
PY
