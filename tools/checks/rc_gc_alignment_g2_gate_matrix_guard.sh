#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
CASES_FILE="$ROOT_DIR/tools/checks/rc_gc_alignment_g2_gate_matrix_cases.txt"
DOC="$ROOT_DIR/docs/development/current/main/design/rc-gc-alignment-g2-fast-milestone-gate-ssot.md"
GATE="$ROOT_DIR/tools/smokes/v2/profiles/integration/apps/rc_gc_alignment_g2_fast_milestone_gate.sh"

cd "$ROOT_DIR"

echo "[rc-gc-g2-guard] checking G-RC-2 fast/milestone gate matrix wiring"

if ! command -v rg >/dev/null 2>&1; then
  echo "[rc-gc-g2-guard] ERROR: rg is required" >&2
  exit 2
fi

for required in "$CASES_FILE" "$DOC" "$GATE"; do
  if [[ ! -f "$required" ]]; then
    echo "[rc-gc-g2-guard] ERROR: required file missing: $required"
    exit 1
  fi
done

if [[ ! -x "$GATE" ]]; then
  echo "[rc-gc-g2-guard] ERROR: gate missing or not executable: $GATE"
  exit 1
fi

if ! rg -q '^Decision: accepted$' "$DOC"; then
  echo "[rc-gc-g2-guard] ERROR: G-RC-2 SSOT decision drift (expected Decision: accepted)"
  exit 1
fi

mapfile -t CASES < <(grep -v '^[[:space:]]*#' "$CASES_FILE" | sed '/^[[:space:]]*$/d')
if [[ "${#CASES[@]}" -lt 3 ]]; then
  echo "[rc-gc-g2-guard] ERROR: gate matrix too small (expected>=3 got=${#CASES[@]})"
  exit 1
fi

FAST_COUNT=0
MILESTONE_COUNT=0

for row in "${CASES[@]}"; do
  IFS='|' read -r case_id gate_rel tier extra <<<"$row"
  if [[ -n "${extra:-}" ]]; then
    echo "[rc-gc-g2-guard] ERROR: malformed row (too many columns): $row"
    exit 1
  fi
  if [[ -z "$case_id" || -z "$gate_rel" || -z "$tier" ]]; then
    echo "[rc-gc-g2-guard] ERROR: malformed row (empty fields): $row"
    exit 1
  fi
  if [[ "$tier" != "fast" && "$tier" != "milestone" ]]; then
    echo "[rc-gc-g2-guard] ERROR: tier must be fast|milestone: $row"
    exit 1
  fi
  if [[ "$tier" == "fast" ]]; then
    FAST_COUNT=$((FAST_COUNT + 1))
  fi
  if [[ "$tier" == "milestone" ]]; then
    MILESTONE_COUNT=$((MILESTONE_COUNT + 1))
  fi

  if [[ ! -x "$ROOT_DIR/$gate_rel" ]]; then
    echo "[rc-gc-g2-guard] ERROR: gate missing or not executable: $gate_rel"
    exit 1
  fi
  if ! rg -q "$case_id" "$DOC"; then
    echo "[rc-gc-g2-guard] ERROR: SSOT missing case id: $case_id"
    exit 1
  fi
  if ! rg -q "$gate_rel" "$DOC"; then
    echo "[rc-gc-g2-guard] ERROR: SSOT missing gate reference: $gate_rel"
    exit 1
  fi
done

if [[ "$FAST_COUNT" -lt 1 ]]; then
  echo "[rc-gc-g2-guard] ERROR: matrix must include at least one fast gate"
  exit 1
fi
if [[ "$MILESTONE_COUNT" -lt 1 ]]; then
  echo "[rc-gc-g2-guard] ERROR: matrix must include at least one milestone gate"
  exit 1
fi

if ! rg -q 'rc_gc_alignment_g2_gate_matrix_guard.sh' "$GATE"; then
  echo "[rc-gc-g2-guard] ERROR: gate missing guard precondition step"
  exit 1
fi
if ! rg -q 'rc_gc_alignment_g2_gate_matrix_cases.txt' "$GATE"; then
  echo "[rc-gc-g2-guard] ERROR: gate missing matrix binding"
  exit 1
fi
if ! rg -q 'g1_lifecycle_parity' "$CASES_FILE"; then
  echo "[rc-gc-g2-guard] ERROR: matrix missing G-RC-1 dependency case"
  exit 1
fi
if ! rg -q 'g3_cycle_timing_matrix' "$CASES_FILE"; then
  echo "[rc-gc-g2-guard] ERROR: matrix missing G-RC-3 dependency case"
  exit 1
fi
if ! rg -q 'g5_gc_mode_semantics_invariance' "$CASES_FILE"; then
  echo "[rc-gc-g2-guard] ERROR: matrix missing G-RC-5 dependency case"
  exit 1
fi

echo "[rc-gc-g2-guard] ok"
