#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
cd "$ROOT_DIR"

python3 - <<'PY'
import json
from pathlib import Path

facts = json.loads(Path("docs/development/current/main/design/fixtures/rust-lifecycle/variable-context-simple-map-facts-v0.json").read_text())
plan = json.loads(Path("docs/development/current/main/design/fixtures/rust-lifecycle/variable-context-simple-map-plan-v0.json").read_text())
oracle = json.loads(Path("docs/development/current/main/design/fixtures/rust-lifecycle/variable-context-simple-map-oracle-vectors-v0.json").read_text())
routes = json.loads(Path("lang/generated/rust_derived/hakorune_mir_builder/family_routes.json").read_text())

assert facts["subject"] == "hakorune_mir_builder::variable_context::VariableContext.simple_map"
assert plan["subject"] == facts["subject"]
assert oracle["subject"] == facts["subject"]

denied = {entry["id"] for entry in facts["excluded_methods"]}
for method in [
    "VariableContext::variable_map",
    "VariableContext::variable_map_mut",
    "VariableContext::snapshot",
    "VariableContext::restore",
]:
    assert method in denied

binding_route = routes["routes"][0]
assert binding_route["family_id"] == "hakorune_mir_builder::binding_context"
assert binding_route["state"] == "DerivedMainline"
assert binding_route["selected_on_mainline"] is True
assert binding_route["fallback_policy"] == "forbidden"
PY

cat <<'REPORT'
output_contract=rust-lifecycle-variable-context-derived-pilot-selection-v0
selected_next_pilot=VariableContext
pilot_scope=VariableContext_simple_map_only
denied_returned_borrow_methods=2
denied_snapshot_restore_methods=2
binding_context_route_state=DerivedMainline
rust_bootstrap_retained=1
runtime_try_hako_then_rust_fallback=0
implementation_started=0
backend_behavior_changed=0
summary=ok
REPORT
