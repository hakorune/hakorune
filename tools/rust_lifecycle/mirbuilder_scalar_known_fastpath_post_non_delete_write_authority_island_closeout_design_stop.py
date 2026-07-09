#!/usr/bin/env python3
"""Keep stopped after the non-Delete Write authority island closeout."""

from __future__ import annotations

import argparse
import json
from pathlib import Path
from typing import Any

from shared_family_generator import sha256_file, stable_json, write_if_changed


ROOT = Path(__file__).resolve().parents[2]
FIXTURES = ROOT / "docs/development/current/main/design/fixtures/rust-lifecycle"
OUTPUT = (
    FIXTURES
    / "mirbuilder-scalar-known-fastpath-post-non-delete-write-authority-island-closeout-design-stop-v0.json"
)

TOKEN = (
    "MIRBUILDER-SCALAR-KNOWN-FASTPATH-POST-NON-DELETE-WRITE-"
    "AUTHORITY-ISLAND-CLOSEOUT-DESIGN-STOP-001"
)
CLOSEOUT = (
    FIXTURES
    / "mirbuilder-scalar-known-fastpath-delete-retired-park-non-delete-write-authority-island-closeout-v0.json"
)


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def read_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text(encoding="utf-8"))


def build_fixture() -> dict[str, Any]:
    closeout = read_json(CLOSEOUT)
    return {
        "schema_version": 0,
        "kind": "MirBuilderScalarKnownFastpathPostNonDeleteWriteAuthorityIslandCloseoutDesignStopV1",
        "token": TOKEN,
        "input_state": {
            "non_delete_write_closeout": rel(CLOSEOUT),
            "non_delete_write_closeout_hash": sha256_file(CLOSEOUT),
            "closeout_selected_next_card": (closeout.get("decision") or {}).get("selected_next_card"),
        },
        "inventory": {
            "non_delete_write_authority_island_closed": True,
            "delete_surface_parked_as_retired_special_case": True,
            "write_surface_authority_closeout": False,
            "source_selfhost_claim": False,
            "next_options": [
                "ReturnToWiderSourceSelfhostRouteSelectionWithScopedEvidence",
                "OpenDeleteRevivalBasisIfNeeded",
                "OpenCallerOrientationAuthorityDesign",
            ],
        },
        "decision": {
            "kind": "KeepStoppedForPostNonDeleteWriteAuthorityIslandCloseoutDecision",
            "reason_token": "NonDeleteWriteIslandClosedWriteWideAndSourceSelfhostStillUnclaimed",
            "selected_next_card": None,
            "consultation_required": True,
        },
        "summary": {
            "post_non_delete_write_authority_island_closeout_design_stop": 1,
            "non_delete_write_hako_route_decision_authority_island_closeout": 1,
            "delete_surface_retired_special_case_parked": 1,
            "write_surface_authority_closeout": 0,
            "write_wide_authority": 0,
            "source_selfhost_claim": 0,
        },
        "claims": {
            "post_non_delete_write_authority_island_closeout_design_stop": 1,
            "non_delete_write_hako_route_decision_authority_island_closeout": 1,
            "delete_surface_retired_special_case_parked": 1,
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
        print("mirbuilder-scalar-known-fastpath-post-non-delete-write-authority-island-closeout-design-stop unchanged")
        return 0
    changed = write_if_changed(OUTPUT, output)
    print(("updated=" if changed else "unchanged=") + rel(OUTPUT))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
