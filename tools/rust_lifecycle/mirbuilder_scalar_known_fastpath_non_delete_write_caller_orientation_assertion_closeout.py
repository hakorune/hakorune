#!/usr/bin/env python3
"""Record the checked-in non-Delete Write caller-orientation assertion closeout."""

from __future__ import annotations

import argparse
import json
from pathlib import Path
from typing import Any

from shared_family_generator import sha256_file, stable_json, write_if_changed


ROOT = Path(__file__).resolve().parents[2]
FIXTURES = ROOT / "docs/development/current/main/design/fixtures/rust-lifecycle"
OUTPUT = FIXTURES / "mirbuilder-scalar-known-fastpath-non-delete-write-caller-orientation-assertion-closeout-v0.json"
TOKEN = "MIRBUILDER-SCALAR-KNOWN-FASTPATH-NON-DELETE-WRITE-CALLER-ORIENTATION-ASSERTION-CLOSEOUT-001"
NEXT_CARD = "MIRBUILDER-SCALAR-KNOWN-FASTPATH-POST-NON-DELETE-WRITE-CALLER-ORIENTATION-DESIGN-CONSULTATION-001"
MODULE = ROOT / "src/mir/generic_method_route_plan/caller_orientation.rs"
SHADOW = ROOT / "src/mir/generic_method_route_plan/scalar_known_hako_shadow.rs"

ROWS = [
    ("MapStoreI64", "map_store_i64_set_surface", "SetSurfacePolicy"),
    ("ArrayAppendAny", "array_append_any_push_surface", "PushSurfacePolicy"),
    ("MapStoreAny", "map_store_any_set_surface", "SetSurfacePolicy/MapStoreAny"),
]


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def build_fixture() -> dict[str, Any]:
    return {
        "schema_version": 0,
        "kind": "MirBuilderScalarKnownFastpathNonDeleteWriteCallerOrientationAssertionCloseoutV1",
        "token": TOKEN,
        "rows": [
            {
                "surface": surface,
                "policy_row_id": row_id,
                "policy_surface": policy_surface,
                "consumer": "assertion_only",
            }
            for surface, row_id, policy_surface in ROWS
        ],
        "provenance": {
            "caller_orientation_module": rel(MODULE),
            "caller_orientation_module_hash": sha256_file(MODULE),
            "shadow_route_module": rel(SHADOW),
            "shadow_route_module_hash": sha256_file(SHADOW),
        },
        "decision": {
            "kind": "CloseoutNonDeleteWriteCallerOrientationAssertionCoverage",
            "live_consumer_kind": "CompilerSideFailFastAssertion",
            "consumer_input": "PolicyRowIdOnly",
            "consumer_return": "Unit",
            "implementation_complete": True,
            "selected_next_card": NEXT_CARD,
        },
        "claims": {
            "non_delete_write_caller_orientation_assertion_closeout": 1,
            "all_three_non_delete_write_rows_live_asserted": 1,
            "mapstore_i64_live_assertion": 1,
            "push_arrayappendany_live_assertion": 1,
            "mapstore_any_live_assertion": 1,
            "assertion_only": 1,
            "caller_orientation_runtime_path": 0,
            "caller_runtime_dispatch_authority": 0,
            "route_selection_authority_switch": 0,
            "hako_runtime_route_authority": 0,
            "scalar_known_hako_runtime_route_authority": 0,
            "backend_lowering_authority": 0,
            "write_mutation_authority": 0,
            "runtime_mutation_authority": 0,
            "publication_execution": 0,
            "delete_hako_route_decision_authority_pilot": 0,
            "write_wide_authority": 0,
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
        print("non-delete-write caller-orientation assertion closeout unchanged")
        return 0
    changed = write_if_changed(OUTPUT, output)
    print(("updated=" if changed else "unchanged=") + rel(OUTPUT))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
