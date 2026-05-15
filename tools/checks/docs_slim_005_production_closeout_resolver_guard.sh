#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="docs-slim-005-production-closeout-resolver"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh
source tools/checks/lib/phase_card_paths.sh

CARD="$(guard_require_phase293x_card "$TAG" "293x-412-DOCS-SLIM-005-PRODUCTION-CLOSEOUT-RESOLVER.md")"
CHECK_INDEX="docs/tools/check-scripts-index.md"
ARCHIVE_POLICY="docs/development/current/main/design/current-docs-archive-policy-ssot.md"
HELPER="tools/checks/lib/phase_card_paths.sh"
DEV_GATE="tools/checks/dev_gate.sh"
ALLOCATOR_GATE="tools/checks/k2_wide_allocator_gate.sh"
TARGET="tools/checks/k2_wide_production_allocator_port_closeout_guard.sh"
SELF_SCRIPT="tools/checks/docs_slim_005_production_closeout_resolver_guard.sh"

echo "[$TAG] running DOCS-SLIM-005 production closeout resolver guard"

guard_require_files \
  "$TAG" \
  "$CARD" \
  "$CHECK_INDEX" \
  "$ARCHIVE_POLICY" \
  "$HELPER" \
  "$DEV_GATE" \
  "$ALLOCATOR_GATE" \
  "$TARGET" \
  "$SELF_SCRIPT"
guard_require_exec_files "$TAG" "$HELPER" "$SELF_SCRIPT"

guard_expect_in_file "$TAG" "DOCS-SLIM-005" "$CARD" "DOCS-SLIM-005 card must exist"
guard_expect_in_file "$TAG" "Do not move numbered cards in this row" "$CARD" "card must keep no-move stop-line"
guard_expect_in_file "$TAG" "Fifth Slimming Phase" "$ARCHIVE_POLICY" "archive policy must record DOCS-SLIM-005"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$CHECK_INDEX" "check index must list DOCS-SLIM-005 guard"
guard_expect_in_file "$TAG" "phase_card_paths.sh" "$TARGET" "production closeout guard must source phase card resolver helper"
guard_expect_in_file "$TAG" "guard_require_phase293x_card" "$TARGET" "production closeout guard must resolve cards via helper"

if rg -n 'docs/development/current/main/phases/phase-293x/293x-[0-9][0-9][0-9]-' "$TARGET" >/tmp/"$TAG".direct_path 2>&1; then
  echo "[$TAG] ERROR: production closeout guard still contains direct phase-293x card paths" >&2
  cat /tmp/"$TAG".direct_path >&2
  rm -f /tmp/"$TAG".direct_path
  exit 1
fi
rm -f /tmp/"$TAG".direct_path

if rg -n 'phase_card_paths|guard_require_phase293x_card' "$DEV_GATE" "$ALLOCATOR_GATE" >/tmp/"$TAG".gate_leak 2>&1; then
  echo "[$TAG] ERROR: phase-card resolver helper must not be wired into dev_gate or allocator-wide directly" >&2
  cat /tmp/"$TAG".gate_leak >&2
  rm -f /tmp/"$TAG".gate_leak
  exit 1
fi
rm -f /tmp/"$TAG".gate_leak

bash "$TARGET" >/dev/null

echo "[$TAG] ok"
