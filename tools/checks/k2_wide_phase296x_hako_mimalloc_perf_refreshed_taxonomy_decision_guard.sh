#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-phase296x-hako-mimalloc-perf-refreshed-taxonomy-decision"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

CARD_50="docs/development/current/main/phases/phase-296x/296x-50-HAKO-MIMALLOC-PERF-REFRESHED-TAXONOMY-DECISION.md"
CARD_51="docs/development/current/main/phases/phase-296x/296x-51-HAKO-MIMALLOC-PERF-FIRST-KEEPER-OPTIMIZATION.md"
TASKBOARD="docs/development/current/main/phases/phase-296x/296x-90-mimalloc-benchmark-taskboard.md"
CURRENT_STATE="docs/development/current/main/CURRENT_STATE.toml"
INDEX="docs/tools/check-scripts-index.md"
DECISION="tools/allocator/hako_mimalloc_refreshed_taxonomy_decision.py"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_hako_mimalloc_perf_refreshed_taxonomy_decision_guard.sh"

echo "[$TAG] checking phase-296x refreshed taxonomy decision"

guard_require_files "$TAG" "$CARD_50" "$CARD_51" "$TASKBOARD" "$CURRENT_STATE" "$INDEX" "$DECISION" "$SELF_SCRIPT"
guard_require_exec_files "$TAG" "$DECISION" "$SELF_SCRIPT"

guard_expect_fixed_in_file "$TAG" 'Status: Landed' "$CARD_50" "refreshed taxonomy decision card must be landed"
guard_expect_fixed_in_file "$TAG" 'Status: Current' "$CARD_51" "first keeper optimization card must be current"
guard_expect_fixed_in_file "$TAG" 'output_contract=hako-mimalloc-refreshed-taxonomy-decision-v0' "$CARD_50" "card must define decision contract"
guard_expect_fixed_in_file "$TAG" 'HAKO-MIMALLOC-PERF-FIRST-KEEPER-OPTIMIZATION-296X-001' "$CARD_50" "card must select optimization conditionally"

guard_expect_fixed_in_file "$TAG" 'latest_card = "296x-50-HAKO-MIMALLOC-PERF-REFRESHED-TAXONOMY-DECISION"' "$CURRENT_STATE" "current state latest card must advance to row 50"
guard_expect_fixed_in_file "$TAG" 'current_blocker_token = "HAKO-MIMALLOC-PERF-FIRST-KEEPER-OPTIMIZATION-296X-001"' "$CURRENT_STATE" "current state must select row 51"
guard_expect_fixed_in_file "$TAG" '| 50 | `HAKO-MIMALLOC-PERF-REFRESHED-TAXONOMY-DECISION-296X-001` | Landed |' "$TASKBOARD" "taskboard row 50 must be landed"
guard_expect_fixed_in_file "$TAG" '| 51 | `HAKO-MIMALLOC-PERF-FIRST-KEEPER-OPTIMIZATION-296X-001` | Current |' "$TASKBOARD" "taskboard row 51 must be current"
guard_expect_fixed_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list this guard"
guard_expect_fixed_in_file "$TAG" "$DECISION" "$INDEX" "check index must list decision tool"

tmp_dir="$(mktemp -d /tmp/hakorune_phase296x_refreshed_decision.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT
stable="$tmp_dir/stable.out"
stable_decision="$tmp_dir/stable-decision.out"
noisy="$tmp_dir/noisy.out"
noisy_decision="$tmp_dir/noisy-decision.out"

cat >"$stable" <<'EOF'
output_contract=hako-mimalloc-gap-taxonomy-v0
workload_id=representative-small-block-v0
gap_owner=allocator_algorithm
evidence_quality=stable
gap_confidence=medium
next_diagnostic=operation_repeat_scaling_or_allocator_counter_diagnostic
winner_claim=0
provider_active=0
replacement_active=0
hook_installed=0
global_allocator=0
summary=ok
EOF

python3 "$DECISION" --input "$stable" --out "$stable_decision"
guard_expect_fixed_in_file "$TAG" 'output_contract=hako-mimalloc-refreshed-taxonomy-decision-v0' "$stable_decision" "stable decision must emit contract"
guard_expect_fixed_in_file "$TAG" 'decision=enter_first_keeper_optimization' "$stable_decision" "stable allocator evidence may enter optimization"
guard_expect_fixed_in_file "$TAG" 'selected_next_row=HAKO-MIMALLOC-PERF-FIRST-KEEPER-OPTIMIZATION-296X-001' "$stable_decision" "stable allocator evidence must select optimization row"
guard_expect_fixed_in_file "$TAG" 'next_optimization_allowed=1' "$stable_decision" "stable allocator evidence must allow optimization"
guard_expect_fixed_in_file "$TAG" 'optimization_started=0' "$stable_decision" "decision must not start optimization"

cat >"$noisy" <<'EOF'
output_contract=hako-mimalloc-gap-taxonomy-v0
workload_id=representative-small-block-v0
gap_owner=benchmark_harness
evidence_quality=noisy
gap_confidence=medium
next_diagnostic=measurement_hygiene_refresh
winner_claim=0
provider_active=0
replacement_active=0
hook_installed=0
global_allocator=0
summary=ok
EOF

python3 "$DECISION" --input "$noisy" --out "$noisy_decision"
guard_expect_fixed_in_file "$TAG" 'decision=continue_owner_diagnostic' "$noisy_decision" "noisy harness evidence must continue diagnostics"
guard_expect_fixed_in_file "$TAG" 'selected_next_row=HAKO-MIMALLOC-PERF-OWNER-NARROW-DIAGNOSTIC-296X-001' "$noisy_decision" "noisy harness evidence must not select optimization row"
guard_expect_fixed_in_file "$TAG" 'next_optimization_allowed=0' "$noisy_decision" "noisy harness evidence must block optimization"
guard_expect_fixed_in_file "$TAG" 'winner_claim=0' "$noisy_decision" "decision must keep winner claims closed"
guard_expect_fixed_in_file "$TAG" 'provider_active=0' "$noisy_decision" "decision must keep provider closed"
guard_expect_fixed_in_file "$TAG" 'replacement_active=0' "$noisy_decision" "decision must keep replacement closed"
guard_expect_fixed_in_file "$TAG" 'summary=ok' "$noisy_decision" "decision must end ok"

echo "[$TAG] ok"
