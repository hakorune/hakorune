#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../../.." && pwd)"
TAG="k2-wide-hako-alloc-segment-map-local-free-reuse-ledger-lifecycle-token-observer-diagnostic-closeout"
cd "$ROOT_DIR"
source "$ROOT_DIR/tools/checks/lib/pure_first_exe_guard.sh"

SSOT="docs/development/current/main/design/hako-alloc-segment-map-local-free-reuse-ledger-lifecycle-token-observer-diagnostic-closeout-ssot.md"
DIAGNOSTIC_SSOT="docs/development/current/main/design/hako-alloc-segment-map-local-free-reuse-ledger-lifecycle-token-observer-diagnostic-ssot.md"
CADENCE="docs/development/current/main/design/mimalloc-row-validation-cadence-ssot.md"
TASKBOARD="docs/development/current/main/phases/phase-293x/293x-mimalloc-port-taskboard.md"
GRANULARITY="docs/development/current/main/design/mimalloc-allocator-first-task-granularity-ssot.md"
JOINT="docs/development/current/main/design/mimalloc-hakorune-joint-task-order-ssot.md"
INDEX="docs/tools/check-scripts-index.md"
PROOF_MANIFEST="tools/checks/proof_apps.toml"
GUARD_MANIFEST="tools/checks/guard_rows.toml"
CARD_216A="docs/development/current/main/phases/phase-293x/293x-739-MIMAP-216A-SEGMENT-MAP-LOCAL-FREE-REUSE-LEDGER-LIFECYCLE-TOKEN-OBSERVER-DIAGNOSTIC.md"
CARD_217A="docs/development/current/main/phases/phase-293x/293x-740-MIMAP-217A-POST-SEGMENT-MAP-LOCAL-FREE-REUSE-LEDGER-LIFECYCLE-TOKEN-OBSERVER-DIAGNOSTIC-ROW-SELECTION.md"
CARD_218A="docs/development/current/main/phases/phase-293x/293x-741-MIMAP-218A-SEGMENT-MAP-LOCAL-FREE-REUSE-LEDGER-LIFECYCLE-TOKEN-OBSERVER-DIAGNOSTIC-CLOSEOUT-PACK.md"
CARD_219A="docs/development/current/main/phases/phase-293x/293x-742-MIMAP-219A-POST-SEGMENT-MAP-LOCAL-FREE-REUSE-LEDGER-LIFECYCLE-TOKEN-OBSERVER-DIAGNOSTIC-CLOSEOUT-ROW-SELECTION.md"
APP="apps/hako-alloc-segment-map-local-free-reuse-ledger-lifecycle-token-observer-diagnostic-proof/main.hako"
APP_README="apps/hako-alloc-segment-map-local-free-reuse-ledger-lifecycle-token-observer-diagnostic-proof/README.md"
APP_TEST="apps/hako-alloc-segment-map-local-free-reuse-ledger-lifecycle-token-observer-diagnostic-proof/test.sh"
GUARD_216A="tools/checks/k2_wide_hako_alloc_segment_map_local_free_reuse_ledger_lifecycle_token_observer_diagnostic_guard.sh"
SELF_SCRIPT="tools/checks/k2_wide_hako_alloc_segment_map_local_free_reuse_ledger_lifecycle_token_observer_diagnostic_closeout_guard.sh"
IMPL_SCRIPT="tools/checks/impl/k2_wide_hako_alloc_segment_map_local_free_reuse_ledger_lifecycle_token_observer_diagnostic_closeout_guard.sh"
RUN_PROOF="tools/checks/run_proof_app.sh"

echo "[$TAG] checking MIMAP-218A lifecycle-token observer diagnostic closeout"

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
  "$CARD_216A" \
  "$CARD_217A" \
  "$CARD_218A" \
  "$CARD_219A" \
  "$APP" \
  "$APP_README" \
  "$APP_TEST" \
  "$GUARD_216A" \
  "$SELF_SCRIPT" \
  "$IMPL_SCRIPT" \
  "$RUN_PROOF"

guard_require_exec_files "$TAG" "$APP_TEST" "$GUARD_216A" "$SELF_SCRIPT" "$IMPL_SCRIPT" "$RUN_PROOF"

guard_expect_in_file "$TAG" "Status: landed" "$CARD_216A" "MIMAP-216A must be landed before closeout"
guard_expect_in_file "$TAG" "Status: landed" "$CARD_217A" "MIMAP-217A must be landed before closeout"
guard_expect_in_file "$TAG" "Status: landed" "$CARD_218A" "MIMAP-218A closeout card must be landed"
guard_expect_in_file "$TAG" "Status: selected current" "$CARD_219A" "MIMAP-219A must be selected current after closeout"

guard_expect_in_file "$TAG" "Decision: accepted" "$SSOT" "MIMAP-218A closeout SSOT must be accepted"
guard_expect_in_file "$TAG" "Decision: accepted" "$DIAGNOSTIC_SSOT" "MIMAP-216A diagnostic SSOT must stay accepted"
guard_expect_in_file "$TAG" "segment-map-local-free-reuse-ledger-lifecycle-token-observer-diagnostic" "$SSOT" "closeout SSOT must name pack"
guard_expect_in_file "$TAG" "--level L2" "$SSOT" "closeout SSOT must freeze L2 daily command"
guard_expect_in_file "$TAG" "representative exact-MIR L3 EXE evidence" "$SSOT" "closeout SSOT must require L3 evidence"
guard_expect_in_file "$TAG" "MIMAP-219A post-segment-map-local-free-reuse-ledger-lifecycle-token-observer-diagnostic-closeout row selection" "$SSOT" "closeout SSOT must name next row"

guard_expect_in_file "$TAG" "segment-map local-free reuse ledger lifecycle-token observer diagnostic family" "$CADENCE" "cadence SSOT must name observer diagnostic family"
guard_expect_in_file "$TAG" "representative L3 EXE evidence in MIMAP-218A" "$CADENCE" "cadence SSOT must anchor MIMAP-218A L3 evidence"
guard_expect_in_file "$TAG" "MIMAP-216A" "$GRANULARITY" "granularity SSOT must describe MIMAP-216A"
guard_expect_in_file "$TAG" "MIMAP-218A" "$GRANULARITY" "granularity SSOT must describe MIMAP-218A"
guard_expect_in_file "$TAG" "MIMAP-219A" "$GRANULARITY" "granularity SSOT must describe MIMAP-219A"
guard_expect_in_file "$TAG" "MIMAP-218A segment-map local-free reuse ledger lifecycle-token observer diagnostic closeout pack" "$JOINT" "joint order must name closeout row"
guard_expect_in_file "$TAG" "MIMAP-219A post-segment-map-local-free-reuse-ledger-lifecycle-token-observer-diagnostic-closeout row selection" "$JOINT" "joint order must name next row"
guard_expect_in_file "$TAG" "MIMAP-219A" "$TASKBOARD" "taskboard must name selected next row"

guard_expect_in_file "$TAG" "id = \"MIMAP-216A\"" "$PROOF_MANIFEST" "proof manifest must include MIMAP-216A"
guard_expect_in_file "$TAG" "closeout_pack = \"segment-map-local-free-reuse-ledger-lifecycle-token-observer-diagnostic\"" "$PROOF_MANIFEST" "proof manifest must assign observer diagnostic pack"
guard_expect_in_file "$TAG" "exe = \"deferred-to-closeout\"" "$PROOF_MANIFEST" "MIMAP-216A EXE evidence must stay deferred to closeout"
guard_expect_in_file "$TAG" "id = \"hako-alloc-segment-map-local-free-reuse-ledger-lifecycle-token-observer-diagnostic-closeout\"" "$GUARD_MANIFEST" "guard manifest must include MIMAP-218A closeout row"
guard_expect_in_file "$TAG" "closeout_pack = \"segment-map-local-free-reuse-ledger-lifecycle-token-observer-diagnostic\"" "$GUARD_MANIFEST" "guard manifest must assign observer diagnostic pack"
guard_expect_in_file "$TAG" "$GUARD_216A" "$INDEX" "check index must list MIMAP-216A guard"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list MIMAP-218A closeout guard"

bash "$RUN_PROOF" --closeout-pack segment-map-local-free-reuse-ledger-lifecycle-token-observer-diagnostic --level L2 --dry-run >/tmp/"$TAG".proof_dry_run 2>&1 || {
  cat /tmp/"$TAG".proof_dry_run >&2
  rm -f /tmp/"$TAG".proof_dry_run
  guard_fail "$TAG" "observer diagnostic L2 dry-run selection failed"
}
guard_expect_in_file "$TAG" "MIMAP-216A" /tmp/"$TAG".proof_dry_run "L2 dry-run must include MIMAP-216A"
rm -f /tmp/"$TAG".proof_dry_run

bash "$GUARD_216A" --level L2

pure_first_guard_build_toolchain

tmp_dir="$(mktemp -d /tmp/hakorune_mimap218a_lifecycle_observer_closeout.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT

mir_json="$tmp_dir/mimap218a.mir.json"
exe_out="$tmp_dir/mimap218a.exe"
build_log="$tmp_dir/build.log"
run_log="$tmp_dir/run.log"
vm_log="$tmp_dir/vm.log"

if ! pure_first_guard_run_vm "$TAG" "$ROOT_DIR" "$APP" "$vm_log"; then
  exit 1
fi

rg -F -q 'hako-alloc-segment-map-local-free-reuse-ledger-lifecycle-token-observer-diagnostic-proof' "$vm_log"
rg -F -q 'base=70007004,2,3' "$vm_log"
rg -F -q 'observer=1,0,70007004,2,2,3,1,1,0' "$vm_log"
rg -F -q 'rejects=0,1,0,2' "$vm_log"
rg -F -q 'counts=3,1,2,1,1' "$vm_log"
rg -F -q 'inactive=0,0,0,0,0,0,0,0,0,0,0' "$vm_log"
rg -F -q 'check=1' "$vm_log"
rg -F -q 'summary=ok' "$vm_log"

pure_first_guard_emit_mir "$ROOT_DIR" "$APP" "$mir_json"
pure_first_guard_build_exe "$TAG" "$ROOT_DIR" "$APP" "$mir_json" "$exe_out" "$build_log"
pure_first_guard_assert_clean_build_log "$TAG" "$build_log"
pure_first_guard_run_exe "$TAG" "$exe_out" "$run_log"

rg -F -q 'hako-alloc-segment-map-local-free-reuse-ledger-lifecycle-token-observer-diagnostic-proof' "$run_log"
rg -F -q 'base=70007004,2,3' "$run_log"
rg -F -q 'observer=1,0,70007004,2,2,3,1,1,0' "$run_log"
rg -F -q 'rejects=0,1,0,2' "$run_log"
rg -F -q 'counts=3,1,2,1,1' "$run_log"
rg -F -q 'inactive=0,0,0,0,0,0,0,0,0,0,0' "$run_log"
rg -F -q 'check=1' "$run_log"
rg -F -q 'summary=ok' "$run_log"

echo "[$TAG] ok"
