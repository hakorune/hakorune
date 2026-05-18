#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../../.." && pwd)"
TAG="k2-wide-hako-alloc-source-release-ledger-lifecycle-key-migration-closeout"
cd "$ROOT_DIR"
source "$ROOT_DIR/tools/checks/lib/pure_first_exe_guard.sh"

SSOT="docs/development/current/main/design/hako-alloc-segment-map-local-free-reuse-ledger-lifecycle-keyed-release-ledger-closeout-ssot.md"
LEDGER_SSOT="docs/development/current/main/design/hako-alloc-segment-map-local-free-reuse-ledger-lifecycle-keyed-release-ledger-ssot.md"
DIAGNOSTICS_SSOT="docs/development/current/main/design/hako-alloc-segment-map-local-free-reuse-ledger-lifecycle-keyed-release-ledger-diagnostics-ssot.md"
CADENCE="docs/development/current/main/design/mimalloc-row-validation-cadence-ssot.md"
TASKBOARD="docs/development/current/main/phases/phase-293x/293x-mimalloc-port-taskboard.md"
GRANULARITY="docs/development/current/main/design/mimalloc-allocator-first-task-granularity-ssot.md"
JOINT="docs/development/current/main/design/mimalloc-hakorune-joint-task-order-ssot.md"
INDEX="docs/tools/check-scripts-index.md"
PROOF_MANIFEST="tools/checks/proof_apps.toml"
GUARD_MANIFEST="tools/checks/guard_rows.toml"
CARD_228A="docs/development/current/main/phases/phase-293x/293x-751-MIMAP-228A-SOURCE-RELEASE-LEDGER-LIFECYCLE-KEY-MIGRATION-PILOT.md"
CARD_229A="docs/development/current/main/phases/phase-293x/293x-752-MIMAP-229A-SOURCE-LIFECYCLE-KEYED-RELEASE-LEDGER-DIAGNOSTICS.md"
CARD_230A="docs/development/current/main/phases/phase-293x/293x-753-MIMAP-230A-SOURCE-RELEASE-LEDGER-LIFECYCLE-KEY-MIGRATION-CLOSEOUT-PACK.md"
CARD_231A="docs/development/current/main/phases/phase-293x/293x-754-MIMAP-231A-POST-SOURCE-RELEASE-LEDGER-LIFECYCLE-KEY-MIGRATION-CLOSEOUT-ROW-SELECTION.md"
GUARD_228A="tools/checks/k2_wide_hako_alloc_segment_map_local_free_reuse_ledger_lifecycle_keyed_release_ledger_guard.sh"
GUARD_229A="tools/checks/k2_wide_hako_alloc_segment_map_local_free_reuse_ledger_lifecycle_keyed_release_ledger_diagnostics_guard.sh"
SELF_SCRIPT="tools/checks/k2_wide_hako_alloc_segment_map_local_free_reuse_ledger_lifecycle_keyed_release_ledger_closeout_guard.sh"
IMPL_SCRIPT="tools/checks/impl/k2_wide_hako_alloc_segment_map_local_free_reuse_ledger_lifecycle_keyed_release_ledger_closeout_guard.sh"
RUN_PROOF="tools/checks/run_proof_app.sh"

echo "[$TAG] checking MIMAP-230A source release-ledger lifecycle-key migration closeout"

guard_require_files \
  "$TAG" \
  "$SSOT" \
  "$LEDGER_SSOT" \
  "$DIAGNOSTICS_SSOT" \
  "$CADENCE" \
  "$TASKBOARD" \
  "$GRANULARITY" \
  "$JOINT" \
  "$INDEX" \
  "$PROOF_MANIFEST" \
  "$GUARD_MANIFEST" \
  "$CARD_228A" \
  "$CARD_229A" \
  "$CARD_230A" \
  "$CARD_231A" \
  "$GUARD_228A" \
  "$GUARD_229A" \
  "$SELF_SCRIPT" \
  "$IMPL_SCRIPT" \
  "$RUN_PROOF"

guard_require_exec_files "$TAG" "$GUARD_228A" "$GUARD_229A" "$SELF_SCRIPT" "$IMPL_SCRIPT" "$RUN_PROOF"

guard_expect_in_file "$TAG" "Status: landed" "$CARD_228A" "MIMAP-228A must be landed before closeout"
guard_expect_in_file "$TAG" "Status: landed" "$CARD_229A" "MIMAP-229A must be landed before closeout"
guard_expect_in_file "$TAG" "Status: landed" "$CARD_230A" "MIMAP-230A closeout card must be landed"
guard_expect_in_file "$TAG" "Status: selected current" "$CARD_231A" "MIMAP-231A must be selected current after closeout"

guard_expect_in_file "$TAG" "Decision: accepted" "$SSOT" "MIMAP-230A closeout SSOT must be accepted"
guard_expect_in_file "$TAG" "Decision: accepted" "$LEDGER_SSOT" "MIMAP-228A source ledger SSOT must stay accepted"
guard_expect_in_file "$TAG" "Decision: accepted" "$DIAGNOSTICS_SSOT" "MIMAP-229A diagnostics SSOT must stay accepted"
guard_expect_in_file "$TAG" "closeout_pack = source-release-ledger-lifecycle-key-migration" "$SSOT" "closeout SSOT must name pack"
guard_expect_in_file "$TAG" "MIMAP-231A post source release-ledger lifecycle-key migration closeout row selection" "$SSOT" "closeout SSOT must name next row"

guard_expect_in_file "$TAG" "source release-ledger lifecycle-key migration family" "$CADENCE" "cadence SSOT must define migration family"
guard_expect_in_file "$TAG" "representative L3 EXE evidence in MIMAP-228A and MIMAP-230A" "$CADENCE" "cadence SSOT must anchor MIMAP-230A L3 evidence"
guard_expect_in_file "$TAG" "MIMAP-230A" "$GRANULARITY" "granularity SSOT must describe MIMAP-230A"
guard_expect_in_file "$TAG" "MIMAP-231A" "$GRANULARITY" "granularity SSOT must describe MIMAP-231A"
guard_expect_in_file "$TAG" "MIMAP-230A source release-ledger lifecycle-key migration closeout pack" "$JOINT" "joint order must name closeout row"
guard_expect_in_file "$TAG" "MIMAP-231A post source release-ledger lifecycle-key migration closeout row selection" "$JOINT" "joint order must name next row"
guard_expect_in_file "$TAG" "MIMAP-231A" "$TASKBOARD" "taskboard must name selected next row"

guard_expect_in_file "$TAG" "id = \"MIMAP-228A\"" "$PROOF_MANIFEST" "proof manifest must include MIMAP-228A"
guard_expect_in_file "$TAG" "id = \"MIMAP-229A\"" "$PROOF_MANIFEST" "proof manifest must include MIMAP-229A"
guard_expect_in_file "$TAG" "closeout_pack = \"source-release-ledger-lifecycle-key-migration\"" "$PROOF_MANIFEST" "proof manifest must assign migration closeout pack"
guard_expect_in_file "$TAG" "id = \"hako-alloc-source-release-ledger-lifecycle-key-migration-closeout\"" "$GUARD_MANIFEST" "guard manifest must include MIMAP-230A closeout row"
guard_expect_in_file "$TAG" "$GUARD_228A" "$INDEX" "check index must list MIMAP-228A guard"
guard_expect_in_file "$TAG" "$GUARD_229A" "$INDEX" "check index must list MIMAP-229A guard"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list MIMAP-230A closeout guard"

bash "$RUN_PROOF" --closeout-pack source-release-ledger-lifecycle-key-migration --level L2 --dry-run >/tmp/"$TAG".proof_dry_run 2>&1 || {
  cat /tmp/"$TAG".proof_dry_run >&2
  rm -f /tmp/"$TAG".proof_dry_run
  guard_fail "$TAG" "source release-ledger lifecycle-key migration L2 dry-run selection failed"
}
guard_expect_in_file "$TAG" "MIMAP-228A" /tmp/"$TAG".proof_dry_run "L2 dry-run must include MIMAP-228A"
guard_expect_in_file "$TAG" "MIMAP-229A" /tmp/"$TAG".proof_dry_run "L2 dry-run must include MIMAP-229A"
rm -f /tmp/"$TAG".proof_dry_run

bash "$GUARD_228A" --level L3
bash "$GUARD_229A" --level L2

echo "[$TAG] ok"
