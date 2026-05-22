#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-hako-alloc-allocator-comparison-c-mimalloc-result-phase293x-inventory-carryover"
cd "$ROOT_DIR"
source "$ROOT_DIR/tools/checks/lib/pure_first_exe_guard.sh"

if [ "$#" -ne 0 ]; then
  VALIDATION_LEVEL="$(pure_first_guard_parse_level "$TAG" "$@")"
  case "$VALIDATION_LEVEL" in
    L0|L1|L2) ;;
    L3|L4)
      echo "[$TAG] ERROR: MIMAP-568A inventory row does not define L3/L4 packs" >&2
      exit 2
      ;;
  esac
fi

CARD_567A="docs/development/current/main/phases/phase-293x/293x-1197-MIMAP-567A-MIMALLOC-BLUEPRINT-LANE-CLOSE-CRITERIA.md"
CARD="docs/development/current/main/phases/phase-293x/293x-1198-MIMAP-568A-MIMALLOC-BLUEPRINT-LANE-INVENTORY-CARRYOVER-BOUNDARY.md"
NEXT_CARD="docs/development/current/main/phases/phase-293x/293x-1199-MIMAP-569A-PHASE-293X-MIMALLOC-BLUEPRINT-LANE-CLOSEOUT.md"
INDEX="docs/tools/check-scripts-index.md"
PREV_GUARD="tools/checks/k2_wide_hako_alloc_allocator_comparison_c_mimalloc_result_phase293x_close_criteria_guard.sh"
SELF_SCRIPT="tools/checks/k2_wide_hako_alloc_allocator_comparison_c_mimalloc_result_phase293x_inventory_carryover_guard.sh"

printf '[%s] checking MIMAP-568A mimalloc blueprint lane inventory carryover boundary\n' "$TAG"

guard_require_files "$TAG" "$CARD_567A" "$CARD" "$NEXT_CARD" "$INDEX" "$PREV_GUARD" "$SELF_SCRIPT"
guard_require_exec_files "$TAG" "$PREV_GUARD" "$SELF_SCRIPT"

guard_expect_in_file "$TAG" 'Status: (completed|landed)' "$CARD_567A" "MIMAP-567A must be completed/landed"
guard_expect_in_file "$TAG" 'Status: (selected current|completed|landed)' "$CARD" "MIMAP-568A must be current/completed/landed"
guard_expect_in_file "$TAG" 'Status: (selected current|completed|landed)' "$NEXT_CARD" "MIMAP-569A must be selected current/completed/landed"
guard_expect_fixed_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list MIMAP-568A guard"
guard_expect_in_file "$TAG" 'taskboard unique MIMAP rows' "$CARD" "MIMAP-568A must publish taskboard row count"
guard_expect_in_file "$TAG" '`lang/src/hako_alloc/\*\*/\*\.hako`' "$CARD" "MIMAP-568A must publish hako inventory count"
guard_expect_in_file "$TAG" '`apps/hako-alloc-\*` with `main.hako`' "$CARD" "MIMAP-568A must publish proof app count"
guard_expect_in_file "$TAG" 'Record syntax expansion' "$CARD" "MIMAP-568A must defer record syntax expansion"
guard_expect_in_file "$TAG" 'MIMAP-569A Phase-293x Mimalloc Blueprint Lane Closeout' "$CARD" "MIMAP-568A must select MIMAP-569A"

bash "$PREV_GUARD" --level L2

printf '[%s] ok\n' "$TAG"
