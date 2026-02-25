#!/usr/bin/env bash
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/../../../../../../.." && pwd)"

echo "[phase2048] Running repeat (determinism) reps..."
bash "$ROOT/tools/smokes/v2/run.sh" --profile quick --filter 'phase2048/s1s2s3_repeat_*'

echo "[phase2048] Running PRIMARY reps (If/Logical/Loop/Array)..."
bash "$ROOT/tools/smokes/v2/run.sh" --profile quick --filter 'phase2048/if_*'
bash "$ROOT/tools/smokes/v2/run.sh" --profile quick --filter 'phase2048/logical_*'
bash "$ROOT/tools/smokes/v2/run.sh" --profile quick --filter 'phase2048/loop_*'
bash "$ROOT/tools/smokes/v2/run.sh" --profile quick --filter 'phase2048/hv1_inline_array_*'
bash "$ROOT/tools/smokes/v2/run.sh" --profile quick --filter 'phase2048/hv1_inline_map_*'
bash "$ROOT/tools/smokes/v2/run.sh" --profile quick --filter 'phase2048/typeop_*'

if [[ "${NYASH_LLVM_S3:-0}" == "1" ]]; then
  echo "[phase2048] Running S3 reps (llvmlite+link+run)..."
  NYASH_LLVM_S3=1 bash "$ROOT/tools/smokes/v2/run.sh" --profile quick --filter 'phase2047/s3_*'
else
  echo "[phase2048] Skipping S3 (set NYASH_LLVM_S3=1 to enable)"
fi

echo "[phase2048] Done."
