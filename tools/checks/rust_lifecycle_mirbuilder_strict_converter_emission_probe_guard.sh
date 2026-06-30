#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
FIXTURE="$ROOT/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-strict-converter-emission-probe-v0.json"
TOOL="$ROOT/tools/rust_lifecycle/mirbuilder_strict_converter_emission_probe.py"

python3 "$TOOL" --check

python3 - "$FIXTURE" <<'PY'
import json
import sys

data = json.load(open(sys.argv[1], encoding="utf-8"))

def need(cond, msg):
    if not cond:
        raise SystemExit(msg)

need(data.get("kind") == "MirBuilderStrictConverterEmissionProbeV1", "bad kind")
need(data.get("token") == "MIRBUILDER-STRICT-CONVERTER-EMISSION-PROBE-001", "bad token")

scope = data.get("probe_scope") or {}
need(scope.get("source") == "existing verifier-result fixtures only", "bad probe source")
for key in ["emits_hako", "constructs_verified_hako_family_ir", "weakens_strict_rules"]:
    need(scope.get(key) is False, f"{key} must be false")

summary = data.get("summary") or {}
need(summary.get("verified_hako_family_ir_count") == 47, "verified fixture count drift")
need(summary.get("carrier_type_transport_candidate_count") == 125, "carrier/type count drift")
need(summary.get("policy_lane_selected_count") == 0, "policy lane must not be selected")
need(len(data.get("verified_hako_family_ir_fixtures") or []) == 47, "verified fixture list length drift")
for row in data.get("verified_hako_family_ir_fixtures") or []:
    need(row.get("result") == "VerifiedHakoFamilyIR", "non-verified result in verified list")
    need(row.get("fixture", "").endswith("verifier-result-v0.json"), "verified row must point at verifier fixture")

decision = data.get("decision") or {}
need(decision.get("kind") == "SelectNativeOwnerSeedCapabilitySurveyRerun", "bad decision kind")
need(
    decision.get("selected_next_card") == "MIRBUILDER-CRATE-WIDE-NATIVE-OWNER-SEED-CAPABILITY-SURVEY-RERUN-003",
    "bad next card",
)

claims = data.get("claims") or {}
for key in [
    "carrier_type_transport_inventory_consumed",
    "existing_verifier_results_consumed",
    "strict_emission_probe_ready",
]:
    need(claims.get(key) == 1, f"{key} must be 1")
for key in [
    "hako_generation",
    "verified_hako_family_ir_constructed_by_probe",
    "strict_rules_changed",
    "fallback_hako_emission",
    "hako_adopted_decision",
    "native_seed_materialization",
    "source_selfhost_claim",
    "runtime_fallback",
    "new_backend_route",
    "new_abi",
    "new_python_semantic_projector",
    "runner_semantic_owner",
]:
    need(claims.get(key) == 0, f"{key} must be 0")

print("output_contract=rust-lifecycle-mirbuilder-strict-converter-emission-probe")
print("verified_hako_family_ir_count=47")
print("hako_generation=0")
print("strict_rules_changed=0")
print(f"selected_next_card={decision.get('selected_next_card')}")
print("source_selfhost_claim=0")
print("runtime_fallback=0")
print("summary=ok")
PY
