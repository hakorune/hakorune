#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
cd "$ROOT_DIR"

python3 tools/rust_lifecycle/convert_mirbuilder_lightweight_facts.py \
  --family mirbuilder-next-value-id-prepared-state-kernel --check
python3 tools/rust_lifecycle/mirbuilder_allocation_policy_mainline_selection.py --check
bash tools/checks/rust_lifecycle_mirbuilder_next_value_id_prepared_state_kernel_guard.sh \
  >/tmp/hako_mirbuilder_allocation_policy_mainline_kernel_guard.out

python3 - <<'PY'
import json
from pathlib import Path

plan = json.loads(Path("docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-allocation-policy-mainline-selection-plan-v0.json").read_text())
route = json.loads(Path("lang/generated/rust_derived/hakorune_mir_builder/mirbuilder_next_value_id_prepared_state_kernel.route.json").read_text())
artifact = json.loads(Path("lang/generated/rust_derived/hakorune_mir_builder/mirbuilder_next_value_id_prepared_state_kernel.artifact.json").read_text())

assert plan["kind"] == "MirBuilderAllocationPolicyMainlineSelectionPlanV1"
assert plan["route_slot_id"] == "hakorune_mir_builder.allocation_policy.next_value_id.prepared_state.v1"
assert plan["profiles"]["selfhost_mainline"]["route"] == "derived_hako"
assert plan["profiles"]["rust_bootstrap"]["route"] == "rust_bootstrap"
assert plan["fallback_policy"] == "Forbidden"
assert "MirBuilderAllocationPolicyApi.next_value_id/4" not in plan["route_slot_id"]
assert plan["claims"]["mainline_selected"] == 1
assert plan["claims"]["full_mirbuilder_object_method"] == 0
assert plan["claims"]["hako_adopted"] == 0
assert plan["claims"]["source_selfhost_claim"] == 0
assert plan["claims"]["runtime_fallback"] == 0

assert artifact["state"] == "DerivedMainline"
assert artifact["claims"]["mainline_selected"] == 1
assert artifact["claims"]["rust_bootstrap_retained"] == 1
assert artifact["claims"]["prepared_state_policy_kernel"] == 1
assert artifact["claims"]["full_mirbuilder_object_method"] == 0
assert artifact["claims"]["source_selfhost_claim"] == 0
assert artifact["claims"]["hako_adopted"] == 0
assert artifact["claims"]["native_hako_edit_authority"] == 0
assert artifact["claims"]["runtime_fallback"] == 0
assert artifact["mainline_selection"]["route_slot_id"] == plan["route_slot_id"]
assert artifact["mainline_selection"]["fallback_policy"] == "Forbidden"

assert route["kind"] == "DerivedMainlineRouteSelectionV1"
assert route["route_slot_id"] == plan["route_slot_id"]
assert route["profiles"]["selfhost_mainline"]["route"] == "derived_hako"
assert route["profiles"]["rust_bootstrap"]["route"] == "rust_bootstrap"
assert route["fallback_policy"] == "Forbidden"
assert route["claims"]["runtime_try_hako_then_rust_fallback"] == 0
assert route["claims"]["source_selfhost_claim"] == 0
assert route["claims"]["new_backend_route"] == 0
assert route["claims"]["new_abi"] == 0
classifications = {row["classification"] for row in route["selected_route_closure"]}
assert "ForbiddenRustSemanticDependency" not in classifications
assert "SameArtifactHako" in classifications
assert "AllowedHostSubstrate" in classifications
PY

cat <<'REPORT'
output_contract=rust-lifecycle-mirbuilder-allocation-policy-mainline-pilot-v0
family_id=hakorune_mir_builder::next_value_id_prepared_state_kernel
selected_scope=PreparedStateMirBuilderNextValueIdKernel
selfhost_mainline=derived_hako
rust_bootstrap=rust_bootstrap
artifact_state=DerivedMainline
mainline_selected=1
fallback_policy=forbidden
runtime_try_hako_then_rust_fallback=0
source_selfhost_claim=0
new_backend_route=0
new_abi=0
summary=ok
REPORT
