#!/usr/bin/env python3
"""Define state mutation frames for bounded ID scalar owner edges."""

from __future__ import annotations

import argparse
import json
from pathlib import Path
from typing import Any

from shared_family_generator import sha256_file, stable_json, write_if_changed


ROOT = Path(__file__).resolve().parents[2]
FIXTURES = ROOT / "docs/development/current/main/design/fixtures/rust-lifecycle"
OUTPUT = FIXTURES / "mirbuilder-id-scalar-state-mutation-frame-basis-v0.json"

TOKEN = "MIRBUILDER-ID-SCALAR-STATE-MUTATION-FRAME-BASIS-001"
DESIGN_STOP = "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001"
NEXT_CARD = "MIRBUILDER-ID-SCALAR-ERROR-AND-DETERMINISTIC-ORDER-BASIS-001"

ID_DOMAIN = FIXTURES / "mirbuilder-id-scalar-id-domain-boundary-basis-v0.json"
FILE_BOUNDARY = FIXTURES / "mirbuilder-id-scalar-native-seed-file-boundary-basis-v0.json"
STATE_TARGETS = FIXTURES / "mirbuilder-id-scalar-state-target-enumeration-basis-v0.json"


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def read_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text(encoding="utf-8"))


def mutation_semantics(access: list[str]) -> dict[str, Any]:
    access_set = set(access)
    return {
        "read_set_declared": "Read" in access_set,
        "write_set_declared": bool(access_set & {"Write", "Append", "Allocate"}),
        "append_semantics": "Append" in access_set,
        "replace_semantics": "Write" in access_set,
        "clear_semantics": False,
        "allocate_semantics": "Allocate" in access_set,
        "mutation_order": "SourceSurfaceOrder",
        "rollback_requirement": "NoRollbackDeclared",
        "cleanup_requirement": "NoCleanupDeclared",
        "owner_return_state": "OwnerRetainedAfterFrame",
    }


def build_fixture() -> dict[str, Any]:
    id_domain = read_json(ID_DOMAIN)
    file_boundary = read_json(FILE_BOUNDARY)
    state_targets = read_json(STATE_TARGETS)

    bounded_owners = {
        row["owner_edge_id"]
        for row in file_boundary.get("boundary_rows") or []
        if row.get("native_seed_file_boundary_derivable")
    }

    frames = []
    for owner_row in state_targets.get("owner_edge_targets") or []:
        owner = owner_row["owner_edge_id"]
        if owner not in bounded_owners:
            continue
        for target in owner_row.get("state_targets") or []:
            if not target.get("mutation_frame_required"):
                continue
            frame_id = target["state_target_id"].replace("::", ".") + ".mutation_frame"
            frames.append(
                {
                    "mutation_frame_id": frame_id,
                    "owner_edge_id": owner,
                    "state_target_id": target["state_target_id"],
                    "semantic_resource": target["semantic_resource"],
                    "target_kind": target["target_kind"],
                    "access": target["access"],
                    "operation_tokens": target["operation_tokens"],
                    "source_surfaces": target["source_surfaces"],
                    **mutation_semantics(target["access"]),
                }
            )

    return {
        "schema_version": 0,
        "kind": "MirBuilderIdScalarStateMutationFrameBasisV1",
        "token": TOKEN,
        "input_state": {
            "current_blocker": DESIGN_STOP,
            "id_domain_boundary_basis": rel(ID_DOMAIN),
            "native_seed_file_boundary_basis": rel(FILE_BOUNDARY),
            "state_target_enumeration_basis": rel(STATE_TARGETS),
        },
        "provenance": {
            "id_domain_boundary_basis_hash": sha256_file(ID_DOMAIN),
            "native_seed_file_boundary_basis_hash": sha256_file(FILE_BOUNDARY),
            "state_target_enumeration_basis_hash": sha256_file(STATE_TARGETS),
        },
        "previous_state": {
            "id_domain_boundary_count": (id_domain.get("candidate_pool") or {}).get(
                "id_domain_boundary_count"
            ),
            "native_seed_file_boundary_derivable_count": (
                file_boundary.get("candidate_pool") or {}
            ).get("native_seed_file_boundary_derivable_count"),
        },
        "mutation_frame_policy": {
            "primary_unit": "semantic_state_target",
            "grouped_by": "owner_edge",
            "requires_bounded_owner": True,
            "requires_native_seed_file_boundary": True,
            "rollback_must_be_declared": True,
            "cleanup_must_be_declared": True,
            "owner_return_state_must_be_declared": True,
            "cross_owner_targets_excluded_until_recipe_authority_split": True,
        },
        "mutation_frames": frames,
        "candidate_pool": {
            "bounded_owner_count": len(bounded_owners),
            "mutation_frame_count": len(frames),
            "rollback_declared_count": len([row for row in frames if row["rollback_requirement"]]),
            "cleanup_declared_count": len([row for row in frames if row["cleanup_requirement"]]),
            "owner_return_state_declared_count": len([row for row in frames if row["owner_return_state"]]),
        },
        "decision": {
            "kind": "StateMutationFrameBasisDefined",
            "reason_token": "IdScalarStateMutationFramesDeclared",
            "selected_next_card": NEXT_CARD,
        },
        "claims": {
            "manual_owner_selection": 0,
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
        print("mirbuilder-id-scalar-state-mutation-frame-basis unchanged")
        return 0

    changed = write_if_changed(OUTPUT, output)
    print(("updated=" if changed else "unchanged=") + rel(OUTPUT))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
