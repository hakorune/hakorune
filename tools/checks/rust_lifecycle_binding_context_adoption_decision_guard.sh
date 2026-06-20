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
assert route["state"] == "DerivedMainline_candidate"
assert route["selected_on_mainline"] is False
assert route["fallback_policy"] == "forbidden"
assert route["rust_bootstrap_route"] == "retained"
assert route["rust_oracle_route"] == "retained"

assert artifact["state"] == "DerivedShadow"
assert artifact["claims"]["generated_hako_manual_edit"] == 0
assert artifact["claims"]["mainline_selected"] == 0
assert artifact["claims"]["rust_bootstrap_retained"] == 1
assert artifact["claims"]["source_selfhost_claim"] == 0
PY

cat <<'REPORT'
output_contract=rust-lifecycle-binding-context-adoption-decision-v0
binding_context_current_state=DerivedMainline_candidate
selected_next_route=wait_for_route_seam
rust_bootstrap_retained=1
rust_oracle_retained=1
generated_artifact_manual_edit=0
source_selfhost_claim=0
backend_behavior_changed=0
summary=ok
REPORT
