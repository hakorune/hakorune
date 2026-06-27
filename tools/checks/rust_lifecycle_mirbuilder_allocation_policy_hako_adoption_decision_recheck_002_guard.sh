#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="rust-lifecycle-mirbuilder-allocation-policy-hako-adoption-decision-recheck-002-guard"

source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

STATE="$ROOT_DIR/docs/development/current/main/CURRENT_STATE.toml"
CARD="$ROOT_DIR/docs/development/current/main/phases/phase-296x/296x-1762-MIRBUILDER-ALLOCATION-POLICY-HAKO-ADOPTION-DECISION-RECHECK-002.md"
PREFIX="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-minimal-path-composed-prefix-advance-v1.json"
ROUTE="$ROOT_DIR/lang/generated/rust_derived/hakorune_mir_builder/mirbuilder_next_value_id_prepared_state_kernel.route.json"
ARTIFACT="$ROOT_DIR/lang/generated/rust_derived/hakorune_mir_builder/mirbuilder_next_value_id_prepared_state_kernel.artifact.json"
FIXTURE="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-allocation-policy-hako-adoption-decision-recheck-v1.json"
NATIVE_OWNER="$ROOT_DIR/lang/src/compiler/lib/next_value_id_prepared_state_kernel.hako"

guard_require_command "$TAG" python3
guard_require_command "$TAG" rg
guard_require_files "$TAG" "$STATE" "$CARD" "$PREFIX" "$ROUTE" "$ARTIFACT" "$FIXTURE" "$NATIVE_OWNER"

python3 - <<'PY'
import json
from pathlib import Path
import tomllib

state = tomllib.loads(Path("docs/development/current/main/CURRENT_STATE.toml").read_text())
assert state["latest_card"] == "MIRBUILDER-ALLOCATION-POLICY-HAKO-ADOPTION-DECISION-RECHECK-002"
assert state["current_blocker_token"] == "MIRBUILDER-ALLOCATION-POLICY-HAKO-ADOPTION-DECISION-RECHECK-002"

prefix = json.loads(Path("docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-minimal-path-composed-prefix-advance-v1.json").read_text())
assert prefix["prefix_advance"]["prefix_state"] == "Green"
assert prefix["next_unconsumed_edge"]["classification"] == "Closed"
assert prefix["prefix_advance"]["stable_next_slice_token"] == "MIRBUILDER-MINIMAL-EXECUTION-PATH-COMPLETION-DESIGN-STOP-001"

route = json.loads(Path("lang/generated/rust_derived/hakorune_mir_builder/mirbuilder_next_value_id_prepared_state_kernel.route.json").read_text())
artifact = json.loads(Path("lang/generated/rust_derived/hakorune_mir_builder/mirbuilder_next_value_id_prepared_state_kernel.artifact.json").read_text())
assert route["kind"] == "DerivedMainlineRouteSelectionV1"
assert route["profiles"]["selfhost_mainline"]["route"] == "derived_hako"
assert route["fallback_policy"] == "Forbidden"
assert route["claims"]["source_selfhost_claim"] == 0
assert artifact["state"] == "DerivedMainline"
assert artifact["claims"]["mainline_selected"] == 1
assert artifact["claims"]["source_selfhost_claim"] == 0
assert artifact["claims"]["hako_adopted"] == 0
assert artifact["claims"]["native_hako_edit_authority"] == 0

native_owner = Path("lang/src/compiler/lib/next_value_id_prepared_state_kernel.hako").read_text()
assert "manual-edit: forbidden" not in native_owner
assert "static box MirBuilderAllocationPolicyApi" in native_owner
assert "next_value_id(current_function_present, function_state, core_context, reserved_membership)" in native_owner
assert "lib.next_value_id_prepared_state_kernel = \"lib/next_value_id_prepared_state_kernel.hako\"" in Path("lang/src/compiler/hako_module.toml").read_text()
assert "first native source owner candidate" in Path("lang/src/compiler/lib/README.md").read_text()

fixture = json.loads(Path("docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-allocation-policy-hako-adoption-decision-recheck-v1.json").read_text())
assert fixture["decision"] == "Adopt"
assert fixture["reason_token"] == ""
assert fixture["input_evidence"]["native_source_owner_present"] == 1
assert fixture["input_evidence"]["generator_overwrite_guard"] == 1
PY

cat <<'REPORT'
output_contract=rust-lifecycle-mirbuilder-allocation-policy-hako-adoption-decision-recheck-v1
family_id=hakorune_mir_builder::next_value_id_prepared_state_kernel
composed_prefix_result_consumed=1
prefix_state=Green
next_unconsumed_edge_classification=Closed
target_family_is_derived_mainline=1
target_scope_is_narrow=1
route_selection_present=1
rust_bootstrap_retained=1
fallback_policy=Forbidden
decision=Adopt
reason_token=
native_hako_source_owner_present=1
generator_overwrite_guard=1
runtime_fallback=0
new_backend_route=0
new_abi=0
source_selfhost_claim=0
rust_source_delete=0
manual_next_owner_selection=0
summary=ok
REPORT
