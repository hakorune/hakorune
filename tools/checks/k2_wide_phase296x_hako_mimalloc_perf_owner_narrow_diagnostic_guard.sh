#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-phase296x-hako-mimalloc-perf-owner-narrow-diagnostic"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

CARD_47="docs/development/current/main/phases/phase-296x/296x-47-HAKO-MIMALLOC-PERF-OWNER-NARROW-DIAGNOSTIC.md"
CARD_48="docs/development/current/main/phases/phase-296x/296x-48-HAKO-MIMALLOC-PERF-POST-DIAGNOSTIC-DECISION.md"
TASKBOARD="docs/development/current/main/phases/phase-296x/296x-90-mimalloc-benchmark-taskboard.md"
CURRENT_STATE="docs/development/current/main/CURRENT_STATE.toml"
INDEX="docs/tools/check-scripts-index.md"
BASELINE_RUNNER="tools/allocator/mimalloc_repeated_measurement_runner.py"
DIAGNOSTIC="tools/allocator/hako_mimalloc_owner_narrow_diagnostic.py"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_hako_mimalloc_perf_owner_narrow_diagnostic_guard.sh"

echo "[$TAG] checking phase-296x owner narrow diagnostic"

guard_require_files "$TAG" "$CARD_47" "$CARD_48" "$TASKBOARD" "$CURRENT_STATE" "$INDEX" "$BASELINE_RUNNER" "$DIAGNOSTIC" "$SELF_SCRIPT"
guard_require_exec_files "$TAG" "$BASELINE_RUNNER" "$DIAGNOSTIC" "$SELF_SCRIPT"

guard_expect_fixed_in_file "$TAG" 'Status: Landed' "$CARD_47" "owner diagnostic card must be landed"
guard_expect_fixed_in_file "$TAG" 'Status: Current' "$CARD_48" "post diagnostic decision card must be current"
guard_expect_fixed_in_file "$TAG" 'output_contract=hako-mimalloc-owner-narrow-diagnostic-v0' "$CARD_47" "card must define diagnostic contract"
guard_expect_fixed_in_file "$TAG" 'sample_count=5' "$CARD_47" "card must record hygiene sample count"
guard_expect_fixed_in_file "$TAG" 'build_compile_excluded=1' "$CARD_47" "card must record build exclusion"
guard_expect_fixed_in_file "$TAG" 'HAKO-MIMALLOC-PERF-POST-DIAGNOSTIC-DECISION-296X-001' "$CARD_47" "card must select row 48"

guard_expect_fixed_in_file "$TAG" 'latest_card = "296x-47-HAKO-MIMALLOC-PERF-OWNER-NARROW-DIAGNOSTIC"' "$CURRENT_STATE" "current state latest card must advance to row 47"
guard_expect_fixed_in_file "$TAG" 'current_blocker_token = "HAKO-MIMALLOC-PERF-POST-DIAGNOSTIC-DECISION-296X-001"' "$CURRENT_STATE" "current state must select row 48"
guard_expect_fixed_in_file "$TAG" '| 47 | `HAKO-MIMALLOC-PERF-OWNER-NARROW-DIAGNOSTIC-296X-001` | Landed |' "$TASKBOARD" "taskboard row 47 must be landed"
guard_expect_fixed_in_file "$TAG" '| 48 | `HAKO-MIMALLOC-PERF-POST-DIAGNOSTIC-DECISION-296X-001` | Current |' "$TASKBOARD" "taskboard row 48 must be current"
guard_expect_fixed_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list this guard"
guard_expect_fixed_in_file "$TAG" "$DIAGNOSTIC" "$INDEX" "check index must list diagnostic tool"

tmp_dir="$(mktemp -d /tmp/hakorune_phase296x_owner_diag.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT
selection="$tmp_dir/selection.out"
measurement="$tmp_dir/measurement.out"
diagnostic_out="$tmp_dir/diagnostic.out"

cat >"$selection" <<'EOF'
output_contract=hako-mimalloc-conditional-diagnostic-selection-v0
input_contract=hako-mimalloc-gap-taxonomy-v0
workload_id=representative-small-block-v0
measurement_profile=phase295x-repeated-v0
gap_owner=benchmark_harness
evidence_quality=noisy
gap_confidence=medium
outlier_observed=1
selected_diagnostic=measurement_hygiene_refresh
next_diagnostic=measurement_hygiene_refresh
next_diagnostic_suggestion_match=1
selection_reason=accepted
measurement_hygiene_required=1
next_optimization_allowed=0
selected_next_row=HAKO-MIMALLOC-PERF-OWNER-NARROW-DIAGNOSTIC-296X-001
body_elapsed_ns_primary=0
winner_claim=0
provider_active=0
replacement_active=0
hook_installed=0
global_allocator=0
summary=ok
EOF

python3 "$BASELINE_RUNNER" \
  --out "$measurement" \
  --workload representative-small-block-v0 \
  --sample-count 5 \
  --warmup-count 1 \
  --hako-runtime-config empty \
  --operation-repeat 128 \
  --c-library /lib/x86_64-linux-gnu/libmimalloc.so.2 >/dev/null

python3 "$DIAGNOSTIC" --selection "$selection" --measurement-report "$measurement" --out "$diagnostic_out"

guard_expect_fixed_in_file "$TAG" 'output_contract=hako-mimalloc-owner-narrow-diagnostic-v0' "$diagnostic_out" "diagnostic must emit contract"
guard_expect_fixed_in_file "$TAG" 'front=representative-small-block-v0' "$diagnostic_out" "diagnostic must preserve front"
guard_expect_fixed_in_file "$TAG" 'gap_owner=benchmark_harness' "$diagnostic_out" "diagnostic must preserve owner"
guard_expect_fixed_in_file "$TAG" 'diagnostic_kind=measurement_hygiene_refresh' "$diagnostic_out" "diagnostic must preserve selected diagnostic"
guard_expect_fixed_in_file "$TAG" 'measurement_contract=mimalloc-comparison-repeated-measurement-v0' "$diagnostic_out" "diagnostic must bind measurement contract"
guard_expect_fixed_in_file "$TAG" 'measurement_hygiene_required=1' "$diagnostic_out" "diagnostic must require hygiene"
guard_expect_fixed_in_file "$TAG" 'body_elapsed_ns_secondary=1' "$diagnostic_out" "diagnostic must keep body timing secondary"
guard_expect_fixed_in_file "$TAG" 'build_compile_excluded=1' "$diagnostic_out" "diagnostic must prove build exclusion"
guard_expect_fixed_in_file "$TAG" 'sample_count=5' "$diagnostic_out" "diagnostic must use sample_count 5"
guard_expect_fixed_in_file "$TAG" 'next_optimization_allowed=0' "$diagnostic_out" "diagnostic must block optimization"
guard_expect_fixed_in_file "$TAG" 'winner_claim=0' "$diagnostic_out" "diagnostic must keep winner claims closed"
guard_expect_fixed_in_file "$TAG" 'provider_active=0' "$diagnostic_out" "diagnostic must keep provider closed"
guard_expect_fixed_in_file "$TAG" 'replacement_active=0' "$diagnostic_out" "diagnostic must keep replacement closed"
guard_expect_fixed_in_file "$TAG" 'hook_installed=0' "$diagnostic_out" "diagnostic must keep hooks closed"
guard_expect_fixed_in_file "$TAG" 'global_allocator=0' "$diagnostic_out" "diagnostic must keep global allocator closed"
guard_expect_fixed_in_file "$TAG" 'summary=ok' "$diagnostic_out" "diagnostic must end ok"

echo "[$TAG] ok"
