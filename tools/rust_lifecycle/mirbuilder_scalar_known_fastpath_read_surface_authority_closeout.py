#!/usr/bin/env python3
"""Close out scoped `.hako` route-decision authority for read surfaces."""

from __future__ import annotations

import argparse
import json
from pathlib import Path
from typing import Any

from shared_family_generator import sha256_file, stable_json, write_if_changed


ROOT = Path(__file__).resolve().parents[2]
FIXTURES = ROOT / "docs/development/current/main/design/fixtures/rust-lifecycle"
OUTPUT = FIXTURES / "mirbuilder-scalar-known-fastpath-read-surface-authority-closeout-v0.json"

TOKEN = "MIRBUILDER-SCALAR-KNOWN-FASTPATH-READ-SURFACE-AUTHORITY-CLOSEOUT-001"
NEXT_CARD = "MIRBUILDER-SCALAR-KNOWN-FASTPATH-READ-SURFACE-AUTHORITY-CLOSEOUT-RERUN-001"

RERUN = FIXTURES / "mirbuilder-scalar-known-fastpath-collection-hako-authority-pilot-rerun-v0.json"
MAPLOAD_FIXTURE = FIXTURES / "mirbuilder-scalar-known-fastpath-mapload-hako-route-decision-authority-pilot-v0.json"
STRING_FIXTURE = FIXTURES / "mirbuilder-scalar-known-fastpath-string-hako-route-decision-authority-pilot-v0.json"
COLLECTION_FIXTURE = (
    FIXTURES
    / "mirbuilder-scalar-known-fastpath-collection-hako-route-decision-authority-pilot-v0.json"
)


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def read_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text(encoding="utf-8"))


def input_fixture(path: Path) -> dict[str, Any]:
    data = read_json(path)
    return {
        "path": rel(path),
        "sha256": sha256_file(path),
        "token": data.get("token"),
    }


def build_fixture() -> dict[str, Any]:
    rerun = read_json(RERUN)
    return {
        "schema_version": 0,
        "kind": "MirBuilderScalarKnownFastpathReadSurfaceAuthorityCloseoutV1",
        "token": TOKEN,
        "input_state": {
            "rerun_fixture": rel(RERUN),
            "rerun_fixture_hash": sha256_file(RERUN),
            "rerun_selected_next_card": (rerun.get("decision") or {}).get("selected_next_card"),
            "read_surface_authority_closeout_design_required": (rerun.get("summary") or {}).get(
                "read_surface_authority_closeout_design_required"
            ),
        },
        "closed_set": {
            "surfaces": [
                "MapLoadScalarI64Routes",
                "StringScalarI64Routes",
                "CollectionScalarI64Routes",
            ],
            "surface_set_id": (
                "MapLoadScalarI64Routes_StringScalarI64Routes_"
                "CollectionScalarI64Routes"
            ),
            "closed_enumerated_read_authority_surface_set": True,
            "write_mutation_surface_explicitly_excluded": True,
        },
        "input_fixtures": {
            "mapload_authority_pilot": input_fixture(MAPLOAD_FIXTURE),
            "string_authority_pilot": input_fixture(STRING_FIXTURE),
            "collection_authority_pilot": input_fixture(COLLECTION_FIXTURE),
        },
        "proof_axis": {
            "closed_enumerated_read_authority_surface_set": True,
            "prior_scoped_hako_route_decision_authority_pilots_rerun_green": True,
            "homogeneous_scalar_i64_no_publication_observe_read_surface": True,
            "generated_typed_artifact_mismatch_gate_current": True,
            "rust_oracle_compat_fail_fast_retained": True,
            "collection_mixed_receiver_domain_guard_retained": True,
            "collection_anylength_box_domain_guard_retained": True,
            "write_mutation_surface_explicitly_excluded": True,
        },
        "decision": {
            "kind": "SelectReadSurfaceAuthorityCloseoutRerun",
            "reason_token": "ReadSurfaceAuthorityIslandClosedNoNewAuthorityExpansion",
            "selected_next_card": NEXT_CARD,
        },
        "summary": {
            "read_surface_authority_closeout": 1,
            "closed_read_surface_set": (
                "MapLoadScalarI64Routes_StringScalarI64Routes_"
                "CollectionScalarI64Routes"
            ),
            "mapload_hako_route_decision_authority_pilot": 1,
            "string_hako_route_decision_authority_pilot": 1,
            "collection_hako_route_decision_authority_pilot": 1,
            "prior_scoped_hako_route_decision_authority_pilots_rerun_green": 1,
            "generated_typed_artifact_mismatch_gate_current": 1,
            "rust_oracle_compat_fail_fast_retained": 1,
            "homogeneous_scalar_i64_no_publication_observe_read_surface": 1,
            "collection_mixed_receiver_domain_guard_retained": 1,
            "collection_anylength_box_domain_guard_retained": 1,
            "write_mutation_surface_explicitly_excluded": 1,
            "closeout_only": 1,
            "new_authority_expansion": 0,
            "source_selfhost_claim": 0,
        },
        "claims": {
            "read_surface_authority_closeout": 1,
            "mapload_hako_route_decision_authority_pilot": 1,
            "string_hako_route_decision_authority_pilot": 1,
            "collection_hako_route_decision_authority_pilot": 1,
            "prior_scoped_hako_route_decision_authority_pilots_rerun_green": 1,
            "generated_typed_artifact_mismatch_gate_current": 1,
            "rust_oracle_compat_fail_fast_retained": 1,
            "homogeneous_scalar_i64_no_publication_observe_read_surface": 1,
            "collection_mixed_receiver_domain_guard_retained": 1,
            "collection_anylength_box_domain_guard_retained": 1,
            "write_mutation_surface_explicitly_excluded": 1,
            "closeout_only": 1,
            "new_authority_expansion": 0,
            "write_surface_authority_pilot": 0,
            "write_mutation_authority": 0,
            "write_publication_authority": 0,
            "mapstore_authority": 0,
            "mapdelete_authority": 0,
            "arrayappend_authority": 0,
            "scalar_known_hako_runtime_route_authority": 0,
            "scalar_known_transport_axis_authority_switch": 0,
            "rust_fastpath_rewired": 0,
            "route_selection_authority_switch": 0,
            "backend_lowering_authority": 0,
            "runtime_mutation_authority": 0,
            "publication_execution": 0,
            "caller_orientation_runtime_path": 0,
            "source_selfhost_claim": 0,
            "source_selfhost_route_selection": 0,
            "wider_source_route_authority": 0,
            "backend_authority": 0,
            "new_backend_route": 0,
            "new_abi": 0,
            "runtime_fallback": 0,
            "route_count_as_proof": 0,
            "row_count_as_proof": 0,
            "coverage_percentage_as_proof": 0,
            "owner_name_as_proof": 0,
            "source_path_as_authority": 0,
            "route_membership_alone_as_proof": 0,
            "manual_surface_selection": 0,
        },
    }


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--check", action="store_true", help="Verify checked-in fixture.")
    args = parser.parse_args()

    output = stable_json(build_fixture())
    if args.check:
        if not OUTPUT.exists() or OUTPUT.read_text(encoding="utf-8") != output:
            raise SystemExit(f"{rel(OUTPUT)} is stale; rerun without --check")
        print("mirbuilder-scalar-known-fastpath-read-surface-authority-closeout unchanged")
        return 0

    changed = write_if_changed(OUTPUT, output)
    print(("updated=" if changed else "unchanged=") + rel(OUTPUT))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
