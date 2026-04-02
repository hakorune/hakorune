#!/usr/bin/env bash
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/../../../../../../.." && pwd)"

if command -v llc >/dev/null 2>&1; then
  echo "[phase2100/experimental] Native backend reps (llc detected)..."
  (cd "$ROOT/crates/nyash_kernel" && timeout 180 cargo build --release -j 24 >/dev/null 2>&1) || true
  bash "$ROOT/tools/smokes/v2/run.sh" --profile quick --filter 'phase2120/native_backend_return42_canary_vm.sh'
  bash "$ROOT/tools/smokes/v2/run.sh" --profile quick --filter 'phase2120/native_backend_binop_add_canary_vm.sh'
  bash "$ROOT/tools/smokes/v2/run.sh" --profile quick --filter 'phase2120/native_backend_compare_eq_canary_vm.sh'
else
  echo "[phase2100/experimental] SKIP native backend reps (llc not available)" >&2
fi
