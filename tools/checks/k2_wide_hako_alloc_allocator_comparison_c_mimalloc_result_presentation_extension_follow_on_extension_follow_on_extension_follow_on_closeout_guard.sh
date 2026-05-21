#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-hako-alloc-allocator-comparison-c-mimalloc-result-presentation-extension-follow-on-extension-follow-on-extension-follow-on-closeout"
cd "$ROOT_DIR"
source "$ROOT_DIR/tools/checks/lib/pure_first_exe_guard.sh"

if [ "$#" -ne 0 ]; then
  VALIDATION_LEVEL="$(pure_first_guard_parse_level "$TAG" "$@")"
  case "$VALIDATION_LEVEL" in
    L0|L1|L2) ;;
    L3|L4)
      echo "[$TAG] ERROR: MIMAP-518A closeout does not define an L3/L4 pack" >&2
      exit 2
      ;;
  esac
fi

CARD_516A="docs/development/current/main/phases/phase-293x/293x-1146-MIMAP-516A-ALLOCATOR-COMPARISON-C-MIMALLOC-RESULT-PRESENTATION-EXTENSION-FOLLOW-ON-EXTENSION-FOLLOW-ON-EXTENSION-FOLLOW-ON-PILOT.md"
CARD="docs/development/current/main/phases/phase-293x/293x-1148-MIMAP-518A-ALLOCATOR-COMPARISON-C-MIMALLOC-RESULT-PRESENTATION-EXTENSION-FOLLOW-ON-EXTENSION-FOLLOW-ON-EXTENSION-FOLLOW-ON-CLOSEOUT.md"
NEXT_CARD="docs/development/current/main/phases/phase-293x/293x-1149-MIMAP-519A-POST-PRESENTATION-EXTENSION-FOLLOW-ON-EXTENSION-FOLLOW-ON-EXTENSION-FOLLOW-ON-CLOSEOUT-ROW-SELECTION.md"
DESIGN_516A="docs/development/current/main/design/hako-alloc-allocator-comparison-c-mimalloc-result-presentation-extension-follow-on-extension-follow-on-extension-follow-on-pilot-ssot.md"
INDEX="docs/tools/check-scripts-index.md"
PROOF_MANIFEST_INCLUDE="tools/checks/manifests/proof_apps/hako_alloc_segment_arena_backing_release_lifecycle.toml"
GUARD_516A="tools/checks/k2_wide_hako_alloc_allocator_comparison_c_mimalloc_result_presentation_extension_follow_on_extension_follow_on_extension_follow_on_pilot_guard.sh"
SELF_SCRIPT="tools/checks/k2_wide_hako_alloc_allocator_comparison_c_mimalloc_result_presentation_extension_follow_on_extension_follow_on_extension_follow_on_closeout_guard.sh"

printf '[%s] checking MIMAP-518A C mimalloc result presentation extension follow-on extension follow-on extension follow-on closeout\n' "$TAG"

guard_require_files "$TAG" "$CARD_516A" "$CARD" "$NEXT_CARD" "$DESIGN_516A" "$INDEX" "$PROOF_MANIFEST_INCLUDE" "$GUARD_516A" "$SELF_SCRIPT"
guard_require_exec_files "$TAG" "$GUARD_516A" "$SELF_SCRIPT"

guard_expect_in_file "$TAG" 'Status: landed' "$CARD_516A" "MIMAP-516A must be landed"
guard_expect_in_file "$TAG" 'Status: (selected current|completed|landed)' "$CARD" "MIMAP-518A must be current/completed/landed"
guard_expect_in_file "$TAG" 'Status: (selected current|landed)' "$NEXT_CARD" "MIMAP-519A must be selected current or landed"
guard_expect_in_file "$TAG" 'Decision: accepted' "$DESIGN_516A" "MIMAP-516A design must remain accepted"
guard_expect_fixed_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list MIMAP-518A guard"
guard_expect_in_file "$TAG" 'id = "MIMAP-516A"' "$PROOF_MANIFEST_INCLUDE" "proof manifest must list MIMAP-516A"
guard_expect_in_file "$TAG" 'closeout_pack = "allocator-comparison-c-mimalloc-result-presentation-extension-follow-on-extension-follow-on-extension-follow-on"' "$PROOF_MANIFEST_INCLUDE" "presentation extension follow-on extension follow-on extension follow-on pilot row must keep its closeout pack"
guard_expect_in_file "$TAG" 'Close the presentation extension follow-on extension follow-on extension follow-on pilot pack after MIMAP-516A.' "$CARD" "MIMAP-518A must remain closeout-only"
guard_expect_in_file "$TAG" 'presentation extension follow-on extension follow-on extension follow-on plan' "$NEXT_CARD" "MIMAP-519A must keep the extension plan option visible"
guard_expect_in_file "$TAG" 'No repeated or heavy benchmark pack' "$NEXT_CARD" "MIMAP-519A must keep benchmark reruns closed"

bash "$GUARD_516A" --level L2

printf '[%s] ok\n' "$TAG"
