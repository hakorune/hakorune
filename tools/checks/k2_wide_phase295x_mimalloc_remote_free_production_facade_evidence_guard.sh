#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-phase295x-mimalloc-remote-free-production-facade-evidence"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

CARD="docs/development/current/main/phases/phase-295x/295x-243-MIMALLOC-COMPARISON-REMOTE-FREE-PRODUCTION-FACADE-EVIDENCE.md"
PREV_CARD="docs/development/current/main/phases/phase-295x/295x-242-MIMALLOC-COMPARISON-REMOTE-FREE-PRODUCTION-FACADE-CONTRACT-REFRESH.md"
PRESENTATION_CARD="docs/development/current/main/phases/phase-295x/295x-253-MIMALLOC-COMPARISON-REMOTE-FREE-PRODUCTION-FACADE-PRESENTATION.md"
TASKBOARD="docs/development/current/main/phases/phase-295x/295x-90-mimalloc-comparison-execution-taskboard.md"
CURRENT_STATE="docs/development/current/main/CURRENT_STATE.toml"
INDEX="docs/tools/check-scripts-index.md"
RUNNER="tools/allocator/mimalloc_remote_free_production_facade_evidence_runner.py"
SELF_SCRIPT="tools/checks/k2_wide_phase295x_mimalloc_remote_free_production_facade_evidence_guard.sh"

echo "[$TAG] checking remote-free production facade evidence"

guard_require_files "$TAG" "$CARD" "$PREV_CARD" "$PRESENTATION_CARD" "$TASKBOARD" "$CURRENT_STATE" "$INDEX" "$RUNNER" "$SELF_SCRIPT"
guard_require_exec_files "$TAG" "$SELF_SCRIPT"

guard_expect_fixed_in_file "$TAG" 'Status: Landed' "$CARD" "evidence card must be landed"
guard_expect_fixed_in_file "$TAG" 'MIMALLOC-COMPARISON-REMOTE-FREE-PRODUCTION-FACADE-EVIDENCE-295X-002' "$CARD" "evidence blocker must be fixed"
guard_expect_fixed_in_file "$TAG" 'output_contract=mimalloc-comparison-remote-free-production-facade-evidence-v0' "$CARD" "evidence card must define the output contract"
guard_expect_fixed_in_file "$TAG" 'worker identity / TLS cache slots' "$CARD" "evidence card must reference worker/TLS proof surfaces"
guard_expect_fixed_in_file "$TAG" 'pointer atomic routes' "$CARD" "evidence card must reference atomic proof surfaces"
guard_expect_fixed_in_file "$TAG" 'remote-free policy' "$CARD" "evidence card must reference remote-free policy proof surfaces"
guard_expect_fixed_in_file "$TAG" 'remote-abandoned-owner policy' "$CARD" "evidence card must reference abandoned-owner proof surfaces"
guard_expect_fixed_in_file "$TAG" 'remote-free page integration' "$CARD" "evidence card must reference page ownership proof surfaces"
guard_expect_fixed_in_file "$TAG" 'thread-safe hako_mem ABI' "$CARD" "evidence card must reference thread-safe ABI proof surfaces"
guard_expect_fixed_in_file "$TAG" 'native multi-worker stress' "$CARD" "evidence card must reference native stress proof surfaces"
guard_expect_fixed_in_file "$TAG" 'proof_bundle=worker_tls_cache+remote_free_policy+ptr_remote_free_list+remote_abandoned_owner_policy+remote_free_page_integration+threadsafe_abi+native_stress' "$CARD" "evidence card must define the proof bundle"
guard_expect_fixed_in_file "$TAG" 'worker_id' "$CARD" "evidence card must include worker_id"
guard_expect_fixed_in_file "$TAG" 'tls_cache_slot' "$CARD" "evidence card must include tls_cache_slot"
guard_expect_fixed_in_file "$TAG" 'atomic_route' "$CARD" "evidence card must include atomic_route"
guard_expect_fixed_in_file "$TAG" 'remote_pending' "$CARD" "evidence card must include remote_pending"
guard_expect_fixed_in_file "$TAG" 'abandoned_owner' "$CARD" "evidence card must include abandoned_owner"
guard_expect_fixed_in_file "$TAG" 'page_ownership' "$CARD" "evidence card must include page_ownership"
guard_expect_fixed_in_file "$TAG" 'thread_safe_abi' "$CARD" "evidence card must include thread_safe_abi"
guard_expect_fixed_in_file "$TAG" 'provider_active' "$CARD" "evidence card must include provider_active"
guard_expect_fixed_in_file "$TAG" 'replacement_active' "$CARD" "evidence card must include replacement_active"
guard_expect_fixed_in_file "$TAG" 'winner_claim' "$CARD" "evidence card must include winner_claim"
guard_expect_fixed_in_file "$TAG" 'counts=6' "$CARD" "evidence card must keep a compact component count"
guard_expect_fixed_in_file "$TAG" 'MIMALLOC-COMPARISON-REMOTE-FREE-PRODUCTION-FACADE-PRESENTATION-295X-002' "$CARD" "evidence card must select the presentation follow-on"

guard_expect_fixed_in_file "$TAG" 'Status: Landed' "$PREV_CARD" "previous contract refresh row must be landed"
guard_expect_fixed_in_file "$TAG" 'MIMALLOC-COMPARISON-REMOTE-FREE-PRODUCTION-FACADE-EVIDENCE-295X-002' "$PREV_CARD" "previous row must select this evidence row"

guard_expect_fixed_in_file "$TAG" '| 242 | `MIMALLOC-COMPARISON-REMOTE-FREE-PRODUCTION-FACADE-CONTRACT-REFRESH-295X-002` | Landed |' "$TASKBOARD" "taskboard must keep contract refresh landed"
guard_expect_fixed_in_file "$TAG" '| 243 | `MIMALLOC-COMPARISON-REMOTE-FREE-PRODUCTION-FACADE-EVIDENCE-295X-002` | Landed |' "$TASKBOARD" "taskboard must keep evidence row landed"
guard_expect_fixed_in_file "$TAG" '| 253 | `MIMALLOC-COMPARISON-REMOTE-FREE-PRODUCTION-FACADE-PRESENTATION-295X-002` | Current |' "$TASKBOARD" "taskboard must expose presentation row as current"

guard_expect_fixed_in_file "$TAG" '295x-253-MIMALLOC-COMPARISON-REMOTE-FREE-PRODUCTION-FACADE-PRESENTATION' "$CURRENT_STATE" "current state must point at the presentation card"
guard_expect_fixed_in_file "$TAG" 'MIMALLOC-COMPARISON-MALLOC-LARGE-MEMORY-GAP-CLOSEOUT-295X-001' "$CURRENT_STATE" "current state must expose the malloc-large closeout blocker"
guard_expect_fixed_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check script index must list evidence guard"

tmp_dir="$(mktemp -d /tmp/hakorune_remote_free_facade_evidence.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT

report="$tmp_dir/evidence.report"
python3 "$RUNNER" --out "$report"

guard_expect_fixed_in_file "$TAG" 'mimalloc_remote_free_production_facade_evidence_runner=1' "$report" "evidence runner marker missing"
guard_expect_fixed_in_file "$TAG" 'output_contract=mimalloc-comparison-remote-free-production-facade-evidence-v0' "$report" "report must define the evidence contract"
guard_expect_fixed_in_file "$TAG" 'proof_bundle=worker_tls_cache+remote_free_policy+ptr_remote_free_list+remote_abandoned_owner_policy+remote_free_page_integration+threadsafe_abi+native_stress' "$report" "report must keep the proof bundle"
guard_expect_fixed_in_file "$TAG" 'worker_id=0' "$report" "report must keep worker_id"
guard_expect_fixed_in_file "$TAG" 'tls_cache_slot=0' "$report" "report must keep tls_cache_slot"
guard_expect_fixed_in_file "$TAG" 'atomic_route=ptr_store_load_cas' "$report" "report must keep the atomic route"
guard_expect_fixed_in_file "$TAG" 'remote_pending=0,6,4,3' "$report" "report must keep remote_pending"
guard_expect_fixed_in_file "$TAG" 'abandoned_owner=3,1,1,1,1' "$report" "report must keep abandoned_owner"
guard_expect_fixed_in_file "$TAG" 'page_ownership=0,2,1,2' "$report" "report must keep page ownership"
guard_expect_fixed_in_file "$TAG" 'thread_safe_abi=1' "$report" "report must keep thread_safe_abi"
guard_expect_fixed_in_file "$TAG" 'native_multi_worker_stress=1' "$report" "report must keep native stress evidence"
guard_expect_fixed_in_file "$TAG" 'worker_count=4' "$report" "report must keep worker_count"
guard_expect_fixed_in_file "$TAG" 'observed_remote_free_count=256' "$report" "report must keep remote free count"
guard_expect_fixed_in_file "$TAG" 'provider_active=0' "$report" "report must keep provider seam closed"
guard_expect_fixed_in_file "$TAG" 'replacement_active=0' "$report" "report must keep replacement seam closed"
guard_expect_fixed_in_file "$TAG" 'winner_claim=0' "$report" "report must keep winner claim closed"
guard_expect_fixed_in_file "$TAG" 'counts=6' "$report" "report must keep compact component count"
guard_expect_fixed_in_file "$TAG" 'summary=ok' "$report" "report must finish cleanly"

echo "[$TAG] ok"
