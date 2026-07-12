#!/usr/bin/env python3
"""Record the 3457 MapStoreI64 Fact/Plan/Boundary inventory result."""

from __future__ import annotations

import argparse
from pathlib import Path
from typing import Any

from shared_family_generator import sha256_file, stable_json, write_if_changed

ROOT = Path(__file__).resolve().parents[2]
OUTPUT = ROOT / "docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-mapstore-i64-fact-plan-boundary-inventory-v0.json"
CARD = ROOT / "docs/development/current/main/phases/phase-296x/3457-MIRBUILDER-MAPSTORE-I64-FACT-PLAN-BOUNDARY-INVENTORY-001.md"
FACTS = ROOT / "src/mir/generic_method_route_facts.rs"
ROUTES = ROOT / "src/mir/generic_method_route_plan/write_routes.rs"
TESTS = ROOT / "src/mir/generic_method_route_plan/tests/map_set_routes/map_get_scalar.rs"
POLICY = ROOT / "lang/src/compiler/lib/write_set_mapstore_route_policy.hako"

TOKEN = "MIRBUILDER-MAPSTORE-I64-FACT-PLAN-BOUNDARY-INVENTORY-001"


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def evidence(path: Path) -> dict[str, str]:
    return {"path": rel(path), "sha256": sha256_file(path)}


def build_fixture() -> dict[str, Any]:
    return {
        "schema_version": 0,
        "kind": "MirBuilderMapStoreI64FactPlanBoundaryInventoryV1",
        "token": TOKEN,
        "scope": {
            "surface": "SetSurfacePolicy",
            "route_kind": "MapStoreI64",
            "policy_row_id": "map_store_i64_set_surface",
            "stored_value_domain": "Any",
        },
        "candidates": [
            {
                "candidate_id": "fact.mapstore.key_domain.i64_const",
                "candidate_kind": "Fact",
                "semantic_statement": "resolved key origin is ConstValue::Integer",
                "authority_source": rel(FACTS),
                "stable_identity": "MapStoreI64ConstKeyFact",
                "current_producer": "mapstore_i64_const_key_fact",
                "refresh_or_rebuild_owner": "value_origin::build_value_def_map",
                "consumers": ["classify_key_route", "write_routes::match_generic_set_route"],
                "derived_projections": ["GenericMethodKeyRoute::I64Const"],
                "independent_oracle": rel(TESTS),
                "fail_fast_boundary": "None: absent fact yields non-I64 route classification",
                "fixture": rel(TESTS),
                "authority_conflicts": [],
                "behavior_delta": "none",
                "eligibility": "implemented_narrow_fact_owner",
                "blocked_reason": "",
            },
            {
                "candidate_id": "fact.mapstore.key_domain.i64_value",
                "candidate_kind": "Fact",
                "semantic_statement": "key is a dynamic integer value",
                "authority_source": rel(FACTS),
                "stable_identity": "GenericMethodKeyRoute::I64Value",
                "current_producer": "classify_key_route metadata branch",
                "refresh_or_rebuild_owner": "unisolated MirType::Integer metadata",
                "consumers": ["write_routes::match_generic_set_route"],
                "derived_projections": ["GenericMethodKeyRoute::I64Value"],
                "independent_oracle": rel(TESTS),
                "fail_fast_boundary": "source provenance owner missing",
                "fixture": rel(TESTS),
                "authority_conflicts": ["MirType::Integer is not source-backed key origin"],
                "behavior_delta": "none",
                "eligibility": "pending",
                "blocked_reason": "dynamic integer provenance owner not isolated",
            },
            {
                "candidate_id": "plan.mapstore.set.route_decision",
                "candidate_kind": "Plan",
                "semantic_statement": "MapStoreI64 route decision payload",
                "authority_source": rel(POLICY),
                "stable_identity": "GenericMethodRouteDecision",
                "current_producer": "write_routes + generated Hako policy",
                "refresh_or_rebuild_owner": "policy generator",
                "consumers": ["write_routes"],
                "derived_projections": [],
                "independent_oracle": rel(ROUTES),
                "fail_fast_boundary": "route-selection authority retained by Rust",
                "fixture": rel(CARD),
                "authority_conflicts": ["would move route selection authority"],
                "behavior_delta": "forbidden",
                "eligibility": "blocked",
                "blocked_reason": "3454 route authority non-claim",
            },
            {
                "candidate_id": "boundary.mapstore.set.mutation",
                "candidate_kind": "Boundary",
                "semantic_statement": "MapStore mutation/publication execution",
                "authority_source": "downstream Rust",
                "stable_identity": "MapStore mutation boundary",
                "current_producer": "downstream consumer",
                "refresh_or_rebuild_owner": "not selected",
                "consumers": [],
                "derived_projections": [],
                "independent_oracle": "",
                "fail_fast_boundary": "runtime mutation/publication remain closed",
                "fixture": rel(CARD),
                "authority_conflicts": ["runtime mutation authority = 0"],
                "behavior_delta": "forbidden",
                "eligibility": "blocked",
                "blocked_reason": "Fact and Plan authority must precede Boundary",
            },
        ],
        "provenance": {
            "card": evidence(CARD),
            "facts": evidence(FACTS),
            "routes": evidence(ROUTES),
            "tests": evidence(TESTS),
            "policy": evidence(POLICY),
        },
        "claims": {
            "mapstore_i64_const_key_fact_candidate": 1,
            "mapstore_i64_const_fact_owner_implemented": 1,
            "mapstore_dynamic_i64_fact_candidate": "pending",
            "mapstore_i64_plan_candidate": "blocked",
            "mapstore_i64_boundary_candidate": "blocked",
            "mapstore_i64_registry_descriptor_candidate": "conditional",
            "mapstore_i64_caller_projection_closed": 1,
            "hard_authority_activation": 0,
            "route_behavior_change": 0,
            "runtime_mutation_authority": 0,
            "backend_lowering_authority": 0,
            "publication_execution": 0,
            "mapstore_any_opened": 0,
            "array_append_any_opened": 0,
            "delete_opened": 0,
            "scalar_known_wide_opened": 0,
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
        print("mapstore-i64 fact/plan/boundary inventory unchanged")
        return 0
    changed = write_if_changed(OUTPUT, output)
    print(("updated=" if changed else "unchanged=") + rel(OUTPUT))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
