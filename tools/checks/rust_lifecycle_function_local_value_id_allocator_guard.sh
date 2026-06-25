#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
cd "$ROOT_DIR"

python3 tools/rust_lifecycle/mirbuilder_function_local_value_id_allocator.py \
  --check-reference \
  --drift-probes

cat <<'REPORT'
output_contract=rust-lifecycle-function-local-value-id-allocator-plan-v0
function_local_value_id_allocator=green
semantic_authority=FunctionAllocatorFacts
allowed_claim=FunctionLocalAllocatorOnly
reserved_exclusion_set_retry_claim=0
current_function_composition_claim=0
module_global_fallback_claim=0
formal_invalid_sentinel_exclusion_claim=0
generated_hako_artifact=0
backend_behavior_changed=0
runtime_fallback=0
summary=ok
REPORT
