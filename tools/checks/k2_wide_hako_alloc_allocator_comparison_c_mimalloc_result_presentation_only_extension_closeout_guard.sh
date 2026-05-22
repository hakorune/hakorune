#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-hako-alloc-allocator-comparison-c-mimalloc-result-presentation-only-extension-closeout"
cd "$ROOT_DIR"
source "$ROOT_DIR/tools/checks/lib/pure_first_exe_guard.sh"

if [ "$#" -ne 0 ]; then
  VALIDATION_LEVEL="$(pure_first_guard_parse_level "$TAG" "$@")"
  case "$VALIDATION_LEVEL" in
    L0|L1|L2) ;;
    L3|L4)
      echo "[$TAG] ERROR: MIMAP-562A closeout does not define an L3/L4 pack" >&2
      exit 2
      ;;
  esac
fi

CARD_560A="docs/development/current/main/phases/phase-293x/293x-1190-MIMAP-560A-ALLOCATOR-COMPARISON-C-MIMALLOC-RESULT-PRESENTATION-ONLY-EXTENSION-PILOT.md"
CARD="docs/development/current/main/phases/phase-293x/293x-1192-MIMAP-562A-ALLOCATOR-COMPARISON-C-MIMALLOC-RESULT-PRESENTATION-ONLY-EXTENSION-CLOSEOUT.md"
NEXT_CARD="docs/development/current/main/phases/phase-293x/293x-1193-MIMAP-563A-POST-PRESENTATION-ONLY-EXTENSION-CLOSEOUT-ROW-SELECTION.md"
DESIGN_560A="docs/development/current/main/design/hako-alloc-allocator-comparison-c-mimalloc-result-presentation-only-extension-pilot-ssot.md"
INDEX="docs/tools/check-scripts-index.md"
PROOF_MANIFEST_INCLUDE="tools/checks/manifests/proof_apps/hako_alloc_segment_arena_backing_release_lifecycle.toml"
GUARD_560A="tools/checks/k2_wide_hako_alloc_allocator_comparison_c_mimalloc_result_presentation_only_extension_pilot_guard.sh"
SELF_SCRIPT="tools/checks/k2_wide_hako_alloc_allocator_comparison_c_mimalloc_result_presentation_only_extension_closeout_guard.sh"

printf '[%s] checking MIMAP-562A allocator comparison C mimalloc result presentation-only extension closeout\n' "$TAG"

guard_require_files "$TAG" "$CARD_560A" "$CARD" "$NEXT_CARD" "$DESIGN_560A" "$INDEX" "$PROOF_MANIFEST_INCLUDE" "$GUARD_560A" "$SELF_SCRIPT"
guard_require_exec_files "$TAG" "$GUARD_560A" "$SELF_SCRIPT"

guard_expect_in_file "$TAG" 'Status: landed' "$CARD_560A" "MIMAP-560A must be landed"
guard_expect_in_file "$TAG" 'Status: (selected current|completed|landed)' "$CARD" "MIMAP-562A must be current/completed/landed"
guard_expect_in_file "$TAG" 'Status: (selected current|landed)' "$NEXT_CARD" "MIMAP-563A must be selected current or landed"
guard_expect_in_file "$TAG" 'Decision: accepted' "$DESIGN_560A" "MIMAP-560A design must remain accepted"
guard_expect_fixed_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list MIMAP-562A guard"
guard_expect_in_file "$TAG" 'id = "MIMAP-560A"' "$PROOF_MANIFEST_INCLUDE" "proof manifest must list MIMAP-560A"
guard_expect_in_file "$TAG" 'closeout_pack = "allocator-comparison-c-mimalloc-result-presentation-only-extension"' "$PROOF_MANIFEST_INCLUDE" "presentation-only extension pilot row must keep its closeout pack"
guard_expect_in_file "$TAG" 'Keep this row closeout-only; do not add new execution or reopen closed seams\.' "$CARD" "MIMAP-562A must remain closeout-only"
guard_expect_in_file "$TAG" 'deeper explicit C mimalloc runner planning row' "$NEXT_CARD" "MIMAP-563A must keep deeper runner planning option visible"
guard_expect_in_file "$TAG" 'No explicit C mimalloc runner execution.' "$NEXT_CARD" "MIMAP-563A must keep runner execution closed"

bash "$GUARD_560A" --level L2

printf '[%s] ok\n' "$TAG"
