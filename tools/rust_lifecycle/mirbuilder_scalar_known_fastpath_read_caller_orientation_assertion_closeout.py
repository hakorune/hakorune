#!/usr/bin/env python3
"""Record the checked-in read caller-orientation assertion closeout."""

from __future__ import annotations

import argparse
import json
from pathlib import Path
from typing import Any

from shared_family_generator import sha256_file, stable_json, write_if_changed


ROOT = Path(__file__).resolve().parents[2]
FIXTURES = ROOT / "docs/development/current/main/design/fixtures/rust-lifecycle"
OUTPUT = FIXTURES / "mirbuilder-scalar-known-fastpath-read-caller-orientation-assertion-closeout-v0.json"
TOKEN = "MIRBUILDER-SCALAR-KNOWN-FASTPATH-READ-CALLER-ORIENTATION-ASSERTION-CLOSEOUT-001"
NEXT_CARD = "MIRBUILDER-SCALAR-KNOWN-FASTPATH-POST-READ-CALLER-ORIENTATION-DESIGN-STOP-001"
MODULE = ROOT / "src/mir/generic_method_route_plan/caller_orientation.rs"
SHADOW = ROOT / "src/mir/generic_method_route_plan/scalar_known_hako_shadow.rs"

SURFACES = [
    ("MapLoad", "map_load_scalar_i64_routes", "MapLoadScalarI64Routes"),
    ("String", "string_indexof_scalar_i64_routes", "StringScalarI64Routes"),
    ("String", "string_lastindexof_scalar_i64_routes", "StringScalarI64Routes"),
    ("String", "string_contains_scalar_i64_routes", "StringScalarI64Routes"),
    ("Collection", "collection_map_entry_count_scalar_i64_routes", "CollectionScalarI64Routes"),
    ("Collection", "collection_array_slot_len_scalar_i64_routes", "CollectionScalarI64Routes"),
    ("Collection", "collection_string_len_scalar_i64_routes", "CollectionScalarI64Routes"),
    ("Collection", "collection_any_length_scalar_i64_routes", "CollectionScalarI64Routes"),
]


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def build_fixture() -> dict[str, Any]:
    return {
        "schema_version": 0,
        "kind": "MirBuilderScalarKnownFastpathReadCallerOrientationAssertionCloseoutV1",
        "token": TOKEN,
        "rows": [
            {"surface": surface, "policy_row_id": row_id, "policy_surface": policy_surface, "consumer": "assertion_only"}
            for surface, row_id, policy_surface in SURFACES
        ],
        "provenance": {
            "caller_orientation_module": rel(MODULE),
            "caller_orientation_module_hash": sha256_file(MODULE),
            "shadow_route_module": rel(SHADOW),
            "shadow_route_module_hash": sha256_file(SHADOW),
        },
        "decision": {
            "kind": "CloseoutReadCallerOrientationAssertionCoverage",
            "live_consumer_kind": "CompilerSideFailFastAssertion",
            "consumer_input": "PolicyRowIdOnly",
            "consumer_return": "Unit",
            "implementation_complete": True,
            "selected_next_card": NEXT_CARD,
        },
        "claims": {
            "read_caller_orientation_assertion_closeout": 1,
            "all_eight_read_rows_live_asserted": 1,
            "mapload_live_assertion": 1,
            "string_three_row_live_assertion": 1,
            "collection_four_row_live_assertion": 1,
            "assertion_only": 1,
            "caller_orientation_runtime_path": 0,
            "caller_runtime_dispatch_authority": 0,
            "route_selection_authority_switch": 0,
            "hako_runtime_route_authority": 0,
            "scalar_known_hako_runtime_route_authority": 0,
            "receiver_domain_authority_switch": 0,
            "receiver_domain_widening_authority": 0,
            "any_length_wildcard_selector": 0,
            "runtime_box_domain_fallback": 0,
            "backend_lowering_authority": 0,
            "runtime_mutation_authority": 0,
            "publication_execution": 0,
            "write_caller_orientation_contract": 0,
            "delete_hako_route_decision_authority_pilot": 0,
            "scalar_known_wide_authority": 0,
            "new_backend_route": 0,
            "new_abi": 0,
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
        print("mirbuilder-scalar-known-fastpath-read-caller-orientation-assertion-closeout unchanged")
        return 0
    changed = write_if_changed(OUTPUT, output)
    print(("updated=" if changed else "unchanged=") + rel(OUTPUT))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
