#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
cd "$ROOT_DIR"

bash tools/checks/rust_lifecycle_binding_context_derived_route_selection_guard.sh >/tmp/hako_binding_context_derived_route_selection_guard.out
bash tools/checks/rust_mirbuilder_binding_context_native_guard.sh >/tmp/phase296x_binding_context_native_min.guard.out

python3 - <<'PY'
import json
from pathlib import Path

route_path = Path("lang/generated/rust_derived/hakorune_mir_builder/family_routes.json")
artifact_path = Path("lang/generated/rust_derived/hakorune_mir_builder/binding_context.artifact.json")
native_path = Path("apps/lib/hakorune_mir_builder/binding_context.hako")

routes = json.loads(route_path.read_text())
artifact = json.loads(artifact_path.read_text())

route_entries = [
    route
    for route in routes["routes"]
    if route["artifact_manifest"] == str(artifact_path)
]
assert len(route_entries) == 1
route = route_entries[0]

assert route["family_id"] == "hakorune_mir_builder::binding_context"
assert route["state"] == "DerivedMainline"
assert route["selected_on_mainline"] is True
assert route["fallback_policy"] == "forbidden"
assert route["rust_bootstrap_route"] == "retained"
assert route["rust_oracle_route"] == "retained"
assert native_path.exists()
assert native_path.as_posix() != artifact_path.as_posix()

assert artifact["state"] == "DerivedShadow"
assert artifact["claims"]["generated_hako_manual_edit"] == 0
assert artifact["claims"]["mainline_selected"] == 0
assert artifact["claims"]["rust_bootstrap_retained"] == 1
assert artifact["claims"]["source_selfhost_claim"] == 0
assert artifact["claims"]["backend_behavior_changed"] == 0
PY

cat <<'REPORT'
output_contract=rust-lifecycle-binding-context-adoption-decision-v0
binding_context_current_state=DerivedMainline
selected_next_route=native_hako_source_owner
native_hako_source_owner_present=1
generator_overwrite_guard=1
rust_bootstrap_retained=1
rust_oracle_retained=1
generated_artifact_manual_edit=0
source_selfhost_claim=0
backend_behavior_changed=0
runtime_fallback=0
summary=ok
REPORT
