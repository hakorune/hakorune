#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-phase295x-mimalloc-abandoned-heap-stress-presentation"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

CARD="docs/development/current/main/phases/phase-295x/295x-217-MIMALLOC-COMPARISON-ABANDONED-HEAP-STRESS-PRESENTATION.md"
PREV_CARD="docs/development/current/main/phases/phase-295x/295x-216-MIMALLOC-COMPARISON-ABANDONED-HEAP-STRESS-EVIDENCE.md"
TASKBOARD="docs/development/current/main/phases/phase-295x/295x-90-mimalloc-comparison-execution-taskboard.md"
INDEX="docs/tools/check-scripts-index.md"
EVIDENCE_RUNNER="tools/allocator/mimalloc_abandoned_heap_stress_evidence_runner.py"
PRESENTATION="tools/allocator/mimalloc_abandoned_heap_stress_presentation.py"
SELF_SCRIPT="tools/checks/k2_wide_phase295x_mimalloc_abandoned_heap_stress_presentation_guard.sh"

echo "[$TAG] checking phase-295x abandoned-heap stress presentation"

guard_require_files "$TAG" "$CARD" "$PREV_CARD" "$TASKBOARD" "$INDEX" "$EVIDENCE_RUNNER" "$PRESENTATION" "$SELF_SCRIPT"
guard_require_exec_files "$TAG" "$EVIDENCE_RUNNER" "$PRESENTATION" "$SELF_SCRIPT"

guard_expect_in_file "$TAG" 'Status: Landed' "$CARD" "card must remain landed after the presentation row is closed"
guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-ABANDONED-HEAP-STRESS-PRESENTATION-295X-002' "$CARD" "card must identify the presentation blocker"
guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-ABANDONED-HEAP-STRESS-CLOSEOUT-295X-002' "$CARD" "card must select closeout follow-on"
guard_expect_in_file "$TAG" 'Status: Landed' "$PREV_CARD" "previous evidence row must be landed before presentation"
guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-ABANDONED-HEAP-STRESS-PRESENTATION-295X-002' "$PREV_CARD" "previous row must select this presentation row"
guard_expect_in_file "$TAG" '| 216 | `MIMALLOC-COMPARISON-ABANDONED-HEAP-STRESS-EVIDENCE-295X-002` | Landed |' "$TASKBOARD" "taskboard must mark the evidence row as landed"
guard_expect_in_file "$TAG" '| 217 | `MIMALLOC-COMPARISON-ABANDONED-HEAP-STRESS-PRESENTATION-295X-002` | Landed |' "$TASKBOARD" "taskboard must mark the presentation row as landed"
guard_expect_in_file "$TAG" '| 218 | `MIMALLOC-COMPARISON-ABANDONED-HEAP-STRESS-CLOSEOUT-295X-002` | Current |' "$TASKBOARD" "taskboard must expose the closeout row as current"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check script index must list this guard"
guard_expect_in_file "$TAG" 'mimalloc_abandoned_heap_stress_evidence_runner=1' "$EVIDENCE_RUNNER" "presentation must depend on the evidence runner"
guard_expect_in_file "$TAG" 'mimalloc_abandoned_heap_stress_presentation=1' "$PRESENTATION" "presentation must publish stable contract"
guard_expect_in_file "$TAG" 'output_contract=mimalloc-comparison-abandoned-heap-stress-presentation-v0' "$PRESENTATION" "presentation must publish presentation contract"

tmp_dir="$(mktemp -d /tmp/hakorune_phase295x_abandoned_heap_stress_presentation.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT
evidence="$tmp_dir/abandoned-heap-stress.evidence"
out="$tmp_dir/abandoned-heap-stress.presentation"

python3 "$EVIDENCE_RUNNER" --out "$evidence" >/dev/null
python3 "$PRESENTATION" --report "$evidence" --out "$out"

rg -F -q 'mimalloc_abandoned_heap_stress_presentation=1' "$out"
rg -F -q 'output_contract=mimalloc-comparison-abandoned-heap-stress-presentation-v0' "$out"
rg -F -q 'input_contract=mimalloc-comparison-abandoned-heap-stress-evidence-v0' "$out"
rg -F -q 'presentation_only=1' "$out"
rg -F -q 'evidence_pair=abandoned-owner-policy+abandoned-reclaim-inventory' "$out"
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
rg -F -q 'provider_activation=0' "$out"
rg -F -q 'host_replacement=0' "$out"
rg -F -q 'hook_installed=0' "$out"
rg -F -q 'global_allocator_installed=0' "$out"
rg -F -q 'winner_claim=0' "$out"
rg -F -q 'summary=ok' "$out"

echo "[$TAG] ok"
