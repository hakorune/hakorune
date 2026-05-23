#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-hako-alloc-mimalloc-comparison-same-workload-memory-report"
cd "$ROOT_DIR"
source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

NORMALIZER="tools/allocator/mimalloc_comparison_memory_report.py"
H_RUNNER="tools/allocator/hako_exe_memory_runner.sh"
C_RUNNER="tools/allocator/c_mimalloc_explicit_runner.sh"
APP="apps/hako-alloc-mimalloc-comparison-representative-small-block-proof/main.hako"
APP_README="apps/hako-alloc-mimalloc-comparison-representative-small-block-proof/README.md"
APP_TEST="apps/hako-alloc-mimalloc-comparison-representative-small-block-proof/test.sh"
CARD="docs/development/current/main/phases/phase-294x/294x-162-MIMALLOC-COMPARISON-SAME-WORKLOAD-MEMORY-REPORT.md"
TASKBOARD="docs/development/current/main/phases/phase-294x/294x-90-usize-semantics-taskboard.md"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_hako_alloc_mimalloc_comparison_same_workload_memory_report_guard.sh"

echo "[$TAG] checking same-workload hako-vs-C mimalloc memory report"

guard_require_files "$TAG" "$NORMALIZER" "$H_RUNNER" "$C_RUNNER" "$APP" "$APP_README" "$APP_TEST" "$CARD" "$TASKBOARD" "$INDEX" "$SELF_SCRIPT"
guard_require_exec_files "$TAG" "$NORMALIZER" "$H_RUNNER" "$C_RUNNER" "$APP_TEST" "$SELF_SCRIPT"

guard_expect_in_file "$TAG" 'workload=representative-small-block-v0' "$APP" "hako proof app must use C runner workload id"
guard_expect_in_file "$TAG" 'proof.expectEq\("requested bytes", page.requested_bytes, 33254\)' "$APP" "hako proof app must mirror C requested bytes"
guard_expect_in_file "$TAG" 'winner_claim=0' "$NORMALIZER" "normalizer must keep winner claims closed"
guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-SAME-WORKLOAD-PACK-001' "$CARD" "card must identify the row"
guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-SAME-WORKLOAD-PACK-CLOSEOUT-001' "$CARD" "card must select the follow-on"
guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-SAME-WORKLOAD-PACK-CLOSEOUT-001' "$TASKBOARD" "taskboard must expose follow-on blocker"
guard_expect_in_file "$TAG" "$NORMALIZER" "$INDEX" "check index must list the memory report normalizer"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list this guard"

if rg -n 'LD_PRELOAD|replace_process_allocator[[:space:]]*\(|install_hook[[:space:]]*\(|#\[global_allocator\]|winner_claim=1|provider_activation=1|host_replacement=1|thread::|worker_local|nowait|await' "$APP" "$NORMALIZER" >/tmp/"$TAG".forbidden 2>&1; then
  echo "[$TAG] ERROR: same-workload report opened replacement/provider/thread/winner seams" >&2
  cat /tmp/"$TAG".forbidden >&2
  rm -f /tmp/"$TAG".forbidden
  exit 1
fi
rm -f /tmp/"$TAG".forbidden

tmp_dir="$(mktemp -d /tmp/hakorune_mimalloc_same_workload_memory_report.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT
hako_out="$tmp_dir/hako.out"
c_out="$tmp_dir/c.out"
report_out="$tmp_dir/report.out"

bash "$H_RUNNER" --app "$APP" --workload representative-small-block-v0 --out "$hako_out" >/tmp/"$TAG".hako_stdout
cat /tmp/"$TAG".hako_stdout
rm -f /tmp/"$TAG".hako_stdout

bash "$C_RUNNER" --out "$c_out" --allow-ldconfig-discovery >/tmp/"$TAG".c_stdout
cat /tmp/"$TAG".c_stdout
rm -f /tmp/"$TAG".c_stdout

python3 "$NORMALIZER" --hako "$hako_out" --c "$c_out" --out "$report_out"

rg -F -q 'mimalloc_comparison_memory_report=1' "$report_out"
rg -F -q 'output_contract=mimalloc-comparison-memory-report-v0' "$report_out"
rg -F -q 'hako_workload=representative-small-block-v0' "$report_out"
rg -F -q 'c_workload=representative-small-block-v0' "$report_out"
rg -F -q 'workload_match=1' "$report_out"
rg -F -q 'hako_requested_bytes=33254' "$report_out"
rg -F -q 'c_requested_bytes=33254' "$report_out"
rg -F -q 'requested_bytes_delta=0' "$report_out"
rg -F -q 'hako_memory_usage_evidence=1' "$report_out"
rg -F -q 'c_memory_usage_evidence=1' "$report_out"
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
    value = int(values.get(key, "0"))
    if value <= 0:
        raise SystemExit(f"{key} must be positive, got {value}")
int(values.get("peak_rss_bytes_delta", "0"))
print("[same-workload-memory-report] ok")
PY

cat "$report_out"
echo "[$TAG] ok"
