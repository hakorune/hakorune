#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-phase296x-hako-mimalloc-keeper-before-after-diff-adapter"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

CARD_84="docs/development/current/main/phases/phase-296x/296x-84-HAKO-MIMALLOC-KEEPER-BEFORE-AFTER-DIFF-ADAPTER.md"
CARD_85="docs/development/current/main/phases/phase-296x/296x-85-HAKO-MIR-METHOD-SHAPE-PYTHON-ADAPTER.md"
TASKBOARD="docs/development/current/main/phases/phase-296x/296x-90-mimalloc-benchmark-taskboard.md"
CURRENT_STATE="docs/development/current/main/CURRENT_STATE.toml"
INDEX="docs/tools/check-scripts-index.md"
TOOL="tools/allocator/hako_mimalloc_keeper_before_after_diff.py"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_hako_mimalloc_keeper_before_after_diff_adapter_guard.sh"

echo "[$TAG] checking keeper before/after diff adapter"

guard_require_files "$TAG" "$CARD_84" "$CARD_85" "$TASKBOARD" "$CURRENT_STATE" "$INDEX" "$TOOL" "$SELF_SCRIPT"
guard_require_exec_files "$TAG" "$TOOL" "$SELF_SCRIPT"

guard_expect_fixed_in_file "$TAG" 'Status: Landed' "$CARD_84" "diff card must be landed"
guard_expect_fixed_in_file "$TAG" 'Status: Current' "$CARD_85" "MIR adapter card must be current"
guard_expect_fixed_in_file "$TAG" 'output_contract=hako-mimalloc-keeper-before-after-diff-v0' "$CARD_84" "card must record output contract"
guard_expect_fixed_in_file "$TAG" 'source_surface_delta_ready=1' "$CARD_84" "card must record source delta"
guard_expect_fixed_in_file "$TAG" 'measurement_delta_ready=1' "$CARD_84" "card must record measurement delta"
guard_expect_fixed_in_file "$TAG" 'keeper_effect=' "$CARD_84" "card must record keeper effect"

guard_expect_fixed_in_file "$TAG" 'latest_card = "296x-84-HAKO-MIMALLOC-KEEPER-BEFORE-AFTER-DIFF-ADAPTER"' "$CURRENT_STATE" "current state latest card must advance to row 84"
guard_expect_fixed_in_file "$TAG" 'current_blocker_token = "HAKO-MIR-METHOD-SHAPE-PYTHON-ADAPTER-296X-001"' "$CURRENT_STATE" "current state must select row 85"
guard_expect_fixed_in_file "$TAG" '| 84 | `HAKO-MIMALLOC-KEEPER-BEFORE-AFTER-DIFF-ADAPTER-296X-001` | Landed |' "$TASKBOARD" "taskboard row 84 must be landed"
guard_expect_fixed_in_file "$TAG" '| 85 | `HAKO-MIR-METHOD-SHAPE-PYTHON-ADAPTER-296X-001` | Current |' "$TASKBOARD" "taskboard row 85 must be current"
guard_expect_fixed_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list this guard"
guard_expect_fixed_in_file "$TAG" "$TOOL" "$INDEX" "check index must list this tool"

tmp_dir="$(mktemp -d /tmp/hakorune_phase296x_keeper_diff.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT
cat > "$tmp_dir/before-source.out" <<'EOF'
output_contract=hako-check-perf-surface-v1
loop_array_get_count=3
loop_field_get_count=8
summary=ok
EOF
cat > "$tmp_dir/after-source.out" <<'EOF'
output_contract=hako-check-perf-surface-v1
loop_array_get_count=1
loop_field_get_count=5
summary=ok
EOF
cat > "$tmp_dir/before-measurement.out" <<'EOF'
output_contract=hako-mimalloc-perf-post-release-keeper-measurement-v0
after_hako_elapsed_median_ms=720
summary=ok
EOF
cat > "$tmp_dir/after-measurement.out" <<'EOF'
output_contract=hako-mimalloc-perf-post-select-page-keeper-measurement-v0
after_hako_elapsed_median_ms=690
summary=ok
EOF
report="$tmp_dir/report.out"
python3 "$TOOL" \
  --keeper-id select_page_single_page_fast_path \
  --before-source "$tmp_dir/before-source.out" \
  --after-source "$tmp_dir/after-source.out" \
  --before-measurement "$tmp_dir/before-measurement.out" \
  --after-measurement "$tmp_dir/after-measurement.out" \
  --out "$report"

guard_expect_fixed_in_file "$TAG" 'output_contract=hako-mimalloc-keeper-before-after-diff-v0' "$report" "tool must emit output contract"
guard_expect_fixed_in_file "$TAG" 'keeper_id=select_page_single_page_fast_path' "$report" "tool must preserve keeper id"
guard_expect_fixed_in_file "$TAG" 'source_surface_delta_ready=1' "$report" "tool must accept source reports"
guard_expect_fixed_in_file "$TAG" 'measurement_delta_ready=1' "$report" "tool must accept measurement reports"
guard_expect_fixed_in_file "$TAG" 'delta_loop_array_get_count=-2' "$report" "tool must compute source delta"
guard_expect_fixed_in_file "$TAG" 'delta_hako_elapsed_median_ms=-30' "$report" "tool must compute measurement delta"
guard_expect_fixed_in_file "$TAG" 'keeper_effect=accepted' "$report" "tool must classify accepted effect"
guard_expect_fixed_in_file "$TAG" 'winner_claim=0' "$report" "tool must keep winner closed"
guard_expect_fixed_in_file "$TAG" 'summary=ok' "$report" "tool must end ok"

echo "[$TAG] ok"
