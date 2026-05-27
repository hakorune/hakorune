#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-phase296x-hako-mimalloc-perf-compiler-allocator-owner-split"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

CARD_57="docs/development/current/main/phases/phase-296x/296x-57-HAKO-MIMALLOC-PERF-COMPILER-ALLOCATOR-OWNER-SPLIT-DIAGNOSTIC.md"
CARD_58="docs/development/current/main/phases/phase-296x/296x-58-HAKO-MIMALLOC-PERF-FIRST-KEEPER-OPTIMIZATION.md"
TASKBOARD="docs/development/current/main/phases/phase-296x/296x-90-mimalloc-benchmark-taskboard.md"
CURRENT_STATE="docs/development/current/main/CURRENT_STATE.toml"
INDEX="docs/tools/check-scripts-index.md"
APP="apps/hako-alloc-mimalloc-comparison-in-process-loop-shell-proof/main.hako"
SPLIT="tools/allocator/hako_mimalloc_compiler_allocator_owner_split.py"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_hako_mimalloc_perf_compiler_allocator_owner_split_guard.sh"

echo "[$TAG] checking phase-296x compiler/allocator owner split"

guard_require_files "$TAG" "$CARD_57" "$CARD_58" "$TASKBOARD" "$CURRENT_STATE" "$INDEX" "$APP" "$SPLIT" "$SELF_SCRIPT"
guard_require_exec_files "$TAG" "$SPLIT" "$SELF_SCRIPT"

guard_expect_fixed_in_file "$TAG" 'Status: Landed' "$CARD_57" "owner split card must be landed"
guard_expect_fixed_in_file "$TAG" 'Status: Current' "$CARD_58" "first keeper optimization card must be current"
guard_expect_fixed_in_file "$TAG" 'output_contract=hako-mimalloc-compiler-allocator-owner-split-v0' "$CARD_57" "card must define owner split contract"
guard_expect_fixed_in_file "$TAG" 'selected_gap_owner=allocator_algorithm' "$CARD_57" "card must select allocator owner"
guard_expect_fixed_in_file "$TAG" 'selected_gap_confidence=high' "$CARD_57" "card must select high confidence"
guard_expect_fixed_in_file "$TAG" 'next_optimization_allowed=1' "$CARD_57" "card must open optimization"
guard_expect_fixed_in_file "$TAG" 'optimization_started=0' "$CARD_57" "card must not optimize"

guard_expect_fixed_in_file "$TAG" 'latest_card = "296x-57-HAKO-MIMALLOC-PERF-COMPILER-ALLOCATOR-OWNER-SPLIT-DIAGNOSTIC"' "$CURRENT_STATE" "current state latest card must advance to row 57"
guard_expect_fixed_in_file "$TAG" 'current_blocker_token = "HAKO-MIMALLOC-PERF-FIRST-KEEPER-OPTIMIZATION-296X-001"' "$CURRENT_STATE" "current state must select row 58"
guard_expect_fixed_in_file "$TAG" '| 57 | `HAKO-MIMALLOC-PERF-COMPILER-ALLOCATOR-OWNER-SPLIT-DIAGNOSTIC-296X-001` | Landed |' "$TASKBOARD" "taskboard row 57 must be landed"
guard_expect_fixed_in_file "$TAG" '| 58 | `HAKO-MIMALLOC-PERF-FIRST-KEEPER-OPTIMIZATION-296X-001` | Current |' "$TASKBOARD" "taskboard row 58 must be current"
guard_expect_fixed_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list this guard"
guard_expect_fixed_in_file "$TAG" "$SPLIT" "$INDEX" "check index must list owner split tool"

tmp_dir="$(mktemp -d /tmp/hakorune_phase296x_owner_split.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT
measurement="$tmp_dir/measurement.out"
shell="$tmp_dir/shell.out"
split="$tmp_dir/split.out"

cat >"$measurement" <<'EOF'
output_contract=hako-mimalloc-in-process-operation-repeat-measurement-v0
timing_repeat_kind=in-process-operation-loop-v0
process_invocation_repeat=0
operation_repeat=8192
hako_external_elapsed_median_ms=330
c_external_elapsed_median_ms=4
external_elapsed_median_gap_ms=326
winner_claim=0
provider_active=0
replacement_active=0
hook_installed=0
global_allocator=0
summary=ok
EOF
cat >"$shell" <<'EOF'
output_contract=hako-exe-memory-evidence-v0
workload=representative-loop-shell-v0
in_process_operation_repeat=8192
app_timing_repeat_kind=in-process-operation-loop-v0
external_elapsed_ms=1
summary=ok
EOF

python3 "$SPLIT" --measurement "$measurement" --shell-report "$shell" --out "$split"

guard_expect_fixed_in_file "$TAG" 'output_contract=hako-mimalloc-compiler-allocator-owner-split-v0' "$split" "split tool must emit contract"
guard_expect_fixed_in_file "$TAG" 'shell_hako_external_elapsed_median_ms=1' "$split" "split tool must record shell median"
guard_expect_fixed_in_file "$TAG" 'shell_explains_hako_ratio_pct=0' "$split" "split tool must record shell ratio"
guard_expect_fixed_in_file "$TAG" 'selected_gap_owner=allocator_algorithm' "$split" "split tool must select allocator owner"
guard_expect_fixed_in_file "$TAG" 'selected_gap_confidence=high' "$split" "split tool must select high confidence"
guard_expect_fixed_in_file "$TAG" 'selected_next_row=HAKO-MIMALLOC-PERF-FIRST-KEEPER-OPTIMIZATION-296X-001' "$split" "split tool must select first optimization"
guard_expect_fixed_in_file "$TAG" 'next_optimization_allowed=1' "$split" "split tool must allow optimization"
guard_expect_fixed_in_file "$TAG" 'optimization_started=0' "$split" "split tool must not optimize"
guard_expect_fixed_in_file "$TAG" 'winner_claim=0' "$split" "split tool must keep winner closed"
guard_expect_fixed_in_file "$TAG" 'provider_active=0' "$split" "split tool must keep provider closed"
guard_expect_fixed_in_file "$TAG" 'replacement_active=0' "$split" "split tool must keep replacement closed"
guard_expect_fixed_in_file "$TAG" 'summary=ok' "$split" "split tool must end ok"

echo "[$TAG] ok"
