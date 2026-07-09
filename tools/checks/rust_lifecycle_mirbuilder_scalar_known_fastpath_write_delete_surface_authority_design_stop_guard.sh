#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
TAG="rust-lifecycle-mirbuilder-scalar-known-fastpath-write-delete-surface-authority-design-stop"
source "$ROOT/tools/checks/lib/guard_common.sh"

FIXTURE="$ROOT/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-scalar-known-fastpath-write-delete-surface-authority-design-stop-v0.json"
TOOL="$ROOT/tools/rust_lifecycle/mirbuilder_scalar_known_fastpath_write_delete_surface_authority_design_stop.py"
CARD="$ROOT/docs/development/current/main/phases/phase-296x/3416-MIRBUILDER-SCALAR-KNOWN-FASTPATH-WRITE-DELETE-SURFACE-AUTHORITY-DESIGN-STOP-001.md"
TASK_ORDER="$ROOT/docs/development/current/main/design/mirbuilder-rust-to-hako-converter-task-order-ssot.md"
MANIFEST="$ROOT/docs/development/current/main/design/fixtures/rust-lifecycle/source-selfhost-family-guard-manifest-v0.json"
WRITE_ROUTES="$ROOT/src/mir/generic_method_route_plan/write_routes.rs"

guard_require_command "$TAG" python3
guard_require_files "$TAG" "$FIXTURE" "$TOOL" "$CARD" "$TASK_ORDER" "$MANIFEST" "$WRITE_ROUTES"
python3 "$TOOL" --check

python3 - "$FIXTURE" "$CARD" "$TASK_ORDER" "$MANIFEST" "$WRITE_ROUTES" <<'PY'
import json, sys
from pathlib import Path
f=json.loads(Path(sys.argv[1]).read_text()); card=Path(sys.argv[2]).read_text(); task=Path(sys.argv[3]).read_text(); m=json.loads(Path(sys.argv[4]).read_text()); write=Path(sys.argv[5]).read_text()
def need(c,msg):
    if not c: raise SystemExit(msg)
token="MIRBUILDER-SCALAR-KNOWN-FASTPATH-WRITE-DELETE-SURFACE-AUTHORITY-DESIGN-STOP-001"
need(f["token"]==token and token in card and token in task, "token drift")
for needle in ["pub(super) fn match_generic_delete_route","GenericMethodRouteKind::MapDeleteAny","GenericMethodRouteProof::DeleteSurfacePolicy"]:
    need(needle in write, f"live delete route drift: {needle}")
inv=f.get("inventory") or {}
need(inv.get("rust_live_route_preserved") is True, "rust route not preserved")
need(inv.get("generated_typed_hako_artifact_exists") is False, "delete artifact unexpectedly exists")
need(inv.get("hako_authority_helper_exists") is False, "delete helper unexpectedly exists")
taxonomy=f.get("taxonomy_application") or {}
for key in ["authority.surface.delete.route_decision","authority.surface.write.wide","authority.runtime.mutation","authority.source_selfhost"]:
    need(taxonomy.get(key)==0, f"taxonomy application drift: {key}")
claims=f.get("claims") or {}
for k in ["delete_surface_authority_design_stop","claim_taxonomy_applied","rust_map_delete_route_preserved"]:
    need(claims.get(k)==1, k)
for k in ["delete_hako_route_decision_authority_pilot","mapdeleteany_authority","write_surface_authority_closeout","source_selfhost_claim"]:
    need(claims.get(k)==0, k)
need(token in {r.get("token") for r in m.get("rows",[])}, "manifest missing token")
need("consultation_required = 1" in task, "task order missing consultation")
print("output_contract=rust-lifecycle-mirbuilder-scalar-known-fastpath-write-delete-surface-authority-design-stop")
print("delete_surface_authority_design_stop=1")
print("claim_taxonomy_applied=1")
print("rust_map_delete_route_preserved=1")
print("delete_hako_route_decision_authority_pilot=0")
print("mapdeleteany_authority=0")
print("source_selfhost_claim=0")
print("summary=ok")
PY
