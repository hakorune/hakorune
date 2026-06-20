#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
cd "$ROOT_DIR"

python3 - <<'PY'
import json
from pathlib import Path

routes = json.loads(Path("lang/generated/rust_derived/hakorune_mir_builder/family_routes.json").read_text())
artifact = json.loads(Path("lang/generated/rust_derived/hakorune_mir_builder/binding_context.artifact.json").read_text())

route = routes["routes"][0]
assert route["family_id"] == "hakorune_mir_builder::binding_context"
assert route["mainline_selection_scope"] == "BindingContext_only"
assert route["route"] == "derived_hako"
assert route["state"] == "DerivedMainline_candidate"
assert route["selected_on_mainline"] is False
assert route["not_selected_reason"] == "no_selfhost_family_artifact_route_seam"
assert route["fallback_policy"] == "forbidden"
assert route["rust_bootstrap_route"] == "retained"
assert route["rust_oracle_route"] == "retained"

claims = routes["claims"]
assert claims["runtime_try_hako_then_rust_fallback"] == 0
assert claims["source_selfhost_claim"] == 0
assert claims["backend_behavior_changed"] == 0
assert claims["variable_context_selected"] == 0
assert claims["mirbuilder_wide_claim"] == 0

assert artifact["claims"]["generated_hako_manual_edit"] == 0
assert artifact["claims"]["rust_bootstrap_retained"] == 1
PY

cat <<'REPORT'
output_contract=rust-lifecycle-binding-context-route-seam-v0
binding_context_route_seam_defined=1
mainline_selection_scope=BindingContext_only
selected_route=not_selected_with_reason
not_selected_reason=no_selfhost_family_artifact_route_seam
rust_bootstrap_retained=1
rust_oracle_retained=1
runtime_try_hako_then_rust_fallback=0
generated_artifact_manual_edit=0
source_selfhost_claim=0
backend_behavior_changed=0
summary=ok
REPORT
