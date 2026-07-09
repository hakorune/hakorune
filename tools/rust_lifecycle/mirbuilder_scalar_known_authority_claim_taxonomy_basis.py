#!/usr/bin/env python3
"""Define the ScalarKnown authority claim taxonomy basis."""

from __future__ import annotations

import argparse
import json
from pathlib import Path
from typing import Any

from shared_family_generator import sha256_file, stable_json, write_if_changed


ROOT = Path(__file__).resolve().parents[2]
FIXTURES = ROOT / "docs/development/current/main/design/fixtures/rust-lifecycle"
OUTPUT = FIXTURES / "mirbuilder-scalar-known-authority-claim-taxonomy-basis-v0.json"

TOKEN = "MIRBUILDER-SCALAR-KNOWN-AUTHORITY-CLAIM-TAXONOMY-BASIS-001"
NEXT_CARD = "MIRBUILDER-SCALAR-KNOWN-FASTPATH-WRITE-DELETE-SURFACE-AUTHORITY-DESIGN-STOP-001"

DESIGN_STOP = ROOT / "docs/development/current/main/phases/phase-296x/3414-MIRBUILDER-SCALAR-KNOWN-FASTPATH-NEXT-WRITE-HAKO-AUTHORITY-SURFACE-DESIGN-STOP-003.md"
TASK_ORDER = ROOT / "docs/development/current/main/design/mirbuilder-rust-to-hako-converter-task-order-ssot.md"


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def build_fixture() -> dict[str, Any]:
    return {
        "schema_version": 0,
        "kind": "MirBuilderScalarKnownAuthorityClaimTaxonomyBasisV1",
        "token": TOKEN,
        "input_state": {
            "design_stop_card": rel(DESIGN_STOP),
            "design_stop_card_hash": sha256_file(DESIGN_STOP),
            "task_order": rel(TASK_ORDER),
            "task_order_hash": sha256_file(TASK_ORDER),
        },
        "taxonomy": {
            "authority.surface.delete.route_decision": [
                "delete_hako_route_decision_authority_pilot",
                "mapdeleteany_authority",
            ],
            "authority.surface.write.wide": [
                "write_surface_authority_closeout",
                "write_wide_authority",
            ],
            "authority.runtime.mutation": [
                "runtime_mutation_authority",
            ],
            "authority.runtime.publication": [
                "publication_execution",
            ],
            "authority.scalar_known.global_route": [
                "scalar_known_hako_runtime_route_authority",
                "scalar_known_transport_axis_authority_switch",
            ],
            "authority.backend.lowering": [
                "backend_lowering_authority",
                "new_backend_route",
                "new_abi",
            ],
            "authority.caller_orientation.runtime_path": [
                "caller_orientation_runtime_path",
                "rust_fastpath_rewired",
                "route_selection_authority_switch",
            ],
            "authority.source_selfhost": [
                "source_selfhost_claim",
            ],
            "proof.forbidden.manual_selection": [
                "manual_surface_selection",
                "manual_subsurface_selection",
                "manual_axis_selection",
                "manual_carrier_selection",
            ],
            "proof.forbidden.counts": [
                "route_count_as_proof",
                "row_count_as_proof",
                "coverage_percentage_as_proof",
            ],
            "proof.forbidden.location_or_name": [
                "source_path_as_authority",
                "owner_name_as_proof",
                "route_membership_alone_as_proof",
            ],
        },
        "rules": {
            "basis_only": True,
            "legacy_claim_names_preserved": True,
            "new_claims_must_map_to_taxonomy": True,
            "taxonomy_is_documentation_layer_only": True,
            "legacy_claim_deletion_allowed": False,
            "authority_semantics_changed": False,
            "route_authority_switch_allowed": False,
        },
        "decision": {
            "kind": "SelectDeleteSurfaceAuthorityDesignStopWithTaxonomy",
            "reason_token": "ClaimTaxonomyBasisDefinedBeforeDeleteSurfaceDecision",
            "selected_next_card": NEXT_CARD,
        },
        "summary": {
            "authority_claim_taxonomy_basis": 1,
            "legacy_claim_names_preserved": 1,
            "new_claims_must_map_to_taxonomy": 1,
            "taxonomy_is_documentation_layer_only": 1,
            "authority_semantics_changed": 0,
            "legacy_claims_deleted": 0,
            "route_authority_switch": 0,
            "source_selfhost_claim": 0,
        },
        "claims": {
            "authority_claim_taxonomy_basis": 1,
            "legacy_claim_names_preserved": 1,
            "new_claims_must_map_to_taxonomy": 1,
            "taxonomy_is_documentation_layer_only": 1,
            "authority_semantics_changed": 0,
            "legacy_claims_deleted": 0,
            "route_authority_switch": 0,
            "delete_hako_route_decision_authority_pilot": 0,
            "mapdeleteany_authority": 0,
            "write_surface_authority_closeout": 0,
            "write_wide_authority": 0,
            "runtime_mutation_authority": 0,
            "publication_execution": 0,
            "scalar_known_hako_runtime_route_authority": 0,
            "backend_lowering_authority": 0,
            "caller_orientation_runtime_path": 0,
            "source_selfhost_claim": 0,
            "runtime_fallback": 0,
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
        print("mirbuilder-scalar-known-authority-claim-taxonomy-basis unchanged")
        return 0
    changed = write_if_changed(OUTPUT, output)
    print(("updated=" if changed else "unchanged=") + rel(OUTPUT))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
