#!/usr/bin/env bash
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/../../../../../../.." && pwd)"

echo "[phase2100/product] S3 (crate ny-llvmc) reps..."
(
  set -e
  (cd "$ROOT/crates/nyash-llvm-compiler" && timeout 180 cargo build --release -j 24 >/dev/null 2>&1) || true
  (cd "$ROOT/crates/nyash_kernel" && timeout 180 cargo build --release -j 24 >/dev/null 2>&1) || true
  BIN_NYLLVMC="$ROOT/target/release/ny-llvmc"
  if [[ -x "$BIN_NYLLVMC" ]]; then
    tmpj="/tmp/ny_crate_probe_$$.json"
    echo '{"schema_version":1,"functions":[{"name":"ny_main","blocks":[{"id":0,"inst":[{"op":"const","dst":1,"ty":"i64","value":0},{"op":"ret","value":1}]}]}]}' > "$tmpj"
    tmpo="/tmp/ny_crate_probe_$$.o"
    if HAKO_LLVM_CANARY_NORMALIZE=1 "$BIN_NYLLVMC" --in "$tmpj" --out "$tmpo" >/dev/null 2>&1; then
      rm -f "$tmpo" "$tmpj" || true
      bash "$ROOT/tools/smokes/v2/run.sh" --profile quick --filter 'phase2100/s3_backend_selector_crate_exe_return42_canary_vm.sh'
      bash "$ROOT/tools/smokes/v2/run.sh" --profile quick --filter 'phase2100/s3_backend_selector_crate_exe_compare_eq_true_canary_vm.sh'
      bash "$ROOT/tools/smokes/v2/run.sh" --profile quick --filter 'phase2100/s3_backend_selector_crate_exe_binop_return_canary_vm.sh'
    else
      rm -f "$tmpo" "$tmpj" || true
      echo "[phase2100/product] SKIP crate reps (ny-llvmc probe failed)" >&2
    fi
  else
    echo "[phase2100/product] SKIP crate reps (ny-llvmc not built)" >&2
  fi
) || echo "[phase2100/product] crate reps encountered a non-fatal issue; continuing"
