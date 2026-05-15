#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="docs-slim-008-recent-cleanup-resolver"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh
source tools/checks/lib/phase_card_paths.sh

CARD="$(guard_require_phase293x_card "$TAG" "293x-415-DOCS-SLIM-008-RECENT-CLEANUP-RESOLVER.md")"
CHECK_INDEX="docs/tools/check-scripts-index.md"
ARCHIVE_POLICY="docs/development/current/main/design/current-docs-archive-policy-ssot.md"
HELPER="tools/checks/lib/phase_card_paths.sh"
DEV_GATE="tools/checks/dev_gate.sh"
ALLOCATOR_GATE="tools/checks/k2_wide_allocator_gate.sh"
SELF_SCRIPT="tools/checks/docs_slim_008_recent_cleanup_resolver_guard.sh"

converted_scripts=(
  "tools/checks/k2_wide_looprange_ast_rename_guard.sh"
  "tools/checks/k2_wide_loopclean_while_parser_facade_guard.sh"
  "tools/checks/k2_wide_clean_stage1_lowering_stmt_split_guard.sh"
)

echo "[$TAG] running DOCS-SLIM-008 recent cleanup resolver guard"

guard_require_files \
  "$TAG" \
  "$CARD" \
  "$CHECK_INDEX" \
  "$ARCHIVE_POLICY" \
  "$HELPER" \
  "$DEV_GATE" \
  "$ALLOCATOR_GATE" \
  "$SELF_SCRIPT" \
  "${converted_scripts[@]}"
guard_require_exec_files "$TAG" "$HELPER" "$SELF_SCRIPT"

guard_expect_in_file "$TAG" "DOCS-SLIM-008" "$CARD" "DOCS-SLIM-008 card must exist"
guard_expect_in_file "$TAG" "Do not move numbered cards in this row" "$CARD" "card must keep no-move stop-line"
guard_expect_in_file "$TAG" "Eighth Slimming Phase" "$ARCHIVE_POLICY" "archive policy must record DOCS-SLIM-008"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$CHECK_INDEX" "check index must list DOCS-SLIM-008 guard"

for script in "${converted_scripts[@]}"; do
  guard_expect_in_file "$TAG" "phase_card_paths.sh" "$script" "$script must source phase card resolver helper"
  guard_expect_in_file "$TAG" "guard_require_phase293x_card" "$script" "$script must resolve cards via helper"
  if rg -n 'docs/development/current/main/phases/phase-293x/293x-[0-9][0-9][0-9]-' "$script" >/tmp/"$TAG".direct_path 2>&1; then
    echo "[$TAG] ERROR: converted script still contains direct phase-293x card paths: $script" >&2
    cat /tmp/"$TAG".direct_path >&2
    rm -f /tmp/"$TAG".direct_path
    exit 1
  fi
done
rm -f /tmp/"$TAG".direct_path

guard_require_no_phase_card_resolver_leak "$TAG" "$DEV_GATE" "$ALLOCATOR_GATE"

for script in "${converted_scripts[@]}"; do
  bash "$script" >/dev/null
done

echo "[$TAG] ok converted=${#converted_scripts[@]}"
