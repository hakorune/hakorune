#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-hako-alloc-mimalloc-comparison-rss-presentation"
cd "$ROOT_DIR"
source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

NORMALIZER="tools/allocator/mimalloc_comparison_memory_report.py"
PRESENTER="tools/allocator/mimalloc_comparison_rss_presentation.py"
H_RUNNER="tools/allocator/hako_exe_memory_runner.sh"
C_RUNNER="tools/allocator/c_mimalloc_explicit_runner.sh"
APP="apps/hako-alloc-mimalloc-comparison-representative-small-block-proof/main.hako"
CARD="docs/development/current/main/phases/phase-294x/294x-164-MIMALLOC-COMPARISON-RSS-PRESENTATION.md"
TASKBOARD="docs/development/current/main/phases/phase-294x/294x-90-usize-semantics-taskboard.md"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_hako_alloc_mimalloc_comparison_rss_presentation_guard.sh"

echo "[$TAG] checking RSS presentation report"

guard_require_files "$TAG" "$NORMALIZER" "$PRESENTER" "$H_RUNNER" "$C_RUNNER" "$APP" "$CARD" "$TASKBOARD" "$INDEX" "$SELF_SCRIPT"
guard_require_exec_files "$TAG" "$NORMALIZER" "$PRESENTER" "$H_RUNNER" "$C_RUNNER" "$SELF_SCRIPT"

guard_expect_in_file "$TAG" 'mimalloc-comparison-rss-presentation-v0' "$PRESENTER" "presenter must publish stable RSS presentation contract"
guard_expect_in_file "$TAG" 'measurement_scope=single-run' "$PRESENTER" "presenter must label single-run evidence"
guard_expect_in_file "$TAG" 'winner_claim=0' "$PRESENTER" "presenter must keep winner claims closed"
guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-RSS-PRESENTATION-001' "$CARD" "card must identify the row"
guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-RSS-PRESENTATION-CLOSEOUT-001' "$CARD" "card must select the follow-on"
guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-RSS-PRESENTATION-CLOSEOUT-001' "$TASKBOARD" "taskboard must expose follow-on blocker"
guard_expect_in_file "$TAG" "$PRESENTER" "$INDEX" "check index must list the presenter"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list this guard"

if rg -n 'winner_claim=1|repeated_runs=1|LD_PRELOAD|replace_process_allocator[[:space:]]*\(|install_hook[[:space:]]*\(|#\[global_allocator\]|provider_activation=1|host_replacement=1' "$PRESENTER" >/tmp/"$TAG".forbidden 2>&1; then
  echo "[$TAG] ERROR: RSS presenter opened repeated-run/winner/replacement seams" >&2
  cat /tmp/"$TAG".forbidden >&2
  rm -f /tmp/"$TAG".forbidden
  exit 1
fi
rm -f /tmp/"$TAG".forbidden

tmp_dir="$(mktemp -d /tmp/hakorune_mimalloc_rss_presentation.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT
hako_out="$tmp_dir/hako.out"
c_out="$tmp_dir/c.out"
report_out="$tmp_dir/report.out"
presentation_out="$tmp_dir/rss.out"

bash "$H_RUNNER" --app "$APP" --workload representative-small-block-v0 --out "$hako_out" >/tmp/"$TAG".hako_stdout
cat /tmp/"$TAG".hako_stdout
rm -f /tmp/"$TAG".hako_stdout

bash "$C_RUNNER" --out "$c_out" --allow-ldconfig-discovery >/tmp/"$TAG".c_stdout
cat /tmp/"$TAG".c_stdout
rm -f /tmp/"$TAG".c_stdout

python3 "$NORMALIZER" --hako "$hako_out" --c "$c_out" --out "$report_out"
python3 "$PRESENTER" --report "$report_out" --out "$presentation_out"

rg -F -q 'mimalloc_comparison_rss_presentation=1' "$presentation_out"
rg -F -q 'output_contract=mimalloc-comparison-rss-presentation-v0' "$presentation_out"
rg -F -q 'measurement_scope=single-run' "$presentation_out"
rg -F -q 'rss_unit=bytes' "$presentation_out"
rg -F -q 'workload_match=1' "$presentation_out"
rg -F -q 'requested_bytes_delta=0' "$presentation_out"
rg -F -q 'repeated_runs=0' "$presentation_out"
rg -F -q 'winner_claim=0' "$presentation_out"
rg -F -q 'summary=ok' "$presentation_out"

python3 - "$presentation_out" <<'PY'
import sys

values = {}
with open(sys.argv[1], encoding="utf-8") as fh:
    for line in fh:
        line = line.strip()
        if "=" in line:
            key, value = line.split("=", 1)
            values[key] = value

for key in ("hako_peak_rss_bytes", "c_peak_rss_bytes", "peak_rss_abs_delta_bytes"):
    value = int(values.get(key, "0"))
    if value <= 0:
        raise SystemExit(f"{key} must be positive, got {value}")
for key in ("peak_rss_bytes_delta", "hako_peak_rss_mib_x100", "c_peak_rss_mib_x100", "peak_rss_abs_delta_mib_x100"):
    int(values.get(key, "0"))
print("[rss-presentation] ok")
PY

cat "$presentation_out"
echo "[$TAG] ok"
