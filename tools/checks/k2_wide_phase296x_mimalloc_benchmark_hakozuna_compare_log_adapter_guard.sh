#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-phase296x-mimalloc-benchmark-hakozuna-compare-log-adapter"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

CARD_03="docs/development/current/main/phases/phase-296x/296x-03-MIMALLOC-BENCHMARK-HAKMEM-BENCHRES-ADAPTER.md"
CARD_04="docs/development/current/main/phases/phase-296x/296x-04-MIMALLOC-BENCHMARK-HAKOZUNA-COMPARE-LOG-ADAPTER.md"
TASKBOARD="docs/development/current/main/phases/phase-296x/296x-90-mimalloc-benchmark-taskboard.md"
INDEX="docs/tools/check-scripts-index.md"
CURRENT_STATE="docs/development/current/main/CURRENT_STATE.toml"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_mimalloc_benchmark_hakozuna_compare_log_adapter_guard.sh"
ADAPTER="tools/allocator/hakmem_hakozuna_compare_log_adapter.py"
LOG="/home/tomoaki/git/hakmem_20260525_extracted/hakmem/bench_results/hakozuna_compare_20260118_034633/hakozuna_compare_20260118_034633_mimalloc_e165faccc.log"

echo "[$TAG] checking phase-296x hakozuna compare log adapter"

guard_require_files "$TAG" "$CARD_03" "$CARD_04" "$TASKBOARD" "$INDEX" "$CURRENT_STATE" "$SELF_SCRIPT" "$ADAPTER" "$LOG"
guard_require_exec_files "$TAG" "$SELF_SCRIPT" "$ADAPTER"

guard_expect_fixed_in_file "$TAG" 'latest_card = "296x-04-MIMALLOC-BENCHMARK-HAKOZUNA-COMPARE-LOG-ADAPTER"' "$CURRENT_STATE" "current state latest card must advance to hakozuna compare adapter"
guard_expect_fixed_in_file "$TAG" 'current_blocker_token = "MIMALLOC-BENCHMARK-EXACT-EXE-HARNESS-PILOT-296X-001"' "$CURRENT_STATE" "current state must expose exact-exe harness blocker"
guard_expect_fixed_in_file "$TAG" 'Status: Landed' "$CARD_03" "benchres adapter card must be landed"
guard_expect_fixed_in_file "$TAG" 'Status: Landed' "$CARD_04" "hakozuna compare adapter card must be landed"
guard_expect_fixed_in_file "$TAG" 'MIMALLOC-BENCHMARK-HAKOZUNA-COMPARE-LOG-ADAPTER-296X-001' "$CARD_04" "hakozuna compare card must identify blocker"
guard_expect_fixed_in_file "$TAG" 'output_contract=hakmem-external-hakozuna-compare-log-adapter-v0' "$CARD_04" "hakozuna compare card must name the adapter contract"
guard_expect_fixed_in_file "$TAG" 'winner_claim=0' "$CARD_04" "hakozuna compare card must keep winner claims closed"
guard_expect_fixed_in_file "$TAG" 'MIMALLOC-BENCHMARK-HAKOZUNA-COMPARE-LOG-ADAPTER-296X-001' "$TASKBOARD" "taskboard must expose the hakozuna compare row"
guard_expect_fixed_in_file "$TAG" 'MIMALLOC-BENCHMARK-EXACT-EXE-HARNESS-PILOT-296X-001' "$CARD_04" "hakozuna compare card must select exact-exe harness next"
guard_expect_fixed_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list this guard"

python3 -m py_compile "$ADAPTER"

tmp_dir="$(mktemp -d /tmp/hakorune_phase296x_hakozuna_adapter.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT
report="$tmp_dir/report.out"

python3 "$ADAPTER" --in "$LOG" --out "$report"
guard_expect_fixed_in_file "$TAG" 'output_contract=hakmem-external-hakozuna-compare-log-adapter-v0' "$report" "adapter must keep output contract"
guard_expect_fixed_in_file "$TAG" 'dataset_role=external-historical-benchmark-corpus' "$report" "adapter must keep dataset role"
guard_expect_fixed_in_file "$TAG" 'label=mimalloc' "$report" "adapter must normalize label"
guard_expect_fixed_in_file "$TAG" 'run_count=10' "$report" "adapter must preserve run count"
guard_expect_fixed_in_file "$TAG" 'throughput_median_ops_per_sec=127684220' "$report" "adapter must compute throughput median"
guard_expect_fixed_in_file "$TAG" 'elapsed_median_ms=156' "$report" "adapter must compute elapsed median"
guard_expect_fixed_in_file "$TAG" 'peak_rss_median_bytes=2123776' "$report" "adapter must compute rss median"
guard_expect_fixed_in_file "$TAG" 'winner_claim=0' "$report" "adapter must keep winner claims closed"
guard_expect_fixed_in_file "$TAG" 'summary=ok' "$report" "adapter must end with summary"

cat "$report"
echo "[$TAG] ok"
