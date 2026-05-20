#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-hako-alloc-execution-seam-summary-closeout"
cd "$ROOT_DIR"
source "$ROOT_DIR/tools/checks/lib/pure_first_exe_guard.sh"

DESIGN="docs/development/current/main/design/hako-alloc-execution-seam-summary-closeout-ssot.md"
INDEX="docs/tools/check-scripts-index.md"
PROOF_MANIFEST_INCLUDE="tools/checks/manifests/proof_apps/hako_alloc_segment_arena_backing_release_lifecycle.toml"
PROVIDER_GUARD="tools/checks/k2_wide_hako_alloc_provider_inactive_boundary_inventory_guard.sh"
MATCHER_GUARD="tools/checks/k2_wide_hako_alloc_backend_matcher_no_growth_closeout_guard.sh"
SELF_SCRIPT="tools/checks/k2_wide_hako_alloc_execution_seam_summary_closeout_guard.sh"

CARD_344A="docs/development/current/main/phases/phase-293x/293x-959-MIMAP-344A-NO-ESCAPE-POINTER-RESIDENCE-PILOT.md"
CARD_345A="docs/development/current/main/phases/phase-293x/293x-960-MIMAP-345A-ARENA-BACKING-HANDLE-PILOT.md"
CARD_346A="docs/development/current/main/phases/phase-293x/293x-961-MIMAP-346A-POINTER-DERIVED-LOOKUP-EXECUTION-PILOT.md"
CARD_347A="docs/development/current/main/phases/phase-293x/293x-962-MIMAP-347A-SEGMENT-MAP-MUTATION-PILOT.md"
CARD_348A="docs/development/current/main/phases/phase-293x/293x-963-MIMAP-348A-ATOMIC-BITMAP-PILOT.md"
CARD_349A="docs/development/current/main/phases/phase-293x/293x-964-MIMAP-349A-OSVM-PAGE-SOURCE-PILOT.md"
CARD_350A="docs/development/current/main/phases/phase-293x/293x-965-MIMAP-350A-WORKER-TLS-PILOT.md"
CARD_352A="docs/development/current/main/phases/phase-293x/293x-968-MIMAP-352A-PROVIDER-INACTIVE-BOUNDARY-INVENTORY.md"
CARD_354A="docs/development/current/main/phases/phase-293x/293x-970-MIMAP-354A-BACKEND-MATCHER-NO-GROWTH-CLOSEOUT.md"
CARD_355A="docs/development/current/main/phases/phase-293x/293x-971-MIMAP-355A-POST-BACKEND-MATCHER-NO-GROWTH-ROW-SELECTION.md"
CARD="docs/development/current/main/phases/phase-293x/293x-972-MIMAP-356A-EXECUTION-SEAM-SUMMARY-CLOSEOUT.md"
NEXT_CARD="docs/development/current/main/phases/phase-293x/293x-973-MIMAP-357A-POST-EXECUTION-SEAM-SUMMARY-ROW-SELECTION.md"

printf '[%s] checking MIMAP-356A allocator execution seam summary closeout\n' "$TAG"

guard_require_files "$TAG" "$DESIGN" "$INDEX" "$PROOF_MANIFEST_INCLUDE" "$PROVIDER_GUARD" "$MATCHER_GUARD" "$SELF_SCRIPT" "$CARD_344A" "$CARD_345A" "$CARD_346A" "$CARD_347A" "$CARD_348A" "$CARD_349A" "$CARD_350A" "$CARD_352A" "$CARD_354A" "$CARD_355A" "$CARD" "$NEXT_CARD"
guard_require_exec_files "$TAG" "$PROVIDER_GUARD" "$MATCHER_GUARD" "$SELF_SCRIPT"

guard_expect_in_file "$TAG" 'Decision: accepted' "$DESIGN" "MIMAP-356A design must be accepted"
guard_expect_fixed_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list MIMAP-356A guard"

for card in "$CARD_344A" "$CARD_345A" "$CARD_346A" "$CARD_347A" "$CARD_348A" "$CARD_349A" "$CARD_350A" "$CARD_352A" "$CARD_354A" "$CARD_355A" "$CARD"; do
  guard_expect_in_file "$TAG" 'Status: landed' "$card" "$card must be landed"
done
guard_expect_in_file "$TAG" 'Status: selected current' "$NEXT_CARD" "MIMAP-357A must be selected current"

for row in MIMAP-344A MIMAP-345A MIMAP-346A MIMAP-347A MIMAP-348A MIMAP-349A MIMAP-350A MIMAP-352A; do
  guard_expect_in_file "$TAG" "id = \"$row\"" "$PROOF_MANIFEST_INCLUDE" "proof manifest must list $row"
done

bash "$PROVIDER_GUARD" --level L2
bash "$MATCHER_GUARD"

printf '[%s] ok\n' "$TAG"
