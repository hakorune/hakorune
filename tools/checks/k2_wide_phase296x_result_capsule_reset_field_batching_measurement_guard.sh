#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
DOC="$ROOT_DIR/docs/development/current/main/phases/phase-296x/296x-259-RESULT-CAPSULE-RESET-FIELD-BATCHING-MEASUREMENT.md"
PREV="$ROOT_DIR/docs/development/current/main/phases/phase-296x/296x-258-RESULT-CAPSULE-RESET-FIELD-BATCHING-IMPLEMENTATION.md"

require_line() {
  local file="$1"
  local expected="$2"
  if ! grep -q "^${expected}$" "$file"; then
    echo "[row259-result-capsule-reset-measure] missing line in ${file#$ROOT_DIR/}: $expected" >&2
    exit 1
  fi
}

require_line "$DOC" "Status: Landed"
require_line "$PREV" "Status: Landed"
require_line "$DOC" "output_contract=result-capsule-reset-field-batching-measurement-v0"
require_line "$DOC" "input_contract=result-capsule-reset-field-batching-implementation-v0"
require_line "$DOC" "base_measurement_contract=typed-object-exact-slot-direct-helper-measurement-v0"
require_line "$DOC" "workload_id=representative-object-lifecycle-small-block-v0"
require_line "$DOC" "measurement_scope=object_lifecycle_exact_exe_after_result_capsule_reset_batching"
require_line "$DOC" "sample_count=3"
require_line "$DOC" "typed_object_backend=single_thread_exact"
require_line "$DOC" "array_slot_backend=single_thread_exact"
require_line "$DOC" "single_thread_exact_floor_body_elapsed_ns=113000000"
require_line "$DOC" "result_capsule_reset_batching_body_elapsed_ns=110000000"
require_line "$DOC" "body_elapsed_delta_ns=3000000"
require_line "$DOC" "result_capsule_reset_batching_body_ratio_pct=97"
require_line "$DOC" "keeper_acceptance_min_improvement_pct=3"
require_line "$DOC" "keeper_effect=accepted"
require_line "$DOC" "result_capsule_reset_batching_keeper=1"
require_line "$DOC" "winner_claim=0"
require_line "$DOC" "replacement_active=0"
require_line "$DOC" "hook_installed=0"
require_line "$DOC" "global_allocator=0"
require_line "$DOC" "summary=ok"

cat <<REPORT
output_contract=result-capsule-reset-field-batching-measurement-v0
input_contract=result-capsule-reset-field-batching-implementation-v0
single_thread_exact_floor_body_elapsed_ns=113000000
result_capsule_reset_batching_body_elapsed_ns=110000000
body_elapsed_delta_ns=3000000
keeper_effect=accepted
result_capsule_reset_batching_keeper=1
winner_claim=0
replacement_active=0
hook_installed=0
global_allocator=0
summary=ok
REPORT
