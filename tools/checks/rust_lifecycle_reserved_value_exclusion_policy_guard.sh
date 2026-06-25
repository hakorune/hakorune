#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
cd "$ROOT_DIR"

python3 tools/rust_lifecycle/mirbuilder_reserved_value_exclusion_policy.py \
  --check-reference \
  --drift-probes

cat <<'REPORT'
output_contract=rust-lifecycle-reserved-value-exclusion-policy-plan-v0
reserved_value_exclusion_policy=green
semantic_authority=ReservedValueExclusionSetFacts
allowed_claim=ReservedExclusionPolicyOnly
member_union=PhiDestinations+JoinIrFunctionParameters
observation=MembershipOnly
rejected_candidate_effect=Consumed
retry=GenerateNextCandidate
concrete_representation_claim=0
current_function_composition_claim=0
module_global_fallback_claim=0
generated_hako_artifact=0
backend_behavior_changed=0
runtime_fallback=0
summary=ok
REPORT
