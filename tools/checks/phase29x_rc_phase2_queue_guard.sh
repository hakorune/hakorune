#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
CASES="$ROOT_DIR/tools/checks/phase29x_rc_phase2_queue_cases.txt"
SELFCHK="$ROOT_DIR/src/bin/rc_insertion_selfcheck.rs"
DOC="$ROOT_DIR/docs/development/current/main/phases/phase-29x/29x-87-rc-insertion-phase2-queue-lock-ssot.md"
GATE="$ROOT_DIR/tools/smokes/v2/profiles/integration/apps/phase29x_rc_phase2_queue_lock_vm.sh"

cd "$ROOT_DIR"

echo "[rc-phase2-queue-guard] checking X60 RC phase2 queue wiring"

if ! command -v rg >/dev/null 2>&1; then
  echo "[rc-phase2-queue-guard] ERROR: rg is required" >&2
  exit 2
fi

for required in "$CASES" "$SELFCHK" "$DOC"; do
  if [[ ! -f "$required" ]]; then
    echo "[rc-phase2-queue-guard] ERROR: required file missing: $required"
    exit 1
  fi
done

if [[ ! -x "$GATE" ]]; then
  echo "[rc-phase2-queue-guard] ERROR: gate missing or not executable: $GATE"
  exit 1
fi

if ! rg -q '^Decision: accepted$' "$DOC"; then
  echo "[rc-phase2-queue-guard] ERROR: X60 SSOT decision drift (expected Decision: accepted)"
  exit 1
fi

while IFS='|' read -r case_id marker; do
  if [[ -z "${case_id}" || "${case_id}" =~ ^# ]]; then
    continue
  fi
  if [[ -z "${marker}" ]]; then
    echo "[rc-phase2-queue-guard] ERROR: malformed case row (missing marker): $case_id"
    exit 1
  fi
  if ! rg -Fq "$marker" "$SELFCHK"; then
    echo "[rc-phase2-queue-guard] ERROR: selfcheck marker missing for case ${case_id}: ${marker}"
    exit 1
  fi
  if ! rg -Fq "$case_id" "$DOC"; then
    echo "[rc-phase2-queue-guard] ERROR: SSOT missing case id: ${case_id}"
    exit 1
  fi
  if ! rg -Fq "$marker" "$DOC"; then
    echo "[rc-phase2-queue-guard] ERROR: SSOT missing case marker: ${marker}"
    exit 1
  fi
done <"$CASES"

if ! rg -Fq '[rc_phase2_queue] loop=ok call=ok early_exit=ok' "$SELFCHK"; then
  echo "[rc-phase2-queue-guard] ERROR: selfcheck summary marker missing"
  exit 1
fi
if ! rg -q 'phase29x_abi_borrowed_owned_conformance_vm.sh' "$GATE"; then
  echo "[rc-phase2-queue-guard] ERROR: gate missing X59 precondition step"
  exit 1
fi
if ! rg -q 'rc_insertion_selfcheck --features rc-insertion-minimal' "$GATE"; then
  echo "[rc-phase2-queue-guard] ERROR: gate missing rc_insertion_selfcheck feature step"
  exit 1
fi
if ! rg -q 'phase29x_rc_phase2_queue_cases.txt' "$GATE"; then
  echo "[rc-phase2-queue-guard] ERROR: gate missing queue cases source binding"
  exit 1
fi

echo "[rc-phase2-queue-guard] ok"
