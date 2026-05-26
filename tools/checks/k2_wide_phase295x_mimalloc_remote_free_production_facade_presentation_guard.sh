#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-phase295x-mimalloc-remote-free-production-facade-presentation"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

CARD="docs/development/current/main/phases/phase-295x/295x-253-MIMALLOC-COMPARISON-REMOTE-FREE-PRODUCTION-FACADE-PRESENTATION.md"
PREV_CARD="docs/development/current/main/phases/phase-295x/295x-243-MIMALLOC-COMPARISON-REMOTE-FREE-PRODUCTION-FACADE-EVIDENCE.md"
TASKBOARD="docs/development/current/main/phases/phase-295x/295x-90-mimalloc-comparison-execution-taskboard.md"
CURRENT_STATE="docs/development/current/main/CURRENT_STATE.toml"
INDEX="docs/tools/check-scripts-index.md"
EVIDENCE_RUNNER="tools/allocator/mimalloc_remote_free_production_facade_evidence_runner.py"
PRESENTATION="tools/allocator/mimalloc_remote_free_production_facade_presentation.py"
SELF_SCRIPT="tools/checks/k2_wide_phase295x_mimalloc_remote_free_production_facade_presentation_guard.sh"

echo "[$TAG] checking remote-free production facade presentation"

guard_require_files "$TAG" "$CARD" "$PREV_CARD" "$TASKBOARD" "$CURRENT_STATE" "$INDEX" "$EVIDENCE_RUNNER" "$PRESENTATION" "$SELF_SCRIPT"
guard_require_exec_files "$TAG" "$EVIDENCE_RUNNER" "$PRESENTATION" "$SELF_SCRIPT"

guard_expect_fixed_in_file "$TAG" 'Status: Current' "$CARD" "presentation card must be current"
guard_expect_fixed_in_file "$TAG" 'MIMALLOC-COMPARISON-REMOTE-FREE-PRODUCTION-FACADE-PRESENTATION-295X-002' "$CARD" "presentation blocker must be fixed"
guard_expect_fixed_in_file "$TAG" 'output_contract=mimalloc-comparison-remote-free-production-facade-presentation-v0' "$CARD" "presentation card must define the output contract"
guard_expect_fixed_in_file "$TAG" 'input_contract=mimalloc-comparison-remote-free-production-facade-evidence-v0' "$CARD" "presentation card must consume the evidence contract"
guard_expect_fixed_in_file "$TAG" 'presentation_only=1' "$CARD" "presentation card must stay presentation-only"
guard_expect_fixed_in_file "$TAG" 'proof_bundle=worker_tls_cache+remote_free_policy+ptr_remote_free_list+remote_abandoned_owner_policy+remote_free_page_integration+threadsafe_abi+native_stress' "$CARD" "presentation card must keep the proof bundle visible"
guard_expect_fixed_in_file "$TAG" 'worker_id=0' "$CARD" "presentation card must keep worker_id visible"
guard_expect_fixed_in_file "$TAG" 'tls_cache_slot=0' "$CARD" "presentation card must keep tls cache visible"
guard_expect_fixed_in_file "$TAG" 'atomic_route=ptr_store_load_cas' "$CARD" "presentation card must keep atomic route visible"
guard_expect_fixed_in_file "$TAG" 'remote_pending=0,6,4,3' "$CARD" "presentation card must keep remote pending visible"
guard_expect_fixed_in_file "$TAG" 'abandoned_owner=3,1,1,1,1' "$CARD" "presentation card must keep abandoned owner visible"
guard_expect_fixed_in_file "$TAG" 'page_ownership=0,2,1,2' "$CARD" "presentation card must keep page ownership visible"
guard_expect_fixed_in_file "$TAG" 'thread_safe_abi=1' "$CARD" "presentation card must keep thread-safe ABI visible"
guard_expect_fixed_in_file "$TAG" 'native_multi_worker_stress=1' "$CARD" "presentation card must keep native stress visible"
guard_expect_fixed_in_file "$TAG" 'provider_active=0' "$CARD" "presentation card must keep provider seam closed"
guard_expect_fixed_in_file "$TAG" 'replacement_active=0' "$CARD" "presentation card must keep replacement seam closed"
guard_expect_fixed_in_file "$TAG" 'winner_claim=0' "$CARD" "presentation card must keep winner claim closed"
guard_expect_fixed_in_file "$TAG" 'counts=6' "$CARD" "presentation card must keep compact component count"
guard_expect_fixed_in_file "$TAG" 'MIMALLOC-COMPARISON-MALLOC-LARGE-MEMORY-GAP-CLOSEOUT-295X-001' "$CARD" "presentation card must return to the malloc-large closeout seam"

guard_expect_fixed_in_file "$TAG" 'Status: Landed' "$PREV_CARD" "previous evidence row must be landed"
guard_expect_fixed_in_file "$TAG" 'MIMALLOC-COMPARISON-REMOTE-FREE-PRODUCTION-FACADE-PRESENTATION-295X-002' "$PREV_CARD" "previous row must select this presentation row"

guard_expect_fixed_in_file "$TAG" '| 243 | `MIMALLOC-COMPARISON-REMOTE-FREE-PRODUCTION-FACADE-EVIDENCE-295X-002` | Landed |' "$TASKBOARD" "taskboard must mark the evidence row as landed"
guard_expect_fixed_in_file "$TAG" '| 253 | `MIMALLOC-COMPARISON-REMOTE-FREE-PRODUCTION-FACADE-PRESENTATION-295X-002` | Current |' "$TASKBOARD" "taskboard must expose the presentation row as current"

guard_expect_fixed_in_file "$TAG" '295x-253-MIMALLOC-COMPARISON-REMOTE-FREE-PRODUCTION-FACADE-PRESENTATION' "$CURRENT_STATE" "current state must point at the presentation card"
guard_expect_fixed_in_file "$TAG" 'MIMALLOC-COMPARISON-MALLOC-LARGE-MEMORY-GAP-CLOSEOUT-295X-001' "$CURRENT_STATE" "current state must expose the malloc-large closeout blocker"
guard_expect_fixed_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check script index must list presentation guard"

tmp_dir="$(mktemp -d /tmp/hakorune_remote_free_facade_presentation.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT

evidence_report="$tmp_dir/evidence.report"
presentation_report="$tmp_dir/presentation.report"
python3 "$EVIDENCE_RUNNER" --out "$evidence_report"
python3 "$PRESENTATION" --report "$evidence_report" --out "$presentation_report"

guard_expect_fixed_in_file "$TAG" 'mimalloc_remote_free_production_facade_presentation=1' "$presentation_report" "presentation report marker missing"
guard_expect_fixed_in_file "$TAG" 'output_contract=mimalloc-comparison-remote-free-production-facade-presentation-v0' "$presentation_report" "presentation report must define the output contract"
guard_expect_fixed_in_file "$TAG" 'input_contract=mimalloc-comparison-remote-free-production-facade-evidence-v0' "$presentation_report" "presentation report must consume the evidence contract"
guard_expect_fixed_in_file "$TAG" 'presentation_only=1' "$presentation_report" "presentation report must stay presentation-only"
guard_expect_fixed_in_file "$TAG" 'proof_bundle=worker_tls_cache+remote_free_policy+ptr_remote_free_list+remote_abandoned_owner_policy+remote_free_page_integration+threadsafe_abi+native_stress' "$presentation_report" "presentation report must keep the proof bundle"
guard_expect_fixed_in_file "$TAG" 'worker_id=0' "$presentation_report" "presentation report must keep worker_id"
guard_expect_fixed_in_file "$TAG" 'tls_cache_slot=0' "$presentation_report" "presentation report must keep tls_cache_slot"
guard_expect_fixed_in_file "$TAG" 'atomic_route=ptr_store_load_cas' "$presentation_report" "presentation report must keep atomic route"
guard_expect_fixed_in_file "$TAG" 'remote_pending=0,6,4,3' "$presentation_report" "presentation report must keep remote_pending"
guard_expect_fixed_in_file "$TAG" 'abandoned_owner=3,1,1,1,1' "$presentation_report" "presentation report must keep abandoned_owner"
guard_expect_fixed_in_file "$TAG" 'page_ownership=0,2,1,2' "$presentation_report" "presentation report must keep page ownership"
guard_expect_fixed_in_file "$TAG" 'thread_safe_abi=1' "$presentation_report" "presentation report must keep thread_safe_abi"
guard_expect_fixed_in_file "$TAG" 'native_multi_worker_stress=1' "$presentation_report" "presentation report must keep native stress evidence"
guard_expect_fixed_in_file "$TAG" 'worker_count=4' "$presentation_report" "presentation report must keep worker_count"
guard_expect_fixed_in_file "$TAG" 'observed_remote_free_count=256' "$presentation_report" "presentation report must keep remote free count"
guard_expect_fixed_in_file "$TAG" 'provider_active=0' "$presentation_report" "presentation report must keep provider seam closed"
guard_expect_fixed_in_file "$TAG" 'replacement_active=0' "$presentation_report" "presentation report must keep replacement seam closed"
guard_expect_fixed_in_file "$TAG" 'winner_claim=0' "$presentation_report" "presentation report must keep winner claim closed"
guard_expect_fixed_in_file "$TAG" 'counts=6' "$presentation_report" "presentation report must keep compact component count"
guard_expect_fixed_in_file "$TAG" 'summary=ok' "$presentation_report" "presentation report must finish cleanly"

echo "[$TAG] ok"
