#!/bin/bash
# phase29bq_typed_terminal_source_backed_gate_mir.sh - source-backed
# (--backend mir) typed-terminal contract gate for corpus rows whose
# VM-lane acceptance came from retired machinery (M10b-I0-R0).
#
# Usage:
#   phase29bq_typed_terminal_source_backed_gate_mir.sh [--only <case_id>]
#
# Contract per row: nonzero exit AND pinned typed-terminal marker substring
# on the source-backed lane. These rows must NOT silently accept; their
# dispositions are recorded in generic-loop-legacy-disposition-v1.tsv.

set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../../../../.." && pwd)" # tools/
LOG_DIR="${PHASE29BQ_FAST_LOG_DIR:-/tmp}"

source "$ROOT_DIR/smokes/v2/lib/test_runner.sh"
source "$ROOT_DIR/smokes/v2/lib/joinir_source_backed_gate.sh"
require_env || exit 2

if [ -n "${NYASH_BIN:-}" ] && [ -x "${NYASH_BIN}" ]; then
  BIN_VER="$("${NYASH_BIN}" --version 2>/dev/null | head -n 1 || true)"
  echo "[INFO] NYASH_BIN=${NYASH_BIN} ${BIN_VER}"
fi

LIST_FILE="$ROOT_DIR/smokes/v2/profiles/integration/joinir/phase29bq_typed_terminal_source_backed_cases.tsv"

usage() {
  cat >&2 <<'EOF2'
Usage:
  phase29bq_typed_terminal_source_backed_gate_mir.sh [--only <case_id>]

  --only <case_id>  Run a single case_id from
                    phase29bq_typed_terminal_source_backed_cases.tsv
EOF2
}

ONLY_CASE=""
if [ "${1:-}" = "--only" ]; then
  ONLY_CASE="${2:-}"
  if [ -z "$ONLY_CASE" ]; then
    usage
    exit 2
  fi
elif [ -n "${1:-}" ]; then
  echo "[FAIL] Unknown arg: ${1:-}" >&2
  usage
  exit 2
fi

if [ -n "$ONLY_CASE" ]; then
  line=$(awk -F '\t' -v key="$ONLY_CASE" '
    $0 ~ /^#/ {next}
    NF >= 3 && $3 == key {print; exit}
  ' "$LIST_FILE")

  if [ -z "$line" ]; then
    echo "[FAIL] case_id not found in list: $ONLY_CASE" >&2
    exit 1
  fi

  fixture=$(echo "$line" | cut -f1)
  marker=$(echo "$line" | cut -f2)

  if [[ "$fixture" != /* ]]; then
    fixture="$NYASH_ROOT/$fixture"
  fi

  LOG_CASE="$LOG_DIR/phase29bq_typed_terminal_source_backed_${$}.log"
  if ! run_source_backed_terminal_gate \
    "phase29bq_typed_terminal_source_backed:$ONLY_CASE" \
    "$fixture" \
    "$marker" \
    "${RUN_TIMEOUT_SECS:-10}" >"$LOG_CASE" 2>&1; then
    echo "[FAIL] gate failed: $ONLY_CASE"
    echo "LOG: $LOG_CASE"
    tail -n 40 "$LOG_CASE" || true
    exit 1
  fi
  cat "$LOG_CASE"
  echo "[PASS] phase29bq_typed_terminal_source_backed_gate_mir: PASS (only=$ONLY_CASE)"
  exit 0
fi

LOG_LIST="$LOG_DIR/phase29bq_typed_terminal_source_backed_${$}.log"
if ! run_source_backed_terminal_list_gate "$LIST_FILE" "phase29bq_typed_terminal_source_backed" "${RUN_TIMEOUT_SECS:-10}" >"$LOG_LIST" 2>&1; then
  echo "[FAIL] gate failed: phase29bq_typed_terminal_source_backed_cases"
  echo "LOG: $LOG_LIST"
  tail -n 60 "$LOG_LIST" || true
  exit 1
fi

cat "$LOG_LIST"
echo "[PASS] phase29bq_typed_terminal_source_backed_gate_mir: PASS"
