#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
DOC="$ROOT_DIR/docs/development/current/main/phases/phase-29x/29x-94-optimization-gate-integration-rollback-lock-ssot.md"
GATE="$ROOT_DIR/tools/smokes/v2/profiles/integration/apps/phase29x_optimization_gate_vm.sh"
X63_GATE="tools/smokes/v2/profiles/integration/apps/phase29x_optimization_allowlist_lock_vm.sh"
X64_GATE="tools/smokes/v2/profiles/integration/apps/phase29x_optimization_parity_fixtures_vm.sh"

cd "$ROOT_DIR"

echo "[optimization-gate-guard] checking X65 optimization gate integration wiring"

if ! command -v rg >/dev/null 2>&1; then
  echo "[optimization-gate-guard] ERROR: rg is required" >&2
  exit 2
fi

for required in "$DOC" "$GATE"; do
  if [[ ! -f "$required" ]]; then
    echo "[optimization-gate-guard] ERROR: required file missing: $required"
    exit 1
  fi
done

if [[ ! -x "$GATE" ]]; then
  echo "[optimization-gate-guard] ERROR: gate missing or not executable: $GATE"
  exit 1
fi

for dep in "$X63_GATE" "$X64_GATE"; do
  if [[ ! -x "$ROOT_DIR/$dep" ]]; then
    echo "[optimization-gate-guard] ERROR: dependency gate missing or not executable: $dep"
    exit 1
  fi
  if ! rg -q "$dep" "$GATE"; then
    echo "[optimization-gate-guard] ERROR: integrated gate missing dependency step: $dep"
    exit 1
  fi
  if ! rg -q "$dep" "$DOC"; then
    echo "[optimization-gate-guard] ERROR: SSOT missing dependency gate reference: $dep"
    exit 1
  fi
done

if ! rg -q '^Decision: accepted$' "$DOC"; then
  echo "[optimization-gate-guard] ERROR: X65 SSOT decision drift (expected Decision: accepted)"
  exit 1
fi

if ! rg -q -- '--no-optimize' "$GATE"; then
  echo "[optimization-gate-guard] ERROR: gate missing rollback probe (--no-optimize)"
  exit 1
fi
if ! rg -q 'phase29x_optimization_parity_const_fold_min.hako' "$GATE"; then
  echo "[optimization-gate-guard] ERROR: gate missing rollback fixture binding"
  exit 1
fi
if ! rg -q 'rollback' "$DOC"; then
  echo "[optimization-gate-guard] ERROR: SSOT missing rollback contract section"
  exit 1
fi
if ! rg -q -- '--no-optimize' "$DOC"; then
  echo "[optimization-gate-guard] ERROR: SSOT missing rollback switch contract (--no-optimize)"
  exit 1
fi
if ! rg -q 'expected rc=6' "$DOC"; then
  echo "[optimization-gate-guard] ERROR: SSOT missing rollback probe expected rc contract"
  exit 1
fi

echo "[optimization-gate-guard] ok"
