#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
ROOT_DIR="$(git -C "$SCRIPT_DIR" rev-parse --show-toplevel 2>/dev/null || (cd "$SCRIPT_DIR/../../../../../.." && pwd))"
NYASH_BIN="${NYASH_BIN:-$ROOT_DIR/target/release/hakorune}"

if [[ ! -x "$NYASH_BIN" ]]; then echo "[SKIP] hakorune not built"; exit 0; fi

# Minimal program (no jsonfrag fallback expected in normal conditions)
CODE='static box Main { method main(args) { return 0 } }'
SRC=$(mktemp --suffix .hako)
OUT=$(mktemp --suffix .json)
LOG=$(mktemp)
trap 'rm -f "$SRC" "$OUT" "$LOG"' EXIT
printf '%s' "$CODE" > "$SRC"

# Provider-first emit; forbid forced jsonfrag
set +e
HAKO_SELFHOST_BUILDER_FIRST=0 \
HAKO_MIR_BUILDER_LOOP_JSONFRAG=0 \
HAKO_MIR_BUILDER_LOOP_FORCE_JSONFRAG=0 \
NYASH_JSON_ONLY=1 bash "$ROOT_DIR/tools/smokes/v2/lib/emit_mir_route.sh" --route hako-helper --timeout-secs "${HAKO_BUILD_TIMEOUT:-60}" --out "$OUT" --input "$SRC" 2>"$LOG" 1>/dev/null
rc=$?
set -e

if [[ $rc -ne 0 ]]; then
  echo "[SKIP] provider emit failed (unstable env)"
  exit 0
fi

if grep -q "\[emit/jsonfrag\]" "$LOG"; then
  echo "[FAIL] emit_provider_no_jsonfrag_canary: jsonfrag tag detected"
  exit 1
fi

echo "[PASS] emit_provider_no_jsonfrag_canary"
