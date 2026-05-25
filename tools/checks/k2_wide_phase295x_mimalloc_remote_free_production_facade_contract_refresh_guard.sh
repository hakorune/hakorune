#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-phase295x-mimalloc-remote-free-production-facade-contract-refresh"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

CARD="docs/development/current/main/phases/phase-295x/295x-242-MIMALLOC-COMPARISON-REMOTE-FREE-PRODUCTION-FACADE-CONTRACT-REFRESH.md"
PREV_CARD="docs/development/current/main/phases/phase-295x/295x-241-MIMALLOC-COMPARISON-REMOTE-FREE-PRODUCTION-FACADE-SELECTION.md"
TASKBOARD="docs/development/current/main/phases/phase-295x/295x-90-mimalloc-comparison-execution-taskboard.md"
CURRENT_STATE="docs/development/current/main/CURRENT_STATE.toml"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase295x_mimalloc_remote_free_production_facade_contract_refresh_guard.sh"

echo "[$TAG] checking remote-free production facade contract refresh"

guard_require_files "$TAG" "$CARD" "$PREV_CARD" "$TASKBOARD" "$CURRENT_STATE" "$INDEX" "$SELF_SCRIPT"
guard_require_exec_files "$TAG" "$SELF_SCRIPT"

guard_expect_fixed_in_file "$TAG" 'Status: Current' "$CARD" "contract refresh card must be current"
guard_expect_fixed_in_file "$TAG" 'MIMALLOC-COMPARISON-REMOTE-FREE-PRODUCTION-FACADE-CONTRACT-REFRESH-295X-002' "$CARD" "contract refresh blocker must be fixed"
guard_expect_fixed_in_file "$TAG" 'output_contract=mimalloc-comparison-remote-free-production-facade-contract-v0' "$CARD" "contract refresh must define the output contract"
guard_expect_fixed_in_file "$TAG" 'worker_id' "$CARD" "contract refresh must include worker_id"
guard_expect_fixed_in_file "$TAG" 'tls_cache_slot' "$CARD" "contract refresh must include tls_cache_slot"
guard_expect_fixed_in_file "$TAG" 'atomic_route' "$CARD" "contract refresh must include atomic_route"
guard_expect_fixed_in_file "$TAG" 'remote_pending' "$CARD" "contract refresh must include remote_pending"
guard_expect_fixed_in_file "$TAG" 'abandoned_owner' "$CARD" "contract refresh must include abandoned_owner"
guard_expect_fixed_in_file "$TAG" 'page_ownership' "$CARD" "contract refresh must include page_ownership"
guard_expect_fixed_in_file "$TAG" 'thread_safe_abi' "$CARD" "contract refresh must include thread_safe_abi"
guard_expect_fixed_in_file "$TAG" 'provider_active' "$CARD" "contract refresh must include provider_active"
guard_expect_fixed_in_file "$TAG" 'replacement_active' "$CARD" "contract refresh must include replacement_active"
guard_expect_fixed_in_file "$TAG" 'winner_claim' "$CARD" "contract refresh must include winner_claim"
guard_expect_fixed_in_file "$TAG" 'MIMALLOC-COMPARISON-REMOTE-FREE-PRODUCTION-FACADE-EVIDENCE-295X-002' "$CARD" "contract refresh must select evidence follow-on"
guard_expect_fixed_in_file "$TAG" 'This row does not open provider activation, DLL/replacement/hook/global' "$CARD" "contract refresh must keep provider seams closed"

guard_expect_fixed_in_file "$TAG" 'Status: Landed' "$PREV_CARD" "previous remote-free selection row must be landed"
guard_expect_fixed_in_file "$TAG" 'MIMALLOC-COMPARISON-REMOTE-FREE-PRODUCTION-FACADE-CONTRACT-REFRESH-295X-002' "$PREV_CARD" "previous row must select this contract refresh"

guard_expect_fixed_in_file "$TAG" '| 241 | `MIMALLOC-COMPARISON-REMOTE-FREE-PRODUCTION-FACADE-SELECTION-295X-002` | Landed |' "$TASKBOARD" "taskboard must keep remote-free selection landed"
guard_expect_fixed_in_file "$TAG" '| 242 | `MIMALLOC-COMPARISON-REMOTE-FREE-PRODUCTION-FACADE-CONTRACT-REFRESH-295X-002` | Current |' "$TASKBOARD" "taskboard must expose contract refresh as current"

guard_expect_fixed_in_file "$TAG" '295x-242-MIMALLOC-COMPARISON-REMOTE-FREE-PRODUCTION-FACADE-CONTRACT-REFRESH' "$CURRENT_STATE" "current state must point at the contract refresh card"
guard_expect_fixed_in_file "$TAG" 'MIMALLOC-COMPARISON-REMOTE-FREE-PRODUCTION-FACADE-CONTRACT-REFRESH-295X-002' "$CURRENT_STATE" "current state must expose the contract refresh blocker"
guard_expect_fixed_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check script index must list contract refresh guard"

echo "[$TAG] ok"
