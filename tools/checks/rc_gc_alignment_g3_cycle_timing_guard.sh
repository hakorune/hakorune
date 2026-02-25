#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
CASES_FILE="$ROOT_DIR/tools/checks/rc_gc_alignment_g3_cycle_timing_cases.txt"
DOC="$ROOT_DIR/docs/development/current/main/design/rc-gc-alignment-g3-cycle-explicit-drop-ssot.md"
GATE="$ROOT_DIR/tools/smokes/v2/profiles/integration/apps/rc_gc_alignment_g3_cycle_timing_gate.sh"

cd "$ROOT_DIR"

echo "[rc-gc-g3-guard] checking G-RC-3 cycle/timing matrix wiring"

if ! command -v rg >/dev/null 2>&1; then
  echo "[rc-gc-g3-guard] ERROR: rg is required" >&2
  exit 2
fi

for required in "$CASES_FILE" "$DOC" "$GATE"; do
  if [[ ! -f "$required" ]]; then
    echo "[rc-gc-g3-guard] ERROR: required file missing: $required"
    exit 1
  fi
done

if [[ ! -x "$GATE" ]]; then
  echo "[rc-gc-g3-guard] ERROR: gate missing or not executable: $GATE"
  exit 1
fi

if ! rg -q '^Decision: accepted$' "$DOC"; then
  echo "[rc-gc-g3-guard] ERROR: G-RC-3 SSOT decision drift (expected Decision: accepted)"
  exit 1
fi

mapfile -t CASES < <(grep -v '^[[:space:]]*#' "$CASES_FILE" | sed '/^[[:space:]]*$/d')
if [[ "${#CASES[@]}" -lt 4 ]]; then
  echo "[rc-gc-g3-guard] ERROR: cycle/timing matrix too small (expected>=4 got=${#CASES[@]})"
  exit 1
fi

HAS_WEAK_AND_DROP=0
HAS_STRONG_CYCLE=0
HAS_EXPLICIT_DROP=0
HAS_SCOPE_END=0

for row in "${CASES[@]}"; do
  IFS='|' read -r case_id gate_rel focus extra <<<"$row"
  if [[ -n "${extra:-}" ]]; then
    echo "[rc-gc-g3-guard] ERROR: malformed row (too many columns): $row"
    exit 1
  fi
  if [[ -z "$case_id" || -z "$gate_rel" || -z "$focus" ]]; then
    echo "[rc-gc-g3-guard] ERROR: malformed row (empty fields): $row"
    exit 1
  fi

  case "$focus" in
    weak_and_drop) HAS_WEAK_AND_DROP=1 ;;
    strong_cycle) HAS_STRONG_CYCLE=1 ;;
    explicit_drop_timing) HAS_EXPLICIT_DROP=1 ;;
    scope_end_timing) HAS_SCOPE_END=1 ;;
    *)
      echo "[rc-gc-g3-guard] ERROR: unknown focus tag: $focus (row=$row)"
      exit 1
      ;;
  esac

  if [[ ! -x "$ROOT_DIR/$gate_rel" ]]; then
    echo "[rc-gc-g3-guard] ERROR: gate missing or not executable: $gate_rel"
    exit 1
  fi
  if ! rg -q "$case_id" "$DOC"; then
    echo "[rc-gc-g3-guard] ERROR: SSOT missing case id: $case_id"
    exit 1
  fi
  if ! rg -q "$gate_rel" "$DOC"; then
    echo "[rc-gc-g3-guard] ERROR: SSOT missing gate reference: $gate_rel"
    exit 1
  fi
done

if [[ "$HAS_WEAK_AND_DROP" -ne 1 ]]; then
  echo "[rc-gc-g3-guard] ERROR: matrix missing weak/drop parity coverage"
  exit 1
fi
if [[ "$HAS_STRONG_CYCLE" -ne 1 ]]; then
  echo "[rc-gc-g3-guard] ERROR: matrix missing strong-cycle coverage"
  exit 1
fi
if [[ "$HAS_EXPLICIT_DROP" -ne 1 ]]; then
  echo "[rc-gc-g3-guard] ERROR: matrix missing explicit-drop timing coverage"
  exit 1
fi
if [[ "$HAS_SCOPE_END" -ne 1 ]]; then
  echo "[rc-gc-g3-guard] ERROR: matrix missing scope-end timing coverage"
  exit 1
fi

if ! rg -q 'rc_gc_alignment_g3_cycle_timing_guard.sh' "$GATE"; then
  echo "[rc-gc-g3-guard] ERROR: gate missing guard precondition step"
  exit 1
fi
if ! rg -q 'rc_gc_alignment_g3_cycle_timing_cases.txt' "$GATE"; then
  echo "[rc-gc-g3-guard] ERROR: gate missing matrix binding"
  exit 1
fi
if ! rg -q 'g1_weak_and_drop_parity' "$CASES_FILE"; then
  echo "[rc-gc-g3-guard] ERROR: matrix missing G-RC-1 dependency case"
  exit 1
fi

echo "[rc-gc-g3-guard] ok"
