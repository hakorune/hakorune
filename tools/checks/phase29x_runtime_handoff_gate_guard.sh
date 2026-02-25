#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
GATE="$ROOT_DIR/tools/smokes/v2/profiles/integration/apps/phase29x_runtime_handoff_gate_vm.sh"

cd "$ROOT_DIR"

echo "[runtime-handoff-gate-guard] checking X52 integration gate wiring"

if [[ ! -x "$GATE" ]]; then
  echo "[runtime-handoff-gate-guard] ERROR: gate missing or not executable: $GATE"
  exit 1
fi

required_steps=(
  phase29x_vm_route_pin_guard_vm.sh
  phase29x_vm_hako_strict_dev_replay_vm.sh
  phase29x_vm_hako_newclosure_contract_vm.sh
  phase29x_core_cabi_delegation_guard_vm.sh
)

for step in "${required_steps[@]}"; do
  if ! rg -q "$step" "$GATE"; then
    echo "[runtime-handoff-gate-guard] ERROR: gate missing step: $step"
    exit 1
  fi
done

echo "[runtime-handoff-gate-guard] ok"
