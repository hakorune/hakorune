#!/bin/bash
# phase29bq_selfhost_planner_required_dev_gate_vm.sh - selfhost entry gate (subset list)

set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../../../../.." && pwd)"
LOG_DIR="/tmp"
DEFAULT_LIST_FILE="$ROOT_DIR/smokes/v2/profiles/integration/selfhost/planner_required_selfhost_subset.tsv"
LIST_FILE="${SMOKES_SELFHOST_LIST:-$DEFAULT_LIST_FILE}"
RUN_TIMEOUT_SECS="${RUN_TIMEOUT_SECS:-20}"

source "$ROOT_DIR/smokes/v2/lib/test_runner.sh"
require_env || exit 2

COMPILER="$NYASH_ROOT/lang/src/compiler/entry/compiler.hako"

if [ "${SMOKES_ENABLE_SELFHOST:-0}" != "1" ]; then
  log_warn "[SKIP] selfhost gate disabled (set SMOKES_ENABLE_SELFHOST=1)"
  exit 0
fi

if [ ! -f "$COMPILER" ]; then
  log_warn "[SKIP] compiler.hako missing: $COMPILER"
  exit 0
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
  local run_log
  raw_log="$(mktemp /tmp/phase29bq_stageb_raw.XXXXXX.log)"
  json_out="$(mktemp /tmp/phase29bq_stageb.XXXXXX.json)"
  run_log="$(mktemp /tmp/phase29bq_run.XXXXXX.log)"
  echo "[stageb] fixture=$fixture" >> "$log_path"

  local stageb_rc=0
  set +e
  HAKO_SRC="$(cat "$fixture")" \
    NYASH_DISABLE_PLUGINS=1 \
    HAKO_JOINIR_STRICT=1 \
    HAKO_JOINIR_PLANNER_REQUIRED=1 \
    NYASH_ALLOW_USING_FILE=1 \
    HAKO_ALLOW_USING_FILE=1 \
    NYASH_USING_AST=1 \
    NYASH_FEATURES=stage3 \
    NYASH_PARSER_ALLOW_SEMICOLON=1 \
    NYASH_VARMAP_GUARD_STRICT=0 \
    NYASH_BLOCK_SCHEDULE_VERIFY=0 \
    NYASH_QUIET=0 HAKO_QUIET=0 NYASH_CLI_VERBOSE=0 \
    timeout "$RUN_TIMEOUT_SECS" \
    "$NYASH_BIN" --backend vm "$COMPILER" -- --stage-b --stage3 \
    > "$raw_log" 2>&1
  stageb_rc=$?
  set -e

  cat "$raw_log" >> "$log_path"

  if [ "$stageb_rc" -eq 124 ]; then
    log_error "$gate_name:$case_name stage-b timed out (> ${RUN_TIMEOUT_SECS}s)"
    echo "LOG: $log_path"
    return 1
  fi

  if [ "$stageb_rc" -ne 0 ]; then
    log_error "$gate_name:$case_name stage-b failed (rc=$stageb_rc)"
    echo "LOG: $log_path"
    return 1
  fi

  if ! grep -qF "$planner_tag" "$raw_log"; then
    log_error "$gate_name:$case_name missing planner tag ($planner_tag)"
    echo "LOG: $log_path"
    return 1
  fi

  if ! awk '(/"version":0/ && /"kind":"Program"/){print;found=1;exit} END{exit(found?0:1)}' \
    "$raw_log" > "$json_out"; then
    log_error "$gate_name:$case_name failed to extract Program(JSON v0)"
    echo "LOG: $log_path"
    return 1
  fi

  echo "[run] json=$json_out" >> "$log_path"

  set +e
  NYASH_DISABLE_PLUGINS=1 \
    NYASH_ALLOW_USING_FILE=1 \
    HAKO_ALLOW_USING_FILE=1 \
    timeout "$RUN_TIMEOUT_SECS" \
    "$NYASH_BIN" --json-file "$json_out" \
    > "$run_log" 2>&1
  local run_rc=$?
  set -e

  cat "$run_log" >> "$log_path"

  if [ "$run_rc" -eq 124 ]; then
    log_error "$gate_name:$case_name run timed out (> ${RUN_TIMEOUT_SECS}s)"
    echo "LOG: $log_path"
    return 1
  fi

  if ! selfhost_exit_code_allowed "$run_rc" "$allowed_rc"; then
    log_error "$gate_name:$case_name expected exit code(s) $allowed_rc, got $run_rc"
    echo "LOG: $log_path"
    return 1
  fi

  local output_clean
  output_clean=$(cat "$run_log" | filter_noise | grep -v '^\[plugins\]' | grep -v '^\[WARN\] \[plugin/init\]' || true)

  if ! compare_outputs "$expected" "$output_clean" "$gate_name:$case_name"; then
    echo "LOG: $log_path"
    return 1
  fi

  log_success "$gate_name:$case_name PASS (exit=$run_rc)"
  return 0
}

run_master_list() {
  local gate_name="phase29bq_selfhost_planner_required_dev_gate_vm"
  local failed=0
  local executed=0
  local fixture expected allowed_rc planner_tag _rest

  while IFS=$'\t' read -r fixture expected allowed_rc planner_tag _rest; do
    if [ -z "$fixture" ] || [[ "$fixture" == \#* ]]; then
      continue
    fi

    fixture=${fixture//$'\r'/}
    expected=${expected//$'\r'/}
    allowed_rc=${allowed_rc//$'\r'/}
    planner_tag=${planner_tag//$'\r'/}

    if [ "$expected" = "__EMPTY__" ]; then
      expected=""
    fi

    if ! run_selfhost_case "$fixture" "$expected" "$allowed_rc" "$planner_tag" "$gate_name"; then
      failed=1
      break
    fi
    executed=$((executed + 1))
  done < "$LIST_FILE"

  if [ "$failed" -ne 0 ]; then
    return 1
  fi

  if [ "$executed" -eq 0 ]; then
    log_warn "$gate_name: SKIP (no runnable entries in list: $LIST_FILE)"
    return 0
  fi

  log_success "$gate_name: PASS"
  return 0
}

run_master_list
