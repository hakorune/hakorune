#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-phase296x-mimalloc-provider-explicit-comparison-adapter-pilot"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

CARD_17="docs/development/current/main/phases/phase-296x/296x-17-MIMALLOC-PROVIDER-EXPLICIT-COMPARISON-CONTRACT.md"
CARD_18="docs/development/current/main/phases/phase-296x/296x-18-MIMALLOC-PROVIDER-EXPLICIT-COMPARISON-ADAPTER-PILOT.md"
TASKBOARD="docs/development/current/main/phases/phase-296x/296x-90-mimalloc-benchmark-taskboard.md"
INDEX="docs/tools/check-scripts-index.md"
TOOL="tools/allocator/provider_explicit_comparison_adapter.py"
PREV_GUARD="tools/checks/k2_wide_phase296x_mimalloc_provider_explicit_comparison_contract_guard.sh"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_mimalloc_provider_explicit_comparison_adapter_pilot_guard.sh"

echo "[$TAG] checking phase-296x provider explicit comparison adapter pilot"

guard_require_files "$TAG" "$CARD_17" "$CARD_18" "$TASKBOARD" "$INDEX" "$TOOL" "$PREV_GUARD" "$SELF_SCRIPT"
guard_require_exec_files "$TAG" "$TOOL" "$PREV_GUARD" "$SELF_SCRIPT"

guard_expect_fixed_in_file "$TAG" 'Status: Landed' "$CARD_17" "comparison contract card must be landed"
guard_expect_fixed_in_file "$TAG" 'Status: Landed' "$CARD_18" "adapter pilot card must be landed"
guard_expect_fixed_in_file "$TAG" 'MIMALLOC-PROVIDER-EXPLICIT-COMPARISON-ADAPTER-PILOT-296X-001' "$CARD_18" "adapter card must identify blocker"
guard_expect_fixed_in_file "$TAG" "$TOOL" "$CARD_18" "adapter card must name tool"
guard_expect_fixed_in_file "$TAG" 'output_contract=mimalloc-provider-explicit-comparison-adapter-v0' "$CARD_18" "adapter card must define output contract"
guard_expect_fixed_in_file "$TAG" 'input_contract=mimalloc-provider-explicit-comparison-contract-v0' "$CARD_18" "adapter card must consume comparison contract"
guard_expect_fixed_in_file "$TAG" 'comparison_subjects=hako_exact_exe,c_mimalloc_explicit_runner,provider_package_explicit_alloc_free' "$CARD_18" "adapter card must define 3-way subjects"
guard_expect_fixed_in_file "$TAG" 'subject_count=3' "$CARD_18" "adapter card must define subject count"
guard_expect_fixed_in_file "$TAG" 'winner_claim=0' "$CARD_18" "adapter card must keep winner claims closed"
guard_expect_fixed_in_file "$TAG" 'replacement_active=0' "$CARD_18" "adapter card must keep replacement closed"
guard_expect_fixed_in_file "$TAG" 'hook_installed=0' "$CARD_18" "adapter card must keep hooks closed"
guard_expect_fixed_in_file "$TAG" 'global_allocator=0' "$CARD_18" "adapter card must keep global allocator closed"
guard_expect_fixed_in_file "$TAG" 'MIMALLOC-PROVIDER-EXPLICIT-COMPARISON-CLOSEOUT-296X-001' "$CARD_18" "adapter card must select closeout"

guard_expect_fixed_in_file "$TAG" 'MIMALLOC-PROVIDER-EXPLICIT-COMPARISON-ADAPTER-PILOT-296X-001' "$TASKBOARD" "taskboard must expose adapter pilot row"
guard_expect_fixed_in_file "$TAG" '| 18 | `MIMALLOC-PROVIDER-EXPLICIT-COMPARISON-ADAPTER-PILOT-296X-001` | Landed |' "$TASKBOARD" "taskboard row 18 must be landed"
guard_expect_fixed_in_file "$TAG" 'MIMALLOC-PROVIDER-EXPLICIT-COMPARISON-CLOSEOUT-296X-001' "$TASKBOARD" "taskboard must expose comparison closeout row"
guard_expect_fixed_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list adapter guard"
guard_expect_fixed_in_file "$TAG" "$TOOL" "$INDEX" "check index must list adapter tool"

python3 -m py_compile "$TOOL"

tmp_dir="$(mktemp -d /tmp/hakorune_phase296x_provider_comparison_adapter.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT
hako_c_report="$tmp_dir/hako_c.out"
provider_report="$tmp_dir/provider.out"
out="$tmp_dir/adapter.out"

cat > "$hako_c_report" <<'REPORT'
output_contract=mimalloc-comparison-repeated-measurement-v0
measurement_profile=phase295x-repeated-v0
warmup_count=1
sample_count=3
operation_repeat=128
winner_claim=0
workload_0_id=representative-small-block-v0
workload_0_operation_family=small-block
workload_0_operation_repeat=128
workload_0_sample_count=3
workload_0_hako_external_elapsed_median_ms=70
workload_0_hako_external_rss_median_bytes=3641344
workload_0_c_external_elapsed_median_ms=70
workload_0_c_external_rss_median_bytes=3985408
REPORT

cat > "$provider_report" <<'REPORT'
output_contract=hakorune-provider-explicit-repeated-measurement-v0
measurement_profile=phase296x-provider-explicit-repeated-v0
provider_name=repeated-fixture
sample_count=3
warmup_count=1
operation_repeat=128
sample_elapsed_median_ns=87607
sample_rss_median_bytes=18714624
provider_active=0
replacement_active=0
hook_installed=0
global_allocator=0
winner_claim=0
summary=ok
REPORT

python3 "$TOOL" --hako-c-report "$hako_c_report" --provider-report "$provider_report" --out "$out"
guard_expect_fixed_in_file "$TAG" 'output_contract=mimalloc-provider-explicit-comparison-adapter-v0' "$out" "tool must emit adapter contract"
guard_expect_fixed_in_file "$TAG" 'input_contract=mimalloc-provider-explicit-comparison-contract-v0' "$out" "tool must emit input contract"
guard_expect_fixed_in_file "$TAG" 'measurement_profile=phase296x-provider-explicit-comparison-v0' "$out" "tool must emit profile"
guard_expect_fixed_in_file "$TAG" 'comparison_subjects=hako_exact_exe,c_mimalloc_explicit_runner,provider_package_explicit_alloc_free' "$out" "tool must emit subjects"
guard_expect_fixed_in_file "$TAG" 'workload_id=representative-small-block-v0' "$out" "tool must emit workload id"
guard_expect_fixed_in_file "$TAG" 'operation_family=small-block' "$out" "tool must emit operation family"
guard_expect_fixed_in_file "$TAG" 'sample_count=3' "$out" "tool must preserve sample count"
guard_expect_fixed_in_file "$TAG" 'warmup_count=1' "$out" "tool must preserve warmup count"
guard_expect_fixed_in_file "$TAG" 'operation_repeat=128' "$out" "tool must preserve operation repeat"
guard_expect_fixed_in_file "$TAG" 'subject_count=3' "$out" "tool must emit subject count"
guard_expect_fixed_in_file "$TAG" 'subject_0_id=hako_exact_exe' "$out" "tool must emit hako subject"
guard_expect_fixed_in_file "$TAG" 'subject_0_elapsed_median_unit=ms' "$out" "tool must emit hako timing unit"
guard_expect_fixed_in_file "$TAG" 'subject_0_elapsed_median_ms=70' "$out" "tool must emit hako timing"
guard_expect_fixed_in_file "$TAG" 'subject_0_rss_median_bytes=3641344' "$out" "tool must emit hako rss"
guard_expect_fixed_in_file "$TAG" 'subject_1_id=c_mimalloc_explicit_runner' "$out" "tool must emit C subject"
guard_expect_fixed_in_file "$TAG" 'subject_1_elapsed_median_unit=ms' "$out" "tool must emit C timing unit"
guard_expect_fixed_in_file "$TAG" 'subject_1_elapsed_median_ms=70' "$out" "tool must emit C timing"
guard_expect_fixed_in_file "$TAG" 'subject_1_rss_median_bytes=3985408' "$out" "tool must emit C rss"
guard_expect_fixed_in_file "$TAG" 'subject_2_id=provider_package_explicit_alloc_free' "$out" "tool must emit provider subject"
guard_expect_fixed_in_file "$TAG" 'subject_2_elapsed_median_unit=ns' "$out" "tool must emit provider timing unit"
guard_expect_fixed_in_file "$TAG" 'subject_2_elapsed_median_ns=87607' "$out" "tool must emit provider timing"
guard_expect_fixed_in_file "$TAG" 'subject_2_rss_median_bytes=18714624' "$out" "tool must emit provider rss"
guard_expect_fixed_in_file "$TAG" 'provider_activation_lane=parked' "$out" "tool must keep activation parked"
guard_expect_fixed_in_file "$TAG" 'provider_active=0' "$out" "tool must keep provider inactive"
guard_expect_fixed_in_file "$TAG" 'replacement_active=0' "$out" "tool must keep replacement closed"
guard_expect_fixed_in_file "$TAG" 'hook_installed=0' "$out" "tool must keep hooks closed"
guard_expect_fixed_in_file "$TAG" 'global_allocator=0' "$out" "tool must keep global allocator closed"
guard_expect_fixed_in_file "$TAG" 'winner_claim=0' "$out" "tool must keep winner closed"
guard_expect_fixed_in_file "$TAG" 'summary=ok' "$out" "tool must end ok"

cat "$out"
echo "[$TAG] ok"
