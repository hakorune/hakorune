#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-phase296x-hako-mimalloc-port-feature-gap-inventory"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

CARD_66="docs/development/current/main/phases/phase-296x/296x-66-HAKO-MIMALLOC-PORT-FEATURE-GAP-INVENTORY.md"
CARD_67="docs/development/current/main/phases/phase-296x/296x-67-HAKO-MIMALLOC-PROVIDER-PACKAGE-REAL-ENTRYPOINT-SELECTION.md"
TASKBOARD="docs/development/current/main/phases/phase-296x/296x-90-mimalloc-benchmark-taskboard.md"
CURRENT_STATE="docs/development/current/main/CURRENT_STATE.toml"
INDEX="docs/tools/check-scripts-index.md"
TOOL="tools/allocator/hako_mimalloc_port_feature_gap_inventory.py"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_hako_mimalloc_port_feature_gap_inventory_guard.sh"

echo "[$TAG] checking phase-296x hako mimalloc port feature gap inventory"

guard_require_files "$TAG" "$CARD_66" "$CARD_67" "$TASKBOARD" "$CURRENT_STATE" "$INDEX" "$TOOL" "$SELF_SCRIPT"
guard_require_exec_files "$TAG" "$TOOL" "$SELF_SCRIPT"

guard_expect_fixed_in_file "$TAG" 'Status: Landed' "$CARD_66" "port feature inventory card must be landed"
guard_expect_fixed_in_file "$TAG" 'Status: Current' "$CARD_67" "provider entrypoint card must be current"
guard_expect_fixed_in_file "$TAG" 'implemented_surface_count=12' "$CARD_66" "card must record implemented surface count"
guard_expect_fixed_in_file "$TAG" 'missing_feature_count=7' "$CARD_66" "card must record missing feature count"
guard_expect_fixed_in_file "$TAG" 'primary_gap_kind=integration_surface_gap' "$CARD_66" "card must classify integration gap"
guard_expect_fixed_in_file "$TAG" 'next_port_feature=real_provider_explicit_entrypoint_selection' "$CARD_66" "card must select provider entrypoint"
guard_expect_fixed_in_file "$TAG" 'ld_preload_shim_ready=0' "$CARD_66" "card must keep LD_PRELOAD later"
guard_expect_fixed_in_file "$TAG" 'provider_entrypoint_selection_ready=1' "$CARD_66" "card must open provider entrypoint selection"
guard_expect_fixed_in_file "$TAG" 'winner_claim=0' "$CARD_66" "card must keep winner closed"

guard_expect_fixed_in_file "$TAG" 'latest_card = "296x-66-HAKO-MIMALLOC-PORT-FEATURE-GAP-INVENTORY"' "$CURRENT_STATE" "current state latest card must advance to row 66"
guard_expect_fixed_in_file "$TAG" 'current_blocker_token = "HAKO-MIMALLOC-PROVIDER-PACKAGE-REAL-ENTRYPOINT-SELECTION-296X-001"' "$CURRENT_STATE" "current state must select row 67"
guard_expect_fixed_in_file "$TAG" '| 66 | `HAKO-MIMALLOC-PORT-FEATURE-GAP-INVENTORY-296X-001` | Landed |' "$TASKBOARD" "taskboard row 66 must be landed"
guard_expect_fixed_in_file "$TAG" '| 67 | `HAKO-MIMALLOC-PROVIDER-PACKAGE-REAL-ENTRYPOINT-SELECTION-296X-001` | Current |' "$TASKBOARD" "taskboard row 67 must be current"
guard_expect_fixed_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list this guard"
guard_expect_fixed_in_file "$TAG" "$TOOL" "$INDEX" "check index must list inventory tool"

tmp_dir="$(mktemp -d /tmp/hakorune_phase296x_feature_inventory.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT
report="$tmp_dir/report.out"
python3 "$TOOL" --repo-root "$ROOT_DIR" --out "$report"

guard_expect_fixed_in_file "$TAG" 'output_contract=hako-mimalloc-port-feature-gap-inventory-v0' "$report" "tool must emit inventory contract"
guard_expect_fixed_in_file "$TAG" 'small_model_checkpoint_elapsed_median_ms=240' "$report" "tool must preserve checkpoint median"
guard_expect_fixed_in_file "$TAG" 'implemented_surface_count=12' "$report" "tool must count implemented surfaces"
guard_expect_fixed_in_file "$TAG" 'missing_feature_count=7' "$report" "tool must count missing features"
guard_expect_fixed_in_file "$TAG" 'primary_gap_kind=integration_surface_gap' "$report" "tool must classify integration gap"
guard_expect_fixed_in_file "$TAG" 'implemented_0_feature=size_class_policy' "$report" "tool must list size class"
guard_expect_fixed_in_file "$TAG" 'implemented_4_feature=production_facade_basic_alloc_realloc_release' "$report" "tool must list production facade"
guard_expect_fixed_in_file "$TAG" 'missing_0_feature=unified_production_allocator_api' "$report" "tool must list unified API gap"
guard_expect_fixed_in_file "$TAG" 'missing_1_feature=real_provider_explicit_entrypoint_selection' "$report" "tool must list provider entrypoint gap"
guard_expect_fixed_in_file "$TAG" 'next_port_feature=real_provider_explicit_entrypoint_selection' "$report" "tool must select provider entrypoint"
guard_expect_fixed_in_file "$TAG" 'ld_preload_shim_ready=0' "$report" "tool must keep LD_PRELOAD later"
guard_expect_fixed_in_file "$TAG" 'provider_entrypoint_selection_ready=1' "$report" "tool must allow provider entrypoint selection"
guard_expect_fixed_in_file "$TAG" 'winner_claim=0' "$report" "tool must keep winner closed"
guard_expect_fixed_in_file "$TAG" 'provider_active=0' "$report" "tool must keep provider inactive"
guard_expect_fixed_in_file "$TAG" 'replacement_active=0' "$report" "tool must keep replacement closed"
guard_expect_fixed_in_file "$TAG" 'hook_installed=0' "$report" "tool must keep hook closed"
guard_expect_fixed_in_file "$TAG" 'global_allocator=0' "$report" "tool must keep global allocator closed"
guard_expect_fixed_in_file "$TAG" 'summary=ok' "$report" "tool must end ok"

echo "[$TAG] ok"
