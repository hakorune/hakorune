#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-phase296x-hako-mimalloc-perf-gap-taxonomy-adapter"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

CARD_45="docs/development/current/main/phases/phase-296x/296x-45-HAKO-MIMALLOC-PERF-GAP-TAXONOMY-ADAPTER.md"
CARD_46="docs/development/current/main/phases/phase-296x/296x-46-HAKO-MIMALLOC-PERF-CONDITIONAL-DIAGNOSTIC-SELECTION.md"
TASKBOARD="docs/development/current/main/phases/phase-296x/296x-90-mimalloc-benchmark-taskboard.md"
CURRENT_STATE="docs/development/current/main/CURRENT_STATE.toml"
PARITY_SSOT="docs/development/current/main/design/hako-mimalloc-performance-parity-ssot.md"
INDEX="docs/tools/check-scripts-index.md"
BASELINE_RUNNER="tools/allocator/mimalloc_repeated_measurement_runner.py"
ADAPTER="tools/allocator/hako_mimalloc_gap_taxonomy_adapter.py"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_hako_mimalloc_perf_gap_taxonomy_adapter_guard.sh"

echo "[$TAG] checking phase-296x gap taxonomy adapter"

guard_require_files "$TAG" "$CARD_45" "$CARD_46" "$TASKBOARD" "$CURRENT_STATE" "$PARITY_SSOT" "$INDEX" "$BASELINE_RUNNER" "$ADAPTER" "$SELF_SCRIPT"
guard_require_exec_files "$TAG" "$BASELINE_RUNNER" "$ADAPTER" "$SELF_SCRIPT"

guard_expect_fixed_in_file "$TAG" 'Status: Landed' "$CARD_45" "gap taxonomy card must be landed"
guard_expect_fixed_in_file "$TAG" 'Status: Current' "$CARD_46" "conditional diagnostic card must be current"
guard_expect_fixed_in_file "$TAG" 'HAKO-MIMALLOC-PERF-GAP-TAXONOMY-ADAPTER-296X-001' "$CARD_45" "gap taxonomy card must identify blocker"
guard_expect_fixed_in_file "$TAG" 'output_contract=hako-mimalloc-gap-taxonomy-v0' "$CARD_45" "gap taxonomy card must define output contract"
guard_expect_fixed_in_file "$TAG" 'outlier_observed=0|1' "$CARD_45" "gap taxonomy must expose outlier flag"
guard_expect_fixed_in_file "$TAG" 'evidence_quality=stable|noisy' "$CARD_45" "gap taxonomy must expose evidence quality"
guard_expect_fixed_in_file "$TAG" 'gap_confidence=low|medium|high' "$CARD_45" "gap taxonomy must expose confidence"
guard_expect_fixed_in_file "$TAG" 'next_diagnostic' "$CARD_45" "gap taxonomy must select next diagnostic"
guard_expect_fixed_in_file "$TAG" 'hako_runtime_baseline' "$CARD_45" "gap taxonomy must include hako runtime baseline owner"
guard_expect_fixed_in_file "$TAG" 'HAKO-MIMALLOC-PERF-CONDITIONAL-DIAGNOSTIC-SELECTION-296X-001' "$CARD_45" "gap taxonomy must select conditional diagnostic next"

guard_expect_fixed_in_file "$TAG" 'latest_card = "296x-45-HAKO-MIMALLOC-PERF-GAP-TAXONOMY-ADAPTER"' "$CURRENT_STATE" "current state latest card must advance to row 45"
guard_expect_fixed_in_file "$TAG" 'current_blocker_token = "HAKO-MIMALLOC-PERF-CONDITIONAL-DIAGNOSTIC-SELECTION-296X-001"' "$CURRENT_STATE" "current state must select row 46"
guard_expect_fixed_in_file "$TAG" '| 45 | `HAKO-MIMALLOC-PERF-GAP-TAXONOMY-ADAPTER-296X-001` | Landed |' "$TASKBOARD" "taskboard row 45 must be landed"
guard_expect_fixed_in_file "$TAG" '| 46 | `HAKO-MIMALLOC-PERF-CONDITIONAL-DIAGNOSTIC-SELECTION-296X-001` | Current |' "$TASKBOARD" "taskboard row 46 must be current"
guard_expect_fixed_in_file "$TAG" 'output_contract=hako-mimalloc-gap-taxonomy-v0' "$PARITY_SSOT" "parity SSOT must define taxonomy contract"
guard_expect_fixed_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list this guard"
guard_expect_fixed_in_file "$TAG" "$ADAPTER" "$INDEX" "check index must list adapter"

tmp_dir="$(mktemp -d /tmp/hakorune_phase296x_gap_taxonomy.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT
baseline_out="$tmp_dir/baseline.out"
taxonomy_out="$tmp_dir/taxonomy.out"
synthetic_in="$tmp_dir/synthetic.out"
synthetic_out="$tmp_dir/synthetic.taxonomy.out"

python3 "$BASELINE_RUNNER" \
  --out "$baseline_out" \
  --workload representative-small-block-v0 \
  --sample-count 3 \
  --warmup-count 1 \
  --hako-runtime-config empty \
  --operation-repeat 128 \
  --c-library /lib/x86_64-linux-gnu/libmimalloc.so.2 >/dev/null

python3 "$ADAPTER" --input "$baseline_out" --out "$taxonomy_out"

guard_expect_fixed_in_file "$TAG" 'output_contract=hako-mimalloc-gap-taxonomy-v0' "$taxonomy_out" "taxonomy adapter must emit contract"
guard_expect_fixed_in_file "$TAG" 'workload_id=representative-small-block-v0' "$taxonomy_out" "taxonomy must preserve workload"
guard_expect_fixed_in_file "$TAG" 'hako_subject=hako_mimalloc_exact_exe' "$taxonomy_out" "taxonomy must preserve hako subject"
guard_expect_fixed_in_file "$TAG" 'c_subject=c_mimalloc_explicit_runner' "$taxonomy_out" "taxonomy must preserve c subject"
guard_expect_fixed_in_file "$TAG" 'sample_count=3' "$taxonomy_out" "taxonomy must preserve sample count"
guard_expect_fixed_in_file "$TAG" 'warmup_count=1' "$taxonomy_out" "taxonomy must preserve warmup count"
guard_expect_fixed_in_file "$TAG" 'operation_repeat=128' "$taxonomy_out" "taxonomy must preserve operation repeat"
guard_expect_fixed_in_file "$TAG" 'winner_claim=0' "$taxonomy_out" "taxonomy must keep winner claims closed"
guard_expect_fixed_in_file "$TAG" 'provider_active=0' "$taxonomy_out" "taxonomy must keep provider closed"
guard_expect_fixed_in_file "$TAG" 'replacement_active=0' "$taxonomy_out" "taxonomy must keep replacement closed"
guard_expect_fixed_in_file "$TAG" 'hook_installed=0' "$taxonomy_out" "taxonomy must keep hooks closed"
guard_expect_fixed_in_file "$TAG" 'global_allocator=0' "$taxonomy_out" "taxonomy must keep global allocator closed"
guard_expect_fixed_in_file "$TAG" 'summary=ok' "$taxonomy_out" "taxonomy must end ok"

python3 - "$taxonomy_out" <<'PY'
import sys

allowed_owners = {
    "allocator_algorithm",
    "compiler_lowering",
    "hako_runtime_baseline",
    "c_abi_memory_bridge",
    "osvm_page_source",
    "provider_wrapper",
    "benchmark_harness",
}
allowed_quality = {"stable", "noisy"}
allowed_confidence = {"low", "medium", "high"}
values = {}
with open(sys.argv[1], encoding="utf-8", errors="replace") as fh:
    for line in fh:
        line = line.strip()
        if "=" in line:
            key, value = line.split("=", 1)
            values[key] = value
for key in (
    "elapsed_median_gap_ms",
    "elapsed_median_ratio",
    "rss_median_gap_bytes",
    "hako_max_to_median_ratio",
    "c_max_to_median_ratio",
    "outlier_observed",
    "evidence_quality",
    "gap_owner",
    "gap_confidence",
    "next_diagnostic",
    "next_optimization_allowed",
):
    if not values.get(key):
        raise SystemExit(f"missing {key}")
if values["gap_owner"] not in allowed_owners:
    raise SystemExit("invalid gap_owner")
if values["evidence_quality"] not in allowed_quality:
    raise SystemExit("invalid evidence_quality")
if values["gap_confidence"] not in allowed_confidence:
    raise SystemExit("invalid gap_confidence")
if values["outlier_observed"] not in {"0", "1"}:
    raise SystemExit("invalid outlier_observed")
if values["next_optimization_allowed"] not in {"0", "1"}:
    raise SystemExit("invalid next_optimization_allowed")
print("[phase296x-gap-taxonomy-live] ok")
PY

cat >"$synthetic_in" <<'EOF'
output_contract=mimalloc-comparison-repeated-measurement-v0
measurement_profile=phase295x-repeated-v0
warmup_count=1
sample_count=3
operation_repeat=128
workload_count=1
workloads=representative-small-block-v0
workload_0_id=representative-small-block-v0
workload_0_operation_repeat=128
workload_0_sample_count=3
workload_0_hako_external_elapsed_min_ms=90
workload_0_hako_external_elapsed_median_ms=90
workload_0_hako_external_elapsed_max_ms=90
workload_0_c_external_elapsed_min_ms=70
workload_0_c_external_elapsed_median_ms=80
workload_0_c_external_elapsed_max_ms=740
workload_0_hako_external_rss_median_bytes=3665920
workload_0_c_external_rss_median_bytes=3985408
provider_activation=0
host_replacement=0
hook_installed=0
global_allocator_installed=0
winner_claim=0
summary=ok
EOF

python3 "$ADAPTER" --input "$synthetic_in" --out "$synthetic_out"
guard_expect_fixed_in_file "$TAG" 'outlier_observed=1' "$synthetic_out" "outlier fixture must classify outlier"
guard_expect_fixed_in_file "$TAG" 'evidence_quality=noisy' "$synthetic_out" "outlier fixture must mark noisy evidence"
guard_expect_fixed_in_file "$TAG" 'gap_owner=benchmark_harness' "$synthetic_out" "outlier fixture must select benchmark harness"
guard_expect_fixed_in_file "$TAG" 'gap_confidence=medium' "$synthetic_out" "outlier fixture must carry confidence"
guard_expect_fixed_in_file "$TAG" 'next_diagnostic=measurement_hygiene_refresh' "$synthetic_out" "outlier fixture must select hygiene refresh"
guard_expect_fixed_in_file "$TAG" 'next_optimization_allowed=0' "$synthetic_out" "outlier fixture must block optimization"
guard_expect_fixed_in_file "$TAG" 'winner_claim=0' "$synthetic_out" "outlier fixture must keep winner claims closed"

echo "[$TAG] ok"
