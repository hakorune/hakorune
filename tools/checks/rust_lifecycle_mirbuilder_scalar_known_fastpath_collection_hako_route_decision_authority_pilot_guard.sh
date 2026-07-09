#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
TAG="rust-lifecycle-mirbuilder-scalar-known-fastpath-collection-hako-route-decision-authority-pilot"

source "$ROOT/tools/checks/lib/guard_common.sh"

FIXTURE="$ROOT/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-scalar-known-fastpath-collection-hako-route-decision-authority-pilot-v0.json"
TOOL="$ROOT/tools/rust_lifecycle/mirbuilder_scalar_known_fastpath_collection_hako_route_decision_authority_pilot.py"
CARD="$ROOT/docs/development/current/main/phases/phase-296x/3397-MIRBUILDER-SCALAR-KNOWN-FASTPATH-COLLECTION-HAKO-ROUTE-DECISION-AUTHORITY-PILOT-001.md"
TASK_ORDER="$ROOT/docs/development/current/main/design/mirbuilder-rust-to-hako-converter-task-order-ssot.md"
MANIFEST="$ROOT/docs/development/current/main/design/fixtures/rust-lifecycle/source-selfhost-family-guard-manifest-v0.json"
SHADOW_SOURCE="$ROOT/src/mir/generic_method_route_plan/scalar_known_hako_shadow.rs"
COLLECTION_ROUTES="$ROOT/src/mir/generic_method_route_plan/collection_read_routes.rs"

guard_require_command "$TAG" python3
guard_require_command "$TAG" cargo
guard_require_files "$TAG" "$FIXTURE" "$TOOL" "$CARD" "$TASK_ORDER" \
  "$MANIFEST" "$SHADOW_SOURCE" "$COLLECTION_ROUTES"

python3 "$TOOL" --check
cargo test -q scalar_known_hako_shadow

python3 - "$FIXTURE" "$CARD" "$TASK_ORDER" "$MANIFEST" "$SHADOW_SOURCE" "$COLLECTION_ROUTES" <<'PY'
import json
import sys
from pathlib import Path

fixture = json.loads(Path(sys.argv[1]).read_text(encoding="utf-8"))
card = Path(sys.argv[2]).read_text(encoding="utf-8")
task_order = Path(sys.argv[3]).read_text(encoding="utf-8")
manifest = json.loads(Path(sys.argv[4]).read_text(encoding="utf-8"))
shadow_source = Path(sys.argv[5]).read_text(encoding="utf-8")
collection_routes = Path(sys.argv[6]).read_text(encoding="utf-8")


def need(cond, msg):
    if not cond:
        raise SystemExit(msg)


token = "MIRBUILDER-SCALAR-KNOWN-FASTPATH-COLLECTION-HAKO-ROUTE-DECISION-AUTHORITY-PILOT-001"
next_token = "MIRBUILDER-SCALAR-KNOWN-FASTPATH-COLLECTION-HAKO-AUTHORITY-PILOT-RERUN-001"
authority_fn = "collection_scalar_i64_hako_route_authority_pilot_decision"

need(fixture.get("kind") == "MirBuilderScalarKnownFastpathCollectionHakoRouteDecisionAuthorityPilotV1", "bad kind")
need(fixture.get("token") == token, "bad token")
need(token in card, "card missing token")
need(next_token in card, "card missing next token")
need(authority_fn in shadow_source, "authority helper missing")
need(authority_fn in collection_routes, "Collection live route missing authority helper")
need("Some(receiver_domain)" in collection_routes, "Collection route must pass receiver domain")

implementation = fixture.get("implementation") or {}
need(implementation.get("surface") == "CollectionScalarI64Routes", "surface drift")
need(implementation.get("authority_function") == authority_fn, "authority function drift")
need(implementation.get("live_route_calls_authority_function") is True, "live route call drift")
need(implementation.get("rust_oracle_compat_checker") is True, "rust oracle drift")
need(implementation.get("mismatch_policy") == "FailFast", "mismatch policy drift")

shape = fixture.get("collection_shape") or {}
rows = set(shape.get("route_rows") or [])
for row in [
    "MapEntryCount:MapLen:MapBox",
    "ArraySlotLen:ArrayLen:ArrayBox",
    "StringLen:StringLen:StringBox",
    "AnyLength:AnyLen:Box",
]:
    need(row in rows, f"route row drift: {row}")
need(shape.get("any_length_box_domain_is_explicit_row_not_wildcard_selector") is True, "AnyLength Box boundary drift")

summary = fixture.get("summary") or {}
for key in [
    "collection_hako_route_decision_authority_pilot",
    "collection_hako_authority_result_consumed",
    "collection_rust_oracle_compat_checker",
    "collection_mismatch_fail_fast",
    "collection_live_route_calls_authority_pilot",
    "collection_mixed_receiver_domain_guarded",
    "collection_anylength_box_domain_guarded",
]:
    need(summary.get(key) == 1, f"summary positive drift: {key}")
for key in [
    "collection_anylength_global_box_authority",
    "any_length_wildcard_selector",
    "runtime_box_domain_fallback",
    "scalar_known_hako_runtime_route_authority",
    "source_selfhost_claim",
]:
    need(summary.get(key) == 0, f"summary forbidden drift: {key}")

claims = fixture.get("claims") or {}
for key in [
    "collection_anylength_global_box_authority",
    "receiver_domain_authority_switch",
    "receiver_domain_widening_authority",
    "receiver_domain_projection",
    "any_length_wildcard_selector",
    "runtime_box_domain_fallback",
    "read_surface_authority_closeout",
    "scalar_known_hako_runtime_route_authority",
    "rust_fastpath_rewired",
    "route_selection_authority_switch",
    "backend_lowering_authority",
    "source_selfhost_claim",
]:
    need(claims.get(key) == 0, f"forbidden claim drift: {key}")

rows_by_token = {row.get("token"): row for row in manifest.get("rows") or []}
need(token in rows_by_token, "manifest missing token")
need(token in task_order and f"selected_next_card={next_token}" in task_order, "task order drift")

print("output_contract=rust-lifecycle-mirbuilder-scalar-known-fastpath-collection-hako-route-decision-authority-pilot")
print("collection_hako_route_decision_authority_pilot=1")
print("collection_rust_oracle_compat_checker=1")
print("collection_mismatch_fail_fast=1")
print("collection_anylength_box_domain_guarded=1")
print("scalar_known_hako_runtime_route_authority=0")
print("source_selfhost_claim=0")
print("selected_next_card=" + next_token)
print("summary=ok")
PY
