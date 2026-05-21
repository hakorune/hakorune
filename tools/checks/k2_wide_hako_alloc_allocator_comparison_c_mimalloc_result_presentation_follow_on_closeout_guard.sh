#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-hako-alloc-allocator-comparison-c-mimalloc-result-presentation-follow-on-closeout"
cd "$ROOT_DIR"
source "$ROOT_DIR/tools/checks/lib/pure_first_exe_guard.sh"

if [ "$#" -ne 0 ]; then
  VALIDATION_LEVEL="$(pure_first_guard_parse_level "$TAG" "$@")"
  case "$VALIDATION_LEVEL" in
    L0|L1|L2) ;;
    L3|L4)
      echo "[$TAG] ERROR: MIMAP-482A closeout does not define an L3/L4 pack" >&2
      exit 2
      ;;
  esac
fi

CARD_480A="docs/development/current/main/phases/phase-293x/293x-1110-MIMAP-480A-ALLOCATOR-COMPARISON-C-MIMALLOC-RESULT-PRESENTATION-FOLLOW-ON-PILOT.md"
CARD="docs/development/current/main/phases/phase-293x/293x-1112-MIMAP-482A-ALLOCATOR-COMPARISON-C-MIMALLOC-RESULT-PRESENTATION-FOLLOW-ON-CLOSEOUT.md"
NEXT_CARD="docs/development/current/main/phases/phase-293x/293x-1113-MIMAP-483A-POST-PRESENTATION-FOLLOW-ON-CLOSEOUT-ROW-SELECTION.md"
DESIGN_480A="docs/development/current/main/design/hako-alloc-allocator-comparison-c-mimalloc-result-presentation-follow-on-pilot-ssot.md"
INDEX="docs/tools/check-scripts-index.md"
PROOF_MANIFEST_INCLUDE="tools/checks/manifests/proof_apps/hako_alloc_segment_arena_backing_release_lifecycle.toml"
GUARD_480A="tools/checks/k2_wide_hako_alloc_allocator_comparison_c_mimalloc_result_presentation_follow_on_pilot_guard.sh"
SELF_SCRIPT="tools/checks/k2_wide_hako_alloc_allocator_comparison_c_mimalloc_result_presentation_follow_on_closeout_guard.sh"

printf '[%s] checking MIMAP-482A C mimalloc result presentation follow-on closeout\n' "$TAG"

guard_require_files "$TAG" "$CARD_480A" "$CARD" "$NEXT_CARD" "$DESIGN_480A" "$INDEX" "$PROOF_MANIFEST_INCLUDE" "$GUARD_480A" "$SELF_SCRIPT"
guard_require_exec_files "$TAG" "$GUARD_480A" "$SELF_SCRIPT"

guard_expect_in_file "$TAG" 'Status: landed' "$CARD_480A" "MIMAP-480A must be landed"
guard_expect_in_file "$TAG" 'Status: (selected current|completed|landed)' "$CARD" "MIMAP-482A must be current/completed/landed"
guard_expect_in_file "$TAG" 'Status: (selected current|landed)' "$NEXT_CARD" "MIMAP-483A must be selected current or landed"
guard_expect_in_file "$TAG" 'Decision: accepted' "$DESIGN_480A" "MIMAP-480A design must remain accepted"
guard_expect_fixed_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list MIMAP-482A guard"
guard_expect_in_file "$TAG" 'id = "MIMAP-480A"' "$PROOF_MANIFEST_INCLUDE" "proof manifest must list MIMAP-480A"
guard_expect_in_file "$TAG" 'closeout_pack = "allocator-comparison-c-mimalloc-result-presentation-follow-on"' "$PROOF_MANIFEST_INCLUDE" "presentation follow-on pilot row must keep its closeout pack"
guard_expect_in_file "$TAG" 'Close the presentation follow-on pilot pack after MIMAP-480A' "$CARD" "MIMAP-482A must remain closeout-only"
guard_expect_in_file "$TAG" 'presentation follow-on extension plan' "$NEXT_CARD" "MIMAP-483A must keep the follow-on plan option visible"
guard_expect_in_file "$TAG" 'No repeated or heavy benchmark pack' "$NEXT_CARD" "MIMAP-483A must keep benchmark reruns closed"

bash "$GUARD_480A" --level L2

printf '[%s] ok\n' "$TAG"
