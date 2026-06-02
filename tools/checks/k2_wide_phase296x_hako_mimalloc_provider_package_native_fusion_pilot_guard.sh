#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-phase296x-hako-mimalloc-provider-package-native-fusion-pilot"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

CARD_70="docs/development/current/main/phases/phase-296x/296x-70-HAKO-MIMALLOC-PROVIDER-PACKAGE-NATIVE-FUSION-PILOT.md"
TASKBOARD="docs/development/current/main/phases/phase-296x/296x-90-mimalloc-benchmark-taskboard.md"
CURRENT_STATE="docs/development/current/main/CURRENT_STATE.toml"
INDEX="docs/tools/check-scripts-index.md"
TOOL="tools/allocator/hako_mimalloc_provider_package_native_fusion_pilot.py"
ALLOC_FREE_TOOL="tools/allocator/provider_package_alloc_free_smoke.py"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_hako_mimalloc_provider_package_native_fusion_pilot_guard.sh"
CLI_IMPL="src/cli/provider_package_hako_derived_build.rs"
CLI_ARGS="src/cli/args.rs"
FIXTURE="apps/provider-package/hako-derived-mimalloc-real-entrypoint-fixture/main.hako"

echo "[$TAG] checking phase-296x hako mimalloc provider package native fusion pilot"

guard_require_files "$TAG" "$CARD_70" "$TASKBOARD" "$CURRENT_STATE" "$INDEX" "$TOOL" "$ALLOC_FREE_TOOL" "$SELF_SCRIPT" "$CLI_IMPL" "$CLI_ARGS" "$FIXTURE"
guard_require_exec_files "$TAG" "$TOOL" "$ALLOC_FREE_TOOL" "$SELF_SCRIPT"

guard_expect_fixed_in_file "$TAG" 'Status: Landed' "$CARD_70" "native fusion pilot card must be landed"
guard_expect_fixed_in_file "$TAG" 'hako_semantic_provider_codegen=object-lifecycle-small-alloc-release-v0' "$CARD_70" "card must record codegen mode"
guard_expect_fixed_in_file "$TAG" 'hako_entrypoint_mir_call_chain_verified=1' "$CARD_70" "card must record MIR call chain"
guard_expect_fixed_in_file "$TAG" 'provider_alloc_executed=1' "$CARD_70" "card must record provider alloc"
guard_expect_fixed_in_file "$TAG" 'provider_free_executed=1' "$CARD_70" "card must record provider free"
guard_expect_fixed_in_file "$TAG" 'replacement_active=0' "$CARD_70" "card must keep replacement closed"
guard_expect_fixed_in_file "$TAG" 'hook_installed=0' "$CARD_70" "card must keep hooks closed"
guard_expect_fixed_in_file "$TAG" 'global_allocator=0' "$CARD_70" "card must keep global allocator closed"
guard_expect_fixed_in_file "$TAG" 'winner_claim=0' "$CARD_70" "card must keep winner closed"

guard_expect_fixed_in_file "$TAG" 'object-lifecycle-small-alloc-release-v0' "$CLI_IMPL" "CLI impl must support object lifecycle mode"
guard_expect_fixed_in_file "$TAG" 'validate_hako_provider_object_lifecycle_entrypoint' "$CLI_IMPL" "CLI impl must validate object lifecycle MIR"
guard_expect_fixed_in_file "$TAG" 'HakoProvider.objectLifecycleSmallAllocReleaseOk/0' "$CLI_IMPL" "CLI impl must require provider entrypoint function"
guard_expect_fixed_in_file "$TAG" 'HakoAllocPageModel.acquireFreshSmall/1' "$CLI_IMPL" "CLI impl must verify current small alloc hot entrypoint"
guard_expect_fixed_in_file "$TAG" 'HakoAllocPageModel.releaseLocalKnownLive/1' "$CLI_IMPL" "CLI impl must verify current release hot entrypoint"
guard_expect_fixed_in_file "$TAG" 'object-lifecycle-small-alloc-release-v0' "$CLI_ARGS" "CLI args must document object lifecycle mode"
guard_expect_fixed_in_file "$TAG" 'objectLifecycleSmallAllocReleaseOk()' "$FIXTURE" "fixture must define selected provider proof method"
guard_expect_fixed_in_file "$TAG" 'objectLifecycleSmallAlloc(8)' "$FIXTURE" "fixture must call selected alloc"
guard_expect_fixed_in_file "$TAG" 'objectLifecycleReleaseBlock(alloc_page, alloc_block)' "$FIXTURE" "fixture must call selected release"

guard_expect_fixed_in_file "$TAG" 'latest_card = "296x-70-HAKO-MIMALLOC-PROVIDER-PACKAGE-NATIVE-FUSION-PILOT"' "$CURRENT_STATE" "current state latest card must advance to row 70"
guard_expect_fixed_in_file "$TAG" 'current_blocker_token = "HAKO-MIMALLOC-HAKMEM-LDPRELOAD-SHIM-DECISION-296X-001"' "$CURRENT_STATE" "current state must select LD_PRELOAD decision"
guard_expect_fixed_in_file "$TAG" '| 70 | `HAKO-MIMALLOC-PROVIDER-PACKAGE-NATIVE-FUSION-PILOT-296X-001` | Landed |' "$TASKBOARD" "taskboard row 70 must be landed"
guard_expect_fixed_in_file "$TAG" '| 71 | `HAKO-MIMALLOC-HAKMEM-LDPRELOAD-SHIM-DECISION-296X-001` | Current |' "$TASKBOARD" "taskboard row 71 must be current"
guard_expect_fixed_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list this guard"
guard_expect_fixed_in_file "$TAG" "$TOOL" "$INDEX" "check index must list pilot tool"

python3 -m py_compile "$TOOL" "$ALLOC_FREE_TOOL"
cargo build -q --bin hakorune

tmp_dir="$(mktemp -d /tmp/hakorune_phase296x_native_fusion_pilot.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT
pkg="$tmp_dir/pkg"
build_out="$tmp_dir/build.out"
alloc_out="$tmp_dir/alloc.out"
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

python3 "$ALLOC_FREE_TOOL" --manifest "$pkg/hakorune_provider.json" --size 32 --align 8 --out "$alloc_out"
python3 "$TOOL" --build-report "$build_out" --alloc-free-report "$alloc_out" --out "$report"

guard_expect_fixed_in_file "$TAG" 'hako_semantic_provider_codegen=object-lifecycle-small-alloc-release-v0' "$build_out" "build must use object lifecycle mode"
guard_expect_fixed_in_file "$TAG" 'hako_provider_object_lifecycle_entrypoint_verified=1' "$build_out" "build must verify object lifecycle call chain"
guard_expect_fixed_in_file "$TAG" 'provider_call_executed=0' "$build_out" "build must not call provider"
guard_expect_fixed_in_file "$TAG" 'summary=ok' "$build_out" "build must end ok"

guard_expect_fixed_in_file "$TAG" 'output_contract=hakorune-provider-package-alloc-free-smoke-v0' "$alloc_out" "alloc/free smoke must pass"
guard_expect_fixed_in_file "$TAG" 'provider_call_executed=1' "$alloc_out" "alloc/free smoke must call provider"
guard_expect_fixed_in_file "$TAG" 'provider_alloc_executed=1' "$alloc_out" "alloc/free smoke must call alloc"
guard_expect_fixed_in_file "$TAG" 'provider_free_executed=1' "$alloc_out" "alloc/free smoke must call free"
guard_expect_fixed_in_file "$TAG" 'provider_owns_result=1' "$alloc_out" "alloc/free smoke must observe owns"
guard_expect_fixed_in_file "$TAG" 'replacement_active=0' "$alloc_out" "alloc/free smoke must keep replacement closed"
guard_expect_fixed_in_file "$TAG" 'summary=ok' "$alloc_out" "alloc/free smoke must end ok"

guard_expect_fixed_in_file "$TAG" 'output_contract=hako-mimalloc-provider-package-native-fusion-pilot-v0' "$report" "tool must emit row contract"
guard_expect_fixed_in_file "$TAG" 'hako_entrypoint_mir_call_chain_verified=1' "$report" "tool must record MIR verification"
guard_expect_fixed_in_file "$TAG" 'provider_package_native_fusion_pilot_executed=1' "$report" "tool must record pilot execution"
guard_expect_fixed_in_file "$TAG" 'provider_alloc_executed=1' "$report" "tool must record provider alloc"
guard_expect_fixed_in_file "$TAG" 'provider_free_executed=1' "$report" "tool must record provider free"
guard_expect_fixed_in_file "$TAG" 'ld_preload_shim_ready=0' "$report" "tool must keep LD_PRELOAD decision later"
guard_expect_fixed_in_file "$TAG" 'next_row=HAKO-MIMALLOC-HAKMEM-LDPRELOAD-SHIM-DECISION-296X-001' "$report" "tool must select LD_PRELOAD decision"
guard_expect_fixed_in_file "$TAG" 'summary=ok' "$report" "tool must end ok"

echo "[$TAG] ok"
