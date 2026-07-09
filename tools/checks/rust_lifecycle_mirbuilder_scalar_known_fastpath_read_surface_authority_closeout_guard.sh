#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
TAG="rust-lifecycle-mirbuilder-scalar-known-fastpath-read-surface-authority-closeout"

source "$ROOT/tools/checks/lib/guard_common.sh"

FIXTURE="$ROOT/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-scalar-known-fastpath-read-surface-authority-closeout-v0.json"
TOOL="$ROOT/tools/rust_lifecycle/mirbuilder_scalar_known_fastpath_read_surface_authority_closeout.py"
CARD="$ROOT/docs/development/current/main/phases/phase-296x/3400-MIRBUILDER-SCALAR-KNOWN-FASTPATH-READ-SURFACE-AUTHORITY-CLOSEOUT-001.md"
TASK_ORDER="$ROOT/docs/development/current/main/design/mirbuilder-rust-to-hako-converter-task-order-ssot.md"
MANIFEST="$ROOT/docs/development/current/main/design/fixtures/rust-lifecycle/source-selfhost-family-guard-manifest-v0.json"

guard_require_command "$TAG" python3
guard_require_files "$TAG" "$FIXTURE" "$TOOL" "$CARD" "$TASK_ORDER" "$MANIFEST"

python3 "$TOOL" --check

python3 - "$FIXTURE" "$CARD" "$TASK_ORDER" "$MANIFEST" <<'PY'
import json
import sys
from pathlib import Path

fixture = json.loads(Path(sys.argv[1]).read_text(encoding="utf-8"))
card = Path(sys.argv[2]).read_text(encoding="utf-8")
task_order = Path(sys.argv[3]).read_text(encoding="utf-8")
manifest = json.loads(Path(sys.argv[4]).read_text(encoding="utf-8"))


def need(cond, msg):
    if not cond:
        raise SystemExit(msg)


token = "MIRBUILDER-SCALAR-KNOWN-FASTPATH-READ-SURFACE-AUTHORITY-CLOSEOUT-001"
next_token = "MIRBUILDER-SCALAR-KNOWN-FASTPATH-READ-SURFACE-AUTHORITY-CLOSEOUT-RERUN-001"

need(fixture.get("kind") == "MirBuilderScalarKnownFastpathReadSurfaceAuthorityCloseoutV1", "bad kind")
need(fixture.get("token") == token, "bad token")
need(token in card, "card missing token")
need(next_token in card, "card missing next token")

closed = fixture.get("closed_set") or {}
surfaces = set(closed.get("surfaces") or [])
for surface in [
    "MapLoadScalarI64Routes",
    "StringScalarI64Routes",
    "CollectionScalarI64Routes",
]:
    need(surface in surfaces, f"closed surface missing: {surface}")
need(closed.get("write_mutation_surface_explicitly_excluded") is True, "write exclusion drift")

summary = fixture.get("summary") or {}
for key in [
    "read_surface_authority_closeout",
    "mapload_hako_route_decision_authority_pilot",
    "string_hako_route_decision_authority_pilot",
    "collection_hako_route_decision_authority_pilot",
    "prior_scoped_hako_route_decision_authority_pilots_rerun_green",
    "generated_typed_artifact_mismatch_gate_current",
    "rust_oracle_compat_fail_fast_retained",
    "homogeneous_scalar_i64_no_publication_observe_read_surface",
    "collection_mixed_receiver_domain_guard_retained",
    "collection_anylength_box_domain_guard_retained",
    "write_mutation_surface_explicitly_excluded",
    "closeout_only",
]:
    need(summary.get(key) == 1, f"summary positive drift: {key}")
for key in ["new_authority_expansion", "source_selfhost_claim"]:
    need(summary.get(key) == 0, f"summary forbidden drift: {key}")

claims = fixture.get("claims") or {}
for key in [
    "write_surface_authority_pilot",
    "write_mutation_authority",
    "write_publication_authority",
    "mapstore_authority",
    "mapdelete_authority",
    "arrayappend_authority",
    "scalar_known_hako_runtime_route_authority",
    "scalar_known_transport_axis_authority_switch",
    "rust_fastpath_rewired",
    "route_selection_authority_switch",
    "backend_lowering_authority",
    "runtime_mutation_authority",
    "publication_execution",
    "caller_orientation_runtime_path",
    "source_selfhost_claim",
    "source_selfhost_route_selection",
    "wider_source_route_authority",
    "backend_authority",
    "new_backend_route",
    "new_abi",
    "runtime_fallback",
    "route_count_as_proof",
    "row_count_as_proof",
    "coverage_percentage_as_proof",
    "owner_name_as_proof",
    "source_path_as_authority",
    "route_membership_alone_as_proof",
    "manual_surface_selection",
]:
    need(claims.get(key) == 0, f"forbidden claim drift: {key}")

rows_by_token = {row.get("token"): row for row in manifest.get("rows") or []}
need(token in rows_by_token, "manifest missing token")
need(token in task_order and f"selected_next_card={next_token}" in task_order, "task order drift")

print("output_contract=rust-lifecycle-mirbuilder-scalar-known-fastpath-read-surface-authority-closeout")
print("read_surface_authority_closeout=1")
print("closeout_only=1")
print("new_authority_expansion=0")
print("write_surface_authority_pilot=0")
print("source_selfhost_claim=0")
print("selected_next_card=" + next_token)
print("summary=ok")
PY
