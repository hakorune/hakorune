#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../../.." && pwd)"
TAG="k2-wide-hako-alloc-segment-map-local-free-reuse-ledger-release-applied-recycle-second-release-diagnostic-closeout"
cd "$ROOT_DIR"
source "$ROOT_DIR/tools/checks/lib/pure_first_exe_guard.sh"

SSOT="docs/development/current/main/design/hako-alloc-segment-map-local-free-reuse-ledger-release-applied-recycle-second-release-diagnostic-closeout-ssot.md"
DIAGNOSTIC_SSOT="docs/development/current/main/design/hako-alloc-segment-map-local-free-reuse-ledger-release-applied-recycle-second-release-diagnostic-ssot.md"
CADENCE="docs/development/current/main/design/mimalloc-row-validation-cadence-ssot.md"
TASKBOARD="docs/development/current/main/phases/phase-293x/293x-mimalloc-port-taskboard.md"
GRANULARITY="docs/development/current/main/design/mimalloc-allocator-first-task-granularity-ssot.md"
JOINT="docs/development/current/main/design/mimalloc-hakorune-joint-task-order-ssot.md"
INDEX="docs/tools/check-scripts-index.md"
PROOF_MANIFEST="tools/checks/proof_apps.toml"
GUARD_MANIFEST="tools/checks/guard_rows.toml"
CARD_208A="docs/development/current/main/phases/phase-293x/293x-731-MIMAP-208A-SEGMENT-MAP-LOCAL-FREE-REUSE-LEDGER-RELEASE-APPLIED-RECYCLE-SECOND-RELEASE-DIAGNOSTIC.md"
CARD_209A="docs/development/current/main/phases/phase-293x/293x-732-MIMAP-209A-POST-SEGMENT-MAP-LOCAL-FREE-REUSE-LEDGER-RELEASE-APPLIED-RECYCLE-SECOND-RELEASE-DIAGNOSTIC-ROW-SELECTION.md"
CARD_210A="docs/development/current/main/phases/phase-293x/293x-733-MIMAP-210A-SEGMENT-MAP-LOCAL-FREE-REUSE-LEDGER-RELEASE-APPLIED-RECYCLE-SECOND-RELEASE-DIAGNOSTIC-CLOSEOUT-PACK.md"
CARD_211A="docs/development/current/main/phases/phase-293x/293x-734-MIMAP-211A-POST-SEGMENT-MAP-LOCAL-FREE-REUSE-LEDGER-RELEASE-APPLIED-RECYCLE-SECOND-RELEASE-DIAGNOSTIC-CLOSEOUT-ROW-SELECTION.md"
APP="apps/hako-alloc-segment-map-local-free-reuse-ledger-release-applied-recycle-second-release-diagnostic-proof/main.hako"
APP_README="apps/hako-alloc-segment-map-local-free-reuse-ledger-release-applied-recycle-second-release-diagnostic-proof/README.md"
APP_TEST="apps/hako-alloc-segment-map-local-free-reuse-ledger-release-applied-recycle-second-release-diagnostic-proof/test.sh"
GUARD_208A="tools/checks/k2_wide_hako_alloc_segment_map_local_free_reuse_ledger_release_applied_recycle_second_release_diagnostic_guard.sh"
SELF_SCRIPT="tools/checks/k2_wide_hako_alloc_segment_map_local_free_reuse_ledger_release_applied_recycle_second_release_diagnostic_closeout_guard.sh"
IMPL_SCRIPT="tools/checks/impl/k2_wide_hako_alloc_segment_map_local_free_reuse_ledger_release_applied_recycle_second_release_diagnostic_closeout_guard.sh"
RUN_PROOF="tools/checks/run_proof_app.sh"

echo "[$TAG] checking MIMAP-210A release-applied recycle second-release diagnostic closeout"

guard_require_files \
  "$TAG" \
  "$SSOT" \
  "$DIAGNOSTIC_SSOT" \
  "$CADENCE" \
  "$TASKBOARD" \
  "$GRANULARITY" \
  "$JOINT" \
  "$INDEX" \
  "$PROOF_MANIFEST" \
  "$GUARD_MANIFEST" \
  "$CARD_208A" \
  "$CARD_209A" \
  "$CARD_210A" \
  "$CARD_211A" \
  "$APP" \
  "$APP_README" \
  "$APP_TEST" \
  "$GUARD_208A" \
  "$SELF_SCRIPT" \
  "$IMPL_SCRIPT" \
  "$RUN_PROOF"

guard_require_exec_files "$TAG" "$APP_TEST" "$GUARD_208A" "$SELF_SCRIPT" "$IMPL_SCRIPT" "$RUN_PROOF"

guard_expect_in_file "$TAG" "Status: landed" "$CARD_208A" "MIMAP-208A must be landed before closeout"
guard_expect_in_file "$TAG" "Status: landed" "$CARD_209A" "MIMAP-209A must be landed before closeout"
guard_expect_in_file "$TAG" "Status: landed" "$CARD_210A" "MIMAP-210A closeout card must be landed"
guard_expect_in_file "$TAG" "Status: selected current" "$CARD_211A" "MIMAP-211A must be selected current after closeout"

guard_expect_in_file "$TAG" "Decision: accepted" "$SSOT" "MIMAP-210A closeout SSOT must be accepted"
guard_expect_in_file "$TAG" "Decision: accepted" "$DIAGNOSTIC_SSOT" "MIMAP-208A diagnostic SSOT must stay accepted"
guard_expect_in_file "$TAG" "segment-map-local-free-reuse-ledger-release-applied-recycle-second-release-diagnostic" "$SSOT" "closeout SSOT must name pack"
guard_expect_in_file "$TAG" "--level L2" "$SSOT" "closeout SSOT must freeze L2 daily command"
guard_expect_in_file "$TAG" "representative exact-MIR L3 EXE evidence" "$SSOT" "closeout SSOT must require L3 evidence"
guard_expect_in_file "$TAG" "MIMAP-211A post-segment-map-local-free-reuse-ledger-release-applied-recycle-second-release-diagnostic-closeout row selection" "$SSOT" "closeout SSOT must name next row"

guard_expect_in_file "$TAG" "segment-map-local-free-reuse-ledger-release-applied-recycle-second-release-diagnostic" "$CADENCE" "cadence SSOT must name second-release diagnostic pack"
guard_expect_in_file "$TAG" "representative L3 EXE evidence in MIMAP-210A" "$CADENCE" "cadence SSOT must anchor MIMAP-210A L3 evidence"
guard_expect_in_file "$TAG" "MIMAP-208A" "$GRANULARITY" "granularity SSOT must describe MIMAP-208A"
guard_expect_in_file "$TAG" "MIMAP-210A" "$GRANULARITY" "granularity SSOT must describe MIMAP-210A"
guard_expect_in_file "$TAG" "MIMAP-211A" "$GRANULARITY" "granularity SSOT must describe MIMAP-211A"
guard_expect_in_file "$TAG" "MIMAP-210A segment-map local-free reuse ledger release-applied recycle second-release diagnostic closeout pack" "$JOINT" "joint order must name closeout row"
guard_expect_in_file "$TAG" "MIMAP-211A post-segment-map-local-free-reuse-ledger-release-applied-recycle-second-release-diagnostic-closeout row selection" "$JOINT" "joint order must name next row"
guard_expect_in_file "$TAG" "MIMAP-211A" "$TASKBOARD" "taskboard must name selected next row"

guard_expect_in_file "$TAG" "id = \"MIMAP-208A\"" "$PROOF_MANIFEST" "proof manifest must include MIMAP-208A"
guard_expect_in_file "$TAG" "closeout_pack = \"segment-map-local-free-reuse-ledger-release-applied-recycle-second-release-diagnostic\"" "$PROOF_MANIFEST" "proof manifest must assign second-release diagnostic pack"
guard_expect_in_file "$TAG" "exe = \"deferred-to-closeout\"" "$PROOF_MANIFEST" "MIMAP-208A EXE evidence must stay deferred to closeout"
guard_expect_in_file "$TAG" "id = \"hako-alloc-segment-map-local-free-reuse-ledger-release-applied-recycle-second-release-diagnostic-closeout\"" "$GUARD_MANIFEST" "guard manifest must include MIMAP-210A closeout row"
guard_expect_in_file "$TAG" "closeout_pack = \"segment-map-local-free-reuse-ledger-release-applied-recycle-second-release-diagnostic\"" "$GUARD_MANIFEST" "guard manifest must assign second-release diagnostic pack"
guard_expect_in_file "$TAG" "$GUARD_208A" "$INDEX" "check index must list MIMAP-208A guard"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list MIMAP-210A closeout guard"

bash "$RUN_PROOF" --closeout-pack segment-map-local-free-reuse-ledger-release-applied-recycle-second-release-diagnostic --level L2 --dry-run >/tmp/"$TAG".proof_dry_run 2>&1 || {
  cat /tmp/"$TAG".proof_dry_run >&2
  rm -f /tmp/"$TAG".proof_dry_run
  guard_fail "$TAG" "second-release diagnostic L2 dry-run selection failed"
}
guard_expect_in_file "$TAG" "MIMAP-208A" /tmp/"$TAG".proof_dry_run "L2 dry-run must include MIMAP-208A"
rm -f /tmp/"$TAG".proof_dry_run

bash "$GUARD_208A" --level L2

pure_first_guard_build_toolchain

tmp_dir="$(mktemp -d /tmp/hakorune_mimap210a_second_release_diagnostic_closeout.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT

mir_json="$tmp_dir/mimap210a.mir.json"
exe_out="$tmp_dir/mimap210a.exe"
build_log="$tmp_dir/build.log"
run_log="$tmp_dir/run.log"
vm_log="$tmp_dir/vm.log"

if ! pure_first_guard_run_vm "$TAG" "$ROOT_DIR" "$APP" "$vm_log"; then
  exit 1
fi

rg -F -q 'hako-alloc-segment-map-local-free-reuse-ledger-release-applied-recycle-second-release-diagnostic-proof' "$vm_log"
rg -F -q 'second_release=0,3,0,70007004' "$vm_log"
rg -F -q 'reuse_counts=2,2,0,2,1,1,1,0' "$vm_log"
rg -F -q 'release_counts=2,1,1,1,1' "$vm_log"
rg -F -q 'inactive=0,0,0,0,0,0,0,0,0,0,0' "$vm_log"
rg -F -q 'check=1' "$vm_log"
rg -F -q 'summary=ok' "$vm_log"

pure_first_guard_emit_mir "$ROOT_DIR" "$APP" "$mir_json"
pure_first_guard_build_exe "$TAG" "$ROOT_DIR" "$APP" "$mir_json" "$exe_out" "$build_log"
pure_first_guard_assert_clean_build_log "$TAG" "$build_log"
pure_first_guard_run_exe "$TAG" "$exe_out" "$run_log"

rg -F -q 'hako-alloc-segment-map-local-free-reuse-ledger-release-applied-recycle-second-release-diagnostic-proof' "$run_log"
rg -F -q 'second_release=0,3,0,70007004' "$run_log"
rg -F -q 'reuse_counts=2,2,0,2,1,1,1,0' "$run_log"
rg -F -q 'release_counts=2,1,1,1,1' "$run_log"
rg -F -q 'inactive=0,0,0,0,0,0,0,0,0,0,0' "$run_log"
rg -F -q 'check=1' "$run_log"
rg -F -q 'summary=ok' "$run_log"

echo "[$TAG] ok"
