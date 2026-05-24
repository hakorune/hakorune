#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-phase295x-speed-stability-observation-pack"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

CARD="docs/development/current/main/phases/phase-295x/295x-67-MIMALLOC-COMPARISON-SPEED-STABILITY-OBSERVATION-PACK.md"
PREV_CARD="docs/development/current/main/phases/phase-295x/295x-66-MIMALLOC-COMPARISON-POST-LOADSET-AWARE-MEASUREMENT-SELECTION.md"
TASKBOARD="docs/development/current/main/phases/phase-295x/295x-90-mimalloc-comparison-execution-taskboard.md"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase295x_speed_stability_observation_pack_guard.sh"
RUNNER="tools/allocator/mimalloc_repeated_measurement_runner.py"
HAKO_RUNNER="tools/allocator/hako_exe_memory_runner.sh"
C_RUNNER="tools/allocator/c_mimalloc_explicit_runner.sh"

echo "[$TAG] checking phase-295x speed/stability observation pack"

guard_require_files "$TAG" "$CARD" "$PREV_CARD" "$TASKBOARD" "$INDEX" "$SELF_SCRIPT" "$RUNNER" "$HAKO_RUNNER" "$C_RUNNER"
guard_require_exec_files "$TAG" "$SELF_SCRIPT" "$RUNNER" "$HAKO_RUNNER" "$C_RUNNER"

guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-SPEED-STABILITY-OBSERVATION-PACK-295X-001' "$CARD" "card must identify current blocker"
guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-SPEED-STABILITY-OBSERVATION-CLOSEOUT-295X-001' "$CARD" "card must select closeout follow-on"
guard_expect_in_file "$TAG" 'external_elapsed_median_ms' "$CARD" "card must define elapsed timing evidence"
guard_expect_in_file "$TAG" 'winner_claim=0' "$CARD" "card must keep winner claims closed"
guard_expect_in_file "$TAG" 'external_elapsed_ms' "$RUNNER" "repeated runner must consume external elapsed evidence"
guard_expect_in_file "$TAG" 'external_elapsed_ms' "$HAKO_RUNNER" "hako runner must emit external elapsed evidence"
guard_expect_in_file "$TAG" 'external_elapsed_ms' "$C_RUNNER" "C runner must emit external elapsed evidence"
guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-SPEED-STABILITY-OBSERVATION-CLOSEOUT-295X-001' "$TASKBOARD" "taskboard must expose selected follow-on"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check script index must list this guard"

tmp_dir="$(mktemp -d /tmp/hakorune_phase295x_speed_stability.XXXXXX)"
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

print("[phase295x-speed-stability-observation-pack] ok")
PY

cat "$out"
echo "[$TAG] ok"
