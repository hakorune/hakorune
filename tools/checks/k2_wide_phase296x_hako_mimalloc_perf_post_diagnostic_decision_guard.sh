#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-phase296x-hako-mimalloc-perf-post-diagnostic-decision"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

CARD_48="docs/development/current/main/phases/phase-296x/296x-48-HAKO-MIMALLOC-PERF-POST-DIAGNOSTIC-DECISION.md"
CARD_49="docs/development/current/main/phases/phase-296x/296x-49-HAKO-MIMALLOC-PERF-GAP-TAXONOMY-REFRESH.md"
TASKBOARD="docs/development/current/main/phases/phase-296x/296x-90-mimalloc-benchmark-taskboard.md"
CURRENT_STATE="docs/development/current/main/CURRENT_STATE.toml"
INDEX="docs/tools/check-scripts-index.md"
DECISION="tools/allocator/hako_mimalloc_post_diagnostic_decision.py"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_hako_mimalloc_perf_post_diagnostic_decision_guard.sh"

echo "[$TAG] checking phase-296x post diagnostic decision"

guard_require_files "$TAG" "$CARD_48" "$CARD_49" "$TASKBOARD" "$CURRENT_STATE" "$INDEX" "$DECISION" "$SELF_SCRIPT"
guard_require_exec_files "$TAG" "$DECISION" "$SELF_SCRIPT"

guard_expect_fixed_in_file "$TAG" 'Status: Landed' "$CARD_48" "post diagnostic card must be landed"
guard_expect_fixed_in_file "$TAG" 'Status: Current' "$CARD_49" "taxonomy refresh card must be current"
guard_expect_fixed_in_file "$TAG" 'output_contract=hako-mimalloc-post-diagnostic-decision-v0' "$CARD_48" "card must define decision contract"
guard_expect_fixed_in_file "$TAG" 'selected_next_row=HAKO-MIMALLOC-PERF-GAP-TAXONOMY-REFRESH-296X-001' "$CARD_48" "card must select refresh row"

guard_expect_fixed_in_file "$TAG" 'latest_card = "296x-48-HAKO-MIMALLOC-PERF-POST-DIAGNOSTIC-DECISION"' "$CURRENT_STATE" "current state latest card must advance to row 48"
guard_expect_fixed_in_file "$TAG" 'current_blocker_token = "HAKO-MIMALLOC-PERF-GAP-TAXONOMY-REFRESH-296X-001"' "$CURRENT_STATE" "current state must select row 49"
guard_expect_fixed_in_file "$TAG" '| 48 | `HAKO-MIMALLOC-PERF-POST-DIAGNOSTIC-DECISION-296X-001` | Landed |' "$TASKBOARD" "taskboard row 48 must be landed"
guard_expect_fixed_in_file "$TAG" '| 49 | `HAKO-MIMALLOC-PERF-GAP-TAXONOMY-REFRESH-296X-001` | Current |' "$TASKBOARD" "taskboard row 49 must be current"
guard_expect_fixed_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list this guard"
guard_expect_fixed_in_file "$TAG" "$DECISION" "$INDEX" "check index must list decision tool"

tmp_dir="$(mktemp -d /tmp/hakorune_phase296x_post_diag.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT
diag="$tmp_dir/diagnostic.out"
decision_out="$tmp_dir/decision.out"

cat >"$diag" <<'EOF'
output_contract=hako-mimalloc-owner-narrow-diagnostic-v0
input_contract=hako-mimalloc-conditional-diagnostic-selection-v0
front=representative-small-block-v0
workload_id=representative-small-block-v0
measurement_profile=phase295x-repeated-v0
gap_owner=benchmark_harness
diagnostic_kind=measurement_hygiene_refresh
measurement_contract=mimalloc-comparison-repeated-measurement-v0
measurement_hygiene_required=1
body_elapsed_ns_secondary=1
build_compile_excluded=1
sample_count=5
next_optimization_allowed=0
winner_claim=0
provider_active=0
replacement_active=0
hook_installed=0
global_allocator=0
summary=ok
EOF

python3 "$DECISION" --input "$diag" --out "$decision_out"
guard_expect_fixed_in_file "$TAG" 'output_contract=hako-mimalloc-post-diagnostic-decision-v0' "$decision_out" "decision must emit contract"
guard_expect_fixed_in_file "$TAG" 'decision=refresh_gap_taxonomy_after_hygiene' "$decision_out" "decision must refresh taxonomy after hygiene"
guard_expect_fixed_in_file "$TAG" 'selected_next_row=HAKO-MIMALLOC-PERF-GAP-TAXONOMY-REFRESH-296X-001' "$decision_out" "decision must select refresh row"
guard_expect_fixed_in_file "$TAG" 'optimization_started=0' "$decision_out" "decision must not start optimization"
guard_expect_fixed_in_file "$TAG" 'winner_claim=0' "$decision_out" "decision must keep winner claims closed"
guard_expect_fixed_in_file "$TAG" 'provider_active=0' "$decision_out" "decision must keep provider closed"
guard_expect_fixed_in_file "$TAG" 'replacement_active=0' "$decision_out" "decision must keep replacement closed"
guard_expect_fixed_in_file "$TAG" 'hook_installed=0' "$decision_out" "decision must keep hooks closed"
guard_expect_fixed_in_file "$TAG" 'global_allocator=0' "$decision_out" "decision must keep global allocator closed"
guard_expect_fixed_in_file "$TAG" 'summary=ok' "$decision_out" "decision must end ok"

echo "[$TAG] ok"
