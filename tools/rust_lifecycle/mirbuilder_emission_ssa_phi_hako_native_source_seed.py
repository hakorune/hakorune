#!/usr/bin/env python3
"""Materialize the emission_ssa_phi native Hako source seed fixture."""

from __future__ import annotations

import argparse
import json
from pathlib import Path

from shared_family_generator import sha256_file, stable_json, write_if_changed


ROOT = Path(__file__).resolve().parents[2]
FIXTURES = ROOT / "docs/development/current/main/design/fixtures/rust-lifecycle"
OUTPUT = FIXTURES / "mirbuilder-emission-ssa-phi-hako-native-source-seed-v0.json"

TOKEN = "MIRBUILDER-EMISSION_SSA_PHI-HAKO-NATIVE-SOURCE-SEED-001"
NEXT_CARD = "MIRBUILDER-EMISSION_SSA_PHI-HAKO-ADOPTION-DECISION-001"
OWNER = "mirbuilder::emission_ssa_phi"
DESIGN_STOP = "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001"

READINESS = FIXTURES / "mirbuilder-id-scalar-domain-seed-readiness-resolution-003-v0.json"
SEED_DRAFT = FIXTURES / "mirbuilder-emission-ssa-phi-id-scalar-derived-artifact-seed-draft-v0.json"
BOUNDARY = FIXTURES / "mirbuilder-id-scalar-native-seed-file-boundary-basis-v0.json"
NATIVE_SEED = ROOT / "lang/src/compiler/lib/mirbuilder/emission_ssa_phi_native_seed.hako"
MODULE = ROOT / "lang/src/compiler/hako_module.toml"


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def read_json(path: Path) -> dict:
    return json.loads(path.read_text(encoding="utf-8"))


def boundary_row() -> dict:
    boundary = read_json(BOUNDARY)
    for row in boundary.get("boundary_rows") or []:
        if row.get("owner_edge_id") == OWNER:
            return row
    raise SystemExit("missing emission_ssa_phi native seed boundary row")


def build_fixture() -> dict:
    readiness = read_json(READINESS)
    seed_draft = read_json(SEED_DRAFT)
    boundary = boundary_row()
    decision = readiness.get("decision") or {}
    if decision.get("selected_owner_edge_id") != OWNER:
        raise SystemExit("readiness 003 no longer selects emission_ssa_phi")

    return {
        "schema_version": 0,
        "kind": "MirBuilderEmissionSsaPhiHakoNativeSourceSeedV1",
        "token": TOKEN,
        "input_state": {
            "current_blocker": DESIGN_STOP,
            "seed_readiness_resolution": rel(READINESS),
            "derived_artifact_seed_draft_input": rel(SEED_DRAFT),
            "native_seed_file_boundary_basis": rel(BOUNDARY),
        },
        "provenance": {
            "seed_readiness_resolution_hash": sha256_file(READINESS),
            "derived_artifact_seed_draft_input_hash": sha256_file(SEED_DRAFT),
            "native_seed_file_boundary_basis_hash": sha256_file(BOUNDARY),
            "native_source_seed_hash": sha256_file(NATIVE_SEED),
            "module_manifest_hash": sha256_file(MODULE),
        },
        "selected_owner": {
            "owner_edge_id": OWNER,
            "selection_reason": decision.get("reason_token"),
            "selected_next_card": TOKEN,
        },
        "native_source_seed": {
            "native_source_seed_path": boundary.get("native_source_seed_path"),
            "module_export": boundary.get("module_export"),
            "native_source_seed_outside_generated_tree": True,
            "generator_overwrite_guard": True,
            "source_input_state": (seed_draft.get("seed_draft_input") or {}).get("state"),
        },
        "seed_status": {
            "native_source_owner_seed_present": 1,
            "hako_adopted_decision": 0,
            "native_edit_authority_claim": 0,
            "generated_artifact_as_edit_authority": 0,
            "source_selfhost_claim": 0,
        },
        "decision": {
            "kind": "HakoAdoptionDecisionDeferred",
            "reason_token": "EmissionSsaPhiNativeSourceSeedMaterialized",
            "selected_next_card": NEXT_CARD,
        },
        "claims": {
            "native_seed_materialization": 1,
            "generated_artifact_as_native_edit_authority": 0,
            "hako_generation": 0,
            "hako_adopted_decision": 0,
            "source_selfhost_claim": 0,
            "rust_deletion": 0,
            "runtime_fallback": 0,
            "new_backend_route": 0,
            "new_abi": 0,
            "new_python_semantic_projector": 0,
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
        print("mirbuilder-emission-ssa-phi-hako-native-source-seed unchanged")
        return 0

    changed = write_if_changed(OUTPUT, output)
    print(("updated=" if changed else "unchanged=") + rel(OUTPUT))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
