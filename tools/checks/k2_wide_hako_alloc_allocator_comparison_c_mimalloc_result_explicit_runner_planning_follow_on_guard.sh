#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-hako-alloc-allocator-comparison-c-mimalloc-result-explicit-runner-planning-follow-on"
cd "$ROOT_DIR"
source "$ROOT_DIR/tools/checks/lib/pure_first_exe_guard.sh"

if [ "$#" -ne 0 ]; then
  VALIDATION_LEVEL="$(pure_first_guard_parse_level "$TAG" "$@")"
  case "$VALIDATION_LEVEL" in
    L0|L1|L2) ;;
    L3|L4)
      echo "[$TAG] ERROR: MIMAP-564A planning follow-on does not define an L3/L4 pack" >&2
      exit 2
      ;;
  esac
fi

CARD_562A="docs/development/current/main/phases/phase-293x/293x-1192-MIMAP-562A-ALLOCATOR-COMPARISON-C-MIMALLOC-RESULT-PRESENTATION-ONLY-EXTENSION-CLOSEOUT.md"
CARD_563A="docs/development/current/main/phases/phase-293x/293x-1193-MIMAP-563A-POST-PRESENTATION-ONLY-EXTENSION-CLOSEOUT-ROW-SELECTION.md"
CARD="docs/development/current/main/phases/phase-293x/293x-1194-MIMAP-564A-ALLOCATOR-COMPARISON-C-MIMALLOC-RESULT-EXPLICIT-RUNNER-PLANNING-FOLLOW-ON.md"
NEXT_CARD="docs/development/current/main/phases/phase-293x/293x-1195-MIMAP-565A-POST-EXPLICIT-RUNNER-PLANNING-FOLLOW-ON-ROW-SELECTION.md"
INDEX="docs/tools/check-scripts-index.md"
GUARD_562A="tools/checks/k2_wide_hako_alloc_allocator_comparison_c_mimalloc_result_presentation_only_extension_closeout_guard.sh"
SELF_SCRIPT="tools/checks/k2_wide_hako_alloc_allocator_comparison_c_mimalloc_result_explicit_runner_planning_follow_on_guard.sh"

printf '[%s] checking MIMAP-564A allocator comparison C mimalloc result explicit runner planning follow-on\n' "$TAG"

guard_require_files "$TAG" "$CARD_562A" "$CARD_563A" "$CARD" "$NEXT_CARD" "$INDEX" "$GUARD_562A" "$SELF_SCRIPT"
guard_require_exec_files "$TAG" "$GUARD_562A" "$SELF_SCRIPT"

guard_expect_in_file "$TAG" 'Status: (completed|landed)' "$CARD_562A" "MIMAP-562A must be completed/landed"
guard_expect_in_file "$TAG" 'Status: landed' "$CARD_563A" "MIMAP-563A must be landed"
guard_expect_in_file "$TAG" 'Status: (selected current|completed|landed)' "$CARD" "MIMAP-564A must be current/completed/landed"
guard_expect_in_file "$TAG" 'Status: (selected current|landed)' "$NEXT_CARD" "MIMAP-565A must be selected current or landed"
guard_expect_fixed_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list MIMAP-564A guard"
guard_expect_in_file "$TAG" 'external evidence source' "$CARD" "MIMAP-564A must keep explicit runner as external evidence source"
guard_expect_in_file "$TAG" '`allocator_id`, `runner_kind`, `workload_id`' "$CARD" "MIMAP-564A must preserve report schema anchor"
guard_expect_in_file "$TAG" 'No explicit C mimalloc runner execution\.' "$CARD" "MIMAP-564A must keep runner execution closed"
guard_expect_in_file "$TAG" 'explicit runner planning pilot row' "$NEXT_CARD" "MIMAP-565A must keep explicit runner pilot option visible"

bash "$GUARD_562A" --level L2

printf '[%s] ok\n' "$TAG"
