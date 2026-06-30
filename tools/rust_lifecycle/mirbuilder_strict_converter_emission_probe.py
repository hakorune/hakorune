#!/usr/bin/env python3
"""Probe strict converter emission capability from existing verifier evidence."""

from __future__ import annotations

import argparse
import hashlib
import json
from pathlib import Path
from typing import Any

from shared_family_generator import stable_json, write_if_changed


ROOT = Path(__file__).resolve().parents[2]
FIXTURES = ROOT / "docs/development/current/main/design/fixtures/rust-lifecycle"
INPUT = FIXTURES / "mirbuilder-carrier-type-transport-policy-inventory-v0.json"
OUTPUT = FIXTURES / "mirbuilder-strict-converter-emission-probe-v0.json"
TOKEN = "MIRBUILDER-STRICT-CONVERTER-EMISSION-PROBE-001"
NEXT = "MIRBUILDER-CRATE-WIDE-NATIVE-OWNER-SEED-CAPABILITY-SURVEY-RERUN-003"


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def read_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text(encoding="utf-8"))


def sha256_file(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def verified_fixtures() -> list[dict[str, str]]:
    rows: list[dict[str, str]] = []
    for path in sorted(FIXTURES.glob("*verifier-result-v0.json")):
        try:
            data = read_json(path)
        except json.JSONDecodeError:
            continue
        if data.get("result") != "VerifiedHakoFamilyIR":
            continue
        rows.append(
            {
                "fixture": rel(path),
                "kind": str(data.get("kind", "")),
                "family_id": str(data.get("family_id", "")),
                "result": "VerifiedHakoFamilyIR",
            }
        )
    return rows


def build_probe() -> dict[str, Any]:
    inventory = read_json(INPUT)
    verified = verified_fixtures()

    return {
        "schema_version": 0,
        "kind": "MirBuilderStrictConverterEmissionProbeV1",
        "token": TOKEN,
        "input_state": {
            "carrier_type_transport_policy_inventory": rel(INPUT),
            "current_blocker": "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001",
        },
        "provenance": {
            "carrier_type_transport_policy_inventory_hash": sha256_file(INPUT),
        },
        "probe_scope": {
            "source": "existing verifier-result fixtures only",
            "emits_hako": False,
            "constructs_verified_hako_family_ir": False,
            "weakens_strict_rules": False,
        },
        "verified_hako_family_ir_fixtures": verified,
        "summary": {
            "verified_hako_family_ir_count": len(verified),
            "carrier_type_transport_candidate_count": inventory["summary"]["carrier_type_transport_candidate_count"],
            "policy_lane_selected_count": inventory["summary"]["policy_lane_selected_count"],
        },
        "decision": {
            "kind": "SelectNativeOwnerSeedCapabilitySurveyRerun",
            "reason_token": "StrictEmissionProbeRecordedFromExistingVerifierEvidence",
            "selected_next_card": NEXT,
        },
        "claims": {
            "carrier_type_transport_inventory_consumed": 1,
            "existing_verifier_results_consumed": 1,
            "strict_emission_probe_ready": 1,
            "hako_generation": 0,
            "verified_hako_family_ir_constructed_by_probe": 0,
            "strict_rules_changed": 0,
            "fallback_hako_emission": 0,
            "hako_adopted_decision": 0,
            "native_seed_materialization": 0,
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
    parser.add_argument("--check", action="store_true", help="Verify checked-in probe fixture.")
    args = parser.parse_args()

    output = stable_json(build_probe())
    if args.check:
        if not OUTPUT.exists() or OUTPUT.read_text(encoding="utf-8") != output:
            raise SystemExit(f"{rel(OUTPUT)} is stale; rerun without --check")
        print("mirbuilder-strict-converter-emission-probe unchanged")
        return 0

    changed = write_if_changed(OUTPUT, output)
    print(("updated=" if changed else "unchanged=") + rel(OUTPUT))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
