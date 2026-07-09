#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
TAG="rust-lifecycle-mirbuilder-scalar-known-fastpath-hako-caller-orientation-authority-design-stop"
source "$ROOT/tools/checks/lib/guard_common.sh"

FIXTURE="$ROOT/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-scalar-known-fastpath-hako-caller-orientation-authority-design-stop-v0.json"
TOOL="$ROOT/tools/rust_lifecycle/mirbuilder_scalar_known_fastpath_hako_caller_orientation_authority_design_stop.py"
CARD="$ROOT/docs/development/current/main/phases/phase-296x/3419-MIRBUILDER-SCALAR-KNOWN-FASTPATH-HAKO-CALLER-ORIENTATION-AUTHORITY-DESIGN-STOP-001.md"
TASK_ORDER="$ROOT/docs/development/current/main/design/mirbuilder-rust-to-hako-converter-task-order-ssot.md"
MANIFEST="$ROOT/docs/development/current/main/design/fixtures/rust-lifecycle/source-selfhost-family-guard-manifest-v0.json"

guard_require_command "$TAG" python3
guard_require_files "$TAG" "$FIXTURE" "$TOOL" "$CARD" "$TASK_ORDER" "$MANIFEST"
python3 "$TOOL" --check

python3 - "$FIXTURE" "$CARD" "$TASK_ORDER" "$MANIFEST" <<'PY'
import json, sys
from pathlib import Path
f=json.loads(Path(sys.argv[1]).read_text()); card=Path(sys.argv[2]).read_text(); task=Path(sys.argv[3]).read_text(); m=json.loads(Path(sys.argv[4]).read_text())
def need(c,msg):
    if not c: raise SystemExit(msg)
token="MIRBUILDER-SCALAR-KNOWN-FASTPATH-HAKO-CALLER-ORIENTATION-AUTHORITY-DESIGN-STOP-001"
need(f["token"]==token and token in card and token in task, "token drift")
need((f.get("decision") or {}).get("consultation_required") is True, "consultation drift")
claims=f.get("claims") or {}
for k in ["hako_caller_orientation_authority_design_stop","all_known_scalar_known_surfaces_shadow_consumed","fastpath_connected_closeout","non_delete_write_hako_route_decision_authority_island_closeout","selected_long_term_hako_caller_orientation","caller_orientation_requires_design_consultation"]:
    need(claims.get(k)==1, k)
for k in ["hako_runtime_route_authority","scalar_known_hako_runtime_route_authority","caller_orientation_runtime_path","rust_fastpath_rewired","route_selection_authority_switch","backend_lowering_authority","runtime_mutation_authority","publication_execution","write_surface_authority_closeout","write_wide_authority","delete_hako_route_decision_authority_pilot","mapdeleteany_authority","source_selfhost_claim","runtime_fallback","route_count_as_proof","row_count_as_proof","coverage_percentage_as_proof","source_path_as_authority","owner_name_as_proof","route_membership_alone_as_proof","manual_surface_selection"]:
    need(claims.get(k)==0, k)
need(token in {r.get("token") for r in m.get("rows",[])}, "manifest missing token")
need(m.get("current_blocker_token")==token, "manifest current blocker drift")
need("consultation_required = 1" in task, "task order missing consultation")
print("output_contract=rust-lifecycle-mirbuilder-scalar-known-fastpath-hako-caller-orientation-authority-design-stop")
print("hako_caller_orientation_authority_design_stop=1")
print("all_known_scalar_known_surfaces_shadow_consumed=1")
print("caller_orientation_requires_design_consultation=1")
print("caller_orientation_runtime_path=0")
print("source_selfhost_claim=0")
print("summary=ok")
PY
