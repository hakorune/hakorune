#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-phase295x-memory-gap-baseline"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

CARD="docs/development/current/main/phases/phase-295x/295x-34-MIMALLOC-COMPARISON-MEMORY-GAP-BASELINE.md"
PREV_CARD="docs/development/current/main/phases/phase-295x/295x-33-MIMALLOC-COMPARISON-MEMORY-GAP-ATTRIBUTION-SELECTION.md"
TASKBOARD="docs/development/current/main/phases/phase-295x/295x-90-mimalloc-comparison-execution-taskboard.md"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase295x_memory_gap_baseline_guard.sh"
RUNNER="tools/allocator/mimalloc_repeated_measurement_runner.py"
C_RUNNER="tools/allocator/c_mimalloc_explicit_runner.c"
APP="apps/hako-alloc-mimalloc-comparison-empty-exe-proof/main.hako"

echo "[$TAG] checking phase-295x memory gap baseline evidence"

guard_require_files "$TAG" "$CARD" "$PREV_CARD" "$TASKBOARD" "$INDEX" "$SELF_SCRIPT" "$RUNNER" "$C_RUNNER" "$APP"
guard_require_exec_files "$TAG" "$SELF_SCRIPT" "$RUNNER"

guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-MEMORY-GAP-BASELINE-295X-001' "$CARD" "card must identify the current blocker"
guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-MEMORY-GAP-INCREMENTAL-295X-001' "$CARD" "card must select incremental follow-on"
guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-MEMORY-GAP-BASELINE-295X-001' "$PREV_CARD" "previous row must select this baseline row"
guard_expect_in_file "$TAG" 'representative-empty-v0' "$C_RUNNER" "C runner must support the empty workload"
guard_expect_in_file "$TAG" 'representative-empty-v0' "$RUNNER" "repeated runner must route the empty workload"
guard_expect_in_file "$TAG" 'DEFAULT_WORKLOADS' "$RUNNER" "repeated runner must keep a default workload pack"
guard_expect_in_file "$TAG" 'representative-empty-v0' "$APP" ".hako app must emit the empty workload id"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check script index must list this guard"

tmp_dir="$(mktemp -d /tmp/hakorune_phase295x_memory_gap_baseline.XXXXXX)"
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

print("[phase295x-memory-gap-baseline] ok")
PY

cat "$out"
echo "[$TAG] ok"
