#!/usr/bin/env bash
# CORE-DIRECT-RETIRE-R0 — one post-decode terminal, no retry/fallback.

set -euo pipefail

source "$(dirname "$0")/../../lib/test_runner.sh"
require_env || exit 2

ROOT="$NYASH_ROOT"
BIN="$(realpath "$NYASH_BIN")"
MIR_FIXTURE="$ROOT/apps/tests/hello_simple_llvm_native_probe_v1.mir.json"
PROGRAM_FIXTURE="$ROOT/tests/json_v0/arith.json"
TMP_DIR="$(mktemp -d /tmp/hako-core-direct-r0.XXXXXX)"
trap 'rm -rf "$TMP_DIR"' EXIT

[[ -f "$MIR_FIXTURE" ]] || { echo "[FAIL] missing MIR fixture: $MIR_FIXTURE" >&2; exit 1; }
[[ -f "$PROGRAM_FIXTURE" ]] || { echo "[FAIL] missing Program fixture: $PROGRAM_FIXTURE" >&2; exit 1; }

run_case() {
  local label="$1"
  local fixture="$2"
  local expected_rc="$3"
  local stdout_file="$TMP_DIR/${label}.stdout"
  local stderr_file="$TMP_DIR/${label}.stderr"
  shift 3

  set +e
  (
    cd "$TMP_DIR"
    env -u HAKO_CORE_DIRECT_INPROC -u NYASH_CORE_DIRECT_INPROC \
      HAKO_CORE_DIRECT=1 NYASH_CORE_DIRECT=0 \
      NYASH_NYRT_SILENT_RESULT=1 "$@" "$BIN" --mir-json-file "$fixture" \
      >"$stdout_file" 2>"$stderr_file"
  )
  local rc=$?
  set -e

  [[ "$rc" -eq "$expected_rc" ]] || {
    echo "[FAIL] $label: expected rc=$expected_rc got rc=$rc" >&2
    cat "$stderr_file" >&2
    exit 1
  }
}

run_case "valid" "$MIR_FIXTURE" 1
printf '[core-direct/retired]\n' >"$TMP_DIR/expected-retired"
cmp "$TMP_DIR/expected-retired" "$TMP_DIR/valid.stderr"
[[ ! -s "$TMP_DIR/valid.stdout" ]] || {
  echo "[FAIL] valid: retired terminal wrote stdout" >&2
  exit 1
}
[[ ! -e "$TMP_DIR/tmp/core_exec_direct.hako" ]] || {
  echo "[FAIL] valid: CoreDirect child wrapper was created" >&2
  exit 1
}

run_case "wrong-entrance" "$PROGRAM_FIXTURE" 1
[[ ! -s "$TMP_DIR/wrong-entrance.stdout" ]] || {
  echo "[FAIL] wrong-entrance: decoder error wrote stdout" >&2
  exit 1
}
if rg -q '\[core-direct/retired\]' "$TMP_DIR/wrong-entrance.stderr"; then
  echo "[FAIL] wrong-entrance: decoder failure was relabeled as CoreDirect retirement" >&2
  exit 1
fi
rg -q 'MIR JSON parse error' "$TMP_DIR/wrong-entrance.stderr" || {
  echo "[FAIL] wrong-entrance: existing decoder terminal is missing" >&2
  exit 1
}

echo "[PASS] core_direct_retire_r0: post-decode retired terminal, wrong-entrance preservation, child/retry/fallback-free"
