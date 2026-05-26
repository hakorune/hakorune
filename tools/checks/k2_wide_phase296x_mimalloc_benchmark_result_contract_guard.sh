#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-phase296x-mimalloc-benchmark-result-contract"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

ROADMAP="docs/development/current/main/design/mimalloc-benchmark-dll-roadmap-ssot.md"
README="docs/development/current/main/phases/phase-296x/README.md"
TASKBOARD="docs/development/current/main/phases/phase-296x/296x-90-mimalloc-benchmark-taskboard.md"
CARD_01="docs/development/current/main/phases/phase-296x/296x-01-MIMALLOC-BENCHMARK-HAKMEM-ASSET-INVENTORY.md"
CARD_02="docs/development/current/main/phases/phase-296x/296x-02-MIMALLOC-BENCHMARK-RESULT-CONTRACT.md"
CURRENT_STATE="docs/development/current/main/CURRENT_STATE.toml"
CHECK_INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_mimalloc_benchmark_result_contract_guard.sh"
BENCHRES_ADAPTER="tools/allocator/hakmem_benchres_adapter.py"
HAKOZUNA_ADAPTER="tools/allocator/hakmem_hakozuna_compare_log_adapter.py"
REPEATED_RUNNER="tools/allocator/mimalloc_repeated_measurement_runner.py"

echo "[$TAG] checking phase-296x benchmark result contract"

guard_require_files "$TAG" \
  "$ROADMAP" \
  "$README" \
  "$TASKBOARD" \
  "$CARD_01" \
  "$CARD_02" \
  "$CURRENT_STATE" \
  "$CHECK_INDEX" \
  "$BENCHRES_ADAPTER" \
  "$HAKOZUNA_ADAPTER" \
  "$REPEATED_RUNNER"
guard_require_exec_files "$TAG" "$BENCHRES_ADAPTER" "$HAKOZUNA_ADAPTER" "$REPEATED_RUNNER"

guard_expect_fixed_in_file "$TAG" 'latest_card = "296x-01-MIMALLOC-BENCHMARK-HAKMEM-ASSET-INVENTORY"' "$CURRENT_STATE" "current state latest card must remain inventory"
guard_expect_fixed_in_file "$TAG" 'current_blocker_token = "MIMALLOC-BENCHMARK-RESULT-CONTRACT-296X-001"' "$CURRENT_STATE" "current state must expose result contract blocker"
guard_expect_fixed_in_file "$TAG" 'Status: Landed' "$CARD_01" "inventory card must be landed"
guard_expect_fixed_in_file "$TAG" 'Status: Current' "$CARD_02" "result contract card must be current"
guard_expect_fixed_in_file "$TAG" 'benchmark_result_contract=hakmem-benchmark-result-v0' "$CARD_02" "result contract card must name the shared contract"
guard_expect_fixed_in_file "$TAG" 'source_corpus=/home/tomoaki/git/hakmem_20260525_extracted/hakmem' "$CARD_02" "result contract card must pin the source corpus"
guard_expect_fixed_in_file "$TAG" 'timing_repeat_kind=process-invocation-v0' "$CARD_02" "result contract must pin timing repeat kind"
guard_expect_fixed_in_file "$TAG" 'summary_statistic=min,median,max' "$CARD_02" "result contract must pin summary statistic"
guard_expect_fixed_in_file "$TAG" 'canonical_rss_collector=external-time' "$CARD_02" "result contract must pin canonical RSS collector"
guard_expect_fixed_in_file "$TAG" 'winner_claim=0' "$CARD_02" "result contract must keep winner claims closed"
guard_expect_fixed_in_file "$TAG" 'MIMALLOC-BENCHMARK-RESULT-CONTRACT-296X-001' "$TASKBOARD" "taskboard must expose the result contract row"
guard_expect_fixed_in_file "$TAG" 'MIMALLOC-BENCHMARK-HAKMEM-BENCHRES-ADAPTER-296X-001' "$CARD_02" "result contract must select benchres adapter next"
guard_expect_fixed_in_file "$TAG" 'MIMALLOC-BENCHMARK-RESULT-CONTRACT-296X-001' "$ROADMAP" "roadmap must name the result contract blocker"
guard_expect_fixed_in_file "$TAG" 'output_contract=mimalloc-comparison-repeated-measurement-v0' "$REPEATED_RUNNER" "repeated runner must keep its output contract"
guard_expect_fixed_in_file "$TAG" 'measurement_profile=phase295x-repeated-v0' "$REPEATED_RUNNER" "repeated runner must keep the repeated measurement profile"
guard_expect_fixed_in_file "$TAG" 'f"{prefix}_id={workload}"' "$REPEATED_RUNNER" "repeated runner must expose workload ids"
guard_expect_fixed_in_file "$TAG" 'summary_statistic=min,median,max' "$REPEATED_RUNNER" "repeated runner must keep summary policy"
guard_expect_fixed_in_file "$TAG" 'canonical_rss_collector=external-time' "$REPEATED_RUNNER" "repeated runner must keep canonical RSS policy"
guard_expect_fixed_in_file "$TAG" 'winner_claim=0' "$REPEATED_RUNNER" "repeated runner must keep winner claims closed"
guard_expect_fixed_in_file "$TAG" "$BENCHRES_ADAPTER" "$CHECK_INDEX" "check index must list the benchres adapter tool"
guard_expect_fixed_in_file "$TAG" "$HAKOZUNA_ADAPTER" "$CHECK_INDEX" "check index must list the hakozuna adapter tool"
guard_expect_fixed_in_file "$TAG" "$SELF_SCRIPT" "$CHECK_INDEX" "check index must list this guard"

python3 -m py_compile "$BENCHRES_ADAPTER" "$HAKOZUNA_ADAPTER" "$REPEATED_RUNNER"

tmp_dir="$(mktemp -d /tmp/hakorune_phase296x_result_contract.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT
benchres="$tmp_dir/benchres.csv"
benchres_report="$tmp_dir/benchres.out"
log="$tmp_dir/hakozuna.log"
log_report="$tmp_dir/hakozuna.out"

cat > "$benchres" <<'EOF'
# benchmark allocator elapsed rss user sys page-faults page-reclaims
cfrac       mimalloc 02.12 3588 2.15 0.00 1 306
cfrac       sys   02.33 3392 2.36 0.01 0 445
EOF

python3 "$BENCHRES_ADAPTER" --in "$benchres" --out "$benchres_report"
guard_expect_fixed_in_file "$TAG" 'output_contract=hakmem-external-benchres-adapter-v0' "$benchres_report" "benchres adapter must keep output contract"
guard_expect_fixed_in_file "$TAG" 'dataset_role=external-historical-benchmark-corpus' "$benchres_report" "benchres adapter must keep dataset role"
guard_expect_fixed_in_file "$TAG" 'winner_claim=0' "$benchres_report" "benchres adapter must keep winner claims closed"

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

python3 "$HAKOZUNA_ADAPTER" --in "$log" --out "$log_report"
guard_expect_fixed_in_file "$TAG" 'output_contract=hakmem-external-hakozuna-compare-log-adapter-v0' "$log_report" "hakozuna adapter must keep output contract"
guard_expect_fixed_in_file "$TAG" 'dataset_role=external-historical-benchmark-corpus' "$log_report" "hakozuna adapter must keep dataset role"
guard_expect_fixed_in_file "$TAG" 'winner_claim=0' "$log_report" "hakozuna adapter must keep winner claims closed"

echo "[$TAG] ok"
