#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-phase296x-hako-mimalloc-provider-real-entrypoint-pilot"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

CARD_68="docs/development/current/main/phases/phase-296x/296x-68-HAKO-MIMALLOC-PROVIDER-PACKAGE-REAL-ENTRYPOINT-PILOT.md"
CARD_69="docs/development/current/main/phases/phase-296x/296x-69-HAKO-MIMALLOC-PROVIDER-PACKAGE-NATIVE-FUSION-SELECTION.md"
TASKBOARD="docs/development/current/main/phases/phase-296x/296x-90-mimalloc-benchmark-taskboard.md"
CURRENT_STATE="docs/development/current/main/CURRENT_STATE.toml"
INDEX="docs/tools/check-scripts-index.md"
TOOL="tools/allocator/hako_mimalloc_provider_real_entrypoint_pilot.py"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_hako_mimalloc_provider_real_entrypoint_pilot_guard.sh"
APP="apps/mimalloc-facade-release-one-block-proof/main.hako"
SURFACE="lang/src/hako_alloc/memory/object_lifecycle_facade_box.hako"
HAKORUNE_BIN="target/debug/hakorune"

echo "[$TAG] checking phase-296x hako mimalloc provider real entrypoint pilot"

guard_require_files "$TAG" "$CARD_68" "$CARD_69" "$TASKBOARD" "$CURRENT_STATE" "$INDEX" "$TOOL" "$SELF_SCRIPT" "$APP" "$SURFACE"
guard_require_exec_files "$TAG" "$TOOL" "$SELF_SCRIPT" "$HAKORUNE_BIN"

guard_expect_fixed_in_file "$TAG" 'Status: Landed' "$CARD_68" "real entrypoint pilot card must be landed"
guard_expect_fixed_in_file "$TAG" 'Status: Current' "$CARD_69" "native fusion selection card must be current"
guard_expect_fixed_in_file "$TAG" 'selected_entrypoint=object_lifecycle_small_alloc_release_v0' "$CARD_68" "card must keep selected entrypoint"
guard_expect_fixed_in_file "$TAG" 'provider_call_executed=1' "$CARD_68" "card must record pilot call"
guard_expect_fixed_in_file "$TAG" 'provider_package_native_fused_to_hako_entrypoint=0' "$CARD_68" "card must not overclaim native fusion"
guard_expect_fixed_in_file "$TAG" 'provider_package_native_fusion_required=1' "$CARD_68" "card must select native fusion"
guard_expect_fixed_in_file "$TAG" 'replacement_active=0' "$CARD_68" "card must keep replacement closed"
guard_expect_fixed_in_file "$TAG" 'hook_installed=0' "$CARD_68" "card must keep hooks closed"
guard_expect_fixed_in_file "$TAG" 'global_allocator=0' "$CARD_68" "card must keep global allocator closed"
guard_expect_fixed_in_file "$TAG" 'winner_claim=0' "$CARD_68" "card must keep winner closed"

guard_expect_fixed_in_file "$TAG" 'objectLifecycleSmallAlloc(size)' "$SURFACE" "selected surface must expose alloc method"
guard_expect_fixed_in_file "$TAG" 'objectLifecycleReleaseBlock(page_id, block_id)' "$SURFACE" "selected surface must expose release method"
guard_expect_fixed_in_file "$TAG" 'objectLifecycleSmallAlloc(8)' "$APP" "pilot app must call selected alloc"
guard_expect_fixed_in_file "$TAG" 'objectLifecycleReleaseBlock(alloc_page, alloc_block)' "$APP" "pilot app must call selected release"

guard_expect_fixed_in_file "$TAG" 'latest_card = "296x-68-HAKO-MIMALLOC-PROVIDER-PACKAGE-REAL-ENTRYPOINT-PILOT"' "$CURRENT_STATE" "current state latest card must advance to row 68"
guard_expect_fixed_in_file "$TAG" 'current_blocker_token = "HAKO-MIMALLOC-PROVIDER-PACKAGE-NATIVE-FUSION-SELECTION-296X-001"' "$CURRENT_STATE" "current state must select row 69 native fusion"
guard_expect_fixed_in_file "$TAG" '| 68 | `HAKO-MIMALLOC-PROVIDER-PACKAGE-REAL-ENTRYPOINT-PILOT-296X-001` | Landed |' "$TASKBOARD" "taskboard row 68 must be landed"
guard_expect_fixed_in_file "$TAG" '| 69 | `HAKO-MIMALLOC-PROVIDER-PACKAGE-NATIVE-FUSION-SELECTION-296X-001` | Current |' "$TASKBOARD" "taskboard row 69 must be current"
guard_expect_fixed_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list this guard"
guard_expect_fixed_in_file "$TAG" "$TOOL" "$INDEX" "check index must list pilot tool"

tmp_dir="$(mktemp -d /tmp/hakorune_phase296x_entrypoint_pilot.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT
mir_json="$tmp_dir/pilot.mir.json"
run_log="$tmp_dir/run.log"
run_err="$tmp_dir/run.err"
report="$tmp_dir/report.out"

NYASH_FEATURES="${NYASH_FEATURES:-rune}" NYASH_DISABLE_PLUGINS="${NYASH_DISABLE_PLUGINS:-1}" \
  "$HAKORUNE_BIN" --backend mir --emit-mir-json "$mir_json" "$APP" >/dev/null

NYASH_FEATURES="${NYASH_FEATURES:-rune}" NYASH_DISABLE_PLUGINS="${NYASH_DISABLE_PLUGINS:-1}" \
  "$HAKORUNE_BIN" --backend vm "$APP" >"$run_log" 2>"$run_err"

guard_expect_fixed_in_file "$TAG" 'summary=ok' "$run_log" "pilot app must run ok"

python3 "$TOOL" --repo-root "$ROOT_DIR" --mir-json "$mir_json" --run-log "$run_log" --out "$report"

guard_expect_fixed_in_file "$TAG" 'output_contract=hako-mimalloc-provider-real-entrypoint-pilot-v0' "$report" "tool must emit pilot contract"
guard_expect_fixed_in_file "$TAG" 'selected_entrypoint=object_lifecycle_small_alloc_release_v0' "$report" "tool must keep selected entrypoint"
guard_expect_fixed_in_file "$TAG" 'provider_call_kind=hako_exact_exe_selected_entrypoint_pilot' "$report" "tool must name call kind"
guard_expect_fixed_in_file "$TAG" 'provider_call_executed=1' "$report" "tool must record call execution"
guard_expect_fixed_in_file "$TAG" 'hako_selected_entrypoint_executed=1' "$report" "tool must record hako entrypoint execution"
guard_expect_fixed_in_file "$TAG" 'alloc_method_called=objectLifecycleSmallAlloc' "$report" "tool must call selected alloc"
guard_expect_fixed_in_file "$TAG" 'release_method_called=objectLifecycleReleaseBlock' "$report" "tool must call selected release"
guard_expect_fixed_in_file "$TAG" 'mir_call_chain_verified=1' "$report" "tool must verify MIR call chain"
guard_expect_fixed_in_file "$TAG" 'exact_exe_run_verified=1' "$report" "tool must verify run output"
guard_expect_fixed_in_file "$TAG" 'provider_package_native_fused_to_hako_entrypoint=0' "$report" "tool must not overclaim native fusion"
guard_expect_fixed_in_file "$TAG" 'provider_package_native_fusion_required=1' "$report" "tool must require native fusion"
guard_expect_fixed_in_file "$TAG" 'replacement_active=0' "$report" "tool must keep replacement closed"
guard_expect_fixed_in_file "$TAG" 'hook_installed=0' "$report" "tool must keep hooks closed"
guard_expect_fixed_in_file "$TAG" 'global_allocator=0' "$report" "tool must keep global allocator closed"
guard_expect_fixed_in_file "$TAG" 'ld_preload_shim_ready=0' "$report" "tool must keep LD_PRELOAD later"
guard_expect_fixed_in_file "$TAG" 'winner_claim=0' "$report" "tool must keep winner closed"
guard_expect_fixed_in_file "$TAG" 'next_row=HAKO-MIMALLOC-PROVIDER-PACKAGE-NATIVE-FUSION-SELECTION-296X-001' "$report" "tool must select native fusion"
guard_expect_fixed_in_file "$TAG" 'summary=ok' "$report" "tool must end ok"

echo "[$TAG] ok"
