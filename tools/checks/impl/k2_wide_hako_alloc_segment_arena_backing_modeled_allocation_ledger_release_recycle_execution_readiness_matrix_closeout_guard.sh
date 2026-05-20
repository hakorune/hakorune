#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../../.." && pwd)"
TAG="k2-wide-hako-alloc-segment-arena-backing-modeled-allocation-ledger-release-recycle-execution-readiness-matrix-closeout"
cd "$ROOT_DIR"
source "$ROOT_DIR/tools/checks/lib/pure_first_exe_guard.sh"

SSOT="docs/development/current/main/design/hako-alloc-segment-arena-backing-modeled-allocation-ledger-release-recycle-execution-readiness-matrix-closeout-ssot.md"
MATRIX_SSOT="docs/development/current/main/design/hako-alloc-segment-arena-backing-modeled-allocation-ledger-release-recycle-execution-readiness-matrix-ssot.md"
DIAGNOSTICS_SSOT="docs/development/current/main/design/hako-alloc-segment-arena-backing-modeled-allocation-ledger-release-recycle-execution-readiness-matrix-diagnostics-ssot.md"
TASKBOARD="docs/development/current/main/phases/phase-293x/293x-mimalloc-port-taskboard.md"
INDEX="docs/tools/check-scripts-index.md"
PROOF_MANIFEST_INCLUDE="tools/checks/manifests/proof_apps/hako_alloc_segment_arena_backing_release_lifecycle.toml"
GUARD_MANIFEST_INCLUDE="tools/checks/manifests/guard_rows/hako_alloc_closeout.toml"
CARD_312A="docs/development/current/main/phases/phase-293x/293x-927-MIMAP-312A-SEGMENT-ARENA-BACKING-MODELED-ALLOCATION-LEDGER-RELEASE-RECYCLE-EXECUTION-READINESS-MATRIX-INVENTORY.md"
CARD_313A="docs/development/current/main/phases/phase-293x/293x-928-MIMAP-313A-SEGMENT-ARENA-BACKING-MODELED-ALLOCATION-LEDGER-RELEASE-RECYCLE-EXECUTION-READINESS-MATRIX-DIAGNOSTICS.md"
CARD_314A="docs/development/current/main/phases/phase-293x/293x-929-MIMAP-314A-SEGMENT-ARENA-BACKING-MODELED-ALLOCATION-LEDGER-RELEASE-RECYCLE-EXECUTION-READINESS-MATRIX-CLOSEOUT.md"
CARD_315A="docs/development/current/main/phases/phase-293x/293x-930-MIMAP-315A-POST-RELEASE-RECYCLE-EXECUTION-READINESS-MATRIX-CLOSEOUT-ROW-SELECTION.md"
GUARD_312A="tools/checks/k2_wide_hako_alloc_segment_arena_backing_modeled_allocation_ledger_release_recycle_execution_readiness_matrix_guard.sh"
GUARD_313A="tools/checks/k2_wide_hako_alloc_segment_arena_backing_modeled_allocation_ledger_release_recycle_execution_readiness_matrix_diagnostics_guard.sh"
SELF_SCRIPT="tools/checks/k2_wide_hako_alloc_segment_arena_backing_modeled_allocation_ledger_release_recycle_execution_readiness_matrix_closeout_guard.sh"
IMPL_SCRIPT="tools/checks/impl/k2_wide_hako_alloc_segment_arena_backing_modeled_allocation_ledger_release_recycle_execution_readiness_matrix_closeout_guard.sh"
RUN_PROOF="tools/checks/run_proof_app.sh"
APP="apps/hako-alloc-segment-arena-backing-modeled-allocation-ledger-release-recycle-execution-readiness-matrix-diagnostics-proof/main.hako"

echo "[$TAG] checking MIMAP-314A segment arena backing modeled allocation-ledger release/recycle execution readiness matrix closeout"

guard_require_files "$TAG" "$SSOT" "$MATRIX_SSOT" "$DIAGNOSTICS_SSOT" "$TASKBOARD" "$INDEX" "$PROOF_MANIFEST_INCLUDE" "$GUARD_MANIFEST_INCLUDE" "$CARD_312A" "$CARD_313A" "$CARD_314A" "$CARD_315A" "$GUARD_312A" "$GUARD_313A" "$SELF_SCRIPT" "$IMPL_SCRIPT" "$RUN_PROOF" "$APP"
guard_require_exec_files "$TAG" "$GUARD_312A" "$GUARD_313A" "$SELF_SCRIPT" "$IMPL_SCRIPT" "$RUN_PROOF"

guard_expect_in_file "$TAG" "Status: landed" "$CARD_312A" "MIMAP-312A must be landed before closeout"
guard_expect_in_file "$TAG" "Status: landed" "$CARD_313A" "MIMAP-313A must be landed before closeout"
guard_expect_in_file "$TAG" "Status: landed" "$CARD_314A" "MIMAP-314A closeout card must be landed"
guard_expect_in_file "$TAG" "MIMAP-315A" "$CARD_315A" "MIMAP-315A selection card must exist after closeout"
guard_expect_in_file "$TAG" "Decision: accepted" "$SSOT" "MIMAP-314A closeout SSOT must be accepted"
guard_expect_in_file "$TAG" "Decision: accepted" "$MATRIX_SSOT" "MIMAP-312A matrix SSOT must stay accepted"
guard_expect_in_file "$TAG" "Decision: accepted" "$DIAGNOSTICS_SSOT" "MIMAP-313A diagnostics SSOT must stay accepted"
guard_expect_in_file "$TAG" "segment-arena-backing-modeled-allocation-ledger-release-recycle-execution-readiness-matrix" "$SSOT" "closeout SSOT must name matrix pack"
guard_expect_in_file "$TAG" "MIMAP-315A post release/recycle execution readiness matrix closeout row selection" "$SSOT" "closeout SSOT must name next row"
guard_expect_in_file "$TAG" "id = \"MIMAP-312A\"" "$PROOF_MANIFEST_INCLUDE" "proof manifest must include MIMAP-312A"
guard_expect_in_file "$TAG" "id = \"MIMAP-313A\"" "$PROOF_MANIFEST_INCLUDE" "proof manifest must include MIMAP-313A"
guard_expect_in_file "$TAG" "closeout_pack = \"segment-arena-backing-modeled-allocation-ledger-release-recycle-execution-readiness-matrix\"" "$PROOF_MANIFEST_INCLUDE" "proof manifest must assign matrix closeout pack"
guard_expect_in_file "$TAG" "id = \"hako-alloc-segment-arena-backing-modeled-allocation-ledger-release-recycle-execution-readiness-matrix-closeout\"" "$GUARD_MANIFEST_INCLUDE" "guard manifest must include MIMAP-314A closeout row"
guard_expect_fixed_in_file "$TAG" "$GUARD_312A" "$INDEX" "check index must list MIMAP-312A guard"
guard_expect_fixed_in_file "$TAG" "$GUARD_313A" "$INDEX" "check index must list MIMAP-313A guard"
guard_expect_fixed_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list MIMAP-314A closeout guard"

bash "$RUN_PROOF" --closeout-pack segment-arena-backing-modeled-allocation-ledger-release-recycle-execution-readiness-matrix --level L2 --dry-run >/tmp/"$TAG".proof_dry_run 2>&1 || {
  cat /tmp/"$TAG".proof_dry_run >&2
  rm -f /tmp/"$TAG".proof_dry_run
  guard_fail "$TAG" "execution readiness matrix L2 dry-run selection failed"
}
guard_expect_in_file "$TAG" "MIMAP-312A" /tmp/"$TAG".proof_dry_run "L2 dry-run must include MIMAP-312A"
guard_expect_in_file "$TAG" "MIMAP-313A" /tmp/"$TAG".proof_dry_run "L2 dry-run must include MIMAP-313A"
rm -f /tmp/"$TAG".proof_dry_run

bash "$GUARD_312A" --level L2
bash "$GUARD_313A" --level L2

tmp_dir="$(mktemp -d /tmp/hakorune_mimap314_execution_readiness_matrix_closeout.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT
mir_json="$tmp_dir/mimap314.mir.json"
exe_out="$tmp_dir/mimap314_exe"
build_log="$tmp_dir/build.log"
run_log="$tmp_dir/run.log"

pure_first_guard_emit_mir "$ROOT_DIR" "$APP" "$mir_json"
pure_first_guard_build_exe "$TAG" "$ROOT_DIR" "$APP" "$mir_json" "$exe_out" "$build_log"
pure_first_guard_assert_clean_build_log "$TAG" "$build_log"
pure_first_guard_run_exe "$TAG" "$exe_out" "$run_log"

rg -F -q 'hako-alloc-segment-arena-backing-modeled-allocation-ledger-release-recycle-execution-readiness-matrix-diagnostics-proof' "$run_log"
rg -F -q 'diag=1,0,1,0,1,97019005005,99019005005' "$run_log"
rg -F -q 'bytes=4096,4096,8192' "$run_log"
rg -F -q 'owner=4,1,3,1,1,1,3' "$run_log"
rg -F -q 'rejected=0,1,0,2,1,3,8' "$run_log"
rg -F -q 'inactive=0,0,0,0,0,0,0,0,0,0,0' "$run_log"
rg -F -q 'check=1' "$run_log"
rg -F -q 'summary=ok' "$run_log"

echo "[$TAG] ok"
