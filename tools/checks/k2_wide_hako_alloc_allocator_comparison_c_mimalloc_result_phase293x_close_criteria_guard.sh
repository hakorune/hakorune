#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-hako-alloc-allocator-comparison-c-mimalloc-result-phase293x-close-criteria"
cd "$ROOT_DIR"
source "$ROOT_DIR/tools/checks/lib/pure_first_exe_guard.sh"

if [ "$#" -ne 0 ]; then
  VALIDATION_LEVEL="$(pure_first_guard_parse_level "$TAG" "$@")"
  case "$VALIDATION_LEVEL" in
    L0|L1|L2) ;;
    L3|L4)
      echo "[$TAG] ERROR: MIMAP-567A close-criteria row does not define L3/L4 packs" >&2
      exit 2
      ;;
  esac
fi

CARD_566A="docs/development/current/main/phases/phase-293x/293x-1196-MIMAP-566A-ALLOCATOR-COMPARISON-C-MIMALLOC-RESULT-EXPLICIT-RUNNER-PLANNING-PILOT.md"
CARD="docs/development/current/main/phases/phase-293x/293x-1197-MIMAP-567A-MIMALLOC-BLUEPRINT-LANE-CLOSE-CRITERIA.md"
NEXT_CARD="docs/development/current/main/phases/phase-293x/293x-1198-MIMAP-568A-MIMALLOC-BLUEPRINT-LANE-INVENTORY-CARRYOVER-BOUNDARY.md"
DESIGN_CLOSE="docs/development/current/main/design/mimalloc-blueprint-lane-close-criteria-ssot.md"
INDEX="docs/tools/check-scripts-index.md"
PREV_GUARD="tools/checks/k2_wide_hako_alloc_allocator_comparison_c_mimalloc_result_explicit_runner_planning_pilot_guard.sh"
SELF_SCRIPT="tools/checks/k2_wide_hako_alloc_allocator_comparison_c_mimalloc_result_phase293x_close_criteria_guard.sh"

printf '[%s] checking MIMAP-567A mimalloc blueprint lane close criteria\n' "$TAG"

guard_require_files "$TAG" "$CARD_566A" "$CARD" "$NEXT_CARD" "$DESIGN_CLOSE" "$INDEX" "$PREV_GUARD" "$SELF_SCRIPT"
guard_require_exec_files "$TAG" "$PREV_GUARD" "$SELF_SCRIPT"

guard_expect_in_file "$TAG" 'Status: landed' "$CARD_566A" "MIMAP-566A must be landed"
guard_expect_in_file "$TAG" 'Status: (selected current|completed|landed)' "$CARD" "MIMAP-567A must be current/completed/landed"
guard_expect_in_file "$TAG" 'Status: (selected current|completed|landed)' "$NEXT_CARD" "MIMAP-568A must be selected current/completed/landed"
guard_expect_in_file "$TAG" 'Decision: accepted' "$DESIGN_CLOSE" "close-criteria SSOT must remain accepted"
guard_expect_in_file "$TAG" '`MIMAP-566A`' "$DESIGN_CLOSE" "close-criteria SSOT must keep terminal sequence"
guard_expect_in_file "$TAG" '`MIMAP-569A`' "$DESIGN_CLOSE" "close-criteria SSOT must include closeout row"
guard_expect_fixed_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list MIMAP-567A guard"
guard_expect_in_file "$TAG" 'A/B' "$CARD" "MIMAP-567A must lock A/B closure scope"
guard_expect_in_file "$TAG" 'MIMAP-568A Mimalloc Blueprint Lane Inventory Carryover Boundary' "$CARD" "MIMAP-567A must select MIMAP-568A"

bash "$PREV_GUARD" --level L2

printf '[%s] ok\n' "$TAG"
