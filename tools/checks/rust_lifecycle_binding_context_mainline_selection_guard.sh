#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
cd "$ROOT_DIR"

ROUTE_MANIFEST="lang/generated/rust_derived/hakorune_mir_builder/family_routes.json"
ARTIFACT_MANIFEST="lang/generated/rust_derived/hakorune_mir_builder/binding_context.artifact.json"
GENERATOR_LOG="/tmp/hako_binding_context_mainline_selection_generator.out"

if ! python3 tools/rust_lifecycle/convert_mirbuilder_lightweight_facts.py --family binding-context --check >"$GENERATOR_LOG" 2>&1; then
    cat "$GENERATOR_LOG"
    exit 1
fi

python3 - <<'PY'
import json
from pathlib import Path

route_path = Path("lang/generated/rust_derived/hakorune_mir_builder/family_routes.json")
artifact_path = Path("lang/generated/rust_derived/hakorune_mir_builder/binding_context.artifact.json")

routes = json.loads(route_path.read_text())
artifact = json.loads(artifact_path.read_text())

assert routes["schema_version"] == 0
assert routes["kind"] == "RustDerivedHakoFamilyRouteManifest"
assert routes["crate"] == "hakorune_mir_builder"

claims = routes["claims"]
assert claims["source_selfhost_claim"] == 0
assert claims["backend_behavior_changed"] == 0
assert claims["runtime_try_hako_then_rust_fallback"] == 0
assert claims["variable_context_selected"] == 0
assert claims["mirbuilder_wide_claim"] == 0

route_entries = routes["routes"]
assert len(route_entries) == 1
route = route_entries[0]

assert route["family_id"] == "hakorune_mir_builder::binding_context"
assert route["route"] == "derived_hako"
assert route["state"] == "DerivedMainline_candidate"
assert route["mainline_selection_scope"] == "BindingContext_only"
assert route["selected_on_mainline"] is False
assert route["artifact_manifest"] == str(artifact_path)
assert route["rust_bootstrap_route"] == "retained"
assert route["rust_oracle_route"] == "retained"
assert route["fallback_policy"] == "forbidden"

assert artifact["kind"] == "RustDerivedHakoArtifact"
assert artifact["family_id"] == route["family_id"]
assert artifact["state"] == "DerivedShadow"
assert artifact["claims"]["mainline_selected"] == 0
assert artifact["claims"]["rust_bootstrap_retained"] == 1
assert artifact["claims"]["source_selfhost_claim"] == 0
assert artifact["claims"]["backend_behavior_changed"] == 0
PY

cat <<'REPORT'
output_contract=rust-lifecycle-binding-context-mainline-selection-v0
binding_context_artifact_state=DerivedMainline_candidate
mainline_selection_scope=BindingContext_only
generated_artifact_manifest_verified=1
rust_bootstrap_retained=1
rust_oracle_retained=1
silent_fallback=0
runtime_try_hako_then_rust_fallback=0
source_selfhost_claim=0
backend_behavior_changed=0
selected_on_mainline=0
summary=ok
REPORT
