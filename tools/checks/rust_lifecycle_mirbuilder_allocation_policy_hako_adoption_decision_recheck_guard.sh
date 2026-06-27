#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="rust-lifecycle-mirbuilder-allocation-policy-hako-adoption-decision-recheck-guard"

source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

STATE="$ROOT_DIR/docs/development/current/main/CURRENT_STATE.toml"
CARD="$ROOT_DIR/docs/development/current/main/phases/phase-296x/296x-1760-MIRBUILDER-ALLOCATION-POLICY-HAKO-ADOPTION-DECISION-RECHECK-001.md"
PREFIX="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-minimal-path-composed-prefix-advance-v1.json"
ROUTE="$ROOT_DIR/lang/generated/rust_derived/hakorune_mir_builder/mirbuilder_next_value_id_prepared_state_kernel.route.json"
ARTIFACT="$ROOT_DIR/lang/generated/rust_derived/hakorune_mir_builder/mirbuilder_next_value_id_prepared_state_kernel.artifact.json"
FIXTURE="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-allocation-policy-hako-adoption-decision-recheck-v0.json"

guard_require_command "$TAG" python3
guard_require_command "$TAG" rg
guard_require_files "$TAG" "$STATE" "$CARD" "$PREFIX" "$ROUTE" "$ARTIFACT" "$FIXTURE"

python3 - <<'PY'
import json
from pathlib import Path
import tomllib

state = tomllib.loads(Path("docs/development/current/main/CURRENT_STATE.toml").read_text())
assert state["latest_card"] == "MIRBUILDER-ALLOCATION-POLICY-HAKO-ADOPTION-DECISION-RECHECK-001"
assert state["current_blocker_token"] == "MIRBUILDER-ALLOCATION-POLICY-HAKO-ADOPTION-DECISION-RECHECK-001"

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

native_owner_hits = []
for path in Path("lang/src").rglob("*.hako"):
    text = path.read_text()
    if "next_value_id_prepared_state_kernel" in text or "prepared_state_next_value_id" in text:
        native_owner_hits.append(str(path))
assert not native_owner_hits, native_owner_hits

fixture = json.loads(Path("docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-allocation-policy-hako-adoption-decision-recheck-v0.json").read_text())
assert fixture["decision"] == "Defer"
assert fixture["reason_token"] == "NativeHakoSourceOwnerMissing"
assert fixture["input_evidence"]["native_source_owner_present"] == 0
PY

cat <<'REPORT'
output_contract=rust-lifecycle-mirbuilder-allocation-policy-hako-adoption-decision-recheck-v0
family_id=hakorune_mir_builder::next_value_id_prepared_state_kernel
composed_prefix_result_consumed=1
prefix_state=Green
next_unconsumed_edge_classification=Closed
target_family_is_derived_mainline=1
target_scope_is_narrow=1
route_selection_present=1
rust_bootstrap_retained=1
fallback_policy=Forbidden
decision=Defer
reason_token=NativeHakoSourceOwnerMissing
native_hako_source_owner_present=0
runtime_fallback=0
new_backend_route=0
new_abi=0
source_selfhost_claim=0
rust_source_delete=0
manual_next_owner_selection=0
summary=ok
REPORT
