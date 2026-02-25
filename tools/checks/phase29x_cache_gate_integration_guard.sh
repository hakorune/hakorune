#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
DAILY_GATE="$ROOT_DIR/tools/smokes/v2/profiles/integration/apps/phase29x_llvm_only_daily_gate.sh"
CACHE_GATE="$ROOT_DIR/tools/smokes/v2/profiles/integration/apps/phase29x_cache_lane_gate_vm.sh"

if [ ! -f "$DAILY_GATE" ]; then
  echo "[cache-gate-integration-guard] ERROR: daily gate script missing: $DAILY_GATE"
  exit 1
fi

if [ ! -x "$CACHE_GATE" ]; then
  echo "[cache-gate-integration-guard] ERROR: cache lane gate missing or not executable: $CACHE_GATE"
  exit 1
fi

if ! rg -q "phase29x_cache_lane_gate_vm\\.sh" "$DAILY_GATE"; then
  echo "[cache-gate-integration-guard] ERROR: daily gate does not include cache lane gate call"
  exit 1
fi

for smoke in \
  phase29x_cache_key_determinism_vm.sh \
  phase29x_l1_mir_cache_vm.sh \
  phase29x_l2_object_cache_vm.sh \
  phase29x_l3_link_cache_vm.sh
do
  if ! rg -q "$smoke" "$CACHE_GATE"; then
    echo "[cache-gate-integration-guard] ERROR: cache lane gate missing smoke call: $smoke"
    exit 1
  fi
done

echo "[cache-gate-integration-guard] ok"
