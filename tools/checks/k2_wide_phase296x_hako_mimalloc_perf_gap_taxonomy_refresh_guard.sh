#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-phase296x-hako-mimalloc-perf-gap-taxonomy-refresh"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

CARD_49="docs/development/current/main/phases/phase-296x/296x-49-HAKO-MIMALLOC-PERF-GAP-TAXONOMY-REFRESH.md"
CARD_50="docs/development/current/main/phases/phase-296x/296x-50-HAKO-MIMALLOC-PERF-REFRESHED-TAXONOMY-DECISION.md"
TASKBOARD="docs/development/current/main/phases/phase-296x/296x-90-mimalloc-benchmark-taskboard.md"
CURRENT_STATE="docs/development/current/main/CURRENT_STATE.toml"
INDEX="docs/tools/check-scripts-index.md"
BASELINE_RUNNER="tools/allocator/mimalloc_repeated_measurement_runner.py"
ADAPTER="tools/allocator/hako_mimalloc_gap_taxonomy_adapter.py"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_hako_mimalloc_perf_gap_taxonomy_refresh_guard.sh"

echo "[$TAG] checking phase-296x gap taxonomy refresh"

guard_require_files "$TAG" "$CARD_49" "$CARD_50" "$TASKBOARD" "$CURRENT_STATE" "$INDEX" "$BASELINE_RUNNER" "$ADAPTER" "$SELF_SCRIPT"
guard_require_exec_files "$TAG" "$BASELINE_RUNNER" "$ADAPTER" "$SELF_SCRIPT"

guard_expect_fixed_in_file "$TAG" 'Status: Landed' "$CARD_49" "taxonomy refresh card must be landed"
guard_expect_fixed_in_file "$TAG" 'Status: Current' "$CARD_50" "refreshed taxonomy decision card must be current"
guard_expect_fixed_in_file "$TAG" 'sample_count=5' "$CARD_49" "refresh must use sample count 5"
guard_expect_fixed_in_file "$TAG" 'HAKO-MIMALLOC-PERF-REFRESHED-TAXONOMY-DECISION-296X-001' "$CARD_49" "refresh must select row 50"

guard_expect_fixed_in_file "$TAG" 'latest_card = "296x-49-HAKO-MIMALLOC-PERF-GAP-TAXONOMY-REFRESH"' "$CURRENT_STATE" "current state latest card must advance to row 49"
guard_expect_fixed_in_file "$TAG" 'current_blocker_token = "HAKO-MIMALLOC-PERF-REFRESHED-TAXONOMY-DECISION-296X-001"' "$CURRENT_STATE" "current state must select row 50"
guard_expect_fixed_in_file "$TAG" '| 49 | `HAKO-MIMALLOC-PERF-GAP-TAXONOMY-REFRESH-296X-001` | Landed |' "$TASKBOARD" "taskboard row 49 must be landed"
guard_expect_fixed_in_file "$TAG" '| 50 | `HAKO-MIMALLOC-PERF-REFRESHED-TAXONOMY-DECISION-296X-001` | Current |' "$TASKBOARD" "taskboard row 50 must be current"
guard_expect_fixed_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list this guard"

tmp_dir="$(mktemp -d /tmp/hakorune_phase296x_taxonomy_refresh.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT
measurement="$tmp_dir/measurement.out"
taxonomy="$tmp_dir/taxonomy.out"

python3 "$BASELINE_RUNNER" \
  --out "$measurement" \
  --workload representative-small-block-v0 \
  --sample-count 5 \
  --warmup-count 1 \
  --hako-runtime-config empty \
  --operation-repeat 128 \
  --c-library /lib/x86_64-linux-gnu/libmimalloc.so.2 >/dev/null

python3 "$ADAPTER" --input "$measurement" --out "$taxonomy"

guard_expect_fixed_in_file "$TAG" 'output_contract=hako-mimalloc-gap-taxonomy-v0' "$taxonomy" "taxonomy refresh must emit taxonomy contract"
guard_expect_fixed_in_file "$TAG" 'workload_id=representative-small-block-v0' "$taxonomy" "taxonomy refresh must preserve workload"
guard_expect_fixed_in_file "$TAG" 'sample_count=5' "$taxonomy" "taxonomy refresh must use sample count 5"
guard_expect_fixed_in_file "$TAG" 'winner_claim=0' "$taxonomy" "taxonomy refresh must keep winner claims closed"
guard_expect_fixed_in_file "$TAG" 'provider_active=0' "$taxonomy" "taxonomy refresh must keep provider closed"
guard_expect_fixed_in_file "$TAG" 'replacement_active=0' "$taxonomy" "taxonomy refresh must keep replacement closed"
guard_expect_fixed_in_file "$TAG" 'hook_installed=0' "$taxonomy" "taxonomy refresh must keep hooks closed"
guard_expect_fixed_in_file "$TAG" 'global_allocator=0' "$taxonomy" "taxonomy refresh must keep global allocator closed"
guard_expect_fixed_in_file "$TAG" 'summary=ok' "$taxonomy" "taxonomy refresh must end ok"

python3 - "$taxonomy" <<'PY'
import sys

values = {}
with open(sys.argv[1], encoding="utf-8", errors="replace") as fh:
    for line in fh:
        line = line.strip()
        if "=" in line:
            key, value = line.split("=", 1)
            values[key] = value
for key in ("gap_owner", "evidence_quality", "gap_confidence", "next_diagnostic", "outlier_observed"):
    if not values.get(key):
        raise SystemExit(f"missing {key}")
if values["evidence_quality"] not in {"stable", "noisy"}:
    raise SystemExit("invalid evidence_quality")
if values["gap_confidence"] not in {"low", "medium", "high"}:
    raise SystemExit("invalid gap_confidence")
if values["outlier_observed"] not in {"0", "1"}:
    raise SystemExit("invalid outlier_observed")
print("[phase296x-gap-taxonomy-refresh] ok")
PY

echo "[$TAG] ok"
