#!/usr/bin/env python3
"""Freeze the ScalarKnown write Push surface .hako parity pilot fixture."""

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
    / "mirbuilder-carrier-type-scalar-known-write-push-surface-hako-parity-pilot-v0.json"
)

TOKEN = (
    "MIRBUILDER-CARRIER-TYPE-SCALAR-KNOWN-WRITE-PUSH-SURFACE-"
    "HAKO-PARITY-PILOT-001"
)
NEXT_CARD = (
    "MIRBUILDER-CARRIER-TYPE-SCALAR-KNOWN-WRITE-PUSH-SURFACE-"
    "PARITY-GATE-001"
)

RUST_ORACLE = (
    FIXTURES
    / "mirbuilder-carrier-type-scalar-known-write-push-surface-rust-oracle-v0.json"
)
HAKO_SOURCE = ROOT / "lang/src/compiler/lib/write_push_surface_policy_classifier.hako"


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def read_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text(encoding="utf-8"))


def oracle_row(oracle: dict[str, Any]) -> dict[str, Any]:
    rows = oracle.get("oracle_fixture", {}).get("rows") or []
    if len(rows) != 1:
        raise SystemExit("expected exactly one write Push Rust oracle row")
    return rows[0]


def expected_output(row: dict[str, Any]) -> str:
    fields = [
        "case_id",
        "subsurface_id",
        "route_kind",
        "core_method_op",
        "core_method_lowering_tier",
        "result_class",
        "return_shape",
        "value_demand",
        "publication_policy",
        "effect_class",
        "mutation_class",
        "hako_role",
    ]
    return "|".join(str(row[field]) for field in fields)


def build_fixture() -> dict[str, Any]:
    oracle = read_json(RUST_ORACLE)
    row = oracle_row(oracle)

    return {
        "schema_version": 0,
        "kind": "MirBuilderCarrierTypeScalarKnownWritePushSurfaceHakoParityPilotV1",
        "token": TOKEN,
        "input_state": {
            "rust_oracle_fixture": rel(RUST_ORACLE),
            "rust_oracle_token": oracle.get("token"),
            "rust_oracle_decision": oracle.get("decision", {}).get("kind"),
            "rust_oracle_selected_next_card": oracle.get("decision", {}).get(
                "selected_next_card"
            ),
            "rust_oracle_hash": sha256_file(RUST_ORACLE),
        },
        "hako_implementation": {
            "source": rel(HAKO_SOURCE),
            "source_hash": sha256_file(HAKO_SOURCE),
            "box": "WritePushSurfacePolicyClassifierBox",
            "method": "classify",
            "role": "classifier_policy_mirror_only",
            "runtime_mutation_authority": False,
        },
        "parity_pilot_fixture": {
            "fixture_id": "WritePushSurfaceHakoParityPilotV0",
            "row_count": 1,
            "rows": [
                {
                    "case_id": row["case_id"],
                    "input_route_kind": row["route_kind"],
                    "expected": expected_output(row),
                }
            ],
        },
        "selection_rule": {
            "name": "WritePushSurfaceHakoParityPilotImplementationOnlyV1",
            "hako_source_landed": True,
            "hako_source_verifies": True,
            "parity_gate_required_before_adoption": True,
            "direct_closeout_materialization_allowed": False,
            "hako_adoption_allowed": False,
            "source_selfhost_claim_allowed": False,
        },
        "summary": {
            "write_push_surface_hako_parity_pilot": 1,
            "hako_implementation_landed": 1,
            "hako_source_verifies": 1,
            "array_append_any_scope": 1,
            "push_surface_policy_scope": 1,
            "classifier_policy_mirror_only": 1,
            "runtime_mutation_authority": 0,
            "parity_gate_required": 1,
            "write_direct_closeout_materialized": 0,
            "write_result_policy_ready": 0,
            "write_scalar_i64_routes_closeout": 0,
            "scalar_known_transport_axis_closeout": 0,
            "hako_adoption": 0,
            "source_selfhost_claim": 0,
        },
        "decision": {
            "kind": "SelectWritePushSurfaceParityGate",
            "reason_token": "WritePushSurfaceHakoParityPilotLanded",
            "selected_next_card": NEXT_CARD,
        },
        "claims": {
            "write_push_surface_hako_parity_pilot": 1,
            "hako_implementation_landed": 1,
            "hako_source_verifies": 1,
            "array_append_any_scope": 1,
            "push_surface_policy_scope": 1,
            "classifier_policy_mirror_only": 1,
            "parity_gate_required": 1,
            "write_subsurface_selected": 0,
            "write_direct_closeout_materialized": 0,
            "write_result_policy_ready": 0,
            "write_scalar_i64_routes_closeout": 0,
            "scalar_known_transport_axis_closeout": 0,
            "component_specific_direct_contract_materialized": 0,
            "hako_adoption": 0,
            "source_selfhost_claim": 0,
            "new_route_authority": 0,
            "behavior_change": 0,
            "runtime_mutation_authority": 0,
            "runtime_fallback": 0,
            "new_backend_route": 0,
            "new_abi": 0,
            "native_seed_materialization": 0,
            "hako_generation": 0,
            "hako_adopted_decision": 0,
            "manual_subsurface_selection": 0,
            "route_count_as_proof": 0,
            "apparent_simplicity_as_proof": 0,
            "accepted_read_contract_similarity_as_proof": 0,
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
        print("mirbuilder-carrier-type-scalar-known-write-push-surface-hako-parity-pilot unchanged")
        return 0

    changed = write_if_changed(OUTPUT, output)
    print(("updated=" if changed else "unchanged=") + rel(OUTPUT))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
