#!/bin/bash
# phase29bq_portable_owner_source_backed_gate_mir.sh - source-backed
# (--backend mir) acceptance gate for portable-owner corpus rows migrated
# off the VM-compat phase29bq list gate (M10b-I0-R0).
#
# Usage:
#   phase29bq_portable_owner_source_backed_gate_mir.sh [--only <case_id>]
#
# Contract per row: fixture + expected output + allowed exit code on the
# source-backed lane. Old VM-gate rows are removed only after this gate is
# registered and passing.

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

LIST_FILE="$ROOT_DIR/smokes/v2/profiles/integration/joinir/phase29bq_portable_owner_source_backed_cases.tsv"

usage() {
  cat >&2 <<'EOF'
Usage:
  phase29bq_portable_owner_source_backed_gate_mir.sh [--only <case_id>]

  --only <case_id>  Run a single case_id from
                    phase29bq_portable_owner_source_backed_cases.tsv
EOF
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
    NF >= 4 && $4 == key {print; exit}
  ' "$LIST_FILE")

  if [ -z "$line" ]; then
    echo "[FAIL] case_id not found in list: $ONLY_CASE" >&2
    exit 1
  fi

  fixture=$(echo "$line" | cut -f1)
  expected=$(echo "$line" | cut -f2)
  allowed_rc=$(echo "$line" | cut -f3)

  if [ "$expected" = "__EMPTY__" ]; then
    expected=""
  fi
  if [ -z "$allowed_rc" ]; then
    allowed_rc="0"
  fi
  if [[ "$fixture" != /* ]]; then
    fixture="$NYASH_ROOT/$fixture"
  fi

  LOG_CASE="$LOG_DIR/phase29bq_portable_owner_source_backed_${$}.log"
  if ! run_source_backed_gate \
    "phase29bq_portable_owner_source_backed:$ONLY_CASE" \
    "$fixture" \
    "$expected" \
    "$allowed_rc" \
    "${RUN_TIMEOUT_SECS:-10}" >"$LOG_CASE" 2>&1; then
    echo "[FAIL] gate failed: $ONLY_CASE"
    echo "LOG: $LOG_CASE"
    tail -n 40 "$LOG_CASE" || true
    exit 1
  fi
  cat "$LOG_CASE"
  echo "[PASS] phase29bq_portable_owner_source_backed_gate_mir: PASS (only=$ONLY_CASE)"
  exit 0
fi

LOG_LIST="$LOG_DIR/phase29bq_portable_owner_source_backed_${$}.log"
if ! run_source_backed_list_gate "$LIST_FILE" "phase29bq_portable_owner_source_backed" "${RUN_TIMEOUT_SECS:-10}" >"$LOG_LIST" 2>&1; then
  echo "[FAIL] gate failed: phase29bq_portable_owner_source_backed_cases"
  echo "LOG: $LOG_LIST"
  tail -n 60 "$LOG_LIST" || true
  exit 1
fi

cat "$LOG_LIST"
echo "[PASS] phase29bq_portable_owner_source_backed_gate_mir: PASS"
