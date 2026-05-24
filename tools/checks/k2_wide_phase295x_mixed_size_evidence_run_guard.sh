#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-phase295x-mixed-size-evidence-run"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

CARD="docs/development/current/main/phases/phase-295x/295x-22-MIMALLOC-COMPARISON-MIXED-SIZE-EVIDENCE-RUN.md"
PREV_CARD="docs/development/current/main/phases/phase-295x/295x-21-MIMALLOC-COMPARISON-MIXED-SIZE-CONTRACT-REFRESH.md"
TASKBOARD="docs/development/current/main/phases/phase-295x/295x-90-mimalloc-comparison-execution-taskboard.md"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase295x_mixed_size_evidence_run_guard.sh"
HAKO_RUNNER="tools/allocator/hako_exe_memory_runner.sh"
C_RUNNER="tools/allocator/c_mimalloc_explicit_runner.sh"
NORMALIZER="tools/allocator/mimalloc_comparison_memory_report.py"
APP="apps/hako-alloc-mimalloc-comparison-mixed-small-exe-proof/main.hako"

echo "[$TAG] checking phase-295x mixed-size evidence run"

guard_require_files "$TAG" "$CARD" "$PREV_CARD" "$TASKBOARD" "$INDEX" "$SELF_SCRIPT" "$HAKO_RUNNER" "$C_RUNNER" "$NORMALIZER" "$APP"
guard_require_exec_files "$TAG" "$SELF_SCRIPT" "$HAKO_RUNNER" "$C_RUNNER" "$NORMALIZER"

guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-MIXED-SIZE-EVIDENCE-295X-RUN-001' "$CARD" "card must identify the current blocker"
guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-MIXED-SIZE-CLOSEOUT-295X-001' "$CARD" "card must select closeout follow-on"
guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-MIXED-SIZE-EVIDENCE-295X-RUN-001' "$PREV_CARD" "previous row must select this evidence run"
guard_expect_in_file "$TAG" '| 22 | `295x-22` | Landed |' "$TASKBOARD" "taskboard must retain this row as landed"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check script index must list this guard"

tmp_dir="$(mktemp -d /tmp/hakorune_phase295x_mixed_evidence.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT
hako_out="$tmp_dir/hako.out"
c_out="$tmp_dir/c.out"
report_out="$tmp_dir/report.out"

bash "$HAKO_RUNNER" --app "$APP" --workload representative-mixed-small-v0 --out "$hako_out"
bash "$C_RUNNER" --out "$c_out" --allow-ldconfig-discovery --workload representative-mixed-small-v0
python3 "$NORMALIZER" --hako "$hako_out" --c "$c_out" --out "$report_out"

rg -F -q 'mimalloc_comparison_memory_report=1' "$report_out"
rg -F -q 'output_contract=mimalloc-comparison-memory-report-v0' "$report_out"
rg -F -q 'hako_workload=representative-mixed-small-v0' "$report_out"
rg -F -q 'c_workload=representative-mixed-small-v0' "$report_out"
rg -F -q 'workload_match=1' "$report_out"
rg -F -q 'operation_family_match=1' "$report_out"
rg -F -q 'operation_sequence_match=1' "$report_out"
rg -F -q 'free_order_match=1' "$report_out"
rg -F -q 'allocation_count_delta=0' "$report_out"
rg -F -q 'free_count_delta=0' "$report_out"
rg -F -q 'requested_bytes_delta=0' "$report_out"
rg -F -q 'hako_requested_bytes=3096' "$report_out"
rg -F -q 'c_requested_bytes=3096' "$report_out"
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
int(values.get("peak_rss_bytes_delta", "0"))
print("[phase295x-mixed-size-evidence] ok")
PY

cat "$report_out"
echo "[$TAG] ok"
