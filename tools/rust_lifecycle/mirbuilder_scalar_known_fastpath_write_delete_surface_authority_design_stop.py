#!/usr/bin/env python3
"""Keep stopped for the Delete surface authority decision after taxonomy basis."""

from __future__ import annotations

import argparse
import json
from pathlib import Path
from typing import Any

from shared_family_generator import sha256_file, stable_json, write_if_changed


ROOT = Path(__file__).resolve().parents[2]
FIXTURES = ROOT / "docs/development/current/main/design/fixtures/rust-lifecycle"
OUTPUT = FIXTURES / "mirbuilder-scalar-known-fastpath-write-delete-surface-authority-design-stop-v0.json"

TOKEN = "MIRBUILDER-SCALAR-KNOWN-FASTPATH-WRITE-DELETE-SURFACE-AUTHORITY-DESIGN-STOP-001"
TAXONOMY = FIXTURES / "mirbuilder-scalar-known-authority-claim-taxonomy-basis-v0.json"
DESIGN_STOP_3414 = ROOT / "docs/development/current/main/phases/phase-296x/3414-MIRBUILDER-SCALAR-KNOWN-FASTPATH-NEXT-WRITE-HAKO-AUTHORITY-SURFACE-DESIGN-STOP-003.md"
WRITE_ROUTES = ROOT / "src/mir/generic_method_route_plan/write_routes.rs"
SHADOW_SOURCE = ROOT / "src/mir/generic_method_route_plan/scalar_known_hako_shadow.rs"


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def read_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text(encoding="utf-8"))


def build_fixture() -> dict[str, Any]:
    taxonomy = read_json(TAXONOMY)
    return {
        "schema_version": 0,
        "kind": "MirBuilderScalarKnownFastpathWriteDeleteSurfaceAuthorityDesignStopV1",
        "token": TOKEN,
        "input_state": {
            "prior_design_stop": rel(DESIGN_STOP_3414),
            "prior_design_stop_hash": sha256_file(DESIGN_STOP_3414),
            "claim_taxonomy_basis": rel(TAXONOMY),
            "claim_taxonomy_basis_hash": sha256_file(TAXONOMY),
            "taxonomy_selected_next_card": (taxonomy.get("decision") or {}).get("selected_next_card"),
        },
        "provenance": {
            "write_routes": {
                "path": rel(WRITE_ROUTES),
                "sha256": sha256_file(WRITE_ROUTES),
            },
            "shadow_consumer": {
                "path": rel(SHADOW_SOURCE),
                "sha256": sha256_file(SHADOW_SOURCE),
            },
        },
        "inventory": {
            "delete_surface": "DeleteSurfacePolicy/MapDeleteAny",
            "rust_live_route_preserved": True,
            "generated_typed_hako_artifact_exists": False,
            "hako_authority_helper_exists": False,
            "old_hako_mirror_retired": True,
            "decision_options": [
                "RestoreDeleteGeneratedTypedArtifactViaRevivalBasis",
                "ParkDeleteAsRetiredRustPreservedRoute",
                "CloseoutNonDeleteWriteAuthorityIslandOnly",
            ],
        },
        "taxonomy_application": {
            "authority.surface.delete.route_decision": 0,
            "authority.surface.write.wide": 0,
            "authority.runtime.mutation": 0,
            "authority.runtime.publication": 0,
            "authority.scalar_known.global_route": 0,
            "authority.backend.lowering": 0,
            "authority.caller_orientation.runtime_path": 0,
            "authority.source_selfhost": 0,
            "proof.forbidden.manual_selection": 0,
            "proof.forbidden.counts": 0,
            "proof.forbidden.location_or_name": 0,
        },
        "decision": {
            "kind": "KeepStoppedForDeleteSurfaceAuthorityDecision",
            "reason_token": "DeleteSurfaceRetiredMirrorNeedsRevivalOrParkDecision",
            "selected_next_card": None,
            "consultation_required": True,
        },
        "summary": {
            "delete_surface_authority_design_stop": 1,
            "claim_taxonomy_applied": 1,
            "rust_map_delete_route_preserved": 1,
            "delete_generated_typed_hako_artifact_exists": 0,
            "delete_hako_authority_helper_exists": 0,
            "delete_hako_route_decision_authority_pilot": 0,
            "mapdeleteany_authority": 0,
            "write_surface_authority_closeout": 0,
            "source_selfhost_claim": 0,
        },
        "claims": {
            "delete_surface_authority_design_stop": 1,
            "claim_taxonomy_applied": 1,
            "rust_map_delete_route_preserved": 1,
            "delete_generated_typed_hako_artifact_exists": 0,
            "delete_hako_authority_helper_exists": 0,
            "delete_hako_route_decision_authority_pilot": 0,
            "mapdeleteany_authority": 0,
            "write_surface_authority_closeout": 0,
            "write_wide_authority": 0,
            "runtime_mutation_authority": 0,
            "publication_execution": 0,
            "scalar_known_hako_runtime_route_authority": 0,
            "rust_fastpath_rewired": 0,
            "route_selection_authority_switch": 0,
            "backend_lowering_authority": 0,
            "caller_orientation_runtime_path": 0,
            "source_selfhost_claim": 0,
            "runtime_fallback": 0,
            "route_count_as_proof": 0,
            "source_path_as_authority": 0,
            "owner_name_as_proof": 0,
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
        print("mirbuilder-scalar-known-fastpath-write-delete-surface-authority-design-stop unchanged")
        return 0
    changed = write_if_changed(OUTPUT, output)
    print(("updated=" if changed else "unchanged=") + rel(OUTPUT))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
