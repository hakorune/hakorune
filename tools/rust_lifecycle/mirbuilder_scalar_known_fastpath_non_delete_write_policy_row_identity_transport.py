#!/usr/bin/env python3
"""Record exact row identity transport for the three non-Delete Write policies."""

from __future__ import annotations

import argparse
import json
from pathlib import Path

from shared_family_generator import sha256_file, stable_json, write_if_changed


ROOT = Path(__file__).resolve().parents[2]
FIXTURES = ROOT / "docs/development/current/main/design/fixtures/rust-lifecycle"
OUTPUT = FIXTURES / "mirbuilder-scalar-known-fastpath-non-delete-write-policy-row-identity-transport-v0.json"
TOKEN = "MIRBUILDER-SCALAR-KNOWN-FASTPATH-NON-DELETE-WRITE-POLICY-ROW-IDENTITY-TRANSPORT-001"
NEXT_CARD = "MIRBUILDER-SCALAR-KNOWN-FASTPATH-WRITE-SET-MAPSTORE-I64-CALLER-ORIENTATION-LIVE-ASSERT-CONSUMER-001"
CARD = ROOT / "docs/development/current/main/phases/phase-296x/3435-MIRBUILDER-SCALAR-KNOWN-FASTPATH-NON-DELETE-WRITE-POLICY-ROW-IDENTITY-TRANSPORT-001.md"

SURFACES = [
    {
        "name": "mapstore_i64",
        "row_id": "map_store_i64_set_surface",
        "source": ROOT / "lang/src/compiler/lib/write_set_mapstore_i64_policy_classifier.hako",
        "generator": ROOT / "tools/rust_lifecycle/generate_write_set_mapstore_i64_hako_policy.py",
        "artifact": ROOT / "src/mir/generic_method_route_plan/generated/write_set_mapstore_i64_hako_policy.rs",
        "contract_fixture": FIXTURES / "mirbuilder-scalar-known-fastpath-write-set-mapstore-i64-caller-orientation-contract-artifact-v0.json",
    },
    {
        "name": "push_arrayappendany",
        "row_id": "array_append_any_push_surface",
        "source": ROOT / "lang/src/compiler/lib/write_push_surface_policy_classifier.hako",
        "generator": ROOT / "tools/rust_lifecycle/generate_write_push_hako_policy.py",
        "artifact": ROOT / "src/mir/generic_method_route_plan/generated/write_push_hako_policy.rs",
        "contract_fixture": FIXTURES / "mirbuilder-scalar-known-fastpath-write-push-arrayappendany-caller-orientation-contract-artifact-v0.json",
    },
    {
        "name": "mapstore_any",
        "row_id": "map_store_any_set_surface",
        "source": ROOT / "lang/src/compiler/lib/write_set_mapstore_any_policy_classifier.hako",
        "generator": ROOT / "tools/rust_lifecycle/generate_write_set_mapstore_any_hako_policy.py",
        "artifact": ROOT / "src/mir/generic_method_route_plan/generated/write_set_mapstore_any_hako_policy.rs",
        "contract_fixture": FIXTURES / "mirbuilder-scalar-known-fastpath-write-set-mapstore-any-caller-orientation-contract-artifact-v0.json",
    },
]


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def build_fixture() -> dict[str, object]:
    return {
        "schema_version": 0,
        "kind": "MirBuilderScalarKnownFastpathNonDeleteWritePolicyRowIdentityTransportV1",
        "token": TOKEN,
        "scope": {
            "row_ids": [item["row_id"] for item in SURFACES],
            "delete_included": False,
            "caller_contract_semantics_copied": False,
        },
        "surfaces": [
            {
                "name": item["name"],
                "policy_row_id": item["row_id"],
                "policy_source": rel(item["source"]),
                "policy_source_hash": sha256_file(item["source"]),
                "generator": rel(item["generator"]),
                "generator_hash": sha256_file(item["generator"]),
                "typed_artifact": rel(item["artifact"]),
                "typed_artifact_hash": sha256_file(item["artifact"]),
                "caller_contract_fixture": rel(item["contract_fixture"]),
                "caller_contract_fixture_hash": sha256_file(item["contract_fixture"]),
            }
            for item in SURFACES
        ],
        "decision": {
            "kind": "TransportNonDeleteWritePolicyRowIdentity",
            "next_card": NEXT_CARD,
            "card": rel(CARD),
        },
        "claims": {
            "non_delete_write_policy_row_identity_transport": 1,
            "mapstore_i64_policy_row_identity_transported": 1,
            "push_arrayappendany_policy_row_identity_transported": 1,
            "mapstore_any_policy_row_identity_transported": 1,
            "exact_three_row_set_verified": 1,
            "caller_orientation_runtime_path": 0,
            "route_selection_authority_switch": 0,
            "backend_lowering_authority": 0,
            "write_mutation_authority": 0,
            "runtime_mutation_authority": 0,
            "publication_execution": 0,
            "delete_hako_route_decision_authority_pilot": 0,
            "write_wide_authority": 0,
            "scalar_known_wide_authority": 0,
            "source_selfhost_claim": 0,
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
        print("non-delete-write policy row identity transport unchanged")
        return 0
    changed = write_if_changed(OUTPUT, output)
    print(("updated=" if changed else "unchanged=") + rel(OUTPUT))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
