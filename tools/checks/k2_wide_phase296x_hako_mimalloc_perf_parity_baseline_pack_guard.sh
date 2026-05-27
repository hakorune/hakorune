#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-phase296x-hako-mimalloc-perf-parity-baseline-pack"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

CARD_43="docs/development/current/main/phases/phase-296x/296x-43-HAKO-MIMALLOC-PERF-PARITY-WORKLOAD-MATRIX.md"
CARD_44="docs/development/current/main/phases/phase-296x/296x-44-HAKO-MIMALLOC-PERF-PARITY-BASELINE-PACK.md"
TASKBOARD="docs/development/current/main/phases/phase-296x/296x-90-mimalloc-benchmark-taskboard.md"
CURRENT_STATE="docs/development/current/main/CURRENT_STATE.toml"
PARITY_SSOT="docs/development/current/main/design/hako-mimalloc-performance-parity-ssot.md"
INDEX="docs/tools/check-scripts-index.md"
RUNNER="tools/allocator/mimalloc_repeated_measurement_runner.py"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_hako_mimalloc_perf_parity_baseline_pack_guard.sh"

echo "[$TAG] checking phase-296x parity baseline pack"

guard_require_files "$TAG" "$CARD_43" "$CARD_44" "$TASKBOARD" "$CURRENT_STATE" "$PARITY_SSOT" "$INDEX" "$RUNNER" "$SELF_SCRIPT"
guard_require_exec_files "$TAG" "$RUNNER" "$SELF_SCRIPT"

guard_expect_fixed_in_file "$TAG" 'Status: Landed' "$CARD_44" "baseline pack card must be landed"
guard_expect_fixed_in_file "$TAG" 'HAKO-MIMALLOC-PERF-PARITY-BASELINE-PACK-296X-001' "$CARD_44" "baseline pack card must identify blocker"
guard_expect_fixed_in_file "$TAG" 'same_workload=1' "$CARD_44" "baseline pack must pin same-workload policy"
guard_expect_fixed_in_file "$TAG" 'sample_count=3' "$CARD_44" "baseline pack must pin sample count"
guard_expect_fixed_in_file "$TAG" 'warmup_count=1' "$CARD_44" "baseline pack must pin warmup count"
guard_expect_fixed_in_file "$TAG" 'operation_repeat=128' "$CARD_44" "baseline pack must pin operation repeat"
guard_expect_fixed_in_file "$TAG" 'winner_claim=0' "$CARD_44" "baseline pack must keep winner claims closed"
guard_expect_fixed_in_file "$TAG" 'representative-small-block-v0' "$CARD_44" "baseline pack must use the selected workload"
guard_expect_fixed_in_file "$TAG" 'hako_mimalloc_exact_exe' "$CARD_44" "baseline pack must keep hako subject active"
guard_expect_fixed_in_file "$TAG" 'c_mimalloc_explicit_runner' "$CARD_44" "baseline pack must keep c subject active"
guard_expect_fixed_in_file "$TAG" 'HAKO-MIMALLOC-PERF-GAP-TAXONOMY-ADAPTER-296X-001' "$CARD_44" "baseline pack must select gap taxonomy next"

guard_expect_fixed_in_file "$TAG" 'latest_card = "296x-44-HAKO-MIMALLOC-PERF-PARITY-BASELINE-PACK"' "$CURRENT_STATE" "current state latest card must advance to baseline pack"
guard_expect_fixed_in_file "$TAG" 'current_blocker_token = "HAKO-MIMALLOC-PERF-GAP-TAXONOMY-ADAPTER-296X-001"' "$CURRENT_STATE" "current state must select gap taxonomy adapter"
guard_expect_fixed_in_file "$TAG" '| 44 | `HAKO-MIMALLOC-PERF-PARITY-BASELINE-PACK-296X-001` | Landed |' "$TASKBOARD" "taskboard row 44 must be landed"
guard_expect_fixed_in_file "$TAG" '| 45 | `HAKO-MIMALLOC-PERF-GAP-TAXONOMY-ADAPTER-296X-001` | Current |' "$TASKBOARD" "taskboard row 45 must be current"
guard_expect_fixed_in_file "$TAG" 'Hako Mimalloc Performance Parity' "$PARITY_SSOT" "parity SSOT must exist"
guard_expect_fixed_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list this guard"

tmp_dir="$(mktemp -d /tmp/hakorune_phase296x_parity_baseline.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT
out="$tmp_dir/baseline.out"

python3 "$RUNNER" \
  --out "$out" \
  --workload representative-small-block-v0 \
  --sample-count 3 \
  --warmup-count 1 \
  --hako-runtime-config empty \
  --operation-repeat 128 \
  --c-library /lib/x86_64-linux-gnu/libmimalloc.so.2

guard_expect_fixed_in_file "$TAG" 'output_contract=mimalloc-comparison-repeated-measurement-v0' "$out" "baseline pack must use repeated-measurement contract"
guard_expect_fixed_in_file "$TAG" 'measurement_profile=phase295x-repeated-v0' "$out" "baseline pack must keep measurement profile"
guard_expect_fixed_in_file "$TAG" 'workload_count=1' "$out" "baseline pack must keep a single workload"
guard_expect_fixed_in_file "$TAG" 'workloads=representative-small-block-v0' "$out" "baseline pack must keep selected workload"
guard_expect_fixed_in_file "$TAG" 'sample_count=3' "$out" "baseline pack must keep sample count"
guard_expect_fixed_in_file "$TAG" 'warmup_count=1' "$out" "baseline pack must keep warmup count"
guard_expect_fixed_in_file "$TAG" 'operation_repeat=128' "$out" "baseline pack must keep operation repeat"
guard_expect_fixed_in_file "$TAG" 'provider_activation=0' "$out" "baseline pack must keep provider closed"
guard_expect_fixed_in_file "$TAG" 'host_replacement=0' "$out" "baseline pack must keep replacement closed"
guard_expect_fixed_in_file "$TAG" 'hook_installed=0' "$out" "baseline pack must keep hooks closed"
guard_expect_fixed_in_file "$TAG" 'global_allocator_installed=0' "$out" "baseline pack must keep global allocator closed"
guard_expect_fixed_in_file "$TAG" 'winner_claim=0' "$out" "baseline pack must keep winner claims closed"
guard_expect_fixed_in_file "$TAG" 'summary=ok' "$out" "baseline pack must end ok"

python3 - "$out" <<'PY'
import sys

values = {}
with open(sys.argv[1], encoding="utf-8", errors="replace") as fh:
    for line in fh:
        line = line.strip()
        if "=" in line:
            key, value = line.split("=", 1)
            values[key] = value

required = [
    "workload_0_id",
    "workload_0_operation_family",
    "workload_0_hako_external_elapsed_min_ms",
    "workload_0_hako_external_elapsed_median_ms",
    "workload_0_hako_external_elapsed_max_ms",
    "workload_0_c_external_elapsed_min_ms",
    "workload_0_c_external_elapsed_median_ms",
    "workload_0_c_external_elapsed_max_ms",
]
for key in required:
    if key not in values:
        raise SystemExit(f"missing key: {key}")

if values["workload_0_id"] != "representative-small-block-v0":
    raise SystemExit("wrong workload id")
if values["workload_0_operation_family"] != "small-block":
    raise SystemExit("wrong operation family")

for side in ("hako", "c"):
    vals = [
        int(values[f"workload_0_{side}_external_elapsed_min_ms"]),
        int(values[f"workload_0_{side}_external_elapsed_median_ms"]),
        int(values[f"workload_0_{side}_external_elapsed_max_ms"]),
    ]
    if any(value <= 0 for value in vals):
        raise SystemExit(f"{side} elapsed values must be positive")
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

print("[phase296x-parity-baseline-pack] ok")
PY

echo "[$TAG] ok"
