#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-phase295x-hakmem-benchres-adapter"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

CARD="docs/development/current/main/phases/phase-295x/295x-84-MIMALLOC-COMPARISON-HAKMEM-BENCHRES-ADAPTER.md"
PREV_CARD="docs/development/current/main/phases/phase-295x/295x-83-MIMALLOC-COMPARISON-HAKMEM-SCHEMA-ADAPTER-SELECTION.md"
TASKBOARD="docs/development/current/main/phases/phase-295x/295x-90-mimalloc-comparison-execution-taskboard.md"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase295x_hakmem_benchres_adapter_guard.sh"
BRIDGE="tools/allocator/hakmem_external_bench.py"
ADAPTER="tools/allocator/hakmem_benchres_adapter.py"

echo "[$TAG] checking phase-295x hakmem benchres adapter"

guard_require_files "$TAG" "$CARD" "$PREV_CARD" "$TASKBOARD" "$INDEX" "$SELF_SCRIPT" "$BRIDGE" "$ADAPTER"
guard_require_exec_files "$TAG" "$SELF_SCRIPT" "$BRIDGE" "$ADAPTER"

guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-HAKMEM-BENCHRES-ADAPTER-295X-001' "$CARD" "card must identify blocker"
guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-HAKMEM-BENCHRES-ADAPTER-CLOSEOUT-295X-001' "$CARD" "card must select closeout"
guard_expect_in_file "$TAG" 'output_contract=hakmem-external-benchres-adapter-v0' "$CARD" "card must document adapter contract"
guard_expect_in_file "$TAG" 'winner_claim=0' "$CARD" "card must keep winner claims closed"
guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-HAKMEM-BENCHRES-ADAPTER-295X-001' "$PREV_CARD" "previous card must select adapter"
guard_expect_in_file "$TAG" '295x-84' "$TASKBOARD" "taskboard must expose adapter row"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check script index must list this guard"

python3 -m py_compile "$ADAPTER"

tmp_dir="$(mktemp -d /tmp/hakmem_benchres_adapter.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT
benchres="$tmp_dir/benchres.csv"
report="$tmp_dir/report.out"
cat > "$benchres" <<'EOF'
# benchmark allocator elapsed rss user sys page-faults page-reclaims
cfrac       mimalloc 02.12 3588 2.15 0.00 1 306
cfrac       sys   02.33 3392 2.36 0.01 0 445
EOF

python3 "$ADAPTER" --in "$benchres" --out "$report"
rg -F -q 'output_contract=hakmem-external-benchres-adapter-v0' "$report"
rg -F -q 'dataset_role=external-historical-benchmark-corpus' "$report"
rg -F -q 'row_count=2' "$report"
rg -F -q 'parsed_row_count=2' "$report"
rg -F -q 'allocators=mimalloc,system' "$report"
rg -F -q 'row_0_elapsed_ms=2120' "$report"
rg -F -q 'row_0_peak_rss_bytes=3674112' "$report"
rg -F -q 'row_1_allocator=system' "$report"
rg -F -q 'winner_claim=0' "$report"
rg -F -q 'summary=ok' "$report"

cat "$report"
echo "[$TAG] ok"
