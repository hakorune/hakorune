#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
FIXTURE="$ROOT/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-strict-converter-emission-to-native-seed-bridge-policy-v0.json"
TOOL="$ROOT/tools/rust_lifecycle/mirbuilder_strict_converter_emission_to_native_seed_bridge_policy.py"
CARD="$ROOT/docs/development/current/main/phases/phase-296x/1963-MIRBUILDER-STRICT-CONVERTER-EMISSION-TO-NATIVE-SEED-BRIDGE-POLICY-001.md"

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

token = "MIRBUILDER-STRICT-CONVERTER-EMISSION-TO-NATIVE-SEED-BRIDGE-POLICY-001"
need(fixture.get("kind") == "MirBuilderStrictConverterEmissionToNativeSeedBridgePolicyV1", "bad kind")
need(fixture.get("token") == token, "bad token")
need(token in card, "card missing token")

previous = fixture.get("previous_state") or {}
need(previous.get("rerun_003_decision") == "KeepStopped", "previous decision drift")
need(
    previous.get("rerun_003_reason_token") == "NoMachineDerivedNativeOwnerSeedCandidateAfterStrictEmissionProbe",
    "previous reason drift",
)
need(previous.get("strict_verified_hako_family_ir_count") == 47, "strict verified count drift")
need(previous.get("strict_probe_hako_generation") == 0, "strict probe must not generate Hako")
need(previous.get("strict_probe_rules_changed") == 0, "strict probe must not change rules")

policy = fixture.get("policy") or {}
need(policy.get("generated_artifact_as_native_edit_authority") is False, "generated artifact authority must remain false")
need(policy.get("generated_artifact_as_seed_draft_input") is True, "seed draft input must be allowed")
need(policy.get("seed_draft_input_state_name") == "DerivedArtifactSeedDraftInput", "bad seed draft state")
need(policy.get("native_seed_state_name") == "NativeSourceSeed", "bad native seed state")
need(policy.get("hako_adopted_state_name") == "HakoAdopted", "bad adopted state")
need(policy.get("source_selfhost_claim_allowed") is False, "Source Selfhost claim must remain false")

requirements = fixture.get("candidate_requirements") or {}
for key in [
    "verified_hako_family_ir_present",
    "deterministic_regeneration_present",
    "generated_artifact_provenance_manifest_present",
    "verifier_or_oracle_or_guard_present",
]:
    need(requirements.get(key) is True, f"{key} must be true")
need(requirements.get("owner_edge_confidence_allowed") == ["ExactSymbol", "FixtureMapped"], "bad owner confidence allowlist")
for key in [
    "borrow_policy_gap",
    "carrier_policy_gap",
    "type_transport_gap",
    "route_repairable_inconsistency",
    "runtime_fallback",
    "new_backend_route",
    "new_abi",
    "new_python_semantic_projector",
]:
    need(requirements.get(key) is False, f"{key} must be false")

states = {row.get("state"): row for row in fixture.get("candidate_states") or []}
need("BridgeEligible" in states, "BridgeEligible missing")
need("BridgeBlocked" in states, "BridgeBlocked missing")
need(states["BridgeBlocked"].get("required_reason_token") is True, "BridgeBlocked must require reason")

decision = fixture.get("decision") or {}
need(decision.get("kind") == "PolicyDefined", "bad decision kind")
need(decision.get("reason_token") == "StrictEmissionToNativeSeedBridgePolicyDefined", "bad decision reason")
need(
    decision.get("selected_next_card") == "MIRBUILDER-STRICT-CONVERTER-EMISSION-NATIVE-SEED-CANDIDATE-SELECTION-001",
    "bad next card",
)

claims = fixture.get("claims") or {}
for key in [
    "native_seed_survey_rerun_003_consumed",
    "strict_converter_emission_probe_consumed",
    "previous_keep_stopped_consumed",
    "generated_artifact_as_seed_draft_input",
]:
    need(claims.get(key) == 1, f"{key} must be 1")
need(claims.get("strict_verified_hako_family_ir_count") == 47, "claim verified count drift")
for key in [
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
    "manual_family_selection",
    "cluster_size_as_proof",
    "coverage_percentage_as_proof",
    "route_membership_alone_as_proof",
]:
    need(claims.get(key) == 0, f"{key} must be 0")

print("output_contract=rust-lifecycle-mirbuilder-strict-converter-emission-to-native-seed-bridge-policy")
print("strict_verified_hako_family_ir_count=47")
print("generated_artifact_as_native_edit_authority=0")
print("generated_artifact_as_seed_draft_input=1")
print(f"selected_next_card={decision.get('selected_next_card')}")
print("source_selfhost_claim=0")
print("runtime_fallback=0")
print("summary=ok")
PY
