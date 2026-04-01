#!/bin/bash
# phase29bq_selfhost_stageb_route_parity_smoke_vm.sh
# Compare Stage-B output between wrapper route and direct invocation route.
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../../../../.." && pwd)"
source "$ROOT_DIR/smokes/v2/lib/test_runner.sh"
require_env || exit 2

WRAPPER="$NYASH_ROOT/tools/selfhost/run.sh"
COMPILER="$NYASH_ROOT/lang/src/compiler/entry/compiler_stageb.hako"
FIXTURE="${1:-$NYASH_ROOT/apps/tests/selfhost_cleanup_only_min.hako}"
TIMEOUT_SECS="${SMOKES_SELFHOST_STAGEB_TIMEOUT_SECS:-${RUN_TIMEOUT_SECS:-20}}"

if ! [[ "$TIMEOUT_SECS" =~ ^[0-9]+$ ]]; then
  log_error "timeout must be integer: $TIMEOUT_SECS"
  exit 2
fi

if [ ! -x "$WRAPPER" ]; then
  log_error "wrapper not found/executable: $WRAPPER"
  exit 2
fi

if [ ! -x "$NYASH_BIN" ]; then
  log_error "nyash binary not found/executable: $NYASH_BIN"
  exit 2
fi

if [ ! -f "$COMPILER" ]; then
  log_error "compiler entry missing: $COMPILER"
  exit 2
fi

if [[ "$FIXTURE" != /* ]]; then
  FIXTURE="$NYASH_ROOT/$FIXTURE"
fi

if [ ! -f "$FIXTURE" ]; then
  log_error "fixture not found: $FIXTURE"
  exit 2
fi

HAKO_STAGEB_MODULES_LIST="$(collect_stageb_modules_list "$NYASH_ROOT")"
HAKO_STAGEB_MODULE_ROOTS_LIST="$(collect_stageb_module_roots_list "$NYASH_ROOT")"

wrapper_log="$(mktemp /tmp/phase29bq_stageb_wrapper.XXXXXX.log)"
direct_log="$(mktemp /tmp/phase29bq_stageb_direct.XXXXXX.log)"
wrapper_json="$(mktemp /tmp/phase29bq_stageb_wrapper.XXXXXX.json)"
direct_json="$(mktemp /tmp/phase29bq_stageb_direct.XXXXXX.json)"
cleanup() {
  rm -f "$wrapper_log" "$direct_log" "$wrapper_json" "$direct_json"
}
trap cleanup EXIT

set +e
SELFHOST_ROUTE_ID="SH-PARITY-WRAPPER" \
  SMOKES_SELFHOST_STAGEB_TIMEOUT_SECS="$TIMEOUT_SECS" \
  HAKO_STAGEB_MODULES_LIST="$HAKO_STAGEB_MODULES_LIST" \
  HAKO_STAGEB_MODULE_ROOTS_LIST="$HAKO_STAGEB_MODULE_ROOTS_LIST" \
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
  HAKO_JOINIR_STRICT=1 \
  HAKO_JOINIR_PLANNER_REQUIRED=1 \
  NYASH_ALLOW_USING_FILE=1 \
  HAKO_ALLOW_USING_FILE=1 \
  NYASH_USING_AST=1 \
  NYASH_FEATURES="${NYASH_FEATURES:-stage3,no-try-compat}" \
  NYASH_PARSER_ALLOW_SEMICOLON=1 \
  NYASH_VARMAP_GUARD_STRICT=0 \
  NYASH_BLOCK_SCHEDULE_VERIFY=0 \
  NYASH_QUIET=0 HAKO_QUIET=0 NYASH_CLI_VERBOSE=0 \
  "$WRAPPER" --direct --source-file "$FIXTURE" --timeout-secs "$TIMEOUT_SECS" --route-id "SH-PARITY-WRAPPER" \
  > "$wrapper_log" 2>&1
wrapper_rc=$?

HAKO_SRC="$(cat "$FIXTURE")" \
  NYASH_DISABLE_PLUGINS=1 \
  NYASH_VM_HAKO_PREFER_STRICT_DEV=0 \
  NYASH_DEV=0 \
  NYASH_OPERATOR_BOX_ALL=0 \
  NYASH_OPERATOR_BOX_STRINGIFY=0 \
  NYASH_OPERATOR_BOX_COMPARE=0 \
  NYASH_OPERATOR_BOX_ADD=0 \
  NYASH_OPERATOR_BOX_COMPARE_ADOPT=0 \
  NYASH_OPERATOR_BOX_ADD_ADOPT=0 \
  NYASH_BUILDER_OPERATOR_BOX_ALL_CALL=0 \
  NYASH_BUILDER_OPERATOR_BOX_ADD_CALL=0 \
  HAKO_JOINIR_STRICT=1 \
  HAKO_JOINIR_PLANNER_REQUIRED=1 \
  NYASH_ALLOW_USING_FILE=1 \
  HAKO_ALLOW_USING_FILE=1 \
  NYASH_USING_AST=1 \
  NYASH_FEATURES="${NYASH_FEATURES:-stage3,no-try-compat}" \
  NYASH_PARSER_ALLOW_SEMICOLON=1 \
  NYASH_VARMAP_GUARD_STRICT=0 \
  NYASH_BLOCK_SCHEDULE_VERIFY=0 \
  HAKO_STAGEB_MODULES_LIST="$HAKO_STAGEB_MODULES_LIST" \
  HAKO_STAGEB_MODULE_ROOTS_LIST="$HAKO_STAGEB_MODULE_ROOTS_LIST" \
  NYASH_QUIET=0 HAKO_QUIET=0 NYASH_CLI_VERBOSE=0 \
  timeout "$TIMEOUT_SECS" \
  "$NYASH_BIN" --backend vm "$COMPILER" -- --source "$(cat "$FIXTURE")" \
  > "$direct_log" 2>&1
direct_rc=$?
set -e

if [ "$wrapper_rc" -ne "$direct_rc" ]; then
  log_error "route parity rc mismatch: wrapper=$wrapper_rc direct=$direct_rc"
  echo "WRAPPER_LOG: $wrapper_log"
  echo "DIRECT_LOG:  $direct_log"
  exit 1
fi

if [ "$wrapper_rc" -ne 0 ]; then
  log_error "route parity failed: both routes exited non-zero rc=$wrapper_rc"
  echo "WRAPPER_LOG: $wrapper_log"
  echo "DIRECT_LOG:  $direct_log"
  exit 1
fi

if ! awk '(/"version":0/ && /"kind":"Program"/){print;found=1;exit} END{exit(found?0:1)}' \
  "$wrapper_log" > "$wrapper_json"; then
  log_error "wrapper route did not emit Program(JSON v0)"
  echo "WRAPPER_LOG: $wrapper_log"
  exit 1
fi

if ! awk '(/"version":0/ && /"kind":"Program"/){print;found=1;exit} END{exit(found?0:1)}' \
  "$direct_log" > "$direct_json"; then
  log_error "direct route did not emit Program(JSON v0)"
  echo "DIRECT_LOG: $direct_log"
  exit 1
fi

if ! cmp -s "$wrapper_json" "$direct_json"; then
  log_error "route parity mismatch: Program(JSON v0) differs"
  diff -u "$direct_json" "$wrapper_json" || true
  echo "WRAPPER_LOG: $wrapper_log"
  echo "DIRECT_LOG:  $direct_log"
  exit 1
fi

log_success "phase29bq_selfhost_stageb_route_parity_smoke_vm: PASS ($(basename "$FIXTURE"))"
