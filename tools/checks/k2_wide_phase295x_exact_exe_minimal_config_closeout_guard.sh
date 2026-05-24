#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-phase295x-exact-exe-minimal-config-closeout"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

CARD="docs/development/current/main/phases/phase-295x/295x-49-MIMALLOC-COMPARISON-EXACT-EXE-MINIMAL-CONFIG-CLOSEOUT.md"
EVIDENCE_CARD="docs/development/current/main/phases/phase-295x/295x-48-MIMALLOC-COMPARISON-EXACT-EXE-MINIMAL-CONFIG-EVIDENCE.md"
TASKBOARD="docs/development/current/main/phases/phase-295x/295x-90-mimalloc-comparison-execution-taskboard.md"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase295x_exact_exe_minimal_config_closeout_guard.sh"
EVIDENCE_GUARD="tools/checks/k2_wide_phase295x_exact_exe_minimal_config_evidence_guard.sh"
RUNNER="tools/allocator/hako_exe_memory_runner.sh"

echo "[$TAG] checking phase-295x exact-EXE minimal config closeout"

guard_require_files "$TAG" "$CARD" "$EVIDENCE_CARD" "$TASKBOARD" "$INDEX" "$SELF_SCRIPT" "$EVIDENCE_GUARD" "$RUNNER"
guard_require_exec_files "$TAG" "$SELF_SCRIPT" "$EVIDENCE_GUARD" "$RUNNER"

guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-EXACT-EXE-MINIMAL-CONFIG-CLOSEOUT-295X-001' "$CARD" "card must identify current blocker"
guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-RUNTIME-CONFIG-PROFILE-CONTRACT-295X-001' "$CARD" "card must select runtime config profile follow-on"
guard_expect_in_file "$TAG" 'hako.toml / package intent' "$CARD" "card must keep hako.toml as package-facing intent"
guard_expect_in_file "$TAG" 'generated minimal runtime nyash.toml' "$CARD" "card must keep generated runtime nyash.toml boundary"
guard_expect_in_file "$TAG" 'runner profile, not a default NyRT behavior change' "$CARD" "card must forbid default runtime behavior change"
guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-RUNTIME-CONFIG-PROFILE-CONTRACT-295X-001' "$TASKBOARD" "taskboard must expose selected follow-on"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check script index must list this guard"

if rg -q 'runtime_config_profile="empty"' "$RUNNER"; then
  guard_fail "$TAG" "runner must not hard-code empty profile as default"
fi
guard_expect_in_file "$TAG" 'RUNTIME_CONFIG="root"' "$RUNNER" "runner default runtime config must remain root"

bash "$EVIDENCE_GUARD"

echo "[$TAG] ok"
