#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-hako-alloc-provider-facing-ladder-closed-plan"
cd "$ROOT_DIR"
source "$ROOT_DIR/tools/checks/lib/pure_first_exe_guard.sh"

CARD_356A="docs/development/current/main/phases/phase-293x/293x-972-MIMAP-356A-EXECUTION-SEAM-SUMMARY-CLOSEOUT.md"
CARD_357A="docs/development/current/main/phases/phase-293x/293x-973-MIMAP-357A-POST-EXECUTION-SEAM-SUMMARY-ROW-SELECTION.md"
CARD="docs/development/current/main/phases/phase-293x/293x-974-MIMAP-358A-PROVIDER-FACING-LADDER-CLOSED-PLAN.md"
NEXT_CARD="docs/development/current/main/phases/phase-293x/293x-975-MIMAP-359A-POST-PROVIDER-FACING-LADDER-PLAN-ROW-SELECTION.md"
DESIGN="docs/development/current/main/design/hako-alloc-provider-facing-ladder-closed-plan-ssot.md"
INDEX="docs/tools/check-scripts-index.md"
SUMMARY_GUARD="tools/checks/k2_wide_hako_alloc_execution_seam_summary_closeout_guard.sh"
SELF_SCRIPT="tools/checks/k2_wide_hako_alloc_provider_facing_ladder_closed_plan_guard.sh"

printf '[%s] checking MIMAP-358A provider-facing ladder closed plan\n' "$TAG"

guard_require_files "$TAG" "$CARD_356A" "$CARD_357A" "$CARD" "$NEXT_CARD" "$DESIGN" "$INDEX" "$SUMMARY_GUARD" "$SELF_SCRIPT"
guard_require_exec_files "$TAG" "$SUMMARY_GUARD" "$SELF_SCRIPT"

guard_expect_in_file "$TAG" 'Status: landed' "$CARD_356A" "MIMAP-356A execution seam summary must be landed"
guard_expect_in_file "$TAG" 'Status: landed' "$CARD_357A" "MIMAP-357A row-selection card must be landed"
guard_expect_in_file "$TAG" 'Status: landed' "$CARD" "MIMAP-358A card must be landed"
guard_expect_in_file "$TAG" 'Status: selected current' "$NEXT_CARD" "MIMAP-359A must be selected current"
guard_expect_in_file "$TAG" 'Decision: accepted' "$DESIGN" "MIMAP-358A design must be accepted"
guard_expect_in_file "$TAG" 'provider boundary diagnostic vocabulary inventory' "$DESIGN" "provider ladder must start with diagnostic vocabulary inventory"
guard_expect_in_file "$TAG" 'provider activation first-pattern row, only after explicit selection' "$DESIGN" "provider activation must require explicit first-pattern selection"
guard_expect_in_file "$TAG" 'host allocator replacement / hooks /' "$DESIGN" "host replacement and hooks must remain separate ladders"
guard_expect_in_file "$TAG" 'optional ladders' "$DESIGN" "host replacement and hooks must remain separate ladders"
guard_expect_fixed_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list MIMAP-358A guard"

bash "$SUMMARY_GUARD"

printf '[%s] ok\n' "$TAG"
