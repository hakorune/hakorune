#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-phase295x-mimalloc-abandoned-heap-stress-speed-stability-observation-pack"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

CARD="docs/development/current/main/phases/phase-295x/295x-231-MIMALLOC-COMPARISON-ABANDONED-HEAP-STRESS-SPEED-STABILITY-OBSERVATION-PACK.md"
PREV_CARD="docs/development/current/main/phases/phase-295x/295x-230-MIMALLOC-COMPARISON-ABANDONED-HEAP-STRESS-NYRT-PLUGIN-LOADSET-SMALLER-DEFAULT-SET-CLOSEOUT.md"
TASKBOARD="docs/development/current/main/phases/phase-295x/295x-90-mimalloc-comparison-execution-taskboard.md"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase295x_mimalloc_abandoned_heap_stress_speed_stability_observation_pack_guard.sh"
RUNNER="tools/allocator/mimalloc_repeated_measurement_runner.py"
HAKO_RUNNER="tools/allocator/hako_exe_memory_runner.sh"
C_RUNNER="tools/allocator/c_mimalloc_explicit_runner.sh"

echo "[$TAG] checking phase-295x abandoned-heap stress speed/stability observation pack"

guard_require_files "$TAG" "$CARD" "$PREV_CARD" "$TASKBOARD" "$INDEX" "$SELF_SCRIPT" "$RUNNER" "$HAKO_RUNNER" "$C_RUNNER"
guard_require_exec_files "$TAG" "$SELF_SCRIPT" "$RUNNER" "$HAKO_RUNNER" "$C_RUNNER"

guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-ABANDONED-HEAP-STRESS-SPEED-STABILITY-OBSERVATION-PACK-295X-002' "$CARD" "card must identify current blocker"
guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-ABANDONED-HEAP-STRESS-SPEED-STABILITY-OBSERVATION-CLOSEOUT-295X-002' "$CARD" "card must select closeout follow-on"
guard_expect_in_file "$TAG" 'external_elapsed_median_ms' "$CARD" "card must define elapsed timing evidence"
guard_expect_in_file "$TAG" 'winner_claim=0' "$CARD" "card must keep winner claims closed"
guard_expect_in_file "$TAG" 'representative-small-block-v0' "$CARD" "card must keep small-block bench"
guard_expect_in_file "$TAG" 'representative-realloc-aligned-v0' "$CARD" "card must keep realloc/aligned bench"
guard_expect_in_file "$TAG" 'representative-mixed-small-v0' "$CARD" "card must keep mixed-small bench"
guard_expect_in_file "$TAG" 'representative-huge-ish-v0' "$CARD" "card must keep huge-ish bench"
guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-ABANDONED-HEAP-STRESS-SPEED-STABILITY-OBSERVATION-PACK-295X-002' "$TASKBOARD" "taskboard must expose selected follow-on"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check script index must list this guard"

tmp_dir="$(mktemp -d /tmp/hakorune_phase295x_speed_stability_abandoned.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT
out="$tmp_dir/speed-stability.out"

python3 "$RUNNER" \
  --out "$out" \
  --sample-count 5 \
  --warmup-count 1 \
  --hako-runtime-config empty \
  --allow-ldconfig-discovery

rg -F -q 'output_contract=mimalloc-comparison-repeated-measurement-v0' "$out"
rg -F -q 'hako_selected_loadset=empty' "$out"
rg -F -q 'winner_claim=0' "$out"
rg -F -q 'summary=ok' "$out"
rg -F -q 'workload_0_hako_external_elapsed_median_ms=' "$out"
rg -F -q 'workload_0_c_external_elapsed_median_ms=' "$out"

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
            int(values.get(f"workload_{idx}_{side}_external_elapsed_min_ms", "0")),
            int(values.get(f"workload_{idx}_{side}_external_elapsed_median_ms", "0")),
            int(values.get(f"workload_{idx}_{side}_external_elapsed_max_ms", "0")),
        ]
        if any(value <= 0 for value in vals):
            raise SystemExit(f"workload {idx} {side} elapsed values must be positive")
        if vals[0] > vals[1] or vals[1] > vals[2]:
            raise SystemExit(f"workload {idx} {side} elapsed min/median/max order invalid")
    if values.get(f"workload_{idx}_winner_claim") != "0":
        raise SystemExit(f"workload {idx} winner claim must remain closed")

print("[phase295x-abandoned-heap-speed-stability-observation-pack] ok")
PY

echo "[$TAG] ok"
