#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../../.." && pwd)"
TAG="k2-wide-hako-alloc-segment-arena-backing-no-escape-address-capability-closeout"
cd "$ROOT_DIR"
source "$ROOT_DIR/tools/checks/lib/pure_first_exe_guard.sh"

SSOT="docs/development/current/main/design/hako-alloc-segment-arena-backing-no-escape-address-capability-closeout-ssot.md"
INVENTORY_SSOT="docs/development/current/main/design/hako-alloc-segment-arena-backing-no-escape-address-capability-ssot.md"
DIAGNOSTICS_SSOT="docs/development/current/main/design/hako-alloc-segment-arena-backing-no-escape-address-capability-diagnostics-ssot.md"
CADENCE="docs/development/current/main/design/mimalloc-row-validation-cadence-ssot.md"
TASKBOARD="docs/development/current/main/phases/phase-293x/293x-mimalloc-port-taskboard.md"
GRANULARITY="docs/development/current/main/design/mimalloc-allocator-first-task-granularity-ssot.md"
JOINT="docs/development/current/main/design/mimalloc-hakorune-joint-task-order-ssot.md"
INDEX="docs/tools/check-scripts-index.md"
PROOF_MANIFEST="tools/checks/proof_apps.toml"
GUARD_MANIFEST="tools/checks/guard_rows.toml"
CARD_244A="docs/development/current/main/phases/phase-293x/293x-767-MIMAP-244A-SEGMENT-ARENA-BACKING-NO-ESCAPE-RAW-POINTER-CAPABILITY-INVENTORY.md"
CARD_245A="docs/development/current/main/phases/phase-293x/293x-768-MIMAP-245A-SEGMENT-ARENA-BACKING-NO-ESCAPE-ADDRESS-CAPABILITY-DIAGNOSTICS.md"
CARD_246A="docs/development/current/main/phases/phase-293x/293x-769-MIMAP-246A-SEGMENT-ARENA-BACKING-NO-ESCAPE-ADDRESS-CAPABILITY-CLOSEOUT-PACK.md"
CARD_247A="docs/development/current/main/phases/phase-293x/293x-770-MIMAP-247A-POST-SEGMENT-ARENA-BACKING-NO-ESCAPE-ADDRESS-CAPABILITY-CLOSEOUT-ROW-SELECTION.md"
GUARD_244A="tools/checks/k2_wide_hako_alloc_segment_arena_backing_no_escape_address_capability_guard.sh"
GUARD_245A="tools/checks/k2_wide_hako_alloc_segment_arena_backing_no_escape_address_capability_diagnostics_guard.sh"
SELF_SCRIPT="tools/checks/k2_wide_hako_alloc_segment_arena_backing_no_escape_address_capability_closeout_guard.sh"
IMPL_SCRIPT="tools/checks/impl/k2_wide_hako_alloc_segment_arena_backing_no_escape_address_capability_closeout_guard.sh"
RUN_PROOF="tools/checks/run_proof_app.sh"
APP="apps/hako-alloc-segment-arena-backing-no-escape-address-capability-diagnostics-proof/main.hako"

echo "[$TAG] checking MIMAP-246A segment arena backing no-escape address capability closeout"

guard_require_files \
  "$TAG" \
  "$SSOT" \
  "$INVENTORY_SSOT" \
  "$DIAGNOSTICS_SSOT" \
  "$CADENCE" \
  "$TASKBOARD" \
  "$GRANULARITY" \
  "$JOINT" \
  "$INDEX" \
  "$PROOF_MANIFEST" \
  "$GUARD_MANIFEST" \
  "$CARD_244A" \
  "$CARD_245A" \
  "$CARD_246A" \
  "$CARD_247A" \
  "$GUARD_244A" \
  "$GUARD_245A" \
  "$SELF_SCRIPT" \
  "$IMPL_SCRIPT" \
  "$RUN_PROOF" \
  "$APP"

guard_require_exec_files "$TAG" "$GUARD_244A" "$GUARD_245A" "$SELF_SCRIPT" "$IMPL_SCRIPT" "$RUN_PROOF"

guard_expect_in_file "$TAG" "Status: landed" "$CARD_244A" "MIMAP-244A must be landed before closeout"
guard_expect_in_file "$TAG" "Status: landed" "$CARD_245A" "MIMAP-245A must be landed before closeout"
guard_expect_in_file "$TAG" "Status: landed" "$CARD_246A" "MIMAP-246A closeout card must be landed"
guard_expect_in_file "$TAG" "Status: selected current" "$CARD_247A" "MIMAP-247A must be selected current after closeout"

guard_expect_in_file "$TAG" "Decision: accepted" "$SSOT" "MIMAP-246A closeout SSOT must be accepted"
guard_expect_in_file "$TAG" "Decision: accepted" "$INVENTORY_SSOT" "MIMAP-244A inventory SSOT must stay accepted"
guard_expect_in_file "$TAG" "Decision: accepted" "$DIAGNOSTICS_SSOT" "MIMAP-245A diagnostics SSOT must stay accepted"
guard_expect_in_file "$TAG" "closeout_pack = segment-arena-backing-no-escape-address-capability" "$SSOT" "closeout SSOT must name pack"
guard_expect_in_file "$TAG" "MIMAP-247A post-segment-arena-backing-no-escape-address-capability-closeout row selection" "$SSOT" "closeout SSOT must name next row"

guard_expect_in_file "$TAG" "MIMAP-246A segment arena backing no-escape address capability closeout pack" "$GRANULARITY" "granularity SSOT must describe MIMAP-246A"
guard_expect_in_file "$TAG" "MIMAP-247A" "$GRANULARITY" "granularity SSOT must describe MIMAP-247A"
guard_expect_in_file "$TAG" "MIMAP-246A segment arena backing no-escape address capability closeout pack" "$JOINT" "joint order must name closeout row"
guard_expect_in_file "$TAG" "MIMAP-247A post-segment-arena-backing-no-escape-address-capability-closeout row selection" "$JOINT" "joint order must name next row"
guard_expect_in_file "$TAG" "MIMAP-247A" "$TASKBOARD" "taskboard must name selected next row"
guard_expect_in_file "$TAG" "MIMAP-246A segment arena backing no-escape address capability closeout pack" "$CADENCE" "cadence SSOT must anchor closeout row"

guard_expect_in_file "$TAG" "id = \"MIMAP-244A\"" "$PROOF_MANIFEST" "proof manifest must include MIMAP-244A"
guard_expect_in_file "$TAG" "id = \"MIMAP-245A\"" "$PROOF_MANIFEST" "proof manifest must include MIMAP-245A"
guard_expect_in_file "$TAG" "closeout_pack = \"segment-arena-backing-no-escape-address-capability\"" "$PROOF_MANIFEST" "proof manifest must assign no-escape address capability closeout pack"
guard_expect_in_file "$TAG" "id = \"hako-alloc-segment-arena-backing-no-escape-address-capability-closeout\"" "$GUARD_MANIFEST" "guard manifest must include MIMAP-246A closeout row"
guard_expect_in_file "$TAG" "$GUARD_244A" "$INDEX" "check index must list MIMAP-244A guard"
guard_expect_in_file "$TAG" "$GUARD_245A" "$INDEX" "check index must list MIMAP-245A guard"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list MIMAP-246A closeout guard"

bash "$RUN_PROOF" --closeout-pack segment-arena-backing-no-escape-address-capability --level L2 --dry-run >/tmp/"$TAG".proof_dry_run 2>&1 || {
  cat /tmp/"$TAG".proof_dry_run >&2
  rm -f /tmp/"$TAG".proof_dry_run
  guard_fail "$TAG" "segment arena backing no-escape address capability L2 dry-run selection failed"
}
guard_expect_in_file "$TAG" "MIMAP-244A" /tmp/"$TAG".proof_dry_run "L2 dry-run must include MIMAP-244A"
guard_expect_in_file "$TAG" "MIMAP-245A" /tmp/"$TAG".proof_dry_run "L2 dry-run must include MIMAP-245A"
rm -f /tmp/"$TAG".proof_dry_run

bash "$GUARD_244A" --level L2
bash "$GUARD_245A" --level L2

tmp_dir="$(mktemp -d /tmp/hakorune_mimap246_no_escape_addr_closeout.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT

mir_json="$tmp_dir/mimap246.mir.json"
exe_out="$tmp_dir/mimap246_exe"
build_log="$tmp_dir/build.log"
run_log="$tmp_dir/run.log"

pure_first_guard_emit_mir "$ROOT_DIR" "$APP" "$mir_json"
pure_first_guard_build_exe "$TAG" "$ROOT_DIR" "$APP" "$mir_json" "$exe_out" "$build_log"
pure_first_guard_assert_clean_build_log "$TAG" "$build_log"
pure_first_guard_run_exe "$TAG" "$exe_out" "$run_log"

rg -F -q 'hako-alloc-segment-arena-backing-no-escape-address-capability-diagnostics-proof' "$run_log"
rg -F -q 'diag=1,15,16,1,15,3,9,15,70,7,1' "$run_log"
rg -F -q 'seen=1,1,1,1,1,1,1,1,1,1,1,1,1,1,1' "$run_log"
rg -F -q 'inactive=0,0,0,0,0,0,0,0,0' "$run_log"
rg -F -q 'summary=ok' "$run_log"

echo "[$TAG] ok"
