#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-phase296x-hako-mimalloc-perf-owner-confidence-refresh"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

CARD_51="docs/development/current/main/phases/phase-296x/296x-51-HAKO-MIMALLOC-PERF-OWNER-CONFIDENCE-REFRESH.md"
CARD_52="docs/development/current/main/phases/phase-296x/296x-52-HAKO-MIMALLOC-PERF-RUNTIME-BASELINE-SCALING-DIAGNOSTIC.md"
CARD_53="docs/development/current/main/phases/phase-296x/296x-53-HAKO-MIMALLOC-PERF-FIRST-KEEPER-OPTIMIZATION.md"
TASKBOARD="docs/development/current/main/phases/phase-296x/296x-90-mimalloc-benchmark-taskboard.md"
CURRENT_STATE="docs/development/current/main/CURRENT_STATE.toml"
INDEX="docs/tools/check-scripts-index.md"
REFRESH="tools/allocator/hako_mimalloc_owner_confidence_refresh.py"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_hako_mimalloc_perf_owner_confidence_refresh_guard.sh"

echo "[$TAG] checking phase-296x owner confidence refresh"

guard_require_files "$TAG" "$CARD_51" "$CARD_52" "$CARD_53" "$TASKBOARD" "$CURRENT_STATE" "$INDEX" "$REFRESH" "$SELF_SCRIPT"
guard_require_exec_files "$TAG" "$REFRESH" "$SELF_SCRIPT"

guard_expect_fixed_in_file "$TAG" 'Status: Landed' "$CARD_51" "owner confidence refresh card must be landed"
guard_expect_fixed_in_file "$TAG" 'Status: Current' "$CARD_52" "runtime scaling diagnostic card must be current"
guard_expect_fixed_in_file "$TAG" 'Status: Planned' "$CARD_53" "first keeper optimization card must be planned"
guard_expect_fixed_in_file "$TAG" 'output_contract=hako-mimalloc-owner-confidence-refresh-v0' "$CARD_51" "row 51 card must define confidence refresh contract"
guard_expect_fixed_in_file "$TAG" 'next_diagnostic=repeat_scaling_runtime_diagnostic' "$CARD_51" "row 51 card must select repeat scaling diagnostic"
guard_expect_fixed_in_file "$TAG" 'next_optimization_allowed=0' "$CARD_51" "row 51 card must keep optimization closed"

guard_expect_fixed_in_file "$TAG" 'latest_card = "296x-51-HAKO-MIMALLOC-PERF-OWNER-CONFIDENCE-REFRESH"' "$CURRENT_STATE" "current state latest card must advance to row 51"
guard_expect_fixed_in_file "$TAG" 'current_blocker_token = "HAKO-MIMALLOC-PERF-RUNTIME-BASELINE-SCALING-DIAGNOSTIC-296X-001"' "$CURRENT_STATE" "current state must select row 52"
guard_expect_fixed_in_file "$TAG" '| 51 | `HAKO-MIMALLOC-PERF-OWNER-CONFIDENCE-REFRESH-296X-001` | Landed |' "$TASKBOARD" "taskboard row 51 must be landed"
guard_expect_fixed_in_file "$TAG" '| 52 | `HAKO-MIMALLOC-PERF-RUNTIME-BASELINE-SCALING-DIAGNOSTIC-296X-001` | Current |' "$TASKBOARD" "taskboard row 52 must be current"
guard_expect_fixed_in_file "$TAG" '| 53 | `HAKO-MIMALLOC-PERF-FIRST-KEEPER-OPTIMIZATION-296X-001` | Planned |' "$TASKBOARD" "taskboard row 53 must be planned"
guard_expect_fixed_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list this guard"
guard_expect_fixed_in_file "$TAG" "$REFRESH" "$INDEX" "check index must list confidence refresh tool"

tmp_dir="$(mktemp -d /tmp/hakorune_phase296x_owner_confidence.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT
taxonomy="$tmp_dir/taxonomy.out"
empty="$tmp_dir/empty.out"
confidence="$tmp_dir/confidence.out"

cat >"$taxonomy" <<'EOF'
output_contract=hako-mimalloc-gap-taxonomy-v0
workload_id=representative-small-block-v0
elapsed_median_gap_ms=10
gap_owner=hako_runtime_baseline
gap_confidence=low
winner_claim=0
provider_active=0
replacement_active=0
hook_installed=0
global_allocator=0
summary=ok
EOF

cat >"$empty" <<'EOF'
output_contract=mimalloc-comparison-repeated-measurement-v0
workload_0_id=representative-empty-v0
workload_0_sample_count=5
workload_0_hako_external_elapsed_median_ms=80
workload_0_c_external_elapsed_median_ms=70
provider_activation=0
host_replacement=0
hook_installed=0
global_allocator_installed=0
winner_claim=0
summary=ok
EOF

python3 "$REFRESH" --taxonomy "$taxonomy" --empty-report "$empty" --out "$confidence"

guard_expect_fixed_in_file "$TAG" 'output_contract=hako-mimalloc-owner-confidence-refresh-v0' "$confidence" "confidence refresh must emit contract"
guard_expect_fixed_in_file "$TAG" 'confidence_refresh_kind=empty_workload_runtime_baseline' "$confidence" "confidence refresh must use empty workload baseline"
guard_expect_fixed_in_file "$TAG" 'empty_workload_id=representative-empty-v0' "$confidence" "confidence refresh must preserve empty workload"
guard_expect_fixed_in_file "$TAG" 'refreshed_gap_owner=hako_runtime_baseline' "$confidence" "empty baseline gap must keep runtime owner"
guard_expect_fixed_in_file "$TAG" 'refreshed_gap_confidence=medium' "$confidence" "empty baseline gap must raise confidence"
guard_expect_fixed_in_file "$TAG" 'next_diagnostic=repeat_scaling_runtime_diagnostic' "$confidence" "confidence refresh must select runtime scaling"
guard_expect_fixed_in_file "$TAG" 'next_optimization_allowed=0' "$confidence" "confidence refresh must block optimization"
guard_expect_fixed_in_file "$TAG" 'winner_claim=0' "$confidence" "confidence refresh must keep winner claims closed"
guard_expect_fixed_in_file "$TAG" 'provider_active=0' "$confidence" "confidence refresh must keep provider closed"
guard_expect_fixed_in_file "$TAG" 'replacement_active=0' "$confidence" "confidence refresh must keep replacement closed"
guard_expect_fixed_in_file "$TAG" 'hook_installed=0' "$confidence" "confidence refresh must keep hooks closed"
guard_expect_fixed_in_file "$TAG" 'global_allocator=0' "$confidence" "confidence refresh must keep global allocator closed"
guard_expect_fixed_in_file "$TAG" 'summary=ok' "$confidence" "confidence refresh must end ok"

echo "[$TAG] ok"
