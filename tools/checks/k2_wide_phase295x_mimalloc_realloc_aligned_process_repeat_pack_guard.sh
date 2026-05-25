#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-phase295x-mimalloc-realloc-aligned-process-repeat-pack"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

CARD="docs/development/current/main/phases/phase-295x/295x-237-MIMALLOC-COMPARISON-REALLOC-ALIGNED-PROCESS-REPEAT-PACK.md"
PREV_CARD="docs/development/current/main/phases/phase-295x/295x-236-MIMALLOC-COMPARISON-REUSE-CYCLE-SMALL-PROCESS-REPEAT-PACK.md"
TASKBOARD="docs/development/current/main/phases/phase-295x/295x-90-mimalloc-comparison-execution-taskboard.md"
CURRENT_STATE="docs/development/current/main/CURRENT_STATE.toml"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase295x_mimalloc_realloc_aligned_process_repeat_pack_guard.sh"
RUNNER="tools/allocator/mimalloc_repeated_measurement_runner.py"
HAKO_RUNNER="tools/allocator/hako_exe_memory_runner.sh"
C_RUNNER="tools/allocator/c_mimalloc_explicit_runner.sh"
APP="apps/hako-alloc-mimalloc-comparison-realloc-aligned-exe-proof/main.hako"

echo "[$TAG] checking phase-295x realloc/aligned process-repeat pack"

guard_require_files "$TAG" "$CARD" "$PREV_CARD" "$TASKBOARD" "$CURRENT_STATE" "$INDEX" "$SELF_SCRIPT" "$RUNNER" "$HAKO_RUNNER" "$C_RUNNER" "$APP"
guard_require_exec_files "$TAG" "$SELF_SCRIPT" "$RUNNER" "$HAKO_RUNNER" "$C_RUNNER"

guard_expect_fixed_in_file "$TAG" 'Status: Current' "$CARD" "card must remain current while the realloc/aligned pack is exercised"
guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-REALLOC-ALIGNED-PROCESS-REPEAT-PACK-295X-002' "$CARD" "card must identify the realloc/aligned process-repeat blocker"
guard_expect_in_file "$TAG" 'workload=representative-realloc-aligned-v0' "$CARD" "card must define workload id"
guard_expect_in_file "$TAG" 'operation_family=realloc-aligned' "$CARD" "card must define operation family"
guard_expect_in_file "$TAG" 'operation_sequence_id=representative-realloc-aligned-v0-seq' "$CARD" "card must define sequence id"
guard_expect_in_file "$TAG" 'free_order_id=ascending-release-v0' "$CARD" "card must define free order"
guard_expect_in_file "$TAG" 'operation_repeat=128' "$CARD" "card must document repeat count"
guard_expect_in_file "$TAG" 'timing_repeat_kind=process-invocation-v0' "$CARD" "card must document timing kind"
guard_expect_in_file "$TAG" 'sample_count=3' "$CARD" "card must document sample count"
guard_expect_in_file "$TAG" 'warmup_count=1' "$CARD" "card must document warmup count"
guard_expect_in_file "$TAG" 'winner_claim=0' "$CARD" "card must keep winner claims closed"
guard_expect_in_file "$TAG" 'Status: Landed' "$PREV_CARD" "previous row must be landed before the pack row"
guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-REALLOC-ALIGNED-PROCESS-REPEAT-PACK-295X-002' "$PREV_CARD" "previous row must select this pack"
guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-REALLOC-ALIGNED-PROCESS-REPEAT-PACK-295X-002' "$TASKBOARD" "taskboard must expose the realloc/aligned pack blocker"
guard_expect_in_file "$TAG" '| 236 | `MIMALLOC-COMPARISON-REUSE-CYCLE-SMALL-PROCESS-REPEAT-PACK-295X-002` | Landed |' "$TASKBOARD" "taskboard must expose the reuse-cycle pack as landed"
guard_expect_in_file "$TAG" '| 237 | `MIMALLOC-COMPARISON-REALLOC-ALIGNED-PROCESS-REPEAT-PACK-295X-002` | Current |' "$TASKBOARD" "taskboard must expose the realloc/aligned process-repeat pack as current"
guard_expect_in_file "$TAG" '237' "$CURRENT_STATE" "current state must point at the realloc/aligned pack card"
guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-REALLOC-ALIGNED-PROCESS-REPEAT-PACK-295X-002' "$CURRENT_STATE" "current state must expose the realloc/aligned pack blocker"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check script index must list this guard"

tmp_dir="$(mktemp -d /tmp/hakorune_phase295x_realloc_aligned_repeat.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT
out="$tmp_dir/realloc-aligned-repeat.out"

library_path="$(guard_find_mimalloc_library "$TAG")"

python3 "$RUNNER" \
  --out "$out" \
  --workload representative-realloc-aligned-v0 \
  --sample-count 3 \
  --warmup-count 1 \
  --hako-runtime-config empty \
  --operation-repeat 128 \
  --c-library "$library_path"

guard_expect_in_file "$TAG" 'workload=representative-realloc-aligned-v0' "$APP" ".hako app must define workload id"
guard_expect_in_file "$TAG" 'operation_family=realloc-aligned' "$APP" ".hako app must define operation family"
guard_expect_in_file "$TAG" 'operation_sequence_id=representative-realloc-aligned-v0-seq' "$APP" ".hako app must define sequence id"
guard_expect_in_file "$TAG" 'free_order_id=ascending-release-v0' "$APP" ".hako app must define free order"
rg -F -q 'workload_0_id=representative-realloc-aligned-v0' "$out"
rg -F -q 'workload_0_operation_family=realloc-aligned' "$out"
rg -F -q 'workload_0_operation_repeat=128' "$out"
rg -F -q 'workload_0_timing_repeat_kind=process-invocation-v0' "$out"
rg -F -q 'workload_0_sample_count=3' "$out"
rg -F -q 'workload_0_winner_claim=0' "$out"
rg -F -q "c_library_path=$library_path" "$out"
rg -F -q 'sample_count=3' "$out"
rg -F -q 'warmup_count=1' "$out"
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

for idx in range(1):
    if values.get("workload_0_operation_repeat") != "128":
        raise SystemExit("workload 0 operation_repeat mismatch")
    if values.get("workload_0_timing_repeat_kind") != "process-invocation-v0":
        raise SystemExit("workload 0 timing_repeat_kind mismatch")
    for side in ("hako", "c"):
        vals = [
            int(values.get(f"workload_0_{side}_external_elapsed_min_ms", "0")),
            int(values.get(f"workload_0_{side}_external_elapsed_median_ms", "0")),
            int(values.get(f"workload_0_{side}_external_elapsed_max_ms", "0")),
        ]
        if any(value <= 1 for value in vals):
            raise SystemExit(f"workload 0 {side} elapsed values must escape the 1ms floor")
        if vals[0] > vals[1] or vals[1] > vals[2]:
            raise SystemExit(f"workload 0 {side} elapsed min/median/max order invalid")
    if values.get("workload_0_winner_claim") != "0":
        raise SystemExit("workload 0 winner claim must remain closed")

print("[phase295x-realloc-aligned-process-repeat-pack] ok")
PY

cat "$out"
echo "[$TAG] ok"
