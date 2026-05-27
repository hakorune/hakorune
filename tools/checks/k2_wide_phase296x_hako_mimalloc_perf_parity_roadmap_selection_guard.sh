#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-phase296x-hako-mimalloc-perf-parity-roadmap-selection"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

CARD_41="docs/development/current/main/phases/phase-296x/296x-41-MIMALLOC-PROVIDER-PACKAGE-EXPLICIT-COMPARISON-CLOSEOUT.md"
CARD_42="docs/development/current/main/phases/phase-296x/296x-42-HAKO-MIMALLOC-PERF-PARITY-ROADMAP-SELECTION.md"
TASKBOARD="docs/development/current/main/phases/phase-296x/296x-90-mimalloc-benchmark-taskboard.md"
CURRENT_STATE="docs/development/current/main/CURRENT_STATE.toml"
PARITY_SSOT="docs/development/current/main/design/hako-mimalloc-performance-parity-ssot.md"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_hako_mimalloc_perf_parity_roadmap_selection_guard.sh"

echo "[$TAG] checking phase-296x parity roadmap selection"

guard_require_files "$TAG" "$CARD_41" "$CARD_42" "$TASKBOARD" "$CURRENT_STATE" "$PARITY_SSOT" "$INDEX" "$SELF_SCRIPT"
guard_require_exec_files "$TAG" "$SELF_SCRIPT"

guard_expect_fixed_in_file "$TAG" 'Status: Landed' "$CARD_41" "closeout card must be landed"
guard_expect_fixed_in_file "$TAG" 'Status: Landed' "$CARD_42" "roadmap card must be landed"
guard_expect_fixed_in_file "$TAG" 'HAKO-MIMALLOC-PERF-PARITY-ROADMAP-SELECTION-296X-001' "$CARD_42" "roadmap card must identify blocker"
guard_expect_fixed_in_file "$TAG" '.hako mimalloc parity' "$CARD_42" "roadmap card must define parity lane"
guard_expect_fixed_in_file "$TAG" 'hakozuna reference-only' "$CARD_42" "roadmap card must keep hakozuna reference-only"
guard_expect_fixed_in_file "$TAG" 'allocator product selection' "$CARD_42" "roadmap card must park allocator selection"
guard_expect_fixed_in_file "$TAG" 'HAKO-MIMALLOC-PERF-PARITY-WORKLOAD-MATRIX-296X-001' "$CARD_42" "roadmap card must select workload matrix"

guard_expect_fixed_in_file "$TAG" 'latest_card = "296x-42-HAKO-MIMALLOC-PERF-PARITY-ROADMAP-SELECTION"' "$CURRENT_STATE" "current state latest card must advance to roadmap selection"
guard_expect_fixed_in_file "$TAG" 'current_blocker_token = "HAKO-MIMALLOC-PERF-PARITY-WORKLOAD-MATRIX-296X-001"' "$CURRENT_STATE" "current state must select workload matrix"
guard_expect_fixed_in_file "$TAG" '| 42 | `HAKO-MIMALLOC-PERF-PARITY-ROADMAP-SELECTION-296X-001` | Landed |' "$TASKBOARD" "taskboard row 42 must be landed"
guard_expect_fixed_in_file "$TAG" '| 43 | `HAKO-MIMALLOC-PERF-PARITY-WORKLOAD-MATRIX-296X-001` | Current |' "$TASKBOARD" "taskboard row 43 must be current"
guard_expect_fixed_in_file "$TAG" 'Hako Mimalloc Performance Parity' "$PARITY_SSOT" "parity SSOT must exist"
guard_expect_fixed_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list this guard"

echo "[$TAG] ok"
