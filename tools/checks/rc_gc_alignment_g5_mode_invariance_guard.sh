#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
CASES_FILE="$ROOT_DIR/tools/checks/rc_gc_alignment_g5_mode_invariance_cases.txt"
DOC="$ROOT_DIR/docs/development/current/main/design/rc-gc-alignment-g5-gc-mode-semantics-invariance-ssot.md"
GATE="$ROOT_DIR/tools/smokes/v2/profiles/integration/apps/rc_gc_alignment_g5_mode_invariance_vm_llvm.sh"

cd "$ROOT_DIR"

echo "[rc-gc-g5-guard] checking G-RC-5 GC mode semantics invariance wiring"

if ! command -v rg >/dev/null 2>&1; then
  echo "[rc-gc-g5-guard] ERROR: rg is required" >&2
  exit 2
fi

for required in "$CASES_FILE" "$DOC" "$GATE"; do
  if [[ ! -f "$required" ]]; then
    echo "[rc-gc-g5-guard] ERROR: required file missing: $required"
    exit 1
  fi
done

if [[ ! -x "$GATE" ]]; then
  echo "[rc-gc-g5-guard] ERROR: gate missing or not executable: $GATE"
  exit 1
fi

if ! rg -q '^Decision: accepted$' "$DOC"; then
  echo "[rc-gc-g5-guard] ERROR: G-RC-5 SSOT decision drift (expected Decision: accepted)"
  exit 1
fi

mapfile -t CASES < <(grep -v '^[[:space:]]*#' "$CASES_FILE" | sed '/^[[:space:]]*$/d')
if [[ "${#CASES[@]}" -lt 4 ]]; then
  echo "[rc-gc-g5-guard] ERROR: mode invariance inventory too small (expected>=4 got=${#CASES[@]})"
  exit 1
fi

HAS_EXPLICIT_DROP=0
HAS_SCOPE_END=0
HAS_WEAK_SUCCESS=0
HAS_WEAK_FAIL=0

for row in "${CASES[@]}"; do
  IFS='|' read -r case_id fixture_rel expected_exit focus extra <<<"$row"
  if [[ -n "${extra:-}" ]]; then
    echo "[rc-gc-g5-guard] ERROR: malformed row (too many columns): $row"
    exit 1
  fi
  if [[ -z "$case_id" || -z "$fixture_rel" || -z "$expected_exit" || -z "$focus" ]]; then
    echo "[rc-gc-g5-guard] ERROR: malformed row (empty fields): $row"
    exit 1
  fi
  if [[ ! "$expected_exit" =~ ^[0-9]+$ ]]; then
    echo "[rc-gc-g5-guard] ERROR: expected_exit must be numeric: $row"
    exit 1
  fi
  if [[ ! -f "$ROOT_DIR/$fixture_rel" ]]; then
    echo "[rc-gc-g5-guard] ERROR: fixture missing on disk: $fixture_rel"
    exit 1
  fi

  case "$focus" in
    explicit_drop) HAS_EXPLICIT_DROP=1 ;;
    scope_end_timing) HAS_SCOPE_END=1 ;;
    weak_success) HAS_WEAK_SUCCESS=1 ;;
    weak_fail) HAS_WEAK_FAIL=1 ;;
    *)
      echo "[rc-gc-g5-guard] ERROR: unknown focus tag: $focus (row=$row)"
      exit 1
      ;;
  esac

  if ! rg -q "$case_id" "$DOC"; then
    echo "[rc-gc-g5-guard] ERROR: SSOT missing case id: $case_id"
    exit 1
  fi
  if ! rg -q "$fixture_rel" "$DOC"; then
    echo "[rc-gc-g5-guard] ERROR: SSOT missing fixture path: $fixture_rel"
    exit 1
  fi
done

if [[ "$HAS_EXPLICIT_DROP" -ne 1 ]]; then
  echo "[rc-gc-g5-guard] ERROR: inventory missing explicit_drop coverage"
  exit 1
fi
if [[ "$HAS_SCOPE_END" -ne 1 ]]; then
  echo "[rc-gc-g5-guard] ERROR: inventory missing scope_end_timing coverage"
  exit 1
fi
if [[ "$HAS_WEAK_SUCCESS" -ne 1 ]]; then
  echo "[rc-gc-g5-guard] ERROR: inventory missing weak_success coverage"
  exit 1
fi
if [[ "$HAS_WEAK_FAIL" -ne 1 ]]; then
  echo "[rc-gc-g5-guard] ERROR: inventory missing weak_fail coverage"
  exit 1
fi

if ! rg -q 'rc_gc_alignment_g5_mode_invariance_guard.sh' "$GATE"; then
  echo "[rc-gc-g5-guard] ERROR: gate missing guard precondition step"
  exit 1
fi
if ! rg -q 'rc_gc_alignment_g5_mode_invariance_cases.txt' "$GATE"; then
  echo "[rc-gc-g5-guard] ERROR: gate missing case inventory binding"
  exit 1
fi
if ! rg -q 'NYASH_GC_MODE=' "$GATE"; then
  echo "[rc-gc-g5-guard] ERROR: gate missing GC mode env wiring"
  exit 1
fi
if ! rg -q 'run_backend_case vm rc\+cycle' "$GATE"; then
  echo "[rc-gc-g5-guard] ERROR: gate missing VM rc+cycle path"
  exit 1
fi
if ! rg -q 'run_backend_case vm off' "$GATE"; then
  echo "[rc-gc-g5-guard] ERROR: gate missing VM off path"
  exit 1
fi
if ! rg -q 'run_backend_case llvm rc\+cycle' "$GATE"; then
  echo "[rc-gc-g5-guard] ERROR: gate missing LLVM rc+cycle path"
  exit 1
fi
if ! rg -q 'run_backend_case llvm off' "$GATE"; then
  echo "[rc-gc-g5-guard] ERROR: gate missing LLVM off path"
  exit 1
fi

echo "[rc-gc-g5-guard] ok"
