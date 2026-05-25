#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-phase295x-mimalloc-abandoned-heap-stress-contract-refresh"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

CARD="docs/development/current/main/phases/phase-295x/295x-215-MIMALLOC-COMPARISON-ABANDONED-HEAP-STRESS-CONTRACT-REFRESH.md"
PREV_CARD="docs/development/current/main/phases/phase-295x/295x-214-MIMALLOC-COMPARISON-ABANDONED-HEAP-STRESS-SELECTION.md"
TASKBOARD="docs/development/current/main/phases/phase-295x/295x-90-mimalloc-comparison-execution-taskboard.md"
INDEX="docs/tools/check-scripts-index.md"
REMOTE_APP="apps/mimalloc-remote-abandoned-owner-policy-proof/main.hako"
REMOTE_README="apps/mimalloc-remote-abandoned-owner-policy-proof/README.md"
HKO_APP="apps/hako-alloc-abandoned-reclaim-inventory-proof/main.hako"
HKO_README="apps/hako-alloc-abandoned-reclaim-inventory-proof/README.md"
REMOTE_GUARD="tools/checks/k2_wide_mimalloc_remote_abandoned_owner_policy_guard.sh"
HKO_GUARD="tools/checks/k2_wide_hako_alloc_abandoned_reclaim_inventory_guard.sh"
SELF_SCRIPT="tools/checks/k2_wide_phase295x_mimalloc_abandoned_heap_stress_contract_refresh_guard.sh"

echo "[$TAG] checking phase-295x abandoned-heap stress contract refresh"

guard_require_files \
  "$TAG" \
  "$CARD" \
  "$PREV_CARD" \
  "$TASKBOARD" \
  "$INDEX" \
  "$REMOTE_APP" \
  "$REMOTE_README" \
  "$HKO_APP" \
  "$HKO_README" \
  "$REMOTE_GUARD" \
  "$HKO_GUARD" \
  "$SELF_SCRIPT"
guard_require_exec_files "$TAG" "$REMOTE_GUARD" "$HKO_GUARD" "$SELF_SCRIPT"

guard_expect_in_file "$TAG" 'Status: Current' "$CARD" "card must remain current while the contract refresh row is being exercised"
guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-ABANDONED-HEAP-STRESS-CONTRACT-REFRESH-295X-002' "$CARD" "card must identify the contract refresh blocker"
guard_expect_in_file "$TAG" 'output_contract=mimalloc-comparison-abandoned-heap-stress-contract-v0' "$CARD" "card must name the comparison contract"
guard_expect_in_file "$TAG" 'mimalloc-remote-abandoned-owner-policy-proof' "$CARD" "card must reference the mimalloc abandoned-owner policy proof"
guard_expect_in_file "$TAG" 'hako-alloc-abandoned-reclaim-inventory-proof' "$CARD" "card must reference the hako abandoned-reclaim inventory proof"
guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-ABANDONED-HEAP-STRESS-EVIDENCE-295X-002' "$CARD" "card must select the evidence follow-on"
guard_expect_in_file "$TAG" 'Status: Landed' "$PREV_CARD" "previous selection row must be landed before contract refresh"
guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-ABANDONED-HEAP-STRESS-CONTRACT-REFRESH-295X-002' "$PREV_CARD" "previous row must select this contract refresh row"
guard_expect_in_file "$TAG" '| 214 | `MIMALLOC-COMPARISON-ABANDONED-HEAP-STRESS-SELECTION-295X-002` | Landed |' "$TASKBOARD" "taskboard must mark the selection row as landed"
guard_expect_in_file "$TAG" '| 215 | `MIMALLOC-COMPARISON-ABANDONED-HEAP-STRESS-CONTRACT-REFRESH-295X-002` | Current |' "$TASKBOARD" "taskboard must expose the contract refresh row as current"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check script index must list this guard"

bash "$REMOTE_GUARD"
bash "$HKO_GUARD"

echo "[$TAG] ok"
