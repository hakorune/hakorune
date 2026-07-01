#!/usr/bin/env python3
"""Materialize the ResultBox carrier verifier contract."""

from __future__ import annotations

import argparse
import hashlib
import json
from pathlib import Path
from typing import Any

from shared_family_generator import stable_json, write_if_changed


ROOT = Path(__file__).resolve().parents[2]
FIXTURES = ROOT / "docs/development/current/main/design/fixtures/rust-lifecycle"
POLICY = FIXTURES / "mirbuilder-result-carrier-verifier-policy-v0.json"
OUTPUT = FIXTURES / "mirbuilder-result-carrier-verifier-contract-v0.json"

TOKEN = "MIRBUILDER-RESULT-CARRIER-VERIFIER-CONTRACT-001"
NEXT = "MIRBUILDER-RESULT-CARRIER-VERIFIER-PROJECTION-POLICY-001"


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def read_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text(encoding="utf-8"))


def sha256_file(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def contract_row(row: dict[str, Any]) -> dict[str, Any]:
    return {
        "owner_edge_id": row["owner_edge_id"],
        "result_transport": row["result_transport"],
        "projection_contract": row["projection_contract"],
        "plan_kind": row["plan_kind"],
        "required_checks": {
            "result_transport_suffix_is_result_box": str(row["result_transport"]).endswith("ResultBox"),
            "projection_contract_present": bool(row["projection_contract"]),
            "canonical_json_parity": row["canonical_json_parity"],
            "runtime_fallback": row["runtime_fallback"],
        },
        "contract_state": "VerifierContractReady",
    }


def build_fixture() -> dict[str, Any]:
    policy = read_json(POLICY)
    rows = [contract_row(row) for row in policy.get("policy_rows", [])]
    ready = all(
        row["required_checks"]["result_transport_suffix_is_result_box"]
        and row["required_checks"]["projection_contract_present"]
        and row["required_checks"]["canonical_json_parity"] == 1
        and row["required_checks"]["runtime_fallback"] == 0
        for row in rows
    )
    decision = {
        "kind": "SelectResultCarrierVerifierProjectionPolicy",
        "reason_token": "ResultCarrierVerifierContractReady",
        "selected_next_card": NEXT,
    } if ready else {
        "kind": "KeepStopped",
        "reason_token": "ResultCarrierVerifierContractIncomplete",
        "selected_next_card": "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001",
    }

    return {
        "schema_version": 0,
        "kind": "MirBuilderResultCarrierVerifierContractV1",
        "token": TOKEN,
        "input_state": {
            "current_blocker": "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001",
            "result_carrier_verifier_policy": rel(POLICY),
        },
        "provenance": {
            "result_carrier_verifier_policy_hash": sha256_file(POLICY),
        },
        "contract": {
            "contract_id": "ResultCarrierVerifierContractV1",
            "applies_to_policy": "ResultCarrierVerifierPolicyV1",
            "row_contract_count": len(rows),
            "required_invariants": [
                "result_transport ends with ResultBox",
                "projection_contract present",
                "canonical_json_parity = 1",
                "runtime_fallback = 0",
            ],
        },
        "contract_rows": rows,
        "summary": {
            "result_carrier_contract_row_count": len(rows),
            "result_carrier_contract_ready": 1 if ready else 0,
        },
        "decision": decision,
        "claims": {
            "result_carrier_verifier_policy_consumed": 1,
            "result_carrier_verifier_contract_ready": 1 if ready else 0,
            "manual_family_selection": 0,
            "manual_shape_selection": 0,
            "manual_axis_selection": 0,
            "manual_carrier_selection": 0,
            "owner_name_as_transport_policy": 0,
            "cluster_size_as_proof": 0,
            "coverage_percentage_as_proof": 0,
            "route_membership_alone_as_proof": 0,
            "generated_artifact_as_native_edit_authority": 0,
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
        print("mirbuilder-result-carrier-verifier-contract unchanged")
        return 0

    changed = write_if_changed(OUTPUT, output)
    print(("updated=" if changed else "unchanged=") + rel(OUTPUT))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
