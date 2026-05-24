#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-phase295x-realloc-aligned-evidence-run"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

CARD="docs/development/current/main/phases/phase-295x/295x-18-MIMALLOC-COMPARISON-REALLOC-ALIGNED-EVIDENCE-RUN.md"
PREV_CARD="docs/development/current/main/phases/phase-295x/295x-17-MIMALLOC-COMPARISON-REALLOC-ALIGNED-HAKO-EXE-ACCEPTANCE.md"
TASKBOARD="docs/development/current/main/phases/phase-295x/295x-90-mimalloc-comparison-execution-taskboard.md"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase295x_realloc_aligned_evidence_run_guard.sh"
HAKO_RUNNER="tools/allocator/hako_exe_memory_runner.sh"
C_RUNNER="tools/allocator/c_mimalloc_explicit_runner.sh"
NORMALIZER="tools/allocator/mimalloc_comparison_memory_report.py"
APP="apps/hako-alloc-mimalloc-comparison-realloc-aligned-exe-proof/main.hako"

echo "[$TAG] checking phase-295x realloc/aligned evidence run"

guard_require_files \
  "$TAG" \
  "$CARD" \
  "$PREV_CARD" \
  "$TASKBOARD" \
  "$INDEX" \
  "$SELF_SCRIPT" \
  "$HAKO_RUNNER" \
  "$C_RUNNER" \
  "$NORMALIZER" \
  "$APP"

guard_require_exec_files "$TAG" "$SELF_SCRIPT" "$HAKO_RUNNER" "$C_RUNNER" "$NORMALIZER"

guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-REALLOC-ALIGNED-EVIDENCE-295X-RUN-001' "$CARD" "card must identify the current blocker"
guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-REALLOC-ALIGNED-CLOSEOUT-295X-001' "$CARD" "card must select closeout follow-on"
guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-REALLOC-ALIGNED-EVIDENCE-295X-RUN-001' "$PREV_CARD" "previous row must select this evidence run"
guard_expect_in_file "$TAG" '| 18 | `295x-18` | Landed |' "$TASKBOARD" "taskboard must retain this row as landed"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check script index must list this guard"
guard_expect_in_file "$TAG" 'moved/copy/RSS as side-by-side evidence only' "$CARD" "card must keep non-parity evidence explicit"

if rg -n 'LD_PRELOAD|replace_process_allocator[[:space:]]*\(|install_hook[[:space:]]*\(|#\[global_allocator\]|winner_claim=1|provider_activation=1|host_replacement=1|thread::|worker_local|nowait|await' "$APP" >/tmp/"$TAG".forbidden 2>&1; then
  echo "[$TAG] ERROR: evidence run opened replacement/provider/thread/winner seams" >&2
  cat /tmp/"$TAG".forbidden >&2
  rm -f /tmp/"$TAG".forbidden
  exit 1
fi
rm -f /tmp/"$TAG".forbidden

tmp_dir="$(mktemp -d /tmp/hakorune_phase295x_realloc_aligned_evidence.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT
hako_out="$tmp_dir/hako.out"
c_out="$tmp_dir/c.out"
report_out="$tmp_dir/report.out"

bash "$HAKO_RUNNER" --app "$APP" --workload representative-realloc-aligned-v0 --out "$hako_out"
bash "$C_RUNNER" --out "$c_out" --allow-ldconfig-discovery --workload representative-realloc-aligned-v0
python3 "$NORMALIZER" --hako "$hako_out" --c "$c_out" --out "$report_out"

rg -F -q 'mimalloc_comparison_memory_report=1' "$report_out"
rg -F -q 'output_contract=mimalloc-comparison-memory-report-v0' "$report_out"
rg -F -q 'hako_workload=representative-realloc-aligned-v0' "$report_out"
rg -F -q 'c_workload=representative-realloc-aligned-v0' "$report_out"
rg -F -q 'workload_match=1' "$report_out"
rg -F -q 'hako_operation_family=realloc-aligned' "$report_out"
rg -F -q 'c_operation_family=realloc-aligned' "$report_out"
rg -F -q 'operation_family_match=1' "$report_out"
rg -F -q 'operation_sequence_match=1' "$report_out"
rg -F -q 'free_order_match=1' "$report_out"
rg -F -q 'allocation_count_delta=0' "$report_out"
rg -F -q 'free_count_delta=0' "$report_out"
rg -F -q 'requested_bytes_delta=0' "$report_out"
rg -F -q 'realloc_count_delta=0' "$report_out"
rg -F -q 'aligned_alloc_count_delta=0' "$report_out"
rg -F -q 'alignment_request_count_delta=0' "$report_out"
rg -F -q 'alignment_ok_count_delta=0' "$report_out"
rg -F -q 'alignment_reject_count_delta=0' "$report_out"
rg -F -q 'hako_copied_bytes=' "$report_out"
rg -F -q 'c_copied_bytes=' "$report_out"
rg -F -q 'hako_realloc_moved_count=' "$report_out"
rg -F -q 'c_realloc_moved_count=' "$report_out"
rg -F -q 'hako_peak_rss_bytes=' "$report_out"
rg -F -q 'c_peak_rss_bytes=' "$report_out"
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

for key in (
    "hako_realloc_same_ptr_count",
    "c_realloc_same_ptr_count",
    "hako_realloc_moved_count",
    "c_realloc_moved_count",
    "hako_copied_bytes",
    "c_copied_bytes",
    "hako_peak_rss_bytes",
    "c_peak_rss_bytes",
    "peak_rss_bytes_delta",
):
    int(values.get(key, "0"))

for key in ("hako_peak_rss_bytes", "c_peak_rss_bytes"):
    if int(values[key]) <= 0:
        raise SystemExit(f"{key} must be positive")
print("[phase295x-realloc-aligned-evidence] ok")
PY

cat "$report_out"
echo "[$TAG] ok"
