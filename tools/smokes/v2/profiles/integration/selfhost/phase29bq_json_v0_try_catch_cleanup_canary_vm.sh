#!/bin/bash
# phase29bq_json_v0_try_catch_cleanup_canary_vm.sh
# JSON v0 canary for throw/catch/cleanup semantics without surface `throw` syntax.
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../../../../../.." && pwd)"
BIN="${NYASH_BIN:-$ROOT_DIR/target/release/hakorune}"

if [ ! -x "$BIN" ]; then
  echo "[FAIL] nyash binary not found: $BIN" >&2
  exit 2
fi

run_case() {
  local json_file="$1"
  local expect_rc="$2"
  local expect_stdout="$3"
  local out_file
  local err_file
  local rc
  local actual_stdout

  out_file="$(mktemp /tmp/phase29bq_json_v0_canary.XXXXXX.out)"
  err_file="$(mktemp /tmp/phase29bq_json_v0_canary.XXXXXX.err)"

  set +e
  NYASH_TRY_RESULT_MODE=1 \
    NYASH_DISABLE_PLUGINS=1 \
    "$BIN" --debug-fuel unlimited --json-file "$json_file" \
    >"$out_file" 2>"$err_file"
  rc=$?
  set -e

  if [ "$rc" -ne "$expect_rc" ]; then
    echo "[FAIL] json canary rc mismatch: file=$json_file expected=$expect_rc actual=$rc" >&2
    tail -n 40 "$err_file" >&2
    rm -f "$out_file" "$err_file"
    exit 1
  fi

  actual_stdout="$(cat "$out_file")"
  if [ "$expect_stdout" = "__EMPTY__" ]; then
    expect_stdout=""
  fi
  if [ "$actual_stdout" != "$expect_stdout" ]; then
    echo "[FAIL] json canary stdout mismatch: file=$json_file" >&2
    echo "expected: <$expect_stdout>" >&2
    echo "actual:   <$actual_stdout>" >&2
    tail -n 20 "$err_file" >&2
    rm -f "$out_file" "$err_file"
    exit 1
  fi

  rm -f "$out_file" "$err_file"
}

run_case "$ROOT_DIR/tests/json_v0_stage3/try_basic.json" 12 "12"
run_case "$ROOT_DIR/tests/json_v0_stage3/block_postfix_catch.json" 43 "__EMPTY__"

echo "[PASS] phase29bq_json_v0_try_catch_cleanup_canary_vm"
