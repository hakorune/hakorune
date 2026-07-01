#!/usr/bin/env python3
"""Define native seed file boundaries for bounded ID scalar owner edges."""

from __future__ import annotations

import argparse
import json
from pathlib import Path
from typing import Any

from shared_family_generator import sha256_file, stable_json, write_if_changed


ROOT = Path(__file__).resolve().parents[2]
FIXTURES = ROOT / "docs/development/current/main/design/fixtures/rust-lifecycle"
OUTPUT = FIXTURES / "mirbuilder-id-scalar-native-seed-file-boundary-basis-v0.json"

TOKEN = "MIRBUILDER-ID-SCALAR-NATIVE-SEED-FILE-BOUNDARY-BASIS-001"
DESIGN_STOP = "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001"
NEXT_CARD = "MIRBUILDER-ID-SCALAR-SOURCE-PLAN-BASIS-COMPONENT-PRIORITY-RESOLUTION-002"

OWNER_SCOPE = FIXTURES / "mirbuilder-id-scalar-owner-scope-boundedness-resolution-002-v0.json"
STATE_TARGETS = FIXTURES / "mirbuilder-id-scalar-state-target-enumeration-basis-v0.json"


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def read_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text(encoding="utf-8"))


def owner_slug(owner_edge_id: str) -> str:
    return owner_edge_id.split("::", 1)[1].replace("::", "_")


def build_fixture() -> dict[str, Any]:
    owner_scope = read_json(OWNER_SCOPE)
    state_targets = read_json(STATE_TARGETS)
    targets_by_owner = {row["owner_edge_id"]: row for row in state_targets.get("owner_edge_targets") or []}

    rows = []
    for candidate in owner_scope.get("candidates") or []:
        owner = candidate["owner_edge_id"]
        slug = owner_slug(owner)
        bounded = bool(candidate.get("owner_scope_bounded"))
        cross_owner = bool(candidate.get("operation_tokens_need_cross_owner_recipe_authority"))
        target_row = targets_by_owner[owner]
        boundary_derivable = bounded and not cross_owner
        rows.append(
            {
                "owner_edge_id": owner,
                "owner_scope_bounded": bounded,
                "state_target_count": target_row.get("state_targets") and len(target_row["state_targets"]),
                "operation_token_count": len(candidate.get("operation_tokens") or []),
                "native_seed_file_boundary_derivable": boundary_derivable,
                "native_source_seed_path": (
                    f"lang/src/compiler/lib/mirbuilder/{slug}_native_seed.hako"
                    if boundary_derivable
                    else None
                ),
                "module_export": f"lib.mirbuilder.{slug}_native_seed" if boundary_derivable else None,
                "generator_overwrite_guard_path": (
                    f"lang/src/compiler/lib/mirbuilder/{slug}_native_seed.hako"
                    if boundary_derivable
                    else None
                ),
                "blocked_by": []
                if boundary_derivable
                else [
                    "NativeSeedFileBoundaryRequiresBoundedOwnerScope",
                    "CrossOwnerRecipeAuthorityMustBeSeparated",
                ],
            }
        )

    boundary_rows = [row for row in rows if row["native_seed_file_boundary_derivable"]]
    return {
        "schema_version": 0,
        "kind": "MirBuilderIdScalarNativeSeedFileBoundaryBasisV1",
        "token": TOKEN,
        "input_state": {
            "current_blocker": DESIGN_STOP,
            "owner_scope_boundedness_rerun_002": rel(OWNER_SCOPE),
            "state_target_enumeration_basis": rel(STATE_TARGETS),
        },
        "provenance": {
            "owner_scope_boundedness_rerun_002_hash": sha256_file(OWNER_SCOPE),
            "state_target_enumeration_basis_hash": sha256_file(STATE_TARGETS),
        },
        "boundary_policy": {
            "authority": [
                "owner_edge",
                "state_target_set",
                "operation_token_set",
                "module_export_plan",
            ],
            "source_path_alone_as_authority": False,
            "surface_count_as_proof": False,
            "owner_name_alone_as_authority": False,
            "native_seed_materialization": False,
        },
        "boundary_rows": rows,
        "candidate_pool": {
            "input_candidate_count": len(rows),
            "owner_scope_bounded_count": len([row for row in rows if row["owner_scope_bounded"]]),
            "native_seed_file_boundary_derivable_count": len(boundary_rows),
            "cross_owner_boundary_blocked_count": len(rows) - len(boundary_rows),
        },
        "decision": {
            "kind": "NativeSeedFileBoundaryBasisDefined",
            "reason_token": "IdScalarNativeSeedFileBoundaryBasisDefined",
            "selected_next_card": NEXT_CARD,
        },
        "claims": {
            "manual_owner_selection": 0,
            "surface_count_as_proof": 0,
            "source_path_alone_as_authority": 0,
            "owner_name_alone_as_authority": 0,
            "source_plan_materialization": 0,
            "behavior_recipe_materialization": 0,
            "verifier_result_materialization": 0,
            "derived_artifact_seed_draft_materialization": 0,
            "native_seed_materialization": 0,
            "hako_generation": 0,
            "hako_adopted_decision": 0,
            "source_selfhost_claim": 0,
            "runtime_fallback": 0,
            "new_backend_route": 0,
            "new_abi": 0,
            "new_python_semantic_projector": 0,
            "runner_semantic_owner": 0,
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
        print("mirbuilder-id-scalar-native-seed-file-boundary-basis unchanged")
        return 0

    changed = write_if_changed(OUTPUT, output)
    print(("updated=" if changed else "unchanged=") + rel(OUTPUT))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
