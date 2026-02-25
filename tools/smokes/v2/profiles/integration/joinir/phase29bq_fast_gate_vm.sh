#!/bin/bash
# phase29bq_fast_gate_vm.sh - fast iteration gate for Phase 29bq (JoinIR/CorePlan)
#
# Default: run only the Phase 29bq lightweight gates (fast).
# Options:
#   --full            Run Phase 29bq lightweight gates, then 29bp dev gate (includes 29ae regression).
#   --only <mode>     Run a single mode/case. Built-ins: bq, 29bp, 29ae.
#                    Any other value is treated as case_id and must exist in phase29bq_fast_gate_cases.tsv.
#
# Logs:
# - Writes per-step logs to /tmp (or PHASE29BQ_FAST_LOG_DIR)
# - On failure prints "LOG: <path>" as the last line.

set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../../../../.." && pwd)" # tools/
LOG_DIR="${PHASE29BQ_FAST_LOG_DIR:-/tmp}"

source "$ROOT_DIR/smokes/v2/lib/test_runner.sh"
source "$ROOT_DIR/smokes/v2/lib/joinir_planner_first_gate.sh"
source "$ROOT_DIR/smokes/v2/lib/joinir_planner_first_list_gate.sh"
source "$ROOT_DIR/smokes/v2/lib/vm_route_pin.sh"
require_env || exit 2

if [ -n "${NYASH_BIN:-}" ] && [ -x "${NYASH_BIN}" ]; then
  BIN_VER="$("${NYASH_BIN}" --version 2>/dev/null | head -n 1 || true)"
  echo "[INFO] NYASH_BIN=${NYASH_BIN} ${BIN_VER}"
fi

# Compiler-lane gate contract:
# keep runtime execution on rust-vm lane to avoid vm-hako route drift
# contaminating JoinIR/CorePlan acceptance checks.
export_vm_route_pin

usage() {
  cat >&2 <<'EOF'
Usage:
  phase29bq_fast_gate_vm.sh [--full] [--only <mode-or-case_id>]

Notes:
  - Built-in modes: bq, 29bp, 29ae, full
  - Any other --only value is treated as case_id and must exist in:
    tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_cases.tsv
EOF
}

MODE="bq"
if [ "${1:-}" = "--full" ]; then
  MODE="full"
  shift
elif [ "${1:-}" = "--only" ]; then
  MODE="${2:-}"
  shift 2 || true
elif [ "${1:-}" = "-h" ] || [ "${1:-}" = "--help" ]; then
  usage
  exit 0
elif [ -n "${1:-}" ]; then
  echo "[FAIL] Unknown arg: $1" >&2
  usage
  exit 2
fi

LIST_FILE="$ROOT_DIR/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_cases.tsv"

GATE_29BP="$ROOT_DIR/smokes/v2/profiles/integration/joinir/phase29bp_planner_required_dev_gate_v4_vm.sh"
GATE_29AE="$ROOT_DIR/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh"
FAST_VERBOSE="${PHASE29BQ_FAST_VERBOSE:-0}"

phase29bq_run_cmd() {
  # Normalize shell-script invocation to avoid exec-bit drift in tools/checks.
  if [ "$#" -gt 0 ] && [ -f "$1" ] && [[ "$1" == *.sh ]]; then
    local script="$1"
    shift
    bash "$script" "$@"
    return
  fi
  "$@"
}

run_step() {
  local step_name="$1"
  local log_path="$2"
  shift 2

  if [ "$FAST_VERBOSE" = "1" ]; then
    if ! phase29bq_run_cmd "$@" 2>&1 | tee "$log_path"; then
      echo "[FAIL] gate failed: $step_name"
      echo "LOG: $log_path"
      return 1
    fi
    return 0
  fi

  if ! phase29bq_run_cmd "$@" >"$log_path" 2>&1; then
    echo "[FAIL] gate failed: $step_name"
    echo "LOG: $log_path"
    echo "[INFO] last log lines ($step_name):"
    tail -n 80 "$log_path" || true
    return 1
  fi

  echo "[PASS] $step_name"
  return 0
}

run_gate() {
  local gate="$1"
  local log_path="$2"
  run_step "$gate" "$log_path" "$gate"
}

RUN_ID="phase29bq_fast_gate_${$}"
LOG_BQ_LIST="$LOG_DIR/${RUN_ID}_bq_list.log"
LOG_LOOP_TRUE="$LOG_DIR/${RUN_ID}_loop_true.log"
LOG_COND_UPDATE="$LOG_DIR/${RUN_ID}_cond_update.log"
LOG_LOOP_COND="$LOG_DIR/${RUN_ID}_loop_cond.log"
LOG_STEP_TAIL_BREAK="$LOG_DIR/${RUN_ID}_step_tail_break.log"
LOG_GENERAL_IF="$LOG_DIR/${RUN_ID}_general_if.log"
LOG_P4_MULTIDELTA="$LOG_DIR/${RUN_ID}_p4_multidelta.log"
LOG_SELFHOST_PARSE_STRING2_MIN="$LOG_DIR/${RUN_ID}_selfhost_parse_string2_min.log"
LOG_SELFHOST_PARSE_STRING2_REAL_MIN="$LOG_DIR/${RUN_ID}_selfhost_parse_string2_real_min.log"
LOG_SELFHOST_PARSE_BLOCK_MIN="$LOG_DIR/${RUN_ID}_selfhost_parse_block_min.log"
LOG_CONTINUE_TARGET_HEADER="$LOG_DIR/${RUN_ID}_continue_target_header.log"
LOG_SELFHOST_SCAN_WITH_QUOTE_MIN="$LOG_DIR/${RUN_ID}_selfhost_scan_with_quote_min.log"
LOG_SELFHOST_PEEK_PARSE_MIN="$LOG_DIR/${RUN_ID}_selfhost_peek_parse_min.log"
LOG_SELFHOST_BUNDLE_RESOLVER_MIN="$LOG_DIR/${RUN_ID}_selfhost_bundle_resolver_min.log"
LOG_SELFHOST_SCAN_IDENT_MIN="$LOG_DIR/${RUN_ID}_selfhost_scan_ident_min.log"
LOG_SELFHOST_PARSE_TRY_MIN="$LOG_DIR/${RUN_ID}_selfhost_parse_try_min.log"
LOG_SELFHOST_PARSE_MAP_MIN="$LOG_DIR/${RUN_ID}_selfhost_parse_map_min.log"
LOG_SELFHOST_PARSE_PROGRAM2_WS_MIN="$LOG_DIR/${RUN_ID}_selfhost_parse_program2_ws_min.log"
LOG_SELFHOST_PARSE_USING_MIN="$LOG_DIR/${RUN_ID}_selfhost_parse_using_min.log"
LOG_SELFHOST_PARSE_STMT_SKIPWS_MIN="$LOG_DIR/${RUN_ID}_selfhost_parse_stmt_skipws_min.log"
LOG_SELFHOST_PARSE_PROGRAM2_NESTED_LOOP_MIN="$LOG_DIR/${RUN_ID}_selfhost_parse_program2_nested_loop_min.log"
LOG_SELFHOST_PARSER_STMT_EQUALS_MIN="$LOG_DIR/${RUN_ID}_selfhost_parser_stmt_equals_min.log"
LOG_HAKO_MIRBUILDER_PHASE0_PIN="$LOG_DIR/${RUN_ID}_hako_mirbuilder_phase0_pin.log"
LOG_HAKO_MIRBUILDER_CLEANUP_TRY_PIN="$LOG_DIR/${RUN_ID}_hako_mirbuilder_cleanup_try_pin.log"
LOG_HAKO_MIRBUILDER_CLEANUP_TRY_FINALLY_LOCAL_PIN="$LOG_DIR/${RUN_ID}_hako_mirbuilder_cleanup_try_finally_local_pin.log"
LOG_HAKO_MIRBUILDER_CLEANUP_TRY_FINALLY_LOCAL_VAR_MISMATCH_REJECT_PIN="$LOG_DIR/${RUN_ID}_hako_mirbuilder_cleanup_try_finally_local_var_mismatch_reject_pin.log"
LOG_HAKO_MIRBUILDER_CLEANUP_TRY_REJECT_NONMINIMAL_PIN="$LOG_DIR/${RUN_ID}_hako_mirbuilder_cleanup_try_reject_nonminimal_pin.log"
LOG_HAKO_MIRBUILDER_MULTI_LOCAL_ACCEPT_PIN="$LOG_DIR/${RUN_ID}_hako_mirbuilder_multi_local_accept_pin.log"
LOG_HAKO_MIRBUILDER_MULTI_LOCAL_REJECT_PIN="$LOG_DIR/${RUN_ID}_hako_mirbuilder_multi_local_reject_pin.log"
LOG_HAKO_PROGRAM_JSON_CONTRACT_PIN="$LOG_DIR/${RUN_ID}_hako_program_json_contract_pin.log"
LOG_JOINIR_PORT04_PHI_EXIT_INVARIANT_LOCK="$LOG_DIR/${RUN_ID}_joinir_port04_phi_exit_invariant_lock.log"
LOG_PLAN_LOWER_ENTRY="$LOG_DIR/${RUN_ID}_plan_lower_entry.log"
LOG_COREPLAN_BOUNDARY="$LOG_DIR/${RUN_ID}_coreplan_boundary.log"
LOG_BUILDER_EMIT_VISIBILITY="$LOG_DIR/${RUN_ID}_builder_emit_visibility.log"
LOG_MIR_DIAGNOSTICS="$LOG_DIR/${RUN_ID}_mir_diagnostics.log"
LOG_MIR_NO_LOWERED_AWAY_EMITTERS="$LOG_DIR/${RUN_ID}_mir_no_lowered_away_emitters.log"
LOG_MIR_PREFLIGHT_UNSUPPORTED="$LOG_DIR/${RUN_ID}_mir_preflight_unsupported.log"
LOG_HAKO_MIRBUILDER_NO_HOSTBRIDGE="$LOG_DIR/${RUN_ID}_hako_mirbuilder_no_hostbridge.log"
LOG_29BP="$LOG_DIR/${RUN_ID}_29bp.log"
LOG_29AE="$LOG_DIR/${RUN_ID}_29ae.log"
LOG_JOINIR_PORT07_EXPR_PARITY_SEED="$LOG_DIR/${RUN_ID}_joinir_port07_expr_parity_seed.log"

run_bq_gates() {
  run_step "no_cross_layer_builder_emit" "$LOG_BUILDER_EMIT_VISIBILITY" \
    "$ROOT_DIR/checks/no_cross_layer_builder_emit.sh" || return 1

  run_step "mir_diagnostics_contract" "$LOG_MIR_DIAGNOSTICS" \
    "$ROOT_DIR/checks/mir_diagnostics_contract.sh" || return 1

  run_step "mir_no_lowered_away_emitters" "$LOG_MIR_NO_LOWERED_AWAY_EMITTERS" \
    "$ROOT_DIR/checks/mir_no_lowered_away_emitters.sh" || return 1

  run_step "phase29bq_mir_preflight_unsupported_reject_vm" "$LOG_MIR_PREFLIGHT_UNSUPPORTED" \
    "$ROOT_DIR/smokes/v2/profiles/integration/joinir/phase29bq_mir_preflight_unsupported_reject_vm.sh" || return 1

  run_step "no_unapproved_plan_lowerer_entry" "$LOG_PLAN_LOWER_ENTRY" \
    "$ROOT_DIR/checks/no_unapproved_plan_lowerer_entry.sh" || return 1

  run_step "no_unapproved_coreplan_boundary" "$LOG_COREPLAN_BOUNDARY" \
    "$ROOT_DIR/checks/no_unapproved_coreplan_boundary.sh" || return 1

  run_step "hako_mirbuilder_no_hostbridge" "$LOG_HAKO_MIRBUILDER_NO_HOSTBRIDGE" \
    "$ROOT_DIR/checks/hako_mirbuilder_no_hostbridge.sh" || return 1

  run_step "phase29bq_fast_gate_cases" "$LOG_BQ_LIST" \
    run_planner_first_list_gate "$LIST_FILE" "phase29bq_fast_gate_cases" "${RUN_TIMEOUT_SECS:-10}" || return 1

  run_step "phase29bq_hako_mirbuilder_phase0_pin_vm" "$LOG_HAKO_MIRBUILDER_PHASE0_PIN" \
    "$ROOT_DIR/smokes/v2/profiles/integration/joinir/phase29bq_hako_mirbuilder_phase0_pin_vm.sh" || return 1

  run_step "phase29bq_hako_mirbuilder_cleanup_try_min_vm" "$LOG_HAKO_MIRBUILDER_CLEANUP_TRY_PIN" \
    "$ROOT_DIR/smokes/v2/profiles/integration/joinir/phase29bq_hako_mirbuilder_cleanup_try_min_vm.sh" || return 1

  run_step "phase29bq_hako_mirbuilder_cleanup_try_finally_local_min_vm" "$LOG_HAKO_MIRBUILDER_CLEANUP_TRY_FINALLY_LOCAL_PIN" \
    "$ROOT_DIR/smokes/v2/profiles/integration/joinir/phase29bq_hako_mirbuilder_cleanup_try_finally_local_min_vm.sh" || return 1

  run_step "phase29bq_hako_mirbuilder_cleanup_try_finally_local_var_mismatch_reject_vm" "$LOG_HAKO_MIRBUILDER_CLEANUP_TRY_FINALLY_LOCAL_VAR_MISMATCH_REJECT_PIN" \
    "$ROOT_DIR/smokes/v2/profiles/integration/joinir/phase29bq_hako_mirbuilder_cleanup_try_finally_local_var_mismatch_reject_vm.sh" || return 1

  run_step "phase29bq_hako_mirbuilder_cleanup_try_reject_nonminimal_vm" "$LOG_HAKO_MIRBUILDER_CLEANUP_TRY_REJECT_NONMINIMAL_PIN" \
    "$ROOT_DIR/smokes/v2/profiles/integration/joinir/phase29bq_hako_mirbuilder_cleanup_try_reject_nonminimal_vm.sh" || return 1

  run_step "phase29bq_hako_mirbuilder_multi_local_accept_min_vm" "$LOG_HAKO_MIRBUILDER_MULTI_LOCAL_ACCEPT_PIN" \
    "$ROOT_DIR/smokes/v2/profiles/integration/joinir/phase29bq_hako_mirbuilder_multi_local_accept_min_vm.sh" || return 1

  run_step "phase29bq_hako_mirbuilder_multi_local_reject_vm" "$LOG_HAKO_MIRBUILDER_MULTI_LOCAL_REJECT_PIN" \
    "$ROOT_DIR/smokes/v2/profiles/integration/joinir/phase29bq_hako_mirbuilder_multi_local_reject_vm.sh" || return 1

  run_step "phase29bq_hako_program_json_contract_pin_vm" "$LOG_HAKO_PROGRAM_JSON_CONTRACT_PIN" \
    "$ROOT_DIR/smokes/v2/profiles/integration/joinir/phase29bq_hako_program_json_contract_pin_vm.sh" || return 1

  run_step "phase29bq_joinir_port04_phi_exit_invariant_lock_vm" "$LOG_JOINIR_PORT04_PHI_EXIT_INVARIANT_LOCK" \
    "$ROOT_DIR/smokes/v2/profiles/integration/joinir/phase29bq_joinir_port04_phi_exit_invariant_lock_vm.sh" || return 1

  run_step "phase29bq_joinir_port07_expr_parity_seed_vm" "$LOG_JOINIR_PORT07_EXPR_PARITY_SEED" \
    "$ROOT_DIR/smokes/v2/profiles/integration/joinir/phase29bq_joinir_port07_expr_parity_seed_vm.sh" || return 1
  return 0
}

run_case_from_list() {
  local case_id="$1"
  local log_path="$2"
  local line

  line=$(awk -F '\t' -v key="$case_id" '
    $0 ~ /^#/ {next}
    NF >= 5 && $5 == key {print; exit}
  ' "$LIST_FILE")

  if [ -z "$line" ]; then
    echo "[FAIL] case_id not found in list: $case_id" >&2
    return 1
  fi

  local fixture expected allowed_rc planner_tag
  IFS=$'\t' read -r fixture expected allowed_rc planner_tag _rest <<<"$line"

  if [ "$expected" = "__EMPTY__" ]; then
    expected=""
  fi
  if [ -z "$allowed_rc" ]; then
    allowed_rc="0"
  fi

  if [[ "$fixture" != /* ]]; then
    fixture="$NYASH_ROOT/$fixture"
  fi

  run_step "phase29bq_fast_gate_cases:$case_id" "$log_path" \
    run_planner_first_gate \
    "phase29bq_fast_gate_cases:$case_id" \
    "$fixture" \
    "$expected" \
    "$planner_tag" \
    "$allowed_rc" \
    "${RUN_TIMEOUT_SECS:-10}"
}

case_in_list() {
  local case_id="$1"
  awk -F '\t' -v key="$case_id" '
    $0 ~ /^#/ {next}
    NF >= 5 && $5 == key {found=1; exit}
    END {exit found ? 0 : 1}
  ' "$LIST_FILE"
}

validate_mode() {
  case "$MODE" in
    bq|29bp|29ae|full) return 0 ;;
    *)
      if case_in_list "$MODE"; then
        return 0
      fi
      echo "[FAIL] Invalid --only value: $MODE (not a built-in mode and not found in list)" >&2
      usage
      exit 2
      ;;
  esac
}

validate_mode

case "$MODE" in
  bq)
    run_bq_gates
    ;;
  29bp)
    run_gate "$GATE_29BP" "$LOG_29BP"
    ;;
  29ae)
    run_gate "$GATE_29AE" "$LOG_29AE"
    ;;
  full)
    run_bq_gates
    # NOTE: 29bp dev gate already runs 29ae regression pack (SSOT).
    run_gate "$GATE_29BP" "$LOG_29BP"
    ;;
  *)
    safe_mode="${MODE//[^a-zA-Z0-9_-]/_}"
    run_case_from_list "$MODE" "$LOG_DIR/${RUN_ID}_${safe_mode}.log"
    ;;
esac

echo "[PASS] phase29bq_fast_gate_vm: PASS (mode=$MODE)"
