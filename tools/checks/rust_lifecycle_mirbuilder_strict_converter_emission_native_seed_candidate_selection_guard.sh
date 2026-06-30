#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
FIXTURE="$ROOT/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-strict-converter-emission-native-seed-candidate-selection-v0.json"
TOOL="$ROOT/tools/rust_lifecycle/mirbuilder_strict_converter_emission_native_seed_candidate_selection.py"
CARD="$ROOT/docs/development/current/main/phases/phase-296x/1964-MIRBUILDER-STRICT-CONVERTER-EMISSION-NATIVE-SEED-CANDIDATE-SELECTION-001.md"

python3 "$TOOL" --check

python3 - "$FIXTURE" "$CARD" <<'PY'
import json
import sys
from pathlib import Path

fixture = json.load(open(sys.argv[1], encoding="utf-8"))
card = Path(sys.argv[2]).read_text(encoding="utf-8")

def need(cond, msg):
    if not cond:
        raise SystemExit(msg)

token = "MIRBUILDER-STRICT-CONVERTER-EMISSION-NATIVE-SEED-CANDIDATE-SELECTION-001"
need(fixture.get("kind") == "MirBuilderStrictConverterEmissionNativeSeedCandidateSelectionV1", "bad kind")
need(fixture.get("token") == token, "bad token")
need(token in card, "card missing token")

rule = fixture.get("selection_rule") or {}
need(rule.get("manual_family_selection") is False, "manual selection must be false")
need(rule.get("cluster_size_as_proof") is False, "cluster size proof must be false")
need(rule.get("coverage_percentage_as_proof") is False, "coverage proof must be false")
need(rule.get("route_membership_alone_as_proof") is False, "route membership proof must be false")

pool = fixture.get("candidate_pool") or {}
need(pool.get("verified_hako_family_ir_count") == 47, "verified count drift")
need(pool.get("bridge_eligible_count") == 9, "eligible count drift")
need(pool.get("bridge_blocked_count") == 38, "blocked count drift")
need(pool.get("gap_blocked_count") == 36, "gap blocked count drift")

decision = fixture.get("decision") or {}
need(decision.get("kind") == "SelectNativeSeedCandidate", "bad decision kind")
need(decision.get("reason_token") == "StrictEmissionBridgeEligibleCandidateSelected", "bad reason")
need(decision.get("selected_owner_edge_id") == "hakorune_mir_builder::core_context", "bad selected owner")
need(
    decision.get("selected_next_card") == "MIRBUILDER-CORE-CONTEXT-HAKO-NATIVE-SOURCE-SEED-001",
    "bad next card",
)

selected = [
    row for row in fixture.get("candidates") or []
    if row.get("owner_edge_id") == decision.get("selected_owner_edge_id")
    and row.get("bridge_state") == "BridgeEligible"
]
need(len(selected) == 1, "selected owner must have exactly one eligible row")
row = selected[0]
need(row.get("owner_edge_confidence") == "FixtureMapped", "selected owner confidence drift")
need(row.get("deterministic_regeneration") is True, "selected owner needs deterministic regeneration")
need(row.get("provenance_manifest_present") is True, "selected owner needs provenance")
need(row.get("carrier_type_transport_gap") is False, "selected owner must have no carrier/type gap")
need(row.get("composite_owner") is False, "selected owner must not be composite")
need(row.get("already_hako_adopted") is False, "selected owner must not be adopted")

claims = fixture.get("claims") or {}
for key in ["bridge_policy_consumed", "strict_converter_emission_probe_consumed"]:
    need(claims.get(key) == 1, f"{key} must be 1")
for key in [
    "manual_family_selection",
    "generated_artifact_as_native_edit_authority",
    "native_seed_materialization",
    "hako_generation",
    "hako_adopted_decision",
    "source_selfhost_claim",
    "runtime_fallback",
    "new_backend_route",
    "new_abi",
    "new_python_semantic_projector",
    "runner_semantic_owner",
    "cluster_size_as_proof",
    "coverage_percentage_as_proof",
    "route_membership_alone_as_proof",
]:
    need(claims.get(key) == 0, f"{key} must be 0")

print("output_contract=rust-lifecycle-mirbuilder-strict-converter-emission-native-seed-candidate-selection")
print("verified_hako_family_ir_count=47")
print("bridge_eligible_count=9")
print("selected_owner_edge_id=hakorune_mir_builder::core_context")
print("selected_next_card=MIRBUILDER-CORE-CONTEXT-HAKO-NATIVE-SOURCE-SEED-001")
print("manual_family_selection=0")
print("source_selfhost_claim=0")
print("runtime_fallback=0")
print("summary=ok")
PY
