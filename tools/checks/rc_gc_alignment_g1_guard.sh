#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
CASES_FILE="$ROOT_DIR/tools/checks/rc_gc_alignment_g1_lifecycle_cases.txt"
DOC="$ROOT_DIR/docs/development/current/main/design/rc-gc-alignment-g1-lifecycle-parity-ssot.md"
GATE="$ROOT_DIR/tools/smokes/v2/profiles/integration/apps/rc_gc_alignment_g1_lifecycle_parity_vm_llvm.sh"

cd "$ROOT_DIR"

echo "[rc-gc-g1-guard] checking G-RC-1 lifecycle parity wiring"

if ! command -v rg >/dev/null 2>&1; then
  echo "[rc-gc-g1-guard] ERROR: rg is required" >&2
  exit 2
fi

for required in "$CASES_FILE" "$DOC" "$GATE"; do
  if [[ ! -f "$required" ]]; then
    echo "[rc-gc-g1-guard] ERROR: required file missing: $required"
    exit 1
  fi
done

if [[ ! -x "$GATE" ]]; then
  echo "[rc-gc-g1-guard] ERROR: gate missing or not executable: $GATE"
  exit 1
fi

if ! rg -q '^Decision: accepted$' "$DOC"; then
  echo "[rc-gc-g1-guard] ERROR: G-RC-1 SSOT decision drift (expected Decision: accepted)"
  exit 1
fi

mapfile -t CASES < <(grep -v '^[[:space:]]*#' "$CASES_FILE" | sed '/^[[:space:]]*$/d')
if [[ "${#CASES[@]}" -lt 3 ]]; then
  echo "[rc-gc-g1-guard] ERROR: lifecycle case inventory too small (expected>=3 got=${#CASES[@]})"
  exit 1
fi

for row in "${CASES[@]}"; do
  IFS='|' read -r case_id fixture_rel expected_exit extra <<<"$row"
  if [[ -n "${extra:-}" ]]; then
    echo "[rc-gc-g1-guard] ERROR: malformed row (too many columns): $row"
    exit 1
  fi
  if [[ -z "$case_id" || -z "$fixture_rel" || -z "$expected_exit" ]]; then
    echo "[rc-gc-g1-guard] ERROR: malformed row (empty fields): $row"
    exit 1
  fi
  if [[ ! "$expected_exit" =~ ^[0-9]+$ ]]; then
    echo "[rc-gc-g1-guard] ERROR: expected_exit must be numeric: $row"
    exit 1
  fi
  if [[ ! -f "$ROOT_DIR/$fixture_rel" ]]; then
    echo "[rc-gc-g1-guard] ERROR: fixture missing on disk: $fixture_rel"
    exit 1
  fi
  if ! rg -q "$case_id" "$DOC"; then
    echo "[rc-gc-g1-guard] ERROR: SSOT missing case id: $case_id"
    exit 1
  fi
  if ! rg -q "$fixture_rel" "$DOC"; then
    echo "[rc-gc-g1-guard] ERROR: SSOT missing fixture path: $fixture_rel"
    exit 1
  fi
done

if ! rg -q 'rc_gc_alignment_g1_guard.sh' "$GATE"; then
  echo "[rc-gc-g1-guard] ERROR: gate missing guard precondition step"
  exit 1
fi
if ! rg -q 'rc_gc_alignment_g1_lifecycle_cases.txt' "$GATE"; then
  echo "[rc-gc-g1-guard] ERROR: gate missing case inventory binding"
  exit 1
fi
if ! rg -q -- '--backend "\$backend"' "$GATE"; then
  echo "[rc-gc-g1-guard] ERROR: gate missing backend switch wiring"
  exit 1
fi
if ! rg -q 'if \[\[ "\$backend" = "vm" \]\]' "$GATE"; then
  echo "[rc-gc-g1-guard] ERROR: gate missing VM execution branch"
  exit 1
fi
if ! rg -q 'run_backend_case llvm' "$GATE"; then
  echo "[rc-gc-g1-guard] ERROR: gate missing LLVM execution path"
  exit 1
fi

echo "[rc-gc-g1-guard] ok"
