#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-phase296x-mimalloc-provider-package-explicit-comparison-pilot"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

CARD_40="docs/development/current/main/phases/phase-296x/296x-40-MIMALLOC-PROVIDER-PACKAGE-EXPLICIT-COMPARISON-PILOT.md"
TASKBOARD="docs/development/current/main/phases/phase-296x/296x-90-mimalloc-benchmark-taskboard.md"
CURRENT_STATE="docs/development/current/main/CURRENT_STATE.toml"
INDEX="docs/tools/check-scripts-index.md"
FIXTURE="apps/provider-package/hako-derived-allocator-fixture/main.hako"
HAKO_C_RUNNER="tools/allocator/mimalloc_repeated_measurement_runner.py"
PROVIDER_MEASURE="tools/allocator/provider_package_explicit_repeated_measurement.py"
COMPARISON_ADAPTER="tools/allocator/provider_explicit_comparison_adapter.py"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_mimalloc_provider_package_explicit_comparison_pilot_guard.sh"

echo "[$TAG] checking phase-296x provider package explicit comparison pilot"

guard_require_files "$TAG" "$CARD_40" "$TASKBOARD" "$CURRENT_STATE" "$INDEX" "$FIXTURE" "$HAKO_C_RUNNER" "$PROVIDER_MEASURE" "$COMPARISON_ADAPTER" "$SELF_SCRIPT"
guard_require_exec_files "$TAG" "$HAKO_C_RUNNER" "$PROVIDER_MEASURE" "$COMPARISON_ADAPTER" "$SELF_SCRIPT"

guard_expect_fixed_in_file "$TAG" 'Status: Landed' "$CARD_40" "pilot card must be landed"
guard_expect_fixed_in_file "$TAG" 'MIMALLOC-PROVIDER-PACKAGE-EXPLICIT-COMPARISON-PILOT-296X-001' "$CARD_40" "pilot card must identify blocker"
guard_expect_fixed_in_file "$TAG" 'alloc-free-owns-literal-v0' "$CARD_40" "pilot card must use .hako-derived semantic provider package"
guard_expect_fixed_in_file "$TAG" 'comparison_subjects=hako_exact_exe,c_mimalloc_explicit_runner,provider_package_explicit_alloc_free' "$CARD_40" "pilot card must keep 3-way subjects"
guard_expect_fixed_in_file "$TAG" 'workload_id=representative-small-block-v0' "$CARD_40" "pilot card must record workload"
guard_expect_fixed_in_file "$TAG" 'sample_count=3' "$CARD_40" "pilot card must record sample count"
guard_expect_fixed_in_file "$TAG" 'warmup_count=1' "$CARD_40" "pilot card must record warmup count"
guard_expect_fixed_in_file "$TAG" 'operation_repeat=128' "$CARD_40" "pilot card must record operation repeat"
guard_expect_fixed_in_file "$TAG" 'output_contract=hakorune-provider-explicit-repeated-measurement-v0' "$CARD_40" "pilot card must include provider repeated evidence"
guard_expect_fixed_in_file "$TAG" 'output_contract=mimalloc-provider-explicit-comparison-adapter-v0' "$CARD_40" "pilot card must include adapter evidence"
guard_expect_fixed_in_file "$TAG" 'provider_active=0' "$CARD_40" "pilot card must keep provider inactive"
guard_expect_fixed_in_file "$TAG" 'replacement_active=0' "$CARD_40" "pilot card must keep replacement closed"
guard_expect_fixed_in_file "$TAG" 'winner_claim=0' "$CARD_40" "pilot card must keep winner claims closed"
guard_expect_fixed_in_file "$TAG" 'MIMALLOC-PROVIDER-PACKAGE-EXPLICIT-COMPARISON-CLOSEOUT-296X-001' "$CARD_40" "pilot card must select closeout"

guard_expect_fixed_in_file "$TAG" 'latest_card = "296x-40-MIMALLOC-PROVIDER-PACKAGE-EXPLICIT-COMPARISON-PILOT"' "$CURRENT_STATE" "current state latest card must advance"
guard_expect_fixed_in_file "$TAG" 'current_blocker_token = "MIMALLOC-PROVIDER-PACKAGE-EXPLICIT-COMPARISON-CLOSEOUT-296X-001"' "$CURRENT_STATE" "current state must select comparison closeout"
guard_expect_fixed_in_file "$TAG" '| 40 | `MIMALLOC-PROVIDER-PACKAGE-EXPLICIT-COMPARISON-PILOT-296X-001` | Landed |' "$TASKBOARD" "taskboard row 40 must be landed"
guard_expect_fixed_in_file "$TAG" '| 41 | `MIMALLOC-PROVIDER-PACKAGE-EXPLICIT-COMPARISON-CLOSEOUT-296X-001` | Current |' "$TASKBOARD" "taskboard row 41 must be current"
guard_expect_fixed_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list comparison pilot guard"

python3 -m py_compile "$HAKO_C_RUNNER" "$PROVIDER_MEASURE" "$COMPARISON_ADAPTER"
cargo build -q --bin hakorune

library_path="$(guard_find_mimalloc_library "$TAG")"
tmp_dir="$(mktemp -d /tmp/hakorune_phase296x_provider_package_comparison.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT
pkg="$tmp_dir/pkg"
build_out="$tmp_dir/build.out"
provider_out="$tmp_dir/provider.out"
hako_c_out="$tmp_dir/hako_c.out"
comparison_out="$tmp_dir/comparison.out"

target/debug/hakorune \
  --provider-package-hako-derived-build-fixture "$FIXTURE" \
  --provider-package-hako-semantic-codegen alloc-free-owns-literal-v0 \
  --provider-package-out-dir "$pkg" \
  --provider-package-artifact-name libhakorune_provider.so \
  --provider-package-id org.hakorune.provider.hako.fixture \
  --provider-package-name hako-derived-fixture-provider \
  --provider-package-version 0.1.0 \
  --provider-package-target-triple x86_64-unknown-linux-gnu \
  --provider-package-platform linux \
  --provider-package-provider-call-allowed \
  --provider-package-force \
  > "$build_out"

python3 "$PROVIDER_MEASURE" \
  --manifest "$pkg/hakorune_provider.json" \
  --sample-count 3 \
  --warmup-count 1 \
  --operation-repeat 128 \
  --size 32 \
  --align 8 \
  --out "$provider_out"

python3 "$HAKO_C_RUNNER" \
  --out "$hako_c_out" \
  --workload representative-small-block-v0 \
  --sample-count 3 \
  --warmup-count 1 \
  --hako-runtime-config empty \
  --operation-repeat 128 \
  --c-library "$library_path"

python3 "$COMPARISON_ADAPTER" \
  --hako-c-report "$hako_c_out" \
  --provider-report "$provider_out" \
  --out "$comparison_out"

guard_expect_fixed_in_file "$TAG" 'hako_semantic_provider_codegen=alloc-free-owns-literal-v0' "$build_out" "build must use .hako-derived provider package mode"
guard_expect_fixed_in_file "$TAG" 'hako_provider_owns_value=1' "$build_out" "build must retain .hako owns semantic"
guard_expect_fixed_in_file "$TAG" 'summary=ok' "$build_out" "build must end ok"

guard_expect_fixed_in_file "$TAG" 'output_contract=hakorune-provider-explicit-repeated-measurement-v0' "$provider_out" "provider measurement must use repeated contract"
guard_expect_fixed_in_file "$TAG" 'sample_count=3' "$provider_out" "provider measurement must keep sample count"
guard_expect_fixed_in_file "$TAG" 'warmup_count=1' "$provider_out" "provider measurement must keep warmup count"
guard_expect_fixed_in_file "$TAG" 'operation_repeat=128' "$provider_out" "provider measurement must keep operation repeat"
guard_expect_fixed_in_file "$TAG" 'provider_call_executed=1' "$provider_out" "provider measurement must call provider"
guard_expect_fixed_in_file "$TAG" 'allocator_entrypoint_called=1' "$provider_out" "provider measurement must call allocator entrypoint"
guard_expect_fixed_in_file "$TAG" 'replacement_active=0' "$provider_out" "provider measurement must keep replacement closed"
guard_expect_fixed_in_file "$TAG" 'winner_claim=0' "$provider_out" "provider measurement must keep winners closed"
guard_expect_fixed_in_file "$TAG" 'summary=ok' "$provider_out" "provider measurement must end ok"

guard_expect_fixed_in_file "$TAG" 'output_contract=mimalloc-comparison-repeated-measurement-v0' "$hako_c_out" "hako/C measurement must use repeated contract"
guard_expect_fixed_in_file "$TAG" 'workload_0_id=representative-small-block-v0' "$hako_c_out" "hako/C measurement must keep workload"
guard_expect_fixed_in_file "$TAG" 'workload_0_winner_claim=0' "$hako_c_out" "hako/C measurement must keep workload winner closed"
guard_expect_fixed_in_file "$TAG" 'winner_claim=0' "$hako_c_out" "hako/C measurement must keep winners closed"
guard_expect_fixed_in_file "$TAG" 'summary=ok' "$hako_c_out" "hako/C measurement must end ok"

guard_expect_fixed_in_file "$TAG" 'output_contract=mimalloc-provider-explicit-comparison-adapter-v0' "$comparison_out" "adapter must emit comparison contract"
guard_expect_fixed_in_file "$TAG" 'comparison_subjects=hako_exact_exe,c_mimalloc_explicit_runner,provider_package_explicit_alloc_free' "$comparison_out" "adapter must keep subject list"
guard_expect_fixed_in_file "$TAG" 'workload_id=representative-small-block-v0' "$comparison_out" "adapter must keep workload"
guard_expect_fixed_in_file "$TAG" 'subject_count=3' "$comparison_out" "adapter must keep three subjects"
guard_expect_fixed_in_file "$TAG" 'subject_0_id=hako_exact_exe' "$comparison_out" "adapter must include hako subject"
guard_expect_fixed_in_file "$TAG" 'subject_1_id=c_mimalloc_explicit_runner' "$comparison_out" "adapter must include C subject"
guard_expect_fixed_in_file "$TAG" 'subject_2_id=provider_package_explicit_alloc_free' "$comparison_out" "adapter must include provider package subject"
guard_expect_fixed_in_file "$TAG" 'provider_active=0' "$comparison_out" "adapter must keep provider inactive"
guard_expect_fixed_in_file "$TAG" 'replacement_active=0' "$comparison_out" "adapter must keep replacement closed"
guard_expect_fixed_in_file "$TAG" 'hook_installed=0' "$comparison_out" "adapter must keep hooks closed"
guard_expect_fixed_in_file "$TAG" 'global_allocator=0' "$comparison_out" "adapter must keep global allocator closed"
guard_expect_fixed_in_file "$TAG" 'winner_claim=0' "$comparison_out" "adapter must keep winner claims closed"
guard_expect_fixed_in_file "$TAG" 'summary=ok' "$comparison_out" "adapter must end ok"

cat "$comparison_out"
echo "[$TAG] ok"
