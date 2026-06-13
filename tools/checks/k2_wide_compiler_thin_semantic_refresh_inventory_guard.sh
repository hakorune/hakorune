#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-compiler-thin-semantic-refresh-inventory"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

SSOT="docs/development/current/main/design/compiler-pipeline-thinning-ssot.md"
README="tools/hako_check/README.md"
INDEX="docs/tools/check-scripts-index.md"
WRAPPER="tools/hako_check.sh"
TOOL="tools/hako_check/semantic_refresh_inventory.py"
SELF_SCRIPT="tools/checks/k2_wide_compiler_thin_semantic_refresh_inventory_guard.sh"

echo "[$TAG] checking semantic refresh inventory surface"

guard_require_files "$TAG" "$SSOT" "$README" "$INDEX" "$WRAPPER" "$TOOL" "$SELF_SCRIPT"
guard_require_exec_files "$TAG" "$TOOL" "$SELF_SCRIPT"

guard_expect_fixed_in_file "$TAG" 'output_contract=hako-check-semantic-refresh-inventory-v0' "$SSOT" "SSOT must record inventory contract"
guard_expect_fixed_in_file "$TAG" 'json_v0_post_canonicalize_metadata_subset' "$SSOT" "SSOT must name JSON v0 bridge seam"
guard_expect_fixed_in_file "$TAG" 'hako_check semantic-refresh-inventory' "$README" "README must document semantic refresh inventory"
guard_expect_fixed_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list this guard"
guard_expect_fixed_in_file "$TAG" 'tools/hako_check.sh semantic-refresh-inventory' "$INDEX" "check index must list wrapper entry"
guard_expect_fixed_in_file "$TAG" "$TOOL" "$INDEX" "check index must list inventory tool"

tmp_dir="$(mktemp -d /tmp/hakorune_semantic_refresh_inventory.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT
report="$tmp_dir/report.kv"

bash "$WRAPPER" semantic-refresh-inventory --out "$report"

guard_expect_fixed_in_file "$TAG" 'output_contract=hako-check-semantic-refresh-inventory-v0' "$report" "tool must emit contract"
guard_expect_fixed_in_file "$TAG" 'tool_surface=hako_check_semantic_refresh_inventory' "$report" "tool must name surface"
guard_expect_fixed_in_file "$TAG" 'observation_only=1' "$report" "tool must be observation only"
guard_expect_fixed_in_file "$TAG" 'rewrite_executed=0' "$report" "tool must not rewrite"
guard_expect_fixed_in_file "$TAG" 'semantic_refresh_truth_source=src/mir/semantic_refresh.rs' "$report" "tool must cite semantic refresh truth source"
guard_expect_fixed_in_file "$TAG" 'semantic_refresh_remaining_duplicate_candidate_count=0' "$report" "tool must report no remaining duplicate candidates"
guard_expect_fixed_in_file "$TAG" 'semantic_refresh_resolved_helper_count=1' "$report" "tool must report one resolved helper"
guard_expect_fixed_in_file "$TAG" 'semantic_refresh_inventory[5].id=json_v0_post_canonicalize_metadata_subset' "$report" "tool must name JSON v0 candidate"
guard_expect_fixed_in_file "$TAG" 'semantic_refresh_inventory[5].kind=resolved_helper' "$report" "tool must mark JSON v0 seam resolved"
guard_expect_fixed_in_file "$TAG" 'semantic_refresh_behavior_changed=0' "$report" "tool must not imply behavior change"
guard_expect_fixed_in_file "$TAG" 'semantic_refresh_order_changed=0' "$report" "tool must not imply ordering change"
guard_expect_fixed_in_file "$TAG" 'summary=ok' "$report" "tool must end ok"

echo "[$TAG] ok"
