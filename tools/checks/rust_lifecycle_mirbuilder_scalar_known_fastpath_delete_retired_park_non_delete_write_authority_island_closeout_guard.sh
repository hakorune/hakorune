#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
TAG="rust-lifecycle-mirbuilder-scalar-known-fastpath-delete-retired-park-non-delete-write-authority-island-closeout"
source "$ROOT/tools/checks/lib/guard_common.sh"

FIXTURE="$ROOT/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-scalar-known-fastpath-delete-retired-park-non-delete-write-authority-island-closeout-v0.json"
TOOL="$ROOT/tools/rust_lifecycle/mirbuilder_scalar_known_fastpath_delete_retired_park_non_delete_write_authority_island_closeout.py"
CARD="$ROOT/docs/development/current/main/phases/phase-296x/3417-MIRBUILDER-SCALAR-KNOWN-FASTPATH-DELETE-RETIRED-PARK-NON-DELETE-WRITE-AUTHORITY-ISLAND-CLOSEOUT-001.md"
TASK_ORDER="$ROOT/docs/development/current/main/design/mirbuilder-rust-to-hako-converter-task-order-ssot.md"
MANIFEST="$ROOT/docs/development/current/main/design/fixtures/rust-lifecycle/source-selfhost-family-guard-manifest-v0.json"
WRITE_ROUTES="$ROOT/src/mir/generic_method_route_plan/write_routes.rs"
SHADOW="$ROOT/src/mir/generic_method_route_plan/scalar_known_hako_shadow.rs"

guard_require_command "$TAG" python3
guard_require_files "$TAG" "$FIXTURE" "$TOOL" "$CARD" "$TASK_ORDER" "$MANIFEST" "$WRITE_ROUTES" "$SHADOW"
python3 "$TOOL" --check

python3 - "$FIXTURE" "$CARD" "$TASK_ORDER" "$MANIFEST" "$WRITE_ROUTES" "$SHADOW" <<'PY'
import json, sys
from pathlib import Path
f=json.loads(Path(sys.argv[1]).read_text()); card=Path(sys.argv[2]).read_text(); task=Path(sys.argv[3]).read_text(); m=json.loads(Path(sys.argv[4]).read_text()); write=Path(sys.argv[5]).read_text(); shadow=Path(sys.argv[6]).read_text()
def need(c,msg):
    if not c: raise SystemExit(msg)
token="MIRBUILDER-SCALAR-KNOWN-FASTPATH-DELETE-RETIRED-PARK-NON-DELETE-WRITE-AUTHORITY-ISLAND-CLOSEOUT-001"
nxt="MIRBUILDER-SCALAR-KNOWN-FASTPATH-POST-NON-DELETE-WRITE-AUTHORITY-ISLAND-CLOSEOUT-DESIGN-STOP-001"
need(f["token"]==token and token in card and nxt in card, "token drift")
for needle in [
    "mapstore_i64_hako_route_authority_pilot_decision",
    "write_push_hako_route_authority_pilot_decision",
    "mapstore_any_hako_route_authority_pilot_decision",
]:
    need(needle in write or needle in shadow, f"non-delete authority helper missing: {needle}")
for needle in ["pub(super) fn match_generic_delete_route","GenericMethodRouteKind::MapDeleteAny","GenericMethodRouteProof::DeleteSurfacePolicy"]:
    need(needle in write, f"delete Rust route drift: {needle}")
claims=f.get("claims") or {}
for k in [
    "non_delete_write_hako_route_decision_authority_island_closeout",
    "delete_surface_retired_special_case_parked",
    "delete_surface_hako_mirror_retired",
    "delete_surface_live_rust_route_preserved",
    "closeout_scope_non_delete_write_only",
]:
    need(claims.get(k)==1, k)
for k in [
    "delete_hako_route_decision_authority_pilot",
    "mapdeleteany_authority",
    "write_surface_authority_closeout",
    "write_wide_authority",
    "runtime_mutation_authority",
    "publication_execution",
    "source_selfhost_claim",
]:
    need(claims.get(k)==0, k)
need(token in {r.get("token") for r in m.get("rows",[])}, "manifest missing token")
need(token in task and f"selected_next_card={nxt}" in task, "task order drift")
print("output_contract=rust-lifecycle-mirbuilder-scalar-known-fastpath-delete-retired-park-non-delete-write-authority-island-closeout")
print("non_delete_write_hako_route_decision_authority_island_closeout=1")
print("delete_surface_retired_special_case_parked=1")
print("write_surface_authority_closeout=0")
print("write_wide_authority=0")
print("source_selfhost_claim=0")
print("selected_next_card="+nxt)
print("summary=ok")
PY
