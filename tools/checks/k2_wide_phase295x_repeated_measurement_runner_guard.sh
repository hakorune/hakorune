#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-phase295x-repeated-measurement-runner"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

CARD="docs/development/current/main/phases/phase-295x/295x-29-MIMALLOC-COMPARISON-REPEATED-MEASUREMENT-RUNNER.md"
PREV_CARD="docs/development/current/main/phases/phase-295x/295x-28-MIMALLOC-COMPARISON-REPEATED-MEASUREMENT-POLICY.md"
TASKBOARD="docs/development/current/main/phases/phase-295x/295x-90-mimalloc-comparison-execution-taskboard.md"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase295x_repeated_measurement_runner_guard.sh"
RUNNER="tools/allocator/mimalloc_repeated_measurement_runner.py"
HAKO_RUNNER="tools/allocator/hako_exe_memory_runner.sh"
C_RUNNER="tools/allocator/c_mimalloc_explicit_runner.sh"
SMALL_APP="apps/hako-alloc-mimalloc-comparison-representative-small-block-proof/main.hako"

echo "[$TAG] checking phase-295x repeated measurement runner"

guard_require_files "$TAG" "$CARD" "$PREV_CARD" "$TASKBOARD" "$INDEX" "$SELF_SCRIPT" "$RUNNER" "$HAKO_RUNNER" "$C_RUNNER" "$SMALL_APP"
guard_require_exec_files "$TAG" "$SELF_SCRIPT" "$RUNNER" "$HAKO_RUNNER" "$C_RUNNER"

guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-REPEATED-MEASUREMENT-RUNNER-295X-001' "$CARD" "card must identify the current blocker"
guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-REPEATED-MEASUREMENT-PACK-RUN-295X-001' "$CARD" "card must select pack run follow-on"
guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-REPEATED-MEASUREMENT-RUNNER-295X-001' "$PREV_CARD" "previous row must select this runner"
guard_expect_fixed_in_file "$TAG" '| 30 | `295x-30` | Landed |' "$TASKBOARD" "taskboard must retain the pack run row as landed"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check script index must list this guard"
guard_expect_in_file "$TAG" 'mimalloc-comparison-repeated-measurement-v0' "$RUNNER" "runner must publish repeated measurement contract"
guard_expect_in_file "$TAG" 'canonical_rss_collector=external-time' "$RUNNER" "runner must use external-time RSS collector"
guard_expect_in_file "$TAG" 'winner_claim=0' "$RUNNER" "runner must keep winner claims closed"
guard_expect_in_file "$TAG" 'external_peak_rss_bytes' "$HAKO_RUNNER" "hako runner must expose external RSS"
guard_expect_in_file "$TAG" 'external_peak_rss_bytes' "$C_RUNNER" "C runner must expose external RSS"
guard_expect_in_file "$TAG" 'operation_family=small-block' "$SMALL_APP" "small-block app must expose operation family"
guard_expect_in_file "$TAG" 'free_order_id=even-odd-release-v0' "$SMALL_APP" "small-block app must match C free order"

tmp_dir="$(mktemp -d /tmp/hakorune_phase295x_repeated_runner.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT
out="$tmp_dir/repeated.out"
library_path="$(guard_find_mimalloc_library "$TAG")"

python3 "$RUNNER" \
  --out "$out" \
  --workload representative-small-block-v0 \
  --sample-count 5 \
  --warmup-count 1 \
  --c-library "$library_path"

rg -F -q 'mimalloc_repeated_measurement_runner=1' "$out"
rg -F -q 'output_contract=mimalloc-comparison-repeated-measurement-v0' "$out"
rg -F -q 'measurement_profile=phase295x-repeated-v0' "$out"
rg -F -q 'warmup_count=1' "$out"
rg -F -q 'sample_count=5' "$out"
rg -F -q 'workload_count=1' "$out"
rg -F -q 'workloads=representative-small-block-v0' "$out"
rg -F -q 'summary_statistic=min,median,max' "$out"
rg -F -q 'canonical_rss_collector=external-time' "$out"
rg -F -q "c_library_path=$library_path" "$out"
rg -F -q 'workload_0_operation_family=small-block' "$out"
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

for key in (
    "workload_0_hako_external_rss_min_bytes",
    "workload_0_hako_external_rss_median_bytes",
    "workload_0_hako_external_rss_max_bytes",
    "workload_0_c_external_rss_min_bytes",
    "workload_0_c_external_rss_median_bytes",
    "workload_0_c_external_rss_max_bytes",
):
    value = int(values.get(key, "0"))
    if value <= 0:
        raise SystemExit(f"{key} must be positive")
if int(values["workload_0_hako_external_rss_min_bytes"]) > int(values["workload_0_hako_external_rss_max_bytes"]):
    raise SystemExit("hako external RSS min/max inverted")
if int(values["workload_0_c_external_rss_min_bytes"]) > int(values["workload_0_c_external_rss_max_bytes"]):
    raise SystemExit("C external RSS min/max inverted")
print("[phase295x-repeated-measurement-runner] ok")
PY

cat "$out"
echo "[$TAG] ok"
