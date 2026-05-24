#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-phase295x-runtime-config-profile-contract"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

CARD="docs/development/current/main/phases/phase-295x/295x-50-MIMALLOC-COMPARISON-RUNTIME-CONFIG-PROFILE-CONTRACT.md"
PREV_CARD="docs/development/current/main/phases/phase-295x/295x-49-MIMALLOC-COMPARISON-EXACT-EXE-MINIMAL-CONFIG-CLOSEOUT.md"
TASKBOARD="docs/development/current/main/phases/phase-295x/295x-90-mimalloc-comparison-execution-taskboard.md"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase295x_runtime_config_profile_contract_guard.sh"
RUNNER="tools/allocator/hako_exe_memory_runner.sh"

echo "[$TAG] checking phase-295x runtime config profile contract"

guard_require_files "$TAG" "$CARD" "$PREV_CARD" "$TASKBOARD" "$INDEX" "$SELF_SCRIPT" "$RUNNER"
guard_require_exec_files "$TAG" "$SELF_SCRIPT" "$RUNNER"

guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-RUNTIME-CONFIG-PROFILE-CONTRACT-295X-001' "$CARD" "card must identify current blocker"
guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-MINIMAL-CONFIG-REPEATED-PACK-RUN-295X-001' "$CARD" "card must select minimal-config repeated pack follow-on"
guard_expect_in_file "$TAG" 'root is the default' "$CARD" "card must keep root default"
guard_expect_in_file "$TAG" 'empty is opt-in' "$CARD" "card must keep empty opt-in"
guard_expect_in_file "$TAG" 'hako.toml remains the package-facing configuration intent' "$CARD" "card must keep hako.toml as package-facing intent"
guard_expect_in_file "$TAG" 'generated runtime `nyash.toml`' "$CARD" "card must define generated runtime nyash.toml lowering"
guard_expect_in_file "$TAG" 'unsupported profile names fail-fast' "$CARD" "card must require fail-fast unsupported profiles"

guard_expect_in_file "$TAG" 'RUNTIME_CONFIG="root"' "$RUNNER" "runner default runtime config must remain root"
guard_expect_in_file "$TAG" 'root\|empty' "$RUNNER" "runner must accept root and empty profiles only"
guard_expect_in_file "$TAG" 'runtime_config_profile' "$RUNNER" "runner must emit runtime_config_profile evidence"
guard_expect_in_file "$TAG" 'nyash.toml' "$RUNNER" "runner must generate runtime nyash.toml for profile execution"

if rg -q 'RUNTIME_CONFIG="empty"' "$RUNNER"; then
  guard_fail "$TAG" "empty profile must not become default"
fi

guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-MINIMAL-CONFIG-REPEATED-PACK-RUN-295X-001' "$TASKBOARD" "taskboard must expose selected follow-on"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check script index must list this guard"

echo "[$TAG] ok"
