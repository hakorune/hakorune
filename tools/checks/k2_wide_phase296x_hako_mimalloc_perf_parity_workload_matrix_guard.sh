#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-phase296x-hako-mimalloc-perf-parity-workload-matrix"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

CARD_42="docs/development/current/main/phases/phase-296x/296x-42-HAKO-MIMALLOC-PERF-PARITY-ROADMAP-SELECTION.md"
CARD_43="docs/development/current/main/phases/phase-296x/296x-43-HAKO-MIMALLOC-PERF-PARITY-WORKLOAD-MATRIX.md"
TASKBOARD="docs/development/current/main/phases/phase-296x/296x-90-mimalloc-benchmark-taskboard.md"
CURRENT_STATE="docs/development/current/main/CURRENT_STATE.toml"
PARITY_SSOT="docs/development/current/main/design/hako-mimalloc-performance-parity-ssot.md"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_hako_mimalloc_perf_parity_workload_matrix_guard.sh"

echo "[$TAG] checking phase-296x parity workload matrix"

guard_require_files "$TAG" "$CARD_42" "$CARD_43" "$TASKBOARD" "$CURRENT_STATE" "$PARITY_SSOT" "$INDEX" "$SELF_SCRIPT"
guard_require_exec_files "$TAG" "$SELF_SCRIPT"

guard_expect_fixed_in_file "$TAG" 'Status: Landed' "$CARD_43" "workload matrix card must be landed"
guard_expect_fixed_in_file "$TAG" 'HAKO-MIMALLOC-PERF-PARITY-WORKLOAD-MATRIX-296X-001' "$CARD_43" "workload matrix card must identify blocker"
guard_expect_fixed_in_file "$TAG" 'hako_mimalloc_exact_exe' "$CARD_43" "workload matrix must include hako subject"
guard_expect_fixed_in_file "$TAG" 'c_mimalloc_explicit_runner' "$CARD_43" "workload matrix must include C subject"
guard_expect_fixed_in_file "$TAG" 'hakozuna_reference' "$CARD_43" "workload matrix must include hakozuna reference"
guard_expect_fixed_in_file "$TAG" 'provider_package_hako_mimalloc_explicit' "$CARD_43" "workload matrix must include provider package reference"
guard_expect_fixed_in_file "$TAG" 'small_block_alloc_free' "$CARD_43" "workload matrix must include the first workload"
guard_expect_fixed_in_file "$TAG" 'hakmem_selected_family' "$CARD_43" "workload matrix must include the hakmem family"
guard_expect_fixed_in_file "$TAG" 'HAKO-MIMALLOC-PERF-PARITY-BASELINE-PACK-296X-001' "$CARD_43" "workload matrix must select baseline pack"

guard_expect_fixed_in_file "$TAG" 'latest_card = "296x-43-HAKO-MIMALLOC-PERF-PARITY-WORKLOAD-MATRIX"' "$CURRENT_STATE" "current state latest card must advance to workload matrix"
guard_expect_fixed_in_file "$TAG" 'current_blocker_token = "HAKO-MIMALLOC-PERF-PARITY-BASELINE-PACK-296X-001"' "$CURRENT_STATE" "current state must select baseline pack"
guard_expect_fixed_in_file "$TAG" '| 43 | `HAKO-MIMALLOC-PERF-PARITY-WORKLOAD-MATRIX-296X-001` | Landed |' "$TASKBOARD" "taskboard row 43 must be landed"
guard_expect_fixed_in_file "$TAG" '| 44 | `HAKO-MIMALLOC-PERF-PARITY-BASELINE-PACK-296X-001` | Current |' "$TASKBOARD" "taskboard row 44 must be current"
guard_expect_fixed_in_file "$TAG" 'Hako Mimalloc Performance Parity' "$PARITY_SSOT" "parity SSOT must exist"
guard_expect_fixed_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list this guard"

echo "[$TAG] ok"
