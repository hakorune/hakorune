#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-phase295x-repeated-measurement-pack-run"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

CARD="docs/development/current/main/phases/phase-295x/295x-30-MIMALLOC-COMPARISON-REPEATED-MEASUREMENT-PACK-RUN.md"
PREV_CARD="docs/development/current/main/phases/phase-295x/295x-29-MIMALLOC-COMPARISON-REPEATED-MEASUREMENT-RUNNER.md"
TASKBOARD="docs/development/current/main/phases/phase-295x/295x-90-mimalloc-comparison-execution-taskboard.md"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase295x_repeated_measurement_pack_run_guard.sh"
RUNNER="tools/allocator/mimalloc_repeated_measurement_runner.py"

echo "[$TAG] checking phase-295x repeated measurement pack run"

guard_require_files "$TAG" "$CARD" "$PREV_CARD" "$TASKBOARD" "$INDEX" "$SELF_SCRIPT" "$RUNNER"
guard_require_exec_files "$TAG" "$SELF_SCRIPT" "$RUNNER"

guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-REPEATED-MEASUREMENT-PACK-RUN-295X-001' "$CARD" "card must identify the current blocker"
guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-REPEATED-MEASUREMENT-CLOSEOUT-295X-001' "$CARD" "card must select closeout follow-on"
guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-REPEATED-MEASUREMENT-PACK-RUN-295X-001' "$PREV_CARD" "previous row must select this pack run"
guard_expect_in_file "$TAG" '| 30 | `295x-30` | Landed |' "$TASKBOARD" "taskboard must retain this row as landed"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check script index must list this guard"
guard_expect_in_file "$TAG" 'winner_claim=0' "$RUNNER" "runner must keep winner claims closed"

tmp_dir="$(mktemp -d /tmp/hakorune_phase295x_repeated_pack.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT
out="$tmp_dir/repeated-pack.out"

python3 "$RUNNER" --out "$out" --sample-count 5 --warmup-count 1 --allow-ldconfig-discovery

rg -F -q 'mimalloc_repeated_measurement_runner=1' "$out"
rg -F -q 'output_contract=mimalloc-comparison-repeated-measurement-v0' "$out"
rg -F -q 'measurement_profile=phase295x-repeated-v0' "$out"
rg -F -q 'warmup_count=1' "$out"
rg -F -q 'sample_count=5' "$out"
rg -F -q 'workload_count=4' "$out"
rg -F -q 'workloads=representative-small-block-v0,representative-realloc-aligned-v0,representative-mixed-small-v0,representative-huge-ish-v0' "$out"
rg -F -q 'canonical_rss_collector=external-time' "$out"
rg -F -q 'workload_0_operation_family=small-block' "$out"
rg -F -q 'workload_1_operation_family=realloc-aligned' "$out"
rg -F -q 'workload_2_operation_family=mixed-small' "$out"
rg -F -q 'workload_3_operation_family=huge-ish' "$out"
rg -F -q 'winner_claim=0' "$out"
rg -F -q 'summary=ok' "$out"

python3 - "$out" <<'PY'
import sys

values = {}
with open(sys.argv[1], encoding="utf-8") as fh:
    for line in fh:
        line = line.strip()
        if "=" in line:
            key, value = line.split("=", 1)
            values[key] = value

for idx in range(4):
    for side in ("hako", "c"):
        vals = [
            int(values.get(f"workload_{idx}_{side}_external_rss_min_bytes", "0")),
            int(values.get(f"workload_{idx}_{side}_external_rss_median_bytes", "0")),
            int(values.get(f"workload_{idx}_{side}_external_rss_max_bytes", "0")),
        ]
        if any(value <= 0 for value in vals):
            raise SystemExit(f"workload {idx} {side} RSS values must be positive")
        if vals[0] > vals[2]:
            raise SystemExit(f"workload {idx} {side} RSS min/max inverted")
print("[phase295x-repeated-measurement-pack-run] ok")
PY

cat "$out"
echo "[$TAG] ok"
