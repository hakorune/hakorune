#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
cd "$ROOT_DIR"

TAG="rust-lifecycle-variable-context-bounded-native-surface-readiness"

source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

CARD="docs/development/current/main/phases/phase-296x/1796-MIRBUILDER-VARIABLE-CONTEXT-BOUNDED-NATIVE-SURFACE-READINESS-RESOLVER-001.md"
FIXTURE="docs/development/current/main/design/fixtures/rust-lifecycle/variable-context-bounded-native-surface-readiness-resolution-v0.json"
REFERENCE_CONTRACT="docs/development/current/main/design/fixtures/rust-lifecycle/variable-context-reference-projection-contract-v0.json"
ADOPTION="docs/development/current/main/design/fixtures/rust-lifecycle/variable-context-explicit-mutation-api-hako-adoption-decision-v0.json"
READ_PROJECTION="docs/development/current/main/design/fixtures/rust-lifecycle/variable-context-owned-read-snapshot-projection-v0.json"
MUT_PROJECTION="docs/development/current/main/design/fixtures/rust-lifecycle/variable-context-explicit-mutation-api-projection-v0.json"
NATIVE_SOURCE="apps/lib/hakorune_mir_builder/variable_context.hako"
STATE="docs/development/current/main/CURRENT_STATE.toml"
TASK_ORDER="docs/development/current/main/design/mirbuilder-rust-to-hako-converter-task-order-ssot.md"
INDEX="docs/tools/check-scripts-index.md"

guard_require_files "$TAG" \
  "$CARD" \
  "$FIXTURE" \
  "$REFERENCE_CONTRACT" \
  "$ADOPTION" \
  "$READ_PROJECTION" \
  "$MUT_PROJECTION" \
  "$NATIVE_SOURCE" \
  "$STATE" \
  "$TASK_ORDER" \
  "$INDEX"

bash tools/checks/rust_lifecycle_variable_context_reference_projection_contract_guard.sh >/tmp/hako_variable_context_reference_projection_contract_guard.out
bash tools/checks/rust_lifecycle_variable_context_explicit_mutation_api_hako_adoption_decision_guard.sh >/tmp/hako_variable_context_explicit_mutation_api_hako_adoption_decision_guard.out
bash tools/checks/rust_lifecycle_variable_context_owned_read_snapshot_projection_guard.sh >/tmp/hako_variable_context_owned_read_snapshot_projection_guard.out
bash tools/checks/rust_lifecycle_variable_context_explicit_mutation_api_projection_guard.sh >/tmp/hako_variable_context_explicit_mutation_api_projection_guard.out
bash tools/checks/rust_mirbuilder_variable_context_native_simple_map_guard.sh >/tmp/hako_variable_context_native_simple_map_guard.out
bash tools/checks/rust_mirbuilder_variable_context_native_snapshot_restore_guard.sh >/tmp/hako_variable_context_native_snapshot_restore_guard.out

python3 - "$CARD" "$FIXTURE" "$REFERENCE_CONTRACT" "$ADOPTION" "$READ_PROJECTION" "$MUT_PROJECTION" "$NATIVE_SOURCE" "$STATE" "$TASK_ORDER" "$INDEX" <<'PY'
from __future__ import annotations

import json
import sys
import tomllib
from pathlib import Path

card_path = Path(sys.argv[1])
fixture_path = Path(sys.argv[2])
reference_path = Path(sys.argv[3])
adoption_path = Path(sys.argv[4])
read_projection_path = Path(sys.argv[5])
mut_projection_path = Path(sys.argv[6])
native_source_path = Path(sys.argv[7])
state_path = Path(sys.argv[8])
task_order_path = Path(sys.argv[9])
index_path = Path(sys.argv[10])

card = card_path.read_text(encoding="utf-8")
fixture = json.loads(fixture_path.read_text(encoding="utf-8"))
reference = json.loads(reference_path.read_text(encoding="utf-8"))
adoption = json.loads(adoption_path.read_text(encoding="utf-8"))
read_projection = json.loads(read_projection_path.read_text(encoding="utf-8"))
mut_projection = json.loads(mut_projection_path.read_text(encoding="utf-8"))
native_source = native_source_path.read_text(encoding="utf-8")
state = tomllib.loads(state_path.read_text(encoding="utf-8"))
task_order = task_order_path.read_text(encoding="utf-8")
index = index_path.read_text(encoding="utf-8")

def require(condition: bool, message: str) -> None:
    if not condition:
        raise SystemExit(message)

token = "MIRBUILDER-VARIABLE-CONTEXT-BOUNDED-NATIVE-SURFACE-READINESS-RESOLVER-001"
output_contract = "rust-lifecycle-mirbuilder-variable-context-bounded-native-surface-readiness-resolution-v0"
surface_id = "VariableContextNativeSurfaceExplicitMutationApiOnlyV1"

require(f"# {token}" in card, "card token drift")
require(f"output_contract={output_contract}" in card, "card output contract drift")
require("ReadyForBoundedVariableContextNativeSurfaceConsumer" in card, "card decision drift")
require("full_variable_context_claim = 0" in card, "card full claim drift")

require(fixture.get("kind") == "VariableContextBoundedNativeSurfaceReadinessResolutionV1", "fixture kind drift")
require(fixture.get("output_contract") == output_contract, "fixture output contract drift")
require(fixture.get("family_id") == "hakorune_mir_builder::variable_context", "fixture family drift")

fixture_state = fixture.get("current_state") or {}
require(fixture_state.get("latest_card") == token, "fixture latest card drift")
require(fixture_state.get("current_blocker_token") == token, "fixture blocker drift")

decision = fixture.get("decision") or {}
require(decision.get("kind") == "ReadyForBoundedVariableContextNativeSurfaceConsumer", "fixture decision drift")
require(decision.get("readiness_state") == "Ready", "fixture readiness drift")
require(decision.get("reason_token") == "ExplicitMutationSurfaceAdoptedAndReferenceProjectionContractClosed", "fixture reason drift")
require(decision.get("selected_surface_id") == surface_id, "fixture surface drift")

bounded = fixture.get("bounded_surface") or {}
require(bounded.get("projection_model") == "SemanticOneToOneVerifiedProjection", "projection model drift")
require(bounded.get("variable_map_projection") == "OwnedReadSnapshotProjection", "variable_map projection drift")
require(bounded.get("variable_map_mut_projection") == "ExplicitMutationApiOnly", "variable_map_mut projection drift")
for key in [
    "native_hako_source_owner_present",
    "native_behavior_guard_green",
    "owned_read_snapshot_projection_green",
    "explicit_mutation_api_projection_green",
    "reference_projection_contract_green",
]:
    require(bounded.get(key) == 1, f"bounded evidence drift: {key}")

current_api = set(bounded.get("current_native_api", []))
for name in ["lookup", "contains", "len", "is_empty", "snapshot", "restore", "replace_owned_map", "insert", "remove"]:
    require(name in current_api, f"bounded API missing {name}")

not_required = fixture.get("not_required_for_bounded_readiness") or {}
require(not_required.get("entries_snapshot") == "FutureConsumerNeedOnly", "entries_snapshot requirement drift")
require(not_required.get("snapshot_owned") == "NamingCompatibilityCleanupOnly", "snapshot_owned requirement drift")
require(not_required.get("restore_owned") == "NamingCompatibilityCleanupOnly", "restore_owned requirement drift")
require(not_required.get("mut_lease") == "DeferredUntilLiveNeed", "mut lease requirement drift")

next_split = fixture.get("next_split") or {}
require(next_split.get("next_action") == "MIRBUILDER-VARIABLE-CONTEXT-ENTRIES-SNAPSHOT-NEED-RESOLVER-001", "next action drift")
require(next_split.get("if_entries_snapshot_needed") == "MIRBUILDER-VARIABLE-CONTEXT-ENTRIES-SNAPSHOT-PROJECTION-001", "entries next drift")
require(next_split.get("if_entries_snapshot_not_needed") == "NextRouteFamilySelectionPolicy", "route policy next drift")
require(next_split.get("if_mut_lease_needed") == "SOURCE-SELFHOST-RUST-MUTLEASE-SEMANTICS-DESIGN-STOP-001", "mut lease next drift")

claims = fixture.get("claims") or {}
for key in [
    "full_variable_context_claim",
    "source_selfhost_claim",
    "syntax_one_to_one_claim",
    "entries_snapshot_implemented",
    "snapshot_owned_implemented",
    "restore_owned_implemented",
    "raw_variable_map_alias_selected",
    "raw_variable_map_mut_alias_selected",
    "mut_lease_selected",
    "runtime_fallback",
    "new_backend_route",
    "new_abi",
    "new_python_semantic_projector",
    "manual_family_selection",
]:
    require(claims.get(key) == 0, f"claim drift: {key}")

reference_claims = reference.get("claims") or {}
require(reference_claims.get("full_variable_context_claim") == 0, "reference full claim drift")
require((reference.get("input_state") or {}).get("projection_model") == "SemanticOneToOneVerifiedProjection", "reference projection model drift")
require((adoption.get("decision") or {}).get("value") == "Adopt", "adoption decision drift")
require((adoption.get("target") or {}).get("surface_id") == surface_id, "adoption surface drift")
require(read_projection.get("kind") == "VariableContextOwnedReadSnapshotProjectionV1", "read projection kind drift")
require(mut_projection.get("kind") == "VariableContextExplicitMutationApiProjectionV1", "mutation projection kind drift")
require((mut_projection.get("projection") or {}).get("selected_policy") == "ExplicitMutationApiOnly", "mutation projection policy drift")

for source_token in [
    "lookup(ctx, name)",
    "contains(ctx, name): i64",
    "len(ctx): i64",
    "is_empty(ctx): i64",
    "snapshot(ctx: VariableContextNative): OrderedMapBox",
    "restore(ctx: VariableContextNative, snapshot: OrderedMapBox)",
    "replace_owned_map(ctx: VariableContextNative, owned_map: OrderedMapBox)",
    "insert(ctx, name, value_id): i64",
    "remove(ctx, name)",
]:
    require(source_token in native_source, f"native source missing {source_token}")
require("variable_map_mut" not in native_source, "native source must not expose variable_map_mut")

require(state.get("latest_card") == token, "current-state latest card drift")
require(state.get("current_blocker_token") == token, "current-state blocker drift")
require(state.get("latest_card_path") == "docs/development/current/main/phases/phase-296x/1796-MIRBUILDER-VARIABLE-CONTEXT-BOUNDED-NATIVE-SURFACE-READINESS-RESOLVER-001.md", "current-state latest card path drift")

for needle in [
    token,
    output_contract,
    "ReadyForBoundedVariableContextNativeSurfaceConsumer",
    surface_id,
    "MIRBUILDER-VARIABLE-CONTEXT-ENTRIES-SNAPSHOT-NEED-RESOLVER-001",
]:
    require(needle in task_order, f"task-order missing {needle}")

require("tools/checks/rust_lifecycle_variable_context_bounded_native_surface_readiness_guard.sh" in index, "check index missing guard")

print(f"output_contract={output_contract}")
print("decision=ReadyForBoundedVariableContextNativeSurfaceConsumer")
print("readiness_state=Ready")
print("reason_token=ExplicitMutationSurfaceAdoptedAndReferenceProjectionContractClosed")
print(f"selected_surface_id={surface_id}")
print("projection_model=SemanticOneToOneVerifiedProjection")
print("variable_map_projection=OwnedReadSnapshotProjection")
print("variable_map_mut_projection=ExplicitMutationApiOnly")
print("full_variable_context_claim=0")
print("source_selfhost_claim=0")
print("entries_snapshot_implemented=0")
print("mut_lease_selected=0")
print("runtime_fallback=0")
print("new_backend_route=0")
print("new_abi=0")
print("new_python_semantic_projector=0")
print("summary=ok")
PY
