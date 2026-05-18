#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../../.." && pwd)"
TAG="k2-wide-hako-alloc-segment-map-readiness-closeout"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

SSOT="docs/development/current/main/design/hako-alloc-segment-map-readiness-closeout-ssot.md"
CADENCE="docs/development/current/main/design/mimalloc-row-validation-cadence-ssot.md"
MATRIX_SSOT="docs/development/current/main/design/hako-alloc-segment-allocation-blocked-substrate-matrix-ssot.md"
LOOKUP_SSOT="docs/development/current/main/design/hako-alloc-segment-map-scalar-lookup-boundary-inventory-ssot.md"
COMPOSITION_SSOT="docs/development/current/main/design/hako-alloc-segment-map-lookup-guarded-readiness-composition-ssot.md"
TASKBOARD="docs/development/current/main/phases/phase-293x/293x-mimalloc-port-taskboard.md"
GRANULARITY="docs/development/current/main/design/mimalloc-allocator-first-task-granularity-ssot.md"
JOINT="docs/development/current/main/design/mimalloc-hakorune-joint-task-order-ssot.md"
INDEX="docs/tools/check-scripts-index.md"
PROOF_MANIFEST="tools/checks/proof_apps.toml"
GUARD_MANIFEST="tools/checks/guard_rows.toml"
CARD_149A="docs/development/current/main/phases/phase-293x/293x-669-MIMAP-149A-SEGMENT-ALLOCATION-BLOCKED-SUBSTRATE-MATRIX-PROOF.md"
CARD_151A="docs/development/current/main/phases/phase-293x/293x-671-MIMAP-151A-SEGMENT-MAP-SCALAR-LOOKUP-BOUNDARY-INVENTORY.md"
CARD_153A="docs/development/current/main/phases/phase-293x/293x-673-MIMAP-153A-SEGMENT-MAP-LOOKUP-GUARDED-READINESS-COMPOSITION.md"
CARD_154A="docs/development/current/main/phases/phase-293x/293x-674-MIMAP-154A-POST-LOOKUP-GUARDED-READINESS-ROW-SELECTION.md"
CARD_155A="docs/development/current/main/phases/phase-293x/293x-677-MIMAP-155A-SEGMENT-MAP-READINESS-VALIDATION-PACK-CLOSEOUT-GUARD.md"
CARD_156A="docs/development/current/main/phases/phase-293x/293x-678-MIMAP-156A-POST-SEGMENT-MAP-READINESS-CLOSEOUT-ROW-SELECTION.md"
OWNER_149A="lang/src/hako_alloc/memory/segment_allocation_blocked_substrate_matrix_box.hako"
OWNER_151A="lang/src/hako_alloc/memory/segment_map_scalar_lookup_boundary_inventory_box.hako"
OWNER_153A="lang/src/hako_alloc/memory/segment_map_lookup_guarded_readiness_composition_box.hako"
APP_149A="apps/hako-alloc-segment-allocation-blocked-substrate-matrix-proof/main.hako"
APP_151A="apps/hako-alloc-segment-map-scalar-lookup-boundary-inventory-proof/main.hako"
APP_153A="apps/hako-alloc-segment-map-lookup-guarded-readiness-composition-proof/main.hako"
GUARD_149A="tools/checks/k2_wide_hako_alloc_segment_allocation_blocked_substrate_matrix_guard.sh"
GUARD_151A="tools/checks/k2_wide_hako_alloc_segment_map_scalar_lookup_boundary_inventory_guard.sh"
GUARD_153A="tools/checks/k2_wide_hako_alloc_segment_map_lookup_guarded_readiness_composition_guard.sh"
SELF_SCRIPT="tools/checks/k2_wide_hako_alloc_segment_map_readiness_closeout_guard.sh"
RUN_PROOF="tools/checks/run_proof_app.sh"

echo "[$TAG] checking MIMAP-155A segment-map readiness closeout"

guard_require_files \
  "$TAG" \
  "$SSOT" \
  "$CADENCE" \
  "$MATRIX_SSOT" \
  "$LOOKUP_SSOT" \
  "$COMPOSITION_SSOT" \
  "$TASKBOARD" \
  "$GRANULARITY" \
  "$JOINT" \
  "$INDEX" \
  "$PROOF_MANIFEST" \
  "$GUARD_MANIFEST" \
  "$CARD_149A" \
  "$CARD_151A" \
  "$CARD_153A" \
  "$CARD_154A" \
  "$CARD_155A" \
  "$CARD_156A" \
  "$OWNER_149A" \
  "$OWNER_151A" \
  "$OWNER_153A" \
  "$APP_149A" \
  "$APP_151A" \
  "$APP_153A" \
  "$GUARD_149A" \
  "$GUARD_151A" \
  "$GUARD_153A" \
  "$SELF_SCRIPT" \
  "$RUN_PROOF"

guard_require_exec_files "$TAG" "$GUARD_149A" "$GUARD_151A" "$GUARD_153A" "$SELF_SCRIPT" "$RUN_PROOF"

guard_expect_in_file "$TAG" "Status: landed" "$CARD_149A" "MIMAP-149A must be landed before closeout"
guard_expect_in_file "$TAG" "Status: landed" "$CARD_151A" "MIMAP-151A must be landed before closeout"
guard_expect_in_file "$TAG" "Status: landed" "$CARD_153A" "MIMAP-153A must be landed before closeout"
guard_expect_in_file "$TAG" "Status: landed" "$CARD_154A" "MIMAP-154A selection must be landed"
guard_expect_in_file "$TAG" "Status: landed" "$CARD_155A" "MIMAP-155A closeout card must be landed"
guard_expect_in_file "$TAG" "Status: selected current" "$CARD_156A" "MIMAP-156A must be the selected current row"

guard_expect_in_file "$TAG" "Decision: accepted" "$SSOT" "closeout SSOT must be accepted"
guard_expect_in_file "$TAG" "Decision: accepted" "$MATRIX_SSOT" "MIMAP-149A SSOT must stay accepted"
guard_expect_in_file "$TAG" "Decision: accepted" "$LOOKUP_SSOT" "MIMAP-151A SSOT must stay accepted"
guard_expect_in_file "$TAG" "Decision: accepted" "$COMPOSITION_SSOT" "MIMAP-153A SSOT must stay accepted"
guard_expect_in_file "$TAG" "MIMAP-149A" "$SSOT" "closeout SSOT must include blocked-substrate matrix row"
guard_expect_in_file "$TAG" "MIMAP-151A" "$SSOT" "closeout SSOT must include scalar lookup row"
guard_expect_in_file "$TAG" "MIMAP-153A" "$SSOT" "closeout SSOT must include lookup-guarded readiness row"
guard_expect_in_file "$TAG" "segment-map-readiness" "$SSOT" "closeout SSOT must name validation pack"
guard_expect_in_file "$TAG" "--level L2" "$SSOT" "closeout SSOT must freeze L2 daily command"
guard_expect_in_file "$TAG" "MIMAP-156A post-segment-map-readiness-closeout row selection" "$SSOT" "closeout SSOT must name next row"

guard_expect_in_file "$TAG" "cmd_l2" "$CADENCE" "cadence SSOT must document level-specific commands"
guard_expect_in_file "$TAG" "segment-map-readiness" "$CADENCE" "cadence SSOT must name segment-map readiness pack"
guard_expect_in_file "$TAG" "L2 MIR contract" "$CADENCE" "cadence SSOT must define L2 MIR contract"
guard_expect_in_file "$TAG" "MIMAP-155A" "$GRANULARITY" "granularity SSOT must describe MIMAP-155A"
guard_expect_in_file "$TAG" "MIMAP-155A segment-map readiness validation pack closeout guard" "$JOINT" "joint order must name MIMAP-155A"
guard_expect_in_file "$TAG" "MIMAP-156A post-segment-map-readiness-closeout row selection" "$JOINT" "joint order must name MIMAP-156A"
guard_expect_in_file "$TAG" "MIMAP-155A" "$TASKBOARD" "taskboard must list MIMAP-155A"
guard_expect_in_file "$TAG" "MIMAP-156A" "$TASKBOARD" "taskboard must list MIMAP-156A"

guard_expect_in_file "$TAG" "id = \"MIMAP-149A\"" "$PROOF_MANIFEST" "proof manifest must include MIMAP-149A"
guard_expect_in_file "$TAG" "id = \"MIMAP-151A\"" "$PROOF_MANIFEST" "proof manifest must include MIMAP-151A"
guard_expect_in_file "$TAG" "id = \"MIMAP-153A\"" "$PROOF_MANIFEST" "proof manifest must include MIMAP-153A"
guard_expect_in_file "$TAG" "closeout_pack = \"segment-map-readiness\"" "$PROOF_MANIFEST" "proof manifest must assign closeout pack"
guard_expect_in_file "$TAG" "cmd_l2" "$PROOF_MANIFEST" "proof manifest must keep L2 commands"
guard_expect_in_file "$TAG" "id = \"hako-alloc-segment-map-readiness-closeout\"" "$GUARD_MANIFEST" "guard manifest must include closeout row"
guard_expect_in_file "$TAG" "closeout_pack = \"segment-map-readiness\"" "$GUARD_MANIFEST" "guard manifest must assign closeout pack"
guard_expect_in_file "$TAG" "$GUARD_149A" "$INDEX" "check index must list MIMAP-149A guard"
guard_expect_in_file "$TAG" "$GUARD_151A" "$INDEX" "check index must list MIMAP-151A guard"
guard_expect_in_file "$TAG" "$GUARD_153A" "$INDEX" "check index must list MIMAP-153A guard"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list MIMAP-155A closeout guard"

guard_expect_in_file "$TAG" "box HakoAllocSegmentAllocationBlockedSubstrateMatrix" "$OWNER_149A" "MIMAP-149A owner box must stay present"
guard_expect_in_file "$TAG" "box HakoAllocSegmentMapScalarLookupBoundaryInventory" "$OWNER_151A" "MIMAP-151A owner box must stay present"
guard_expect_in_file "$TAG" "box HakoAllocSegmentMapLookupGuardedReadinessComposition" "$OWNER_153A" "MIMAP-153A owner box must stay present"
guard_expect_in_file "$TAG" "HakoAllocSegmentAllocationBlockedSubstrateMatrix" "$APP_149A" "MIMAP-149A proof must construct owner"
guard_expect_in_file "$TAG" "HakoAllocSegmentMapScalarLookupBoundaryInventory" "$APP_151A" "MIMAP-151A proof must construct owner"
guard_expect_in_file "$TAG" "HakoAllocSegmentMapLookupGuardedReadinessComposition" "$APP_153A" "MIMAP-153A proof must construct owner"

bash "$RUN_PROOF" --closeout-pack segment-map-readiness --level L2 --dry-run >/tmp/"$TAG".proof_dry_run 2>&1 || {
  cat /tmp/"$TAG".proof_dry_run >&2
  rm -f /tmp/"$TAG".proof_dry_run
  guard_fail "$TAG" "segment-map readiness L2 dry-run selection failed"
}
guard_expect_in_file "$TAG" "MIMAP-149A" /tmp/"$TAG".proof_dry_run "L2 dry-run must include MIMAP-149A"
guard_expect_in_file "$TAG" "MIMAP-151A" /tmp/"$TAG".proof_dry_run "L2 dry-run must include MIMAP-151A"
guard_expect_in_file "$TAG" "MIMAP-153A" /tmp/"$TAG".proof_dry_run "L2 dry-run must include MIMAP-153A"
rm -f /tmp/"$TAG".proof_dry_run

if rg -n 'hako-alloc-segment-allocation-blocked-substrate-matrix-proof|hako-alloc-segment-map-scalar-lookup-boundary-inventory-proof|hako-alloc-segment-map-lookup-guarded-readiness-composition-proof|HakoAllocSegmentAllocationBlockedSubstrateMatrix|HakoAllocSegmentMapScalarLookupBoundaryInventory|HakoAllocSegmentMapLookupGuardedReadinessComposition|segment_allocation_blocked_substrate_matrix|segment_map_scalar_lookup_boundary_inventory|segment_map_lookup_guarded_readiness_composition' \
  lang/c-abi/shims >/tmp/"$TAG".inc_leak 2>&1; then
  cat /tmp/"$TAG".inc_leak >&2
  rm -f /tmp/"$TAG".inc_leak
  guard_fail "$TAG" "segment-map readiness family matcher leaked into .inc"
fi
rm -f /tmp/"$TAG".inc_leak

bash tools/checks/allocator_provider_inactive_sentinel_guard.sh >/tmp/"$TAG".provider_sentinel 2>&1 || {
  cat /tmp/"$TAG".provider_sentinel >&2
  rm -f /tmp/"$TAG".provider_sentinel
  guard_fail "$TAG" "allocator provider inactive sentinel failed"
}
rm -f /tmp/"$TAG".provider_sentinel

echo "[$TAG] ok"
