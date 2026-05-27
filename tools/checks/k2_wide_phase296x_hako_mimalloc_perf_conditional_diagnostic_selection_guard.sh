#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-phase296x-hako-mimalloc-perf-conditional-diagnostic-selection"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

CARD_46="docs/development/current/main/phases/phase-296x/296x-46-HAKO-MIMALLOC-PERF-CONDITIONAL-DIAGNOSTIC-SELECTION.md"
CARD_47="docs/development/current/main/phases/phase-296x/296x-47-HAKO-MIMALLOC-PERF-OWNER-NARROW-DIAGNOSTIC.md"
TASKBOARD="docs/development/current/main/phases/phase-296x/296x-90-mimalloc-benchmark-taskboard.md"
CURRENT_STATE="docs/development/current/main/CURRENT_STATE.toml"
INDEX="docs/tools/check-scripts-index.md"
SELECTOR="tools/allocator/hako_mimalloc_conditional_diagnostic_selector.py"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_hako_mimalloc_perf_conditional_diagnostic_selection_guard.sh"

echo "[$TAG] checking phase-296x conditional diagnostic selection"

guard_require_files "$TAG" "$CARD_46" "$CARD_47" "$TASKBOARD" "$CURRENT_STATE" "$INDEX" "$SELECTOR" "$SELF_SCRIPT"
guard_require_exec_files "$TAG" "$SELECTOR" "$SELF_SCRIPT"

guard_expect_fixed_in_file "$TAG" 'Status: Landed' "$CARD_46" "conditional diagnostic card must be landed"
guard_expect_fixed_in_file "$TAG" 'Status: Current' "$CARD_47" "owner narrow diagnostic card must be current"
guard_expect_fixed_in_file "$TAG" 'output_contract=hako-mimalloc-conditional-diagnostic-selection-v0' "$CARD_46" "card must define selector contract"
guard_expect_fixed_in_file "$TAG" 'selected_next_row=HAKO-MIMALLOC-PERF-OWNER-NARROW-DIAGNOSTIC-296X-001' "$CARD_46" "card must select row 47"
guard_expect_fixed_in_file "$TAG" 'body_elapsed_ns_primary=0' "$CARD_46" "card must keep body elapsed secondary"

guard_expect_fixed_in_file "$TAG" 'latest_card = "296x-46-HAKO-MIMALLOC-PERF-CONDITIONAL-DIAGNOSTIC-SELECTION"' "$CURRENT_STATE" "current state latest card must advance to row 46"
guard_expect_fixed_in_file "$TAG" 'current_blocker_token = "HAKO-MIMALLOC-PERF-OWNER-NARROW-DIAGNOSTIC-296X-001"' "$CURRENT_STATE" "current state must select row 47"
guard_expect_fixed_in_file "$TAG" '| 46 | `HAKO-MIMALLOC-PERF-CONDITIONAL-DIAGNOSTIC-SELECTION-296X-001` | Landed |' "$TASKBOARD" "taskboard row 46 must be landed"
guard_expect_fixed_in_file "$TAG" '| 47 | `HAKO-MIMALLOC-PERF-OWNER-NARROW-DIAGNOSTIC-296X-001` | Current |' "$TASKBOARD" "taskboard row 47 must be current"
guard_expect_fixed_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list this guard"
guard_expect_fixed_in_file "$TAG" "$SELECTOR" "$INDEX" "check index must list selector"

tmp_dir="$(mktemp -d /tmp/hakorune_phase296x_conditional_diagnostic.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT
taxonomy_in="$tmp_dir/taxonomy.out"
selection_out="$tmp_dir/selection.out"
stable_in="$tmp_dir/stable-taxonomy.out"
stable_out="$tmp_dir/stable-selection.out"

cat >"$taxonomy_in" <<'EOF'
output_contract=hako-mimalloc-gap-taxonomy-v0
input_contract=mimalloc-comparison-repeated-measurement-v0
workload_id=representative-small-block-v0
measurement_profile=phase295x-repeated-v0
outlier_observed=1
evidence_quality=noisy
gap_owner=benchmark_harness
gap_confidence=medium
next_diagnostic=measurement_hygiene_refresh
next_optimization_allowed=0
winner_claim=0
provider_active=0
replacement_active=0
hook_installed=0
global_allocator=0
summary=ok
EOF

python3 "$SELECTOR" --input "$taxonomy_in" --out "$selection_out"
guard_expect_fixed_in_file "$TAG" 'output_contract=hako-mimalloc-conditional-diagnostic-selection-v0' "$selection_out" "selector must emit contract"
guard_expect_fixed_in_file "$TAG" 'gap_owner=benchmark_harness' "$selection_out" "selector must preserve owner"
guard_expect_fixed_in_file "$TAG" 'evidence_quality=noisy' "$selection_out" "selector must preserve evidence quality"
guard_expect_fixed_in_file "$TAG" 'selected_diagnostic=measurement_hygiene_refresh' "$selection_out" "noisy evidence must select hygiene refresh"
guard_expect_fixed_in_file "$TAG" 'measurement_hygiene_required=1' "$selection_out" "noisy evidence must require hygiene"
guard_expect_fixed_in_file "$TAG" 'next_optimization_allowed=0' "$selection_out" "noisy evidence must block optimization"
guard_expect_fixed_in_file "$TAG" 'body_elapsed_ns_primary=0' "$selection_out" "selector must keep body elapsed secondary"
guard_expect_fixed_in_file "$TAG" 'winner_claim=0' "$selection_out" "selector must keep winner claims closed"
guard_expect_fixed_in_file "$TAG" 'provider_active=0' "$selection_out" "selector must keep provider closed"
guard_expect_fixed_in_file "$TAG" 'replacement_active=0' "$selection_out" "selector must keep replacement closed"
guard_expect_fixed_in_file "$TAG" 'hook_installed=0' "$selection_out" "selector must keep hook closed"
guard_expect_fixed_in_file "$TAG" 'global_allocator=0' "$selection_out" "selector must keep global allocator closed"
guard_expect_fixed_in_file "$TAG" 'summary=ok' "$selection_out" "selector must end ok"

cat >"$stable_in" <<'EOF'
output_contract=hako-mimalloc-gap-taxonomy-v0
input_contract=mimalloc-comparison-repeated-measurement-v0
workload_id=representative-small-block-v0
measurement_profile=phase295x-repeated-v0
outlier_observed=0
evidence_quality=stable
gap_owner=allocator_algorithm
gap_confidence=medium
next_diagnostic=operation_repeat_scaling_or_allocator_counter_diagnostic
next_optimization_allowed=1
winner_claim=0
provider_active=0
replacement_active=0
hook_installed=0
global_allocator=0
summary=ok
EOF

python3 "$SELECTOR" --input "$stable_in" --out "$stable_out"
guard_expect_fixed_in_file "$TAG" 'selected_diagnostic=operation_repeat_scaling_or_allocator_counter_diagnostic' "$stable_out" "stable allocator evidence must select allocator diagnostic"
guard_expect_fixed_in_file "$TAG" 'measurement_hygiene_required=0' "$stable_out" "stable allocator evidence must not require hygiene"
guard_expect_fixed_in_file "$TAG" 'next_optimization_allowed=1' "$stable_out" "stable allocator evidence may allow next optimization"
guard_expect_fixed_in_file "$TAG" 'selected_next_row=HAKO-MIMALLOC-PERF-OWNER-NARROW-DIAGNOSTIC-296X-001' "$stable_out" "selector must select row 47"

echo "[$TAG] ok"
