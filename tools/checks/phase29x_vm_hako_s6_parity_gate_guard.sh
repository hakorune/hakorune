#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
GATE="$ROOT_DIR/tools/smokes/v2/profiles/integration/apps/phase29x_vm_hako_s6_parity_gate_vm.sh"

cd "$ROOT_DIR"

echo "[vm-hako-s6-parity-gate-guard] checking X56 gate wiring"

if ! command -v rg >/dev/null 2>&1; then
  echo "[vm-hako-s6-parity-gate-guard] ERROR: rg is required" >&2
  exit 2
fi

if [[ ! -x "$GATE" ]]; then
  echo "[vm-hako-s6-parity-gate-guard] ERROR: gate missing or not executable: $GATE"
  exit 1
fi

required_steps=(
  phase29x_vm_hako_s6_vocab_guard_vm.sh
  phase29z_vm_hako_s5_array_get_parity_vm.sh
  phase29z_vm_hako_s5_array_set_parity_vm.sh
  phase29z_vm_hako_s5_await_non_future_reject_vm.sh
  phase29z_vm_hako_backend_frame_vm.sh
)

for step in "${required_steps[@]}"; do
  if ! rg -q "$step" "$GATE"; then
    echo "[vm-hako-s6-parity-gate-guard] ERROR: gate missing step: $step"
    exit 1
  fi
done

if ! rg -q 'run_with_vm_route_pin' "$GATE"; then
  echo "[vm-hako-s6-parity-gate-guard] ERROR: gate missing route pin helper call: run_with_vm_route_pin"
  exit 1
fi

echo "[vm-hako-s6-parity-gate-guard] ok"
