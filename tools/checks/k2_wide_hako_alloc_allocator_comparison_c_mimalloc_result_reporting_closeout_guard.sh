#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-hako-alloc-allocator-comparison-c-mimalloc-result-reporting-closeout"
cd "$ROOT_DIR"
source "$ROOT_DIR/tools/checks/lib/pure_first_exe_guard.sh"

if [ "$#" -ne 0 ]; then
  VALIDATION_LEVEL="$(pure_first_guard_parse_level "$TAG" "$@")"
  case "$VALIDATION_LEVEL" in
    L0|L1|L2) ;;
    L3|L4)
      echo "[$TAG] ERROR: MIMAP-462A closeout does not define an L3/L4 benchmark pack" >&2
      exit 2
      ;;
  esac
fi

CARD_460A="docs/development/current/main/phases/phase-293x/293x-1090-MIMAP-460A-ALLOCATOR-COMPARISON-C-MIMALLOC-RESULT-REPORTING-INVENTORY.md"
CARD_461A="docs/development/current/main/phases/phase-293x/293x-1091-MIMAP-461A-ALLOCATOR-COMPARISON-C-MIMALLOC-RESULT-REPORTING-DIAGNOSTICS.md"
CARD="docs/development/current/main/phases/phase-293x/293x-1092-MIMAP-462A-ALLOCATOR-COMPARISON-C-MIMALLOC-RESULT-REPORTING-CLOSEOUT.md"
NEXT_CARD="docs/development/current/main/phases/phase-293x/293x-1093-MIMAP-463A-ALLOCATOR-COMPARISON-C-MIMALLOC-RESULT-PRESENTATION-DECISION-ROW-SELECTION.md"
DESIGN="docs/development/current/main/design/hako-alloc-allocator-comparison-c-mimalloc-result-reporting-closeout-ssot.md"
DESIGN_460A="docs/development/current/main/design/hako-alloc-allocator-comparison-c-mimalloc-result-reporting-inventory-ssot.md"
DESIGN_461A="docs/development/current/main/design/hako-alloc-allocator-comparison-c-mimalloc-result-reporting-diagnostics-ssot.md"
INDEX="docs/tools/check-scripts-index.md"
PROOF_MANIFEST_INCLUDE="tools/checks/manifests/proof_apps/hako_alloc_segment_arena_backing_release_lifecycle.toml"
GUARD_460A="tools/checks/k2_wide_hako_alloc_allocator_comparison_c_mimalloc_result_reporting_inventory_guard.sh"
GUARD_461A="tools/checks/k2_wide_hako_alloc_allocator_comparison_c_mimalloc_result_reporting_diagnostics_guard.sh"
SELF_SCRIPT="tools/checks/k2_wide_hako_alloc_allocator_comparison_c_mimalloc_result_reporting_closeout_guard.sh"

printf '[%s] checking MIMAP-462A C mimalloc result reporting closeout\n' "$TAG"

guard_require_files "$TAG" "$CARD_460A" "$CARD_461A" "$CARD" "$NEXT_CARD" "$DESIGN" "$DESIGN_460A" "$DESIGN_461A" "$INDEX" "$PROOF_MANIFEST_INCLUDE" "$GUARD_460A" "$GUARD_461A" "$SELF_SCRIPT"
guard_require_exec_files "$TAG" "$GUARD_460A" "$GUARD_461A" "$SELF_SCRIPT"

for card in "$CARD_460A" "$CARD_461A"; do
  guard_expect_in_file "$TAG" 'Status: landed' "$card" "$card must be landed"
done
guard_expect_in_file "$TAG" 'Status: (selected current|completed|landed)' "$CARD" "MIMAP-462A must be current/completed/landed"
guard_expect_in_file "$TAG" 'Status: (selected current|landed)' "$NEXT_CARD" "MIMAP-463A must be selected current or landed"
guard_expect_in_file "$TAG" 'Decision: accepted' "$DESIGN" "MIMAP-462A design must be accepted"
guard_expect_in_file "$TAG" 'Decision: accepted' "$DESIGN_460A" "MIMAP-460A design must remain accepted"
guard_expect_in_file "$TAG" 'Decision: accepted' "$DESIGN_461A" "MIMAP-461A design must remain accepted"
guard_expect_fixed_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list MIMAP-462A guard"
guard_expect_in_file "$TAG" 'id = "MIMAP-460A"' "$PROOF_MANIFEST_INCLUDE" "proof manifest must list MIMAP-460A"
guard_expect_in_file "$TAG" 'id = "MIMAP-461A"' "$PROOF_MANIFEST_INCLUDE" "proof manifest must list MIMAP-461A"
guard_expect_in_file "$TAG" 'closeout_pack = "allocator-comparison-c-mimalloc-result-reporting"' "$PROOF_MANIFEST_INCLUDE" "reporting rows must share closeout pack"
guard_expect_in_file "$TAG" 'MIMAP-463A should decide whether the next row is presentation-only or a guarded' "$CARD" "MIMAP-462A must select presentation / decision row selection"
guard_expect_in_file "$TAG" 'first performance / memory-use conclusion preflight' "$CARD" "MIMAP-462A must keep the guarded conclusion-preflight option visible"
guard_expect_in_file "$TAG" 'presentation-only' "$NEXT_CARD" "MIMAP-463A must keep the presentation-only option visible"
guard_expect_in_file "$TAG" 'guarded first performance / memory-use conclusion preflight' "$NEXT_CARD" "MIMAP-463A must keep the guarded conclusion-preflight option visible"
guard_expect_in_file "$TAG" 'No performance conclusion' "$NEXT_CARD" "MIMAP-463A must keep performance conclusion closed"
guard_expect_in_file "$TAG" 'No memory-use conclusion' "$NEXT_CARD" "MIMAP-463A must keep memory conclusion closed"
guard_expect_in_file "$TAG" 'No `#\[global_allocator\]`' "$NEXT_CARD" "MIMAP-463A must keep global allocator closed"

bash "$GUARD_460A" --level L2
bash "$GUARD_461A" --level L2

printf '[%s] ok\n' "$TAG"
