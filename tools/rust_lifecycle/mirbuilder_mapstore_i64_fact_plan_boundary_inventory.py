#!/usr/bin/env python3
"""Record the MapStoreI64 Fact/Plan/Boundary inventory and active Fact bridge."""

from __future__ import annotations

import argparse
from pathlib import Path
from typing import Any

from shared_family_generator import sha256_file, stable_json, write_if_changed

ROOT = Path(__file__).resolve().parents[2]
OUTPUT = ROOT / "docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-mapstore-i64-fact-plan-boundary-inventory-v0.json"
CARD = ROOT / "docs/development/current/main/phases/phase-296x/3457-MIRBUILDER-MAPSTORE-I64-FACT-PLAN-BOUNDARY-INVENTORY-001.md"
FACTS = ROOT / "src/mir/generic_method_route_facts.rs"
EXACT_FACTS = ROOT / "src/mir/exact_numeric_value_facts.rs"
LOCAL_SLOT = ROOT / "src/mir/type_contracts/local_slot.rs"
LOCAL_TESTS = ROOT / "src/mir/exact_numeric_value_facts/tests/local_contract_write.rs"
WITNESS = ROOT / "src/mir/generic_method_route_plan/mapstore_i64_key_witness.rs"
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
                "candidate_id": "fact.mapstore.key_domain.local_contract_write_i64",
                "candidate_kind": "FactSourceBridge",
                "semantic_statement": "checked i64 LocalContractWrite publishes exact evidence on dst",
                "authority_source": rel(EXACT_FACTS),
                "stable_identity": "ExactNumericValueFactSource::LocalContractWrite",
                "current_producer": "seed_local_contract_write_facts",
                "refresh_or_rebuild_owner": "refresh_module_exact_numeric_value_facts",
                "consumers": ["MapStoreI64KeyWitness"],
                "derived_projections": [],
                "independent_oracle": rel(LOCAL_TESTS),
                "fail_fast_boundary": "stale contract or identity evidence publishes no Fact",
                "fixture": rel(LOCAL_TESTS),
                "authority_conflicts": ["LocalSlotContract or LocalContractWrite alone is insufficient"],
                "behavior_delta": "exact i64 Fact source only",
                "eligibility": "implemented_checked_local_bridge",
                "blocked_reason": "",
            },
            {
                "candidate_id": "projection.mapstore.key_domain.exact_i64_witness",
                "candidate_kind": "ValidationProjection",
                "semantic_statement": "MapStoreI64 key has source-backed exact i64 evidence",
                "authority_source": rel(EXACT_FACTS),
                "stable_identity": "MapStoreI64KeyWitness",
                "current_producer": "refresh_function_mapstore_i64_key_witnesses",
                "refresh_or_rebuild_owner": "refresh_module_exact_numeric_value_facts",
                "consumers": ["verify_mapstore_i64_key_route"],
                "derived_projections": [],
                "independent_oracle": rel(WITNESS),
                "fail_fast_boundary": "missing witness is report-only; claimed witness must verify",
                "fixture": rel(WITNESS),
                "authority_conflicts": ["must not become a second numeric Fact owner"],
                "behavior_delta": "none",
                "eligibility": "implemented_exact_i64_projection",
                "blocked_reason": "",
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
            "exact_numeric_facts": evidence(EXACT_FACTS),
            "local_slot": evidence(LOCAL_SLOT),
            "local_contract_write_tests": evidence(LOCAL_TESTS),
            "witness": evidence(WITNESS),
            "routes": evidence(ROUTES),
            "tests": evidence(TESTS),
            "policy": evidence(POLICY),
        },
        "claims": {
            "mapstore_i64_const_key_fact_candidate": 1,
            "mapstore_i64_const_fact_owner_implemented": 1,
            "existing_exact_numeric_fact_owner_reused": 1,
            "local_contract_write_exact_i64_bridge": 1,
            "mapstore_i64_source_backed_key_witness_candidate": 1,
            "mapstore_i64_first_hard_scope": "exact_i64_only",
            "current_i64value_disposition": "derived_projection",
            "mirtype_integer_hard_authority": 0,
            "new_dynamic_integer_owner": 0,
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
