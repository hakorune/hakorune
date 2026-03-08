#!/bin/bash
# phase29bq_selfhost_planner_required_dev_gate_vm.sh - selfhost entry gate (subset list)
# Filter contract: use `SMOKES_SELFHOST_FILTER=<substring>`; this script does not parse `--only`.
# Prefer semantic route substrings; exact legacy fixture/filter tokens should be treated via the pin inventory SSOT.

set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../../../../.." && pwd)"
LOG_DIR="/tmp"
DEFAULT_LIST_FILE="$ROOT_DIR/smokes/v2/profiles/integration/selfhost/planner_required_selfhost_subset.tsv"
LIST_FILE="${SMOKES_SELFHOST_LIST:-$DEFAULT_LIST_FILE}"
RUN_TIMEOUT_SECS="${RUN_TIMEOUT_SECS:-20}"
STAGEB_TIMEOUT_SECS="${SMOKES_SELFHOST_STAGEB_TIMEOUT_SECS:-$RUN_TIMEOUT_SECS}"
JSON_TIMEOUT_SECS="${SMOKES_SELFHOST_JSON_TIMEOUT_SECS:-$RUN_TIMEOUT_SECS}"
MAX_CASES="${SMOKES_SELFHOST_MAX_CASES:-0}"
CASE_FILTER="${SMOKES_SELFHOST_FILTER:-}"
SHOW_PROGRESS="${SMOKES_SELFHOST_PROGRESS:-1}"
JOBS="${SMOKES_SELFHOST_JOBS:-4}"

source "$ROOT_DIR/smokes/v2/lib/test_runner.sh"
source "$ROOT_DIR/smokes/v2/lib/vm_route_pin.sh"
require_env || exit 2

if ! [[ "$MAX_CASES" =~ ^[0-9]+$ ]]; then
  log_error "SMOKES_SELFHOST_MAX_CASES must be an integer (got: $MAX_CASES)"
  exit 2
fi

if ! [[ "$SHOW_PROGRESS" =~ ^[01]$ ]]; then
  log_error "SMOKES_SELFHOST_PROGRESS must be 0 or 1 (got: $SHOW_PROGRESS)"
  exit 2
fi

if ! [[ "$JOBS" =~ ^[0-9]+$ ]] || [ "$JOBS" -lt 1 ]; then
  log_error "SMOKES_SELFHOST_JOBS must be an integer >= 1 (got: $JOBS)"
  exit 2
fi

COMPILER="$NYASH_ROOT/lang/src/compiler/entry/compiler.hako"
STAGEB_WRAPPER="$NYASH_ROOT/tools/selfhost/run_stageb_compiler_vm.sh"

HAKO_STAGEB_MODULES_LIST="$(collect_stageb_modules_list "$NYASH_ROOT")"
HAKO_STAGEB_MODULE_ROOTS_LIST="$(collect_stageb_module_roots_list "$NYASH_ROOT")"

if [ "${SMOKES_ENABLE_SELFHOST:-0}" != "1" ]; then
  log_warn "[SKIP] selfhost gate disabled (set SMOKES_ENABLE_SELFHOST=1)"
  exit 0
fi

if [ ! -f "$COMPILER" ]; then
  log_warn "[SKIP] compiler.hako missing: $COMPILER"
  exit 0
fi

if [ ! -x "$STAGEB_WRAPPER" ]; then
  log_error "selfhost Stage-B wrapper missing/executable: $STAGEB_WRAPPER"
  exit 2
fi

if [ ! -f "$LIST_FILE" ]; then
  missing_log="$LOG_DIR/phase29bq_selfhost_list_missing.log"
  : > "$missing_log"
  echo "[error] list not found: $LIST_FILE" >> "$missing_log"
  log_error "selfhost gate list not found: $LIST_FILE"
  echo "LOG: $missing_log"
  exit 2
fi

selfhost_exit_code_allowed() {
  local exit_code="$1"
  local allowed_codes="$2"
  local code

  for code in $allowed_codes; do
    if [ "$exit_code" -eq "$code" ]; then
      return 0
    fi
  done

  return 1
}

emit_failure_summary_from_log() {
  local log_path="$1"

  if [ ! -f "$log_path" ]; then
    return 0
  fi

  echo "[diag/selfhost] summary_begin log=$log_path"
  echo "[diag/selfhost] env=NYASH_DEV=${NYASH_DEV:-} NYASH_OPERATOR_BOX_ALL=${NYASH_OPERATOR_BOX_ALL:-} NYASH_OPERATOR_BOX_STRINGIFY=${NYASH_OPERATOR_BOX_STRINGIFY:-} NYASH_OPERATOR_BOX_COMPARE=${NYASH_OPERATOR_BOX_COMPARE:-} NYASH_OPERATOR_BOX_ADD=${NYASH_OPERATOR_BOX_ADD:-} NYASH_OPERATOR_BOX_COMPARE_ADOPT=${NYASH_OPERATOR_BOX_COMPARE_ADOPT:-} NYASH_OPERATOR_BOX_ADD_ADOPT=${NYASH_OPERATOR_BOX_ADD_ADOPT:-} NYASH_BUILDER_OPERATOR_BOX_ALL_CALL=${NYASH_BUILDER_OPERATOR_BOX_ALL_CALL:-} NYASH_BUILDER_OPERATOR_BOX_ADD_CALL=${NYASH_BUILDER_OPERATOR_BOX_ADD_CALL:-} NYASH_DISABLE_PLUGINS=${NYASH_DISABLE_PLUGINS:-} NYASH_VM_HAKO_PREFER_STRICT_DEV=${NYASH_VM_HAKO_PREFER_STRICT_DEV:-} HAKO_JOINIR_STRICT=${HAKO_JOINIR_STRICT:-} HAKO_JOINIR_PLANNER_REQUIRED=${HAKO_JOINIR_PLANNER_REQUIRED:-} HAKO_JOINIR_DEBUG=${HAKO_JOINIR_DEBUG:-} NYASH_FEATURES=${NYASH_FEATURES:-} NYASH_USING_AST=${NYASH_USING_AST:-}"

  local stageb_secs
  stageb_secs="$(grep -E '^\[diag/selfhost\] stageb_secs=' "$log_path" | tail -n 1 || true)"
  if [ -n "$stageb_secs" ]; then
    echo "$stageb_secs"
  fi

  local run_secs
  run_secs="$(grep -E '^\[diag/selfhost\] run_secs=' "$log_path" | tail -n 1 || true)"
  if [ -n "$run_secs" ]; then
    echo "$run_secs"
  fi

  local first_error
  first_error="$(rg -n "\\[ERROR\\]" "$log_path" 2>/dev/null | head -n 1 || true)"
  if [ -n "$first_error" ]; then
    echo "[diag/selfhost] first_error=$first_error"
  fi

  local last_reject_detail
  last_reject_detail="$(rg -n "\\[plan/reject_detail\\]" "$log_path" 2>/dev/null | tail -n 1 || true)"
  if [ -n "$last_reject_detail" ]; then
    echo "[diag/selfhost] last_reject_detail=$last_reject_detail"
  fi

  local last_reject
  last_reject="$(rg -n "\\[plan/reject\\]" "$log_path" 2>/dev/null | tail -n 1 || true)"
  if [ -n "$last_reject" ]; then
    echo "[diag/selfhost] last_reject=$last_reject"
  fi

  local stageb_root
  stageb_root="$(rg -n "StepTree root for 'StageBBodyExtractorBox\\.build_body_src/2'" "$log_path" 2>/dev/null | head -n 1 || true)"
  if [ -n "$stageb_root" ]; then
    echo "[diag/selfhost] steptree_root=$(echo "$stageb_root" | cut -c1-220)"
  fi

  echo "[diag/selfhost] summary_end"
}

case_matches_filter() {
  local fixture="$1"
  local planner_tag="$2"
  local reason="$3"

  if [ -z "$CASE_FILTER" ]; then
    return 0
  fi

  # Live contract note:
  # `SMOKES_SELFHOST_FILTER` still matches fixture + planner_tag + reason.
  # Semantic route substrings are preferred, but exact historical tokens may
  # remain live while they survive in the `reason` column of the subset TSV.
  local haystack="$fixture $planner_tag $reason"
  [[ "$haystack" == *"$CASE_FILTER"* ]]
}

count_selected_cases() {
  local count=0
  local fixture expected allowed_rc planner_tag reason

  while IFS=$'\t' read -r fixture expected allowed_rc planner_tag reason; do
    if [ -z "$fixture" ] || [[ "$fixture" == \#* ]]; then
      continue
    fi

    fixture=${fixture//$'\r'/}
    planner_tag=${planner_tag//$'\r'/}
    reason=${reason//$'\r'/}

    if ! case_matches_filter "$fixture" "$planner_tag" "$reason"; then
      continue
    fi

    count=$((count + 1))
    if [ "$MAX_CASES" -gt 0 ] && [ "$count" -ge "$MAX_CASES" ]; then
      break
    fi
  done < "$LIST_FILE"

  echo "$count"
}

TOTAL_STAGEB_SECS=0
TOTAL_RUN_SECS=0
PARALLEL_EXECUTED=0

run_selfhost_case() {
  local fixture="$1"
  local expected="$2"
  local allowed_rc="$3"
  local planner_tag="$4"
  local gate_name="$5"
  local case_name="unknown"
  local log_path

  if [ -n "$fixture" ]; then
    case_name="$(basename "$fixture")"
  fi

  log_path="$LOG_DIR/phase29bq_selfhost_${case_name}.log"
  : > "$log_path"
  echo "[setup] fixture=$fixture" >> "$log_path"

  if [ -z "$fixture" ] || [ -z "$planner_tag" ]; then
    log_error "$gate_name: missing fixture or planner_tag"
    echo "[error] missing fixture or planner_tag" >> "$log_path"
    echo "LOG: $log_path"
    return 1
  fi

  if [ -z "$allowed_rc" ]; then
    allowed_rc="0"
  fi

  if [[ "$fixture" != /* ]]; then
    fixture="$NYASH_ROOT/$fixture"
  fi

  if [ ! -f "$fixture" ]; then
    log_error "$gate_name: fixture not found: $fixture"
    echo "[error] fixture not found: $fixture" >> "$log_path"
    echo "LOG: $log_path"
    return 1
  fi

  local raw_log
  local json_out
  local run_out
  local run_log
  raw_log="$(mktemp /tmp/phase29bq_stageb_raw.XXXXXX.log)"
  json_out="$(mktemp /tmp/phase29bq_stageb.XXXXXX.json)"
  run_out="$(mktemp /tmp/phase29bq_run.XXXXXX.out)"
  run_log="$(mktemp /tmp/phase29bq_run.XXXXXX.log)"
  echo "[stageb] fixture=$fixture" >> "$log_path"

  local stageb_rc=0
  set +e
  local stageb_start=$SECONDS
  # Phase 29x X22: keep Stage-B compiler route on Rust VM core lane even under strict/dev.
  run_with_vm_route_pin env \
    SELFHOST_ROUTE_ID="SH-GATE-STAGEB" \
    SMOKES_SELFHOST_STAGEB_TIMEOUT_SECS="$STAGEB_TIMEOUT_SECS" \
    HAKO_STAGEB_MODULES_LIST="$HAKO_STAGEB_MODULES_LIST" \
    HAKO_STAGEB_MODULE_ROOTS_LIST="$HAKO_STAGEB_MODULE_ROOTS_LIST" \
    NYASH_FEATURES="${NYASH_FEATURES:-stage3,no-try-compat}" \
    NYASH_QUIET=0 HAKO_QUIET=0 NYASH_CLI_VERBOSE=0 \
    "$STAGEB_WRAPPER" --source-file "$fixture" \
    > "$raw_log" 2>&1
  stageb_rc=$?
  local stageb_end=$SECONDS
  set -e

  local stageb_secs=$((stageb_end - stageb_start))
  TOTAL_STAGEB_SECS=$((TOTAL_STAGEB_SECS + stageb_secs))
  echo "[diag/selfhost] stageb_secs=$stageb_secs timeout_secs=${STAGEB_TIMEOUT_SECS}" >> "$log_path"
  cat "$raw_log" >> "$log_path"

  if [ "$stageb_rc" -eq 124 ]; then
    log_error "$gate_name:$case_name stage-b timed out (> ${STAGEB_TIMEOUT_SECS}s)"
    emit_failure_summary_from_log "$log_path"
    echo "LOG: $log_path"
    return 1
  fi

  if [ "$stageb_rc" -ne 0 ]; then
    log_error "$gate_name:$case_name stage-b failed (rc=$stageb_rc)"
    emit_failure_summary_from_log "$log_path"
    echo "LOG: $log_path"
    return 1
  fi

  if ! grep -qF "$planner_tag" "$raw_log"; then
    log_error "$gate_name:$case_name missing planner tag ($planner_tag)"
    emit_failure_summary_from_log "$log_path"
    echo "LOG: $log_path"
    return 1
  fi

  if ! awk '(/"version":0/ && /"kind":"Program"/){print;found=1;exit} END{exit(found?0:1)}' \
    "$raw_log" > "$json_out"; then
    log_error "$gate_name:$case_name failed to extract Program(JSON v0)"
    emit_failure_summary_from_log "$log_path"
    echo "LOG: $log_path"
    return 1
  fi

  echo "[run] json=$json_out" >> "$log_path"

  set +e
  local run_start=$SECONDS
  echo "[selfhost/route] id=SH-JSONRUN mode=json-run fixture=$(basename "$fixture") json=$(basename "$json_out")" > "$run_log"
  # JSON execute lane also uses Rust VM core route to avoid vm-hako subset mismatch during selfhost gate.
  run_with_vm_route_pin env \
    NYASH_DISABLE_PLUGINS=1 \
    NYASH_DEV=0 \
    NYASH_OPERATOR_BOX_ALL=0 \
    NYASH_OPERATOR_BOX_STRINGIFY=0 \
    NYASH_OPERATOR_BOX_COMPARE=0 \
    NYASH_OPERATOR_BOX_ADD=0 \
    NYASH_OPERATOR_BOX_COMPARE_ADOPT=0 \
    NYASH_OPERATOR_BOX_ADD_ADOPT=0 \
    NYASH_BUILDER_OPERATOR_BOX_ALL_CALL=0 \
    NYASH_BUILDER_OPERATOR_BOX_ADD_CALL=0 \
    HAKO_STAGEB_MODULES_LIST="$HAKO_STAGEB_MODULES_LIST" \
    HAKO_STAGEB_MODULE_ROOTS_LIST="$HAKO_STAGEB_MODULE_ROOTS_LIST" \
    NYASH_ALLOW_USING_FILE=1 \
    HAKO_ALLOW_USING_FILE=1 \
    NYASH_USING_AST=1 \
    NYASH_FEATURES="${NYASH_FEATURES:-stage3,no-try-compat}" \
    NYASH_PARSER_ALLOW_SEMICOLON=1 \
    NYASH_VARMAP_GUARD_STRICT=0 \
    NYASH_BLOCK_SCHEDULE_VERIFY=0 \
    NYASH_TRY_RESULT_MODE=1 \
    NYASH_QUIET=0 HAKO_QUIET=0 NYASH_CLI_VERBOSE=0 \
    HAKO_JOINIR_STRICT=1 \
    HAKO_JOINIR_PLANNER_REQUIRED=1 \
    timeout "$JSON_TIMEOUT_SECS" \
    "$NYASH_BIN" --debug-fuel unlimited --json-file "$json_out" \
    > "$run_out" 2>> "$run_log"
  local run_rc=$?
  local run_end=$SECONDS
  set -e

  local run_secs=$((run_end - run_start))
  TOTAL_RUN_SECS=$((TOTAL_RUN_SECS + run_secs))
  echo "[diag/selfhost] run_secs=$run_secs timeout_secs=${JSON_TIMEOUT_SECS}" >> "$log_path"
  echo "[stdout]" >> "$log_path"
  cat "$run_out" >> "$log_path"
  echo "[stderr]" >> "$log_path"
  cat "$run_log" >> "$log_path"

  if [ "$run_rc" -eq 124 ]; then
    log_error "$gate_name:$case_name run timed out (> ${JSON_TIMEOUT_SECS}s)"
    emit_failure_summary_from_log "$log_path"
    echo "LOG: $log_path"
    return 1
  fi

  if ! selfhost_exit_code_allowed "$run_rc" "$allowed_rc"; then
    log_error "$gate_name:$case_name expected exit code(s) $allowed_rc, got $run_rc"
    emit_failure_summary_from_log "$log_path"
    echo "LOG: $log_path"
    return 1
  fi

  local output_clean
  output_clean=$(cat "$run_out" | filter_noise | grep -v '^\[plugins\]' | grep -v '^\[WARN\] \[plugin/init\]' || true)

  if ! compare_outputs "$expected" "$output_clean" "$gate_name:$case_name"; then
    emit_failure_summary_from_log "$log_path"
    echo "LOG: $log_path"
    return 1
  fi

  log_success "$gate_name:$case_name PASS (exit=$run_rc)"
  return 0
}

extract_diag_secs_from_log() {
  local log_path="$1"
  local key="$2"
  local value

  value="$(grep -E "^\\[diag/selfhost\\] ${key}=" "$log_path" 2>/dev/null | tail -n 1 | sed -E "s/.*${key}=([0-9]+).*/\\1/" || true)"
  if [[ "$value" =~ ^[0-9]+$ ]]; then
    echo "$value"
  else
    echo "0"
  fi
}

run_selfhost_case_worker() {
  local index="$1"
  local fixture="$2"
  local expected="$3"
  local allowed_rc="$4"
  local planner_tag="$5"
  local gate_name="$6"
  local result_file="$7"
  local rc=0
  local case_name
  local log_path
  local stageb_secs
  local run_secs

  run_selfhost_case "$fixture" "$expected" "$allowed_rc" "$planner_tag" "$gate_name" || rc=$?

  case_name="$(basename "$fixture")"
  log_path="$LOG_DIR/phase29bq_selfhost_${case_name}.log"
  stageb_secs="$(extract_diag_secs_from_log "$log_path" "stageb_secs")"
  run_secs="$(extract_diag_secs_from_log "$log_path" "run_secs")"

  printf "%s\t%s\t%s\t%s\t%s\t%s\n" \
    "$index" "$rc" "$fixture" "$stageb_secs" "$run_secs" "$log_path" > "$result_file"
  return 0
}

run_master_list_parallel() {
  local gate_name="$1"
  local selected="$2"
  local fixture expected allowed_rc planner_tag reason
  local launched=0
  local running=0
  local result_dir
  local failed=0
  local first_failed_case=""
  local first_failed_log=""

  result_dir="$(mktemp -d /tmp/phase29bq_selfhost_results.XXXXXX)"
  PARALLEL_EXECUTED=0
  TOTAL_STAGEB_SECS=0
  TOTAL_RUN_SECS=0

  while IFS=$'\t' read -r fixture expected allowed_rc planner_tag reason; do
    if [ -z "$fixture" ] || [[ "$fixture" == \#* ]]; then
      continue
    fi

    fixture=${fixture//$'\r'/}
    expected=${expected//$'\r'/}
    allowed_rc=${allowed_rc//$'\r'/}
    planner_tag=${planner_tag//$'\r'/}
    reason=${reason//$'\r'/}

    if ! case_matches_filter "$fixture" "$planner_tag" "$reason"; then
      continue
    fi

    if [ "$MAX_CASES" -gt 0 ] && [ "$launched" -ge "$MAX_CASES" ]; then
      break
    fi

    if [ "$expected" = "__EMPTY__" ]; then
      expected=""
    fi

    launched=$((launched + 1))
    if [ "$SHOW_PROGRESS" -eq 1 ]; then
      log_info "$gate_name: case $launched/$selected $(basename "$fixture")"
    fi

    run_selfhost_case_worker \
      "$launched" "$fixture" "$expected" "$allowed_rc" "$planner_tag" "$gate_name" \
      "$result_dir/$launched.tsv" &

    running=$((running + 1))
    if [ "$running" -ge "$JOBS" ]; then
      if ! wait -n; then
        :
      fi
      running=$((running - 1))
    fi
  done < "$LIST_FILE"

  while [ "$running" -gt 0 ]; do
    if ! wait -n; then
      :
    fi
    running=$((running - 1))
  done

  local index=1
  while [ "$index" -le "$launched" ]; do
    local result_file="$result_dir/$index.tsv"
    if [ ! -f "$result_file" ]; then
      failed=1
      if [ -z "$first_failed_case" ]; then
        first_failed_case="case#$index"
      fi
      index=$((index + 1))
      continue
    fi

    local rc fixture_path stageb_secs run_secs log_path
    IFS=$'\t' read -r _ rc fixture_path stageb_secs run_secs log_path < "$result_file"
    PARALLEL_EXECUTED=$((PARALLEL_EXECUTED + 1))
    TOTAL_STAGEB_SECS=$((TOTAL_STAGEB_SECS + stageb_secs))
    TOTAL_RUN_SECS=$((TOTAL_RUN_SECS + run_secs))

    if [ "$rc" -ne 0 ] && [ "$failed" -eq 0 ]; then
      failed=1
      first_failed_case="$(basename "$fixture_path")"
      first_failed_log="$log_path"
    fi

    index=$((index + 1))
  done

  rm -rf "$result_dir"

  if [ "$failed" -ne 0 ]; then
    log_error "$gate_name: parallel failure detected (first=${first_failed_case:-unknown})"
    if [ -n "$first_failed_log" ] && [ -f "$first_failed_log" ]; then
      emit_failure_summary_from_log "$first_failed_log"
      echo "LOG: $first_failed_log"
    fi
    return 1
  fi

  return 0
}

run_master_list() {
  local gate_name="phase29bq_selfhost_planner_required_dev_gate_vm"
  local failed=0
  local executed=0
  local fixture expected allowed_rc planner_tag reason
  local selected
  local summary_total_secs
  local summary_avg_case

  selected="$(count_selected_cases)"
  if [ "$selected" -eq 0 ]; then
    log_warn "$gate_name: SKIP (no runnable entries in list: $LIST_FILE, filter=${CASE_FILTER:-none})"
    return 0
  fi

  if [ "$JOBS" -gt 1 ]; then
    log_info "$gate_name: parallel mode enabled (jobs=$JOBS, selected=$selected)"
    if ! run_master_list_parallel "$gate_name" "$selected"; then
      failed=1
    fi
    executed="$PARALLEL_EXECUTED"
  else
    while IFS=$'\t' read -r fixture expected allowed_rc planner_tag reason; do
      if [ -z "$fixture" ] || [[ "$fixture" == \#* ]]; then
        continue
      fi

      fixture=${fixture//$'\r'/}
      expected=${expected//$'\r'/}
      allowed_rc=${allowed_rc//$'\r'/}
      planner_tag=${planner_tag//$'\r'/}
      reason=${reason//$'\r'/}

      if ! case_matches_filter "$fixture" "$planner_tag" "$reason"; then
        continue
      fi

      if [ "$MAX_CASES" -gt 0 ] && [ "$executed" -ge "$MAX_CASES" ]; then
        break
      fi

      if [ "$expected" = "__EMPTY__" ]; then
        expected=""
      fi

      if [ "$SHOW_PROGRESS" -eq 1 ]; then
        log_info "$gate_name: case $((executed + 1))/$selected $(basename "$fixture")"
      fi

      if ! run_selfhost_case "$fixture" "$expected" "$allowed_rc" "$planner_tag" "$gate_name"; then
        failed=1
        break
      fi
      executed=$((executed + 1))
    done < "$LIST_FILE"
  fi

  summary_total_secs=$((TOTAL_STAGEB_SECS + TOTAL_RUN_SECS))
  if [ "$executed" -gt 0 ]; then
    summary_avg_case="$(awk -v total="$summary_total_secs" -v n="$executed" 'BEGIN { printf "%.2f", total / n }')"
  else
    summary_avg_case="0.00"
  fi
  log_info "[diag/selfhost] gate_summary status=$([ "$failed" -eq 0 ] && echo PASS || echo FAIL) cases=$executed/$selected stageb_total_secs=$TOTAL_STAGEB_SECS run_total_secs=$TOTAL_RUN_SECS total_secs=$summary_total_secs avg_case_secs=$summary_avg_case timeout_stageb=$STAGEB_TIMEOUT_SECS timeout_json=$JSON_TIMEOUT_SECS filter=${CASE_FILTER:-none} limit=$MAX_CASES"

  if [ "$failed" -ne 0 ]; then
    return 1
  fi

  log_success "$gate_name: PASS"
  return 0
}

run_master_list
