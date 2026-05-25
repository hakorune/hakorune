#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-phase295x-malloc-large-memory-gap-baseline"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

CARD="docs/development/current/main/phases/phase-295x/295x-191-MIMALLOC-COMPARISON-MALLOC-LARGE-MEMORY-GAP-BASELINE.md"
PREV_CARD="docs/development/current/main/phases/phase-295x/295x-190-MIMALLOC-COMPARISON-MALLOC-LARGE-MEMORY-GAP-ATTRIBUTION-SELECTION.md"
SSOT="docs/development/current/main/design/mimalloc-comparison-execution-ssot.md"
TASKBOARD="docs/development/current/main/phases/phase-295x/295x-90-mimalloc-comparison-execution-taskboard.md"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase295x_malloc_large_memory_gap_baseline_guard.sh"
RUNNER="tools/allocator/mimalloc_repeated_measurement_runner.py"
C_RUNNER="tools/allocator/c_mimalloc_explicit_runner.c"
APP="apps/hako-alloc-mimalloc-comparison-empty-exe-proof/main.hako"

echo "[$TAG] checking phase-295x malloc-large memory gap baseline"

guard_require_files "$TAG" "$CARD" "$PREV_CARD" "$SSOT" "$TASKBOARD" "$INDEX" "$SELF_SCRIPT" "$RUNNER" "$C_RUNNER" "$APP"
guard_require_exec_files "$TAG" "$SELF_SCRIPT" "$RUNNER"

guard_expect_in_file "$TAG" 'Status: Landed' "$CARD" "card must be landed after opening the incremental row"
guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-MALLOC-LARGE-MEMORY-GAP-BASELINE-295X-001' "$CARD" "card must identify the current blocker"
guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-MALLOC-LARGE-MEMORY-GAP-INCREMENTAL-295X-001' "$CARD" "card must select incremental follow-on"
guard_expect_in_file "$TAG" 'workload=representative-empty-v0' "$CARD" "card must select empty baseline workload"
guard_expect_in_file "$TAG" 'operation_family=empty-baseline' "$CARD" "card must select empty baseline operation family"
guard_expect_in_file "$TAG" 'operation_sequence_id=representative-empty-v0-seq' "$CARD" "card must select empty baseline operation sequence"
guard_expect_in_file "$TAG" 'free_order_id=no-release-v0' "$CARD" "card must select empty baseline free order"
guard_expect_in_file "$TAG" 'measurement_profile=phase295x-repeated-v0' "$CARD" "card must keep repeated measurement profile"
guard_expect_in_file "$TAG" 'warmup_count=1' "$CARD" "card must keep warmup count"
guard_expect_in_file "$TAG" 'sample_count=5' "$CARD" "card must keep sample count"
guard_expect_in_file "$TAG" 'canonical_rss_collector=external-time' "$CARD" "card must keep canonical RSS collector"
guard_expect_in_file "$TAG" 'winner_claim=0' "$CARD" "card must keep winner claims closed"
guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-MALLOC-LARGE-MEMORY-GAP-BASELINE-295X-001' "$PREV_CARD" "previous row must select this baseline seam"
guard_expect_in_file "$TAG" 'Status: Landed' "$PREV_CARD" "previous row must be landed"
guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-MALLOC-LARGE-MEMORY-GAP-BASELINE-295X-001' "$TASKBOARD" "taskboard must expose the baseline row"
guard_expect_in_file "$TAG" '| 190 | `MIMALLOC-COMPARISON-MALLOC-LARGE-MEMORY-GAP-ATTRIBUTION-SELECTION-295X-001` | Landed |' "$TASKBOARD" "taskboard must keep the attribution row landed"
guard_expect_in_file "$TAG" '| 191 | `MIMALLOC-COMPARISON-MALLOC-LARGE-MEMORY-GAP-BASELINE-295X-001` | Landed |' "$TASKBOARD" "taskboard must mark the baseline row landed"
guard_expect_in_file "$TAG" 'Memory Gap Attribution' "$SSOT" "SSOT must define attribution policy"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check script index must list this guard"

tmp_dir="$(mktemp -d /tmp/hakorune_phase295x_malloc_large_memory_gap_baseline.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT
out="$tmp_dir/baseline.out"

python3 "$RUNNER" \
  --out "$out" \
  --sample-count 5 \
  --warmup-count 1 \
  --workload representative-empty-v0 \
  --allow-ldconfig-discovery

rg -F -q 'mimalloc_repeated_measurement_runner=1' "$out"
rg -F -q 'output_contract=mimalloc-comparison-repeated-measurement-v0' "$out"
rg -F -q 'measurement_profile=phase295x-repeated-v0' "$out"
rg -F -q 'warmup_count=1' "$out"
rg -F -q 'sample_count=5' "$out"
rg -F -q 'workload_count=1' "$out"
rg -F -q 'workloads=representative-empty-v0' "$out"
rg -F -q 'workload_0_operation_family=empty-baseline' "$out"
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

for side in ("hako", "c"):
    median = int(values.get(f"workload_0_{side}_external_rss_median_bytes", "0"))
    if median <= 0:
        raise SystemExit(f"{side} baseline median RSS must be positive")

if values.get("workload_0_winner_claim") != "0" or values.get("winner_claim") != "0":
    raise SystemExit("baseline evidence must not make winner claims")

print("[phase295x-malloc-large-memory-gap-baseline] ok")
PY

cat "$out"
echo "[$TAG] ok"
