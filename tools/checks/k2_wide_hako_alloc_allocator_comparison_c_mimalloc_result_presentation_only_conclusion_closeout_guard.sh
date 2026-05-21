#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-hako-alloc-allocator-comparison-c-mimalloc-result-presentation-only-conclusion-closeout"
cd "$ROOT_DIR"
source "$ROOT_DIR/tools/checks/lib/pure_first_exe_guard.sh"

if [ "$#" -ne 0 ]; then
  VALIDATION_LEVEL="$(pure_first_guard_parse_level "$TAG" "$@")"
  case "$VALIDATION_LEVEL" in
    L0|L1|L2) ;;
    L3|L4)
      echo "[$TAG] ERROR: MIMAP-476A closeout does not define an L3/L4 pack" >&2
      exit 2
      ;;
  esac
fi

CARD_474A="docs/development/current/main/phases/phase-293x/293x-1104-MIMAP-474A-ALLOCATOR-COMPARISON-C-MIMALLOC-RESULT-PRESENTATION-ONLY-CONCLUSION-PILOT.md"
CARD="docs/development/current/main/phases/phase-293x/293x-1106-MIMAP-476A-ALLOCATOR-COMPARISON-C-MIMALLOC-RESULT-PRESENTATION-ONLY-CONCLUSION-CLOSEOUT.md"
NEXT_CARD="docs/development/current/main/phases/phase-293x/293x-1107-MIMAP-477A-POST-PRESENTATION-ONLY-CONCLUSION-CLOSEOUT-ROW-SELECTION.md"
DESIGN_474A="docs/development/current/main/design/hako-alloc-allocator-comparison-c-mimalloc-result-presentation-only-conclusion-pilot-ssot.md"
INDEX="docs/tools/check-scripts-index.md"
PROOF_MANIFEST_INCLUDE="tools/checks/manifests/proof_apps/hako_alloc_segment_arena_backing_release_lifecycle.toml"
GUARD_474A="tools/checks/k2_wide_hako_alloc_allocator_comparison_c_mimalloc_result_presentation_only_conclusion_pilot_guard.sh"
SELF_SCRIPT="tools/checks/k2_wide_hako_alloc_allocator_comparison_c_mimalloc_result_presentation_only_conclusion_closeout_guard.sh"

printf '[%s] checking MIMAP-476A C mimalloc result presentation-only conclusion closeout\n' "$TAG"

guard_require_files "$TAG" "$CARD_474A" "$CARD" "$NEXT_CARD" "$DESIGN_474A" "$INDEX" "$PROOF_MANIFEST_INCLUDE" "$GUARD_474A" "$SELF_SCRIPT"
guard_require_exec_files "$TAG" "$GUARD_474A" "$SELF_SCRIPT"

guard_expect_in_file "$TAG" 'Status: landed' "$CARD_474A" "MIMAP-474A must be landed"
guard_expect_in_file "$TAG" 'Status: (selected current|completed|landed)' "$CARD" "MIMAP-476A must be current/completed/landed"
guard_expect_in_file "$TAG" 'Status: (selected current|landed)' "$NEXT_CARD" "MIMAP-477A must be selected current or landed"
guard_expect_in_file "$TAG" 'Decision: accepted' "$DESIGN_474A" "MIMAP-474A design must remain accepted"
guard_expect_fixed_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list MIMAP-476A guard"
guard_expect_in_file "$TAG" 'id = "MIMAP-474A"' "$PROOF_MANIFEST_INCLUDE" "proof manifest must list MIMAP-474A"
guard_expect_in_file "$TAG" 'closeout_pack = "allocator-comparison-c-mimalloc-result-presentation-only-conclusion"' "$PROOF_MANIFEST_INCLUDE" "presentation-only pilot row must keep its closeout pack"
guard_expect_in_file "$TAG" 'Close the presentation-only conclusion pilot pack after MIMAP-474A' "$CARD" "MIMAP-476A must remain closeout-only"
guard_expect_in_file "$TAG" 'presentation follow-on plan' "$NEXT_CARD" "MIMAP-477A must keep the presentation plan option visible"
guard_expect_in_file "$TAG" 'No repeated or heavy benchmark pack' "$NEXT_CARD" "MIMAP-477A must keep benchmark reruns closed"

bash "$GUARD_474A" --level L2

printf '[%s] ok\n' "$TAG"
