#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-hako-alloc-wide-report-argument-cleanup-plan"
cd "$ROOT_DIR"
source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

CARD="docs/development/current/main/phases/phase-293x/293x-1077-ARG-DATA-001-WIDE-REPORT-ARGUMENT-CLEANUP-PLAN.md"
NEXT_CARD="docs/development/current/main/phases/phase-293x/293x-1078-ARG-DATA-002-C-MIMALLOC-EXPLICIT-RUNNER-ARGUMENT-OBJECT-PILOT.md"
QUEUED_MIMAP="docs/development/current/main/phases/phase-293x/293x-1076-MIMAP-454A-ALLOCATOR-COMPARISON-C-MIMALLOC-RESULT-LEDGER-PILOT.md"
DESIGN="docs/development/current/main/design/hako-alloc-wide-report-argument-cleanup-ssot.md"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_hako_alloc_wide_report_argument_cleanup_plan_guard.sh"

printf '[%s] checking ARG-DATA-001 wide report argument cleanup plan\n' "$TAG"

guard_require_files "$TAG" "$CARD" "$NEXT_CARD" "$QUEUED_MIMAP" "$DESIGN" "$INDEX" "$SELF_SCRIPT"
guard_require_exec_files "$TAG" "$SELF_SCRIPT"

guard_expect_in_file "$TAG" 'Status: landed' "$CARD" "ARG-DATA-001 card must be landed"
guard_expect_in_file "$TAG" 'Status: selected current' "$NEXT_CARD" "ARG-DATA-002 must be selected current"
guard_expect_in_file "$TAG" 'Status: queued after ARG-DATA-002' "$QUEUED_MIMAP" "MIMAP-454A must stay queued after ARG-DATA-002"
guard_expect_in_file "$TAG" 'Decision: accepted' "$DESIGN" "wide report argument cleanup SSOT must be accepted"
guard_expect_in_file "$TAG" 'Replace long positional argument lists with owner-local context records' "$DESIGN" "SSOT must prefer context records"
guard_expect_fixed_in_file "$TAG" 'No `...fields` / spread syntax' "$DESIGN" "spread syntax must stay parked"
guard_expect_in_file "$TAG" 'No named argument syntax' "$DESIGN" "named args must stay parked"
guard_expect_in_file "$TAG" 'No record default value semantics' "$DESIGN" "record defaults must stay parked"
guard_expect_in_file "$TAG" 'No automatic record-to-box copy semantics' "$DESIGN" "record-to-box auto-copy must stay parked"
guard_expect_fixed_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list ARG-DATA-001 guard"

printf '[%s] ok\n' "$TAG"
