#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-phase295x-hakozuna-compare-log-adapter"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

CARD="docs/development/current/main/phases/phase-295x/295x-86-MIMALLOC-COMPARISON-HAKOZUNA-COMPARE-LOG-ADAPTER.md"
PREV_CARD="docs/development/current/main/phases/phase-295x/295x-85-MIMALLOC-COMPARISON-HAKMEM-BENCHRES-ADAPTER-CLOSEOUT.md"
TASKBOARD="docs/development/current/main/phases/phase-295x/295x-90-mimalloc-comparison-execution-taskboard.md"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase295x_hakozuna_compare_log_adapter_guard.sh"
ADAPTER="tools/allocator/hakmem_hakozuna_compare_log_adapter.py"

echo "[$TAG] checking phase-295x hakozuna compare log adapter"

guard_require_files "$TAG" "$CARD" "$PREV_CARD" "$TASKBOARD" "$INDEX" "$SELF_SCRIPT" "$ADAPTER"
guard_require_exec_files "$TAG" "$SELF_SCRIPT" "$ADAPTER"

guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-HAKOZUNA-COMPARE-LOG-ADAPTER-295X-001' "$CARD" "card must identify blocker"
guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-HAKMEM-EXTERNAL-CORPUS-CATALOG-295X-001' "$CARD" "card must select catalog row"
guard_expect_in_file "$TAG" 'output_contract=hakmem-external-hakozuna-compare-log-adapter-v0' "$CARD" "card must document contract"
guard_expect_in_file "$TAG" 'winner_claim=0' "$CARD" "card must keep winner claims closed"
guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-HAKOZUNA-COMPARE-LOG-ADAPTER-295X-001' "$PREV_CARD" "previous card must select log adapter"
guard_expect_in_file "$TAG" '295x-86' "$TASKBOARD" "taskboard must expose adapter row"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check script index must list this guard"

python3 -m py_compile "$ADAPTER"

tmp_dir="$(mktemp -d /tmp/hakozuna_compare_log_adapter.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT
log="$tmp_dir/sample.log"
report="$tmp_dir/report.out"
cat > "$log" <<'EOF'
[BENCH-HEADER] ts=20260118_034633 git=e165faccc label=mimalloc
[BENCH-ENV] iters=20000000 ws=400 runs=2
[BENCH-CMD] LD_PRELOAD=/lib/x86_64-linux-gnu/libmimalloc.so.2 ./system_bench_random_mixed 20000000 400
== run 1/2 ==
[RSS] max_kb=2072
[ALLOCATOR] mimalloc
Throughput = 131120001 ops/s [allocator=mimalloc] [iter=20000000 ws=400] time=0.153s
== run 2/2 ==
[RSS] max_kb=2160
[ALLOCATOR] mimalloc
Throughput = 128758338 ops/s [allocator=mimalloc] [iter=20000000 ws=400] time=0.155s
EOF

python3 "$ADAPTER" --in "$log" --out "$report"
rg -F -q 'output_contract=hakmem-external-hakozuna-compare-log-adapter-v0' "$report"
rg -F -q 'dataset_role=external-historical-benchmark-corpus' "$report"
rg -F -q 'label=mimalloc' "$report"
rg -F -q 'run_count=2' "$report"
rg -F -q 'throughput_median_ops_per_sec=129939169' "$report"
rg -F -q 'elapsed_median_ms=154' "$report"
rg -F -q 'peak_rss_median_bytes=2166784' "$report"
rg -F -q 'winner_claim=0' "$report"
rg -F -q 'summary=ok' "$report"

cat "$report"
echo "[$TAG] ok"
