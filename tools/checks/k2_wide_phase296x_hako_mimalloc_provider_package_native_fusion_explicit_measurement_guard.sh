#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-phase296x-hako-mimalloc-provider-package-native-fusion-explicit-measurement"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

CARD_71="docs/development/current/main/phases/phase-296x/296x-71-HAKO-MIMALLOC-PROVIDER-PACKAGE-NATIVE-FUSION-EXPLICIT-MEASUREMENT.md"
CARD_72="docs/development/current/main/phases/phase-296x/296x-72-HAKO-MIMALLOC-HAKMEM-LDPRELOAD-SHIM-DECISION.md"
TASKBOARD="docs/development/current/main/phases/phase-296x/296x-90-mimalloc-benchmark-taskboard.md"
CURRENT_STATE="docs/development/current/main/CURRENT_STATE.toml"
INDEX="docs/tools/check-scripts-index.md"
TOOL="tools/allocator/hako_mimalloc_provider_package_native_fusion_explicit_measurement.py"
MEASURE_TOOL="tools/allocator/provider_package_explicit_repeated_measurement.py"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_hako_mimalloc_provider_package_native_fusion_explicit_measurement_guard.sh"
FIXTURE="apps/provider-package/hako-derived-mimalloc-real-entrypoint-fixture/main.hako"

echo "[$TAG] checking phase-296x native-fusion explicit provider measurement"

guard_require_files "$TAG" "$CARD_71" "$CARD_72" "$TASKBOARD" "$CURRENT_STATE" "$INDEX" "$TOOL" "$MEASURE_TOOL" "$SELF_SCRIPT" "$FIXTURE"
guard_require_exec_files "$TAG" "$TOOL" "$MEASURE_TOOL" "$SELF_SCRIPT"

guard_expect_fixed_in_file "$TAG" 'Status: Landed' "$CARD_71" "measurement card must be landed"
guard_expect_fixed_in_file "$TAG" 'Status: Current' "$CARD_72" "LD_PRELOAD decision card must be current"
guard_expect_fixed_in_file "$TAG" 'output_contract=hako-mimalloc-provider-package-native-fusion-explicit-measurement-v0' "$CARD_71" "card must record measurement contract"
guard_expect_fixed_in_file "$TAG" 'provider_explicit_measurement_ready=1' "$CARD_71" "card must record measurement readiness"
guard_expect_fixed_in_file "$TAG" 'ld_preload_decision_ready=1' "$CARD_71" "card must open LD_PRELOAD decision"
guard_expect_fixed_in_file "$TAG" 'replacement_active=0' "$CARD_71" "card must keep replacement closed"
guard_expect_fixed_in_file "$TAG" 'hook_installed=0' "$CARD_71" "card must keep hooks closed"
guard_expect_fixed_in_file "$TAG" 'global_allocator=0' "$CARD_71" "card must keep global allocator closed"
guard_expect_fixed_in_file "$TAG" 'winner_claim=0' "$CARD_71" "card must keep winner closed"

guard_expect_fixed_in_file "$TAG" 'latest_card = "296x-71-HAKO-MIMALLOC-PROVIDER-PACKAGE-NATIVE-FUSION-EXPLICIT-MEASUREMENT"' "$CURRENT_STATE" "current state latest card must advance to row 71"
guard_expect_fixed_in_file "$TAG" 'current_blocker_token = "HAKO-MIMALLOC-HAKMEM-LDPRELOAD-SHIM-DECISION-296X-001"' "$CURRENT_STATE" "current state must select LD_PRELOAD decision"
guard_expect_fixed_in_file "$TAG" '| 71 | `HAKO-MIMALLOC-PROVIDER-PACKAGE-NATIVE-FUSION-EXPLICIT-MEASUREMENT-296X-001` | Landed |' "$TASKBOARD" "taskboard row 71 must be landed"
guard_expect_fixed_in_file "$TAG" '| 72 | `HAKO-MIMALLOC-HAKMEM-LDPRELOAD-SHIM-DECISION-296X-001` | Current |' "$TASKBOARD" "taskboard row 72 must be current"
guard_expect_fixed_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list this guard"
guard_expect_fixed_in_file "$TAG" "$TOOL" "$INDEX" "check index must list measurement adapter"

python3 -m py_compile "$TOOL" "$MEASURE_TOOL"
cargo build -q --bin hakorune

tmp_dir="$(mktemp -d /tmp/hakorune_phase296x_native_fusion_measurement.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT
pkg="$tmp_dir/pkg"
build_out="$tmp_dir/build.out"
measurement_out="$tmp_dir/measurement.out"
report="$tmp_dir/report.out"

target/debug/hakorune \
  --provider-package-hako-derived-build-fixture "$FIXTURE" \
  --provider-package-hako-semantic-codegen object-lifecycle-small-alloc-release-v0 \
  --provider-package-out-dir "$pkg" \
  --provider-package-artifact-name libhakorune_provider.so \
  --provider-package-id org.hakorune.provider.hako.mimalloc.real-entrypoint \
  --provider-package-name hako-mimalloc-real-entrypoint-provider \
  --provider-package-version 0.1.0 \
  --provider-package-target-triple x86_64-unknown-linux-gnu \
  --provider-package-platform linux \
  --provider-package-provider-call-allowed \
  --provider-package-force \
  > "$build_out"

python3 "$MEASURE_TOOL" \
  --manifest "$pkg/hakorune_provider.json" \
  --sample-count 3 \
  --warmup-count 1 \
  --operation-repeat 8192 \
  --size 32 \
  --align 8 \
  --out "$measurement_out"

python3 "$TOOL" --build-report "$build_out" --measurement-report "$measurement_out" --out "$report"

guard_expect_fixed_in_file "$TAG" 'output_contract=hako-mimalloc-provider-package-native-fusion-explicit-measurement-v0' "$report" "tool must emit row contract"
guard_expect_fixed_in_file "$TAG" 'hako_semantic_provider_codegen=object-lifecycle-small-alloc-release-v0' "$report" "tool must preserve codegen mode"
guard_expect_fixed_in_file "$TAG" 'measurement_profile=provider-native-fusion-explicit-repeated-v0' "$report" "tool must record profile"
guard_expect_fixed_in_file "$TAG" 'operation_repeat=8192' "$report" "tool must use selected operation repeat"
guard_expect_fixed_in_file "$TAG" 'provider_explicit_measurement_ready=1' "$report" "tool must mark measurement ready"
guard_expect_fixed_in_file "$TAG" 'ld_preload_decision_ready=1' "$report" "tool must mark LD decision ready"
guard_expect_fixed_in_file "$TAG" 'provider_call_executed=1' "$report" "tool must call provider"
guard_expect_fixed_in_file "$TAG" 'replacement_active=0' "$report" "tool must keep replacement closed"
guard_expect_fixed_in_file "$TAG" 'hook_installed=0' "$report" "tool must keep hooks closed"
guard_expect_fixed_in_file "$TAG" 'global_allocator=0' "$report" "tool must keep global allocator closed"
guard_expect_fixed_in_file "$TAG" 'ld_preload_shim_ready=0' "$report" "tool must keep LD_PRELOAD not built"
guard_expect_fixed_in_file "$TAG" 'winner_claim=0' "$report" "tool must keep winner closed"
guard_expect_fixed_in_file "$TAG" 'next_row=HAKO-MIMALLOC-HAKMEM-LDPRELOAD-SHIM-DECISION-296X-001' "$report" "tool must select LD decision"
guard_expect_fixed_in_file "$TAG" 'summary=ok' "$report" "tool must end ok"

echo "[$TAG] ok"
