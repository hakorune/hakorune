#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
TAG="rust-lifecycle-mirbuilder-scalar-known-fastpath-push-write-hako-authority-pilot-rerun"

source "$ROOT/tools/checks/lib/guard_common.sh"

FIXTURE="$ROOT/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-scalar-known-fastpath-push-write-hako-authority-pilot-rerun-v0.json"
TOOL="$ROOT/tools/rust_lifecycle/mirbuilder_scalar_known_fastpath_push_write_hako_authority_pilot_rerun.py"
CARD="$ROOT/docs/development/current/main/phases/phase-296x/3409-MIRBUILDER-SCALAR-KNOWN-FASTPATH-PUSH-WRITE-HAKO-AUTHORITY-PILOT-RERUN-001.md"
TASK_ORDER="$ROOT/docs/development/current/main/design/mirbuilder-rust-to-hako-converter-task-order-ssot.md"
MANIFEST="$ROOT/docs/development/current/main/design/fixtures/rust-lifecycle/source-selfhost-family-guard-manifest-v0.json"

guard_require_command "$TAG" python3
guard_require_files "$TAG" "$FIXTURE" "$TOOL" "$CARD" "$TASK_ORDER" "$MANIFEST"

python3 "$TOOL" --check

python3 - "$FIXTURE" "$CARD" "$TASK_ORDER" "$MANIFEST" <<'PY'
import json, sys
from pathlib import Path

fixture = json.loads(Path(sys.argv[1]).read_text(encoding="utf-8"))
card = Path(sys.argv[2]).read_text(encoding="utf-8")
task_order = Path(sys.argv[3]).read_text(encoding="utf-8")
manifest = json.loads(Path(sys.argv[4]).read_text(encoding="utf-8"))

def need(cond, msg):
    if not cond:
        raise SystemExit(msg)

token = "MIRBUILDER-SCALAR-KNOWN-FASTPATH-PUSH-WRITE-HAKO-AUTHORITY-PILOT-RERUN-001"
next_token = "MIRBUILDER-SCALAR-KNOWN-FASTPATH-NEXT-WRITE-HAKO-AUTHORITY-SURFACE-DESIGN-STOP-002"
need(fixture.get("kind") == "MirBuilderScalarKnownFastpathPushWriteHakoAuthorityPilotRerunV1", "bad kind")
need(fixture.get("token") == token, "bad token")
need(token in card and next_token in card, "card token drift")
summary = fixture.get("summary") or {}
for key in ["push_write_hako_authority_pilot_rerun", "push_hako_route_decision_authority_pilot", "push_no_any_write_boundary_opened", "next_write_authority_surface_design_required"]:
    need(summary.get(key) == 1, f"summary positive drift: {key}")
for key in ["runtime_mutation_authority", "publication_execution", "source_selfhost_claim"]:
    need(summary.get(key) == 0, f"summary forbidden drift: {key}")
need(token in {row.get("token") for row in manifest.get("rows") or []}, "manifest missing token")
need(token in task_order and f"selected_next_card={next_token}" in task_order, "task order drift")
print("output_contract=rust-lifecycle-mirbuilder-scalar-known-fastpath-push-write-hako-authority-pilot-rerun")
print("push_write_hako_authority_pilot_rerun=1")
print("next_write_authority_surface_design_required=1")
print("source_selfhost_claim=0")
print("selected_next_card=" + next_token)
print("summary=ok")
PY
