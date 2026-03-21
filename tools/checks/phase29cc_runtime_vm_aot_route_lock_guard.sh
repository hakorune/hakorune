#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
cd "$ROOT_DIR"

CONCAT_ROUTE_SMOKE="tools/smokes/v2/profiles/integration/phase21_5/perf/kilo/phase21_5_perf_kilo_text_concat_contract_vm.sh"
ARRAY_ROUTE_SMOKE="tools/smokes/v2/profiles/integration/phase21_5/perf/kilo/phase21_5_perf_kilo_runtime_data_array_route_contract_vm.sh"
LOCK_DOC="docs/development/current/main/phases/phase-29cc/29cc-217-runtime-vm-aot-route-lock-ssot.md"
DEV_GATE="tools/checks/dev_gate.sh"

for file in \
  "$CONCAT_ROUTE_SMOKE" \
  "$ARRAY_ROUTE_SMOKE" \
  "$LOCK_DOC" \
  "$DEV_GATE"; do
  if [ ! -f "$file" ]; then
    echo "[runtime-vm-aot-route-lock-guard] missing file: $file" >&2
    exit 1
  fi
done

if ! rg -q "phase29cc_runtime_vm_aot_route_lock_guard.sh" "$LOCK_DOC" "$DEV_GATE"; then
  echo "[runtime-vm-aot-route-lock-guard] lock wiring missing in docs/dev_gate" >&2
  exit 1
fi

echo "[runtime-vm-aot-route-lock-guard] running kilo text concat route contract"
env NYASH_LLVM_SKIP_BUILD="${NYASH_LLVM_SKIP_BUILD:-1}" \
  bash "$CONCAT_ROUTE_SMOKE"

echo "[runtime-vm-aot-route-lock-guard] running kilo runtime_data->array route contract"
env NYASH_LLVM_SKIP_BUILD="${NYASH_LLVM_SKIP_BUILD:-1}" \
  bash "$ARRAY_ROUTE_SMOKE"

echo "[runtime-vm-aot-route-lock-guard] ok"
