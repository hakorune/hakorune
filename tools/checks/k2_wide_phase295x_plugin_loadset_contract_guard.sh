#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-phase295x-plugin-loadset-contract"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

CARD="docs/development/current/main/phases/phase-295x/295x-55-MIMALLOC-COMPARISON-PLUGIN-LOADSET-CONTRACT.md"
SSOT="docs/development/current/main/design/plugin-loadset-linking-ssot.md"
TASKBOARD="docs/development/current/main/phases/phase-295x/295x-90-mimalloc-comparison-execution-taskboard.md"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase295x_plugin_loadset_contract_guard.sh"

echo "[$TAG] checking phase-295x plugin loadset contract"

guard_require_files "$TAG" "$CARD" "$SSOT" "$TASKBOARD" "$INDEX" "$SELF_SCRIPT"
guard_require_exec_files "$TAG" "$SELF_SCRIPT"

guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-PLUGIN-LOADSET-CONTRACT-295X-001' "$CARD" "card must identify current blocker"
guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-PLUGIN-LOADSET-PREFLIGHT-PLAN-295X-001' "$CARD" "card must select preflight plan follow-on"
guard_expect_in_file "$TAG" 'selected_loadset' "$CARD" "card must require selected loadset evidence"
guard_expect_in_file "$TAG" 'plugin_load_policy=eager_selected' "$CARD" "card must require eager selected policy"
guard_expect_in_file "$TAG" 'hako-plugin-loadset-plan-v0' "$CARD" "card must define preflight output contract"

guard_expect_in_file "$TAG" 'Use manifest-selected loadsets with eager loading of the selected set.' "$SSOT" "SSOT must state selected eager policy"
guard_expect_in_file "$TAG" 'no implicit lazy loading' "$SSOT" "SSOT must forbid implicit lazy loading"
guard_expect_in_file "$TAG" 'output_contract=hako-plugin-loadset-plan-v0' "$SSOT" "SSOT must define preflight contract"
guard_expect_in_file "$TAG" 'provider_activation=0' "$SSOT" "SSOT must keep provider activation closed"
guard_expect_in_file "$TAG" 'global_allocator_installed=0' "$SSOT" "SSOT must keep global allocator closed"
guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-PLUGIN-LOADSET-PREFLIGHT-PLAN-295X-001' "$TASKBOARD" "taskboard must expose selected follow-on"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check script index must list this guard"

echo "[$TAG] ok"
