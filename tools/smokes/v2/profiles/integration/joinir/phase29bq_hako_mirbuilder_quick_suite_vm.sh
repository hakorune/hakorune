#!/bin/bash
# phase29bq_hako_mirbuilder_quick_suite_vm.sh
# Quick verification bundle for .hako parser/mirbuilder changes.
#
# Default:
# - internal-only emit (cleanup_only_min)
# - M7 cleanup pins (min / finally-local / reject-nonminimal / reject-var-mismatch)
# - phase12 pin (Return(NewBox) minimal)
# - phase13 pin (Return(Call id()) minimal)
# - phase14 pin (Return(BoxCall StringBox(\"abc\").length()) minimal)
# - phase15 pin (Return(Call id(9)) minimal)
# - phase16 pin (Return(New StringBox(\"abc\")) minimal)
# - phase17 pin (Return(BoxCall StringBox(\"abc\").indexOf(\"b\")) minimal)
# - phase18 pin (Return(Call id(7)) minimal)
# - phase19 pin (Local(Int)>Local(Var)>Return(Var) with load/store)
# - phase20 pin (Local(Int)>Assignment(Int)>Return(Var) with load/store)
# - LS0 pin (mir_json_v0 load/store minimal)
#
# Options:
#   --with-stage1  Include stage1_cli internal-only emit milestone check
#   --with-bq      Include phase29bq fast gate (--only bq)

set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../../../../.." && pwd)" # tools/
source "$ROOT_DIR/smokes/v2/lib/test_runner.sh"
source "$ROOT_DIR/smokes/v2/lib/vm_route_pin.sh"
require_env || exit 2
# Contract: this suite validates .hako mirbuilder semantics on vm lane.
# Keep vm-hako preference disabled to avoid subset-check interference.
export_vm_route_pin

RUN_STAGE1=0
RUN_BQ=0

usage() {
  cat >&2 <<'EOF'
Usage:
  phase29bq_hako_mirbuilder_quick_suite_vm.sh [--with-stage1] [--with-bq]

Examples:
  bash tools/smokes/v2/profiles/integration/joinir/phase29bq_hako_mirbuilder_quick_suite_vm.sh
  bash tools/smokes/v2/profiles/integration/joinir/phase29bq_hako_mirbuilder_quick_suite_vm.sh --with-stage1
  bash tools/smokes/v2/profiles/integration/joinir/phase29bq_hako_mirbuilder_quick_suite_vm.sh --with-stage1 --with-bq
EOF
}

while [ $# -gt 0 ]; do
  case "$1" in
    --with-stage1)
      RUN_STAGE1=1
      shift
      ;;
    --with-bq)
      RUN_BQ=1
      shift
      ;;
    -h|--help)
      usage
      exit 0
      ;;
    *)
      echo "[FAIL] unknown option: $1" >&2
      usage
      exit 2
      ;;
  esac
done

run_step() {
  local label="$1"
  shift
  echo "[INFO] $label"
  "$@"
}

run_step "internal-only emit: cleanup_only_min" \
  bash "$ROOT_DIR/hakorune_emit_mir_mainline.sh" \
  "$NYASH_ROOT/apps/tests/phase29bq_selfhost_cleanup_only_min.hako" \
  "/tmp/phase29bq_cleanup_only_internal.mir.json"

run_step "pin: cleanup_try_min" \
  bash "$ROOT_DIR/smokes/v2/profiles/integration/joinir/phase29bq_hako_mirbuilder_cleanup_try_min_vm.sh"

run_step "pin: cleanup_try_finally_local_min" \
  bash "$ROOT_DIR/smokes/v2/profiles/integration/joinir/phase29bq_hako_mirbuilder_cleanup_try_finally_local_min_vm.sh"

run_step "pin: cleanup_try_reject_nonminimal" \
  bash "$ROOT_DIR/smokes/v2/profiles/integration/joinir/phase29bq_hako_mirbuilder_cleanup_try_reject_nonminimal_vm.sh"

run_step "pin: cleanup_try_finally_local_var_mismatch_reject" \
  bash "$ROOT_DIR/smokes/v2/profiles/integration/joinir/phase29bq_hako_mirbuilder_cleanup_try_finally_local_var_mismatch_reject_vm.sh"

run_step "pin: multi_local_accept_min" \
  bash "$ROOT_DIR/smokes/v2/profiles/integration/joinir/phase29bq_hako_mirbuilder_multi_local_accept_min_vm.sh"

run_step "pin: multi_local_reject" \
  bash "$ROOT_DIR/smokes/v2/profiles/integration/joinir/phase29bq_hako_mirbuilder_multi_local_reject_vm.sh"

run_step "pin: phase12_return_newbox_min" \
  bash "$ROOT_DIR/smokes/v2/profiles/integration/joinir/phase29bq_hako_mirbuilder_phase12_return_newbox_min_vm.sh"

run_step "pin: phase13_return_call_id0_min" \
  bash "$ROOT_DIR/smokes/v2/profiles/integration/joinir/phase29bq_hako_mirbuilder_phase13_return_call_id0_min_vm.sh"

run_step "pin: phase14_return_boxcall_stringbox_length_abc_min" \
  bash "$ROOT_DIR/smokes/v2/profiles/integration/joinir/phase29bq_hako_mirbuilder_phase14_return_boxcall_stringbox_length_abc_min_vm.sh"

run_step "pin: phase15_return_call_id1_int9_min" \
  bash "$ROOT_DIR/smokes/v2/profiles/integration/joinir/phase29bq_hako_mirbuilder_phase15_return_call_id1_int9_min_vm.sh"

run_step "pin: phase16_return_newbox_stringbox_abc_min" \
  bash "$ROOT_DIR/smokes/v2/profiles/integration/joinir/phase29bq_hako_mirbuilder_phase16_return_newbox_stringbox_abc_min_vm.sh"

run_step "pin: phase17_return_boxcall_stringbox_indexof_b_abc_min" \
  bash "$ROOT_DIR/smokes/v2/profiles/integration/joinir/phase29bq_hako_mirbuilder_phase17_return_boxcall_stringbox_indexof_b_abc_min_vm.sh"

run_step "pin: phase18_return_call_id1_int7_min" \
  bash "$ROOT_DIR/smokes/v2/profiles/integration/joinir/phase29bq_hako_mirbuilder_phase18_return_call_id1_int7_min_vm.sh"

run_step "pin: phase19_load_local_var_min" \
  bash "$ROOT_DIR/smokes/v2/profiles/integration/joinir/phase29bq_hako_mirbuilder_phase19_load_local_var_min_vm.sh"

run_step "pin: phase20_store_assignment_int_min" \
  bash "$ROOT_DIR/smokes/v2/profiles/integration/joinir/phase29bq_hako_mirbuilder_phase20_store_assignment_int_min_vm.sh"

run_step "pin: phase21_loop_if_return_var_min" \
  bash "$ROOT_DIR/smokes/v2/profiles/integration/joinir/phase29bq_hako_mirbuilder_phase21_loop_if_return_var_min_vm.sh"

run_step "pin: ls0_mir_json_v0_load_store_min" \
  bash "$ROOT_DIR/smokes/v2/profiles/integration/joinir/phase29bq_hako_mirbuilder_ls0_mir_json_v0_load_store_min_vm.sh"

if [ "$RUN_STAGE1" -eq 1 ]; then
  run_step "internal-only emit milestone: stage1_cli" \
    bash "$ROOT_DIR/hakorune_emit_mir_mainline.sh" \
    "$NYASH_ROOT/lang/src/runner/stage1_cli.hako" \
    "/tmp/stage1_cli_fullbuilder.mir.json"
fi

if [ "$RUN_BQ" -eq 1 ]; then
  run_step "fast gate: phase29bq --only bq" \
    bash "$ROOT_DIR/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh" --only bq
fi

echo "[PASS] phase29bq_hako_mirbuilder_quick_suite_vm: PASS (with_stage1=$RUN_STAGE1 with_bq=$RUN_BQ)"
