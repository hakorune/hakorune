#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-hako-alloc-mimalloc-comparison-repeated-run-evidence"
cd "$ROOT_DIR"
source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

NORMALIZER="tools/allocator/mimalloc_comparison_memory_report.py"
PRESENTER="tools/allocator/mimalloc_comparison_rss_presentation.py"
AGGREGATOR="tools/allocator/mimalloc_comparison_repeated_run_evidence.py"
H_RUNNER="tools/allocator/hako_exe_memory_runner.sh"
C_RUNNER="tools/allocator/c_mimalloc_explicit_runner.sh"
APP="apps/hako-alloc-mimalloc-comparison-representative-small-block-proof/main.hako"
CARD="docs/development/current/main/phases/phase-294x/294x-166-MIMALLOC-COMPARISON-REPEATED-RUN-EVIDENCE.md"
TASKBOARD="docs/development/current/main/phases/phase-294x/294x-90-usize-semantics-taskboard.md"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_hako_alloc_mimalloc_comparison_repeated_run_evidence_guard.sh"
SAMPLE_COUNT=3

echo "[$TAG] checking repeated-run evidence"

guard_require_files "$TAG" "$NORMALIZER" "$PRESENTER" "$AGGREGATOR" "$H_RUNNER" "$C_RUNNER" "$APP" "$CARD" "$TASKBOARD" "$INDEX" "$SELF_SCRIPT"
guard_require_exec_files "$TAG" "$NORMALIZER" "$PRESENTER" "$AGGREGATOR" "$H_RUNNER" "$C_RUNNER" "$SELF_SCRIPT"

guard_expect_in_file "$TAG" 'mimalloc-comparison-repeated-run-evidence-v0' "$AGGREGATOR" "aggregator must publish stable repeated-run evidence contract"
guard_expect_in_file "$TAG" 'winner_claim=0' "$AGGREGATOR" "aggregator must keep winner claims closed"
guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-REPEATED-RUN-EVIDENCE-001' "$CARD" "card must identify the row"
guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-REPEATED-RUN-EVIDENCE-CLOSEOUT-001' "$CARD" "card must select the follow-on"
guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-REPEATED-RUN-EVIDENCE-CLOSEOUT-001' "$TASKBOARD" "taskboard must expose follow-on blocker"
guard_expect_in_file "$TAG" "$AGGREGATOR" "$INDEX" "check index must list the aggregator"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list this guard"

if rg -n 'winner_claim=1|LD_PRELOAD|replace_process_allocator[[:space:]]*\(|install_hook[[:space:]]*\(|#\[global_allocator\]|provider_activation=1|host_replacement=1' "$AGGREGATOR" >/tmp/"$TAG".forbidden 2>&1; then
  echo "[$TAG] ERROR: repeated-run aggregator opened winner/replacement seams" >&2
  cat /tmp/"$TAG".forbidden >&2
  rm -f /tmp/"$TAG".forbidden
  exit 1
fi
rm -f /tmp/"$TAG".forbidden

tmp_dir="$(mktemp -d /tmp/hakorune_mimalloc_repeated_run.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT

samples=()
for sample_id in $(seq 1 "$SAMPLE_COUNT"); do
  hako_out="$tmp_dir/hako.$sample_id.out"
  c_out="$tmp_dir/c.$sample_id.out"
  report_out="$tmp_dir/report.$sample_id.out"
  presentation_out="$tmp_dir/rss.$sample_id.out"

  bash "$H_RUNNER" --app "$APP" --workload representative-small-block-v0 --out "$hako_out" >/tmp/"$TAG".hako_stdout
  cat /tmp/"$TAG".hako_stdout
  rm -f /tmp/"$TAG".hako_stdout

  bash "$C_RUNNER" --out "$c_out" --allow-ldconfig-discovery >/tmp/"$TAG".c_stdout
  cat /tmp/"$TAG".c_stdout
  rm -f /tmp/"$TAG".c_stdout

  python3 "$NORMALIZER" --hako "$hako_out" --c "$c_out" --out "$report_out"
  python3 "$PRESENTER" --report "$report_out" --out "$presentation_out"
  samples+=(--sample "$presentation_out")
done

aggregate_out="$tmp_dir/repeated.out"
python3 "$AGGREGATOR" "${samples[@]}" --out "$aggregate_out"

rg -F -q 'mimalloc_comparison_repeated_run_evidence=1' "$aggregate_out"
rg -F -q 'output_contract=mimalloc-comparison-repeated-run-evidence-v0' "$aggregate_out"
rg -F -q 'measurement_scope=repeated-rss-samples' "$aggregate_out"
rg -F -q "sample_count=$SAMPLE_COUNT" "$aggregate_out"
rg -F -q 'workload_match=1' "$aggregate_out"
rg -F -q 'requested_bytes_delta=0' "$aggregate_out"
rg -F -q 'winner_claim=0' "$aggregate_out"
rg -F -q 'summary=ok' "$aggregate_out"

python3 - "$aggregate_out" <<'PY'
import sys

values = {}
with open(sys.argv[1], encoding="utf-8") as fh:
    for line in fh:
        line = line.strip()
        if "=" in line:
            key, value = line.split("=", 1)
            values[key] = value

for key in (
    "hako_peak_rss_min_bytes",
    "hako_peak_rss_max_bytes",
    "c_peak_rss_min_bytes",
    "c_peak_rss_max_bytes",
    "peak_rss_abs_delta_min_bytes",
    "peak_rss_abs_delta_max_bytes",
):
    value = int(values.get(key, "0"))
    if value <= 0:
        raise SystemExit(f"{key} must be positive, got {value}")
if int(values["hako_peak_rss_min_bytes"]) > int(values["hako_peak_rss_max_bytes"]):
    raise SystemExit("hako RSS min/max inverted")
if int(values["c_peak_rss_min_bytes"]) > int(values["c_peak_rss_max_bytes"]):
    raise SystemExit("C RSS min/max inverted")
print("[repeated-run-evidence] ok")
PY

cat "$aggregate_out"
echo "[$TAG] ok"
