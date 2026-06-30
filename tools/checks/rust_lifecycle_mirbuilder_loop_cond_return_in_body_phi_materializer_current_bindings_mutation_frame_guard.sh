#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
FIXTURE="$ROOT/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-loop-cond-return-in-body-phi-materializer-current-bindings-mutation-frame-v0.json"
TOOL="$ROOT/tools/rust_lifecycle/mirbuilder_loop_cond_return_in_body_phi_materializer_current_bindings_mutation_frame.py"

python3 "$TOOL" --check

python3 - "$FIXTURE" <<'PY'
import json
import sys
from pathlib import Path

data = json.load(open(sys.argv[1], encoding="utf-8"))

def need(cond, msg):
    if not cond:
        raise SystemExit(msg)

token = "MIRBUILDER-LOOP-COND-RETURN-IN-BODY-PHI-MATERIALIZER-CURRENT-BINDINGS-MUTATION-FRAME-001"
need(data.get("kind") == "MirBuilderLoopCondReturnInBodyPhiMaterializerCurrentBindingsMutationFrameV1", "bad kind")
need(data.get("token") == token, "bad token")

input_state = data.get("input_state") or {}
need(input_state.get("selected_policy") == "BoundedWithMapOperation", "bad input policy")
need(input_state.get("strict_raw_borrow_policy") == "Deny", "raw borrow policy must stay Deny")
need(
    input_state.get("replacement_id")
    == "LoopCondReturnInBodyPhiMaterializerCurrentBindingsMutationFrameV1",
    "bad replacement id",
)

contract = data.get("mutation_frame_contract") or {}
need(contract.get("frame_kind") == "BoundedOwnerMutationFrame", "bad frame kind")
need(contract.get("replacement_policy") == "BoundedWithMapOperation", "bad replacement policy")
need(contract.get("owner") == "LoopCondReturnInBodyPhiMaterializer", "bad owner")
need(contract.get("owned_field") == "current_bindings", "bad owned field")
need(contract.get("entry_surface") == "current_bindings_mut", "bad entry surface")
need(contract.get("bounded_callsite") == "lower_return_in_body_block", "bad bounded callsite")

for value in [
    "LoopCondReturnInBodyPhiMaterializer.current_bindings",
    "carrier_updates",
    "builder.variable_ctx.variable_map",
    "lowered body plans",
]:
    need(value in contract.get("state_outputs", []), f"missing state output {value}")
for value in ["carrier_phis", "carrier_step_phis", "recipe.items", "recipe.body"]:
    need(value in contract.get("read_only_inputs", []), f"missing read-only input {value}")
for value in [
    "EnterBoundedCurrentBindingsFrame",
    "IterateRecipeItemsInSourceOrder",
    "DispatchStatementAst",
    "LowerAssignmentThroughCarrierMerge",
    "LowerLocalInitThroughCarrierMerge",
    "LowerMethodCallEffectsWithCurrentBindings",
    "LowerFunctionCallEffectsWithCurrentBindings",
    "LowerPrintEffectsWithCurrentBindings",
    "LowerIfWithJoinedBranchBindings",
    "RecordCarrierUpdatesFromJoinedCarrierPhis",
    "LowerReturnWithCurrentBindings",
    "ReturnLoweredBodyPlans",
    "ExitFrameWithoutAliasEscape",
]:
    need(value in contract.get("mutation_order", []), f"missing mutation order item {value}")
for value in [
    "ReturnMutableMapAlias",
    "StoreMutableBorrow",
    "CallerOwnedMutableAlias",
    "RustLifetimeSyntaxTransport",
    "RuntimeFallback",
]:
    need(value in contract.get("forbidden_operations", []), f"missing forbidden op {value}")

for section in data.get("source_order_sections") or []:
    source = Path(section["source_path"]).read_text(encoding="utf-8")
    last = -1
    for marker in section["markers"]:
        index = source.find(marker, last + 1)
        if index < 0:
            raise SystemExit(f"source marker missing or out of order: {section['source_path']} :: {marker}")
        last = index

decision = data.get("decision") or {}
need(decision.get("kind") == "SelectNextDiagnosticClusterResolution", "bad decision kind")
need(decision.get("selected_next_card") == "MIRBUILDER-MULTI-AXIS-DIAGNOSTIC-CLUSTER-RESOLUTION-001", "bad next card")

claims = data.get("claims") or {}
need(claims.get("bounded_mutation_frame_contract_ready") == 1, "contract must be ready")
for key in [
    "raw_mutable_alias_selected",
    "returned_mutable_borrow_allowed",
    "stored_borrow_allowed",
    "caller_owned_mutable_alias",
    "hako_generation",
    "hako_shadow_projector_selected",
    "hako_adopted_decision",
    "native_seed_materialization",
    "source_selfhost_claim",
    "runtime_fallback",
    "new_backend_route",
    "new_abi",
    "new_python_semantic_projector",
    "runner_semantic_owner",
    "manual_family_selection",
    "manual_axis_selection",
]:
    need(claims.get(key) == 0, f"{key} must be 0")

print("output_contract=rust-lifecycle-mirbuilder-loop-cond-return-in-body-current-bindings-mutation-frame")
print("bounded_mutation_frame_contract_ready=1")
print("raw_returned_mutable_borrow=Deny")
print(f"selected_next_card={decision.get('selected_next_card')}")
print("hako_generation=0")
print("source_selfhost_claim=0")
print("runtime_fallback=0")
print("summary=ok")
PY
