#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-hako-alloc-allocator-comparison-c-mimalloc-result-first-conclusion-closeout"
cd "$ROOT_DIR"
source "$ROOT_DIR/tools/checks/lib/pure_first_exe_guard.sh"

if [ "$#" -ne 0 ]; then
  VALIDATION_LEVEL="$(pure_first_guard_parse_level "$TAG" "$@")"
  case "$VALIDATION_LEVEL" in
    L0|L1|L2) ;;
    L3|L4)
      echo "[$TAG] ERROR: MIMAP-470A closeout does not define an L3/L4 pack" >&2
      exit 2
      ;;
  esac
fi

CARD_468A="docs/development/current/main/phases/phase-293x/293x-1098-MIMAP-468A-ALLOCATOR-COMPARISON-C-MIMALLOC-RESULT-FIRST-CONCLUSION-PILOT.md"
CARD="docs/development/current/main/phases/phase-293x/293x-1100-MIMAP-470A-ALLOCATOR-COMPARISON-C-MIMALLOC-RESULT-FIRST-CONCLUSION-CLOSEOUT.md"
NEXT_CARD="docs/development/current/main/phases/phase-293x/293x-1101-MIMAP-471A-POST-FIRST-CONCLUSION-CLOSEOUT-ROW-SELECTION.md"
DESIGN_468A="docs/development/current/main/design/hako-alloc-allocator-comparison-c-mimalloc-result-first-conclusion-pilot-ssot.md"
INDEX="docs/tools/check-scripts-index.md"
PROOF_MANIFEST_INCLUDE="tools/checks/manifests/proof_apps/hako_alloc_segment_arena_backing_release_lifecycle.toml"
GUARD_468A="tools/checks/k2_wide_hako_alloc_allocator_comparison_c_mimalloc_result_first_conclusion_pilot_guard.sh"
SELF_SCRIPT="tools/checks/k2_wide_hako_alloc_allocator_comparison_c_mimalloc_result_first_conclusion_closeout_guard.sh"

printf '[%s] checking MIMAP-470A C mimalloc result first conclusion closeout\n' "$TAG"

guard_require_files "$TAG" "$CARD_468A" "$CARD" "$NEXT_CARD" "$DESIGN_468A" "$INDEX" "$PROOF_MANIFEST_INCLUDE" "$GUARD_468A" "$SELF_SCRIPT"
guard_require_exec_files "$TAG" "$GUARD_468A" "$SELF_SCRIPT"

guard_expect_in_file "$TAG" 'Status: landed' "$CARD_468A" "MIMAP-468A must be landed"
guard_expect_in_file "$TAG" 'Status: (selected current|completed|landed)' "$CARD" "MIMAP-470A must be current/completed/landed"
guard_expect_in_file "$TAG" 'Status: (selected current|landed)' "$NEXT_CARD" "MIMAP-471A must be selected current or landed"
guard_expect_in_file "$TAG" 'Decision: accepted' "$DESIGN_468A" "MIMAP-468A design must remain accepted"
guard_expect_fixed_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list MIMAP-470A guard"
guard_expect_in_file "$TAG" 'id = "MIMAP-468A"' "$PROOF_MANIFEST_INCLUDE" "proof manifest must list MIMAP-468A"
guard_expect_in_file "$TAG" 'closeout_pack = "allocator-comparison-c-mimalloc-result-first-conclusion-pilot"' "$PROOF_MANIFEST_INCLUDE" "first conclusion pilot row must keep its closeout pack"
guard_expect_in_file "$TAG" 'Close the first conclusion pilot pack after MIMAP-468A' "$CARD" "MIMAP-470A must remain closeout-only"
guard_expect_in_file "$TAG" 'presentation-only' "$NEXT_CARD" "MIMAP-471A must keep presentation option visible"
guard_expect_in_file "$TAG" 'No repeated or heavy benchmark pack' "$NEXT_CARD" "MIMAP-471A must keep benchmark reruns closed"

bash "$GUARD_468A" --level L2

printf '[%s] ok\n' "$TAG"
