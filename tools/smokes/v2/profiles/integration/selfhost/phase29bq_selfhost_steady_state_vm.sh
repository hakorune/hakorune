#!/bin/bash
# phase29bq_selfhost_steady_state_vm.sh
# Single-command steady-state run for "Current blocker: none".
#
# Default sequence:
# 1) .hako mirbuilder quick suite
# 2) JoinIR fast gate (--only bq)
# 3) selfhost Stage-B route parity smoke
#
# Option:
#   --with-runtime-parity  Also run runtime route/mode parity smokes.
#   --no-collect-blocker   Do not auto-run blocker collector on bq gate failure.
#   --quiet                Suppress step command output (logs still saved).
#   --cleanup-old-logs     Remove old steady-state logs in LOG_DIR (>2 days).

set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../../../../.." && pwd)"
source "$ROOT_DIR/smokes/v2/lib/test_runner.sh"

RUN_RUNTIME_PARITY=0
AUTO_COLLECT_BLOCKER=1
QUIET=0
CLEANUP_OLD_LOGS=0
CLEANUP_DAYS=2
LOG_DIR="${PHASE29BQ_FAST_LOG_DIR:-/tmp}"
RUN_ID="phase29bq_selfhost_steady_state_${$}"
JOINIR_LIST_FILE="$ROOT_DIR/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_cases.tsv"
COLLECTOR_SCRIPT="$ROOT_DIR/smokes/v2/profiles/integration/joinir/phase29bq_collect_planner_required_blocker_vm.sh"

usage() {
  cat >&2 <<'EOF'
Usage:
  phase29bq_selfhost_steady_state_vm.sh [--with-runtime-parity] [--no-collect-blocker] [--quiet] [--cleanup-old-logs]

Examples:
  bash tools/smokes/v2/profiles/integration/selfhost/phase29bq_selfhost_steady_state_vm.sh
  bash tools/smokes/v2/profiles/integration/selfhost/phase29bq_selfhost_steady_state_vm.sh --with-runtime-parity
  bash tools/smokes/v2/profiles/integration/selfhost/phase29bq_selfhost_steady_state_vm.sh --no-collect-blocker
  bash tools/smokes/v2/profiles/integration/selfhost/phase29bq_selfhost_steady_state_vm.sh --quiet
  bash tools/smokes/v2/profiles/integration/selfhost/phase29bq_selfhost_steady_state_vm.sh --cleanup-old-logs
EOF
}

while [ $# -gt 0 ]; do
  case "$1" in
    --with-runtime-parity)
      RUN_RUNTIME_PARITY=1
      shift
      ;;
    --no-collect-blocker)
      AUTO_COLLECT_BLOCKER=0
      shift
      ;;
    --quiet)
      QUIET=1
      shift
      ;;
    --cleanup-old-logs)
      CLEANUP_OLD_LOGS=1
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

require_env || exit 2

sanitize_label() {
  local s="$1"
  s="${s//[^A-Za-z0-9_]/_}"
  s="${s//__/_}"
  echo "$s"
}

cleanup_old_logs() {
  if [ ! -d "$LOG_DIR" ]; then
    return 0
  fi
  local removed
  removed="$(find "$LOG_DIR" -maxdepth 1 -type f -name 'phase29bq_selfhost_steady_state_*' -mtime +"$CLEANUP_DAYS" -print -delete 2>/dev/null | wc -l | tr -d '[:space:]')"
  if [ "${removed:-0}" -gt 0 ]; then
    echo "[INFO] cleanup-old-logs: removed=$removed dir=$LOG_DIR mtime_days>$CLEANUP_DAYS"
  fi
}

extract_first_marker() {
  local log_path="$1"
  rg -n "\\[(plan/freeze:|plan/reject|joinir/freeze|freeze:)" "$log_path" | head -n 1 || true
}

extract_step_tree_nearby() {
  local log_path="$1"
  local marker_line="$2"
  awk -v stop="$marker_line" 'NR < stop && $0 ~ /StepTree root for/ { last = NR ":" $0 } END { print last }' "$log_path"
}

resolve_fixture_for_case_id() {
  local case_id="$1"
  if [ ! -f "$JOINIR_LIST_FILE" ]; then
    return 1
  fi
  awk -F'\t' -v key="$case_id" '
    $0 ~ /^#/ {next}
    NF >= 5 && $5 == key {print $1; exit}
  ' "$JOINIR_LIST_FILE"
}

collect_blocker_from_bq_log() {
  local log_path="$1"
  local case_id fixture fixture_abs label collector_log summary_path

  case_id="$(rg -n 'phase29bq_fast_gate_cases:[^:]+: FAIL' "$log_path" | head -n 1 | sed -E 's/.*phase29bq_fast_gate_cases:([^:]+): FAIL.*/\1/' || true)"
  if [ -z "$case_id" ]; then
    echo "[WARN] blocker collect skipped: no failed case_id found in bq log" >&2
    return 0
  fi

  fixture="$(resolve_fixture_for_case_id "$case_id" || true)"
  if [ -z "$fixture" ]; then
    # Fallback: treat case_id itself as fixture path if it exists.
    if [ -f "$case_id" ]; then
      fixture="$case_id"
    elif [ -f "$NYASH_ROOT/$case_id" ]; then
      fixture="$NYASH_ROOT/$case_id"
    else
      echo "[WARN] blocker collect skipped: fixture not found for case_id=$case_id" >&2
      return 0
    fi
  fi

  if [[ "$fixture" != /* ]]; then
    fixture_abs="$NYASH_ROOT/$fixture"
  else
    fixture_abs="$fixture"
  fi

  if [ ! -f "$fixture_abs" ]; then
    echo "[WARN] blocker collect skipped: fixture path missing: $fixture_abs" >&2
    return 0
  fi

  if [ ! -x "$COLLECTOR_SCRIPT" ]; then
    echo "[WARN] blocker collect skipped: collector missing/executable: $COLLECTOR_SCRIPT" >&2
    return 0
  fi

  label="$(sanitize_label "$case_id")"
  collector_log="$LOG_DIR/${RUN_ID}_collector_${label}.log"
  echo "[INFO] blocker collect: fixture=$fixture_abs label=$label" >&2

  set +e
  bash "$COLLECTOR_SCRIPT" "$fixture_abs" "$label" 2>&1 | tee "$collector_log"
  local rc=${PIPESTATUS[0]}
  set -e

  if [ "$rc" -ne 0 ]; then
    echo "[WARN] blocker collect failed rc=$rc log=$collector_log" >&2
    return 0
  fi

  summary_path="$(awk '/^\[ok\] wrote .*\.summary$/ {print $3}' "$collector_log" | tail -n 1 || true)"
  if [ -n "$summary_path" ]; then
    echo "[INFO] blocker summary: $summary_path" >&2
  fi
  return 0
}

run_step() {
  local key="$1"
  local label="$2"
  shift
  shift
  local log_path="$LOG_DIR/${RUN_ID}_${key}.log"
  local marker marker_line step_tree

  echo "[INFO] $label"
  local rc
  set +e
  if [ "$QUIET" -eq 1 ]; then
    "$@" >"$log_path" 2>&1
    rc=$?
  else
    "$@" 2>&1 | tee "$log_path"
    rc=${PIPESTATUS[0]}
  fi
  set -e

  if [ "$rc" -eq 0 ]; then
    if [ "$QUIET" -eq 1 ]; then
      echo "[OK] $key log=$log_path"
    fi
    return 0
  fi

  marker="$(extract_first_marker "$log_path")"
  marker_line="99999999"
  if [ -n "$marker" ]; then
    marker_line="${marker%%:*}"
  fi
  step_tree="$(extract_step_tree_nearby "$log_path" "$marker_line")"

  echo "[FAIL] step failed: key=$key rc=$rc" >&2
  echo "[FAIL] log: $log_path" >&2
  if [ -n "$marker" ]; then
    echo "[FAIL] first_freeze_or_reject: $marker" >&2
  fi
  if [ -n "$step_tree" ]; then
    echo "[FAIL] step_tree_root_nearby: $step_tree" >&2
  fi

  if [ "$key" = "fast_gate_bq" ] && [ "$AUTO_COLLECT_BLOCKER" -eq 1 ]; then
    collect_blocker_from_bq_log "$log_path"
  fi

  return "$rc"
}

if [ "$CLEANUP_OLD_LOGS" -eq 1 ]; then
  cleanup_old_logs
fi

run_step "quick_suite" "quick: hako mirbuilder quick suite" \
  bash "$ROOT_DIR/smokes/v2/profiles/integration/joinir/phase29bq_hako_mirbuilder_quick_suite_vm.sh"

run_step "fast_gate_bq" "quick: joinir fast gate (bq only)" \
  bash "$ROOT_DIR/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh" --only bq

run_step "route_stageb" "quick: selfhost stageb route parity" \
  bash "$ROOT_DIR/smokes/v2/profiles/integration/selfhost/phase29bq_selfhost_stageb_route_parity_smoke_vm.sh"

if [ "$RUN_RUNTIME_PARITY" -eq 1 ]; then
  run_step "route_runtime" "opt: selfhost runtime route parity" \
    bash "$ROOT_DIR/smokes/v2/profiles/integration/selfhost/phase29bq_selfhost_runtime_route_smoke_vm.sh"
  run_step "route_runtime_mode" "opt: selfhost runtime mode parity" \
    bash "$ROOT_DIR/smokes/v2/profiles/integration/selfhost/phase29bq_selfhost_runtime_mode_parity_smoke_vm.sh"
fi

echo "[PASS] phase29bq_selfhost_steady_state_vm: PASS (with_runtime_parity=$RUN_RUNTIME_PARITY auto_collect_blocker=$AUTO_COLLECT_BLOCKER quiet=$QUIET cleanup_old_logs=$CLEANUP_OLD_LOGS)"
