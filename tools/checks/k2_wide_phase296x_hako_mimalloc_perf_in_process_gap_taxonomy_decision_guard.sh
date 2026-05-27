#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-phase296x-hako-mimalloc-perf-in-process-gap-taxonomy-decision"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

CARD_56="docs/development/current/main/phases/phase-296x/296x-56-HAKO-MIMALLOC-PERF-IN-PROCESS-GAP-TAXONOMY-DECISION.md"
CARD_57="docs/development/current/main/phases/phase-296x/296x-57-HAKO-MIMALLOC-PERF-COMPILER-ALLOCATOR-OWNER-SPLIT-DIAGNOSTIC.md"
CARD_58="docs/development/current/main/phases/phase-296x/296x-58-HAKO-MIMALLOC-PERF-FIRST-KEEPER-OPTIMIZATION.md"
TASKBOARD="docs/development/current/main/phases/phase-296x/296x-90-mimalloc-benchmark-taskboard.md"
CURRENT_STATE="docs/development/current/main/CURRENT_STATE.toml"
INDEX="docs/tools/check-scripts-index.md"
DECISION="tools/allocator/hako_mimalloc_in_process_gap_taxonomy_decision.py"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_hako_mimalloc_perf_in_process_gap_taxonomy_decision_guard.sh"

echo "[$TAG] checking phase-296x in-process gap taxonomy decision"

guard_require_files "$TAG" "$CARD_56" "$CARD_57" "$CARD_58" "$TASKBOARD" "$CURRENT_STATE" "$INDEX" "$DECISION" "$SELF_SCRIPT"
guard_require_exec_files "$TAG" "$DECISION" "$SELF_SCRIPT"

guard_expect_fixed_in_file "$TAG" 'Status: Landed' "$CARD_56" "taxonomy decision card must be landed"
guard_expect_fixed_in_file "$TAG" 'Status: Current' "$CARD_57" "owner split card must be current"
guard_expect_fixed_in_file "$TAG" 'Status: Planned' "$CARD_58" "first keeper optimization card must be planned"
guard_expect_fixed_in_file "$TAG" 'output_contract=hako-mimalloc-in-process-gap-taxonomy-decision-v0' "$CARD_56" "card must define decision contract"
guard_expect_fixed_in_file "$TAG" 'gap_owner=allocator_algorithm' "$CARD_56" "card must classify allocator owner"
guard_expect_fixed_in_file "$TAG" 'gap_confidence=low' "$CARD_56" "card must keep low confidence"
guard_expect_fixed_in_file "$TAG" 'next_diagnostic=compiler_allocator_owner_split_diagnostic' "$CARD_56" "card must select owner split"
guard_expect_fixed_in_file "$TAG" 'next_optimization_allowed=0' "$CARD_56" "card must keep optimization closed"

guard_expect_fixed_in_file "$TAG" 'latest_card = "296x-56-HAKO-MIMALLOC-PERF-IN-PROCESS-GAP-TAXONOMY-DECISION"' "$CURRENT_STATE" "current state latest card must advance to row 56"
guard_expect_fixed_in_file "$TAG" 'current_blocker_token = "HAKO-MIMALLOC-PERF-COMPILER-ALLOCATOR-OWNER-SPLIT-DIAGNOSTIC-296X-001"' "$CURRENT_STATE" "current state must select row 57"
guard_expect_fixed_in_file "$TAG" '| 56 | `HAKO-MIMALLOC-PERF-IN-PROCESS-GAP-TAXONOMY-DECISION-296X-001` | Landed |' "$TASKBOARD" "taskboard row 56 must be landed"
guard_expect_fixed_in_file "$TAG" '| 57 | `HAKO-MIMALLOC-PERF-COMPILER-ALLOCATOR-OWNER-SPLIT-DIAGNOSTIC-296X-001` | Current |' "$TASKBOARD" "taskboard row 57 must be current"
guard_expect_fixed_in_file "$TAG" '| 58 | `HAKO-MIMALLOC-PERF-FIRST-KEEPER-OPTIMIZATION-296X-001` | Planned |' "$TASKBOARD" "taskboard row 58 must be planned"
guard_expect_fixed_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list this guard"
guard_expect_fixed_in_file "$TAG" "$DECISION" "$INDEX" "check index must list decision tool"

tmp_dir="$(mktemp -d /tmp/hakorune_phase296x_in_process_taxonomy.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT
measurement="$tmp_dir/measurement.out"
decision="$tmp_dir/decision.out"

cat >"$measurement" <<'EOF'
output_contract=hako-mimalloc-in-process-operation-repeat-measurement-v0
measurement_profile=hako-mimalloc-in-process-operation-repeat-v0
timing_repeat_kind=in-process-operation-loop-v0
workload_id=representative-small-block-v0
operation_repeat=8192
process_repeat=3
same_workload=1
same_operation_count=1
process_invocation_repeat=0
hako_external_elapsed_median_ms=330
c_external_elapsed_median_ms=4
c_body_elapsed_median_ns=3240447
external_elapsed_median_gap_ms=326
winner_claim=0
provider_active=0
replacement_active=0
hook_installed=0
global_allocator=0
summary=ok
EOF

python3 "$DECISION" --input "$measurement" --out "$decision"

guard_expect_fixed_in_file "$TAG" 'output_contract=hako-mimalloc-in-process-gap-taxonomy-decision-v0' "$decision" "decision tool must emit contract"
guard_expect_fixed_in_file "$TAG" 'gap_owner=allocator_algorithm' "$decision" "decision tool must classify allocator owner"
guard_expect_fixed_in_file "$TAG" 'gap_confidence=low' "$decision" "decision tool must keep low confidence"
guard_expect_fixed_in_file "$TAG" 'next_diagnostic=compiler_allocator_owner_split_diagnostic' "$decision" "decision tool must select owner split"
guard_expect_fixed_in_file "$TAG" 'next_optimization_allowed=0' "$decision" "decision tool must block optimization"
guard_expect_fixed_in_file "$TAG" 'optimization_started=0' "$decision" "decision tool must not optimize"
guard_expect_fixed_in_file "$TAG" 'winner_claim=0' "$decision" "decision tool must keep winner closed"
guard_expect_fixed_in_file "$TAG" 'provider_active=0' "$decision" "decision tool must keep provider closed"
guard_expect_fixed_in_file "$TAG" 'replacement_active=0' "$decision" "decision tool must keep replacement closed"
guard_expect_fixed_in_file "$TAG" 'summary=ok' "$decision" "decision tool must end ok"

echo "[$TAG] ok"
