#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
TAG="rust-lifecycle-mirbuilder-scalar-known-authority-claim-taxonomy-basis"
source "$ROOT/tools/checks/lib/guard_common.sh"

FIXTURE="$ROOT/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-scalar-known-authority-claim-taxonomy-basis-v0.json"
TOOL="$ROOT/tools/rust_lifecycle/mirbuilder_scalar_known_authority_claim_taxonomy_basis.py"
CARD="$ROOT/docs/development/current/main/phases/phase-296x/3415-MIRBUILDER-SCALAR-KNOWN-AUTHORITY-CLAIM-TAXONOMY-BASIS-001.md"
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
token="MIRBUILDER-SCALAR-KNOWN-AUTHORITY-CLAIM-TAXONOMY-BASIS-001"; nxt="MIRBUILDER-SCALAR-KNOWN-FASTPATH-WRITE-DELETE-SURFACE-AUTHORITY-DESIGN-STOP-001"
need(f["token"]==token and token in card and nxt in card, "token drift")
taxonomy=f.get("taxonomy") or {}
for key in [
    "authority.surface.delete.route_decision",
    "authority.surface.write.wide",
    "authority.runtime.mutation",
    "authority.runtime.publication",
    "authority.scalar_known.global_route",
    "authority.backend.lowering",
    "authority.caller_orientation.runtime_path",
    "authority.source_selfhost",
    "proof.forbidden.manual_selection",
    "proof.forbidden.counts",
    "proof.forbidden.location_or_name",
]:
    need(key in taxonomy, f"taxonomy key missing: {key}")
claims=f.get("claims") or {}
for k in ["authority_claim_taxonomy_basis","legacy_claim_names_preserved","new_claims_must_map_to_taxonomy"]:
    need(claims.get(k)==1, k)
for k in ["authority_semantics_changed","legacy_claims_deleted","route_authority_switch","source_selfhost_claim"]:
    need(claims.get(k)==0, k)
need(token in {r.get("token") for r in m.get("rows",[])}, "manifest missing token")
need(token in task and f"selected_next_card={nxt}" in task, "task order drift")
print("output_contract=rust-lifecycle-mirbuilder-scalar-known-authority-claim-taxonomy-basis")
print("authority_claim_taxonomy_basis=1")
print("legacy_claim_names_preserved=1")
print("new_claims_must_map_to_taxonomy=1")
print("authority_semantics_changed=0")
print("source_selfhost_claim=0")
print("selected_next_card="+nxt)
print("summary=ok")
PY
