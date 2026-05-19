#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../../.." && pwd)"
TAG="k2-wide-hako-alloc-segment-arena-backing-modeled-allocation-ledger-release-candidate-closeout"
cd "$ROOT_DIR"
source "$ROOT_DIR/tools/checks/lib/pure_first_exe_guard.sh"

SSOT="docs/development/current/main/design/hako-alloc-segment-arena-backing-modeled-allocation-ledger-release-candidate-closeout-ssot.md"
INVENTORY_SSOT="docs/development/current/main/design/hako-alloc-segment-arena-backing-modeled-allocation-ledger-release-candidate-ssot.md"
DIAGNOSTICS_SSOT="docs/development/current/main/design/hako-alloc-segment-arena-backing-modeled-allocation-ledger-release-candidate-diagnostics-ssot.md"
TASKBOARD="docs/development/current/main/phases/phase-293x/293x-mimalloc-port-taskboard.md"
JOINT="docs/development/current/main/design/mimalloc-hakorune-joint-task-order-ssot.md"
INDEX="docs/tools/check-scripts-index.md"
PROOF_MANIFEST="tools/checks/proof_apps.toml"
GUARD_MANIFEST="tools/checks/guard_rows.toml"
CARD_280A="docs/development/current/main/phases/phase-293x/293x-805-MIMAP-280A-SEGMENT-ARENA-BACKING-MODELED-ALLOCATION-LEDGER-RELEASE-CANDIDATE-INVENTORY.md"
CARD_281A="docs/development/current/main/phases/phase-293x/293x-806-MIMAP-281A-SEGMENT-ARENA-BACKING-MODELED-ALLOCATION-LEDGER-RELEASE-CANDIDATE-DIAGNOSTICS.md"
CARD_282A="docs/development/current/main/phases/phase-293x/293x-808-MIMAP-282A-SEGMENT-ARENA-BACKING-MODELED-ALLOCATION-LEDGER-RELEASE-CANDIDATE-CLOSEOUT.md"
CARD_USIZE="docs/development/current/main/phases/phase-293x/293x-809-HAKO-ALLOC-USIZE-FIELD-GROUP-001-BYTE-CAPACITY-FIELD-GROUP-SELECTION.md"
GUARD_280A="tools/checks/k2_wide_hako_alloc_segment_arena_backing_modeled_allocation_ledger_release_candidate_guard.sh"
GUARD_281A="tools/checks/k2_wide_hako_alloc_segment_arena_backing_modeled_allocation_ledger_release_candidate_diagnostics_guard.sh"
SELF_SCRIPT="tools/checks/k2_wide_hako_alloc_segment_arena_backing_modeled_allocation_ledger_release_candidate_closeout_guard.sh"
IMPL_SCRIPT="tools/checks/impl/k2_wide_hako_alloc_segment_arena_backing_modeled_allocation_ledger_release_candidate_closeout_guard.sh"
RUN_PROOF="tools/checks/run_proof_app.sh"
APP="apps/hako-alloc-segment-arena-backing-modeled-allocation-ledger-release-candidate-diagnostics-proof/main.hako"

echo "[$TAG] checking MIMAP-282A segment arena backing modeled allocation-ledger release candidate closeout"

guard_require_files \
  "$TAG" \
  "$SSOT" \
  "$INVENTORY_SSOT" \
  "$DIAGNOSTICS_SSOT" \
  "$TASKBOARD" \
  "$JOINT" \
  "$INDEX" \
  "$PROOF_MANIFEST" \
  "$GUARD_MANIFEST" \
  "$CARD_280A" \
  "$CARD_281A" \
  "$CARD_282A" \
  "$CARD_USIZE" \
  "$GUARD_280A" \
  "$GUARD_281A" \
  "$SELF_SCRIPT" \
  "$IMPL_SCRIPT" \
  "$RUN_PROOF" \
  "$APP"

guard_require_exec_files "$TAG" "$GUARD_280A" "$GUARD_281A" "$SELF_SCRIPT" "$IMPL_SCRIPT" "$RUN_PROOF"

guard_expect_in_file "$TAG" "Status: landed" "$CARD_280A" "MIMAP-280A must be landed before closeout"
guard_expect_in_file "$TAG" "Status: landed" "$CARD_281A" "MIMAP-281A must be landed before closeout"
guard_expect_in_file "$TAG" "Status: landed" "$CARD_282A" "MIMAP-282A closeout card must be landed"
guard_expect_in_file "$TAG" "Status: selected current" "$CARD_USIZE" "HAKO-ALLOC-USIZE-FIELD-GROUP-001 must be selected current after closeout"

guard_expect_in_file "$TAG" "Decision: accepted" "$SSOT" "MIMAP-282A closeout SSOT must be accepted"
guard_expect_in_file "$TAG" "Decision: accepted" "$INVENTORY_SSOT" "MIMAP-280A inventory SSOT must stay accepted"
guard_expect_in_file "$TAG" "Decision: accepted" "$DIAGNOSTICS_SSOT" "MIMAP-281A diagnostics SSOT must stay accepted"
guard_expect_in_file "$TAG" "closeout_pack:" "$SSOT" "closeout SSOT must name pack"
guard_expect_in_file "$TAG" "segment-arena-backing-modeled-allocation-ledger-release-candidate" "$SSOT" "closeout SSOT must name release-candidate pack"
guard_expect_in_file "$TAG" "HAKO-ALLOC-USIZE-FIELD-GROUP-001 select allocator byte/capacity field-group pilot" "$SSOT" "closeout SSOT must name next row"
guard_expect_in_file "$TAG" "No exact-\`usize\` stored field migration" "$SSOT" "closeout SSOT must keep usize migration out of closeout"

guard_expect_in_file "$TAG" "MIMAP-282A segment arena backing modeled allocation-ledger release candidate closeout pack" "$JOINT" "joint order must name closeout row"
guard_expect_in_file "$TAG" "HAKO-ALLOC-USIZE-FIELD-GROUP-001 select allocator byte/capacity field-group pilot" "$JOINT" "joint order must name next usize field-group selection row"
guard_expect_in_file "$TAG" "HAKO-ALLOC-USIZE-FIELD-GROUP-001" "$TASKBOARD" "taskboard must name selected next usize field-group row"

guard_expect_in_file "$TAG" "id = \"MIMAP-280A\"" "$PROOF_MANIFEST" "proof manifest must include MIMAP-280A"
guard_expect_in_file "$TAG" "id = \"MIMAP-281A\"" "$PROOF_MANIFEST" "proof manifest must include MIMAP-281A"
guard_expect_in_file "$TAG" "closeout_pack = \"segment-arena-backing-modeled-allocation-ledger-release-candidate\"" "$PROOF_MANIFEST" "proof manifest must assign release-candidate closeout pack"
guard_expect_in_file "$TAG" "id = \"hako-alloc-segment-arena-backing-modeled-allocation-ledger-release-candidate-closeout\"" "$GUARD_MANIFEST" "guard manifest must include MIMAP-282A closeout row"
guard_expect_in_file "$TAG" "$GUARD_280A" "$INDEX" "check index must list MIMAP-280A guard"
guard_expect_in_file "$TAG" "$GUARD_281A" "$INDEX" "check index must list MIMAP-281A guard"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list MIMAP-282A closeout guard"

bash "$RUN_PROOF" --closeout-pack segment-arena-backing-modeled-allocation-ledger-release-candidate --level L2 --dry-run >/tmp/"$TAG".proof_dry_run 2>&1 || {
  cat /tmp/"$TAG".proof_dry_run >&2
  rm -f /tmp/"$TAG".proof_dry_run
  guard_fail "$TAG" "release-candidate L2 dry-run selection failed"
}
guard_expect_in_file "$TAG" "MIMAP-280A" /tmp/"$TAG".proof_dry_run "L2 dry-run must include MIMAP-280A"
guard_expect_in_file "$TAG" "MIMAP-281A" /tmp/"$TAG".proof_dry_run "L2 dry-run must include MIMAP-281A"
rm -f /tmp/"$TAG".proof_dry_run

bash "$GUARD_280A" --level L2
bash "$GUARD_281A" --level L2

tmp_dir="$(mktemp -d /tmp/hakorune_mimap282_release_candidate_closeout.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT

mir_json="$tmp_dir/mimap282.mir.json"
exe_out="$tmp_dir/mimap282_exe"
build_log="$tmp_dir/build.log"
run_log="$tmp_dir/run.log"

pure_first_guard_emit_mir "$ROOT_DIR" "$APP" "$mir_json"
pure_first_guard_build_exe "$TAG" "$ROOT_DIR" "$APP" "$mir_json" "$exe_out" "$build_log"
pure_first_guard_assert_clean_build_log "$TAG" "$build_log"
pure_first_guard_run_exe "$TAG" "$exe_out" "$run_log"

rg -F -q 'hako-alloc-segment-arena-backing-modeled-allocation-ledger-release-candidate-diagnostics-proof' "$run_log"
rg -F -q 'diag=1,0,6,1,5,150,15,94015005005' "$run_log"
rg -F -q 'seen=1,1,1,1,1' "$run_log"
rg -F -q 'candidate=93015005005,94015005005,4096,4096,8192' "$run_log"
rg -F -q 'present=1,1,1' "$run_log"
rg -F -q 'owner=2,1,1,1,1' "$run_log"
rg -F -q 'empty=0,1,0' "$run_log"
rg -F -q 'inactive=0,0,0,0,0,0,0,0,0' "$run_log"
rg -F -q 'check=1' "$run_log"
rg -F -q 'summary=ok' "$run_log"

echo "[$TAG] ok"
