#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../../.." && pwd)"
TAG="k2-wide-hako-alloc-segment-arena-backing-modeled-no-escape-address-residence-closeout"
cd "$ROOT_DIR"
source "$ROOT_DIR/tools/checks/lib/pure_first_exe_guard.sh"

SSOT="docs/development/current/main/design/hako-alloc-segment-arena-backing-modeled-no-escape-address-residence-closeout-ssot.md"
INVENTORY_SSOT="docs/development/current/main/design/hako-alloc-segment-arena-backing-modeled-no-escape-address-residence-ssot.md"
DIAGNOSTICS_SSOT="docs/development/current/main/design/hako-alloc-segment-arena-backing-modeled-no-escape-address-residence-diagnostics-ssot.md"
CADENCE="docs/development/current/main/design/mimalloc-row-validation-cadence-ssot.md"
TASKBOARD="docs/development/current/main/phases/phase-293x/293x-mimalloc-port-taskboard.md"
GRANULARITY="docs/development/current/main/design/mimalloc-allocator-first-task-granularity-ssot.md"
JOINT="docs/development/current/main/design/mimalloc-hakorune-joint-task-order-ssot.md"
INDEX="docs/tools/check-scripts-index.md"
PROOF_MANIFEST="tools/checks/proof_apps.toml"
GUARD_MANIFEST="tools/checks/guard_rows.toml"
CARD_248A="docs/development/current/main/phases/phase-293x/293x-771-MIMAP-248A-SEGMENT-ARENA-BACKING-MODELED-NO-ESCAPE-ADDRESS-RESIDENCE-INVENTORY.md"
CARD_249A="docs/development/current/main/phases/phase-293x/293x-772-MIMAP-249A-SEGMENT-ARENA-BACKING-MODELED-NO-ESCAPE-ADDRESS-RESIDENCE-DIAGNOSTICS.md"
CARD_250A="docs/development/current/main/phases/phase-293x/293x-773-MIMAP-250A-SEGMENT-ARENA-BACKING-MODELED-NO-ESCAPE-ADDRESS-RESIDENCE-CLOSEOUT-PACK.md"
CARD_251A="docs/development/current/main/phases/phase-293x/293x-774-MIMAP-251A-POST-SEGMENT-ARENA-BACKING-MODELED-NO-ESCAPE-ADDRESS-RESIDENCE-CLOSEOUT-ROW-SELECTION.md"
GUARD_248A="tools/checks/k2_wide_hako_alloc_segment_arena_backing_modeled_no_escape_address_residence_guard.sh"
GUARD_249A="tools/checks/k2_wide_hako_alloc_segment_arena_backing_modeled_no_escape_address_residence_diagnostics_guard.sh"
SELF_SCRIPT="tools/checks/k2_wide_hako_alloc_segment_arena_backing_modeled_no_escape_address_residence_closeout_guard.sh"
IMPL_SCRIPT="tools/checks/impl/k2_wide_hako_alloc_segment_arena_backing_modeled_no_escape_address_residence_closeout_guard.sh"
RUN_PROOF="tools/checks/run_proof_app.sh"
APP="apps/hako-alloc-segment-arena-backing-modeled-no-escape-address-residence-diagnostics-proof/main.hako"

echo "[$TAG] checking MIMAP-250A segment arena backing modeled no-escape address residence closeout"

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
  "$CARD_248A" \
  "$CARD_249A" \
  "$CARD_250A" \
  "$CARD_251A" \
  "$GUARD_248A" \
  "$GUARD_249A" \
  "$SELF_SCRIPT" \
  "$IMPL_SCRIPT" \
  "$RUN_PROOF" \
  "$APP"

guard_require_exec_files "$TAG" "$GUARD_248A" "$GUARD_249A" "$SELF_SCRIPT" "$IMPL_SCRIPT" "$RUN_PROOF"

guard_expect_in_file "$TAG" "Status: landed" "$CARD_248A" "MIMAP-248A must be landed before closeout"
guard_expect_in_file "$TAG" "Status: landed" "$CARD_249A" "MIMAP-249A must be landed before closeout"
guard_expect_in_file "$TAG" "Status: landed" "$CARD_250A" "MIMAP-250A closeout card must be landed"
guard_expect_in_file "$TAG" "Status: selected current" "$CARD_251A" "MIMAP-251A must be selected current after closeout"

guard_expect_in_file "$TAG" "Decision: accepted" "$SSOT" "MIMAP-250A closeout SSOT must be accepted"
guard_expect_in_file "$TAG" "Decision: accepted" "$INVENTORY_SSOT" "MIMAP-248A inventory SSOT must stay accepted"
guard_expect_in_file "$TAG" "Decision: accepted" "$DIAGNOSTICS_SSOT" "MIMAP-249A diagnostics SSOT must stay accepted"
guard_expect_in_file "$TAG" "closeout_pack = segment-arena-backing-modeled-no-escape-address-residence" "$SSOT" "closeout SSOT must name pack"
guard_expect_in_file "$TAG" "MIMAP-251A post-segment-arena-backing-modeled-no-escape-address-residence-closeout row selection" "$SSOT" "closeout SSOT must name next row"

guard_expect_in_file "$TAG" "MIMAP-250A segment arena backing modeled no-escape address residence closeout pack" "$GRANULARITY" "granularity SSOT must describe MIMAP-250A"
guard_expect_in_file "$TAG" "MIMAP-251A" "$GRANULARITY" "granularity SSOT must describe MIMAP-251A"
guard_expect_in_file "$TAG" "MIMAP-250A segment arena backing modeled no-escape address residence closeout pack" "$JOINT" "joint order must name closeout row"
guard_expect_in_file "$TAG" "MIMAP-251A post-segment-arena-backing-modeled-no-escape-address-residence-closeout row selection" "$JOINT" "joint order must name next row"
guard_expect_in_file "$TAG" "MIMAP-251A" "$TASKBOARD" "taskboard must name selected next row"
guard_expect_in_file "$TAG" "MIMAP-250A segment arena backing modeled no-escape address residence closeout pack" "$CADENCE" "cadence SSOT must anchor closeout row"

guard_expect_in_file "$TAG" "id = \"MIMAP-248A\"" "$PROOF_MANIFEST" "proof manifest must include MIMAP-248A"
guard_expect_in_file "$TAG" "id = \"MIMAP-249A\"" "$PROOF_MANIFEST" "proof manifest must include MIMAP-249A"
guard_expect_in_file "$TAG" "closeout_pack = \"segment-arena-backing-modeled-no-escape-address-residence\"" "$PROOF_MANIFEST" "proof manifest must assign modeled residence closeout pack"
guard_expect_in_file "$TAG" "id = \"hako-alloc-segment-arena-backing-modeled-no-escape-address-residence-closeout\"" "$GUARD_MANIFEST" "guard manifest must include MIMAP-250A closeout row"
guard_expect_in_file "$TAG" "$GUARD_248A" "$INDEX" "check index must list MIMAP-248A guard"
guard_expect_in_file "$TAG" "$GUARD_249A" "$INDEX" "check index must list MIMAP-249A guard"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list MIMAP-250A closeout guard"

bash "$RUN_PROOF" --closeout-pack segment-arena-backing-modeled-no-escape-address-residence --level L2 --dry-run >/tmp/"$TAG".proof_dry_run 2>&1 || {
  cat /tmp/"$TAG".proof_dry_run >&2
  rm -f /tmp/"$TAG".proof_dry_run
  guard_fail "$TAG" "modeled no-escape address residence L2 dry-run selection failed"
}
guard_expect_in_file "$TAG" "MIMAP-248A" /tmp/"$TAG".proof_dry_run "L2 dry-run must include MIMAP-248A"
guard_expect_in_file "$TAG" "MIMAP-249A" /tmp/"$TAG".proof_dry_run "L2 dry-run must include MIMAP-249A"
rm -f /tmp/"$TAG".proof_dry_run

bash "$GUARD_248A" --level L2
bash "$GUARD_249A" --level L2

tmp_dir="$(mktemp -d /tmp/hakorune_mimap250_modeled_residence_closeout.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT

mir_json="$tmp_dir/mimap250.mir.json"
exe_out="$tmp_dir/mimap250_exe"
build_log="$tmp_dir/build.log"
run_log="$tmp_dir/run.log"

pure_first_guard_emit_mir "$ROOT_DIR" "$APP" "$mir_json"
pure_first_guard_build_exe "$TAG" "$ROOT_DIR" "$APP" "$mir_json" "$exe_out" "$build_log"
pure_first_guard_assert_clean_build_log "$TAG" "$build_log"
pure_first_guard_run_exe "$TAG" "$exe_out" "$run_log"

rg -F -q 'hako-alloc-segment-arena-backing-modeled-no-escape-address-residence-diagnostics-proof' "$run_log"
rg -F -q 'diag=1,14,14,1,13,9,14,70,7,1' "$run_log"
rg -F -q 'seen=1,1,1,1,1,1,1,1,1,1,1,1,1' "$run_log"
rg -F -q 'inactive=0,0,0,0,0,0,0,0,0' "$run_log"
rg -F -q 'summary=ok' "$run_log"

echo "[$TAG] ok"
