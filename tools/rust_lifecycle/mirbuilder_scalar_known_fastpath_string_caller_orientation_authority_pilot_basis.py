#!/usr/bin/env python3
"""Select the next caller-orientation authority pilot from closed candidates."""

from __future__ import annotations

import argparse
from pathlib import Path
from typing import Any

from shared_family_generator import sha256_file, stable_json, write_if_changed


ROOT = Path(__file__).resolve().parents[2]
FIXTURES = ROOT / "docs/development/current/main/design/fixtures/rust-lifecycle"
OUTPUT = FIXTURES / "mirbuilder-scalar-known-fastpath-string-caller-orientation-authority-pilot-basis-v0.json"
TOKEN = "MIRBUILDER-SCALAR-KNOWN-FASTPATH-STRING-CALLER-ORIENTATION-AUTHORITY-PILOT-BASIS-001"
NEXT_CARD = "MIRBUILDER-SCALAR-KNOWN-FASTPATH-STRING-CALLER-ORIENTATION-AUTHORITY-PILOT-001"

DESIGN_STOP = ROOT / "docs/development/current/main/phases/phase-296x/3443-MIRBUILDER-SCALAR-KNOWN-FASTPATH-POST-MAPLOAD-CALLER-ORIENTATION-PILOT-DESIGN-CONSULTATION-001.md"
MAPLOAD_PILOT = ROOT / "docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-scalar-known-fastpath-mapload-caller-orientation-authority-pilot-v0.json"
STRING_POLICY = ROOT / "src/mir/generic_method_route_plan/generated/string_search_scalar_i64_hako_policy.rs"
STRING_CONTRACT = ROOT / "src/mir/generic_method_route_plan/generated/string_scalar_i64_caller_orientation_contract.rs"
COLLECTION_POLICY = ROOT / "src/mir/generic_method_route_plan/generated/collection_len_scalar_i64_hako_policy.rs"
WRITE_I64_POLICY = ROOT / "src/mir/generic_method_route_plan/generated/write_set_mapstore_i64_hako_policy.rs"
WRITE_ANY_POLICY = ROOT / "src/mir/generic_method_route_plan/generated/write_set_mapstore_any_hako_policy.rs"
WRITE_PUSH_POLICY = ROOT / "src/mir/generic_method_route_plan/generated/write_push_hako_policy.rs"
DELETE_RETIRE = ROOT / "docs/development/current/main/phases/phase-296x/3353-MIRBUILDER-SCALAR-KNOWN-WRITE-DELETE-SURFACE-MIRROR-RETIRE-001.md"


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def evidence(path: Path) -> dict[str, str]:
    return {"path": rel(path), "sha256": sha256_file(path)}


def build_fixture() -> dict[str, Any]:
    return {
        "schema_version": 0,
        "kind": "MirBuilderScalarKnownFastpathStringCallerOrientationAuthorityPilotBasisV1",
        "token": TOKEN,
        "input_state": {
            "design_stop": evidence(DESIGN_STOP),
            "mapload_authority_pilot": evidence(MAPLOAD_PILOT),
        },
        "candidate_inventory": [
            {
                "axis": "A",
                "surface": "StringScalarI64Routes",
                "disposition": "selected",
                "reason": "HomogeneousScalarI64NoPublicationReadSurface",
                "evidence": [evidence(STRING_POLICY), evidence(STRING_CONTRACT)],
            },
            {
                "axis": "A",
                "surface": "CollectionScalarI64Routes",
                "disposition": "deferred",
                "reason": "MixedReceiverDomainAndAnyLengthBoxBoundary",
                "evidence": [evidence(COLLECTION_POLICY)],
            },
            {
                "axis": "B",
                "surface": "NonDeleteWrite",
                "disposition": "deferred",
                "reason": "MutationAndAnyWriteBoundary",
                "evidence": [
                    evidence(WRITE_I64_POLICY),
                    evidence(WRITE_PUSH_POLICY),
                    evidence(WRITE_ANY_POLICY),
                ],
            },
            {
                "axis": "C",
                "surface": "ScalarKnownWide",
                "disposition": "rejected_for_next_slice",
                "reason": "NoAuthorityBearingMultiSurfaceCallerOrientationProof",
                "evidence": [],
            },
            {
                "axis": "D",
                "surface": "DeleteSurfacePolicy",
                "disposition": "parked",
                "reason": "RetiredSpecialCaseRequiresSeparateRevival",
                "evidence": [evidence(DELETE_RETIRE)],
            },
            {
                "axis": "E_F",
                "surface": "SourceSelfhostOrPark",
                "disposition": "not_selected",
                "reason": "SafeReadOnlyContinuationAvailable",
                "evidence": [],
            },
        ],
        "selection": {
            "surface": "StringScalarI64Routes",
            "exhaustive_policy_row_ids": [
                "string_indexof_scalar_i64_routes",
                "string_lastindexof_scalar_i64_routes",
                "string_contains_scalar_i64_routes",
            ],
            "proof_axis": [
                "PriorScopedMapLoadCallerOrientationAuthorityContinuation",
                "HomogeneousScalarI64NoPublicationReadSurface",
                "ExistingStringHakoRouteDecisionAuthorityRetained",
                "RustOracleCompatFailFastRetained",
            ],
            "authority_scope": "policy_row_id_contract_only",
            "consumer_input": "PolicyRowIdOnly",
            "consumer_return": "Unit",
            "implementation_deferred": True,
            "selected_next_card": NEXT_CARD,
        },
        "claims": {
            "string_caller_orientation_authority_pilot_basis": 1,
            "prior_scoped_mapload_caller_orientation_authority_continuation": 1,
            "homogeneous_scalar_i64_no_publication_read_surface": 1,
            "string_exact_three_row_scope": 1,
            "string_hako_route_decision_authority_retained": 1,
            "string_rust_oracle_compat_checker_retained": 1,
            "string_mismatch_fail_fast_required": 1,
            "basis_only": 1,
            "no_new_route_authority": 1,
            "string_caller_orientation_authority_pilot": 0,
            "collection_caller_orientation_authority": 0,
            "non_delete_write_caller_orientation_authority": 0,
            "delete_hako_route_decision_authority_pilot": 0,
            "scalar_known_wide_authority": 0,
            "caller_orientation_runtime_path": 0,
            "hako_runtime_route_authority": 0,
            "route_selection_authority_switch": 0,
            "backend_lowering_authority": 0,
            "runtime_mutation_authority": 0,
            "publication_execution": 0,
            "runtime_fallback": 0,
            "source_selfhost_claim": 0,
            "route_count_as_proof": 0,
            "row_count_as_proof": 0,
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
        print("string caller-orientation authority pilot basis unchanged")
        return 0
    changed = write_if_changed(OUTPUT, output)
    print(("updated=" if changed else "unchanged=") + rel(OUTPUT))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
