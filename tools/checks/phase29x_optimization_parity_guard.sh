#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
PARITY_FIXTURES="$ROOT_DIR/tools/checks/phase29x_optimization_parity_fixtures.txt"
REJECT_FIXTURES="$ROOT_DIR/tools/checks/phase29x_optimization_reject_fixtures.txt"
DOC="$ROOT_DIR/docs/development/current/main/phases/phase-29x/29x-93-optimization-parity-fixtures-lock-ssot.md"
GATE="$ROOT_DIR/tools/smokes/v2/profiles/integration/apps/phase29x_optimization_parity_fixtures_vm.sh"
X63_GATE="tools/smokes/v2/profiles/integration/apps/phase29x_optimization_allowlist_lock_vm.sh"

cd "$ROOT_DIR"

echo "[optimization-parity-guard] checking X64 optimization parity fixture wiring"

if ! command -v rg >/dev/null 2>&1; then
  echo "[optimization-parity-guard] ERROR: rg is required" >&2
  exit 2
fi

for required in "$PARITY_FIXTURES" "$REJECT_FIXTURES" "$DOC" "$GATE"; do
  if [[ ! -f "$required" ]]; then
    echo "[optimization-parity-guard] ERROR: required file missing: $required"
    exit 1
  fi
done

if [[ ! -x "$GATE" ]]; then
  echo "[optimization-parity-guard] ERROR: gate missing or not executable: $GATE"
  exit 1
fi

if [[ ! -x "$ROOT_DIR/$X63_GATE" ]]; then
  echo "[optimization-parity-guard] ERROR: X63 precondition gate missing or not executable: $X63_GATE"
  exit 1
fi

if ! rg -q '^Decision: accepted$' "$DOC"; then
  echo "[optimization-parity-guard] ERROR: X64 SSOT decision drift (expected Decision: accepted)"
  exit 1
fi

mapfile -t PARITY_LINES < <(grep -v '^[[:space:]]*#' "$PARITY_FIXTURES" | sed '/^[[:space:]]*$/d')
if [[ "${#PARITY_LINES[@]}" -ne 2 ]]; then
  echo "[optimization-parity-guard] ERROR: parity fixture count drift (expected=2 got=${#PARITY_LINES[@]})"
  exit 1
fi

for line in "${PARITY_LINES[@]}"; do
  IFS='|' read -r fixture_rel expected_rc expected_stdout extra <<<"$line"
  if [[ -n "${extra:-}" ]]; then
    echo "[optimization-parity-guard] ERROR: parity fixture line has too many columns: $line"
    exit 1
  fi
  if [[ -z "$fixture_rel" || -z "$expected_rc" || -z "$expected_stdout" ]]; then
    echo "[optimization-parity-guard] ERROR: parity fixture line has empty fields: $line"
    exit 1
  fi
  if [[ ! "$expected_rc" =~ ^[0-9]+$ ]]; then
    echo "[optimization-parity-guard] ERROR: parity expected_rc is not numeric: $line"
    exit 1
  fi
  if [[ ! -f "$ROOT_DIR/$fixture_rel" ]]; then
    echo "[optimization-parity-guard] ERROR: parity fixture missing on disk: $fixture_rel"
    exit 1
  fi
done

mapfile -t REJECT_LINES < <(grep -v '^[[:space:]]*#' "$REJECT_FIXTURES" | sed '/^[[:space:]]*$/d')
if [[ "${#REJECT_LINES[@]}" -ne 1 ]]; then
  echo "[optimization-parity-guard] ERROR: reject fixture count drift (expected=1 got=${#REJECT_LINES[@]})"
  exit 1
fi

for line in "${REJECT_LINES[@]}"; do
  IFS='|' read -r fixture_rel expected_error extra <<<"$line"
  if [[ -n "${extra:-}" ]]; then
    echo "[optimization-parity-guard] ERROR: reject fixture line has too many columns: $line"
    exit 1
  fi
  if [[ -z "$fixture_rel" || -z "$expected_error" ]]; then
    echo "[optimization-parity-guard] ERROR: reject fixture line has empty fields: $line"
    exit 1
  fi
  if [[ ! -f "$ROOT_DIR/$fixture_rel" ]]; then
    echo "[optimization-parity-guard] ERROR: reject fixture missing on disk: $fixture_rel"
    exit 1
  fi
done

if ! rg -q 'phase29x_optimization_parity_guard.sh' "$GATE"; then
  echo "[optimization-parity-guard] ERROR: gate missing X64 guard step"
  exit 1
fi
if ! rg -q "$X63_GATE" "$GATE"; then
  echo "[optimization-parity-guard] ERROR: gate missing X63 precondition step"
  exit 1
fi
if ! rg -q 'phase29x_optimization_parity_fixtures.txt' "$GATE"; then
  echo "[optimization-parity-guard] ERROR: gate missing parity fixture inventory binding"
  exit 1
fi
if ! rg -q 'phase29x_optimization_reject_fixtures.txt' "$GATE"; then
  echo "[optimization-parity-guard] ERROR: gate missing reject fixture inventory binding"
  exit 1
fi
if ! rg -q -- '--no-optimize' "$GATE"; then
  echo "[optimization-parity-guard] ERROR: gate missing no-optimize replay path"
  exit 1
fi

if ! rg -q 'phase29x_optimization_parity_fixtures.txt' "$DOC"; then
  echo "[optimization-parity-guard] ERROR: SSOT missing parity fixture inventory reference"
  exit 1
fi
if ! rg -q 'phase29x_optimization_reject_fixtures.txt' "$DOC"; then
  echo "[optimization-parity-guard] ERROR: SSOT missing reject fixture inventory reference"
  exit 1
fi
if ! rg -q 'phase29x_optimization_allowlist_lock_vm.sh' "$DOC"; then
  echo "[optimization-parity-guard] ERROR: SSOT missing X63 precondition reference"
  exit 1
fi
if ! rg -q 'Division by zero' "$DOC"; then
  echo "[optimization-parity-guard] ERROR: SSOT missing reject failure text contract"
  exit 1
fi

echo "[optimization-parity-guard] ok"
