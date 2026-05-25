#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-phase295x-malloc-large-memory-gap-incremental"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

CARD="docs/development/current/main/phases/phase-295x/295x-192-MIMALLOC-COMPARISON-MALLOC-LARGE-MEMORY-GAP-INCREMENTAL.md"
PREV_CARD="docs/development/current/main/phases/phase-295x/295x-191-MIMALLOC-COMPARISON-MALLOC-LARGE-MEMORY-GAP-BASELINE.md"
NEXT_CARD="docs/development/current/main/phases/phase-295x/295x-193-MIMALLOC-COMPARISON-MALLOC-LARGE-MEMORY-GAP-CLOSEOUT.md"
TASKBOARD="docs/development/current/main/phases/phase-295x/295x-90-mimalloc-comparison-execution-taskboard.md"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase295x_malloc_large_memory_gap_incremental_guard.sh"
REPEATED_RUNNER="tools/allocator/mimalloc_repeated_measurement_runner.py"
INCREMENTAL_SCRIPT="tools/allocator/mimalloc_memory_gap_incremental.py"

echo "[$TAG] checking phase-295x malloc-large memory gap incremental"

guard_require_files "$TAG" "$CARD" "$PREV_CARD" "$TASKBOARD" "$INDEX" "$SELF_SCRIPT" "$REPEATED_RUNNER" "$INCREMENTAL_SCRIPT"
guard_require_exec_files "$TAG" "$SELF_SCRIPT" "$REPEATED_RUNNER" "$INCREMENTAL_SCRIPT"

guard_expect_in_file "$TAG" 'Status: Landed' "$CARD" "card must be landed after opening the closeout row"
guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-MALLOC-LARGE-MEMORY-GAP-INCREMENTAL-295X-001' "$CARD" "card must identify the current blocker"
guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-MALLOC-LARGE-MEMORY-GAP-CLOSEOUT-295X-001' "$CARD" "card must select closeout follow-on"
guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-MALLOC-LARGE-MEMORY-GAP-INCREMENTAL-295X-001' "$PREV_CARD" "previous row must select this incremental row"
guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-MALLOC-LARGE-MEMORY-GAP-CLOSEOUT-295X-001' "$NEXT_CARD" "next row must select closeout follow-on"
guard_expect_in_file "$TAG" 'mimalloc-comparison-memory-gap-incremental-v0' "$INCREMENTAL_SCRIPT" "incremental script must define stable output contract"
guard_expect_in_file "$TAG" 'winner_claim=0' "$INCREMENTAL_SCRIPT" "incremental script must keep winner claims closed"
guard_expect_in_file "$TAG" '| 191 | `MIMALLOC-COMPARISON-MALLOC-LARGE-MEMORY-GAP-BASELINE-295X-001` | Landed |' "$TASKBOARD" "taskboard must mark baseline landed"
guard_expect_in_file "$TAG" '| 192 | `MIMALLOC-COMPARISON-MALLOC-LARGE-MEMORY-GAP-INCREMENTAL-295X-001` | Landed |' "$TASKBOARD" "taskboard must mark incremental landed"
guard_expect_in_file "$TAG" '| 193 | `MIMALLOC-COMPARISON-MALLOC-LARGE-MEMORY-GAP-CLOSEOUT-295X-001` | Current |' "$TASKBOARD" "taskboard must expose the closeout row"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check script index must list this guard"

tmp_dir="$(mktemp -d /tmp/hakorune_phase295x_malloc_large_memory_gap_incremental.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT
baseline="$tmp_dir/baseline.out"
pack="$tmp_dir/pack.out"
out="$tmp_dir/incremental.out"

python3 "$REPEATED_RUNNER" \
  --out "$baseline" \
  --sample-count 5 \
  --warmup-count 1 \
  --workload representative-empty-v0 \
  --allow-ldconfig-discovery >/dev/null

python3 "$REPEATED_RUNNER" \
  --out "$pack" \
  --sample-count 5 \
  --warmup-count 1 \
  --allow-ldconfig-discovery >/dev/null

python3 "$INCREMENTAL_SCRIPT" --baseline "$baseline" --pack "$pack" --out "$out"

rg -F -q 'mimalloc_memory_gap_incremental=1' "$out"
rg -F -q 'output_contract=mimalloc-comparison-memory-gap-incremental-v0' "$out"
rg -F -q 'baseline_workload=representative-empty-v0' "$out"
rg -F -q 'workload_count=4' "$out"
rg -F -q 'workload_0_id=representative-small-block-v0' "$out"
rg -F -q 'workload_1_id=representative-realloc-aligned-v0' "$out"
rg -F -q 'workload_2_id=representative-mixed-small-v0' "$out"
rg -F -q 'workload_3_id=representative-huge-ish-v0' "$out"
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

fixed = int(values["fixed_process_runtime_baseline_delta_bytes"])
if fixed <= 0:
    raise SystemExit("fixed baseline delta should be positive before attribution")

for idx in range(4):
    total = int(values[f"workload_{idx}_total_delta_bytes"])
    incremental = int(values[f"workload_{idx}_incremental_delta_bytes"])
    residual = int(values[f"workload_{idx}_unattributed_residual_bytes"])
    if residual != 0:
        raise SystemExit(f"workload {idx} residual must be zero by decomposition")
    if total != fixed + incremental:
        raise SystemExit(f"workload {idx} decomposition mismatch")
    if values.get(f"workload_{idx}_winner_claim") != "0":
        raise SystemExit(f"workload {idx} must keep winner claim closed")

print("[phase295x-malloc-large-memory-gap-incremental] ok")
PY

cat "$out"
echo "[$TAG] ok"
