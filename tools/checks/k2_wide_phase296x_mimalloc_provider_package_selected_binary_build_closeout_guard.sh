#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-phase296x-mimalloc-provider-package-selected-binary-build-closeout"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

CARD_27="docs/development/current/main/phases/phase-296x/296x-27-MIMALLOC-PROVIDER-PACKAGE-SELECTED-BINARY-BUILD-CONTRACT-PILOT.md"
CARD_28="docs/development/current/main/phases/phase-296x/296x-28-MIMALLOC-PROVIDER-PACKAGE-SELECTED-BINARY-BUILD-CLOSEOUT.md"
TASKBOARD="docs/development/current/main/phases/phase-296x/296x-90-mimalloc-benchmark-taskboard.md"
INDEX="docs/tools/check-scripts-index.md"
CURRENT_STATE="docs/development/current/main/CURRENT_STATE.toml"
CLI_IMPL="src/cli/provider_package_selected_binary_build.rs"
METADATA_TOOL="tools/allocator/provider_package_metadata_preflight.py"
DESCRIPTOR_TOOL="tools/allocator/provider_package_descriptor_smoke.py"
API_BIND_TOOL="tools/allocator/provider_package_api_bind_smoke.py"
MEASURE_TOOL="tools/allocator/provider_package_explicit_repeated_measurement.py"
COMPARISON_TOOL="tools/allocator/provider_explicit_comparison_adapter.py"
REPEATED_RUNNER="tools/allocator/mimalloc_repeated_measurement_runner.py"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_mimalloc_provider_package_selected_binary_build_closeout_guard.sh"

echo "[$TAG] checking phase-296x selected provider binary build closeout"

guard_require_files "$TAG" "$CARD_27" "$CARD_28" "$TASKBOARD" "$INDEX" "$CURRENT_STATE" "$CLI_IMPL" "$METADATA_TOOL" "$DESCRIPTOR_TOOL" "$API_BIND_TOOL" "$MEASURE_TOOL" "$COMPARISON_TOOL" "$REPEATED_RUNNER" "$SELF_SCRIPT"
guard_require_exec_files "$TAG" "$METADATA_TOOL" "$DESCRIPTOR_TOOL" "$API_BIND_TOOL" "$MEASURE_TOOL" "$COMPARISON_TOOL" "$REPEATED_RUNNER" "$SELF_SCRIPT"

guard_expect_fixed_in_file "$TAG" 'Status: Landed' "$CARD_27" "selected build pilot must be landed"
guard_expect_fixed_in_file "$TAG" 'Status: Landed' "$CARD_28" "selected build closeout must be landed"
guard_expect_fixed_in_file "$TAG" 'MIMALLOC-PROVIDER-PACKAGE-SELECTED-BINARY-BUILD-CLOSEOUT-296X-001' "$CARD_28" "closeout card must identify blocker"
guard_expect_fixed_in_file "$TAG" 'output_contract=hakorune-provider-package-selected-binary-build-v0' "$CARD_28" "closeout must preserve selected build contract"
guard_expect_fixed_in_file "$TAG" 'hako_shared_library_generation=0' "$CARD_28" "closeout must keep .hako generation closed"
guard_expect_fixed_in_file "$TAG" 'metadata-preflight=ok' "$CARD_28" "closeout must record metadata preflight evidence"
guard_expect_fixed_in_file "$TAG" 'descriptor-smoke=ok' "$CARD_28" "closeout must record descriptor evidence"
guard_expect_fixed_in_file "$TAG" 'provider-api-bind=ok' "$CARD_28" "closeout must record API bind evidence"
guard_expect_fixed_in_file "$TAG" 'provider-explicit-repeated-measurement=ok' "$CARD_28" "closeout must record repeated measurement evidence"
guard_expect_fixed_in_file "$TAG" 'provider-explicit-comparison-adapter=ok' "$CARD_28" "closeout must record comparison adapter evidence"
guard_expect_fixed_in_file "$TAG" 'provider_active=0' "$CARD_28" "closeout must keep provider inactive"
guard_expect_fixed_in_file "$TAG" 'replacement_active=0' "$CARD_28" "closeout must keep replacement closed"
guard_expect_fixed_in_file "$TAG" 'hook_installed=0' "$CARD_28" "closeout must keep hooks closed"
guard_expect_fixed_in_file "$TAG" 'global_allocator=0' "$CARD_28" "closeout must keep global allocator closed"
guard_expect_fixed_in_file "$TAG" 'winner_claim=0' "$CARD_28" "closeout must keep winner claims closed"
guard_expect_fixed_in_file "$TAG" 'MIMALLOC-PROVIDER-PACKAGE-HAKO-DERIVED-BUILD-SELECTION-296X-001' "$CARD_28" "closeout must select Phase C .hako-derived build selection"

guard_expect_fixed_in_file "$TAG" 'selected_fixture_source(&contract_hash, &function_table_hash)' "$CLI_IMPL" "selected build must inject descriptor hashes into generated source"
guard_expect_fixed_in_file "$TAG" 'hakorune-provider-api-v1' "$CLI_IMPL" "selected build must use Hakorune API table schema name"
guard_expect_fixed_in_file "$TAG" 'const HakoProviderApiV1* hakorune_provider_get_api_v1(void)' "$CLI_IMPL" "selected build must expose API table pointer ABI"
guard_expect_fixed_in_file "$TAG" 'descriptor_read_executed=0' "$CLI_IMPL" "package build command must not read descriptor"
guard_expect_fixed_in_file "$TAG" 'provider_call_executed=0' "$CLI_IMPL" "package build command must not call provider"

guard_expect_fixed_in_file "$TAG" '| 28 | `MIMALLOC-PROVIDER-PACKAGE-SELECTED-BINARY-BUILD-CLOSEOUT-296X-001` | Landed |' "$TASKBOARD" "taskboard row 28 must be landed"
guard_expect_fixed_in_file "$TAG" '| 29 | `MIMALLOC-PROVIDER-PACKAGE-HAKO-DERIVED-BUILD-SELECTION-296X-001` | Current |' "$TASKBOARD" "taskboard row 29 must be current"
guard_expect_fixed_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list closeout guard"

python3 -m py_compile "$METADATA_TOOL" "$DESCRIPTOR_TOOL" "$API_BIND_TOOL" "$MEASURE_TOOL" "$COMPARISON_TOOL" "$REPEATED_RUNNER"
cargo build -q --bin hakorune

tmp_dir="$(mktemp -d /tmp/hakorune_phase296x_selected_build_closeout.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT
pkg="$tmp_dir/pkg"
build_out="$tmp_dir/build.out"
metadata_out="$tmp_dir/metadata.out"
descriptor_out="$tmp_dir/descriptor.out"
api_bind_out="$tmp_dir/api-bind.out"
provider_out="$tmp_dir/provider.out"
hako_c_out="$tmp_dir/hako-c.out"
comparison_out="$tmp_dir/comparison.out"

target/debug/hakorune \
  --provider-package-selected-binary-build-fixture \
  --provider-package-out-dir "$pkg" \
  --provider-package-artifact-name libhakorune_provider.so \
  --provider-package-id org.hakorune.provider.selected.fixture \
  --provider-package-name selected-fixture-provider \
  --provider-package-version 0.1.0 \
  --provider-package-target-triple x86_64-unknown-linux-gnu \
  --provider-package-platform linux \
  --provider-package-provider-call-allowed \
  --provider-package-force \
  > "$build_out"

python3 "$METADATA_TOOL" --manifest "$pkg/hakorune_provider.json" --out "$metadata_out"
python3 "$DESCRIPTOR_TOOL" --manifest "$pkg/hakorune_provider.json" --out "$descriptor_out"
python3 "$API_BIND_TOOL" --manifest "$pkg/hakorune_provider.json" --out "$api_bind_out"
python3 "$MEASURE_TOOL" --manifest "$pkg/hakorune_provider.json" --out "$provider_out" --sample-count 3 --warmup-count 1 --operation-repeat 128 --size 32 --align 8

library_path="$(guard_find_mimalloc_library "$TAG")"
python3 "$REPEATED_RUNNER" \
  --out "$hako_c_out" \
  --workload representative-small-block-v0 \
  --sample-count 3 \
  --warmup-count 1 \
  --hako-runtime-config empty \
  --operation-repeat 128 \
  --c-library "$library_path" \
  > "$tmp_dir/hako-c.stdout"

python3 "$COMPARISON_TOOL" --hako-c-report "$hako_c_out" --provider-report "$provider_out" --out "$comparison_out"

guard_expect_fixed_in_file "$TAG" 'output_contract=hakorune-provider-package-selected-binary-build-v0' "$build_out" "selected build must emit package contract"
guard_expect_fixed_in_file "$TAG" 'hako_shared_library_generation=0' "$build_out" "selected build must keep .hako generation closed"
guard_expect_fixed_in_file "$TAG" 'shared_library_load_executed=0' "$build_out" "selected build command must not load library"
guard_expect_fixed_in_file "$TAG" 'summary=ok' "$build_out" "selected build must end ok"

guard_expect_fixed_in_file "$TAG" 'output_contract=hakorune-provider-package-metadata-preflight-v0' "$metadata_out" "metadata preflight must pass"
guard_expect_fixed_in_file "$TAG" 'shared_library_load_executed=0' "$metadata_out" "metadata preflight must stay no-load"
guard_expect_fixed_in_file "$TAG" 'summary=ok' "$metadata_out" "metadata preflight must end ok"

guard_expect_fixed_in_file "$TAG" 'output_contract=hakorune-provider-package-descriptor-smoke-v0' "$descriptor_out" "descriptor smoke must pass"
guard_expect_fixed_in_file "$TAG" 'descriptor_contract_hash=' "$descriptor_out" "descriptor smoke must expose descriptor contract hash"
guard_expect_fixed_in_file "$TAG" 'summary=ok' "$descriptor_out" "descriptor smoke must end ok"

guard_expect_fixed_in_file "$TAG" 'output_contract=hakorune-provider-package-api-bind-smoke-v0' "$api_bind_out" "API bind smoke must pass"
guard_expect_fixed_in_file "$TAG" 'provider_api_bound=1' "$api_bind_out" "API bind smoke must bind API shape"
guard_expect_fixed_in_file "$TAG" 'provider_call_executed=0' "$api_bind_out" "API bind smoke must not call provider"
guard_expect_fixed_in_file "$TAG" 'summary=ok' "$api_bind_out" "API bind smoke must end ok"

guard_expect_fixed_in_file "$TAG" 'output_contract=hakorune-provider-explicit-repeated-measurement-v0' "$provider_out" "provider measurement must pass"
guard_expect_fixed_in_file "$TAG" 'sample_count=3' "$provider_out" "provider measurement must keep sample count"
guard_expect_fixed_in_file "$TAG" 'operation_repeat=128' "$provider_out" "provider measurement must keep operation repeat"
guard_expect_fixed_in_file "$TAG" 'provider_alloc_executed=1' "$provider_out" "provider measurement must call alloc"
guard_expect_fixed_in_file "$TAG" 'provider_free_executed=1' "$provider_out" "provider measurement must call free"
guard_expect_fixed_in_file "$TAG" 'winner_claim=0' "$provider_out" "provider measurement must not claim winner"
guard_expect_fixed_in_file "$TAG" 'summary=ok' "$provider_out" "provider measurement must end ok"

guard_expect_fixed_in_file "$TAG" 'output_contract=mimalloc-provider-explicit-comparison-adapter-v0' "$comparison_out" "comparison adapter must pass"
guard_expect_fixed_in_file "$TAG" 'comparison_subjects=hako_exact_exe,c_mimalloc_explicit_runner,provider_package_explicit_alloc_free' "$comparison_out" "comparison adapter must keep subjects"
guard_expect_fixed_in_file "$TAG" 'subject_count=3' "$comparison_out" "comparison adapter must keep three subjects"
guard_expect_fixed_in_file "$TAG" 'provider_active=0' "$comparison_out" "comparison adapter must keep provider inactive"
guard_expect_fixed_in_file "$TAG" 'replacement_active=0' "$comparison_out" "comparison adapter must keep replacement closed"
guard_expect_fixed_in_file "$TAG" 'winner_claim=0' "$comparison_out" "comparison adapter must keep winner claims closed"
guard_expect_fixed_in_file "$TAG" 'summary=ok' "$comparison_out" "comparison adapter must end ok"

cat "$build_out"
cat "$metadata_out"
cat "$descriptor_out"
cat "$api_bind_out"
cat "$provider_out"
cat "$comparison_out"
echo "[$TAG] ok"
