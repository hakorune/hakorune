#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-phase295x-repeated-measurement-presentation"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

CARD="docs/development/current/main/phases/phase-295x/295x-32-MIMALLOC-COMPARISON-REPEATED-MEASUREMENT-PRESENTATION.md"
PREV_CARD="docs/development/current/main/phases/phase-295x/295x-31-MIMALLOC-COMPARISON-REPEATED-MEASUREMENT-CLOSEOUT.md"
TASKBOARD="docs/development/current/main/phases/phase-295x/295x-90-mimalloc-comparison-execution-taskboard.md"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase295x_repeated_measurement_presentation_guard.sh"
RUNNER="tools/allocator/mimalloc_repeated_measurement_runner.py"
PRESENTER="tools/allocator/mimalloc_repeated_measurement_presentation.py"

echo "[$TAG] checking phase-295x repeated measurement presentation"

guard_require_files "$TAG" "$CARD" "$PREV_CARD" "$TASKBOARD" "$INDEX" "$SELF_SCRIPT" "$RUNNER" "$PRESENTER"
guard_require_exec_files "$TAG" "$SELF_SCRIPT" "$RUNNER" "$PRESENTER"

guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-REPEATED-MEASUREMENT-PRESENTATION-295X-001' "$CARD" "card must identify the current blocker"
guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-MEMORY-GAP-ATTRIBUTION-SELECTION-295X-001' "$CARD" "card must select attribution follow-on"
guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-REPEATED-MEASUREMENT-PRESENTATION-295X-001' "$PREV_CARD" "previous row must select this presentation"
guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-MEMORY-GAP-ATTRIBUTION-SELECTION-295X-001' "$TASKBOARD" "taskboard must expose attribution follow-on"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check script index must list this guard"
guard_expect_in_file "$TAG" 'winner_claim=0' "$PRESENTER" "presenter must keep winner claims closed"
guard_expect_in_file "$TAG" 'presentation_only=1' "$PRESENTER" "presenter must be presentation-only"

tmp_dir="$(mktemp -d /tmp/hakorune_phase295x_repeated_presentation.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT
report_out="$tmp_dir/repeated.out"
presentation_out="$tmp_dir/presentation.out"

python3 "$RUNNER" \
  --out "$report_out" \
  --sample-count 5 \
  --warmup-count 1 \
  --allow-ldconfig-discovery
python3 "$PRESENTER" --report "$report_out" --out "$presentation_out"

rg -F -q 'mimalloc_repeated_measurement_presentation=1' "$presentation_out"
rg -F -q 'output_contract=mimalloc-comparison-repeated-measurement-presentation-v0' "$presentation_out"
rg -F -q 'input_contract=mimalloc-comparison-repeated-measurement-v0' "$presentation_out"
rg -F -q 'measurement_profile=phase295x-repeated-v0' "$presentation_out"
rg -F -q 'workload_count=4' "$presentation_out"
rg -F -q 'presentation_only=1' "$presentation_out"
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

for idx in range(4):
    for key in (
        f"workload_{idx}_hako_external_rss_median_bytes",
        f"workload_{idx}_c_external_rss_median_bytes",
        f"workload_{idx}_external_rss_median_delta_bytes",
    ):
        if key not in values:
            raise SystemExit(f"missing {key}")
    hako = int(values[f"workload_{idx}_hako_external_rss_median_bytes"])
    c = int(values[f"workload_{idx}_c_external_rss_median_bytes"])
    delta = int(values[f"workload_{idx}_external_rss_median_delta_bytes"])
    if hako <= 0 or c <= 0:
        raise SystemExit(f"workload {idx} median RSS must be positive")
    if hako - c != delta:
        raise SystemExit(f"workload {idx} median delta mismatch")
print("[phase295x-repeated-measurement-presentation] ok")
PY

cat "$presentation_out"
echo "[$TAG] ok"
