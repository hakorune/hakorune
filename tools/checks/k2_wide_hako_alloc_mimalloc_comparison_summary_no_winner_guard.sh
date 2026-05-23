#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-hako-alloc-mimalloc-comparison-summary-no-winner"
cd "$ROOT_DIR"
source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

REPEATED_GUARD="tools/checks/k2_wide_hako_alloc_mimalloc_comparison_repeated_run_evidence_guard.sh"
AGGREGATOR="tools/allocator/mimalloc_comparison_repeated_run_evidence.py"
SUMMARY="tools/allocator/mimalloc_comparison_summary_no_winner.py"
CARD="docs/development/current/main/phases/phase-294x/294x-168-MIMALLOC-COMPARISON-SUMMARY-NO-WINNER.md"
TASKBOARD="docs/development/current/main/phases/phase-294x/294x-90-usize-semantics-taskboard.md"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_hako_alloc_mimalloc_comparison_summary_no_winner_guard.sh"

echo "[$TAG] checking no-winner comparison summary"

guard_require_files "$TAG" "$REPEATED_GUARD" "$AGGREGATOR" "$SUMMARY" "$CARD" "$TASKBOARD" "$INDEX" "$SELF_SCRIPT"
guard_require_exec_files "$TAG" "$REPEATED_GUARD" "$AGGREGATOR" "$SUMMARY" "$SELF_SCRIPT"

guard_expect_in_file "$TAG" 'mimalloc-comparison-summary-no-winner-v0' "$SUMMARY" "summary formatter must publish stable no-winner contract"
guard_expect_in_file "$TAG" 'winner_claim=0' "$SUMMARY" "summary formatter must keep winner claims closed"
guard_expect_in_file "$TAG" 'comparison_claim=range-only' "$SUMMARY" "summary formatter must expose range-only claim"
guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-SUMMARY-NO-WINNER-001' "$CARD" "card must identify the row"
guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-SUMMARY-NO-WINNER-CLOSEOUT-001' "$CARD" "card must select the follow-on"
guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-SUMMARY-NO-WINNER-CLOSEOUT-001' "$TASKBOARD" "taskboard must expose follow-on blocker"
guard_expect_in_file "$TAG" "$SUMMARY" "$INDEX" "check index must list the formatter"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list this guard"

if rg -n 'winner_claim=1|comparison_claim=winner|LD_PRELOAD|replace_process_allocator[[:space:]]*\(|install_hook[[:space:]]*\(|#\[global_allocator\]|provider_activation=1|host_replacement=1' "$SUMMARY" >/tmp/"$TAG".forbidden 2>&1; then
  echo "[$TAG] ERROR: no-winner summary opened winner/replacement seams" >&2
  cat /tmp/"$TAG".forbidden >&2
  rm -f /tmp/"$TAG".forbidden
  exit 1
fi
rm -f /tmp/"$TAG".forbidden

tmp_dir="$(mktemp -d /tmp/hakorune_mimalloc_summary_no_winner.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT
repeated_stdout="$tmp_dir/repeated.guard.stdout"
evidence_out="$tmp_dir/repeated.out"
summary_out="$tmp_dir/summary.out"

bash "$REPEATED_GUARD" >"$repeated_stdout"
awk '
  /^mimalloc_comparison_repeated_run_evidence=1$/ {capture=1; block=$0 "\n"; next}
  capture && /^output_contract=mimalloc-comparison-repeated-run-evidence-v0$/ {block=block $0 "\n"; next}
  capture && /^[A-Za-z0-9_]+=/ {block=block $0 "\n"; next}
  capture && /^\[/ {last=block; capture=0; next}
  capture {block=block $0 "\n"}
  END {if (capture) last=block; printf "%s", last}
' "$repeated_stdout" >"$evidence_out"

python3 "$SUMMARY" --evidence "$evidence_out" --out "$summary_out"

rg -F -q 'mimalloc_comparison_summary_no_winner=1' "$summary_out"
rg -F -q 'output_contract=mimalloc-comparison-summary-no-winner-v0' "$summary_out"
rg -F -q 'measurement_scope=repeated-rss-samples' "$summary_out"
rg -F -q 'comparison_claim=range-only' "$summary_out"
rg -F -q 'winner_claim=0' "$summary_out"
rg -F -q 'summary=ok' "$summary_out"

python3 - "$summary_out" <<'PY'
import sys

values = {}
with open(sys.argv[1], encoding="utf-8") as fh:
    for line in fh:
        line = line.strip()
        if "=" in line:
            key, value = line.split("=", 1)
            values[key] = value

if int(values.get("sample_count", "0")) < 2:
    raise SystemExit("sample_count must be at least 2")
for key in (
    "hako_peak_rss_range_bytes",
    "c_peak_rss_range_bytes",
    "peak_rss_delta_range_bytes",
    "peak_rss_abs_delta_range_bytes",
):
    if ".." not in values.get(key, ""):
        raise SystemExit(f"{key} must be a range")
print("[summary-no-winner] ok")
PY

cat "$summary_out"
echo "[$TAG] ok"
