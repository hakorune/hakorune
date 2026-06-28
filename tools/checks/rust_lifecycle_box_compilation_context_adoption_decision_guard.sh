#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
cd "$ROOT_DIR"

bash tools/checks/rust_lifecycle_box_compilation_context_derived_route_selection_guard.sh >/tmp/hako_box_compilation_context_route_guard.out
bash tools/checks/rust_mirbuilder_box_compilation_context_native_guard.sh >/tmp/hako_box_compilation_context_native_guard.out

python3 - <<'PY'
import json
from pathlib import Path

routes = json.loads(Path("lang/generated/rust_derived/hakorune_mir_builder/family_routes.json").read_text())
route = None
for row in routes["routes"]:
    if row["artifact_manifest"] == "lang/generated/rust_derived/hakorune_mir_builder/box_compilation_context.artifact.json":
        route = row
        break
assert route is not None
assert route["family_id"] == "hakorune_mir_builder::context"
assert route["route"] == "derived_hako"
assert route["state"] == "DerivedMainline"
assert route["selected_on_mainline"] is True
assert route["fallback_policy"] == "forbidden"
assert route["rust_bootstrap_route"] == "retained"
assert route["rust_oracle_route"] == "retained"

artifact = json.loads(Path("lang/generated/rust_derived/hakorune_mir_builder/box_compilation_context.artifact.json").read_text())
assert artifact["kind"] == "RustDerivedHakoArtifact"
assert artifact["family_id"] == "hakorune_mir_builder::context"
assert artifact["state"] == "DerivedShadow"
assert artifact["claims"]["generated_hako_manual_edit"] == 0
assert artifact["claims"]["rust_bootstrap_retained"] == 1
assert artifact["claims"]["source_selfhost_claim"] == 0

fixture = json.loads(Path("docs/development/current/main/design/fixtures/rust-lifecycle/box-compilation-context-adoption-decision-v0.json").read_text())
assert fixture["decision"] == "Adopt"
assert fixture["input_evidence"]["native_hako_source_owner_present"] == 1
assert fixture["generator_overwrite_guard"] == 1
PY

cat <<'REPORT'
output_contract=rust-lifecycle-box-compilation-context-adoption-decision-v0
family_id=hakorune_mir_builder::context
selected_route=derived_hako
route_state=DerivedMainline
native_hako_source_owner_present=1
generator_overwrite_guard=1
decision=Adopt
runtime_fallback=0
new_backend_route=0
new_abi=0
source_selfhost_claim=0
manual_next_owner_selection=0
summary=ok
REPORT
