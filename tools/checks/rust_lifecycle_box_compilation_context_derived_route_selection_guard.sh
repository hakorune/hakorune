#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
cd "$ROOT_DIR"

GENERATOR_LOG="/tmp/hako_box_compilation_context_derived_route_selection_generator.out"

if ! python3 tools/rust_lifecycle/convert_mirbuilder_lightweight_facts.py --family box-compilation-context --check >"$GENERATOR_LOG" 2>&1; then
    cat "$GENERATOR_LOG"
    exit 1
fi

bash tools/checks/selfhost_family_artifact_route_seam_ssot_guard.sh >/tmp/hako_family_artifact_route_seam_guard.out

python3 - <<'PY'
import json
from pathlib import Path

routes = json.loads(Path("lang/generated/rust_derived/hakorune_mir_builder/family_routes.json").read_text())
artifact = json.loads(Path("lang/generated/rust_derived/hakorune_mir_builder/box_compilation_context.artifact.json").read_text())

route = None
for row in routes["routes"]:
    if row["artifact_manifest"] == "lang/generated/rust_derived/hakorune_mir_builder/box_compilation_context.artifact.json":
        route = row
        break
assert route is not None
assert route["family_id"] == "hakorune_mir_builder::context"
assert route["mainline_selection_scope"] == "BoxCompilationContext_only"
assert route["route"] == "derived_hako"
assert route["state"] == "DerivedMainline"
assert route["selected_on_mainline"] is True
assert route["fallback_policy"] == "forbidden"
assert route["rust_bootstrap_route"] == "retained"
assert route["rust_oracle_route"] == "retained"
assert "not_selected_reason" not in route

assert artifact["kind"] == "RustDerivedHakoArtifact"
assert artifact["family_id"] == route["family_id"]
assert artifact["state"] == "DerivedShadow"
assert artifact["claims"]["generated_hako_manual_edit"] == 0
assert artifact["claims"]["rust_bootstrap_retained"] == 1
assert artifact["claims"]["source_selfhost_claim"] == 0
assert artifact["claims"]["backend_behavior_changed"] == 0

claims = routes["claims"]
assert claims["runtime_try_hako_then_rust_fallback"] == 0
assert claims["source_selfhost_claim"] == 0
assert claims["backend_behavior_changed"] == 0
PY

cat <<'REPORT'
output_contract=rust-lifecycle-box-compilation-context-derived-route-selection-v0
family_id=hakorune_mir_builder::context
selected_route=derived_hako
route_state=DerivedMainline
route_seam_ssot_verified=1
artifact_manifest_verified=1
rust_bootstrap_retained=1
rust_oracle_retained=1
runtime_try_hako_then_rust_fallback=0
generated_artifact_manual_edit=0
source_selfhost_claim=0
backend_behavior_changed=0
summary=ok
REPORT
