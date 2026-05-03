#!/usr/bin/env bash
set -euo pipefail

ROOT="${NYASH_ROOT:-$(cd "$(dirname "$0")/../../../.." && pwd)}"
STAGE1_BIN="${STAGE1_BIN:-${ROOT}/target/selfhost/hakorune.stage1_cli}"
STAGE2_BIN="${STAGE2_BIN:-${ROOT}/target/selfhost/hakorune.stage1_cli.stage2}"
ENTRY="${ENTRY:-${ROOT}/apps/tests/hello_simple_llvm.hako}"

for bin in "$STAGE1_BIN" "$STAGE2_BIN"; do
  if [[ ! -x "$bin" ]]; then
    echo "[FAIL] missing selfhost bin: $bin" >&2
    exit 2
  fi
done

if [[ ! -f "$ENTRY" ]]; then
  echo "[FAIL] missing entry: $ENTRY" >&2
  exit 2
fi

probe_rc97() {
  local label="$1"
  shift
  local stdout_file
  local stderr_file
  stdout_file="$(mktemp)"
  stderr_file="$(mktemp)"
  set +e
  "$@" >"$stdout_file" 2>"$stderr_file"
  local rc=$?
  set -e
  echo "[raw-direct-probe] ${label}.rc=${rc}"
  rm -f "$stdout_file" "$stderr_file"
  if [[ "$rc" -ne 97 ]]; then
    echo "[FAIL] expected rc=97 for ${label}" >&2
    exit 1
  fi
}

for bin in "$STAGE1_BIN" "$STAGE2_BIN"; do
  label="$(basename "$bin")"
  probe_rc97 "${label}.raw_source" "$bin" "$ENTRY"
  probe_rc97 "${label}.raw_emit_program_retired" "$bin" emit program-json "$ENTRY"
  probe_rc97 "${label}.raw_emit_mir" "$bin" emit mir-json "$ENTRY"
done

echo "[raw-direct-probe] result=PASS (retired raw direct lane pinned)"
