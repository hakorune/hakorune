#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-mimap022c-next-row-selection"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh
source tools/checks/lib/phase_card_paths.sh

STATE="docs/development/current/main/CURRENT_STATE.toml"
CURRENT_TASK="CURRENT_TASK.md"
RESUME="docs/development/current/main/05-Restart-Quick-Resume.md"
NOW="docs/development/current/main/10-Now.md"
PHASE_README="docs/development/current/main/phases/phase-293x/README.md"
TASKBOARD="docs/development/current/main/phases/phase-293x/293x-mimalloc-port-taskboard.md"
GRANULARITY="docs/development/current/main/design/mimalloc-allocator-first-task-granularity-ssot.md"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_mimap022c_next_row_selection_guard.sh"

SELECTION_CARD="$(guard_require_phase293x_card "$TAG" "293x-435-MIMAP-022C-POST-HUGE-FAILFAST-ROW-SELECTION.md")"
NEXT_CARD="$(guard_require_phase293x_card "$TAG" "293x-436-MIMAP-023A-FACADE-HUGE-PAGE-MODEL-ROUTE.md")"

echo "[$TAG] checking MIMAP-022C next-row selection"

guard_require_files \
  "$TAG" \
  "$STATE" \
  "$CURRENT_TASK" \
  "$RESUME" \
  "$NOW" \
  "$PHASE_README" \
  "$TASKBOARD" \
  "$GRANULARITY" \
  "$INDEX" \
  "$SELF_SCRIPT" \
  "$SELECTION_CARD" \
  "$NEXT_CARD"
guard_require_exec_files "$TAG" "$SELF_SCRIPT"

guard_expect_in_file "$TAG" 'Status: landed' "$SELECTION_CARD" "MIMAP-022C selection card must be landed"
guard_expect_in_file "$TAG" 'MIMAP-023A facade huge-page model route' "$SELECTION_CARD" "MIMAP-022C must select MIMAP-023A"
guard_expect_in_file "$TAG" 'object_lifecycle_facade_huge_page_model_box\.hako' "$SELECTION_CARD" "MIMAP-022C must name the next owner"
guard_expect_in_file "$TAG" 'tools/checks/k2_wide_mimalloc_facade_huge_page_model_exe_guard\.sh' "$SELECTION_CARD" "MIMAP-022C must name the next guard"
guard_expect_in_file "$TAG" 'Do not implement allocator behavior in this row' "$SELECTION_CARD" "MIMAP-022C must remain planning-only"

guard_expect_in_file "$TAG" 'Status: landed' "$NEXT_CARD" "MIMAP-023A card must be landed after implementation"
guard_expect_in_file "$TAG" 'lang/src/hako_alloc/memory/object_lifecycle_facade_huge_page_model_box\.hako' "$NEXT_CARD" "MIMAP-023A must name the planned owner"
guard_expect_in_file "$TAG" 'apps/mimalloc-facade-huge-page-model-proof/main\.hako' "$NEXT_CARD" "MIMAP-023A must name the proof app"
guard_expect_in_file "$TAG" 'tools/checks/k2_wide_mimalloc_facade_huge_page_model_exe_guard\.sh' "$NEXT_CARD" "MIMAP-023A must name the first behavior guard"
guard_expect_in_file "$TAG" 'No new huge page model owner; reuse the existing M180 owner' "$NEXT_CARD" "MIMAP-023A must reuse the existing huge model"
guard_expect_in_file "$TAG" 'No huge release, unregister, unreserve, decommit, or OS release behavior' "$NEXT_CARD" "MIMAP-023A must stop before huge release/unreserve/decommit"
guard_expect_in_file "$TAG" 'No page-map lookup route' "$NEXT_CARD" "MIMAP-023A must stop before page-map lookup"
guard_expect_in_file "$TAG" 'No provider hooks, host allocator replacement, or `#\[global_allocator\]`' "$NEXT_CARD" "MIMAP-023A must keep provider/replacement inactive"
guard_expect_in_file "$TAG" 'No backend `\.inc` matcher shortcut' "$NEXT_CARD" "MIMAP-023A must forbid .inc matcher shortcuts"

guard_expect_in_file "$TAG" 'phase_status = "docs/development/current/main/phases/phase-293x/293x-437-MIMAP-023B-POST-HUGE-PAGE-MODEL-ROW-SELECTION.md"' "$STATE" "CURRENT_STATE must move phase_status after MIMAP-023A lands"
guard_expect_in_file "$TAG" 'current_blocker_token = "MIMAP-023B"' "$STATE" "CURRENT_STATE must set MIMAP-023B as blocker after MIMAP-023A lands"
guard_expect_in_file "$TAG" 'latest_card = "293x-436-MIMAP-023A-FACADE-HUGE-PAGE-MODEL-ROUTE"' "$STATE" "CURRENT_STATE latest card must close MIMAP-023A"
guard_expect_in_file "$TAG" 'MIMAP-023B post-huge-page-model row selection' "$CURRENT_TASK" "CURRENT_TASK must restart on MIMAP-023B"
guard_expect_in_file "$TAG" 'MIMAP-023B post-huge-page-model row selection' "$RESUME" "quick resume must restart on MIMAP-023B"
guard_expect_in_file "$TAG" 'MIMAP-023B post-huge-page-model row selection' "$NOW" "Now mirror must restart on MIMAP-023B"
guard_expect_in_file "$TAG" '293x-437-MIMAP-023B-POST-HUGE-PAGE-MODEL-ROW-SELECTION.md' "$PHASE_README" "phase README must point at MIMAP-023B as current blocker"

guard_expect_in_file "$TAG" '| `MIMAP-022C` | landed | Post-huge-failfast allocator row selection. | after MIMAP-022B |' "$TASKBOARD" "taskboard must close MIMAP-022C"
guard_expect_in_file "$TAG" '| `MIMAP-023A` | landed | Facade huge-page model route using the existing M180 huge-page model owner. | after MIMAP-022C |' "$TASKBOARD" "taskboard must close MIMAP-023A"
guard_expect_in_file "$TAG" '| `MIMAP-023B` | ready current | Post-huge-page-model allocator row selection. | current |' "$TASKBOARD" "taskboard must mark MIMAP-023B current"
guard_expect_in_file "$TAG" '`MIMAP-023A` | facade huge-page model route' "$GRANULARITY" "granularity SSOT must list MIMAP-023A follow-up"
guard_expect_in_file "$TAG" '`MIMAP-023B` | post-huge-page-model allocator row selection' "$GRANULARITY" "granularity SSOT must list MIMAP-023B follow-up"
guard_expect_in_file "$TAG" '### MIMAP-023A granularity' "$GRANULARITY" "granularity SSOT must define MIMAP-023A"
guard_expect_in_file "$TAG" 'huge request -> existing HakoAllocHugePageModel allocation' "$GRANULARITY" "granularity SSOT must pin the huge model behavior"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list MIMAP-022C selection guard"

echo "[$TAG] ok"
