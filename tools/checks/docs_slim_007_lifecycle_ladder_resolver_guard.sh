#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="docs-slim-007-lifecycle-ladder-resolver"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh
source tools/checks/lib/phase_card_paths.sh

CARD="$(guard_require_phase293x_card "$TAG" "293x-414-DOCS-SLIM-007-LIFECYCLE-LADDER-RESOLVER.md")"
CHECK_INDEX="docs/tools/check-scripts-index.md"
ARCHIVE_POLICY="docs/development/current/main/design/current-docs-archive-policy-ssot.md"
HELPER="tools/checks/lib/phase_card_paths.sh"
DEV_GATE="tools/checks/dev_gate.sh"
ALLOCATOR_GATE="tools/checks/k2_wide_allocator_gate.sh"
SELF_SCRIPT="tools/checks/docs_slim_007_lifecycle_ladder_resolver_guard.sh"

converted_scripts=(
  "tools/checks/k2_wide_lifecycle_birth_new_only_guard.sh"
  "tools/checks/k2_wide_parser_birth_direct_call_guard.sh"
  "tools/checks/k2_wide_parser_birth_diagnostic_hint_guard.sh"
  "tools/checks/k2_wide_reuse_lifecycle_explicit_methods_guard.sh"
)

echo "[$TAG] running DOCS-SLIM-007 lifecycle ladder resolver guard"

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

guard_expect_in_file "$TAG" "DOCS-SLIM-007" "$CARD" "DOCS-SLIM-007 card must exist"
guard_expect_in_file "$TAG" "Do not move numbered cards in this row" "$CARD" "card must keep no-move stop-line"
guard_expect_in_file "$TAG" "Seventh Slimming Phase" "$ARCHIVE_POLICY" "archive policy must record DOCS-SLIM-007"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$CHECK_INDEX" "check index must list DOCS-SLIM-007 guard"

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

if rg -n 'phase_card_paths|guard_require_phase293x_card' "$DEV_GATE" "$ALLOCATOR_GATE" >/tmp/"$TAG".gate_leak 2>&1; then
  echo "[$TAG] ERROR: phase-card resolver helper must not be wired into dev_gate or allocator-wide directly" >&2
  cat /tmp/"$TAG".gate_leak >&2
  rm -f /tmp/"$TAG".gate_leak
  exit 1
fi
rm -f /tmp/"$TAG".gate_leak

for script in "${converted_scripts[@]}"; do
  bash "$script" >/dev/null
done

echo "[$TAG] ok converted=${#converted_scripts[@]}"
