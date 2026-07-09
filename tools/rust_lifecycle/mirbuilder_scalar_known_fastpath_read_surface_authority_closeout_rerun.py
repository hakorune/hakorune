#!/usr/bin/env python3
"""Rerun after the ScalarKnown read-surface authority closeout."""

from __future__ import annotations

import argparse
import json
from pathlib import Path
from typing import Any

from shared_family_generator import sha256_file, stable_json, write_if_changed


ROOT = Path(__file__).resolve().parents[2]
FIXTURES = ROOT / "docs/development/current/main/design/fixtures/rust-lifecycle"
OUTPUT = FIXTURES / "mirbuilder-scalar-known-fastpath-read-surface-authority-closeout-rerun-v0.json"

TOKEN = "MIRBUILDER-SCALAR-KNOWN-FASTPATH-READ-SURFACE-AUTHORITY-CLOSEOUT-RERUN-001"
NEXT_CARD = "MIRBUILDER-SCALAR-KNOWN-FASTPATH-WRITE-SURFACE-AUTHORITY-PILOT-DESIGN-STOP-001"

CLOSEOUT = FIXTURES / "mirbuilder-scalar-known-fastpath-read-surface-authority-closeout-v0.json"


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def read_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text(encoding="utf-8"))


def build_fixture() -> dict[str, Any]:
    closeout = read_json(CLOSEOUT)
    return {
        "schema_version": 0,
        "kind": "MirBuilderScalarKnownFastpathReadSurfaceAuthorityCloseoutRerunV1",
        "token": TOKEN,
        "input_state": {
            "closeout_fixture": rel(CLOSEOUT),
            "closeout_fixture_hash": sha256_file(CLOSEOUT),
            "closeout_selected_next_card": (closeout.get("decision") or {}).get(
                "selected_next_card"
            ),
            "read_surface_authority_closeout": (closeout.get("summary") or {}).get(
                "read_surface_authority_closeout"
            ),
        },
        "rerun": {
            "read_surface_authority_closeout": True,
            "closed_read_surface_set": [
                "MapLoadScalarI64Routes",
                "StringScalarI64Routes",
                "CollectionScalarI64Routes",
            ],
            "write_surface_authority_pilot": False,
            "scalar_known_hako_runtime_route_authority": False,
            "next_step_requires_write_surface_design_consultation": True,
        },
        "decision": {
            "kind": "KeepStoppedForWriteSurfaceAuthorityPilotDesign",
            "reason_token": "ReadSurfaceAuthorityCloseoutCompleteWriteMutationAuthorityStillConsultationGated",
            "selected_next_card": NEXT_CARD,
        },
        "summary": {
            "read_surface_authority_closeout_rerun": 1,
            "read_surface_authority_closeout": 1,
            "write_surface_authority_pilot_design_required": 1,
            "write_surface_authority_pilot": 0,
            "scalar_known_hako_runtime_route_authority": 0,
            "source_selfhost_claim": 0,
        },
        "claims": {
            "read_surface_authority_closeout_rerun": 1,
            "read_surface_authority_closeout": 1,
            "write_surface_authority_pilot_design_required": 1,
            "write_surface_authority_pilot": 0,
            "write_mutation_authority": 0,
            "write_publication_authority": 0,
            "scalar_known_hako_runtime_route_authority": 0,
            "scalar_known_transport_axis_authority_switch": 0,
            "rust_fastpath_rewired": 0,
            "route_selection_authority_switch": 0,
            "backend_lowering_authority": 0,
            "runtime_mutation_authority": 0,
            "publication_execution": 0,
            "caller_orientation_runtime_path": 0,
            "source_selfhost_claim": 0,
            "runtime_fallback": 0,
            "new_backend_route": 0,
            "new_abi": 0,
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
        print("mirbuilder-scalar-known-fastpath-read-surface-authority-closeout-rerun unchanged")
        return 0

    changed = write_if_changed(OUTPUT, output)
    print(("updated=" if changed else "unchanged=") + rel(OUTPUT))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
