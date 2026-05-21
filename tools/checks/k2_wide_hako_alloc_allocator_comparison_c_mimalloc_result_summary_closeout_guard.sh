#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-hako-alloc-allocator-comparison-c-mimalloc-result-summary-closeout"
cd "$ROOT_DIR"
source "$ROOT_DIR/tools/checks/lib/pure_first_exe_guard.sh"

if [ "$#" -ne 0 ]; then
  VALIDATION_LEVEL="$(pure_first_guard_parse_level "$TAG" "$@")"
  case "$VALIDATION_LEVEL" in
    L0|L1|L2) ;;
    L3|L4)
      echo "[$TAG] ERROR: MIMAP-459A closeout does not define an L3/L4 benchmark pack" >&2
      exit 2
      ;;
  esac
fi

CARD_457A="docs/development/current/main/phases/phase-293x/293x-1087-MIMAP-457A-ALLOCATOR-COMPARISON-C-MIMALLOC-RESULT-SUMMARY-INVENTORY.md"
CARD_458A="docs/development/current/main/phases/phase-293x/293x-1088-MIMAP-458A-ALLOCATOR-COMPARISON-C-MIMALLOC-RESULT-SUMMARY-DIAGNOSTICS.md"
CARD="docs/development/current/main/phases/phase-293x/293x-1089-MIMAP-459A-ALLOCATOR-COMPARISON-C-MIMALLOC-RESULT-SUMMARY-CLOSEOUT.md"
NEXT_CARD="docs/development/current/main/phases/phase-293x/293x-1090-MIMAP-460A-ALLOCATOR-COMPARISON-C-MIMALLOC-RESULT-REPORTING-INVENTORY.md"
DESIGN="docs/development/current/main/design/hako-alloc-allocator-comparison-c-mimalloc-result-summary-closeout-ssot.md"
DESIGN_457A="docs/development/current/main/design/hako-alloc-allocator-comparison-c-mimalloc-result-summary-inventory-ssot.md"
DESIGN_458A="docs/development/current/main/design/hako-alloc-allocator-comparison-c-mimalloc-result-summary-diagnostics-ssot.md"
INDEX="docs/tools/check-scripts-index.md"
PROOF_MANIFEST_INCLUDE="tools/checks/manifests/proof_apps/hako_alloc_segment_arena_backing_release_lifecycle.toml"
GUARD_457A="tools/checks/k2_wide_hako_alloc_allocator_comparison_c_mimalloc_result_summary_inventory_guard.sh"
GUARD_458A="tools/checks/k2_wide_hako_alloc_allocator_comparison_c_mimalloc_result_summary_diagnostics_guard.sh"
SELF_SCRIPT="tools/checks/k2_wide_hako_alloc_allocator_comparison_c_mimalloc_result_summary_closeout_guard.sh"

printf '[%s] checking MIMAP-459A C mimalloc result summary closeout\n' "$TAG"

guard_require_files "$TAG" "$CARD_457A" "$CARD_458A" "$CARD" "$NEXT_CARD" "$DESIGN" "$DESIGN_457A" "$DESIGN_458A" "$INDEX" "$PROOF_MANIFEST_INCLUDE" "$GUARD_457A" "$GUARD_458A" "$SELF_SCRIPT"
guard_require_exec_files "$TAG" "$GUARD_457A" "$GUARD_458A" "$SELF_SCRIPT"

for card in "$CARD_457A" "$CARD_458A"; do
  guard_expect_in_file "$TAG" 'Status: landed' "$card" "$card must be landed"
done
guard_expect_in_file "$TAG" 'Status: (selected current|completed|landed)' "$CARD" "MIMAP-459A must be current/completed/landed"
guard_expect_in_file "$TAG" 'Status: (selected current|landed)' "$NEXT_CARD" "MIMAP-460A must be selected current or landed"
guard_expect_in_file "$TAG" 'Decision: accepted' "$DESIGN" "MIMAP-459A design must be accepted"
guard_expect_in_file "$TAG" 'Decision: accepted' "$DESIGN_457A" "MIMAP-457A design must remain accepted"
guard_expect_in_file "$TAG" 'Decision: accepted' "$DESIGN_458A" "MIMAP-458A design must remain accepted"
guard_expect_fixed_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list MIMAP-459A guard"
guard_expect_in_file "$TAG" 'id = "MIMAP-457A"' "$PROOF_MANIFEST_INCLUDE" "proof manifest must list MIMAP-457A"
guard_expect_in_file "$TAG" 'id = "MIMAP-458A"' "$PROOF_MANIFEST_INCLUDE" "proof manifest must list MIMAP-458A"
guard_expect_in_file "$TAG" 'closeout_pack = "allocator-comparison-c-mimalloc-result-summary"' "$PROOF_MANIFEST_INCLUDE" "summary rows must share closeout pack"
guard_expect_in_file "$TAG" 'MIMAP-460A should add a reporting inventory' "$CARD" "MIMAP-459A must select reporting inventory"
guard_expect_in_file "$TAG" 'No performance conclusion' "$NEXT_CARD" "MIMAP-460A must keep performance conclusion closed"
guard_expect_in_file "$TAG" 'No memory-use conclusion' "$NEXT_CARD" "MIMAP-460A must keep memory conclusion closed"
guard_expect_in_file "$TAG" 'No `#\[global_allocator\]`' "$NEXT_CARD" "MIMAP-460A must keep global allocator closed"

bash "$GUARD_457A" --level L2
bash "$GUARD_458A" --level L2

printf '[%s] ok\n' "$TAG"
