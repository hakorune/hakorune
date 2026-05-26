#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-phase296x-mimalloc-benchmark-external-corpus-closeout"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

CARD_03="docs/development/current/main/phases/phase-296x/296x-03-MIMALLOC-BENCHMARK-HAKMEM-BENCHRES-ADAPTER.md"
CARD_04="docs/development/current/main/phases/phase-296x/296x-04-MIMALLOC-BENCHMARK-HAKOZUNA-COMPARE-LOG-ADAPTER.md"
CARD_05="docs/development/current/main/phases/phase-296x/296x-05-MIMALLOC-BENCHMARK-EXACT-EXE-HARNESS-PILOT.md"
CARD_06="docs/development/current/main/phases/phase-296x/296x-06-MIMALLOC-BENCHMARK-EXTERNAL-CORPUS-CLOSEOUT.md"
TASKBOARD="docs/development/current/main/phases/phase-296x/296x-90-mimalloc-benchmark-taskboard.md"
INDEX="docs/tools/check-scripts-index.md"
CURRENT_STATE="docs/development/current/main/CURRENT_STATE.toml"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_mimalloc_benchmark_external_corpus_closeout_guard.sh"
BENCHRES_ADAPTER="tools/allocator/hakmem_benchres_adapter.py"
HAKOZUNA_ADAPTER="tools/allocator/hakmem_hakozuna_compare_log_adapter.py"
RUNNER="tools/allocator/mimalloc_repeated_measurement_runner.py"

echo "[$TAG] checking phase-296x external corpus closeout"

guard_require_files "$TAG" "$CARD_03" "$CARD_04" "$CARD_05" "$CARD_06" "$TASKBOARD" "$INDEX" "$CURRENT_STATE" "$SELF_SCRIPT" "$BENCHRES_ADAPTER" "$HAKOZUNA_ADAPTER" "$RUNNER"
guard_require_exec_files "$TAG" "$SELF_SCRIPT" "$BENCHRES_ADAPTER" "$HAKOZUNA_ADAPTER" "$RUNNER"

guard_expect_fixed_in_file "$TAG" 'Status: Landed' "$CARD_03" "benchres adapter must be landed"
guard_expect_fixed_in_file "$TAG" 'Status: Landed' "$CARD_04" "hakozuna compare adapter must be landed"
guard_expect_fixed_in_file "$TAG" 'Status: Landed' "$CARD_05" "exact-exe harness pilot must be landed"
guard_expect_fixed_in_file "$TAG" 'Status: Landed' "$CARD_06" "external corpus closeout must be landed"
guard_expect_fixed_in_file "$TAG" 'MIMALLOC-BENCHMARK-EXTERNAL-CORPUS-CLOSEOUT-296X-001' "$CARD_06" "closeout card must identify blocker"
guard_expect_fixed_in_file "$TAG" 'benchres_adapter=accepted' "$CARD_06" "closeout card must accept benchres adapter"
guard_expect_fixed_in_file "$TAG" 'hakozuna_compare_adapter=accepted' "$CARD_06" "closeout card must accept hakozuna adapter"
guard_expect_fixed_in_file "$TAG" 'exact_exe_harness_pilot=accepted' "$CARD_06" "closeout card must accept exact-exe pilot"
guard_expect_fixed_in_file "$TAG" 'MIMALLOC-BENCHMARK-EXACT-EXE-REPEATED-MEASUREMENT-296X-001' "$CARD_06" "closeout card must select real measurement"
guard_expect_fixed_in_file "$TAG" 'MIMALLOC-BENCHMARK-EXTERNAL-CORPUS-CLOSEOUT-296X-001' "$TASKBOARD" "taskboard must expose closeout row"
guard_expect_fixed_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list this guard"

python3 -m py_compile "$BENCHRES_ADAPTER" "$HAKOZUNA_ADAPTER" "$RUNNER"

guard_expect_fixed_in_file "$TAG" 'output_contract=hakmem-external-benchres-adapter-v0' "$BENCHRES_ADAPTER" "benchres adapter must keep output contract"
guard_expect_fixed_in_file "$TAG" 'output_contract=hakmem-external-hakozuna-compare-log-adapter-v0' "$HAKOZUNA_ADAPTER" "hakozuna adapter must keep output contract"
guard_expect_fixed_in_file "$TAG" 'output_contract=mimalloc-comparison-repeated-measurement-v0' "$RUNNER" "runner must keep repeated measurement contract"
guard_expect_fixed_in_file "$TAG" 'winner_claim=0' "$RUNNER" "runner must keep winner claims closed"

echo "[$TAG] ok"
