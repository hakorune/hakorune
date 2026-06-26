#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
cd "$ROOT_DIR"

REPORT_GUARD_LOG="/tmp/hako_mirbuilder_minimal_path_composed_execution_report_guard.out"
ROUTE_SCRIPT="tools/rust_lifecycle/mirbuilder_minimal_path_composed_execution.py"
ROUTE_PATH="lang/generated/rust_derived/hakorune_mir_builder/mirbuilder_minimal_path_composed_execution.route.json"

bash tools/checks/rust_lifecycle_mirbuilder_minimal_execution_path_semantic_closure_report_guard.sh \
  >"$REPORT_GUARD_LOG" 2>&1

python3 "$ROUTE_SCRIPT" --check

python3 - <<'PY'
import json
from pathlib import Path

route = json.loads(
    Path(
        "lang/generated/rust_derived/hakorune_mir_builder/"
        "mirbuilder_minimal_path_composed_execution.route.json"
    ).read_text()
)

assert route["kind"] == "MinimalMirBuilderComposedExecutionRouteV1"
assert route["route_slot_id"] == "hakorune_mir_builder.minimal_path.composed_execution.v1"
assert route["selected_scope"] == "PreparedMirBuilderStateV1"
assert route["input_profile"]["ast"] == "ASTNode::Literal(Integer(0))"

source_prefix = route["source_order_prefix"]
assert [row["edge_id"] for row in source_prefix] == [
    "entry.prepared_state_profile",
    "build_module.prepare_module",
    "prepare_module.module_new",
    "prepare_module.next_block",
    "prepare_module.function_new",
    "prepare_module.state_install",
    "lower_root.literal_integer",
]

composition_prefix = route["composition_prefix"]
assert [row["edge_id"] for row in composition_prefix] == [
    "prepare_module.module_new",
    "prepare_module.next_block",
    "prepare_module.function_new",
    "prepare_module.state_install",
    "lower_root.literal_integer",
]
assert all(row["route_state"] == "DerivedShadow" for row in composition_prefix)

same_state = route["same_state_handoff"]
assert same_state["state_transport"] == "PreparedMirBuilderStateShell"
assert same_state["observed"] == 1
assert same_state["selected_existing_contracts_consumed"] == 1
assert same_state["fallback_to_standalone_harness"] == 0
assert same_state["generated_hako_change"] == 0

dependency_routes = route["dependency_routes"]
assert len(dependency_routes) == 1
dependency = dependency_routes[0]
assert dependency["kind"] == "DerivedMainlineRouteSelectionV1"
assert dependency["route_slot_id"] == "hakorune_mir_builder.allocation_policy.next_value_id.prepared_state.v1"
assert dependency["artifact_state"] == "DerivedMainline"

selected_contracts = route["selected_existing_contracts"]
assert len(selected_contracts) == 5
assert [row["edge_id"] for row in selected_contracts] == [
    "prepare_module.module_new",
    "prepare_module.next_block",
    "prepare_module.function_new",
    "prepare_module.state_install",
    "lower_root.literal_integer",
]
for row in selected_contracts:
    assert row["state"] == "DerivedShadow"
    assert row["manifest_path"].startswith("lang/generated/rust_derived/hakorune_mir_builder/")
    assert row["hako_path"].endswith(".hako")

claims = route["claims"]
assert claims["generated_route_change"] == 1
assert claims["selected_existing_contracts_consumed"] == 1
assert claims["same_state_handoff_observed"] == 1
assert claims["generated_hako_change"] == 0
assert claims["semantic_recipe_recopy"] == 0
assert claims["fallback_to_standalone_harness"] == 0
assert claims["runtime_fallback"] == 0
assert claims["new_backend_route"] == 0
assert claims["new_abi"] == 0
assert claims["source_selfhost_claim"] == 0
assert claims["manual_next_edge_selection"] == 0

print("output_contract=rust-lifecycle-minimal-path-composed-execution-route-v0")
print("composition_guard=green")
print("same_state_handoff_observed=1")
print("selected_existing_contracts_consumed=1")
print("generated_route_change=1")
print("generated_hako_change=0")
print("fallback_to_standalone_harness=0")
print("runtime_fallback=0")
print("new_backend_route=0")
print("new_abi=0")
print("source_selfhost_claim=0")
print("summary=ok")
PY
