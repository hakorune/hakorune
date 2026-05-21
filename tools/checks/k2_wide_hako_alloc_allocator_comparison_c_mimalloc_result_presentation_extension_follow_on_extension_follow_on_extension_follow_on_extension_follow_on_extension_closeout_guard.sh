#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-hako-alloc-allocator-comparison-c-mimalloc-result-presentation-extension-follow-on-extension-follow-on-extension-follow-on-extension-follow-on-extension-closeout"
cd "$ROOT_DIR"
source "$ROOT_DIR/tools/checks/lib/pure_first_exe_guard.sh"

if [ "$#" -ne 0 ]; then
  VALIDATION_LEVEL="$(pure_first_guard_parse_level "$TAG" "$@")"
  case "$VALIDATION_LEVEL" in
    L0|L1|L2) ;;
    L3|L4)
      echo "[$TAG] ERROR: MIMAP-536A closeout does not define an L3/L4 pack" >&2
      exit 2
      ;;
  esac
fi

CARD_534A="docs/development/current/main/phases/phase-293x/293x-1164-MIMAP-534A-ALLOCATOR-COMPARISON-C-MIMALLOC-RESULT-PRESENTATION-EXTENSION-FOLLOW-ON-EXTENSION-FOLLOW-ON-EXTENSION-FOLLOW-ON-EXTENSION-FOLLOW-ON-EXTENSION-PILOT.md"
CARD="docs/development/current/main/phases/phase-293x/293x-1166-MIMAP-536A-ALLOCATOR-COMPARISON-C-MIMALLOC-RESULT-PRESENTATION-EXTENSION-FOLLOW-ON-EXTENSION-FOLLOW-ON-EXTENSION-FOLLOW-ON-EXTENSION-FOLLOW-ON-EXTENSION-CLOSEOUT.md"
NEXT_CARD="docs/development/current/main/phases/phase-293x/293x-1167-MIMAP-537A-POST-PRESENTATION-EXTENSION-FOLLOW-ON-EXTENSION-FOLLOW-ON-EXTENSION-FOLLOW-ON-EXTENSION-FOLLOW-ON-EXTENSION-CLOSEOUT-ROW-SELECTION.md"
DESIGN_534A="docs/development/current/main/design/hako-alloc-allocator-comparison-c-mimalloc-result-presentation-extension-follow-on-extension-follow-on-extension-follow-on-extension-follow-on-extension-pilot-ssot.md"
INDEX="docs/tools/check-scripts-index.md"
PROOF_MANIFEST_INCLUDE="tools/checks/manifests/proof_apps/hako_alloc_segment_arena_backing_release_lifecycle.toml"
GUARD_534A="tools/checks/k2_wide_hako_alloc_allocator_comparison_c_mimalloc_result_presentation_extension_follow_on_extension_follow_on_extension_follow_on_extension_follow_on_extension_pilot_guard.sh"
SELF_SCRIPT="tools/checks/k2_wide_hako_alloc_allocator_comparison_c_mimalloc_result_presentation_extension_follow_on_extension_follow_on_extension_follow_on_extension_follow_on_extension_closeout_guard.sh"

printf '[%s] checking MIMAP-536A C mimalloc result presentation extension follow-on extension follow-on extension follow-on extension follow-on extension closeout\n' "$TAG"

guard_require_files "$TAG" "$CARD_534A" "$CARD" "$NEXT_CARD" "$DESIGN_534A" "$INDEX" "$PROOF_MANIFEST_INCLUDE" "$GUARD_534A" "$SELF_SCRIPT"
guard_require_exec_files "$TAG" "$GUARD_534A" "$SELF_SCRIPT"

guard_expect_in_file "$TAG" 'Status: landed' "$CARD_534A" "MIMAP-534A must be landed"
guard_expect_in_file "$TAG" 'Status: (selected current|completed|landed)' "$CARD" "MIMAP-536A must be current/completed/landed"
guard_expect_in_file "$TAG" 'Status: (selected current|landed)' "$NEXT_CARD" "MIMAP-537A must be selected current or landed"
guard_expect_in_file "$TAG" 'Decision: accepted' "$DESIGN_534A" "MIMAP-534A design must remain accepted"
guard_expect_fixed_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list MIMAP-536A guard"
guard_expect_in_file "$TAG" 'id = "MIMAP-534A"' "$PROOF_MANIFEST_INCLUDE" "proof manifest must list MIMAP-534A"
guard_expect_in_file "$TAG" 'closeout_pack = "allocator-comparison-c-mimalloc-result-presentation-extension-follow-on-extension-follow-on-extension-follow-on-extension-follow-on-extension"' "$PROOF_MANIFEST_INCLUDE" "presentation extension follow-on extension follow-on extension follow-on extension follow-on extension pilot row must keep its closeout pack"
guard_expect_in_file "$TAG" 'Close the presentation extension follow-on extension follow-on extension follow-on extension follow-on extension pilot pack after MIMAP-534A.' "$CARD" "MIMAP-536A must remain closeout-only"
guard_expect_in_file "$TAG" 'presentation extension follow-on extension follow-on extension follow-on extension follow-on extension follow-on plan' "$NEXT_CARD" "MIMAP-537A must keep the follow-on plan option visible"
guard_expect_in_file "$TAG" 'No repeated or heavy benchmark pack' "$NEXT_CARD" "MIMAP-537A must keep benchmark reruns closed"

bash "$GUARD_534A" --level L2

printf '[%s] ok\n' "$TAG"
