#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-phase295x-same-workload-refresh"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

CARD="docs/development/current/main/phases/phase-295x/295x-07-MIMALLOC-COMPARISON-SAME-WORKLOAD-REFRESH.md"
PREV_CARD="docs/development/current/main/phases/phase-295x/295x-06-MIMALLOC-COMPARISON-RESULT-LEDGER-CLOSEOUT.md"
TASKBOARD="docs/development/current/main/phases/phase-295x/295x-90-mimalloc-comparison-execution-taskboard.md"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase295x_same_workload_refresh_guard.sh"
SAME_WORKLOAD_GUARD="tools/checks/k2_wide_hako_alloc_mimalloc_comparison_same_workload_memory_report_guard.sh"
NORMALIZER="tools/allocator/mimalloc_comparison_memory_report.py"
H_RUNNER="tools/allocator/hako_exe_memory_runner.sh"
C_RUNNER="tools/allocator/c_mimalloc_explicit_runner.sh"
APP="apps/hako-alloc-mimalloc-comparison-representative-small-block-proof/main.hako"

echo "[$TAG] checking phase-295x same-workload comparison refresh"

guard_require_files \
  "$TAG" \
  "$CARD" \
  "$PREV_CARD" \
  "$TASKBOARD" \
  "$INDEX" \
  "$SELF_SCRIPT" \
  "$SAME_WORKLOAD_GUARD" \
  "$NORMALIZER" \
  "$H_RUNNER" \
  "$C_RUNNER" \
  "$APP"

guard_require_exec_files "$TAG" "$SELF_SCRIPT" "$SAME_WORKLOAD_GUARD" "$NORMALIZER" "$H_RUNNER" "$C_RUNNER"

guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-SAME-WORKLOAD-295X-REFRESH-001' "$CARD" "card must identify the current blocker"
guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-SAME-WORKLOAD-CLOSEOUT-295X-001' "$CARD" "card must select the same-workload closeout follow-on"
guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-SAME-WORKLOAD-295X-REFRESH-001' "$PREV_CARD" "previous row must select this refresh"
guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-SAME-WORKLOAD-CLOSEOUT-295X-001' "$TASKBOARD" "taskboard must expose closeout follow-on"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check script index must list this guard"
guard_expect_in_file "$TAG" "$SAME_WORKLOAD_GUARD" "$INDEX" "check script index must list the reused same-workload guard"
guard_expect_in_file "$TAG" 'workload_match=1' "$CARD" "same-workload refresh must require matching workload ids"
guard_expect_in_file "$TAG" 'requested_bytes_delta=0' "$CARD" "same-workload refresh must require equal requested bytes"
guard_expect_in_file "$TAG" 'winner_claim=0' "$CARD" "same-workload refresh must keep winner claims closed"
guard_expect_in_file "$TAG" 'workload=representative-small-block-v0' "$APP" "hako proof app must use C runner workload id"
guard_expect_in_file "$TAG" 'requested bytes", page.requested_bytes, 33254' "$APP" "hako proof app must mirror C requested bytes"

if rg -n 'LD_PRELOAD|replace_process_allocator[[:space:]]*\(|install_hook[[:space:]]*\(|#\[global_allocator\]|winner_claim=1|provider_activation=1|host_replacement=1|thread::|worker_local|nowait|await' "$APP" "$NORMALIZER" >/tmp/"$TAG".forbidden 2>&1; then
  echo "[$TAG] ERROR: same-workload refresh opened replacement/provider/thread/winner seams" >&2
  cat /tmp/"$TAG".forbidden >&2
  rm -f /tmp/"$TAG".forbidden
  exit 1
fi
rm -f /tmp/"$TAG".forbidden

tmp_dir="$(mktemp -d /tmp/hakorune_phase295x_same_workload_refresh.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT
hako_out="$tmp_dir/hako.out"
c_out="$tmp_dir/c.out"
report_out="$tmp_dir/report.out"

bash "$H_RUNNER" --app "$APP" --workload representative-small-block-v0 --out "$hako_out"
bash "$C_RUNNER" --out "$c_out" --allow-ldconfig-discovery
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
print("[phase295x-same-workload-refresh] ok")
PY

cat "$report_out"

echo "[$TAG] ok"
