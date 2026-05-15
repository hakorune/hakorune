#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="docs-slim-010-manifest-runner-decoupling"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh
source tools/checks/lib/phase_card_paths.sh

CARD="$(guard_require_phase293x_card "$TAG" "293x-417-DOCS-SLIM-010-MANIFEST-RUNNER-DECOUPLING.md")"
CHECK_INDEX="docs/tools/check-scripts-index.md"
ARCHIVE_POLICY="docs/development/current/main/design/current-docs-archive-policy-ssot.md"
MANIFEST_GUARD="tools/checks/manifest_runner_pilot_guard.sh"
HELPER="tools/checks/lib/phase_card_paths.sh"
DEV_GATE="tools/checks/dev_gate.sh"
ALLOCATOR_GATE="tools/checks/k2_wide_allocator_gate.sh"
SELF_SCRIPT="tools/checks/docs_slim_010_manifest_runner_decoupling_guard.sh"

echo "[$TAG] running DOCS-SLIM-010 manifest runner decoupling guard"

guard_require_files \
  "$TAG" \
  "$CARD" \
  "$CHECK_INDEX" \
  "$ARCHIVE_POLICY" \
  "$MANIFEST_GUARD" \
  "$HELPER" \
  "$DEV_GATE" \
  "$ALLOCATOR_GATE" \
  "$SELF_SCRIPT"
guard_require_exec_files "$TAG" "$HELPER" "$MANIFEST_GUARD" "$SELF_SCRIPT"

guard_require_docs_slim_card_metadata \
  "$TAG" \
  "$CARD" \
  "$ARCHIVE_POLICY" \
  "$CHECK_INDEX" \
  "$SELF_SCRIPT" \
  "DOCS-SLIM-010" \
  "Tenth Slimming Phase"

guard_expect_in_file "$TAG" "phase_card_paths.sh" "$MANIFEST_GUARD" "manifest guard must source phase card resolver helper"
guard_expect_in_file "$TAG" "guard_require_phase293x_card" "$MANIFEST_GUARD" "manifest guard must resolve its card via helper"

if rg -n 'docs/development/current/main/phases/phase-293x/293x-[0-9][0-9][0-9]-' "$MANIFEST_GUARD" >/tmp/"$TAG".direct_path 2>&1; then
  echo "[$TAG] ERROR: manifest guard still contains direct phase-293x card paths" >&2
  cat /tmp/"$TAG".direct_path >&2
  rm -f /tmp/"$TAG".direct_path
  exit 1
fi
rm -f /tmp/"$TAG".direct_path

if rg -n 'PHASE_README|TASKBOARD|293x-90-real-app-taskboard|phase-293x/README' "$MANIFEST_GUARD" >/tmp/"$TAG".landed_pin 2>&1; then
  echo "[$TAG] ERROR: manifest guard must not depend on landed-history README/taskboard pins" >&2
  cat /tmp/"$TAG".landed_pin >&2
  rm -f /tmp/"$TAG".landed_pin
  exit 1
fi
rm -f /tmp/"$TAG".landed_pin

guard_require_no_phase_card_resolver_leak "$TAG" "$DEV_GATE" "$ALLOCATOR_GATE"

bash "$MANIFEST_GUARD" >/dev/null

echo "[$TAG] ok"
