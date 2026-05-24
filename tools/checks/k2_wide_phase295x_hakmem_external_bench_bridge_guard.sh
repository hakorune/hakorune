#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-phase295x-hakmem-external-bench-bridge"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

CARD="docs/development/current/main/phases/phase-295x/295x-82-MIMALLOC-COMPARISON-HAKMEM-EXTERNAL-BENCH-BRIDGE.md"
PREV_CARD="docs/development/current/main/phases/phase-295x/295x-81-MIMALLOC-COMPARISON-REUSE-CYCLE-SMALL-WORKLOAD-CLOSEOUT.md"
TASKBOARD="docs/development/current/main/phases/phase-295x/295x-90-mimalloc-comparison-execution-taskboard.md"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase295x_hakmem_external_bench_bridge_guard.sh"
BRIDGE="tools/allocator/hakmem_external_bench.py"

echo "[$TAG] checking phase-295x hakmem external bench bridge"

guard_require_files "$TAG" "$CARD" "$PREV_CARD" "$TASKBOARD" "$INDEX" "$SELF_SCRIPT" "$BRIDGE"
guard_require_exec_files "$TAG" "$SELF_SCRIPT" "$BRIDGE"

guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-HAKMEM-EXTERNAL-BENCH-BRIDGE-295X-001' "$CARD" "card must identify blocker"
guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-HAKMEM-SCHEMA-ADAPTER-SELECTION-295X-001' "$CARD" "card must select next adapter row"
guard_expect_in_file "$TAG" 'target/hakmem-bench' "$CARD" "card must keep copied binaries under target"
guard_expect_in_file "$TAG" 'tools/allocator/hakmem_external_bench.py --list' "$CARD" "card must document list entrypoint"
guard_expect_in_file "$TAG" 'winner_claim=0' "$CARD" "card must keep winner claims closed"
guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-HAKMEM-EXTERNAL-BENCH-BRIDGE-295X-001' "$PREV_CARD" "previous card must select bridge"
guard_expect_in_file "$TAG" '295x-82' "$TASKBOARD" "taskboard must expose bridge row"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check script index must list this guard"

python3 -m py_compile "$BRIDGE"

list_out="$(mktemp /tmp/hakmem_external_bench_bridge_list.XXXXXX)"
python3 "$BRIDGE" --list > "$list_out"
rg -F -q 'output_contract=hakmem-external-bench-bridge-list-v0' "$list_out"
rg -F -q 'supported_allocators=sys,mimalloc,tcmalloc,hz3,hakozuna' "$list_out"
rg -F -q 'supported_benches=' "$list_out"
rg -F -q 'winner_claim=0' "$list_out"

out="$(mktemp /tmp/hakmem_external_bench_bridge.XXXXXX)"
python3 "$BRIDGE" --prepare-only --allocator sys --allocator mimalloc > "$out"
rg -F -q 'output_contract=hakmem-external-bench-bridge-v0' "$out"
rg -F -q 'dataset_role=external-historical-benchmark-corpus' "$out"
rg -F -q 'winner_claim=0' "$out"
rg -F -q 'provider_activation=0' "$out"
rg -F -q 'summary=ok' "$out"
rg -F -q 'mimalloc ' target/hakmem-bench/external_allocators.txt
test -x target/hakmem-bench/out/bench/cfrac
test -x target/hakmem-bench/bench.sh

cat "$out"
rm -f "$out" "$list_out"
echo "[$TAG] ok"
