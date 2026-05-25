#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-phase295x-malloc-large-evidence-run"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

CARD="docs/development/current/main/phases/phase-295x/295x-188-MIMALLOC-COMPARISON-MIMALLOC-BENCH-MALLOC-LARGE-EVIDENCE-RUN.md"
PREV_CARD="docs/development/current/main/phases/phase-295x/295x-187-MIMALLOC-COMPARISON-MIMALLOC-BENCH-MALLOC-LARGE-WORKLOAD-CONTRACT.md"
NEXT_CARD="docs/development/current/main/phases/phase-295x/295x-189-MIMALLOC-COMPARISON-MALLOC-LARGE-CLOSEOUT.md"
TASKBOARD="docs/development/current/main/phases/phase-295x/295x-90-mimalloc-comparison-execution-taskboard.md"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase295x_malloc_large_evidence_run_guard.sh"
NEXT_GUARD="tools/checks/k2_wide_phase295x_malloc_large_closeout_guard.sh"
EXTERNAL_BENCH="tools/allocator/hakmem_external_bench.py"
EXTERNAL_ADAPTER="tools/allocator/hakmem_benchres_adapter.py"
HAKO_RUNNER="tools/allocator/hako_exe_memory_runner.sh"
C_RUNNER="tools/allocator/c_mimalloc_explicit_runner.sh"
NORMALIZER="tools/allocator/mimalloc_comparison_memory_report.py"
HUGE_APP="apps/hako-alloc-mimalloc-comparison-huge-ish-exe-proof/main.hako"

echo "[$TAG] checking phase-295x malloc-large evidence run"

guard_require_files \
  "$TAG" \
  "$CARD" \
  "$PREV_CARD" \
  "$NEXT_CARD" \
  "$TASKBOARD" \
  "$INDEX" \
  "$SELF_SCRIPT" \
  "$NEXT_GUARD" \
  "$EXTERNAL_BENCH" \
  "$EXTERNAL_ADAPTER" \
  "$HAKO_RUNNER" \
  "$C_RUNNER" \
  "$NORMALIZER" \
  "$HUGE_APP"

guard_require_exec_files \
  "$TAG" \
  "$SELF_SCRIPT" \
  "$NEXT_GUARD" \
  "$EXTERNAL_BENCH" \
  "$EXTERNAL_ADAPTER" \
  "$HAKO_RUNNER" \
  "$C_RUNNER" \
  "$NORMALIZER"

guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-MIMALLOC-BENCH-MALLOC-LARGE-EVIDENCE-295X-RUN-001' "$CARD" "card must identify the current blocker"
guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-MALLOC-LARGE-CLOSEOUT-295X-001' "$CARD" "card must select closeout follow-on"
guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-MIMALLOC-BENCH-MALLOC-LARGE-EVIDENCE-295X-RUN-001' "$PREV_CARD" "previous row must select this evidence run"
guard_expect_in_file "$TAG" '| 188 | `MIMALLOC-COMPARISON-MIMALLOC-BENCH-MALLOC-LARGE-EVIDENCE-295X-RUN-001` | Landed |' "$TASKBOARD" "taskboard must keep the evidence row landed"
guard_expect_in_file "$TAG" '| 189 | `MIMALLOC-COMPARISON-MALLOC-LARGE-CLOSEOUT-295X-001` | Current |' "$TASKBOARD" "taskboard must expose the closeout row"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check script index must list this guard"
guard_expect_in_file "$TAG" "$NEXT_GUARD" "$INDEX" "check script index must list the closeout guard"
guard_expect_in_file "$TAG" 'workload=representative-huge-ish-v0' "$CARD" "card must keep the selected .hako workload"
guard_expect_in_file "$TAG" 'requested_bytes=4194321' "$CARD" "card must keep the selected requested bytes"
guard_expect_in_file "$TAG" 'large_request_count=1' "$CARD" "card must keep the selected large request count"

tmp_dir="$(mktemp -d /tmp/hakorune_phase295x_malloc_large_evidence.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT
external_benchres="$tmp_dir/external.benchres.csv"
external_report="$tmp_dir/external.report.out"
hako_out="$tmp_dir/hako.out"
c_out="$tmp_dir/c.out"
report_out="$tmp_dir/report.out"

python3 "$EXTERNAL_BENCH" \
  --bench malloc-large \
  --out "$external_benchres" \
  --repeats 1 \
  --test-repeats 1
python3 "$EXTERNAL_ADAPTER" --in "$external_benchres" --out "$external_report"
bash "$HAKO_RUNNER" --app "$HUGE_APP" --workload representative-huge-ish-v0 --out "$hako_out"
bash "$C_RUNNER" --out "$c_out" --allow-ldconfig-discovery --workload representative-huge-ish-v0
python3 "$NORMALIZER" --hako "$hako_out" --c "$c_out" --out "$report_out"

rg -F -q 'output_contract=hakmem-external-benchres-adapter-v0' "$external_report"
rg -F -q 'dataset_role=external-historical-benchmark-corpus' "$external_report"
rg -F -q 'benchmarks=malloc-large' "$external_report"
rg -F -q 'allocators=mimalloc,system' "$external_report"
rg -F -q 'summary=ok' "$external_report"
rg -F -q 'mimalloc_comparison_memory_report=1' "$report_out"
rg -F -q 'output_contract=mimalloc-comparison-memory-report-v0' "$report_out"
rg -F -q 'hako_workload=representative-huge-ish-v0' "$report_out"
rg -F -q 'c_workload=representative-huge-ish-v0' "$report_out"
rg -F -q 'workload_match=1' "$report_out"
rg -F -q 'operation_family_match=1' "$report_out"
rg -F -q 'operation_sequence_match=1' "$report_out"
rg -F -q 'free_order_match=1' "$report_out"
rg -F -q 'allocation_count_delta=0' "$report_out"
rg -F -q 'free_count_delta=0' "$report_out"
rg -F -q 'requested_bytes_delta=0' "$report_out"
rg -F -q 'large_request_count_delta=0' "$report_out"
rg -F -q 'hako_requested_bytes=4194321' "$report_out"
rg -F -q 'hako_large_request_count=1' "$report_out"
rg -F -q 'winner_claim=0' "$report_out"
rg -F -q 'summary=ok' "$report_out"

python3 - "$report_out" <<'PY'
import sys

values = {}
with open(sys.argv[1], encoding="utf-8") as fh:
    for line in fh:
        line = line.strip()
        if "=" in line:
            key, value = line.split("=", 1)
            values[key] = value

for key in ("hako_peak_rss_bytes", "c_peak_rss_bytes"):
    if int(values.get(key, "0")) <= 0:
        raise SystemExit(f"{key} must be positive")

print("[phase295x-malloc-large-evidence] ok")
PY

cat "$external_report"
cat "$report_out"
echo "[$TAG] ok"
