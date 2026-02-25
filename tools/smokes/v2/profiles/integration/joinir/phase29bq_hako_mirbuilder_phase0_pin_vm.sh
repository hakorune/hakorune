#!/bin/bash
# phase29bq_hako_mirbuilder_phase0_pin_vm.sh
# Phase-0 pin: AST JSON → (.hako entry) MIR JSON v0 → --mir-json-file execution
#
# Expected: stdout=0

set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../../../../.." && pwd)" # tools/
source "$ROOT_DIR/smokes/v2/lib/test_runner.sh"
require_env || exit 2

# Phase-29bq selfhost pins run in planner-required mode to exercise JoinIR plan boxes (no legacy fallback).
export HAKO_JOINIR_PLANNER_REQUIRED=1

FIXTURE="$NYASH_ROOT/apps/tests/phase29bq_blockexpr_basic_min.hako"
ENTRY="$NYASH_ROOT/lang/src/compiler/mirbuilder/emit_mir_json_v0.hako"

TMP_DIR="${PHASE29BQ_FAST_LOG_DIR:-/tmp}"
RUN_ID="phase29bq_hako_mirbuilder_phase0_pin_${$}"
PJSON="$TMP_DIR/${RUN_ID}_program.json"
MJSON="$TMP_DIR/${RUN_ID}_mir.json"

rm -f "$PJSON" "$MJSON"

# 1) Rust Stage-0: emit AST JSON
"$NYASH_BIN" --emit-ast-json "$PJSON" "$FIXTURE" >/dev/null

# 2) .hako entry: AST JSON → MIR JSON v0
HAKO_PROGRAM_JSON_FILE="$PJSON" "$NYASH_BIN" --backend vm "$ENTRY" >"$MJSON"

# 2.1) Fail-fast guard: reject known "unexecutable" patterns in MIR JSON v0 output (SSOT: docs/development/current/main/design/hako-mirbuilder-migration-phase0-entry-contract-ssot.md)
# - u32::MAX (4294967295) indicates an invalid ValueId and typically comes from unified-call drift.
if rg -n --fixed-strings '4294967295' "$MJSON" >/dev/null 2>&1; then
  echo "[FAIL] hako_mirbuilder phase0 pin: MIR JSON contains invalid ValueId (4294967295)" >&2
  echo "[FAIL] mir_json=$MJSON" >&2
  exit 1
fi
# - Unified schema op must not appear in v0 output (Phase-0 contract).
if rg -n --fixed-strings '\"op\":\"mir_call\"' "$MJSON" >/dev/null 2>&1; then
  echo "[FAIL] hako_mirbuilder phase0 pin: MIR JSON contains unified op (mir_call) but Phase-0 requires v0" >&2
  echo "[FAIL] mir_json=$MJSON" >&2
  exit 1
fi

# 3) Execute MIR JSON v0 and compare stdout
OUT="$("$NYASH_BIN" --mir-json-file "$MJSON" 2>&1 | filter_noise)"

if [ "$OUT" != "0" ]; then
  echo "[FAIL] hako_mirbuilder phase0 pin: expected stdout=0, got: '$OUT'" >&2
  echo "[FAIL] fixture=$FIXTURE" >&2
  echo "[FAIL] entry=$ENTRY" >&2
  echo "[FAIL] program_json=$PJSON" >&2
  echo "[FAIL] mir_json=$MJSON" >&2
  exit 1
fi

echo "[PASS] hako_mirbuilder phase0 pin: PASS"
