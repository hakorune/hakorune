#!/bin/bash
# phase29bq_selfhost_stageb_funcscanner_box_from_min_vm.sh
# Pin Stage-B FuncScanner delegated box header:
#   static box Child from Parent { ... }
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../../../../.." && pwd)"
source "$ROOT_DIR/smokes/v2/lib/test_runner.sh"
require_env || exit 2

ROUTE_SCRIPT="$NYASH_ROOT/tools/selfhost/run_stageb_compiler_vm.sh"
FIXTURE="${1:-$NYASH_ROOT/apps/tests/phase29bq_selfhost_funcscanner_box_from_min.hako}"
TIMEOUT_SECS="${SMOKES_SELFHOST_STAGEB_TIMEOUT_SECS:-${RUN_TIMEOUT_SECS:-20}}"

if ! [[ "$TIMEOUT_SECS" =~ ^[0-9]+$ ]]; then
  log_error "timeout must be integer: $TIMEOUT_SECS"
  exit 2
fi

if [[ "$FIXTURE" != /* ]]; then
  FIXTURE="$NYASH_ROOT/$FIXTURE"
fi

if [ ! -x "$ROUTE_SCRIPT" ]; then
  log_error "route script missing/executable: $ROUTE_SCRIPT"
  exit 2
fi

if [ ! -f "$FIXTURE" ]; then
  log_error "fixture not found: $FIXTURE"
  exit 2
fi

log_file="$(mktemp /tmp/phase29bq_stageb_funcscanner_box_from.XXXXXX.log)"
json_file="$(mktemp /tmp/phase29bq_stageb_funcscanner_box_from.XXXXXX.json)"
cleanup() {
  rm -f "$log_file" "$json_file"
}
trap cleanup EXIT

set +e
NYASH_SELFHOST_STAGEB_PROOF_ONLY=1 \
SELFHOST_ROUTE_ID="SH-GATE-STAGEB-FUNCSCANNER-FROM" \
  "$ROUTE_SCRIPT" --source-file "$FIXTURE" --timeout-secs "$TIMEOUT_SECS" >"$log_file" 2>&1
rc=$?
set -e

if [ "$rc" -ne 0 ]; then
  log_error "stage-b route failed (rc=$rc)"
  echo "LOG: $log_file"
  exit 1
fi

if ! awk '(/"version":0/ && /"kind":"Program"/){print;found=1;exit} END{exit(found?0:1)}' \
  "$log_file" > "$json_file"; then
  log_error "Program(JSON v0) line not found"
  echo "LOG: $log_file"
  exit 1
fi

if ! rg -q '"defs":\[' "$json_file"; then
  log_error "defs field missing in Program(JSON v0)"
  echo "JSON: $json_file"
  exit 1
fi

if ! rg -q '"box":"Parent"' "$json_file" || ! rg -q '"name":"ping"' "$json_file"; then
  log_error "Parent.ping definition missing from defs"
  echo "JSON: $json_file"
  exit 1
fi

if ! rg -q '"box":"Child"' "$json_file" || ! rg -q '"name":"run"' "$json_file"; then
  log_error "Child.run definition missing from defs"
  echo "JSON: $json_file"
  exit 1
fi

log_success "phase29bq_selfhost_stageb_funcscanner_box_from_min_vm: PASS ($(basename "$FIXTURE"))"
