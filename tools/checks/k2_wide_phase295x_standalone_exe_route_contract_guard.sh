#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-phase295x-standalone-exe-route-contract"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

CARD="docs/development/current/main/phases/phase-295x/295x-60-MIMALLOC-COMPARISON-STANDALONE-EXE-ROUTE-CONTRACT.md"
SSOT="docs/development/current/main/design/standalone-exe-route-contract-ssot.md"
LOADSET_SSOT="docs/development/current/main/design/plugin-loadset-linking-ssot.md"
TASKBOARD="docs/development/current/main/phases/phase-295x/295x-90-mimalloc-comparison-execution-taskboard.md"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase295x_standalone_exe_route_contract_guard.sh"

echo "[$TAG] checking phase-295x standalone EXE route contract"

guard_require_files "$TAG" "$CARD" "$SSOT" "$LOADSET_SSOT" "$TASKBOARD" "$INDEX" "$SELF_SCRIPT"
guard_require_exec_files "$TAG" "$SELF_SCRIPT"

guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-STANDALONE-EXE-ROUTE-CONTRACT-295X-001' "$CARD" "card must identify current blocker"
guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-POST-STANDALONE-ROUTE-SELECTION-295X-001' "$CARD" "card must select post-standalone route selection"
guard_expect_in_file "$TAG" 'standalone-minimal' "$CARD" "card must define standalone-minimal"
guard_expect_in_file "$TAG" 'standalone-root' "$CARD" "card must define standalone-root"
guard_expect_in_file "$TAG" 'standalone-diagnostic' "$CARD" "card must define standalone-diagnostic"
guard_expect_in_file "$TAG" 'runtime_config_profile' "$CARD" "card must require runtime config evidence"
guard_expect_in_file "$TAG" 'selected_loadset' "$CARD" "card must require selected loadset evidence"
guard_expect_in_file "$TAG" 'standalone_packaging_generated' "$SSOT" "SSOT must keep packaging state explicit"
guard_expect_in_file "$TAG" 'link_policy=<exact-mir-exe|standalone-package|provider-package>' "$SSOT" "SSOT must define link policy vocabulary"
guard_expect_in_file "$TAG" 'does not implement `hakorune build --kind standalone`' "$SSOT" "SSOT must keep implementation parked"
guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-POST-STANDALONE-ROUTE-SELECTION-295X-001' "$TASKBOARD" "taskboard must expose selected follow-on"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check script index must list this guard"

echo "[$TAG] ok"
