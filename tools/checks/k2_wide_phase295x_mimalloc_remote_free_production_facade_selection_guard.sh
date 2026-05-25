#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-phase295x-mimalloc-remote-free-production-facade-selection"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

CARD="docs/development/current/main/phases/phase-295x/295x-241-MIMALLOC-COMPARISON-REMOTE-FREE-PRODUCTION-FACADE-SELECTION.md"
PREV_CARD="docs/development/current/main/phases/phase-295x/295x-240-MIMALLOC-COMPARISON-NEXT-SEMANTIC-SEAM-SELECTION.md"
TASKBOARD="docs/development/current/main/phases/phase-295x/295x-90-mimalloc-comparison-execution-taskboard.md"
CURRENT_STATE="docs/development/current/main/CURRENT_STATE.toml"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase295x_mimalloc_remote_free_production_facade_selection_guard.sh"

echo "[$TAG] checking remote-free production facade selection"

guard_require_files "$TAG" "$CARD" "$PREV_CARD" "$TASKBOARD" "$CURRENT_STATE" "$INDEX" "$SELF_SCRIPT"
guard_require_exec_files "$TAG" "$SELF_SCRIPT"

guard_expect_fixed_in_file "$TAG" 'Status: Current' "$CARD" "remote-free selection card must be current"
guard_expect_fixed_in_file "$TAG" 'MIMALLOC-COMPARISON-REMOTE-FREE-PRODUCTION-FACADE-SELECTION-295X-002' "$CARD" "remote-free selection blocker must be fixed"
guard_expect_fixed_in_file "$TAG" 'MIMAP-REMOTE-001 production-facade remote-free policy integration over existing atomic/TLS proofs' "$CARD" "remote-free selection must identify the chosen seam"
guard_expect_fixed_in_file "$TAG" 'worker identity' "$CARD" "remote-free selection must reference substrate evidence"
guard_expect_fixed_in_file "$TAG" 'TLS cache slots' "$CARD" "remote-free selection must reference substrate evidence"
guard_expect_fixed_in_file "$TAG" 'atomic routes' "$CARD" "remote-free selection must reference substrate evidence"
guard_expect_fixed_in_file "$TAG" 'thread-safe hako_mem ABI' "$CARD" "remote-free selection must reference substrate evidence"
guard_expect_fixed_in_file "$TAG" 'native multi-worker stress' "$CARD" "remote-free selection must reference substrate evidence"
guard_expect_fixed_in_file "$TAG" 'This row does not open provider activation, DLL/replacement/hook/global' "$CARD" "remote-free selection must keep provider seams closed"
guard_expect_fixed_in_file "$TAG" 'MIMALLOC-COMPARISON-REMOTE-FREE-PRODUCTION-FACADE-CONTRACT-REFRESH-295X-002' "$CARD" "remote-free selection must choose a closeout follow-on"

guard_expect_fixed_in_file "$TAG" 'Status: Landed' "$PREV_CARD" "previous bridge card must be landed"
guard_expect_fixed_in_file "$TAG" 'MIMALLOC-COMPARISON-REMOTE-FREE-PRODUCTION-FACADE-SELECTION-295X-002' "$PREV_CARD" "previous card must select this row"

guard_expect_fixed_in_file "$TAG" '| 240 | `MIMALLOC-COMPARISON-NEXT-SEMANTIC-SEAM-SELECTION-295X-002` | Landed |' "$TASKBOARD" "taskboard must keep the bridge landed"
guard_expect_fixed_in_file "$TAG" '| 241 | `MIMALLOC-COMPARISON-REMOTE-FREE-PRODUCTION-FACADE-SELECTION-295X-002` | Current |' "$TASKBOARD" "taskboard must expose remote-free selection as current"

guard_expect_fixed_in_file "$TAG" '295x-241-MIMALLOC-COMPARISON-REMOTE-FREE-PRODUCTION-FACADE-SELECTION' "$CURRENT_STATE" "current state must point at the remote-free selection card"
guard_expect_fixed_in_file "$TAG" 'MIMALLOC-COMPARISON-REMOTE-FREE-PRODUCTION-FACADE-SELECTION-295X-002' "$CURRENT_STATE" "current state must expose the remote-free selection blocker"
guard_expect_fixed_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check script index must list remote-free selection guard"

echo "[$TAG] ok"
