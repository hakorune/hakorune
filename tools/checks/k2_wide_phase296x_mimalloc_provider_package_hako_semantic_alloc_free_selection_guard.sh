#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-phase296x-mimalloc-provider-package-hako-semantic-alloc-free-selection"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

CARD_35="docs/development/current/main/phases/phase-296x/296x-35-MIMALLOC-PROVIDER-PACKAGE-HAKO-SEMANTIC-ALLOC-FREE-SELECTION.md"
TASKBOARD="docs/development/current/main/phases/phase-296x/296x-90-mimalloc-benchmark-taskboard.md"
CURRENT_STATE="docs/development/current/main/CURRENT_STATE.toml"
INDEX="docs/tools/check-scripts-index.md"
SSOT="docs/development/current/main/design/provider-package-artifact-ssot.md"
ALLOC_FREE_TOOL="tools/allocator/provider_package_alloc_free_smoke.py"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_mimalloc_provider_package_hako_semantic_alloc_free_selection_guard.sh"

echo "[$TAG] checking phase-296x .hako semantic alloc/free selection"

guard_require_files "$TAG" "$CARD_35" "$TASKBOARD" "$CURRENT_STATE" "$INDEX" "$SSOT" "$ALLOC_FREE_TOOL" "$SELF_SCRIPT"
guard_require_exec_files "$TAG" "$ALLOC_FREE_TOOL" "$SELF_SCRIPT"

guard_expect_fixed_in_file "$TAG" 'Status: Landed' "$CARD_35" "selection card must be landed"
guard_expect_fixed_in_file "$TAG" 'MIMALLOC-PROVIDER-PACKAGE-HAKO-SEMANTIC-ALLOC-FREE-SELECTION-296X-001' "$CARD_35" "selection card must identify blocker"
guard_expect_fixed_in_file "$TAG" 'alloc-free-owns-literal-v0' "$CARD_35" "selection card must name selected mode"
guard_expect_fixed_in_file "$TAG" 'HakoProvider.ownsAllocated/0' "$CARD_35" "selection card must identify .hako owns function"
guard_expect_fixed_in_file "$TAG" 'provider_package_alloc_free_smoke.py' "$CARD_35" "selection card must select alloc/free smoke"
guard_expect_fixed_in_file "$TAG" 'hako_provider_owns_codegen=1' "$CARD_35" "selection card must define owns codegen evidence"
guard_expect_fixed_in_file "$TAG" 'provider_alloc_executed=1' "$CARD_35" "selection card must open explicit alloc call"
guard_expect_fixed_in_file "$TAG" 'provider_free_executed=1' "$CARD_35" "selection card must open explicit free call"
guard_expect_fixed_in_file "$TAG" 'allocator_entrypoint_called=1' "$CARD_35" "selection card must open allocator entrypoint smoke"
guard_expect_fixed_in_file "$TAG" 'provider_active=0' "$CARD_35" "selection card must keep provider inactive"
guard_expect_fixed_in_file "$TAG" 'replacement_active=0' "$CARD_35" "selection card must keep replacement closed"
guard_expect_fixed_in_file "$TAG" 'global_allocator=0' "$CARD_35" "selection card must keep global allocator closed"
guard_expect_fixed_in_file "$TAG" 'winner_claim=0' "$CARD_35" "selection card must keep winners closed"
guard_expect_fixed_in_file "$TAG" 'does not make `.hako` responsible for native pointer allocation' "$CARD_35" "selection card must state pointer allocation stop line"
guard_expect_fixed_in_file "$TAG" 'MIMALLOC-PROVIDER-PACKAGE-HAKO-SEMANTIC-ALLOC-FREE-PILOT-296X-001' "$CARD_35" "selection card must select pilot"

guard_expect_fixed_in_file "$TAG" 'alloc-free-owns-literal-v0' "$SSOT" "artifact SSOT must record selected allocator semantic mode"
guard_expect_fixed_in_file "$TAG" 'hako_provider_owns_value=<0|1>' "$SSOT" "artifact SSOT must define owns value evidence"
guard_expect_fixed_in_file "$TAG" 'provider_owns_result=<same owns literal for non-null pointer>' "$SSOT" "artifact SSOT must define runtime owns evidence"

guard_expect_fixed_in_file "$TAG" 'latest_card = "296x-35-MIMALLOC-PROVIDER-PACKAGE-HAKO-SEMANTIC-ALLOC-FREE-SELECTION"' "$CURRENT_STATE" "current state latest card must advance"
guard_expect_fixed_in_file "$TAG" 'current_blocker_token = "MIMALLOC-PROVIDER-PACKAGE-HAKO-SEMANTIC-ALLOC-FREE-PILOT-296X-001"' "$CURRENT_STATE" "current state must select alloc/free pilot"
guard_expect_fixed_in_file "$TAG" '| 35 | `MIMALLOC-PROVIDER-PACKAGE-HAKO-SEMANTIC-ALLOC-FREE-SELECTION-296X-001` | Landed |' "$TASKBOARD" "taskboard row 35 must be landed"
guard_expect_fixed_in_file "$TAG" '| 36 | `MIMALLOC-PROVIDER-PACKAGE-HAKO-SEMANTIC-ALLOC-FREE-PILOT-296X-001` | Current |' "$TASKBOARD" "taskboard row 36 must be current"
guard_expect_fixed_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list semantic alloc/free selection guard"

python3 -m py_compile "$ALLOC_FREE_TOOL"

echo "[$TAG] ok"
