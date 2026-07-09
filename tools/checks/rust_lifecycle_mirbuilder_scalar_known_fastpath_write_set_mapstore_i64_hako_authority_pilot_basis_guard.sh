#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
TAG="rust-lifecycle-mirbuilder-scalar-known-fastpath-write-set-mapstore-i64-hako-authority-pilot-basis"

source "$ROOT/tools/checks/lib/guard_common.sh"

FIXTURE="$ROOT/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-scalar-known-fastpath-write-set-mapstore-i64-hako-authority-pilot-basis-v0.json"
TOOL="$ROOT/tools/rust_lifecycle/mirbuilder_scalar_known_fastpath_write_set_mapstore_i64_hako_authority_pilot_basis.py"
CARD="$ROOT/docs/development/current/main/phases/phase-296x/3403-MIRBUILDER-SCALAR-KNOWN-FASTPATH-WRITE-SET-MAPSTORE-I64-HAKO-AUTHORITY-PILOT-BASIS-001.md"
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


token = "MIRBUILDER-SCALAR-KNOWN-FASTPATH-WRITE-SET-MAPSTORE-I64-HAKO-AUTHORITY-PILOT-BASIS-001"
next_token = "MIRBUILDER-SCALAR-KNOWN-FASTPATH-WRITE-SET-MAPSTORE-I64-HAKO-ROUTE-DECISION-AUTHORITY-PILOT-001"

need(fixture.get("kind") == "MirBuilderScalarKnownFastpathWriteSetMapStoreI64HakoAuthorityPilotBasisV1", "bad kind")
need(fixture.get("token") == token, "bad token")
need(token in card, "card missing token")
need(next_token in card, "card missing next token")

basis = fixture.get("basis") or {}
need(basis.get("basis_only") is True, "basis must be basis-only")
need(basis.get("surface") == "SetSurfacePolicy/MapStoreI64", "wrong surface")
need(basis.get("authority_source") == "WRITE_SET_MAPSTORE_I64_HAKO_POLICY", "authority source drift")
for axis in [
    "ReadSurfaceAuthorityCloseoutPrecedesWriteAuthority",
    "TypedScalarWriteBeforeAnyWrite",
    "PriorGeneratedTypedArtifactShadowConsumed",
    "RustOracleCompatFailFastRetained",
]:
    need(axis in (basis.get("proof_axis") or []), f"proof axis missing: {axis}")
need(basis.get("selected_next_card") == next_token, "basis next drift")

shape = fixture.get("write_shape") or {}
need(shape.get("route_kind") == "MapStoreI64", "route kind drift")
need(shape.get("value_boundary") == "ScalarI64", "value boundary drift")
need(shape.get("effect_class") == "mutate", "effect drift")

summary = fixture.get("summary") or {}
for key in [
    "write_set_mapstore_i64_hako_authority_pilot_basis",
    "typed_scalar_write_before_any_write",
    "prior_generated_typed_artifact_shadow_consumed",
    "rust_oracle_compat_fail_fast_retained",
    "basis_only",
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
    "route_count_as_proof",
    "source_path_as_authority",
    "owner_name_as_proof",
    "route_membership_alone_as_proof",
    "manual_surface_selection",
]:
    need(claims.get(key) == 0, f"forbidden claim drift: {key}")

rows_by_token = {row.get("token"): row for row in manifest.get("rows") or []}
need(token in rows_by_token, "manifest missing token")
need(token in task_order and f"selected_next_card={next_token}" in task_order, "task order drift")

print("output_contract=rust-lifecycle-mirbuilder-scalar-known-fastpath-write-set-mapstore-i64-hako-authority-pilot-basis")
print("write_set_mapstore_i64_hako_authority_pilot_basis=1")
print("typed_scalar_write_before_any_write=1")
print("write_surface_authority_pilot=0")
print("mapstore_authority=0")
print("source_selfhost_claim=0")
print("selected_next_card=" + next_token)
print("summary=ok")
PY
