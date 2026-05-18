#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../../.." && pwd)"
TAG="k2-wide-hako-alloc-segment-map-local-free-reuse-ledger-lifecycle-keyed-release-shadow-closeout"
cd "$ROOT_DIR"
source "$ROOT_DIR/tools/checks/lib/pure_first_exe_guard.sh"

SSOT="docs/development/current/main/design/hako-alloc-segment-map-local-free-reuse-ledger-lifecycle-keyed-release-shadow-closeout-ssot.md"
SHADOW_SSOT="docs/development/current/main/design/hako-alloc-segment-map-local-free-reuse-ledger-lifecycle-keyed-release-shadow-ssot.md"
CADENCE="docs/development/current/main/design/mimalloc-row-validation-cadence-ssot.md"
TASKBOARD="docs/development/current/main/phases/phase-293x/293x-mimalloc-port-taskboard.md"
GRANULARITY="docs/development/current/main/design/mimalloc-allocator-first-task-granularity-ssot.md"
JOINT="docs/development/current/main/design/mimalloc-hakorune-joint-task-order-ssot.md"
INDEX="docs/tools/check-scripts-index.md"
PROOF_MANIFEST="tools/checks/proof_apps.toml"
GUARD_MANIFEST="tools/checks/guard_rows.toml"
CARD_224A="docs/development/current/main/phases/phase-293x/293x-747-MIMAP-224A-SEGMENT-MAP-LOCAL-FREE-REUSE-LEDGER-LIFECYCLE-KEYED-RELEASE-SHADOW-PILOT.md"
CARD_225A="docs/development/current/main/phases/phase-293x/293x-748-MIMAP-225A-POST-SEGMENT-MAP-LOCAL-FREE-REUSE-LEDGER-LIFECYCLE-KEYED-RELEASE-SHADOW-ROW-SELECTION.md"
CARD_226A="docs/development/current/main/phases/phase-293x/293x-749-MIMAP-226A-SEGMENT-MAP-LOCAL-FREE-REUSE-LEDGER-LIFECYCLE-KEYED-RELEASE-SHADOW-CLOSEOUT-PACK.md"
CARD_227A="docs/development/current/main/phases/phase-293x/293x-750-MIMAP-227A-POST-SEGMENT-MAP-LOCAL-FREE-REUSE-LEDGER-LIFECYCLE-KEYED-RELEASE-SHADOW-CLOSEOUT-ROW-SELECTION.md"
APP="apps/hako-alloc-segment-map-local-free-reuse-ledger-lifecycle-keyed-release-shadow-proof/main.hako"
APP_README="apps/hako-alloc-segment-map-local-free-reuse-ledger-lifecycle-keyed-release-shadow-proof/README.md"
APP_TEST="apps/hako-alloc-segment-map-local-free-reuse-ledger-lifecycle-keyed-release-shadow-proof/test.sh"
GUARD_224A="tools/checks/k2_wide_hako_alloc_segment_map_local_free_reuse_ledger_lifecycle_keyed_release_shadow_guard.sh"
SELF_SCRIPT="tools/checks/k2_wide_hako_alloc_segment_map_local_free_reuse_ledger_lifecycle_keyed_release_shadow_closeout_guard.sh"
IMPL_SCRIPT="tools/checks/impl/k2_wide_hako_alloc_segment_map_local_free_reuse_ledger_lifecycle_keyed_release_shadow_closeout_guard.sh"
RUN_PROOF="tools/checks/run_proof_app.sh"

echo "[$TAG] checking MIMAP-226A lifecycle-keyed release shadow closeout"

guard_require_files \
  "$TAG" \
  "$SSOT" \
  "$SHADOW_SSOT" \
  "$CADENCE" \
  "$TASKBOARD" \
  "$GRANULARITY" \
  "$JOINT" \
  "$INDEX" \
  "$PROOF_MANIFEST" \
  "$GUARD_MANIFEST" \
  "$CARD_224A" \
  "$CARD_225A" \
  "$CARD_226A" \
  "$CARD_227A" \
  "$APP" \
  "$APP_README" \
  "$APP_TEST" \
  "$GUARD_224A" \
  "$SELF_SCRIPT" \
  "$IMPL_SCRIPT" \
  "$RUN_PROOF"

guard_require_exec_files "$TAG" "$APP_TEST" "$GUARD_224A" "$SELF_SCRIPT" "$IMPL_SCRIPT" "$RUN_PROOF"

guard_expect_in_file "$TAG" "Status: landed" "$CARD_224A" "MIMAP-224A must be landed before closeout"
guard_expect_in_file "$TAG" "Status: landed" "$CARD_225A" "MIMAP-225A must be landed before closeout"
guard_expect_in_file "$TAG" "Status: landed" "$CARD_226A" "MIMAP-226A closeout card must be landed"
guard_expect_in_file "$TAG" "Status: selected current" "$CARD_227A" "MIMAP-227A must be selected current after closeout"

guard_expect_in_file "$TAG" "Decision: accepted" "$SSOT" "MIMAP-226A closeout SSOT must be accepted"
guard_expect_in_file "$TAG" "Decision: accepted" "$SHADOW_SSOT" "MIMAP-224A shadow SSOT must stay accepted"
guard_expect_in_file "$TAG" "segment-map-local-free-reuse-ledger-lifecycle-keyed-release-shadow" "$SSOT" "closeout SSOT must name pack"
guard_expect_in_file "$TAG" "--level L2" "$SSOT" "closeout SSOT must freeze L2 daily command"
guard_expect_in_file "$TAG" "representative exact-MIR L3 EXE evidence" "$SSOT" "closeout SSOT must require L3 evidence"
guard_expect_in_file "$TAG" "MIMAP-227A post-segment-map-local-free-reuse-ledger-lifecycle-keyed-release-shadow-closeout row selection" "$SSOT" "closeout SSOT must name next row"

guard_expect_in_file "$TAG" "segment-map local-free reuse ledger lifecycle-keyed release shadow family" "$CADENCE" "cadence SSOT must name lifecycle-keyed release shadow family"
guard_expect_in_file "$TAG" "representative L3 EXE evidence in MIMAP-226A" "$CADENCE" "cadence SSOT must anchor MIMAP-226A L3 evidence"
guard_expect_in_file "$TAG" "MIMAP-224A" "$GRANULARITY" "granularity SSOT must describe MIMAP-224A"
guard_expect_in_file "$TAG" "MIMAP-226A" "$GRANULARITY" "granularity SSOT must describe MIMAP-226A"
guard_expect_in_file "$TAG" "MIMAP-227A" "$GRANULARITY" "granularity SSOT must describe MIMAP-227A"
guard_expect_in_file "$TAG" "MIMAP-226A segment-map local-free reuse ledger lifecycle-keyed release shadow closeout pack" "$JOINT" "joint order must name closeout row"
guard_expect_in_file "$TAG" "MIMAP-227A post-segment-map-local-free-reuse-ledger-lifecycle-keyed-release-shadow-closeout row selection" "$JOINT" "joint order must name next row"
guard_expect_in_file "$TAG" "MIMAP-227A" "$TASKBOARD" "taskboard must name selected next row"

guard_expect_in_file "$TAG" "id = \"MIMAP-224A\"" "$PROOF_MANIFEST" "proof manifest must include MIMAP-224A"
guard_expect_in_file "$TAG" "closeout_pack = \"segment-map-local-free-reuse-ledger-lifecycle-keyed-release-shadow\"" "$PROOF_MANIFEST" "proof manifest must assign lifecycle-keyed release shadow pack"
guard_expect_in_file "$TAG" "exe = \"deferred-to-closeout\"" "$PROOF_MANIFEST" "MIMAP-224A EXE evidence must stay deferred to closeout"
guard_expect_in_file "$TAG" "id = \"hako-alloc-segment-map-local-free-reuse-ledger-lifecycle-keyed-release-shadow-closeout\"" "$GUARD_MANIFEST" "guard manifest must include MIMAP-226A closeout row"
guard_expect_in_file "$TAG" "closeout_pack = \"segment-map-local-free-reuse-ledger-lifecycle-keyed-release-shadow\"" "$GUARD_MANIFEST" "guard manifest must assign lifecycle-keyed release shadow pack"
guard_expect_in_file "$TAG" "$GUARD_224A" "$INDEX" "check index must list MIMAP-224A guard"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list MIMAP-226A closeout guard"

bash "$RUN_PROOF" --closeout-pack segment-map-local-free-reuse-ledger-lifecycle-keyed-release-shadow --level L2 --dry-run >/tmp/"$TAG".proof_dry_run 2>&1 || {
  cat /tmp/"$TAG".proof_dry_run >&2
  rm -f /tmp/"$TAG".proof_dry_run
  guard_fail "$TAG" "lifecycle-keyed release shadow L2 dry-run selection failed"
}
guard_expect_in_file "$TAG" "MIMAP-224A" /tmp/"$TAG".proof_dry_run "L2 dry-run must include MIMAP-224A"
rm -f /tmp/"$TAG".proof_dry_run

bash "$GUARD_224A" --level L2

pure_first_guard_build_toolchain

tmp_dir="$(mktemp -d /tmp/hakorune_mimap226a_lifecycle_shadow_closeout.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT

mir_json="$tmp_dir/mimap226a.mir.json"
exe_out="$tmp_dir/mimap226a.exe"
build_log="$tmp_dir/build.log"
run_log="$tmp_dir/run.log"
vm_log="$tmp_dir/vm.log"

if ! pure_first_guard_run_vm "$TAG" "$ROOT_DIR" "$APP" "$vm_log"; then
  exit 1
fi

rg -F -q 'hako-alloc-segment-map-local-free-reuse-ledger-lifecycle-keyed-release-shadow-proof' "$vm_log"
rg -F -q 'base=70007004,70007004002,0' "$vm_log"
rg -F -q 'shadow=1,0,0,70007004,2,70007004002,1,0' "$vm_log"
rg -F -q 'rejects=0,1,0,2,0,3,0,4,0,5' "$vm_log"
rg -F -q 'counts=6,1,5,1,1,1,1,1' "$vm_log"
rg -F -q 'inactive=0,0,0,0,0,0,0,0,0,0,0' "$vm_log"
rg -F -q 'check=1' "$vm_log"
rg -F -q 'summary=ok' "$vm_log"

pure_first_guard_emit_mir "$ROOT_DIR" "$APP" "$mir_json"
pure_first_guard_build_exe "$TAG" "$ROOT_DIR" "$APP" "$mir_json" "$exe_out" "$build_log"
pure_first_guard_assert_clean_build_log "$TAG" "$build_log"
pure_first_guard_run_exe "$TAG" "$exe_out" "$run_log"

rg -F -q 'hako-alloc-segment-map-local-free-reuse-ledger-lifecycle-keyed-release-shadow-proof' "$run_log"
rg -F -q 'base=70007004,70007004002,0' "$run_log"
rg -F -q 'shadow=1,0,0,70007004,2,70007004002,1,0' "$run_log"
rg -F -q 'rejects=0,1,0,2,0,3,0,4,0,5' "$run_log"
rg -F -q 'counts=6,1,5,1,1,1,1,1' "$run_log"
rg -F -q 'inactive=0,0,0,0,0,0,0,0,0,0,0' "$run_log"
rg -F -q 'check=1' "$run_log"
rg -F -q 'summary=ok' "$run_log"

echo "[$TAG] ok"
