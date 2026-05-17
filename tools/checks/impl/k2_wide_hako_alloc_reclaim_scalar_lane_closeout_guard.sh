#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../../.." && pwd)"
TAG="k2-wide-hako-alloc-reclaim-scalar-lane-closeout"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

SSOT="docs/development/current/main/design/hako-alloc-reclaim-scalar-lane-closeout-ssot.md"
TASKBOARD="docs/development/current/main/phases/phase-293x/293x-mimalloc-port-taskboard.md"
GRANULARITY="docs/development/current/main/design/mimalloc-allocator-first-task-granularity-ssot.md"
JOINT="docs/development/current/main/design/mimalloc-hakorune-joint-task-order-ssot.md"
INDEX="docs/tools/check-scripts-index.md"
PROOF_MANIFEST="tools/checks/proof_apps.toml"
MODULE="lang/src/hako_alloc/hako_module.toml"
MEMORY_README="lang/src/hako_alloc/memory/README.md"
CARD_051A="docs/development/current/main/phases/phase-293x/293x-535-MIMAP-051A-RECLAIM-OWNER-TRANSFER-CONTRACT-INVENTORY.md"
CARD_054A="docs/development/current/main/phases/phase-293x/293x-541-MIMAP-054A-RECLAIM-ATOMIC-CLAIM-CONTRACT.md"
CARD_055A="docs/development/current/main/phases/phase-293x/293x-542-MIMAP-055A-RECLAIM-OWNER-TRANSFER-FIRST-EXECUTION-ROUTE.md"
CARD_056A="docs/development/current/main/phases/phase-293x/293x-543-MIMAP-056A-RECLAIM-REMOTE-FREE-DRAIN-CONTRACT-INVENTORY.md"
CARD_057A="docs/development/current/main/phases/phase-293x/293x-544-MIMAP-057A-RECLAIM-REMOTE-FREE-DRAIN-FIRST-EXECUTION-ROUTE.md"
CARD_058A="docs/development/current/main/phases/phase-293x/293x-545-MIMAP-058A-RECLAIM-POST-DRAIN-OWNER-TRANSFER-INTEGRATION-ROUTE.md"
CARD_060A="docs/development/current/main/phases/phase-293x/293x-547-MIMAP-060A-RECLAIM-COMPLETION-MARKER-ROUTE.md"
CARD_061A="docs/development/current/main/phases/phase-293x/293x-548-MIMAP-061A-RECLAIM-SCALAR-LANE-CLOSEOUT-GUARD.md"
CARD_062A="docs/development/current/main/phases/phase-293x/293x-549-MIMAP-062A-POST-RECLAIM-SCALAR-CLOSEOUT-ROW-SELECTION.md"
OWNER_051A="lang/src/hako_alloc/memory/reclaim_owner_transfer_contract_box.hako"
OWNER_054A="lang/src/hako_alloc/memory/reclaim_atomic_claim_contract_box.hako"
OWNER_055A="lang/src/hako_alloc/memory/reclaim_owner_transfer_execution_box.hako"
OWNER_056A="lang/src/hako_alloc/memory/reclaim_remote_free_drain_contract_box.hako"
OWNER_057A="lang/src/hako_alloc/memory/reclaim_remote_free_drain_execution_box.hako"
OWNER_058A="lang/src/hako_alloc/memory/reclaim_post_drain_owner_transfer_box.hako"
OWNER_060A="lang/src/hako_alloc/memory/reclaim_completion_marker_box.hako"
GUARD_051A="tools/checks/k2_wide_hako_alloc_reclaim_owner_transfer_contract_guard.sh"
GUARD_054A="tools/checks/k2_wide_hako_alloc_reclaim_atomic_claim_contract_guard.sh"
GUARD_055A="tools/checks/k2_wide_hako_alloc_reclaim_owner_transfer_execution_guard.sh"
GUARD_056A="tools/checks/k2_wide_hako_alloc_reclaim_remote_free_drain_contract_guard.sh"
GUARD_057A="tools/checks/k2_wide_hako_alloc_reclaim_remote_free_drain_execution_guard.sh"
GUARD_058A="tools/checks/k2_wide_hako_alloc_reclaim_post_drain_owner_transfer_guard.sh"
GUARD_060A="tools/checks/k2_wide_hako_alloc_reclaim_completion_marker_guard.sh"
SELF_SCRIPT="tools/checks/k2_wide_hako_alloc_reclaim_scalar_lane_closeout_guard.sh"

echo "[$TAG] checking MIMAP-061A scalar reclaim lane closeout"

guard_require_files \
  "$TAG" \
  "$SSOT" \
  "$TASKBOARD" \
  "$GRANULARITY" \
  "$JOINT" \
  "$INDEX" \
  "$PROOF_MANIFEST" \
  "$MODULE" \
  "$MEMORY_README" \
  "$CARD_051A" \
  "$CARD_054A" \
  "$CARD_055A" \
  "$CARD_056A" \
  "$CARD_057A" \
  "$CARD_058A" \
  "$CARD_060A" \
  "$CARD_061A" \
  "$CARD_062A" \
  "$OWNER_051A" \
  "$OWNER_054A" \
  "$OWNER_055A" \
  "$OWNER_056A" \
  "$OWNER_057A" \
  "$OWNER_058A" \
  "$OWNER_060A" \
  "$GUARD_051A" \
  "$GUARD_054A" \
  "$GUARD_055A" \
  "$GUARD_056A" \
  "$GUARD_057A" \
  "$GUARD_058A" \
  "$GUARD_060A" \
  "$SELF_SCRIPT"

guard_require_exec_files \
  "$TAG" \
  "$GUARD_051A" \
  "$GUARD_054A" \
  "$GUARD_055A" \
  "$GUARD_056A" \
  "$GUARD_057A" \
  "$GUARD_058A" \
  "$GUARD_060A" \
  "$SELF_SCRIPT"

for card in "$CARD_051A" "$CARD_054A" "$CARD_055A" "$CARD_056A" "$CARD_057A" "$CARD_058A" "$CARD_060A" "$CARD_061A"; do
  guard_expect_in_file "$TAG" "Status: landed" "$card" "$card must be landed"
done
guard_expect_in_file "$TAG" "Status:" "$CARD_062A" "MIMAP-062A selection card must have status"
guard_expect_in_file "$TAG" "MIMAP-062A" "$CARD_062A" "MIMAP-062A selection card must stay present after closeout"

for row in MIMAP-051A MIMAP-054A MIMAP-055A MIMAP-056A MIMAP-057A MIMAP-058A MIMAP-060A; do
  guard_expect_in_file "$TAG" "$row" "$SSOT" "closeout SSOT must include $row"
  guard_expect_in_file "$TAG" "id = \"$row\"" "$PROOF_MANIFEST" "proof manifest must include $row"
done
guard_expect_in_file "$TAG" "MIMAP-062A post-reclaim-scalar-closeout row selection" "$SSOT" "closeout SSOT must name next selection row"
guard_expect_in_file "$TAG" "MIMAP-061A granularity" "$GRANULARITY" "granularity SSOT must describe MIMAP-061A"
guard_expect_in_file "$TAG" "MIMAP-061A reclaim scalar lane closeout guard" "$JOINT" "joint order must name closeout row"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list MIMAP-061A closeout guard"

guard_expect_in_file "$TAG" 'memory.reclaim_owner_transfer_contract_box = "memory/reclaim_owner_transfer_contract_box.hako"' "$MODULE" "MIMAP-051A owner must stay exported"
guard_expect_in_file "$TAG" 'memory.reclaim_atomic_claim_contract_box = "memory/reclaim_atomic_claim_contract_box.hako"' "$MODULE" "MIMAP-054A owner must stay exported"
guard_expect_in_file "$TAG" 'memory.reclaim_owner_transfer_execution_box = "memory/reclaim_owner_transfer_execution_box.hako"' "$MODULE" "MIMAP-055A owner must stay exported"
guard_expect_in_file "$TAG" 'memory.reclaim_remote_free_drain_contract_box = "memory/reclaim_remote_free_drain_contract_box.hako"' "$MODULE" "MIMAP-056A owner must stay exported"
guard_expect_in_file "$TAG" 'memory.reclaim_remote_free_drain_execution_box = "memory/reclaim_remote_free_drain_execution_box.hako"' "$MODULE" "MIMAP-057A owner must stay exported"
guard_expect_in_file "$TAG" 'memory.reclaim_post_drain_owner_transfer_box = "memory/reclaim_post_drain_owner_transfer_box.hako"' "$MODULE" "MIMAP-058A owner must stay exported"
guard_expect_in_file "$TAG" 'memory.reclaim_completion_marker_box = "memory/reclaim_completion_marker_box.hako"' "$MODULE" "MIMAP-060A owner must stay exported"

guard_expect_in_file "$TAG" 'reclaim_owner_transfer_contract_box.hako` owns MIMAP-051A' "$MEMORY_README" "memory README must name MIMAP-051A owner"
guard_expect_in_file "$TAG" 'reclaim_atomic_claim_contract_box.hako` owns MIMAP-054A' "$MEMORY_README" "memory README must name MIMAP-054A owner"
guard_expect_in_file "$TAG" 'reclaim_owner_transfer_execution_box.hako` owns MIMAP-055A' "$MEMORY_README" "memory README must name MIMAP-055A owner"
guard_expect_in_file "$TAG" 'reclaim_remote_free_drain_contract_box.hako` owns MIMAP-056A' "$MEMORY_README" "memory README must name MIMAP-056A owner"
guard_expect_in_file "$TAG" 'reclaim_remote_free_drain_execution_box.hako` owns MIMAP-057A' "$MEMORY_README" "memory README must name MIMAP-057A owner"
guard_expect_in_file "$TAG" 'reclaim_post_drain_owner_transfer_box.hako` owns MIMAP-058A' "$MEMORY_README" "memory README must name MIMAP-058A owner"
guard_expect_in_file "$TAG" 'reclaim_completion_marker_box.hako` owns MIMAP-060A' "$MEMORY_README" "memory README must name MIMAP-060A owner"

for guard in "$GUARD_051A" "$GUARD_054A" "$GUARD_055A" "$GUARD_056A" "$GUARD_057A" "$GUARD_058A" "$GUARD_060A"; do
  guard_expect_in_file "$TAG" "$guard" "$INDEX" "check index must list $guard"
done

if rg -n 'hako-alloc-reclaim-(owner-transfer|atomic-claim|remote-free-drain|post-drain-owner-transfer|completion-marker)|HakoAllocReclaim(OwnerTransfer|AtomicClaim|RemoteFreeDrain|PostDrainOwnerTransfer|CompletionMarker)|reclaim_(owner_transfer|atomic_claim|remote_free_drain|post_drain_owner_transfer|completion_marker)' \
  lang/c-abi/shims >/tmp/"$TAG".inc_leak 2>&1; then
  cat /tmp/"$TAG".inc_leak >&2
  rm -f /tmp/"$TAG".inc_leak
  guard_fail "$TAG" "scalar reclaim app/owner matcher leaked into .inc"
fi
rm -f /tmp/"$TAG".inc_leak

if rg -n 'HakoAllocPageSourcePolicy|OsVmCoreBox|hako_osvm_(unreserve|release)|unreserve_bytes_i64|releasePage[[:space:]]*\(|spawn[[:space:]]*\(|thread::|global_allocator|GlobalAlloc|replace_allocator' \
  "$OWNER_051A" "$OWNER_054A" "$OWNER_055A" "$OWNER_056A" "$OWNER_057A" "$OWNER_058A" "$OWNER_060A" >/tmp/"$TAG".stop_line_leak 2>&1; then
  cat /tmp/"$TAG".stop_line_leak >&2
  rm -f /tmp/"$TAG".stop_line_leak
  guard_fail "$TAG" "scalar reclaim owners must keep page-source/OS release/scheduling/provider replacement inactive"
fi
rm -f /tmp/"$TAG".stop_line_leak

echo "[$TAG] ok"
