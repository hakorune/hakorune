#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-phase295x-long-process-repeat-timing-pack"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

CARD="docs/development/current/main/phases/phase-295x/295x-70-MIMALLOC-COMPARISON-LONG-PROCESS-REPEAT-TIMING-PACK.md"
PREV_CARD="docs/development/current/main/phases/phase-295x/295x-69-MIMALLOC-COMPARISON-HIGH-RES-TIMING-SEAM-SELECTION.md"
TASKBOARD="docs/development/current/main/phases/phase-295x/295x-90-mimalloc-comparison-execution-taskboard.md"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase295x_long_process_repeat_timing_pack_guard.sh"
RUNNER="tools/allocator/mimalloc_repeated_measurement_runner.py"
HAKO_RUNNER="tools/allocator/hako_exe_memory_runner.sh"
C_RUNNER="tools/allocator/c_mimalloc_explicit_runner.sh"

echo "[$TAG] checking phase-295x long process-repeat timing pack"

guard_require_files "$TAG" "$CARD" "$PREV_CARD" "$TASKBOARD" "$INDEX" "$SELF_SCRIPT" "$RUNNER" "$HAKO_RUNNER" "$C_RUNNER"
guard_require_exec_files "$TAG" "$SELF_SCRIPT" "$RUNNER" "$HAKO_RUNNER" "$C_RUNNER"

guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-LONG-PROCESS-REPEAT-TIMING-PACK-295X-001' "$CARD" "card must identify pack blocker"
guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-LONG-PROCESS-REPEAT-TIMING-CLOSEOUT-295X-001' "$CARD" "card must select closeout follow-on"
guard_expect_in_file "$TAG" 'operation_repeat=128' "$CARD" "card must document repeat count"
guard_expect_in_file "$TAG" 'timing_repeat_kind=process-invocation-v0' "$CARD" "card must document repeat kind"
guard_expect_in_file "$TAG" '--operation-repeat' "$RUNNER" "repeated runner must expose operation repeat"
guard_expect_in_file "$TAG" '--operation-repeat' "$HAKO_RUNNER" "hako runner must expose operation repeat"
guard_expect_in_file "$TAG" '--operation-repeat' "$C_RUNNER" "C runner must expose operation repeat"
guard_expect_in_file "$TAG" '295x-71' "$TASKBOARD" "taskboard must include the selected closeout row"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check script index must list this guard"

tmp_dir="$(mktemp -d /tmp/hakorune_phase295x_long_timing.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT
out="$tmp_dir/long-timing.out"

python3 "$RUNNER" \
  --out "$out" \
  --sample-count 3 \
  --warmup-count 1 \
  --hako-runtime-config empty \
  --operation-repeat 128 \
  --allow-ldconfig-discovery

rg -F -q 'operation_repeat=128' "$out"
rg -F -q 'timing_repeat_kind=process-invocation-v0' "$out"
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
    if values.get(f"workload_{idx}_operation_repeat") != "128":
        raise SystemExit(f"workload {idx} operation_repeat mismatch")
    if values.get(f"workload_{idx}_timing_repeat_kind") != "process-invocation-v0":
        raise SystemExit(f"workload {idx} timing_repeat_kind mismatch")
    for side in ("hako", "c"):
        vals = [
            int(values.get(f"workload_{idx}_{side}_external_elapsed_min_ms", "0")),
            int(values.get(f"workload_{idx}_{side}_external_elapsed_median_ms", "0")),
            int(values.get(f"workload_{idx}_{side}_external_elapsed_max_ms", "0")),
        ]
        if any(value <= 1 for value in vals):
            raise SystemExit(f"workload {idx} {side} elapsed values must escape the 1ms floor")
        if vals[0] > vals[1] or vals[1] > vals[2]:
            raise SystemExit(f"workload {idx} {side} elapsed min/median/max order invalid")
    if values.get(f"workload_{idx}_winner_claim") != "0":
        raise SystemExit(f"workload {idx} winner claim must remain closed")

print("[phase295x-long-process-repeat-timing-pack] ok")
PY

cat "$out"
echo "[$TAG] ok"
