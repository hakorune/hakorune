#!/usr/bin/env python3
"""Define bridge policy V2 for mention-only forbidden non-claims."""

from __future__ import annotations

import argparse
import hashlib
import json
from pathlib import Path
from typing import Any

from shared_family_generator import stable_json, write_if_changed


ROOT = Path(__file__).resolve().parents[2]
FIXTURES = ROOT / "docs/development/current/main/design/fixtures/rust-lifecycle"
BRIDGE_V1 = FIXTURES / "mirbuilder-strict-converter-emission-to-native-seed-bridge-policy-v0.json"
SCOPE = FIXTURES / "mirbuilder-forbidden-nonclaim-boundary-scope-resolution-v0.json"
OUTPUT = FIXTURES / "mirbuilder-strict-emission-to-native-seed-bridge-policy-v2-v0.json"

TOKEN = "MIRBUILDER-STRICT-EMISSION-TO-NATIVE-SEED-BRIDGE-POLICY-V2-001"
NEXT = "MIRBUILDER-STRICT-CONVERTER-EMISSION-NATIVE-SEED-CANDIDATE-SELECTION-RERUN-002"


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def read_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text(encoding="utf-8"))


def sha256_file(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def build_fixture() -> dict[str, Any]:
    bridge_v1 = read_json(BRIDGE_V1)
    scope = read_json(SCOPE)
    pool = scope.get("candidate_pool") or {}
    all_mention_only = (
        pool.get("wider_denied_boundary_mention_only_count", 0) > 0
        and pool.get("required_by_selected_narrow_seed_surface_count") == 0
        and pool.get("permanent_forbidden_nonclaim_count") == 0
        and pool.get("unclassified_forbidden_nonclaim_count") == 0
    )
    decision = {
        "kind": "PolicyDefined",
        "reason_token": "StrictBridgePolicyV2DefinedForMentionOnlyForbiddenNonclaims",
        "selected_next_card": NEXT,
    } if all_mention_only else {
        "kind": "KeepStopped",
        "reason_token": "StrictBridgePolicyV2RequiresMentionOnlyForbiddenNonclaimScope",
        "selected_next_card": "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001",
    }

    return {
        "schema_version": 0,
        "kind": "MirBuilderStrictEmissionToNativeSeedBridgePolicyV2",
        "token": TOKEN,
        "input_state": {
            "current_blocker": "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001",
            "bridge_policy_v1": rel(BRIDGE_V1),
            "forbidden_nonclaim_boundary_scope_resolution": rel(SCOPE),
        },
        "provenance": {
            "bridge_policy_v1_hash": sha256_file(BRIDGE_V1),
            "forbidden_nonclaim_boundary_scope_resolution_hash": sha256_file(SCOPE),
        },
        "base_policy": {
            "policy_id": bridge_v1["policy"]["seed_draft_input_state_name"],
            "generated_artifact_as_native_edit_authority": bridge_v1["policy"]["generated_artifact_as_native_edit_authority"],
            "source_selfhost_claim_allowed": bridge_v1["policy"]["source_selfhost_claim_allowed"],
        },
        "v2_policy": {
            "policy_id": "StrictEmissionToNativeSeedBridgePolicyV2",
            "mention_only_forbidden_nonclaim_is_seed_evidence": False,
            "mention_only_forbidden_nonclaim_blocks_clean_narrow_seed_surface": False,
            "required_forbidden_nonclaim_blocks_seed": True,
            "unclassified_forbidden_nonclaim_blocks_seed": True,
            "runtime_fallback_allowed": False,
            "new_backend_route_allowed": False,
            "new_abi_allowed": False,
            "new_canonical_mir_instruction_allowed": False,
        },
        "scope_resolution_summary": {
            "input_owner_edge_count": pool.get("input_owner_edge_count"),
            "wider_denied_boundary_mention_only_count": pool.get("wider_denied_boundary_mention_only_count"),
            "required_by_selected_narrow_seed_surface_count": pool.get("required_by_selected_narrow_seed_surface_count"),
            "permanent_forbidden_nonclaim_count": pool.get("permanent_forbidden_nonclaim_count"),
            "unclassified_forbidden_nonclaim_count": pool.get("unclassified_forbidden_nonclaim_count"),
            "bridge_policy_v2_candidate_count": pool.get("bridge_policy_v2_candidate_count"),
        },
        "decision": decision,
        "claims": {
            "bridge_policy_v1_consumed": 1,
            "forbidden_nonclaim_boundary_scope_resolution_consumed": 1,
            "mention_only_forbidden_nonclaim_scope_consumed": 1 if all_mention_only else 0,
            "seed_eligibility_from_forbidden_nonclaim": 0,
            "runtime_fallback": 0,
            "new_backend_route": 0,
            "new_abi": 0,
            "new_canonical_mir_instruction": 0,
            "new_python_semantic_projector": 0,
            "generated_artifact_as_native_edit_authority": 0,
            "native_seed_materialization": 0,
            "hako_generation": 0,
            "hako_adopted_decision": 0,
            "source_selfhost_claim": 0,
            "runner_semantic_owner": 0,
            "manual_family_selection": 0,
            "manual_boundary_reclassification": 0,
            "cluster_size_as_proof": 0,
            "coverage_percentage_as_proof": 0,
            "route_membership_alone_as_proof": 0,
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
        print("mirbuilder-strict-emission-to-native-seed-bridge-policy-v2 unchanged")
        return 0

    changed = write_if_changed(OUTPUT, output)
    print(("updated=" if changed else "unchanged=") + rel(OUTPUT))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
