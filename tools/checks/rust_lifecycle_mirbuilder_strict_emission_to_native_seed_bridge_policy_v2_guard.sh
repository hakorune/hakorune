#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
FIXTURE="$ROOT/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-strict-emission-to-native-seed-bridge-policy-v2-v0.json"
TOOL="$ROOT/tools/rust_lifecycle/mirbuilder_strict_emission_to_native_seed_bridge_policy_v2.py"

python3 "$TOOL" --check

python3 - "$FIXTURE" <<'PY'
import json
import sys
from pathlib import Path

data = json.loads(Path(sys.argv[1]).read_text())

def die(message: str) -> None:
    print(f"[strict-emission-to-native-seed-bridge-policy-v2-guard] {message}", file=sys.stderr)
    raise SystemExit(1)

if data.get("kind") != "MirBuilderStrictEmissionToNativeSeedBridgePolicyV2":
    die("fixture kind mismatch")
if data.get("token") != "MIRBUILDER-STRICT-EMISSION-TO-NATIVE-SEED-BRIDGE-POLICY-V2-001":
    die("fixture token mismatch")

policy = data.get("v2_policy") or {}
if policy.get("policy_id") != "StrictEmissionToNativeSeedBridgePolicyV2":
    die("policy id mismatch")
if policy.get("mention_only_forbidden_nonclaim_is_seed_evidence") is not False:
    die("mention-only forbidden nonclaim must not be seed evidence")
if policy.get("mention_only_forbidden_nonclaim_blocks_clean_narrow_seed_surface") is not False:
    die("mention-only forbidden nonclaim must not block clean narrow seed surface")
for key in [
    "required_forbidden_nonclaim_blocks_seed",
    "unclassified_forbidden_nonclaim_blocks_seed",
]:
    if policy.get(key) is not True:
        die(f"{key} must be true")
for key in [
    "runtime_fallback_allowed",
    "new_backend_route_allowed",
    "new_abi_allowed",
    "new_canonical_mir_instruction_allowed",
]:
    if policy.get(key) is not False:
        die(f"{key} must be false")

summary = data.get("scope_resolution_summary") or {}
expected = {
    "input_owner_edge_count": 3,
    "wider_denied_boundary_mention_only_count": 12,
    "required_by_selected_narrow_seed_surface_count": 0,
    "permanent_forbidden_nonclaim_count": 0,
    "unclassified_forbidden_nonclaim_count": 0,
    "bridge_policy_v2_candidate_count": 3,
}
for key, value in expected.items():
    if summary.get(key) != value:
        die(f"scope summary drift: {key}")

decision = data.get("decision") or {}
if decision.get("kind") != "PolicyDefined":
    die("decision kind mismatch")
if decision.get("selected_next_card") != "MIRBUILDER-STRICT-CONVERTER-EMISSION-NATIVE-SEED-CANDIDATE-SELECTION-RERUN-002":
    die("selected next card mismatch")

claims = data.get("claims") or {}
for key in [
    "bridge_policy_v1_consumed",
    "forbidden_nonclaim_boundary_scope_resolution_consumed",
    "mention_only_forbidden_nonclaim_scope_consumed",
]:
    if claims.get(key) != 1:
        die(f"{key} must be 1")
for key in [
    "seed_eligibility_from_forbidden_nonclaim",
    "runtime_fallback",
    "new_backend_route",
    "new_abi",
    "new_canonical_mir_instruction",
    "new_python_semantic_projector",
    "generated_artifact_as_native_edit_authority",
    "native_seed_materialization",
    "hako_generation",
    "hako_adopted_decision",
    "source_selfhost_claim",
    "runner_semantic_owner",
    "manual_family_selection",
    "manual_boundary_reclassification",
    "cluster_size_as_proof",
    "coverage_percentage_as_proof",
    "route_membership_alone_as_proof",
]:
    if claims.get(key) != 0:
        die(f"claim must remain 0: {key}")

print("[strict-emission-to-native-seed-bridge-policy-v2-guard] OK")
PY
