#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-phase296x-hako-mimalloc-provider-package-native-fusion-selection"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

CARD_69="docs/development/current/main/phases/phase-296x/296x-69-HAKO-MIMALLOC-PROVIDER-PACKAGE-NATIVE-FUSION-SELECTION.md"
CARD_70="docs/development/current/main/phases/phase-296x/296x-70-HAKO-MIMALLOC-PROVIDER-PACKAGE-NATIVE-FUSION-PILOT.md"
TASKBOARD="docs/development/current/main/phases/phase-296x/296x-90-mimalloc-benchmark-taskboard.md"
CURRENT_STATE="docs/development/current/main/CURRENT_STATE.toml"
INDEX="docs/tools/check-scripts-index.md"
TOOL="tools/allocator/hako_mimalloc_provider_package_native_fusion_selection.py"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_hako_mimalloc_provider_package_native_fusion_selection_guard.sh"
CLI_IMPL="src/cli/provider_package_hako_derived_build.rs"
CLI_ARGS="src/cli/args.rs"
SURFACE="lang/src/hako_alloc/memory/object_lifecycle_facade_box.hako"

echo "[$TAG] checking phase-296x hako mimalloc provider package native fusion selection"

guard_require_files "$TAG" "$CARD_69" "$CARD_70" "$TASKBOARD" "$CURRENT_STATE" "$INDEX" "$TOOL" "$SELF_SCRIPT" "$CLI_IMPL" "$CLI_ARGS" "$SURFACE"
guard_require_exec_files "$TAG" "$TOOL" "$SELF_SCRIPT"

guard_expect_fixed_in_file "$TAG" 'Status: Landed' "$CARD_69" "native fusion selection card must be landed"
guard_expect_fixed_in_file "$TAG" 'Status: Current' "$CARD_70" "native fusion pilot card must be current"
guard_expect_fixed_in_file "$TAG" 'native_fusion_strategy=hako_derived_provider_semantic_mode_extension_v0' "$CARD_69" "card must select semantic mode extension"
guard_expect_fixed_in_file "$TAG" 'required_codegen_mode=object-lifecycle-small-alloc-release-v0' "$CARD_69" "card must name required codegen mode"
guard_expect_fixed_in_file "$TAG" 'provider_package_native_fusion_allowed=1' "$CARD_69" "card must allow native fusion pilot"
guard_expect_fixed_in_file "$TAG" 'replacement_active=0' "$CARD_69" "card must keep replacement closed"
guard_expect_fixed_in_file "$TAG" 'hook_installed=0' "$CARD_69" "card must keep hook closed"
guard_expect_fixed_in_file "$TAG" 'global_allocator=0' "$CARD_69" "card must keep global allocator closed"
guard_expect_fixed_in_file "$TAG" 'winner_claim=0' "$CARD_69" "card must keep winner closed"

guard_expect_fixed_in_file "$TAG" 'provider_package_hako_derived_build' "$CLI_IMPL" "strategy owner must exist"
guard_expect_fixed_in_file "$TAG" 'alloc-free-owns-literal-v0' "$CLI_IMPL" "strategy owner must have existing semantic mode pattern"
guard_expect_fixed_in_file "$TAG" 'provider-package-hako-semantic-codegen' "$CLI_ARGS" "args owner must expose semantic codegen"
guard_expect_fixed_in_file "$TAG" 'objectLifecycleSmallAlloc(size)' "$SURFACE" "selected surface must expose alloc"
guard_expect_fixed_in_file "$TAG" 'objectLifecycleReleaseBlock(page_id, block_id)' "$SURFACE" "selected surface must expose release"

guard_expect_fixed_in_file "$TAG" 'latest_card = "296x-69-HAKO-MIMALLOC-PROVIDER-PACKAGE-NATIVE-FUSION-SELECTION"' "$CURRENT_STATE" "current state latest card must advance to row 69"
guard_expect_fixed_in_file "$TAG" 'current_blocker_token = "HAKO-MIMALLOC-PROVIDER-PACKAGE-NATIVE-FUSION-PILOT-296X-001"' "$CURRENT_STATE" "current state must select row 70 pilot"
guard_expect_fixed_in_file "$TAG" '| 69 | `HAKO-MIMALLOC-PROVIDER-PACKAGE-NATIVE-FUSION-SELECTION-296X-001` | Landed |' "$TASKBOARD" "taskboard row 69 must be landed"
guard_expect_fixed_in_file "$TAG" '| 70 | `HAKO-MIMALLOC-PROVIDER-PACKAGE-NATIVE-FUSION-PILOT-296X-001` | Current |' "$TASKBOARD" "taskboard row 70 must be current"
guard_expect_fixed_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list this guard"
guard_expect_fixed_in_file "$TAG" "$TOOL" "$INDEX" "check index must list selection tool"

tmp_dir="$(mktemp -d /tmp/hakorune_phase296x_native_fusion_selection.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT
report="$tmp_dir/report.out"
python3 "$TOOL" --repo-root "$ROOT_DIR" --out "$report"

guard_expect_fixed_in_file "$TAG" 'output_contract=hako-mimalloc-provider-package-native-fusion-selection-v0' "$report" "tool must emit selection contract"
guard_expect_fixed_in_file "$TAG" 'native_fusion_strategy=hako_derived_provider_semantic_mode_extension_v0' "$report" "tool must select strategy"
guard_expect_fixed_in_file "$TAG" 'required_codegen_mode=object-lifecycle-small-alloc-release-v0' "$report" "tool must name codegen mode"
guard_expect_fixed_in_file "$TAG" 'required_fixture=apps/provider-package/hako-derived-mimalloc-real-entrypoint-fixture/main.hako' "$report" "tool must name fixture"
guard_expect_fixed_in_file "$TAG" 'required_mir_call_chain_check=1' "$report" "tool must require MIR call checks"
guard_expect_fixed_in_file "$TAG" 'required_provider_alloc_free_smoke=1' "$report" "tool must require provider alloc/free smoke"
guard_expect_fixed_in_file "$TAG" 'provider_package_native_fusion_allowed=1' "$report" "tool must allow native fusion"
guard_expect_fixed_in_file "$TAG" 'replacement_active=0' "$report" "tool must keep replacement closed"
guard_expect_fixed_in_file "$TAG" 'hook_installed=0' "$report" "tool must keep hook closed"
guard_expect_fixed_in_file "$TAG" 'global_allocator=0' "$report" "tool must keep global allocator closed"
guard_expect_fixed_in_file "$TAG" 'ld_preload_shim_ready=0' "$report" "tool must keep LD_PRELOAD later"
guard_expect_fixed_in_file "$TAG" 'winner_claim=0' "$report" "tool must keep winner closed"
guard_expect_fixed_in_file "$TAG" 'next_row=HAKO-MIMALLOC-PROVIDER-PACKAGE-NATIVE-FUSION-PILOT-296X-001' "$report" "tool must select pilot"
guard_expect_fixed_in_file "$TAG" 'summary=ok' "$report" "tool must end ok"

echo "[$TAG] ok"
