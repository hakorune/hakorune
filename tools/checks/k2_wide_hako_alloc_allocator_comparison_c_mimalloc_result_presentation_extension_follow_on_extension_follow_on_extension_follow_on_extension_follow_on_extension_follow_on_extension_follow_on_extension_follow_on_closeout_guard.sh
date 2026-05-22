#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-hako-alloc-allocator-comparison-c-mimalloc-result-presentation-extension-follow-on-extension-follow-on-extension-follow-on-extension-follow-on-extension-follow-on-extension-follow-on-extension-follow-on-closeout"
cd "$ROOT_DIR"
source "$ROOT_DIR/tools/checks/lib/pure_first_exe_guard.sh"

if [ "$#" -ne 0 ]; then
  VALIDATION_LEVEL="$(pure_first_guard_parse_level "$TAG" "$@")"
  case "$VALIDATION_LEVEL" in
    L0|L1|L2) ;;
    L3|L4)
      echo "[$TAG] ERROR: MIMAP-554A closeout does not define an L3/L4 pack" >&2
      exit 2
      ;;
  esac
fi

CARD_552A="docs/development/current/main/phases/phase-293x/293x-1182-MIMAP-552A-ALLOCATOR-COMPARISON-C-MIMALLOC-RESULT-PRESENTATION-EXTENSION-FOLLOW-ON-EXTENSION-FOLLOW-ON-EXTENSION-FOLLOW-ON-EXTENSION-FOLLOW-ON-EXTENSION-FOLLOW-ON-EXTENSION-FOLLOW-ON-EXTENSION-PILOT.md"
CARD="docs/development/current/main/phases/phase-293x/293x-1184-MIMAP-554A-ALLOCATOR-COMPARISON-C-MIMALLOC-RESULT-PRESENTATION-EXTENSION-FOLLOW-ON-EXTENSION-FOLLOW-ON-EXTENSION-FOLLOW-ON-EXTENSION-FOLLOW-ON-EXTENSION-FOLLOW-ON-EXTENSION-FOLLOW-ON-EXTENSION-FOLLOW-ON-CLOSEOUT.md"
NEXT_CARD="docs/development/current/main/phases/phase-293x/293x-1185-MIMAP-555A-POST-PRESENTATION-EXTENSION-FOLLOW-ON-EXTENSION-FOLLOW-ON-EXTENSION-FOLLOW-ON-EXTENSION-FOLLOW-ON-EXTENSION-FOLLOW-ON-EXTENSION-FOLLOW-ON-EXTENSION-FOLLOW-ON-CLOSEOUT-ROW-SELECTION.md"
DESIGN_552A="docs/development/current/main/design/hako-alloc-allocator-comparison-c-mimalloc-result-presentation-extension-follow-on-extension-follow-on-extension-follow-on-extension-follow-on-extension-follow-on-extension-follow-on-extension-follow-on-pilot-ssot.md"
INDEX="docs/tools/check-scripts-index.md"
PROOF_MANIFEST_INCLUDE="tools/checks/manifests/proof_apps/hako_alloc_segment_arena_backing_release_lifecycle.toml"
GUARD_552A="tools/checks/k2_wide_hako_alloc_allocator_comparison_c_mimalloc_result_presentation_extension_follow_on_extension_follow_on_extension_follow_on_extension_follow_on_extension_follow_on_extension_follow_on_extension_follow_on_pilot_guard.sh"
SELF_SCRIPT="tools/checks/k2_wide_hako_alloc_allocator_comparison_c_mimalloc_result_presentation_extension_follow_on_extension_follow_on_extension_follow_on_extension_follow_on_extension_follow_on_extension_follow_on_extension_follow_on_closeout_guard.sh"

printf '[%s] checking MIMAP-554A C mimalloc result presentation extension follow-on extension follow-on extension follow-on extension follow-on extension follow-on extension follow-on extension follow-on closeout\n' "$TAG"

guard_require_files "$TAG" "$CARD_552A" "$CARD" "$NEXT_CARD" "$DESIGN_552A" "$INDEX" "$PROOF_MANIFEST_INCLUDE" "$GUARD_552A" "$SELF_SCRIPT"
guard_require_exec_files "$TAG" "$GUARD_552A" "$SELF_SCRIPT"

guard_expect_in_file "$TAG" 'Status: landed' "$CARD_552A" "MIMAP-552A must be landed"
guard_expect_in_file "$TAG" 'Status: (selected current|completed|landed)' "$CARD" "MIMAP-554A must be current/completed/landed"
guard_expect_in_file "$TAG" 'Status: (selected current|landed)' "$NEXT_CARD" "MIMAP-555A must be selected current or landed"
guard_expect_in_file "$TAG" 'Decision: accepted' "$DESIGN_552A" "MIMAP-552A design must remain accepted"
guard_expect_fixed_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list MIMAP-554A guard"
guard_expect_in_file "$TAG" 'id = "MIMAP-552A"' "$PROOF_MANIFEST_INCLUDE" "proof manifest must list MIMAP-552A"
guard_expect_in_file "$TAG" 'closeout_pack = "allocator-comparison-c-mimalloc-result-presentation-extension-follow-on-extension-follow-on-extension-follow-on-extension-follow-on-extension-follow-on-extension-follow-on-extension-follow-on"' "$PROOF_MANIFEST_INCLUDE" "comparison-ready pilot row must keep its closeout pack"
guard_expect_in_file "$TAG" 'Keep this row closeout-only; do not add new execution or reopen closed seams\.' "$CARD" "MIMAP-554A must remain closeout-only"
guard_expect_in_file "$TAG" 'presentation extension follow-on extension follow-on extension follow-on extension follow-on extension follow-on extension follow-on extension follow-on plan closeout' "$NEXT_CARD" "MIMAP-555A must keep the plan closeout option visible"
guard_expect_in_file "$TAG" 'No repeated or heavy benchmark pack' "$NEXT_CARD" "MIMAP-555A must keep benchmark reruns closed"
guard_expect_in_file "$TAG" 'No explicit C mimalloc runner execution.' "$NEXT_CARD" "MIMAP-555A must keep runner execution closed"

bash "$GUARD_552A" --level L2

printf '[%s] ok\n' "$TAG"
