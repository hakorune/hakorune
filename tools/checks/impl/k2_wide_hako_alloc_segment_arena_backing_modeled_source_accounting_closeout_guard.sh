#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../../.." && pwd)"
TAG="k2-wide-hako-alloc-segment-arena-backing-modeled-source-accounting-closeout"
cd "$ROOT_DIR"
source "$ROOT_DIR/tools/checks/lib/pure_first_exe_guard.sh"

SSOT="docs/development/current/main/design/hako-alloc-segment-arena-backing-modeled-source-accounting-closeout-ssot.md"
INVENTORY_SSOT="docs/development/current/main/design/hako-alloc-segment-arena-backing-modeled-source-accounting-ssot.md"
DIAGNOSTICS_SSOT="docs/development/current/main/design/hako-alloc-segment-arena-backing-modeled-source-accounting-diagnostics-ssot.md"
CADENCE="docs/development/current/main/design/mimalloc-row-validation-cadence-ssot.md"
TASKBOARD="docs/development/current/main/phases/phase-293x/293x-mimalloc-port-taskboard.md"
GRANULARITY="docs/development/current/main/design/mimalloc-allocator-first-task-granularity-ssot.md"
JOINT="docs/development/current/main/design/mimalloc-hakorune-joint-task-order-ssot.md"
INDEX="docs/tools/check-scripts-index.md"
PROOF_MANIFEST="tools/checks/proof_apps.toml"
GUARD_MANIFEST="tools/checks/guard_rows.toml"
CARD_264A="docs/development/current/main/phases/phase-293x/293x-787-MIMAP-264A-SEGMENT-ARENA-BACKING-MODELED-SOURCE-ACCOUNTING-INVENTORY.md"
CARD_265A="docs/development/current/main/phases/phase-293x/293x-788-MIMAP-265A-SEGMENT-ARENA-BACKING-MODELED-SOURCE-ACCOUNTING-DIAGNOSTICS.md"
CARD_266A="docs/development/current/main/phases/phase-293x/293x-789-MIMAP-266A-SEGMENT-ARENA-BACKING-MODELED-SOURCE-ACCOUNTING-CLOSEOUT.md"
CARD_REPORT_RECORD="docs/development/current/main/phases/phase-293x/293x-790-HAKO-ALLOC-REPORT-RECORD-003-SEGMENT-ARENA-BACKING-REPORT-RECORD-CARRIER-INVENTORY.md"
GUARD_264A="tools/checks/k2_wide_hako_alloc_segment_arena_backing_modeled_source_accounting_guard.sh"
GUARD_265A="tools/checks/k2_wide_hako_alloc_segment_arena_backing_modeled_source_accounting_diagnostics_guard.sh"
SELF_SCRIPT="tools/checks/k2_wide_hako_alloc_segment_arena_backing_modeled_source_accounting_closeout_guard.sh"
IMPL_SCRIPT="tools/checks/impl/k2_wide_hako_alloc_segment_arena_backing_modeled_source_accounting_closeout_guard.sh"
RUN_PROOF="tools/checks/run_proof_app.sh"
APP="apps/hako-alloc-segment-arena-backing-modeled-source-accounting-diagnostics-proof/main.hako"

echo "[$TAG] checking MIMAP-266A segment arena backing modeled source accounting closeout"

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
  "$CARD_264A" \
  "$CARD_265A" \
  "$CARD_266A" \
  "$CARD_REPORT_RECORD" \
  "$GUARD_264A" \
  "$GUARD_265A" \
  "$SELF_SCRIPT" \
  "$IMPL_SCRIPT" \
  "$RUN_PROOF" \
  "$APP"

guard_require_exec_files "$TAG" "$GUARD_264A" "$GUARD_265A" "$SELF_SCRIPT" "$IMPL_SCRIPT" "$RUN_PROOF"

guard_expect_in_file "$TAG" "Status: landed" "$CARD_264A" "MIMAP-264A must be landed before closeout"
guard_expect_in_file "$TAG" "Status: landed" "$CARD_265A" "MIMAP-265A must be landed before closeout"
guard_expect_in_file "$TAG" "Status: landed" "$CARD_266A" "MIMAP-266A closeout card must be landed"
guard_expect_in_file "$TAG" "Status: selected current" "$CARD_REPORT_RECORD" "report-record inventory must be selected current after closeout"

guard_expect_in_file "$TAG" "Decision: accepted" "$SSOT" "MIMAP-266A closeout SSOT must be accepted"
guard_expect_in_file "$TAG" "Decision: accepted" "$INVENTORY_SSOT" "MIMAP-264A inventory SSOT must stay accepted"
guard_expect_in_file "$TAG" "Decision: accepted" "$DIAGNOSTICS_SSOT" "MIMAP-265A diagnostics SSOT must stay accepted"
guard_expect_in_file "$TAG" "closeout_pack = segment-arena-backing-modeled-source-accounting" "$SSOT" "closeout SSOT must name pack"
guard_expect_in_file "$TAG" "HAKO-ALLOC-REPORT-RECORD-003 segment arena backing report record carrier inventory" "$SSOT" "closeout SSOT must name next row"

guard_expect_in_file "$TAG" "MIMAP-266A segment arena backing modeled source accounting closeout pack" "$GRANULARITY" "granularity SSOT must describe MIMAP-266A"
guard_expect_in_file "$TAG" "HAKO-ALLOC-REPORT-RECORD-003" "$GRANULARITY" "granularity SSOT must describe report-record follow-up"
guard_expect_in_file "$TAG" "MIMAP-266A segment arena backing modeled source accounting closeout pack" "$JOINT" "joint order must name closeout row"
guard_expect_in_file "$TAG" "HAKO-ALLOC-REPORT-RECORD-003 segment arena backing report record carrier inventory" "$JOINT" "joint order must name report-record follow-up"
guard_expect_in_file "$TAG" "HAKO-ALLOC-REPORT-RECORD-003" "$TASKBOARD" "taskboard must name selected report-record row"
guard_expect_in_file "$TAG" "MIMAP-266A segment arena backing modeled source accounting closeout pack" "$CADENCE" "cadence SSOT must anchor closeout row"

guard_expect_in_file "$TAG" "id = \"MIMAP-264A\"" "$PROOF_MANIFEST" "proof manifest must include MIMAP-264A"
guard_expect_in_file "$TAG" "id = \"MIMAP-265A\"" "$PROOF_MANIFEST" "proof manifest must include MIMAP-265A"
guard_expect_in_file "$TAG" "closeout_pack = \"segment-arena-backing-modeled-source-accounting\"" "$PROOF_MANIFEST" "proof manifest must assign modeled source accounting closeout pack"
guard_expect_in_file "$TAG" "id = \"hako-alloc-segment-arena-backing-modeled-source-accounting-closeout\"" "$GUARD_MANIFEST" "guard manifest must include MIMAP-266A closeout row"
guard_expect_in_file "$TAG" "$GUARD_264A" "$INDEX" "check index must list MIMAP-264A guard"
guard_expect_in_file "$TAG" "$GUARD_265A" "$INDEX" "check index must list MIMAP-265A guard"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list MIMAP-266A closeout guard"

bash "$RUN_PROOF" --closeout-pack segment-arena-backing-modeled-source-accounting --level L2 --dry-run >/tmp/"$TAG".proof_dry_run 2>&1 || {
  cat /tmp/"$TAG".proof_dry_run >&2
  rm -f /tmp/"$TAG".proof_dry_run
  guard_fail "$TAG" "modeled source accounting L2 dry-run selection failed"
}
guard_expect_in_file "$TAG" "MIMAP-264A" /tmp/"$TAG".proof_dry_run "L2 dry-run must include MIMAP-264A"
guard_expect_in_file "$TAG" "MIMAP-265A" /tmp/"$TAG".proof_dry_run "L2 dry-run must include MIMAP-265A"
rm -f /tmp/"$TAG".proof_dry_run

bash "$GUARD_264A" --level L2
bash "$GUARD_265A" --level L2

tmp_dir="$(mktemp -d /tmp/hakorune_mimap266_source_accounting_closeout.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT

mir_json="$tmp_dir/mimap266.mir.json"
exe_out="$tmp_dir/mimap266_exe"
build_log="$tmp_dir/build.log"
run_log="$tmp_dir/run.log"

pure_first_guard_emit_mir "$ROOT_DIR" "$APP" "$mir_json"
pure_first_guard_build_exe "$TAG" "$ROOT_DIR" "$APP" "$mir_json" "$exe_out" "$build_log"
pure_first_guard_assert_clean_build_log "$TAG" "$build_log"
pure_first_guard_run_exe "$TAG" "$exe_out" "$run_log"

rg -F -q 'hako-alloc-segment-arena-backing-modeled-source-accounting-diagnostics-proof' "$run_log"
rg -F -q 'diag=1,5,6,1,5,1,5,70,7,1' "$run_log"
rg -F -q 'seen=1,1,1,1,1' "$run_log"
rg -F -q 'account=0,16384,4096,0,0,0' "$run_log"
rg -F -q 'owner=2,1,1,1,1' "$run_log"
rg -F -q 'empty=0,1,0' "$run_log"
rg -F -q 'inactive=0,0,0,0,0,0,0,0,0' "$run_log"
rg -F -q 'check=1' "$run_log"
rg -F -q 'summary=ok' "$run_log"

echo "[$TAG] ok"
