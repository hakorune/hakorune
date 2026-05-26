#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-phase296x-mimalloc-benchmark-exact-exe-harness-pilot"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

CARD_04="docs/development/current/main/phases/phase-296x/296x-04-MIMALLOC-BENCHMARK-HAKOZUNA-COMPARE-LOG-ADAPTER.md"
CARD_05="docs/development/current/main/phases/phase-296x/296x-05-MIMALLOC-BENCHMARK-EXACT-EXE-HARNESS-PILOT.md"
TASKBOARD="docs/development/current/main/phases/phase-296x/296x-90-mimalloc-benchmark-taskboard.md"
INDEX="docs/tools/check-scripts-index.md"
CURRENT_STATE="docs/development/current/main/CURRENT_STATE.toml"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_mimalloc_benchmark_exact_exe_harness_pilot_guard.sh"
RUNNER="tools/allocator/mimalloc_repeated_measurement_runner.py"
HAKO_RUNNER="tools/allocator/hako_exe_memory_runner.sh"
C_RUNNER="tools/allocator/c_mimalloc_explicit_runner.sh"

echo "[$TAG] checking phase-296x exact-EXE harness pilot"

guard_require_files "$TAG" "$CARD_04" "$CARD_05" "$TASKBOARD" "$INDEX" "$CURRENT_STATE" "$SELF_SCRIPT" "$RUNNER" "$HAKO_RUNNER" "$C_RUNNER"
guard_require_exec_files "$TAG" "$SELF_SCRIPT" "$RUNNER" "$HAKO_RUNNER" "$C_RUNNER"

guard_expect_fixed_in_file "$TAG" 'latest_card = "296x-05-MIMALLOC-BENCHMARK-EXACT-EXE-HARNESS-PILOT"' "$CURRENT_STATE" "current state latest card must advance to exact-exe harness pilot"
guard_expect_fixed_in_file "$TAG" 'current_blocker_token = "MIMALLOC-BENCHMARK-EXTERNAL-CORPUS-CLOSEOUT-296X-001"' "$CURRENT_STATE" "current state must expose external corpus closeout blocker"
guard_expect_fixed_in_file "$TAG" 'Status: Landed' "$CARD_04" "hakozuna compare card must be landed"
guard_expect_fixed_in_file "$TAG" 'Status: Landed' "$CARD_05" "exact-exe harness card must be landed"
guard_expect_fixed_in_file "$TAG" 'MIMALLOC-BENCHMARK-EXACT-EXE-HARNESS-PILOT-296X-001' "$CARD_05" "exact-exe harness card must identify blocker"
guard_expect_fixed_in_file "$TAG" 'output_contract=mimalloc-comparison-repeated-measurement-v0' "$CARD_05" "exact-exe harness card must name the repeated measurement contract"
guard_expect_fixed_in_file "$TAG" 'winner_claim=0' "$CARD_05" "exact-exe harness card must keep winner claims closed"
guard_expect_fixed_in_file "$TAG" 'MIMALLOC-BENCHMARK-EXACT-EXE-HARNESS-PILOT-296X-001' "$TASKBOARD" "taskboard must expose the exact-exe harness row"
guard_expect_fixed_in_file "$TAG" 'MIMALLOC-BENCHMARK-EXTERNAL-CORPUS-CLOSEOUT-296X-001' "$CARD_05" "exact-exe harness card must select corpus closeout next"
guard_expect_fixed_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list this guard"

library_path="$(guard_find_mimalloc_library "$TAG")"
tmp_dir="$(mktemp -d /tmp/hakorune_phase296x_exact_exe_harness.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT
out="$tmp_dir/repeated.out"

python3 "$RUNNER" \
  --out "$out" \
  --workload representative-small-block-v0 \
  --sample-count 1 \
  --warmup-count 0 \
  --operation-repeat 1 \
  --c-library "$library_path"

guard_expect_fixed_in_file "$TAG" 'mimalloc_repeated_measurement_runner=1' "$out" "runner must identify itself"
guard_expect_fixed_in_file "$TAG" 'output_contract=mimalloc-comparison-repeated-measurement-v0' "$out" "runner must keep repeated measurement contract"
guard_expect_fixed_in_file "$TAG" 'measurement_profile=phase295x-repeated-v0' "$out" "runner must keep the repeated measurement profile"
guard_expect_fixed_in_file "$TAG" 'warmup_count=0' "$out" "runner must keep warmup count"
guard_expect_fixed_in_file "$TAG" 'sample_count=1' "$out" "runner must keep sample count"
guard_expect_fixed_in_file "$TAG" 'operation_repeat=1' "$out" "runner must keep operation repeat"
guard_expect_fixed_in_file "$TAG" 'workload_count=1' "$out" "runner must keep the single-workload pilot"
guard_expect_fixed_in_file "$TAG" 'workloads=representative-small-block-v0' "$out" "runner must compare the selected workload"
guard_expect_fixed_in_file "$TAG" 'workload_0_id=representative-small-block-v0' "$out" "runner must expose workload identity"
guard_expect_fixed_in_file "$TAG" 'workload_0_operation_family=small-block' "$out" "runner must expose operation family"
guard_expect_fixed_in_file "$TAG" 'sample_0_0_workload=representative-small-block-v0' "$out" "runner must expose sample workload"
guard_expect_fixed_in_file "$TAG" 'sample_0_0_winner_claim=0' "$out" "runner must keep winner claims closed"
guard_expect_fixed_in_file "$TAG" 'provider_activation=0' "$out" "runner must keep provider seams closed"
guard_expect_fixed_in_file "$TAG" 'host_replacement=0' "$out" "runner must keep replacement seams closed"
guard_expect_fixed_in_file "$TAG" 'hook_installed=0' "$out" "runner must keep hook seams closed"
guard_expect_fixed_in_file "$TAG" 'global_allocator_installed=0' "$out" "runner must keep global allocator seams closed"
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

for key in (
    "sample_0_0_hako_external_peak_rss_bytes",
    "sample_0_0_c_external_peak_rss_bytes",
    "sample_0_0_hako_external_elapsed_ms",
    "sample_0_0_c_external_elapsed_ms",
    "workload_0_hako_external_rss_min_bytes",
    "workload_0_c_external_rss_min_bytes",
):
    if int(values.get(key, "0")) <= 0:
        raise SystemExit(f"{key} must be positive")

if values.get("workload_0_operation_family") != "small-block":
    raise SystemExit("operation family mismatch")
if values.get("workload_0_operation_repeat") != "1":
    raise SystemExit("operation repeat mismatch")
print("[phase296x-exact-exe-harness] ok")
PY

cat "$out"
echo "[$TAG] ok"
