#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-mimap022a-next-row-selection"
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
PURPOSE="docs/development/current/main/design/mimalloc-hako-port-purpose-ssot.md"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_mimap022a_next_row_selection_guard.sh"

SELECTION_CARD="$(guard_require_phase293x_card "$TAG" "293x-404-MIMAP-022A-POST-LIFECYCLE-ROW-SELECTION.md")"
NEXT_CARD="$(guard_require_phase293x_card "$TAG" "293x-434-MIMAP-022B-FACADE-HUGE-REQUEST-FAILFAST-ROUTING.md")"

echo "[$TAG] checking MIMAP-022A next-row selection"

guard_require_files \
  "$TAG" \
  "$STATE" \
  "$CURRENT_TASK" \
  "$RESUME" \
  "$NOW" \
  "$PHASE_README" \
  "$TASKBOARD" \
  "$GRANULARITY" \
  "$PURPOSE" \
  "$INDEX" \
  "$SELF_SCRIPT" \
  "$SELECTION_CARD" \
  "$NEXT_CARD"
guard_require_exec_files "$TAG" "$SELF_SCRIPT"

guard_expect_in_file "$TAG" 'Status: landed' "$SELECTION_CARD" "MIMAP-022A selection card must be landed"
guard_expect_in_file "$TAG" 'MIMAP-022B facade huge-request fail-fast routing' "$SELECTION_CARD" "MIMAP-022A must select MIMAP-022B"
guard_expect_in_file "$TAG" "$NEXT_CARD" "$SELECTION_CARD" "MIMAP-022A must point to the selected next card"
guard_expect_in_file "$TAG" 'Do not implement allocator behavior in this row' "$SELECTION_CARD" "MIMAP-022A must remain planning-only"

guard_expect_in_file "$TAG" 'Status: ready' "$NEXT_CARD" "MIMAP-022B card must be ready"
guard_expect_in_file "$TAG" 'lang/src/hako_alloc/memory/object_lifecycle_facade_huge_failfast_box.hako' "$NEXT_CARD" "MIMAP-022B must name the planned owner"
guard_expect_in_file "$TAG" 'tools/checks/k2_wide_mimalloc_facade_huge_failfast_exe_guard.sh' "$NEXT_CARD" "MIMAP-022B must name the first behavior guard"
guard_expect_in_file "$TAG" 'No huge page model' "$NEXT_CARD" "MIMAP-022B must stop before huge page model"
guard_expect_in_file "$TAG" 'No provider hooks, host allocator replacement, or `#\[global_allocator\]`' "$NEXT_CARD" "MIMAP-022B must keep provider/replacement inactive"

guard_expect_in_file "$TAG" 'phase_status = "docs/development/current/main/phases/phase-293x/293x-434-MIMAP-022B-FACADE-HUGE-REQUEST-FAILFAST-ROUTING.md"' "$STATE" "CURRENT_STATE must point phase_status at MIMAP-022B"
guard_expect_in_file "$TAG" 'current_blocker_token = "MIMAP-022B"' "$STATE" "CURRENT_STATE must set MIMAP-022B as blocker"
guard_expect_in_file "$TAG" 'latest_card = "293x-404-MIMAP-022A-POST-LIFECYCLE-ROW-SELECTION"' "$STATE" "CURRENT_STATE latest card must close MIMAP-022A"
guard_expect_in_file "$TAG" 'MIMAP-022B facade huge-request fail-fast routing' "$CURRENT_TASK" "CURRENT_TASK must restart on MIMAP-022B"
guard_expect_in_file "$TAG" 'MIMAP-022B facade huge-request fail-fast routing' "$RESUME" "quick resume must restart on MIMAP-022B"
guard_expect_in_file "$TAG" 'MIMAP-022B facade huge-request fail-fast routing' "$NOW" "Now mirror must restart on MIMAP-022B"
guard_expect_in_file "$TAG" "$NEXT_CARD" "$PHASE_README" "phase README must point at MIMAP-022B as current blocker"

guard_expect_in_file "$TAG" '| `MIMAP-022A` | landed | Post-lifecycle allocator row selection. | after REUSE-LIFECYCLE-001 |' "$TASKBOARD" "taskboard must close MIMAP-022A"
guard_expect_in_file "$TAG" '| `MIMAP-022B` | ready current | Facade huge-request fail-fast routing before page-source attach/retry. | current |' "$TASKBOARD" "taskboard must mark MIMAP-022B current"
guard_expect_in_file "$TAG" '### MIMAP-022A / MIMAP-022B granularity' "$GRANULARITY" "granularity SSOT must define MIMAP-022A/B"
guard_expect_in_file "$TAG" 'huge request -> fail-fast scalar report' "$GRANULARITY" "granularity SSOT must pin the MIMAP-022B behavior"
guard_expect_in_file "$TAG" 'The next row is `M179 huge threshold and routing`' "$PURPOSE" "purpose SSOT must retain the historical M179 reading"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list MIMAP-022A selection guard"

if rg -n 'provider hooks|host allocator replacement|#\[global_allocator\]' "$NEXT_CARD" >/tmp/"$TAG".stop_lines 2>&1; then
  :
else
  guard_fail "$TAG" "MIMAP-022B must mention provider/replacement/global allocator stop lines"
fi
rm -f /tmp/"$TAG".stop_lines

echo "[$TAG] ok"
