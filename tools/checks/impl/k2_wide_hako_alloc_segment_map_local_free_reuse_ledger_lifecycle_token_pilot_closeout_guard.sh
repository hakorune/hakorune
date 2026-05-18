#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../../.." && pwd)"
TAG="k2-wide-hako-alloc-segment-map-local-free-reuse-ledger-lifecycle-token-pilot-closeout"
cd "$ROOT_DIR"
source "$ROOT_DIR/tools/checks/lib/pure_first_exe_guard.sh"

SSOT="docs/development/current/main/design/hako-alloc-segment-map-local-free-reuse-ledger-lifecycle-token-pilot-closeout-ssot.md"
PILOT_SSOT="docs/development/current/main/design/hako-alloc-segment-map-local-free-reuse-ledger-lifecycle-token-pilot-ssot.md"
CADENCE="docs/development/current/main/design/mimalloc-row-validation-cadence-ssot.md"
TASKBOARD="docs/development/current/main/phases/phase-293x/293x-mimalloc-port-taskboard.md"
GRANULARITY="docs/development/current/main/design/mimalloc-allocator-first-task-granularity-ssot.md"
JOINT="docs/development/current/main/design/mimalloc-hakorune-joint-task-order-ssot.md"
INDEX="docs/tools/check-scripts-index.md"
PROOF_MANIFEST="tools/checks/proof_apps.toml"
GUARD_MANIFEST="tools/checks/guard_rows.toml"
CARD_212A="docs/development/current/main/phases/phase-293x/293x-735-MIMAP-212A-SEGMENT-MAP-LOCAL-FREE-REUSE-LEDGER-LIFECYCLE-TOKEN-PILOT.md"
CARD_213A="docs/development/current/main/phases/phase-293x/293x-736-MIMAP-213A-POST-SEGMENT-MAP-LOCAL-FREE-REUSE-LEDGER-LIFECYCLE-TOKEN-PILOT-ROW-SELECTION.md"
CARD_214A="docs/development/current/main/phases/phase-293x/293x-737-MIMAP-214A-SEGMENT-MAP-LOCAL-FREE-REUSE-LEDGER-LIFECYCLE-TOKEN-PILOT-CLOSEOUT-PACK.md"
CARD_215A="docs/development/current/main/phases/phase-293x/293x-738-MIMAP-215A-POST-SEGMENT-MAP-LOCAL-FREE-REUSE-LEDGER-LIFECYCLE-TOKEN-PILOT-CLOSEOUT-ROW-SELECTION.md"
APP="apps/hako-alloc-segment-map-local-free-reuse-ledger-lifecycle-token-pilot-proof/main.hako"
APP_README="apps/hako-alloc-segment-map-local-free-reuse-ledger-lifecycle-token-pilot-proof/README.md"
APP_TEST="apps/hako-alloc-segment-map-local-free-reuse-ledger-lifecycle-token-pilot-proof/test.sh"
GUARD_212A="tools/checks/k2_wide_hako_alloc_segment_map_local_free_reuse_ledger_lifecycle_token_pilot_guard.sh"
SELF_SCRIPT="tools/checks/k2_wide_hako_alloc_segment_map_local_free_reuse_ledger_lifecycle_token_pilot_closeout_guard.sh"
IMPL_SCRIPT="tools/checks/impl/k2_wide_hako_alloc_segment_map_local_free_reuse_ledger_lifecycle_token_pilot_closeout_guard.sh"
RUN_PROOF="tools/checks/run_proof_app.sh"

echo "[$TAG] checking MIMAP-214A lifecycle-token pilot closeout"

guard_require_files \
  "$TAG" \
  "$SSOT" \
  "$PILOT_SSOT" \
  "$CADENCE" \
  "$TASKBOARD" \
  "$GRANULARITY" \
  "$JOINT" \
  "$INDEX" \
  "$PROOF_MANIFEST" \
  "$GUARD_MANIFEST" \
  "$CARD_212A" \
  "$CARD_213A" \
  "$CARD_214A" \
  "$CARD_215A" \
  "$APP" \
  "$APP_README" \
  "$APP_TEST" \
  "$GUARD_212A" \
  "$SELF_SCRIPT" \
  "$IMPL_SCRIPT" \
  "$RUN_PROOF"

guard_require_exec_files "$TAG" "$APP_TEST" "$GUARD_212A" "$SELF_SCRIPT" "$IMPL_SCRIPT" "$RUN_PROOF"

guard_expect_in_file "$TAG" "Status: landed" "$CARD_212A" "MIMAP-212A must be landed before closeout"
guard_expect_in_file "$TAG" "Status: landed" "$CARD_213A" "MIMAP-213A must be landed before closeout"
guard_expect_in_file "$TAG" "Status: landed" "$CARD_214A" "MIMAP-214A closeout card must be landed"
guard_expect_in_file "$TAG" "Status: selected current" "$CARD_215A" "MIMAP-215A must be selected current after closeout"

guard_expect_in_file "$TAG" "Decision: accepted" "$SSOT" "MIMAP-214A closeout SSOT must be accepted"
guard_expect_in_file "$TAG" "Decision: accepted" "$PILOT_SSOT" "MIMAP-212A pilot SSOT must stay accepted"
guard_expect_in_file "$TAG" "segment-map-local-free-reuse-ledger-lifecycle-token-pilot" "$SSOT" "closeout SSOT must name pack"
guard_expect_in_file "$TAG" "--level L2" "$SSOT" "closeout SSOT must freeze L2 daily command"
guard_expect_in_file "$TAG" "representative exact-MIR L3 EXE evidence" "$SSOT" "closeout SSOT must require L3 evidence"
guard_expect_in_file "$TAG" "MIMAP-215A post-segment-map-local-free-reuse-ledger-lifecycle-token-pilot-closeout row selection" "$SSOT" "closeout SSOT must name next row"

guard_expect_in_file "$TAG" "segment-map local-free reuse ledger lifecycle-token pilot family" "$CADENCE" "cadence SSOT must name lifecycle-token pilot family"
guard_expect_in_file "$TAG" "representative L3 EXE evidence in MIMAP-214A" "$CADENCE" "cadence SSOT must anchor MIMAP-214A L3 evidence"
guard_expect_in_file "$TAG" "MIMAP-212A" "$GRANULARITY" "granularity SSOT must describe MIMAP-212A"
guard_expect_in_file "$TAG" "MIMAP-214A" "$GRANULARITY" "granularity SSOT must describe MIMAP-214A"
guard_expect_in_file "$TAG" "MIMAP-215A" "$GRANULARITY" "granularity SSOT must describe MIMAP-215A"
guard_expect_in_file "$TAG" "MIMAP-214A segment-map local-free reuse ledger lifecycle-token pilot closeout pack" "$JOINT" "joint order must name closeout row"
guard_expect_in_file "$TAG" "MIMAP-215A post-segment-map-local-free-reuse-ledger-lifecycle-token-pilot-closeout row selection" "$JOINT" "joint order must name next row"
guard_expect_in_file "$TAG" "MIMAP-215A" "$TASKBOARD" "taskboard must name selected next row"

guard_expect_in_file "$TAG" "id = \"MIMAP-212A\"" "$PROOF_MANIFEST" "proof manifest must include MIMAP-212A"
guard_expect_in_file "$TAG" "closeout_pack = \"segment-map-local-free-reuse-ledger-lifecycle-token-pilot\"" "$PROOF_MANIFEST" "proof manifest must assign lifecycle-token pilot pack"
guard_expect_in_file "$TAG" "exe = \"deferred-to-closeout\"" "$PROOF_MANIFEST" "MIMAP-212A EXE evidence must stay deferred to closeout"
guard_expect_in_file "$TAG" "id = \"hako-alloc-segment-map-local-free-reuse-ledger-lifecycle-token-pilot-closeout\"" "$GUARD_MANIFEST" "guard manifest must include MIMAP-214A closeout row"
guard_expect_in_file "$TAG" "closeout_pack = \"segment-map-local-free-reuse-ledger-lifecycle-token-pilot\"" "$GUARD_MANIFEST" "guard manifest must assign lifecycle-token pilot pack"
guard_expect_in_file "$TAG" "$GUARD_212A" "$INDEX" "check index must list MIMAP-212A guard"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list MIMAP-214A closeout guard"

bash "$RUN_PROOF" --closeout-pack segment-map-local-free-reuse-ledger-lifecycle-token-pilot --level L2 --dry-run >/tmp/"$TAG".proof_dry_run 2>&1 || {
  cat /tmp/"$TAG".proof_dry_run >&2
  rm -f /tmp/"$TAG".proof_dry_run
  guard_fail "$TAG" "lifecycle-token pilot L2 dry-run selection failed"
}
guard_expect_in_file "$TAG" "MIMAP-212A" /tmp/"$TAG".proof_dry_run "L2 dry-run must include MIMAP-212A"
rm -f /tmp/"$TAG".proof_dry_run

bash "$GUARD_212A" --level L2

pure_first_guard_build_toolchain

tmp_dir="$(mktemp -d /tmp/hakorune_mimap214a_lifecycle_token_closeout.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT

mir_json="$tmp_dir/mimap214a.mir.json"
exe_out="$tmp_dir/mimap214a.exe"
build_log="$tmp_dir/build.log"
run_log="$tmp_dir/run.log"
vm_log="$tmp_dir/vm.log"

if ! pure_first_guard_run_vm "$TAG" "$ROOT_DIR" "$APP" "$vm_log"; then
  exit 1
fi

rg -F -q 'hako-alloc-segment-map-local-free-reuse-ledger-lifecycle-token-pilot-proof' "$vm_log"
rg -F -q 'base=70007004,1,0' "$vm_log"
rg -F -q 'lifecycle=1,0,0,70007004,1,70007004001' "$vm_log"
rg -F -q 'second=1,0,1,70007004,2,70007004002' "$vm_log"
rg -F -q 'duplicate=0,2,1,70007004002' "$vm_log"
rg -F -q 'counts=5,2,3,1,1,1,2' "$vm_log"
rg -F -q 'inactive=0,0,0,0,0,0,0,0,0,0,0' "$vm_log"
rg -F -q 'check=1' "$vm_log"
rg -F -q 'summary=ok' "$vm_log"

pure_first_guard_emit_mir "$ROOT_DIR" "$APP" "$mir_json"
pure_first_guard_build_exe "$TAG" "$ROOT_DIR" "$APP" "$mir_json" "$exe_out" "$build_log"
pure_first_guard_assert_clean_build_log "$TAG" "$build_log"
pure_first_guard_run_exe "$TAG" "$exe_out" "$run_log"

rg -F -q 'hako-alloc-segment-map-local-free-reuse-ledger-lifecycle-token-pilot-proof' "$run_log"
rg -F -q 'base=70007004,1,0' "$run_log"
rg -F -q 'lifecycle=1,0,0,70007004,1,70007004001' "$run_log"
rg -F -q 'second=1,0,1,70007004,2,70007004002' "$run_log"
rg -F -q 'duplicate=0,2,1,70007004002' "$run_log"
rg -F -q 'counts=5,2,3,1,1,1,2' "$run_log"
rg -F -q 'inactive=0,0,0,0,0,0,0,0,0,0,0' "$run_log"
rg -F -q 'check=1' "$run_log"
rg -F -q 'summary=ok' "$run_log"

echo "[$TAG] ok"
