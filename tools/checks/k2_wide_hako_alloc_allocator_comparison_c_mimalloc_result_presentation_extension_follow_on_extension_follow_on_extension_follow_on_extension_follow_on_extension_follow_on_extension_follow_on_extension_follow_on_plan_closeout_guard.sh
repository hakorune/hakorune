#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-hako-alloc-allocator-comparison-c-mimalloc-result-presentation-extension-follow-on-extension-follow-on-extension-follow-on-extension-follow-on-extension-follow-on-extension-follow-on-extension-follow-on-plan-closeout"
cd "$ROOT_DIR"
source "$ROOT_DIR/tools/checks/lib/pure_first_exe_guard.sh"

if [ "$#" -ne 0 ]; then
  VALIDATION_LEVEL="$(pure_first_guard_parse_level "$TAG" "$@")"
  case "$VALIDATION_LEVEL" in
    L0|L1|L2) ;;
    L3|L4)
      echo "[$TAG] ERROR: MIMAP-556A plan closeout does not define an L3/L4 pack" >&2
      exit 2
      ;;
  esac
fi

CARD_550A="docs/development/current/main/phases/phase-293x/293x-1180-MIMAP-550A-ALLOCATOR-COMPARISON-C-MIMALLOC-RESULT-PRESENTATION-EXTENSION-FOLLOW-ON-EXTENSION-FOLLOW-ON-EXTENSION-FOLLOW-ON-EXTENSION-FOLLOW-ON-EXTENSION-FOLLOW-ON-EXTENSION-FOLLOW-ON-PLAN.md"
CARD_554A="docs/development/current/main/phases/phase-293x/293x-1184-MIMAP-554A-ALLOCATOR-COMPARISON-C-MIMALLOC-RESULT-PRESENTATION-EXTENSION-FOLLOW-ON-EXTENSION-FOLLOW-ON-EXTENSION-FOLLOW-ON-EXTENSION-FOLLOW-ON-EXTENSION-FOLLOW-ON-EXTENSION-FOLLOW-ON-EXTENSION-FOLLOW-ON-CLOSEOUT.md"
CARD="docs/development/current/main/phases/phase-293x/293x-1186-MIMAP-556A-ALLOCATOR-COMPARISON-C-MIMALLOC-RESULT-PRESENTATION-EXTENSION-FOLLOW-ON-EXTENSION-FOLLOW-ON-EXTENSION-FOLLOW-ON-EXTENSION-FOLLOW-ON-EXTENSION-FOLLOW-ON-EXTENSION-FOLLOW-ON-EXTENSION-FOLLOW-ON-PLAN-CLOSEOUT.md"
NEXT_CARD="docs/development/current/main/phases/phase-293x/293x-1187-MIMAP-557A-POST-PRESENTATION-EXTENSION-FOLLOW-ON-EXTENSION-FOLLOW-ON-EXTENSION-FOLLOW-ON-EXTENSION-FOLLOW-ON-EXTENSION-FOLLOW-ON-EXTENSION-FOLLOW-ON-EXTENSION-FOLLOW-ON-PLAN-CLOSEOUT-ROW-SELECTION.md"
INDEX="docs/tools/check-scripts-index.md"
GUARD_554A="tools/checks/k2_wide_hako_alloc_allocator_comparison_c_mimalloc_result_presentation_extension_follow_on_extension_follow_on_extension_follow_on_extension_follow_on_extension_follow_on_extension_follow_on_extension_follow_on_closeout_guard.sh"
SELF_SCRIPT="tools/checks/k2_wide_hako_alloc_allocator_comparison_c_mimalloc_result_presentation_extension_follow_on_extension_follow_on_extension_follow_on_extension_follow_on_extension_follow_on_extension_follow_on_extension_follow_on_plan_closeout_guard.sh"

printf '[%s] checking MIMAP-556A C mimalloc result presentation extension follow-on extension follow-on extension follow-on extension follow-on extension follow-on extension follow-on extension follow-on plan closeout\n' "$TAG"

guard_require_files "$TAG" "$CARD_550A" "$CARD_554A" "$CARD" "$NEXT_CARD" "$INDEX" "$GUARD_554A" "$SELF_SCRIPT"
guard_require_exec_files "$TAG" "$GUARD_554A" "$SELF_SCRIPT"

guard_expect_in_file "$TAG" 'Status: landed' "$CARD_550A" "MIMAP-550A must be landed"
guard_expect_in_file "$TAG" 'Status: completed' "$CARD_554A" "MIMAP-554A must be completed"
guard_expect_in_file "$TAG" 'Status: (selected current|completed|landed)' "$CARD" "MIMAP-556A must be current/completed/landed"
guard_expect_in_file "$TAG" 'Status: (selected current|landed)' "$NEXT_CARD" "MIMAP-557A must be selected current or landed"
guard_expect_fixed_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list MIMAP-556A guard"
guard_expect_in_file "$TAG" 'allocator_id' "$CARD_550A" "MIMAP-550A must keep the shared comparison fields"
guard_expect_in_file "$TAG" 'runner_kind' "$CARD_550A" "MIMAP-550A must keep runner kind in the shared contract"
guard_expect_in_file "$TAG" 'workload_id' "$CARD_550A" "MIMAP-550A must keep workload identity in the shared contract"
guard_expect_in_file "$TAG" 'evidence_complete' "$CARD_550A" "MIMAP-550A must keep evidence completeness in the shared contract"
guard_expect_in_file "$TAG" 'it does not rely on hooks, `LD_PRELOAD`, hidden discovery, DLL/provider' "$CARD_550A" "MIMAP-550A must keep explicit runner activation closed"
guard_expect_in_file "$TAG" 'Keep this row closeout-only; do not add new execution or reopen closed seams\.' "$CARD" "MIMAP-556A must remain closeout-only"
guard_expect_in_file "$TAG" 'presentation-only extension row' "$NEXT_CARD" "MIMAP-557A must keep the presentation-only extension option visible"
guard_expect_in_file "$TAG" 'No explicit C mimalloc runner execution.' "$NEXT_CARD" "MIMAP-557A must keep runner execution closed"

bash "$GUARD_554A"

printf '[%s] ok\n' "$TAG"
