#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
TAG="rust-lifecycle-mirbuilder-scalar-known-fastpath-mapload-narrow-caller-orientation-basis"
source "$ROOT/tools/checks/lib/guard_common.sh"

FIXTURE="$ROOT/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-scalar-known-fastpath-mapload-narrow-caller-orientation-basis-v0.json"
TOOL="$ROOT/tools/rust_lifecycle/mirbuilder_scalar_known_fastpath_mapload_narrow_caller_orientation_basis.py"
CARD="$ROOT/docs/development/current/main/phases/phase-296x/3420-MIRBUILDER-SCALAR-KNOWN-FASTPATH-MAPLOAD-NARROW-CALLER-ORIENTATION-BASIS-001.md"
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


def need(condition, message):
    if not condition:
        raise SystemExit(message)


token = "MIRBUILDER-SCALAR-KNOWN-FASTPATH-MAPLOAD-NARROW-CALLER-ORIENTATION-BASIS-001"
need(fixture.get("token") == token, "bad token")
need(token in card and token in task_order, "token pointer drift")
need(manifest.get("current_blocker_token") == token, "manifest current blocker drift")
need(token in {row.get("token") for row in manifest.get("rows", [])}, "manifest missing token")

basis = fixture.get("basis") or {}
for key, value in {
    "surface": "MapLoadScalarI64Routes",
    "route_kind": "MapLoadScalarI64",
    "scope": "single_surface",
    "orientation_kind": "CallerOrientationContractMetadataOnly",
    "mismatch_policy": "FailFast",
    "effect_class": "read",
    "publication_policy": "NoPublication",
}.items():
    need(basis.get(key) == value, f"basis drift: {key}")
for key in ["runtime_consumer", "backend_lowering_consumer", "mutation_consumer", "publication_consumer"]:
    need(basis.get(key) is False, f"forbidden consumer drift: {key}")

decision = fixture.get("decision") or {}
need(decision.get("kind") == "AdoptMapLoadNarrowCallerOrientationBasisOnly", "decision drift")
need(decision.get("implementation_deferred") is True, "implementation must remain deferred")

claims = fixture.get("claims") or {}
for key in [
    "mapload_caller_orientation_basis",
    "mapload_hako_route_decision_authority_retained",
    "mapload_rust_oracle_compat_checker_retained",
    "mapload_mismatch_fail_fast",
    "basis_only",
    "mapload_single_surface_scope",
    "caller_orientation_implementation_deferred",
    "caller_orientation_contract_metadata_only",
    "no_new_route_authority",
    "prior_scoped_mapload_hako_route_decision_authority",
    "single_surface_mapload_caller_orientation_scope",
    "rust_oracle_compat_fail_fast_retained",
    "no_runtime_path_no_backend_lowering_no_mutation_no_publication",
]:
    need(claims.get(key) == 1, f"positive claim drift: {key}")
for key in [
    "caller_orientation_runtime_path",
    "hako_runtime_route_authority",
    "scalar_known_hako_runtime_route_authority",
    "rust_fastpath_rewired",
    "backend_lowering_authority",
    "runtime_mutation_authority",
    "publication_execution",
    "source_selfhost_claim",
    "delete_hako_route_decision_authority_pilot",
    "caller_selected_route_authority",
    "caller_runtime_dispatch_authority",
    "caller_orientation_result_consumed_by_runtime",
    "caller_orientation_result_consumed_by_backend",
    "route_selection_authority_switch",
    "mapload_to_scalar_known_wide_authority",
    "read_surface_to_runtime_authority",
    "write_surface_authority_closeout",
    "write_wide_authority",
    "runtime_fallback",
    "new_backend_route",
    "new_abi",
    "route_count_as_proof",
    "row_count_as_proof",
    "coverage_percentage_as_proof",
    "source_path_as_authority",
    "owner_name_as_proof",
    "route_membership_alone_as_proof",
    "manual_surface_selection",
]:
    need(claims.get(key) == 0, f"non-claim drift: {key}")

print("output_contract=rust-lifecycle-mirbuilder-scalar-known-fastpath-mapload-narrow-caller-orientation-basis")
print("mapload_caller_orientation_basis=1")
print("caller_orientation_contract_metadata_only=1")
print("caller_orientation_runtime_path=0")
print("source_selfhost_claim=0")
print("summary=ok")
PY
