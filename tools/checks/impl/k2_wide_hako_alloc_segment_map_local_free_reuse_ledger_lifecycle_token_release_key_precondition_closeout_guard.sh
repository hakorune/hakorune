#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../../.." && pwd)"
TAG="k2-wide-hako-alloc-segment-map-local-free-reuse-ledger-lifecycle-token-release-key-precondition-closeout"
cd "$ROOT_DIR"
source "$ROOT_DIR/tools/checks/lib/pure_first_exe_guard.sh"

SSOT="docs/development/current/main/design/hako-alloc-segment-map-local-free-reuse-ledger-lifecycle-token-release-key-precondition-closeout-ssot.md"
PRECONDITION_SSOT="docs/development/current/main/design/hako-alloc-segment-map-local-free-reuse-ledger-lifecycle-token-release-key-precondition-ssot.md"
CADENCE="docs/development/current/main/design/mimalloc-row-validation-cadence-ssot.md"
TASKBOARD="docs/development/current/main/phases/phase-293x/293x-mimalloc-port-taskboard.md"
GRANULARITY="docs/development/current/main/design/mimalloc-allocator-first-task-granularity-ssot.md"
JOINT="docs/development/current/main/design/mimalloc-hakorune-joint-task-order-ssot.md"
INDEX="docs/tools/check-scripts-index.md"
PROOF_MANIFEST="tools/checks/proof_apps.toml"
GUARD_MANIFEST="tools/checks/guard_rows.toml"
CARD_220A="docs/development/current/main/phases/phase-293x/293x-743-MIMAP-220A-SEGMENT-MAP-LOCAL-FREE-REUSE-LEDGER-LIFECYCLE-TOKEN-RELEASE-KEY-PRECONDITION-OBSERVER.md"
CARD_221A="docs/development/current/main/phases/phase-293x/293x-744-MIMAP-221A-POST-SEGMENT-MAP-LOCAL-FREE-REUSE-LEDGER-LIFECYCLE-TOKEN-RELEASE-KEY-PRECONDITION-ROW-SELECTION.md"
CARD_222A="docs/development/current/main/phases/phase-293x/293x-745-MIMAP-222A-SEGMENT-MAP-LOCAL-FREE-REUSE-LEDGER-LIFECYCLE-TOKEN-RELEASE-KEY-PRECONDITION-CLOSEOUT-PACK.md"
CARD_223A="docs/development/current/main/phases/phase-293x/293x-746-MIMAP-223A-POST-SEGMENT-MAP-LOCAL-FREE-REUSE-LEDGER-LIFECYCLE-TOKEN-RELEASE-KEY-PRECONDITION-CLOSEOUT-ROW-SELECTION.md"
APP="apps/hako-alloc-segment-map-local-free-reuse-ledger-lifecycle-token-release-key-precondition-proof/main.hako"
APP_README="apps/hako-alloc-segment-map-local-free-reuse-ledger-lifecycle-token-release-key-precondition-proof/README.md"
APP_TEST="apps/hako-alloc-segment-map-local-free-reuse-ledger-lifecycle-token-release-key-precondition-proof/test.sh"
GUARD_220A="tools/checks/k2_wide_hako_alloc_segment_map_local_free_reuse_ledger_lifecycle_token_release_key_precondition_guard.sh"
SELF_SCRIPT="tools/checks/k2_wide_hako_alloc_segment_map_local_free_reuse_ledger_lifecycle_token_release_key_precondition_closeout_guard.sh"
IMPL_SCRIPT="tools/checks/impl/k2_wide_hako_alloc_segment_map_local_free_reuse_ledger_lifecycle_token_release_key_precondition_closeout_guard.sh"
RUN_PROOF="tools/checks/run_proof_app.sh"

echo "[$TAG] checking MIMAP-222A lifecycle-token release-key precondition closeout"

guard_require_files \
  "$TAG" \
  "$SSOT" \
  "$PRECONDITION_SSOT" \
  "$CADENCE" \
  "$TASKBOARD" \
  "$GRANULARITY" \
  "$JOINT" \
  "$INDEX" \
  "$PROOF_MANIFEST" \
  "$GUARD_MANIFEST" \
  "$CARD_220A" \
  "$CARD_221A" \
  "$CARD_222A" \
  "$CARD_223A" \
  "$APP" \
  "$APP_README" \
  "$APP_TEST" \
  "$GUARD_220A" \
  "$SELF_SCRIPT" \
  "$IMPL_SCRIPT" \
  "$RUN_PROOF"

guard_require_exec_files "$TAG" "$APP_TEST" "$GUARD_220A" "$SELF_SCRIPT" "$IMPL_SCRIPT" "$RUN_PROOF"

guard_expect_in_file "$TAG" "Status: landed" "$CARD_220A" "MIMAP-220A must be landed before closeout"
guard_expect_in_file "$TAG" "Status: landed" "$CARD_221A" "MIMAP-221A must be landed before closeout"
guard_expect_in_file "$TAG" "Status: landed" "$CARD_222A" "MIMAP-222A closeout card must be landed"
guard_expect_in_file "$TAG" "Status: selected current" "$CARD_223A" "MIMAP-223A must be selected current after closeout"

guard_expect_in_file "$TAG" "Decision: accepted" "$SSOT" "MIMAP-222A closeout SSOT must be accepted"
guard_expect_in_file "$TAG" "Decision: accepted" "$PRECONDITION_SSOT" "MIMAP-220A precondition SSOT must stay accepted"
guard_expect_in_file "$TAG" "segment-map-local-free-reuse-ledger-lifecycle-token-release-key-precondition" "$SSOT" "closeout SSOT must name pack"
guard_expect_in_file "$TAG" "--level L2" "$SSOT" "closeout SSOT must freeze L2 daily command"
guard_expect_in_file "$TAG" "representative exact-MIR L3 EXE evidence" "$SSOT" "closeout SSOT must require L3 evidence"
guard_expect_in_file "$TAG" "MIMAP-223A post-segment-map-local-free-reuse-ledger-lifecycle-token-release-key-precondition-closeout row selection" "$SSOT" "closeout SSOT must name next row"

guard_expect_in_file "$TAG" "segment-map local-free reuse ledger lifecycle-token release-key precondition family" "$CADENCE" "cadence SSOT must name release-key precondition family"
guard_expect_in_file "$TAG" "representative L3 EXE evidence in MIMAP-222A" "$CADENCE" "cadence SSOT must anchor MIMAP-222A L3 evidence"
guard_expect_in_file "$TAG" "MIMAP-220A" "$GRANULARITY" "granularity SSOT must describe MIMAP-220A"
guard_expect_in_file "$TAG" "MIMAP-222A" "$GRANULARITY" "granularity SSOT must describe MIMAP-222A"
guard_expect_in_file "$TAG" "MIMAP-223A" "$GRANULARITY" "granularity SSOT must describe MIMAP-223A"
guard_expect_in_file "$TAG" "MIMAP-222A segment-map local-free reuse ledger lifecycle-token release-key precondition closeout pack" "$JOINT" "joint order must name closeout row"
guard_expect_in_file "$TAG" "MIMAP-223A post-segment-map-local-free-reuse-ledger-lifecycle-token-release-key-precondition-closeout row selection" "$JOINT" "joint order must name next row"
guard_expect_in_file "$TAG" "MIMAP-223A" "$TASKBOARD" "taskboard must name selected next row"

guard_expect_in_file "$TAG" "id = \"MIMAP-220A\"" "$PROOF_MANIFEST" "proof manifest must include MIMAP-220A"
guard_expect_in_file "$TAG" "closeout_pack = \"segment-map-local-free-reuse-ledger-lifecycle-token-release-key-precondition\"" "$PROOF_MANIFEST" "proof manifest must assign release-key precondition pack"
guard_expect_in_file "$TAG" "exe = \"deferred-to-closeout\"" "$PROOF_MANIFEST" "MIMAP-220A EXE evidence must stay deferred to closeout"
guard_expect_in_file "$TAG" "id = \"hako-alloc-segment-map-local-free-reuse-ledger-lifecycle-token-release-key-precondition-closeout\"" "$GUARD_MANIFEST" "guard manifest must include MIMAP-222A closeout row"
guard_expect_in_file "$TAG" "closeout_pack = \"segment-map-local-free-reuse-ledger-lifecycle-token-release-key-precondition\"" "$GUARD_MANIFEST" "guard manifest must assign release-key precondition pack"
guard_expect_in_file "$TAG" "$GUARD_220A" "$INDEX" "check index must list MIMAP-220A guard"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list MIMAP-222A closeout guard"

bash "$RUN_PROOF" --closeout-pack segment-map-local-free-reuse-ledger-lifecycle-token-release-key-precondition --level L2 --dry-run >/tmp/"$TAG".proof_dry_run 2>&1 || {
  cat /tmp/"$TAG".proof_dry_run >&2
  rm -f /tmp/"$TAG".proof_dry_run
  guard_fail "$TAG" "release-key precondition L2 dry-run selection failed"
}
guard_expect_in_file "$TAG" "MIMAP-220A" /tmp/"$TAG".proof_dry_run "L2 dry-run must include MIMAP-220A"
rm -f /tmp/"$TAG".proof_dry_run

bash "$GUARD_220A" --level L2

pure_first_guard_build_toolchain

tmp_dir="$(mktemp -d /tmp/hakorune_mimap222a_release_key_precondition_closeout.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT

mir_json="$tmp_dir/mimap222a.mir.json"
exe_out="$tmp_dir/mimap222a.exe"
build_log="$tmp_dir/build.log"
run_log="$tmp_dir/run.log"
vm_log="$tmp_dir/vm.log"

if ! pure_first_guard_run_vm "$TAG" "$ROOT_DIR" "$APP" "$vm_log"; then
  exit 1
fi

rg -F -q 'hako-alloc-segment-map-local-free-reuse-ledger-lifecycle-token-release-key-precondition-proof' "$vm_log"
rg -F -q 'base=70007004,2,3' "$vm_log"
rg -F -q 'ready=1,0,70007004,2,1,1,1,0' "$vm_log"
rg -F -q 'blocked=0,1,0,2,0,3,0,4' "$vm_log"
rg -F -q 'counts=5,1,4,1,1,1,1' "$vm_log"
rg -F -q 'inactive=0,0,0,0,0,0,0,0,0,0,0' "$vm_log"
rg -F -q 'check=1' "$vm_log"
rg -F -q 'summary=ok' "$vm_log"

pure_first_guard_emit_mir "$ROOT_DIR" "$APP" "$mir_json"
pure_first_guard_build_exe "$TAG" "$ROOT_DIR" "$APP" "$mir_json" "$exe_out" "$build_log"
pure_first_guard_assert_clean_build_log "$TAG" "$build_log"
pure_first_guard_run_exe "$TAG" "$exe_out" "$run_log"

rg -F -q 'hako-alloc-segment-map-local-free-reuse-ledger-lifecycle-token-release-key-precondition-proof' "$run_log"
rg -F -q 'base=70007004,2,3' "$run_log"
rg -F -q 'ready=1,0,70007004,2,1,1,1,0' "$run_log"
rg -F -q 'blocked=0,1,0,2,0,3,0,4' "$run_log"
rg -F -q 'counts=5,1,4,1,1,1,1' "$run_log"
rg -F -q 'inactive=0,0,0,0,0,0,0,0,0,0,0' "$run_log"
rg -F -q 'check=1' "$run_log"
rg -F -q 'summary=ok' "$run_log"

echo "[$TAG] ok"
