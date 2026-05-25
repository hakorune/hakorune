#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-phase295x-mimalloc-abandoned-heap-stress-evidence"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

CARD="docs/development/current/main/phases/phase-295x/295x-216-MIMALLOC-COMPARISON-ABANDONED-HEAP-STRESS-EVIDENCE.md"
PREV_CARD="docs/development/current/main/phases/phase-295x/295x-215-MIMALLOC-COMPARISON-ABANDONED-HEAP-STRESS-CONTRACT-REFRESH.md"
TASKBOARD="docs/development/current/main/phases/phase-295x/295x-90-mimalloc-comparison-execution-taskboard.md"
INDEX="docs/tools/check-scripts-index.md"
RUNNER="tools/allocator/mimalloc_abandoned_heap_stress_evidence_runner.py"
SELF_SCRIPT="tools/checks/k2_wide_phase295x_mimalloc_abandoned_heap_stress_evidence_guard.sh"

echo "[$TAG] checking phase-295x abandoned-heap stress evidence"

guard_require_files "$TAG" "$CARD" "$PREV_CARD" "$TASKBOARD" "$INDEX" "$RUNNER" "$SELF_SCRIPT"
guard_require_exec_files "$TAG" "$RUNNER" "$SELF_SCRIPT"

guard_expect_in_file "$TAG" 'Status: Current' "$CARD" "card must remain current while the evidence row is being exercised"
guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-ABANDONED-HEAP-STRESS-EVIDENCE-295X-002' "$CARD" "card must identify the evidence blocker"
guard_expect_in_file "$TAG" 'output_contract=mimalloc-comparison-abandoned-heap-stress-evidence-v0' "$CARD" "card must name the evidence contract"
guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-ABANDONED-HEAP-STRESS-PRESENTATION-295X-002' "$CARD" "card must select the presentation follow-on"
guard_expect_in_file "$TAG" 'Status: Landed' "$PREV_CARD" "previous contract refresh row must be landed before evidence"
guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-ABANDONED-HEAP-STRESS-EVIDENCE-295X-002' "$PREV_CARD" "previous row must select this evidence row"
guard_expect_in_file "$TAG" '| 215 | `MIMALLOC-COMPARISON-ABANDONED-HEAP-STRESS-CONTRACT-REFRESH-295X-002` | Landed |' "$TASKBOARD" "taskboard must mark the contract refresh row as landed"
guard_expect_in_file "$TAG" '| 216 | `MIMALLOC-COMPARISON-ABANDONED-HEAP-STRESS-EVIDENCE-295X-002` | Current |' "$TASKBOARD" "taskboard must expose the evidence row as current"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check script index must list this guard"

tmp_dir="$(mktemp -d /tmp/hakorune_phase295x_abandoned_heap_stress_evidence.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT
out="$tmp_dir/abandoned-heap-stress-evidence.out"

python3 "$RUNNER" --out "$out"

rg -F -q 'mimalloc_abandoned_heap_stress_evidence_runner=1' "$out"
rg -F -q 'output_contract=mimalloc-comparison-abandoned-heap-stress-evidence-v0' "$out"
rg -F -q 'evidence_pair=abandoned-owner-policy+abandoned-reclaim-inventory' "$out"
rg -F -q 'remote_proof_guard=k2_wide_mimalloc_remote_abandoned_owner_policy_guard.sh' "$out"
rg -F -q 'reclaim_proof_guard=k2_wide_hako_alloc_abandoned_reclaim_inventory_guard.sh' "$out"
rg -F -q 'remote_same=1,1,0' "$out"
rg -F -q 'remote_remote=2,1,1,0' "$out"
rg -F -q 'remote_abandoned=3,1,1,1,1' "$out"
rg -F -q 'remote_pending=0,6,4,3' "$out"
rg -F -q 'remote_counts=4,1,1,1,1' "$out"
rg -F -q 'remote_mailbox=0,0,0' "$out"
rg -F -q 'remote_shape=9' "$out"
rg -F -q 'reclaim_missing=0,1,10,0' "$out"
rg -F -q 'reclaim_active_owner=0,2,0,1' "$out"
rg -F -q 'reclaim_same_owner=0,2,2,2' "$out"
rg -F -q 'reclaim_remote_pending=0,3,3' "$out"
rg -F -q 'reclaim_decommitted=0,4,1' "$out"
rg -F -q 'reclaim_live=1,0,1,1,1,0' "$out"
rg -F -q 'reclaim_retired=1,0,1,1,1' "$out"
rg -F -q 'reclaim_would=0,0,0,0,0,0' "$out"
rg -F -q 'reclaim_counts=7,2,5,1,2,1,1,1,1,1,16,0' "$out"
rg -F -q 'proof_pair_summary=ok' "$out"
rg -F -q 'provider_activation=0' "$out"
rg -F -q 'host_replacement=0' "$out"
rg -F -q 'hook_installed=0' "$out"
rg -F -q 'global_allocator_installed=0' "$out"
rg -F -q 'winner_claim=0' "$out"
rg -F -q 'summary=ok' "$out"

echo "[$TAG] ok"
