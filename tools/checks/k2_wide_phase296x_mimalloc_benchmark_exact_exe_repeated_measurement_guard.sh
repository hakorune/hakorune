#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-phase296x-mimalloc-benchmark-exact-exe-repeated-measurement"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

CARD_06="docs/development/current/main/phases/phase-296x/296x-06-MIMALLOC-BENCHMARK-EXTERNAL-CORPUS-CLOSEOUT.md"
CARD_07="docs/development/current/main/phases/phase-296x/296x-07-MIMALLOC-BENCHMARK-EXACT-EXE-REPEATED-MEASUREMENT.md"
TASKBOARD="docs/development/current/main/phases/phase-296x/296x-90-mimalloc-benchmark-taskboard.md"
INDEX="docs/tools/check-scripts-index.md"
CURRENT_STATE="docs/development/current/main/CURRENT_STATE.toml"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_mimalloc_benchmark_exact_exe_repeated_measurement_guard.sh"
RUNNER="tools/allocator/mimalloc_repeated_measurement_runner.py"
HAKO_RUNNER="tools/allocator/hako_exe_memory_runner.sh"
C_RUNNER="tools/allocator/c_mimalloc_explicit_runner.sh"
APP="apps/hako-alloc-mimalloc-comparison-representative-small-block-proof/main.hako"

echo "[$TAG] checking phase-296x exact-EXE repeated measurement"

guard_require_files "$TAG" "$CARD_06" "$CARD_07" "$TASKBOARD" "$INDEX" "$CURRENT_STATE" "$SELF_SCRIPT" "$RUNNER" "$HAKO_RUNNER" "$C_RUNNER" "$APP"
guard_require_exec_files "$TAG" "$SELF_SCRIPT" "$RUNNER" "$HAKO_RUNNER" "$C_RUNNER"

guard_expect_fixed_in_file "$TAG" 'latest_card = "296x-07-MIMALLOC-BENCHMARK-EXACT-EXE-REPEATED-MEASUREMENT"' "$CURRENT_STATE" "current state latest card must advance to repeated measurement"
guard_expect_fixed_in_file "$TAG" 'current_blocker_token = "MIMALLOC-DLL-LOAD-ONLY-SELECTION-296X-001"' "$CURRENT_STATE" "current state must expose load-only DLL selection blocker"
guard_expect_fixed_in_file "$TAG" 'Status: Landed' "$CARD_06" "external corpus closeout must be landed"
guard_expect_fixed_in_file "$TAG" 'Status: Landed' "$CARD_07" "repeated measurement card must be landed"
guard_expect_fixed_in_file "$TAG" 'MIMALLOC-BENCHMARK-EXACT-EXE-REPEATED-MEASUREMENT-296X-001' "$CARD_07" "repeated measurement card must identify blocker"
guard_expect_fixed_in_file "$TAG" 'workload=representative-small-block-v0' "$CARD_07" "card must record workload"
guard_expect_fixed_in_file "$TAG" 'sample_count=3' "$CARD_07" "card must record sample count"
guard_expect_fixed_in_file "$TAG" 'warmup_count=1' "$CARD_07" "card must record warmup count"
guard_expect_fixed_in_file "$TAG" 'operation_repeat=128' "$CARD_07" "card must record operation repeat"
guard_expect_fixed_in_file "$TAG" 'winner_claim=0' "$CARD_07" "card must keep winner claims closed"
guard_expect_fixed_in_file "$TAG" 'MIMALLOC-DLL-LOAD-ONLY-SELECTION-296X-001' "$CARD_07" "card must select load-only DLL row"
guard_expect_fixed_in_file "$TAG" 'MIMALLOC-BENCHMARK-EXACT-EXE-REPEATED-MEASUREMENT-296X-001' "$TASKBOARD" "taskboard must expose repeated measurement row"
guard_expect_fixed_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list this guard"

library_path="$(guard_find_mimalloc_library "$TAG")"
tmp_dir="$(mktemp -d /tmp/hakorune_phase296x_exact_exe_real_measurement.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT
out="$tmp_dir/repeated.out"

python3 "$RUNNER" \
  --out "$out" \
  --workload representative-small-block-v0 \
  --sample-count 3 \
  --warmup-count 1 \
  --hako-runtime-config empty \
  --operation-repeat 128 \
  --c-library "$library_path"

guard_expect_fixed_in_file "$TAG" 'output_contract=mimalloc-comparison-repeated-measurement-v0' "$out" "runner must keep repeated measurement contract"
guard_expect_fixed_in_file "$TAG" 'measurement_profile=phase295x-repeated-v0' "$out" "runner must keep measurement profile"
guard_expect_fixed_in_file "$TAG" 'warmup_count=1' "$out" "runner must keep warmup count"
guard_expect_fixed_in_file "$TAG" 'sample_count=3' "$out" "runner must keep sample count"
guard_expect_fixed_in_file "$TAG" 'operation_repeat=128' "$out" "runner must keep operation repeat"
guard_expect_fixed_in_file "$TAG" 'workload_count=1' "$out" "runner must keep one selected workload"
guard_expect_fixed_in_file "$TAG" 'workloads=representative-small-block-v0' "$out" "runner must keep selected workload"
guard_expect_fixed_in_file "$TAG" 'workload_0_operation_family=small-block' "$out" "runner must keep operation family"
guard_expect_fixed_in_file "$TAG" 'workload_0_operation_repeat=128' "$out" "runner must keep workload operation repeat"
guard_expect_fixed_in_file "$TAG" 'workload_0_sample_count=3' "$out" "runner must keep workload sample count"
guard_expect_fixed_in_file "$TAG" 'workload_0_winner_claim=0' "$out" "runner must keep workload winner claim closed"
guard_expect_fixed_in_file "$TAG" 'provider_activation=0' "$out" "runner must keep provider seams closed"
guard_expect_fixed_in_file "$TAG" 'host_replacement=0' "$out" "runner must keep replacement seams closed"
guard_expect_fixed_in_file "$TAG" 'hook_installed=0' "$out" "runner must keep hooks closed"
guard_expect_fixed_in_file "$TAG" 'global_allocator_installed=0' "$out" "runner must keep global allocator closed"
guard_expect_fixed_in_file "$TAG" 'winner_claim=0' "$out" "runner must keep winner claims closed"
guard_expect_fixed_in_file "$TAG" 'summary=ok' "$out" "runner must end with summary"

python3 - "$out" <<'PY'
import sys

values = {}
with open(sys.argv[1], encoding="utf-8", errors="replace") as fh:
    for line in fh:
        line = line.strip()
        if "=" in line:
            key, value = line.split("=", 1)
            values[key] = value

for side in ("hako", "c"):
    vals = [
        int(values.get(f"workload_0_{side}_external_elapsed_min_ms", "0")),
        int(values.get(f"workload_0_{side}_external_elapsed_median_ms", "0")),
        int(values.get(f"workload_0_{side}_external_elapsed_max_ms", "0")),
    ]
    if any(value <= 1 for value in vals):
        raise SystemExit(f"{side} elapsed values must escape the 1ms floor")
    if vals[0] > vals[1] or vals[1] > vals[2]:
        raise SystemExit(f"{side} elapsed min/median/max order invalid")

for key in (
    "workload_0_hako_external_rss_median_bytes",
    "workload_0_c_external_rss_median_bytes",
    "workload_0_hako_internal_rss_median_bytes",
    "workload_0_c_internal_rss_median_bytes",
):
    if int(values.get(key, "0")) <= 0:
        raise SystemExit(f"{key} must be positive")

print("[phase296x-exact-exe-repeated-measurement] ok")
PY

cat "$out"
echo "[$TAG] ok"
