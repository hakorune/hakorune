#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
TAG="rust-lifecycle-mirbuilder-scalar-known-fastpath-read-surface-authority-closeout-rerun"

source "$ROOT/tools/checks/lib/guard_common.sh"

FIXTURE="$ROOT/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-scalar-known-fastpath-read-surface-authority-closeout-rerun-v0.json"
TOOL="$ROOT/tools/rust_lifecycle/mirbuilder_scalar_known_fastpath_read_surface_authority_closeout_rerun.py"
CARD="$ROOT/docs/development/current/main/phases/phase-296x/3401-MIRBUILDER-SCALAR-KNOWN-FASTPATH-READ-SURFACE-AUTHORITY-CLOSEOUT-RERUN-001.md"
NEXT_CARD="$ROOT/docs/development/current/main/phases/phase-296x/3402-MIRBUILDER-SCALAR-KNOWN-FASTPATH-WRITE-SURFACE-AUTHORITY-PILOT-DESIGN-STOP-001.md"
TASK_ORDER="$ROOT/docs/development/current/main/design/mirbuilder-rust-to-hako-converter-task-order-ssot.md"
MANIFEST="$ROOT/docs/development/current/main/design/fixtures/rust-lifecycle/source-selfhost-family-guard-manifest-v0.json"

guard_require_command "$TAG" python3
guard_require_files "$TAG" "$FIXTURE" "$TOOL" "$CARD" "$NEXT_CARD" "$TASK_ORDER" "$MANIFEST"

python3 "$TOOL" --check

python3 - "$FIXTURE" "$CARD" "$NEXT_CARD" "$TASK_ORDER" "$MANIFEST" <<'PY'
import json
import sys
from pathlib import Path

fixture = json.loads(Path(sys.argv[1]).read_text(encoding="utf-8"))
card = Path(sys.argv[2]).read_text(encoding="utf-8")
next_card_text = Path(sys.argv[3]).read_text(encoding="utf-8")
task_order = Path(sys.argv[4]).read_text(encoding="utf-8")
manifest = json.loads(Path(sys.argv[5]).read_text(encoding="utf-8"))


def need(cond, msg):
    if not cond:
        raise SystemExit(msg)


token = "MIRBUILDER-SCALAR-KNOWN-FASTPATH-READ-SURFACE-AUTHORITY-CLOSEOUT-RERUN-001"
next_token = "MIRBUILDER-SCALAR-KNOWN-FASTPATH-WRITE-SURFACE-AUTHORITY-PILOT-DESIGN-STOP-001"

need(fixture.get("kind") == "MirBuilderScalarKnownFastpathReadSurfaceAuthorityCloseoutRerunV1", "bad kind")
need(fixture.get("token") == token, "bad token")
need(token in card, "card missing token")
need(next_token in next_card_text, "next design stop card missing token")

summary = fixture.get("summary") or {}
for key in [
    "read_surface_authority_closeout_rerun",
    "read_surface_authority_closeout",
    "write_surface_authority_pilot_design_required",
]:
    need(summary.get(key) == 1, f"summary positive drift: {key}")
for key in [
    "write_surface_authority_pilot",
    "scalar_known_hako_runtime_route_authority",
    "source_selfhost_claim",
]:
    need(summary.get(key) == 0, f"summary forbidden drift: {key}")

decision = fixture.get("decision") or {}
need(decision.get("selected_next_card") == next_token, "decision next drift")
claims = fixture.get("claims") or {}
for key in [
    "write_surface_authority_pilot",
    "write_mutation_authority",
    "write_publication_authority",
    "scalar_known_hako_runtime_route_authority",
    "rust_fastpath_rewired",
    "route_selection_authority_switch",
    "source_selfhost_claim",
]:
    need(claims.get(key) == 0, f"forbidden claim drift: {key}")

rows_by_token = {row.get("token"): row for row in manifest.get("rows") or []}
need(token in rows_by_token, "manifest missing token")
need(token in task_order and f"selected_next_card={next_token}" in task_order, "task order drift")

print("output_contract=rust-lifecycle-mirbuilder-scalar-known-fastpath-read-surface-authority-closeout-rerun")
print("read_surface_authority_closeout_rerun=1")
print("read_surface_authority_closeout=1")
print("write_surface_authority_pilot_design_required=1")
print("write_surface_authority_pilot=0")
print("source_selfhost_claim=0")
print("selected_next_card=" + next_token)
print("summary=ok")
PY
