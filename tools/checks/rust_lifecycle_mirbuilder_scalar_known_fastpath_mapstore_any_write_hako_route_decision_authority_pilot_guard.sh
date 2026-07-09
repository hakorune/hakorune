#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
TAG="rust-lifecycle-mirbuilder-scalar-known-fastpath-mapstore-any-write-hako-route-decision-authority-pilot"
source "$ROOT/tools/checks/lib/guard_common.sh"

FIXTURE="$ROOT/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-scalar-known-fastpath-mapstore-any-write-hako-route-decision-authority-pilot-v0.json"
TOOL="$ROOT/tools/rust_lifecycle/mirbuilder_scalar_known_fastpath_mapstore_any_write_hako_route_decision_authority_pilot.py"
CARD="$ROOT/docs/development/current/main/phases/phase-296x/3412-MIRBUILDER-SCALAR-KNOWN-FASTPATH-MAPSTORE-ANY-WRITE-HAKO-ROUTE-DECISION-AUTHORITY-PILOT-001.md"
TASK_ORDER="$ROOT/docs/development/current/main/design/mirbuilder-rust-to-hako-converter-task-order-ssot.md"
MANIFEST="$ROOT/docs/development/current/main/design/fixtures/rust-lifecycle/source-selfhost-family-guard-manifest-v0.json"
SHADOW_SOURCE="$ROOT/src/mir/generic_method_route_plan/scalar_known_hako_shadow.rs"
WRITE_ROUTES="$ROOT/src/mir/generic_method_route_plan/write_routes.rs"

guard_require_command "$TAG" python3
guard_require_command "$TAG" cargo
guard_require_files "$TAG" "$FIXTURE" "$TOOL" "$CARD" "$TASK_ORDER" "$MANIFEST" "$SHADOW_SOURCE" "$WRITE_ROUTES"
python3 "$TOOL" --check
cargo test -q scalar_known_hako_shadow

python3 - "$FIXTURE" "$CARD" "$TASK_ORDER" "$MANIFEST" "$SHADOW_SOURCE" "$WRITE_ROUTES" <<'PY'
import json, sys
from pathlib import Path
f=json.loads(Path(sys.argv[1]).read_text()); card=Path(sys.argv[2]).read_text(); task=Path(sys.argv[3]).read_text(); m=json.loads(Path(sys.argv[4]).read_text()); shadow=Path(sys.argv[5]).read_text(); routes=Path(sys.argv[6]).read_text()
def need(c,msg):
    if not c: raise SystemExit(msg)
token="MIRBUILDER-SCALAR-KNOWN-FASTPATH-MAPSTORE-ANY-WRITE-HAKO-ROUTE-DECISION-AUTHORITY-PILOT-001"; nxt="MIRBUILDER-SCALAR-KNOWN-FASTPATH-MAPSTORE-ANY-WRITE-HAKO-AUTHORITY-PILOT-RERUN-001"; fn="mapstore_any_hako_route_authority_pilot_decision"
need(f["token"]==token and token in card and nxt in card, "token drift")
need(fn in shadow and fn in routes, "live authority helper missing")
need("MapStoreAny .hako authority pilot diverged from Rust oracle" in shadow, "fail-fast missing")
s=f["summary"]
for k in ["mapstore_any_hako_route_decision_authority_pilot","mapstore_any_hako_authority_result_consumed","mapstore_any_live_route_calls_authority_pilot","mapstore_any_mismatch_fail_fast"]: need(s.get(k)==1,k)
for k in ["any_write_boundary_runtime_authority","runtime_mutation_authority","source_selfhost_claim"]: need(s.get(k)==0,k)
need(token in {r.get("token") for r in m.get("rows",[])}, "manifest missing token")
need(token in task and f"selected_next_card={nxt}" in task, "task order drift")
print("output_contract=rust-lifecycle-mirbuilder-scalar-known-fastpath-mapstore-any-write-hako-route-decision-authority-pilot")
print("mapstore_any_hako_route_decision_authority_pilot=1")
print("mapstore_any_mismatch_fail_fast=1")
print("any_write_boundary_runtime_authority=0")
print("source_selfhost_claim=0")
print("selected_next_card="+nxt)
print("summary=ok")
PY
