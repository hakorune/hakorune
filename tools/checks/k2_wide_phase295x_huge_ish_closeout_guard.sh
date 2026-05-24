#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-phase295x-huge-ish-closeout"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

CARD="docs/development/current/main/phases/phase-295x/295x-27-MIMALLOC-COMPARISON-HUGE-ISH-CLOSEOUT.md"
PREV_CARD="docs/development/current/main/phases/phase-295x/295x-26-MIMALLOC-COMPARISON-HUGE-ISH-EVIDENCE-RUN.md"
TASKBOARD="docs/development/current/main/phases/phase-295x/295x-90-mimalloc-comparison-execution-taskboard.md"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase295x_huge_ish_closeout_guard.sh"
EVIDENCE_GUARD="tools/checks/k2_wide_phase295x_huge_ish_evidence_run_guard.sh"

echo "[$TAG] checking phase-295x huge-ish closeout"

guard_require_files "$TAG" "$CARD" "$PREV_CARD" "$TASKBOARD" "$INDEX" "$SELF_SCRIPT" "$EVIDENCE_GUARD"
guard_require_exec_files "$TAG" "$SELF_SCRIPT" "$EVIDENCE_GUARD"

guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-HUGE-ISH-CLOSEOUT-295X-001' "$CARD" "card must identify the current blocker"
guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-REPEATED-MEASUREMENT-POLICY-295X-001' "$CARD" "card must select repeated measurement policy"
guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-HUGE-ISH-CLOSEOUT-295X-001' "$PREV_CARD" "previous row must select this closeout"
guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-REPEATED-MEASUREMENT-POLICY-295X-001' "$TASKBOARD" "taskboard must expose repeated measurement policy follow-on"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check script index must list this guard"
guard_expect_in_file "$TAG" "$EVIDENCE_GUARD" "$INDEX" "check script index must list evidence guard"
guard_expect_in_file "$TAG" 'winner claims still closed' "$CARD" "closeout must keep winner claims closed"
guard_expect_in_file "$TAG" 'representative-huge-ish-v0' "$CARD" "closeout must name the closed workload"

bash "$EVIDENCE_GUARD"

echo "[$TAG] ok"
