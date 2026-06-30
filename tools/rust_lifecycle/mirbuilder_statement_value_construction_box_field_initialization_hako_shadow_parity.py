#!/usr/bin/env python3
"""Materialize box field initialization HakoShadow parity result."""

from __future__ import annotations

import argparse
import json
from pathlib import Path
from typing import Any

from shared_family_generator import stable_json, write_if_changed


ROOT = Path(__file__).resolve().parents[2]
FIXTURES = ROOT / "docs/development/current/main/design/fixtures/rust-lifecycle"
CONTRACT = (
    FIXTURES
    / "mirbuilder-statement-value-construction-box-field-initialization-mutation-frame-contract-v0.json"
)
OUTPUT = (
    FIXTURES
    / "mirbuilder-statement-value-construction-box-field-initialization-hako-shadow-result-v0.json"
)


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def read_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text(encoding="utf-8"))


def build_result() -> dict[str, Any]:
    contract = read_json(CONTRACT)
    frame = contract["mutation_frame_contract"]
    shadow_record = {
        "schema_version": 0,
        "kind": "BoxFieldInitializationShadowCandidateV1",
        "family_id": "hakorune_mir_builder::statement_value_construction_box_field_initialization",
        "stage_id": "box_field_initialization",
        "subject": "StatementValueConstruction box field initialization mutation frame",
        "source_authority": contract["input_state"]["source_surfaces"],
        "available_capabilities": [
            "BoxFieldInitializationMutationFrame",
        ],
        "mutation_frame_contract": frame,
        "result_contract": {
            "mutation_order": frame["mutation_order"],
            "read_only_inputs": frame["read_only_inputs"],
            "state_outputs": frame["state_outputs"],
            "delegated_mutation_owner": frame["delegated_mutation_owner"],
        },
        "non_claims": {
            "hako_adopted_decision": 0,
            "hako_generation": 0,
            "native_seed_materialization": 0,
            "runtime_fallback": 0,
            "source_selfhost_claim": 0,
        },
    }
    return {
        "schema_version": 0,
        "kind": "MirBuilderStatementValueConstructionBoxFieldInitializationHakoShadowResultV1",
        "subject": "StatementValueConstruction box field initialization mutation frame",
        "result": {
            "err": 0,
            "err_line": "",
            "shadow_record": shadow_record,
            "shadow_json": json.dumps(shadow_record, indent=2, sort_keys=True) + "\n",
        },
        "stage_state": {
            "family_id": "hakorune_mir_builder::statement_value_construction_box_field_initialization",
            "stage_id": "box_field_initialization",
            "input_json": "mirbuilder-statement-value-construction-box-field-initialization-mutation-frame-contract-v0.json",
            "output_json": "BoxFieldInitializationShadowCandidateV1",
            "python_oracle": "BoxFieldInitializationMutationFrameContractV1",
            "hako_shadow": "BoxFieldInitializationHakoProjector",
            "parity_gate": "rust_lifecycle_mirbuilder_statement_value_construction_box_field_initialization_hako_shadow_parity_guard.sh",
            "promotion_token": "BoxFieldInitializationHakoShadowPromotionTokenV1",
            "retirement_token": "BoxFieldInitializationHakoShadowRetirementTokenV1",
        },
        "non_claims": {
            "hako_adopted_decision": 0,
            "hako_generation": 0,
            "native_seed_materialization": 0,
            "runtime_fallback": 0,
            "source_selfhost_claim": 0,
        },
        "input_authority": {
            "mutation_frame_contract": rel(CONTRACT),
        },
    }


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--check", action="store_true", help="Verify checked-in result fixture.")
    args = parser.parse_args()

    output = stable_json(build_result())
    if args.check:
        if not OUTPUT.exists() or OUTPUT.read_text(encoding="utf-8") != output:
            raise SystemExit(f"{rel(OUTPUT)} is stale; rerun without --check")
        print("mirbuilder-statement-value-construction-box-field-initialization-hako-shadow-result unchanged")
        return 0

    changed = write_if_changed(OUTPUT, output)
    print(("updated=" if changed else "unchanged=") + rel(OUTPUT))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
