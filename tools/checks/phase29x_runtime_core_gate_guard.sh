#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
DOC="$ROOT_DIR/docs/development/current/main/phases/phase-29x/29x-89-runtime-core-integrated-gate-ssot.md"
GATE="$ROOT_DIR/tools/smokes/v2/profiles/integration/apps/phase29x_runtime_core_gate_vm.sh"

X59_GATE="tools/smokes/v2/profiles/integration/apps/phase29x_abi_borrowed_owned_conformance_vm.sh"
X60_GATE="tools/smokes/v2/profiles/integration/apps/phase29x_rc_phase2_queue_lock_vm.sh"
X61_GATE="tools/smokes/v2/profiles/integration/apps/phase29x_observability_drift_guard_vm.sh"

cd "$ROOT_DIR"

echo "[runtime-core-gate-guard] checking X62 runtime core integrated gate wiring"

if ! command -v rg >/dev/null 2>&1; then
  echo "[runtime-core-gate-guard] ERROR: rg is required" >&2
  exit 2
fi

for required in "$DOC" "$GATE"; do
  if [[ ! -f "$required" ]]; then
    echo "[runtime-core-gate-guard] ERROR: required file missing: $required"
    exit 1
  fi
done

if [[ ! -x "$GATE" ]]; then
  echo "[runtime-core-gate-guard] ERROR: gate missing or not executable: $GATE"
  exit 1
fi

if ! rg -q '^Decision: accepted$' "$DOC"; then
  echo "[runtime-core-gate-guard] ERROR: X62 SSOT decision drift (expected Decision: accepted)"
  exit 1
fi

for gate in "$X59_GATE" "$X60_GATE" "$X61_GATE"; do
  if [[ ! -x "$ROOT_DIR/$gate" ]]; then
    echo "[runtime-core-gate-guard] ERROR: dependency gate missing or not executable: $gate"
    exit 1
  fi
  if ! rg -q "$gate" "$GATE"; then
    echo "[runtime-core-gate-guard] ERROR: integrated gate missing dependency step: $gate"
    exit 1
  fi
  if ! rg -q "$gate" "$DOC"; then
    echo "[runtime-core-gate-guard] ERROR: SSOT missing dependency gate reference: $gate"
    exit 1
  fi
done

echo "[runtime-core-gate-guard] ok"
