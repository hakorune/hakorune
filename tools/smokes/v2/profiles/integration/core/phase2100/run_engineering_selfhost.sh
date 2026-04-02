#!/usr/bin/env bash
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/../../../../../../.." && pwd)"

echo "[phase2100/selfhost] S1/S2 (v1) repeat determinism..."
bash "$ROOT/tools/smokes/v2/run.sh" --profile quick --filter 'phase2100/selfhost_canary_minimal.sh'
bash "$ROOT/tools/smokes/v2/run.sh" --profile quick --filter 'phase2048/s1s2s3_repeat_const_canary_vm.sh'
bash "$ROOT/tools/smokes/v2/run.sh" --profile quick --filter 'phase2048/s1s2s3_repeat_compare_cfg_canary_vm.sh'
bash "$ROOT/tools/smokes/v2/run.sh" --profile quick --filter 'phase2048/s1s2s3_repeat_threeblock_collect_canary_vm.sh'

if [[ "${HAKO_PHASE2100_ENABLE_HV1:-1}" == "1" ]]; then
  echo "[phase2100/selfhost] PRIMARY (hv1 inline) - selfhost v1 minimal..."
  bash "$ROOT/tools/smokes/v2/run.sh" --profile quick --filter 'phase2051/selfhost_v1_primary_rc42_canary_vm.sh'
  bash "$ROOT/tools/smokes/v2/run.sh" --profile quick --filter 'phase2051/selfhost_v1_provider_primary_rc42_canary_vm.sh'
else
  echo "[phase2100/selfhost] Skipping hv1 inline PRIMARY (default). Set HAKO_PHASE2100_ENABLE_HV1=1 to run."
fi

if [[ "${SMOKES_ENABLE_SELFHOST:-0}" == "1" ]]; then
  if command -v llvm-config-18 >/dev/null 2>&1; then
    echo "[phase2100/selfhost] EXE-first selfhost smokes (opt-in)..."
    timeout 300 bash "$ROOT/tools/exe_first_smoke.sh"
    timeout 300 bash "$ROOT/tools/exe_first_runner_smoke.sh"
  else
    echo "[phase2100/selfhost] SKIP EXE-first selfhost (LLVM18 not available)" >&2
  fi
fi
