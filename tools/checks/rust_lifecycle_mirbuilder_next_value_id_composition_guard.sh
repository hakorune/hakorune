#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
cd "$ROOT_DIR"

python3 tools/rust_lifecycle/mirbuilder_next_value_id_composition.py \
  --check-reference \
  --drift-probes

cat <<'REPORT'
output_contract=rust-lifecycle-mirbuilder-next-value-id-composition-plan-v0
mirbuilder_next_value_id_composition=green
semantic_authority=ResolvedValueAllocationPolicyV1
subplan_function_local=FunctionLocalValueIdAllocatorPlanV1
subplan_reserved_exclusion=ReservedValueExclusionPolicyPlanV1
allocator_selector=CurrentFunctionPresent
rejected_candidate_effect=Consumed
retry=GenerateNextCandidate
generated_hako_artifact=0
backend_behavior_changed=0
runtime_fallback=0
summary=ok
REPORT
