#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../../.." && pwd)"
TAG="k2-wide-hako-alloc-source-lifecycle-keyed-release-apply-recycle-continuation-closeout"
cd "$ROOT_DIR"
source "$ROOT_DIR/tools/checks/lib/pure_first_exe_guard.sh"

SSOT="docs/development/current/main/design/hako-alloc-segment-map-local-free-reuse-ledger-lifecycle-keyed-release-apply-recycle-continuation-closeout-ssot.md"
BRIDGE_SSOT="docs/development/current/main/design/hako-alloc-segment-map-local-free-reuse-ledger-lifecycle-keyed-release-apply-recycle-continuation-ssot.md"
DIAGNOSTICS_SSOT="docs/development/current/main/design/hako-alloc-segment-map-local-free-reuse-ledger-lifecycle-keyed-release-apply-recycle-continuation-diagnostics-ssot.md"
CADENCE="docs/development/current/main/design/mimalloc-row-validation-cadence-ssot.md"
TASKBOARD="docs/development/current/main/phases/phase-293x/293x-mimalloc-port-taskboard.md"
GRANULARITY="docs/development/current/main/design/mimalloc-allocator-first-task-granularity-ssot.md"
JOINT="docs/development/current/main/design/mimalloc-hakorune-joint-task-order-ssot.md"
INDEX="docs/tools/check-scripts-index.md"
PROOF_MANIFEST="tools/checks/proof_apps.toml"
GUARD_MANIFEST="tools/checks/guard_rows.toml"
CARD_232A="docs/development/current/main/phases/phase-293x/293x-755-MIMAP-232A-SOURCE-LIFECYCLE-KEYED-RELEASE-APPLY-RECYCLE-CONTINUATION-BRIDGE.md"
CARD_233A="docs/development/current/main/phases/phase-293x/293x-756-MIMAP-233A-SOURCE-LIFECYCLE-KEYED-RELEASE-APPLY-RECYCLE-CONTINUATION-DIAGNOSTICS.md"
CARD_234A="docs/development/current/main/phases/phase-293x/293x-757-MIMAP-234A-SOURCE-LIFECYCLE-KEYED-RELEASE-APPLY-RECYCLE-CONTINUATION-CLOSEOUT-PACK.md"
CARD_235A="docs/development/current/main/phases/phase-293x/293x-758-MIMAP-235A-POST-SOURCE-LIFECYCLE-KEYED-RELEASE-APPLY-RECYCLE-CONTINUATION-CLOSEOUT-ROW-SELECTION.md"
GUARD_232A="tools/checks/k2_wide_hako_alloc_segment_map_local_free_reuse_ledger_lifecycle_keyed_release_apply_recycle_continuation_guard.sh"
GUARD_233A="tools/checks/k2_wide_hako_alloc_segment_map_local_free_reuse_ledger_lifecycle_keyed_release_apply_recycle_continuation_diagnostics_guard.sh"
SELF_SCRIPT="tools/checks/k2_wide_hako_alloc_segment_map_local_free_reuse_ledger_lifecycle_keyed_release_apply_recycle_continuation_closeout_guard.sh"
IMPL_SCRIPT="tools/checks/impl/k2_wide_hako_alloc_segment_map_local_free_reuse_ledger_lifecycle_keyed_release_apply_recycle_continuation_closeout_guard.sh"
RUN_PROOF="tools/checks/run_proof_app.sh"

echo "[$TAG] checking MIMAP-234A source lifecycle-keyed release apply/recycle continuation closeout"

guard_require_files \
  "$TAG" \
  "$SSOT" \
  "$BRIDGE_SSOT" \
  "$DIAGNOSTICS_SSOT" \
  "$CADENCE" \
  "$TASKBOARD" \
  "$GRANULARITY" \
  "$JOINT" \
  "$INDEX" \
  "$PROOF_MANIFEST" \
  "$GUARD_MANIFEST" \
  "$CARD_232A" \
  "$CARD_233A" \
  "$CARD_234A" \
  "$CARD_235A" \
  "$GUARD_232A" \
  "$GUARD_233A" \
  "$SELF_SCRIPT" \
  "$IMPL_SCRIPT" \
  "$RUN_PROOF"

guard_require_exec_files "$TAG" "$GUARD_232A" "$GUARD_233A" "$SELF_SCRIPT" "$IMPL_SCRIPT" "$RUN_PROOF"

guard_expect_in_file "$TAG" "Status: landed" "$CARD_232A" "MIMAP-232A must be landed before closeout"
guard_expect_in_file "$TAG" "Status: landed" "$CARD_233A" "MIMAP-233A must be landed before closeout"
guard_expect_in_file "$TAG" "Status: landed" "$CARD_234A" "MIMAP-234A closeout card must be landed"
guard_expect_in_file "$TAG" "Status: selected current" "$CARD_235A" "MIMAP-235A must be selected current after closeout"

guard_expect_in_file "$TAG" "Decision: accepted" "$SSOT" "MIMAP-234A closeout SSOT must be accepted"
guard_expect_in_file "$TAG" "Decision: accepted" "$BRIDGE_SSOT" "MIMAP-232A bridge SSOT must stay accepted"
guard_expect_in_file "$TAG" "Decision: accepted" "$DIAGNOSTICS_SSOT" "MIMAP-233A diagnostics SSOT must stay accepted"
guard_expect_in_file "$TAG" "closeout_pack = source-lifecycle-keyed-release-apply-recycle-continuation" "$SSOT" "closeout SSOT must name pack"
guard_expect_in_file "$TAG" "MIMAP-235A post source lifecycle-keyed release apply/recycle continuation closeout row selection" "$SSOT" "closeout SSOT must name next row"

guard_expect_in_file "$TAG" "source lifecycle-keyed release apply/recycle continuation family" "$CADENCE" "cadence SSOT must define continuation family"
guard_expect_in_file "$TAG" "representative L3 EXE evidence in MIMAP-232A and MIMAP-234A" "$CADENCE" "cadence SSOT must anchor MIMAP-234A L3 evidence"
guard_expect_in_file "$TAG" "MIMAP-234A" "$GRANULARITY" "granularity SSOT must describe MIMAP-234A"
guard_expect_in_file "$TAG" "MIMAP-235A" "$GRANULARITY" "granularity SSOT must describe MIMAP-235A"
guard_expect_in_file "$TAG" "MIMAP-234A source lifecycle-keyed release apply/recycle continuation closeout pack" "$JOINT" "joint order must name closeout row"
guard_expect_in_file "$TAG" "MIMAP-235A post source lifecycle-keyed release apply/recycle continuation closeout row selection" "$JOINT" "joint order must name next row"
guard_expect_in_file "$TAG" "MIMAP-235A" "$TASKBOARD" "taskboard must name selected next row"

guard_expect_in_file "$TAG" "id = \"MIMAP-232A\"" "$PROOF_MANIFEST" "proof manifest must include MIMAP-232A"
guard_expect_in_file "$TAG" "id = \"MIMAP-233A\"" "$PROOF_MANIFEST" "proof manifest must include MIMAP-233A"
guard_expect_in_file "$TAG" "closeout_pack = \"source-lifecycle-keyed-release-apply-recycle-continuation\"" "$PROOF_MANIFEST" "proof manifest must assign continuation closeout pack"
guard_expect_in_file "$TAG" "id = \"hako-alloc-source-lifecycle-keyed-release-apply-recycle-continuation-closeout\"" "$GUARD_MANIFEST" "guard manifest must include MIMAP-234A closeout row"
guard_expect_in_file "$TAG" "$GUARD_232A" "$INDEX" "check index must list MIMAP-232A guard"
guard_expect_in_file "$TAG" "$GUARD_233A" "$INDEX" "check index must list MIMAP-233A guard"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list MIMAP-234A closeout guard"

bash "$RUN_PROOF" --closeout-pack source-lifecycle-keyed-release-apply-recycle-continuation --level L2 --dry-run >/tmp/"$TAG".proof_dry_run 2>&1 || {
  cat /tmp/"$TAG".proof_dry_run >&2
  rm -f /tmp/"$TAG".proof_dry_run
  guard_fail "$TAG" "source lifecycle-keyed release apply/recycle continuation L2 dry-run selection failed"
}
guard_expect_in_file "$TAG" "MIMAP-232A" /tmp/"$TAG".proof_dry_run "L2 dry-run must include MIMAP-232A"
guard_expect_in_file "$TAG" "MIMAP-233A" /tmp/"$TAG".proof_dry_run "L2 dry-run must include MIMAP-233A"
rm -f /tmp/"$TAG".proof_dry_run

bash "$GUARD_232A" --level L3
bash "$GUARD_233A" --level L2

echo "[$TAG] ok"
