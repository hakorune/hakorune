#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../../.." && pwd)"
TAG="k2-wide-hako-alloc-segment-map-local-free-reuse-ledger-bridge-closeout"
cd "$ROOT_DIR"
source "$ROOT_DIR/tools/checks/lib/pure_first_exe_guard.sh"

SSOT="docs/development/current/main/design/hako-alloc-segment-map-local-free-reuse-ledger-bridge-closeout-ssot.md"
BRIDGE_SSOT="docs/development/current/main/design/hako-alloc-segment-map-local-free-reuse-ledger-bridge-ssot.md"
CADENCE="docs/development/current/main/design/mimalloc-row-validation-cadence-ssot.md"
TASKBOARD="docs/development/current/main/phases/phase-293x/293x-mimalloc-port-taskboard.md"
GRANULARITY="docs/development/current/main/design/mimalloc-allocator-first-task-granularity-ssot.md"
JOINT="docs/development/current/main/design/mimalloc-hakorune-joint-task-order-ssot.md"
INDEX="docs/tools/check-scripts-index.md"
PROOF_MANIFEST="tools/checks/proof_apps.toml"
GUARD_MANIFEST="tools/checks/guard_rows.toml"
CARD_192A="docs/development/current/main/phases/phase-293x/293x-714-MIMAP-192A-SEGMENT-MAP-LOCAL-FREE-REUSE-LEDGER-BRIDGE.md"
CARD_193A="docs/development/current/main/phases/phase-293x/293x-715-MIMAP-193A-POST-SEGMENT-MAP-LOCAL-FREE-REUSE-LEDGER-BRIDGE-ROW-SELECTION.md"
CARD_194A="docs/development/current/main/phases/phase-293x/293x-716-MIMAP-194A-SEGMENT-MAP-LOCAL-FREE-REUSE-LEDGER-BRIDGE-CLOSEOUT-PACK.md"
CARD_195A="docs/development/current/main/phases/phase-293x/293x-717-MIMAP-195A-POST-SEGMENT-MAP-LOCAL-FREE-REUSE-LEDGER-BRIDGE-CLOSEOUT-ROW-SELECTION.md"
APP="apps/hako-alloc-segment-map-local-free-reuse-ledger-bridge-proof/main.hako"
APP_README="apps/hako-alloc-segment-map-local-free-reuse-ledger-bridge-proof/README.md"
APP_TEST="apps/hako-alloc-segment-map-local-free-reuse-ledger-bridge-proof/test.sh"
OWNER="lang/src/hako_alloc/memory/segment_map_accepted_readiness_modeled_consume_ledger_box.hako"
SPAN_OWNER="lang/src/hako_alloc/memory/segment_allocation_modeled_released_span_ledger_box.hako"
REUSE_OWNER="lang/src/hako_alloc/memory/segment_allocation_modeled_local_free_reuse_box.hako"
LEDGER_OWNER="lang/src/hako_alloc/memory/segment_allocation_modeled_local_free_reuse_ledger_box.hako"
GUARD_192A="tools/checks/k2_wide_hako_alloc_segment_map_local_free_reuse_ledger_bridge_guard.sh"
SELF_SCRIPT="tools/checks/k2_wide_hako_alloc_segment_map_local_free_reuse_ledger_bridge_closeout_guard.sh"
IMPL_SCRIPT="tools/checks/impl/k2_wide_hako_alloc_segment_map_local_free_reuse_ledger_bridge_closeout_guard.sh"
RUN_PROOF="tools/checks/run_proof_app.sh"

echo "[$TAG] checking MIMAP-194A segment-map local-free reuse ledger bridge closeout"

guard_require_files \
  "$TAG" \
  "$SSOT" \
  "$BRIDGE_SSOT" \
  "$CADENCE" \
  "$TASKBOARD" \
  "$GRANULARITY" \
  "$JOINT" \
  "$INDEX" \
  "$PROOF_MANIFEST" \
  "$GUARD_MANIFEST" \
  "$CARD_192A" \
  "$CARD_193A" \
  "$CARD_194A" \
  "$CARD_195A" \
  "$APP" \
  "$APP_README" \
  "$APP_TEST" \
  "$OWNER" \
  "$SPAN_OWNER" \
  "$REUSE_OWNER" \
  "$LEDGER_OWNER" \
  "$GUARD_192A" \
  "$SELF_SCRIPT" \
  "$IMPL_SCRIPT" \
  "$RUN_PROOF"

guard_require_exec_files "$TAG" "$APP_TEST" "$GUARD_192A" "$SELF_SCRIPT" "$IMPL_SCRIPT" "$RUN_PROOF"

guard_expect_in_file "$TAG" "Status: landed" "$CARD_192A" "MIMAP-192A must be landed before closeout"
guard_expect_in_file "$TAG" "Status: landed" "$CARD_193A" "MIMAP-193A must be landed before closeout"
guard_expect_in_file "$TAG" "Status: landed" "$CARD_194A" "MIMAP-194A closeout card must be landed"
guard_expect_in_file "$TAG" "Status: selected current" "$CARD_195A" "MIMAP-195A must be selected current after closeout"

guard_expect_in_file "$TAG" "Decision: accepted" "$SSOT" "MIMAP-194A closeout SSOT must be accepted"
guard_expect_in_file "$TAG" "Decision: accepted" "$BRIDGE_SSOT" "MIMAP-192A bridge SSOT must stay accepted"
guard_expect_in_file "$TAG" "segment-map-local-free-reuse-ledger-bridge" "$SSOT" "closeout SSOT must name pack"
guard_expect_in_file "$TAG" "--level L2" "$SSOT" "closeout SSOT must freeze L2 daily command"
guard_expect_in_file "$TAG" "representative L3 EXE evidence" "$SSOT" "closeout SSOT must require L3 evidence"
guard_expect_in_file "$TAG" "MIMAP-195A post-segment-map-local-free-reuse-ledger-bridge-closeout row selection" "$SSOT" "closeout SSOT must name next row"

guard_expect_in_file "$TAG" "segment-map-local-free-reuse-ledger-bridge" "$CADENCE" "cadence SSOT must name reuse ledger bridge pack"
guard_expect_in_file "$TAG" "representative L3 EXE evidence in MIMAP-194A" "$CADENCE" "cadence SSOT must anchor MIMAP-194A L3 evidence"
guard_expect_in_file "$TAG" "MIMAP-192A" "$GRANULARITY" "granularity SSOT must describe MIMAP-192A"
guard_expect_in_file "$TAG" "MIMAP-194A" "$GRANULARITY" "granularity SSOT must describe MIMAP-194A"
guard_expect_in_file "$TAG" "MIMAP-195A" "$GRANULARITY" "granularity SSOT must describe MIMAP-195A"
guard_expect_in_file "$TAG" "MIMAP-194A segment-map local-free reuse ledger bridge closeout pack" "$JOINT" "joint order must name closeout row"
guard_expect_in_file "$TAG" "MIMAP-195A post-segment-map-local-free-reuse-ledger-bridge-closeout row selection" "$JOINT" "joint order must name next row"
guard_expect_in_file "$TAG" "MIMAP-195A" "$TASKBOARD" "taskboard must name selected next row"

guard_expect_in_file "$TAG" "id = \"MIMAP-192A\"" "$PROOF_MANIFEST" "proof manifest must include MIMAP-192A"
guard_expect_in_file "$TAG" "closeout_pack = \"segment-map-local-free-reuse-ledger-bridge\"" "$PROOF_MANIFEST" "proof manifest must assign reuse ledger bridge pack"
guard_expect_in_file "$TAG" "exe = \"deferred-to-closeout\"" "$PROOF_MANIFEST" "MIMAP-192A EXE evidence must stay deferred to closeout"
guard_expect_in_file "$TAG" "id = \"hako-alloc-segment-map-local-free-reuse-ledger-bridge-closeout\"" "$GUARD_MANIFEST" "guard manifest must include MIMAP-194A closeout row"
guard_expect_in_file "$TAG" "closeout_pack = \"segment-map-local-free-reuse-ledger-bridge\"" "$GUARD_MANIFEST" "guard manifest must assign reuse ledger bridge pack"
guard_expect_in_file "$TAG" "$GUARD_192A" "$INDEX" "check index must list MIMAP-192A guard"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list MIMAP-194A closeout guard"
guard_expect_in_file "$TAG" "recordLocalFreeReuse" "$LEDGER_OWNER" "closeout must keep ledger recording in the reuse ledger owner"

bash "$RUN_PROOF" --closeout-pack segment-map-local-free-reuse-ledger-bridge --level L2 --dry-run >/tmp/"$TAG".proof_dry_run 2>&1 || {
  cat /tmp/"$TAG".proof_dry_run >&2
  rm -f /tmp/"$TAG".proof_dry_run
  guard_fail "$TAG" "segment-map local-free reuse ledger bridge L2 dry-run selection failed"
}
guard_expect_in_file "$TAG" "MIMAP-192A" /tmp/"$TAG".proof_dry_run "L2 dry-run must include MIMAP-192A"
rm -f /tmp/"$TAG".proof_dry_run

bash "$GUARD_192A" --level L2

pure_first_guard_build_toolchain

tmp_dir="$(mktemp -d /tmp/hakorune_mimap194a_reuse_ledger_bridge_closeout.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT

mir_json="$tmp_dir/mimap194a.mir.json"
exe_out="$tmp_dir/mimap194a.exe"
build_log="$tmp_dir/build.log"
run_log="$tmp_dir/run.log"
vm_log="$tmp_dir/vm.log"

if ! pure_first_guard_run_vm "$TAG" "$ROOT_DIR" "$APP" "$vm_log"; then
  exit 1
fi

rg -F -q 'hako-alloc-segment-map-local-free-reuse-ledger-bridge-proof' "$vm_log"
rg -F -q 'reuse=1,0,4,70007002' "$vm_log"
rg -F -q 'first=1,0,0,-1,70007004,70007002,70,7,4,5,6,1,1' "$vm_log"
rg -F -q 'duplicate=0,4,0' "$vm_log"
rg -F -q 'missing=0,1' "$vm_log"
rg -F -q 'unsupported=0,5' "$vm_log"
rg -F -q 'reads=70007004,4' "$vm_log"
rg -F -q 'counts=4,1,3,1,1,1,1,1' "$vm_log"
rg -F -q 'inactive=0,0,0,0,0,0,0,0,0,0,0' "$vm_log"
rg -F -q 'check=1' "$vm_log"
rg -F -q 'summary=ok' "$vm_log"

pure_first_guard_emit_mir "$ROOT_DIR" "$APP" "$mir_json"
pure_first_guard_build_exe "$TAG" "$ROOT_DIR" "$APP" "$mir_json" "$exe_out" "$build_log"
pure_first_guard_assert_clean_build_log "$TAG" "$build_log"
pure_first_guard_run_exe "$TAG" "$exe_out" "$run_log"

rg -F -q 'hako-alloc-segment-map-local-free-reuse-ledger-bridge-proof' "$run_log"
rg -F -q 'reuse=1,0,4,70007002' "$run_log"
rg -F -q 'first=1,0,0,-1,70007004,70007002,70,7,4,5,6,1,1' "$run_log"
rg -F -q 'duplicate=0,4,0' "$run_log"
rg -F -q 'missing=0,1' "$run_log"
rg -F -q 'unsupported=0,5' "$run_log"
rg -F -q 'reads=70007004,4' "$run_log"
rg -F -q 'counts=4,1,3,1,1,1,1,1' "$run_log"
rg -F -q 'inactive=0,0,0,0,0,0,0,0,0,0,0' "$run_log"
rg -F -q 'check=1' "$run_log"
rg -F -q 'summary=ok' "$run_log"

echo "[$TAG] ok"
