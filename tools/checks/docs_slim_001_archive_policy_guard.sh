#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="docs-slim-001-archive-policy"
cd "$ROOT_DIR"
source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

STATE="docs/development/current/main/CURRENT_STATE.toml"
CARD="docs/development/current/main/phases/phase-293x/293x-408-DOCS-SLIM-001-ARCHIVE-POLICY-AND-INVENTORY.md"
POLICY="docs/development/current/main/design/current-docs-archive-policy-ssot.md"
UPDATE_POLICY="docs/development/current/main/design/current-docs-update-policy-ssot.md"
DOCS_LAYOUT="docs/development/current/main/DOCS_LAYOUT.md"
INDEX="docs/tools/check-scripts-index.md"
LANG_TASKBOARD="docs/development/current/main/phases/phase-293x/293x-language-minimal-taskboard.md"
MIMAP_TASKBOARD="docs/development/current/main/phases/phase-293x/293x-mimalloc-port-taskboard.md"
LOOPRANGE_GUARD="tools/checks/k2_wide_looprange_ast_rename_guard.sh"
WHILE_GUARD="tools/checks/k2_wide_loopclean_while_parser_facade_guard.sh"
STMT_GUARD="tools/checks/k2_wide_clean_stage1_lowering_stmt_split_guard.sh"
SELF_SCRIPT="tools/checks/docs_slim_001_archive_policy_guard.sh"

echo "[$TAG] running DOCS-SLIM-001 archive policy guard"

guard_require_command "$TAG" awk
guard_require_files \
  "$TAG" \
  "$STATE" \
  "$CARD" \
  "$POLICY" \
  "$UPDATE_POLICY" \
  "$DOCS_LAYOUT" \
  "$INDEX" \
  "$LANG_TASKBOARD" \
  "$MIMAP_TASKBOARD" \
  "$LOOPRANGE_GUARD" \
  "$WHILE_GUARD" \
  "$STMT_GUARD" \
  "$SELF_SCRIPT"
guard_require_exec_files "$TAG" "$SELF_SCRIPT"

guard_expect_in_file "$TAG" 'DOCS-SLIM-001' "$CARD" "DOCS-SLIM-001 card must exist"
guard_expect_in_file "$TAG" 'Do not move phase cards in this row' "$CARD" "card must keep archive move stop-line"
guard_expect_in_file "$TAG" 'current docs archive policy SSOT' "$CARD" "card must mention policy SSOT"
guard_expect_in_file "$TAG" 'Current Docs Archive Policy' "$POLICY" "archive policy SSOT must exist"
guard_expect_in_file "$TAG" 'target maximum:' "$POLICY" "archive policy must cap landed_tail"
guard_expect_in_file "$TAG" 'Do not add a taskboard assertion just to prove a card landed' "$POLICY" "policy must forbid taskboard closeout proof"
guard_expect_in_file "$TAG" 'current-docs-archive-policy-ssot' "$DOCS_LAYOUT" "DOCS_LAYOUT must link archive policy"
guard_expect_in_file "$TAG" 'current-docs-archive-policy-ssot' "$UPDATE_POLICY" "update policy must link archive policy"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check script index must list this guard"

guard_expect_in_file "$TAG" 'latest_card = "293x-408-DOCS-SLIM-001-ARCHIVE-POLICY-AND-INVENTORY"' "$STATE" "CURRENT_STATE latest_card must point at DOCS-SLIM-001"

tail_count="$(awk '
  /^[[:space:]]*landed_tail[[:space:]]*=/ { in_tail=1; next }
  in_tail && /^[[:space:]]*]/ { print count; exit }
  in_tail && /^[[:space:]]*"/ { count++ }
' "$STATE")"
if [[ -z "$tail_count" ]]; then
  guard_fail "$TAG" "could not count CURRENT_STATE landed_tail"
fi
if (( tail_count > 12 )); then
  guard_fail "$TAG" "CURRENT_STATE landed_tail too long: $tail_count > 12"
fi

for script in "$LOOPRANGE_GUARD" "$WHILE_GUARD" "$STMT_GUARD"; do
  if rg -n 'TASKBOARD|293x-(language-minimal|mimalloc-port)-taskboard' "$script" >/tmp/docs_slim_guard_hits.$$ 2>/dev/null; then
    echo "[$TAG] ERROR: recent cleanup guard still uses taskboard as landed proof: $script" >&2
    cat /tmp/docs_slim_guard_hits.$$ >&2
    rm -f /tmp/docs_slim_guard_hits.$$
    exit 1
  fi
done
rm -f /tmp/docs_slim_guard_hits.$$

if rg -n 'LOOPCLEAN-005' "$LANG_TASKBOARD" >/tmp/docs_slim_guard_hits.$$ 2>/dev/null; then
  echo "[$TAG] ERROR: language taskboard still carries LOOPCLEAN-005 closeout ledger row" >&2
  cat /tmp/docs_slim_guard_hits.$$ >&2
  rm -f /tmp/docs_slim_guard_hits.$$
  exit 1
fi

if rg -n 'LOOPCLEAN-006|CLEAN-STAGE1-LOWERING-002' "$MIMAP_TASKBOARD" >/tmp/docs_slim_guard_hits.$$ 2>/dev/null; then
  echo "[$TAG] ERROR: mimalloc taskboard still carries recent cleanup closeout ledger row" >&2
  cat /tmp/docs_slim_guard_hits.$$ >&2
  rm -f /tmp/docs_slim_guard_hits.$$
  exit 1
fi
rm -f /tmp/docs_slim_guard_hits.$$

echo "[$TAG] ok landed_tail=$tail_count"
