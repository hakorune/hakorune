#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-hako-alloc-mimalloc-comparison-memory-report"
cd "$ROOT_DIR"
source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

H_REPORT="tools/allocator/mimalloc_comparison_memory_report.py"
H_RUNNER="tools/allocator/hako_exe_memory_runner.sh"
C_RUNNER="tools/allocator/c_mimalloc_explicit_runner.sh"
APP="apps/hako-alloc-mimalloc-comparison-huge-osvm-slice-proof/main.hako"
CARD="docs/development/current/main/phases/phase-294x/294x-161-MIMALLOC-COMPARISON-MEMORY-REPORT.md"
TASKBOARD="docs/development/current/main/phases/phase-294x/294x-90-usize-semantics-taskboard.md"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_hako_alloc_mimalloc_comparison_memory_report_guard.sh"

echo "[$TAG] checking normalized hako-vs-C mimalloc memory report"

guard_require_files "$TAG" "$H_REPORT" "$H_RUNNER" "$C_RUNNER" "$APP" "$CARD" "$TASKBOARD" "$INDEX" "$SELF_SCRIPT"
guard_require_exec_files "$TAG" "$H_REPORT" "$H_RUNNER" "$C_RUNNER" "$SELF_SCRIPT"

guard_expect_in_file "$TAG" 'mimalloc-comparison-memory-report-v0' "$H_REPORT" "normalizer must publish stable report contract"
guard_expect_in_file "$TAG" 'winner_claim=0' "$H_REPORT" "normalizer must keep winner claims closed"
guard_expect_in_file "$TAG" 'provider_activation=0' "$H_REPORT" "normalizer must keep provider activation closed"
guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-MEMORY-REPORT-001' "$CARD" "card must identify the row"
guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-MEMORY-REPORT-CLOSEOUT-001' "$CARD" "card must select the follow-on"
guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-MEMORY-REPORT-CLOSEOUT-001' "$TASKBOARD" "taskboard must expose follow-on blocker"
guard_expect_in_file "$TAG" "$H_REPORT" "$INDEX" "check index must list the normalizer"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list this guard"

if rg -n 'LD_PRELOAD|replace_process_allocator[[:space:]]*\(|install_hook[[:space:]]*\(|#\[global_allocator\]|winner_claim=1|provider_activation=1|host_replacement=1' "$H_REPORT" >/tmp/"$TAG".forbidden 2>&1; then
  echo "[$TAG] ERROR: memory report normalizer opened replacement/provider/winner seams" >&2
  cat /tmp/"$TAG".forbidden >&2
  rm -f /tmp/"$TAG".forbidden
  exit 1
fi
rm -f /tmp/"$TAG".forbidden

tmp_dir="$(mktemp -d /tmp/hakorune_mimalloc_memory_report.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT
hako_out="$tmp_dir/hako.out"
c_out="$tmp_dir/c.out"
report_out="$tmp_dir/report.out"

bash "$H_RUNNER" --app "$APP" --workload huge-osvm-v1 --out "$hako_out" >/tmp/"$TAG".hako_stdout
cat /tmp/"$TAG".hako_stdout
rm -f /tmp/"$TAG".hako_stdout

bash "$C_RUNNER" --out "$c_out" --allow-ldconfig-discovery >/tmp/"$TAG".c_stdout
cat /tmp/"$TAG".c_stdout
rm -f /tmp/"$TAG".c_stdout

python3 "$H_REPORT" --hako "$hako_out" --c "$c_out" --out "$report_out"

rg -F -q 'mimalloc_comparison_memory_report=1' "$report_out"
rg -F -q 'output_contract=mimalloc-comparison-memory-report-v0' "$report_out"
rg -F -q 'hako_workload=huge-osvm-v1' "$report_out"
rg -F -q 'c_workload=representative-small-block-v0' "$report_out"
rg -F -q 'workload_match=0' "$report_out"
rg -F -q 'hako_result_code=0' "$report_out"
rg -F -q 'c_result_code=0' "$report_out"
rg -F -q 'hako_memory_usage_evidence=1' "$report_out"
rg -F -q 'c_memory_usage_evidence=1' "$report_out"
rg -F -q 'provider_activation=0' "$report_out"
rg -F -q 'host_replacement=0' "$report_out"
rg -F -q 'hook_installed=0' "$report_out"
rg -F -q 'global_allocator_installed=0' "$report_out"
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

for key in ("hako_requested_bytes", "c_requested_bytes", "hako_peak_rss_bytes", "c_peak_rss_bytes"):
    value = int(values.get(key, "0"))
    if value <= 0:
        raise SystemExit(f"{key} must be positive, got {value}")
for key in ("requested_bytes_delta", "peak_rss_bytes_delta"):
    int(values.get(key, "0"))
print("[mimalloc-memory-report] ok")
PY

cat "$report_out"
echo "[$TAG] ok"
