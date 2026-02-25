#!/bin/bash
# check_phase29x_x23_readiness.sh
# Preflight checker for Phase 29x X23 (Rust-optional done sync).

set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
STRICT=0

usage() {
  cat <<'USAGE' >&2
Usage:
  check_phase29x_x23_readiness.sh [--strict]

Options:
  --strict   exit non-zero when any required condition is unmet
USAGE
}

while [ $# -gt 0 ]; do
  case "$1" in
    --strict)
      STRICT=1
      shift
      ;;
    -h|--help)
      usage
      exit 0
      ;;
    *)
      echo "unknown arg: $1" >&2
      usage
      exit 2
      ;;
  esac
done

DOC_X22="$ROOT_DIR/docs/development/current/main/phases/phase-29x/29x-44-vm-route-three-day-gate-evidence.md"
DOC_X23="$ROOT_DIR/docs/development/current/main/phases/phase-29x/29x-45-rust-optional-done-sync-ssot.md"
DOC_GC_POLICY="$ROOT_DIR/docs/development/current/main/design/runtime-gc-policy-and-order-ssot.md"
DOC_LIFECYCLE="$ROOT_DIR/docs/reference/language/lifecycle.md"

ok=1

check_file() {
  local path="$1"
  local label="$2"
  if [ -f "$path" ]; then
    echo "[x23-ready] PASS file:$label"
  else
    echo "[x23-ready] FAIL file:$label missing ($path)"
    ok=0
  fi
}

check_contains() {
  local path="$1"
  local pattern="$2"
  local label="$3"
  if rg -q "$pattern" "$path"; then
    echo "[x23-ready] PASS text:$label"
  else
    echo "[x23-ready] FAIL text:$label pattern-not-found"
    ok=0
  fi
}

check_file "$DOC_X22" "x22-evidence"
check_file "$DOC_X23" "x23-ssot"
check_file "$DOC_GC_POLICY" "gc-policy-ssot"
check_file "$DOC_LIFECYCLE" "lifecycle-spec"

if [ -f "$DOC_GC_POLICY" ]; then
  check_contains "$DOC_GC_POLICY" "GC.*optional|optional.*GC" "gc-policy-optional-wording"
fi
if [ -f "$DOC_LIFECYCLE" ]; then
  check_contains "$DOC_LIFECYCLE" "not as a semantic requirement|must not change \\*program meaning\\*|must not rely on GC timing|GC.*必須ではない|意味論.*GC" "lifecycle-gc-semantics-wording"
fi

set +e
"$ROOT_DIR/tools/selfhost/check_phase29x_x22_evidence.sh" --strict >/tmp/phase29x_x23_x22_strict.log 2>&1
x22_rc=$?
set -e

if [ "$x22_rc" -eq 0 ]; then
  echo "[x23-ready] PASS gate:x22-strict"
else
  echo "[x23-ready] FAIL gate:x22-strict"
  sed -n '1,120p' /tmp/phase29x_x23_x22_strict.log
  ok=0
fi

if [ "$ok" -eq 1 ]; then
  echo "[x23-ready] status=READY"
else
  echo "[x23-ready] status=NOT_READY"
fi

if [ "$STRICT" -eq 1 ] && [ "$ok" -ne 1 ]; then
  exit 1
fi
