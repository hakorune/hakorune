#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
TAG="rust-lifecycle-mirbuilder-scalar-known-fastpath-write-set-mapstore-i64-hako-route-decision-authority-pilot"

source "$ROOT/tools/checks/lib/guard_common.sh"

FIXTURE="$ROOT/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-scalar-known-fastpath-write-set-mapstore-i64-hako-route-decision-authority-pilot-v0.json"
TOOL="$ROOT/tools/rust_lifecycle/mirbuilder_scalar_known_fastpath_write_set_mapstore_i64_hako_route_decision_authority_pilot.py"
CARD="$ROOT/docs/development/current/main/phases/phase-296x/3404-MIRBUILDER-SCALAR-KNOWN-FASTPATH-WRITE-SET-MAPSTORE-I64-HAKO-ROUTE-DECISION-AUTHORITY-PILOT-001.md"
TASK_ORDER="$ROOT/docs/development/current/main/design/mirbuilder-rust-to-hako-converter-task-order-ssot.md"
MANIFEST="$ROOT/docs/development/current/main/design/fixtures/rust-lifecycle/source-selfhost-family-guard-manifest-v0.json"
SHADOW_SOURCE="$ROOT/src/mir/generic_method_route_plan/scalar_known_hako_shadow.rs"
WRITE_ROUTES="$ROOT/src/mir/generic_method_route_plan/write_routes.rs"

guard_require_command "$TAG" python3
guard_require_command "$TAG" cargo
guard_require_files "$TAG" "$FIXTURE" "$TOOL" "$CARD" "$TASK_ORDER" \
  "$MANIFEST" "$SHADOW_SOURCE" "$WRITE_ROUTES"

python3 "$TOOL" --check
cargo test -q scalar_known_hako_shadow

python3 - "$FIXTURE" "$CARD" "$TASK_ORDER" "$MANIFEST" "$SHADOW_SOURCE" "$WRITE_ROUTES" <<'PY'
import json
import sys
from pathlib import Path

fixture = json.loads(Path(sys.argv[1]).read_text(encoding="utf-8"))
card = Path(sys.argv[2]).read_text(encoding="utf-8")
task_order = Path(sys.argv[3]).read_text(encoding="utf-8")
manifest = json.loads(Path(sys.argv[4]).read_text(encoding="utf-8"))
shadow_source = Path(sys.argv[5]).read_text(encoding="utf-8")
write_routes = Path(sys.argv[6]).read_text(encoding="utf-8")


def need(cond, msg):
    if not cond:
        raise SystemExit(msg)


token = "MIRBUILDER-SCALAR-KNOWN-FASTPATH-WRITE-SET-MAPSTORE-I64-HAKO-ROUTE-DECISION-AUTHORITY-PILOT-001"
next_token = "MIRBUILDER-SCALAR-KNOWN-FASTPATH-WRITE-SET-MAPSTORE-I64-HAKO-AUTHORITY-PILOT-RERUN-001"
authority_fn = "mapstore_i64_hako_route_authority_pilot_decision"
legacy_fn = "mapstore_i64_shadow_consumed_decision"

need(fixture.get("kind") == "MirBuilderScalarKnownFastpathWriteSetMapStoreI64HakoRouteDecisionAuthorityPilotV1", "bad kind")
need(fixture.get("token") == token, "bad token")
need(token in card, "card missing token")
need(next_token in card, "card missing next token")
need(authority_fn in shadow_source, "authority helper missing")
need(legacy_fn in shadow_source, "legacy shadow wrapper missing")
need(authority_fn in write_routes, "Write live route missing authority helper")
need("WRITE_SET_MAPSTORE_I64_HAKO_POLICY" in shadow_source, "typed artifact not consumed")
need("MapStoreI64 .hako authority pilot diverged from Rust oracle" in shadow_source, "fail-fast oracle compare missing")

implementation = fixture.get("implementation") or {}
need(implementation.get("surface") == "SetSurfacePolicy/MapStoreI64", "surface drift")
need(implementation.get("authority_function") == authority_fn, "authority function drift")
need(implementation.get("live_route_calls_authority_function") is True, "live route call drift")
need(implementation.get("rust_oracle_compat_checker") is True, "rust oracle drift")
need(implementation.get("mismatch_policy") == "FailFast", "mismatch policy drift")

shape = fixture.get("write_shape") or {}
need(shape.get("route_kind") == "MapStoreI64", "route drift")
need(shape.get("value_boundary") == "ScalarI64", "value boundary drift")
need(shape.get("effect_class") == "mutate", "effect drift")
need(shape.get("mutation_class") == "MutatesReceiverOrContainer", "mutation class drift")

summary = fixture.get("summary") or {}
for key in [
    "write_set_mapstore_i64_hako_route_decision_authority_pilot",
    "write_set_mapstore_i64_hako_authority_result_consumed",
    "write_set_mapstore_i64_rust_oracle_compat_checker",
    "write_set_mapstore_i64_mismatch_fail_fast",
    "write_set_mapstore_i64_live_route_calls_authority_pilot",
]:
    need(summary.get(key) == 1, f"summary positive drift: {key}")
for key in [
    "write_surface_authority_pilot",
    "mapstore_authority",
    "runtime_mutation_authority",
    "publication_execution",
    "scalar_known_hako_runtime_route_authority",
    "source_selfhost_claim",
]:
    need(summary.get(key) == 0, f"summary forbidden drift: {key}")

claims = fixture.get("claims") or {}
for key in [
    "write_surface_authority_pilot",
    "mapstore_authority",
    "mapdelete_authority",
    "arrayappend_authority",
    "write_mutation_authority",
    "write_publication_authority",
    "runtime_mutation_authority",
    "publication_execution",
    "scalar_known_hako_runtime_route_authority",
    "rust_fastpath_rewired",
    "route_selection_authority_switch",
    "backend_lowering_authority",
    "source_selfhost_claim",
    "runtime_fallback",
    "new_backend_route",
    "new_abi",
]:
    need(claims.get(key) == 0, f"forbidden claim drift: {key}")

rows_by_token = {row.get("token"): row for row in manifest.get("rows") or []}
need(token in rows_by_token, "manifest missing token")
need(token in task_order and f"selected_next_card={next_token}" in task_order, "task order drift")

print("output_contract=rust-lifecycle-mirbuilder-scalar-known-fastpath-write-set-mapstore-i64-hako-route-decision-authority-pilot")
print("write_set_mapstore_i64_hako_route_decision_authority_pilot=1")
print("write_set_mapstore_i64_rust_oracle_compat_checker=1")
print("write_set_mapstore_i64_mismatch_fail_fast=1")
print("write_surface_authority_pilot=0")
print("scalar_known_hako_runtime_route_authority=0")
print("source_selfhost_claim=0")
print("selected_next_card=" + next_token)
print("summary=ok")
PY
