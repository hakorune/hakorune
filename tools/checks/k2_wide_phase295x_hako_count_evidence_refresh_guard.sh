#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-phase295x-hako-count-evidence-refresh"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

CARD="docs/development/current/main/phases/phase-295x/295x-13-MIMALLOC-COMPARISON-HAKO-COUNT-EVIDENCE-REFRESH.md"
PREV_CARD="docs/development/current/main/phases/phase-295x/295x-12-MIMALLOC-COMPARISON-COUNT-EVIDENCE-SELECTION.md"
TASKBOARD="docs/development/current/main/phases/phase-295x/295x-90-mimalloc-comparison-execution-taskboard.md"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase295x_hako_count_evidence_refresh_guard.sh"
H_RUNNER="tools/allocator/hako_exe_memory_runner.sh"
C_RUNNER="tools/allocator/c_mimalloc_explicit_runner.sh"
NORMALIZER="tools/allocator/mimalloc_comparison_memory_report.py"
APP="apps/hako-alloc-mimalloc-comparison-representative-small-block-proof/main.hako"

echo "[$TAG] checking phase-295x hako count evidence refresh"

guard_require_files \
  "$TAG" \
  "$CARD" \
  "$PREV_CARD" \
  "$TASKBOARD" \
  "$INDEX" \
  "$SELF_SCRIPT" \
  "$H_RUNNER" \
  "$C_RUNNER" \
  "$NORMALIZER" \
  "$APP"

guard_require_exec_files "$TAG" "$SELF_SCRIPT" "$H_RUNNER" "$C_RUNNER" "$NORMALIZER"

guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-HAKO-COUNT-EVIDENCE-295X-REFRESH-001' "$CARD" "card must identify the current blocker"
guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-COUNT-EVIDENCE-CLOSEOUT-295X-001' "$CARD" "card must select closeout follow-on"
guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-HAKO-COUNT-EVIDENCE-295X-REFRESH-001' "$PREV_CARD" "previous row must select this refresh"
guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-COUNT-EVIDENCE-CLOSEOUT-295X-001' "$TASKBOARD" "taskboard must expose count evidence closeout"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check script index must list this guard"
guard_expect_in_file "$TAG" 'allocation_count=' "$H_RUNNER" "hako runner must publish allocation_count"
guard_expect_in_file "$TAG" 'free_count=' "$H_RUNNER" "hako runner must publish free_count"
guard_expect_in_file "$TAG" 'allocation_count_delta=' "$NORMALIZER" "normalizer must publish allocation count delta"
guard_expect_in_file "$TAG" 'free_count_delta=' "$NORMALIZER" "normalizer must publish free count delta"

tmp_dir="$(mktemp -d /tmp/hakorune_phase295x_hako_count_evidence.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT
hako_out="$tmp_dir/hako.out"
c_out="$tmp_dir/c.out"
report_out="$tmp_dir/report.out"

bash "$H_RUNNER" --app "$APP" --workload representative-small-block-v0 --out "$hako_out"
bash "$C_RUNNER" --out "$c_out" --allow-ldconfig-discovery
python3 "$NORMALIZER" --hako "$hako_out" --c "$c_out" --out "$report_out"

rg -F -q 'allocation_count=64' "$hako_out"
rg -F -q 'free_count=64' "$hako_out"
rg -F -q 'hako_allocation_count=64' "$report_out"
rg -F -q 'c_allocation_count=64' "$report_out"
rg -F -q 'allocation_count_delta=0' "$report_out"
rg -F -q 'hako_free_count=64' "$report_out"
rg -F -q 'c_free_count=64' "$report_out"
rg -F -q 'free_count_delta=0' "$report_out"
rg -F -q 'requested_bytes_delta=0' "$report_out"
rg -F -q 'winner_claim=0' "$report_out"
rg -F -q 'summary=ok' "$report_out"

cat "$report_out"
echo "[$TAG] ok"
