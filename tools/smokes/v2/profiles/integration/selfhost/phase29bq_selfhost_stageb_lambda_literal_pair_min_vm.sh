#!/bin/bash
# phase29bq_selfhost_stageb_lambda_literal_pair_min_vm.sh
# Pin Stage-B legacy lambda pair in method defs:
#   return fn(x) { ... }
# -> Return(Call "fn") + Expr(BlockExpr ...)
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../../../../.." && pwd)"
source "$ROOT_DIR/smokes/v2/lib/test_runner.sh"
require_env || exit 2

ROUTE_SCRIPT="$NYASH_ROOT/tools/selfhost/run_stageb_compiler_vm.sh"
FIXTURE="${1:-$NYASH_ROOT/apps/tests/phase29bq_selfhost_funcscanner_lambda_literal_min.hako}"
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

log_file="$(mktemp /tmp/phase29bq_stageb_lambda_pair.XXXXXX.log)"
json_file="$(mktemp /tmp/phase29bq_stageb_lambda_pair.XXXXXX.json)"
cleanup() {
  rm -f "$log_file" "$json_file"
}
trap cleanup EXIT

set +e
NYASH_SELFHOST_STAGEB_PROOF_ONLY=1 \
SELFHOST_ROUTE_ID="SH-GATE-STAGEB-LAMBDA-PAIR" \
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

if ! rg -q '"box":"Helper"' "$json_file" || ! rg -q '"name":"make"' "$json_file"; then
  log_error "Helper.make definition missing from defs"
  echo "JSON: $json_file"
  exit 1
fi

if ! rg -q '"type":"Return","expr":\{"type":"Call","name":"fn"' "$json_file"; then
  log_error "legacy lambda call-head shape missing (Return(Call fn ...))"
  echo "JSON: $json_file"
  exit 1
fi

if ! rg -q '"type":"Expr","expr":\{"type":"BlockExpr"' "$json_file"; then
  log_error "legacy lambda block-tail shape missing (Expr(BlockExpr ...))"
  echo "JSON: $json_file"
  exit 1
fi

log_success "phase29bq_selfhost_stageb_lambda_literal_pair_min_vm: PASS ($(basename "$FIXTURE"))"
