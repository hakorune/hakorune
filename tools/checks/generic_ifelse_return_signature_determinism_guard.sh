#!/usr/bin/env bash
# Determinism guard for GENERIC-LEGACY-IFELSE-RETURN-NONDETERMINISM-D0:
# `function.blocks` is a HashMap; the return-type strategy must infer the
# same signature on every run for multi-return functions. Runs the two
# if_else_return{,_var} fixtures serially N times and requires the
# canonical `define i64 @main()` on every run.
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="ifelse-return-signature-determinism"
BIN="${NYASH_BIN:-$ROOT_DIR/target/debug/hakorune}"
RUNS="${HAKO_DETERMINISM_RUNS:-20}"

if [[ ! -x "$BIN" ]]; then
  echo "[$TAG] ERROR: binary not executable: $BIN" >&2
  exit 2
fi

fail=0
for fixture in \
  "$ROOT_DIR/apps/tests/phase29bq_selfhost_blocker_parse_program2_if_else_return_min.hako" \
  "$ROOT_DIR/apps/tests/phase29bq_selfhost_blocker_parse_program2_if_else_return_var_min.hako"; do
  for i in $(seq 1 "$RUNS"); do
    sig="$("$BIN" --dump-mir "$fixture" 2>/dev/null | rg -o 'define (i64|void) @main' | head -1 || true)"
    if [[ "$sig" != "define i64 @main" ]]; then
      echo "[$TAG] FAIL run $i $(basename "$fixture"): ${sig:-<no output>}" >&2
      fail=1
    fi
  done
done

if [[ "$fail" -ne 0 ]]; then
  exit 1
fi
echo "[$TAG] OK ${RUNS}x2 runs, canonical i64 @main every time"
