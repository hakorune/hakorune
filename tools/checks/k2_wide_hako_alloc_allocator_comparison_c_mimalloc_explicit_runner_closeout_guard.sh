#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-hako-alloc-allocator-comparison-c-mimalloc-explicit-runner-closeout"
cd "$ROOT_DIR"
source "$ROOT_DIR/tools/checks/lib/pure_first_exe_guard.sh"

if [ "$#" -ne 0 ]; then
  VALIDATION_LEVEL="$(pure_first_guard_parse_level "$TAG" "$@")"
  case "$VALIDATION_LEVEL" in
    L0|L1|L2) ;;
    L3|L4)
      echo "[$TAG] ERROR: MIMAP-453A closeout does not define a repeated L3/L4 benchmark pack" >&2
      exit 2
      ;;
  esac
fi

CARD_451A="docs/development/current/main/phases/phase-293x/293x-1073-MIMAP-451A-ALLOCATOR-COMPARISON-C-MIMALLOC-EXPLICIT-RUNNER-EXECUTION-PILOT.md"
CARD_452A="docs/development/current/main/phases/phase-293x/293x-1074-MIMAP-452A-ALLOCATOR-COMPARISON-C-MIMALLOC-EXPLICIT-RUNNER-EVIDENCE-DIAGNOSTICS.md"
CARD="docs/development/current/main/phases/phase-293x/293x-1075-MIMAP-453A-ALLOCATOR-COMPARISON-C-MIMALLOC-EXPLICIT-RUNNER-CLOSEOUT.md"
NEXT_CARD="docs/development/current/main/phases/phase-293x/293x-1076-MIMAP-454A-ALLOCATOR-COMPARISON-C-MIMALLOC-RESULT-LEDGER-PILOT.md"
DESIGN="docs/development/current/main/design/hako-alloc-allocator-comparison-c-mimalloc-explicit-runner-closeout-ssot.md"
DESIGN_451A="docs/development/current/main/design/hako-alloc-allocator-comparison-c-mimalloc-explicit-runner-execution-pilot-ssot.md"
DESIGN_452A="docs/development/current/main/design/hako-alloc-allocator-comparison-c-mimalloc-explicit-runner-evidence-diagnostics-ssot.md"
INDEX="docs/tools/check-scripts-index.md"
PROOF_MANIFEST_INCLUDE="tools/checks/manifests/proof_apps/hako_alloc_segment_arena_backing_release_lifecycle.toml"
GUARD_451A="tools/checks/k2_wide_hako_alloc_allocator_comparison_c_mimalloc_explicit_runner_execution_pilot_guard.sh"
GUARD_452A="tools/checks/k2_wide_hako_alloc_allocator_comparison_c_mimalloc_explicit_runner_evidence_diagnostics_guard.sh"
SELF_SCRIPT="tools/checks/k2_wide_hako_alloc_allocator_comparison_c_mimalloc_explicit_runner_closeout_guard.sh"

printf '[%s] checking MIMAP-453A explicit C mimalloc runner closeout\n' "$TAG"

guard_require_files "$TAG" "$CARD_451A" "$CARD_452A" "$CARD" "$NEXT_CARD" "$DESIGN" "$DESIGN_451A" "$DESIGN_452A" "$INDEX" "$PROOF_MANIFEST_INCLUDE" "$GUARD_451A" "$GUARD_452A" "$SELF_SCRIPT"
guard_require_exec_files "$TAG" "$GUARD_451A" "$GUARD_452A" "$SELF_SCRIPT"

for card in "$CARD_451A" "$CARD_452A" "$CARD"; do
  guard_expect_in_file "$TAG" 'Status: landed' "$card" "$card must be landed"
done
guard_expect_in_file "$TAG" 'Status: (selected current|landed)' "$NEXT_CARD" "MIMAP-454A must be selected current or landed"
guard_expect_in_file "$TAG" 'Decision: accepted' "$DESIGN" "MIMAP-453A design must be accepted"
guard_expect_in_file "$TAG" 'Decision: accepted' "$DESIGN_451A" "MIMAP-451A design must remain accepted"
guard_expect_in_file "$TAG" 'Decision: accepted' "$DESIGN_452A" "MIMAP-452A design must remain accepted"
guard_expect_fixed_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list MIMAP-453A guard"
guard_expect_in_file "$TAG" 'id = "MIMAP-451A"' "$PROOF_MANIFEST_INCLUDE" "proof manifest must list MIMAP-451A"
guard_expect_in_file "$TAG" 'id = "MIMAP-452A"' "$PROOF_MANIFEST_INCLUDE" "proof manifest must list MIMAP-452A"
guard_expect_in_file "$TAG" 'closeout_pack = "allocator-comparison-c-mimalloc-explicit-runner-execution"' "$PROOF_MANIFEST_INCLUDE" "explicit runner rows must share closeout pack"
guard_expect_in_file "$TAG" 'MIMAP-454A should open a narrow C-vs-Hako comparison result ledger' "$CARD" "MIMAP-453A must select result ledger pilot"
guard_expect_in_file "$TAG" 'No process allocator replacement' "$NEXT_CARD" "MIMAP-454A must keep process replacement closed"
guard_expect_in_file "$TAG" 'No `#\[global_allocator\]`' "$NEXT_CARD" "MIMAP-454A must keep global allocator closed"

bash "$GUARD_451A" --level L2
bash "$GUARD_452A" --level L2

printf '[%s] ok\n' "$TAG"
