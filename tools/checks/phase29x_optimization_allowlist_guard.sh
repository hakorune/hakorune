#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
ALLOWLIST="$ROOT_DIR/tools/checks/phase29x_optimization_allowlist.txt"
OPTIMIZER="$ROOT_DIR/src/mir/optimizer.rs"
DOC="$ROOT_DIR/docs/development/current/main/phases/phase-29x/29x-92-optimization-allowlist-lock-ssot.md"
GATE="$ROOT_DIR/tools/smokes/v2/profiles/integration/apps/phase29x_optimization_allowlist_lock_vm.sh"

cd "$ROOT_DIR"

echo "[optimization-allowlist-guard] checking X63 optimization allowlist wiring"

if ! command -v rg >/dev/null 2>&1; then
  echo "[optimization-allowlist-guard] ERROR: rg is required" >&2
  exit 2
fi

for required in "$ALLOWLIST" "$OPTIMIZER" "$DOC"; do
  if [[ ! -f "$required" ]]; then
    echo "[optimization-allowlist-guard] ERROR: required file missing: $required"
    exit 1
  fi
done

if [[ ! -x "$GATE" ]]; then
  echo "[optimization-allowlist-guard] ERROR: gate missing or not executable: $GATE"
  exit 1
fi

if ! rg -q '^Decision: accepted$' "$DOC"; then
  echo "[optimization-allowlist-guard] ERROR: X63 SSOT decision drift (expected Decision: accepted)"
  exit 1
fi

EXPECTED=(const_fold dce cfg_simplify)
mapfile -t ACTUAL < <(grep -v '^[[:space:]]*#' "$ALLOWLIST" | sed '/^[[:space:]]*$/d')

if [[ "${#ACTUAL[@]}" -ne 3 ]]; then
  echo "[optimization-allowlist-guard] ERROR: allowlist count drift (expected=3 got=${#ACTUAL[@]})"
  exit 1
fi

UNIQUE_COUNT="$(printf '%s\n' "${ACTUAL[@]}" | sort -u | wc -l | tr -d ' ')"
if [[ "$UNIQUE_COUNT" -ne 3 ]]; then
  echo "[optimization-allowlist-guard] ERROR: allowlist contains duplicates"
  exit 1
fi

for i in "${!EXPECTED[@]}"; do
  if [[ "${ACTUAL[$i]}" != "${EXPECTED[$i]}" ]]; then
    echo "[optimization-allowlist-guard] ERROR: allowlist order drift at index $i (expected='${EXPECTED[$i]}' got='${ACTUAL[$i]}')"
    exit 1
  fi
done

for vocab in "${EXPECTED[@]}"; do
  if ! rg -q "\\b${vocab}\\b" "$DOC"; then
    echo "[optimization-allowlist-guard] ERROR: SSOT missing vocabulary: $vocab"
    exit 1
  fi
done

if ! rg -q 'PHASE29X_OPT_SAFESET' "$OPTIMIZER"; then
  echo "[optimization-allowlist-guard] ERROR: optimizer safeset constant missing"
  exit 1
fi
if ! rg -q 'mir_optimizer_phase29x_allowlist_lock' "$OPTIMIZER"; then
  echo "[optimization-allowlist-guard] ERROR: optimizer allowlist lock test missing"
  exit 1
fi
for vocab in "${EXPECTED[@]}"; do
  if ! rg -q "\"${vocab}\"" "$OPTIMIZER"; then
    echo "[optimization-allowlist-guard] ERROR: optimizer safeset missing vocabulary: $vocab"
    exit 1
  fi
done

if ! rg -q 'phase29x_runtime_core_gate_vm.sh' "$GATE"; then
  echo "[optimization-allowlist-guard] ERROR: gate missing X62 precondition step"
  exit 1
fi
if ! rg -q 'mir_optimizer_phase29x_allowlist_lock' "$GATE"; then
  echo "[optimization-allowlist-guard] ERROR: gate missing allowlist lock cargo test step"
  exit 1
fi
if ! rg -q 'phase29x_optimization_allowlist.txt' "$GATE"; then
  echo "[optimization-allowlist-guard] ERROR: gate missing allowlist source binding"
  exit 1
fi

echo "[optimization-allowlist-guard] ok"
