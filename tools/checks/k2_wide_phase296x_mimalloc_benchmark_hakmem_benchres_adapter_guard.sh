#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-phase296x-mimalloc-benchmark-hakmem-benchres-adapter"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

CARD_02="docs/development/current/main/phases/phase-296x/296x-02-MIMALLOC-BENCHMARK-RESULT-CONTRACT.md"
CARD_03="docs/development/current/main/phases/phase-296x/296x-03-MIMALLOC-BENCHMARK-HAKMEM-BENCHRES-ADAPTER.md"
TASKBOARD="docs/development/current/main/phases/phase-296x/296x-90-mimalloc-benchmark-taskboard.md"
INDEX="docs/tools/check-scripts-index.md"
CURRENT_STATE="docs/development/current/main/CURRENT_STATE.toml"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_mimalloc_benchmark_hakmem_benchres_adapter_guard.sh"
ADAPTER="tools/allocator/hakmem_benchres_adapter.py"

echo "[$TAG] checking phase-296x hakmem benchres adapter"

guard_require_files "$TAG" "$CARD_02" "$CARD_03" "$TASKBOARD" "$INDEX" "$CURRENT_STATE" "$SELF_SCRIPT" "$ADAPTER"
guard_require_exec_files "$TAG" "$SELF_SCRIPT" "$ADAPTER"

guard_expect_fixed_in_file "$TAG" 'latest_card = "296x-03-MIMALLOC-BENCHMARK-HAKMEM-BENCHRES-ADAPTER"' "$CURRENT_STATE" "current state latest card must advance to benchres adapter"
guard_expect_fixed_in_file "$TAG" 'current_blocker_token = "MIMALLOC-BENCHMARK-HAKOZUNA-COMPARE-LOG-ADAPTER-296X-001"' "$CURRENT_STATE" "current state must expose hakozuna compare blocker"
guard_expect_fixed_in_file "$TAG" 'Status: Landed' "$CARD_02" "result contract card must be landed"
guard_expect_fixed_in_file "$TAG" 'Status: Current' "$CARD_03" "benchres adapter card must be current"
guard_expect_fixed_in_file "$TAG" 'MIMALLOC-BENCHMARK-HAKMEM-BENCHRES-ADAPTER-296X-001' "$CARD_03" "benchres adapter card must identify blocker"
guard_expect_fixed_in_file "$TAG" 'output_contract=hakmem-external-benchres-adapter-v0' "$CARD_03" "benchres adapter card must name the adapter contract"
guard_expect_fixed_in_file "$TAG" 'winner_claim=0' "$CARD_03" "benchres adapter card must keep winner claims closed"
guard_expect_fixed_in_file "$TAG" 'MIMALLOC-BENCHMARK-HAKMEM-BENCHRES-ADAPTER-296X-001' "$TASKBOARD" "taskboard must expose the benchres adapter row"
guard_expect_fixed_in_file "$TAG" 'MIMALLOC-BENCHMARK-HAKMEM-BENCHRES-ADAPTER-296X-001' "$CARD_02" "result contract must select benchres adapter next"
guard_expect_fixed_in_file "$TAG" 'MIMALLOC-BENCHMARK-HAKOZUNA-COMPARE-LOG-ADAPTER-296X-001' "$CARD_03" "benchres adapter must select hakozuna compare next"
guard_expect_fixed_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list this guard"

python3 -m py_compile "$ADAPTER"

tmp_dir="$(mktemp -d /tmp/hakorune_phase296x_benchres_adapter.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT
benchres="$tmp_dir/benchres.csv"
report="$tmp_dir/report.out"

cat > "$benchres" <<'EOF'
# benchmark allocator elapsed rss user sys page-faults page-reclaims
cfrac       mimalloc 02.12 3588 2.15 0.00 1 306
cfrac       sys   02.33 3392 2.36 0.01 0 445
EOF

python3 "$ADAPTER" --in "$benchres" --out "$report"
guard_expect_fixed_in_file "$TAG" 'output_contract=hakmem-external-benchres-adapter-v0' "$report" "benchres adapter must keep output contract"
guard_expect_fixed_in_file "$TAG" 'dataset_role=external-historical-benchmark-corpus' "$report" "benchres adapter must keep dataset role"
guard_expect_fixed_in_file "$TAG" 'row_count=2' "$report" "benchres adapter must keep both rows"
guard_expect_fixed_in_file "$TAG" 'parsed_row_count=2' "$report" "benchres adapter must parse both rows"
guard_expect_fixed_in_file "$TAG" 'allocators=mimalloc,system' "$report" "benchres adapter must normalize allocators"
guard_expect_fixed_in_file "$TAG" 'row_0_elapsed_ms=2120' "$report" "benchres adapter must convert elapsed seconds"
guard_expect_fixed_in_file "$TAG" 'row_0_peak_rss_bytes=3674112' "$report" "benchres adapter must convert RSS to bytes"
guard_expect_fixed_in_file "$TAG" 'winner_claim=0' "$report" "benchres adapter must keep winner claims closed"
guard_expect_fixed_in_file "$TAG" 'summary=ok' "$report" "benchres adapter must end with summary"

cat "$report"
echo "[$TAG] ok"
