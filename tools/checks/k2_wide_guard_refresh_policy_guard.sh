#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-guard-refresh-policy"
cd "$ROOT_DIR"
source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

CARD="docs/development/current/main/phases/phase-293x/293x-222-D196-STOP-THE-LINE-GUARD-REFRESH.md"
PLAN="docs/development/current/main/design/mimalloc-hako-port-implementation-plan-ssot.md"
PHASE_README="docs/development/current/main/phases/phase-293x/README.md"
INDEX="docs/tools/check-scripts-index.md"
DEV_GATE="tools/checks/dev_gate.sh"
ALLOCATOR_GATE="tools/checks/k2_wide_allocator_gate.sh"
SELF_SCRIPT="tools/checks/k2_wide_guard_refresh_policy_guard.sh"

echo "[$TAG] checking D196 guard refresh policy"

guard_require_files \
  "$TAG" \
  "$CARD" \
  "$PLAN" \
  "$PHASE_README" \
  "$INDEX" \
  "$DEV_GATE" \
  "$ALLOCATOR_GATE" \
  "$SELF_SCRIPT"

guard_require_exec_files "$TAG" "$SELF_SCRIPT"

guard_expect_in_file "$TAG" 'Status: Complete' "$CARD" "D196 card must be complete"
guard_expect_in_file "$TAG" 'D196 status:' "$PLAN" "mimalloc plan must record D196 status"
guard_expect_in_file "$TAG" '`293x-222`' "$PHASE_README" "phase README must list D196 row"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check script index must list D196 guard"

guard_expect_in_file "$TAG" 'tools/checks/k2_wide_arraybox_inline_record_storage_guard.sh' "$INDEX" "C204b inline-record storage guard must stay indexed"
guard_expect_in_file "$TAG" 'tools/checks/k2_wide_arraybox_inline_record_probe_guard.sh' "$INDEX" "C206b/C206c inline-record probe guard must stay indexed"
guard_expect_in_file "$TAG" 'tools/checks/k2_wide_arraybox_inline_record_plan_probe_guard.sh' "$INDEX" "C206d plan probe guard must stay indexed"

if rg -n 'k2_wide_arraybox_inline_record_(storage|probe|plan_probe)_guard\.sh' \
  "$DEV_GATE" "$ALLOCATOR_GATE" >/tmp/"$TAG".gate_growth 2>&1; then
  echo "[$TAG] ERROR: C206+ cleanup/probe guards must not grow quick/dev or allocator-wide gates by default" >&2
  cat /tmp/"$TAG".gate_growth >&2
  rm -f /tmp/"$TAG".gate_growth
  exit 1
fi
rm -f /tmp/"$TAG".gate_growth

echo "[$TAG] ok"
