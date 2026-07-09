#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
TAG="rust-lifecycle-mirbuilder-scalar-known-fastpath-post-non-delete-write-authority-island-closeout-design-stop"
source "$ROOT/tools/checks/lib/guard_common.sh"

FIXTURE="$ROOT/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-scalar-known-fastpath-post-non-delete-write-authority-island-closeout-design-stop-v0.json"
TOOL="$ROOT/tools/rust_lifecycle/mirbuilder_scalar_known_fastpath_post_non_delete_write_authority_island_closeout_design_stop.py"
CARD="$ROOT/docs/development/current/main/phases/phase-296x/3418-MIRBUILDER-SCALAR-KNOWN-FASTPATH-POST-NON-DELETE-WRITE-AUTHORITY-ISLAND-CLOSEOUT-DESIGN-STOP-001.md"
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
token="MIRBUILDER-SCALAR-KNOWN-FASTPATH-POST-NON-DELETE-WRITE-AUTHORITY-ISLAND-CLOSEOUT-DESIGN-STOP-001"
need(f["token"]==token and token in card and token in task, "token drift")
claims=f.get("claims") or {}
for k in ["post_non_delete_write_authority_island_closeout_design_stop","non_delete_write_hako_route_decision_authority_island_closeout","delete_surface_retired_special_case_parked"]:
    need(claims.get(k)==1, k)
for k in ["delete_hako_route_decision_authority_pilot","mapdeleteany_authority","write_surface_authority_closeout","write_wide_authority","source_selfhost_claim"]:
    need(claims.get(k)==0, k)
need(token in {r.get("token") for r in m.get("rows",[])}, "manifest missing token")
need("consultation_required = 1" in task, "task order missing consultation")
print("output_contract=rust-lifecycle-mirbuilder-scalar-known-fastpath-post-non-delete-write-authority-island-closeout-design-stop")
print("post_non_delete_write_authority_island_closeout_design_stop=1")
print("non_delete_write_hako_route_decision_authority_island_closeout=1")
print("write_surface_authority_closeout=0")
print("source_selfhost_claim=0")
print("summary=ok")
PY
