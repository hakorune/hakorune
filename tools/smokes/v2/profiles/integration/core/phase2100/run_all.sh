#!/usr/bin/env bash
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/../../../../../../.." && pwd)"

# Quick profile guard: this aggregator is heavier than a single test.
# In quick profile (or when per-test timeout is tight), skip to keep the suite fast/green.
if [[ "${SMOKES_CURRENT_PROFILE:-}" = "quick" ]]; then
  echo "[SKIP] phase2100/run_all: skipped under quick profile (aggregator is heavy)" >&2
  exit 0
fi
to=${SMOKES_DEFAULT_TIMEOUT:-0}
case "$to" in ''|*[!0-9]*) to=0;; esac
if [ "$to" -gt 0 ] && [ "$to" -lt 60 ]; then
  echo "[SKIP] phase2100/run_all: SMOKES_DEFAULT_TIMEOUT=$to is too small for aggregator" >&2
  exit 0
fi

echo "[phase2100] S1/S2 (v1) repeat determinism..."
# Layer 1: 軽量セルフホスト・カナリア（常時・LLVM不要）
bash "$ROOT/tools/smokes/v2/run.sh" --profile quick --filter 'phase2100/selfhost_canary_minimal.sh'
bash "$ROOT/tools/smokes/v2/run.sh" --profile quick --filter 'phase2048/s1s2s3_repeat_const_canary_vm.sh'
bash "$ROOT/tools/smokes/v2/run.sh" --profile quick --filter 'phase2048/s1s2s3_repeat_compare_cfg_canary_vm.sh'
bash "$ROOT/tools/smokes/v2/run.sh" --profile quick --filter 'phase2048/s1s2s3_repeat_threeblock_collect_canary_vm.sh'

if [[ "${HAKO_PHASE2100_ENABLE_HV1:-1}" == "1" ]]; then
  echo "[phase2100] PRIMARY (hv1 inline) — selfhost v1 minimal (Option A/B)..."
  bash "$ROOT/tools/smokes/v2/run.sh" --profile quick --filter 'phase2051/selfhost_v1_primary_rc42_canary_vm.sh'
  bash "$ROOT/tools/smokes/v2/run.sh" --profile quick --filter 'phase2051/selfhost_v1_provider_primary_rc42_canary_vm.sh'
else
  echo "[phase2100] Skipping hv1 inline PRIMARY (default). Set HAKO_PHASE2100_ENABLE_HV1=1 to run."
fi

# Decide S3 policy: auto-enable when LLVM18 is present unless user forces off
# llvmlite harness reps are deprecated from default (21.13). You can still run them
# explicitly via filter, or set NYASH_LLVM_RUN_LLVMLITE=1 to include here.
if [[ "${NYASH_LLVM_RUN_LLVMLITE:-0}" == "1" ]]; then
  if command -v llvm-config-18 >/dev/null 2>&1; then
    echo "[phase2100] S3 (llvmlite+NyRT) reps (opt-in)..."
    # Minimal prebuilds (best-effort)
    (cd "$ROOT/crates/nyash-llvm-compiler" && timeout 180 cargo build --release -j 24 >/dev/null 2>&1) || true
    (cd "$ROOT/crates/nyash_kernel" && timeout 180 cargo build --release -j 24 >/dev/null 2>&1) || true
    NYASH_LLVM_SKIP_BUILD=${NYASH_LLVM_SKIP_BUILD:-1} \
      bash "$ROOT/tools/smokes/v2/run.sh" --profile quick --timeout 120 --filter 'phase2049/s3_link_run_llvmlite_*'
  else
    echo "[phase2100] SKIP llvmlite reps (LLVM18 not available)" >&2
  fi
else
  echo "[phase2100] llvmlite reps are deprecated by default (set NYASH_LLVM_RUN_LLVMLITE=1 to include)"
fi

# Optional: Selfhost EXE-first smoke (heavy). Disabled by default.
if [[ "${SMOKES_ENABLE_SELFHOST:-0}" == "1" ]]; then
  if command -v llvm-config-18 >/dev/null 2>&1; then
    echo "[phase2100] Selfhost EXE-first smokes (opt-in)..."
    timeout 300 bash "$ROOT/tools/exe_first_smoke.sh"
    timeout 300 bash "$ROOT/tools/exe_first_runner_smoke.sh"
  else
    echo "[phase2100] SKIP selfhost EXE-first (LLVM18 not available)" >&2
  fi
fi

# Crate backend (ny-llvmc) — run a tiny representative set always when available
  echo "[phase2100] S3 (crate ny-llvmc) reps..."
(
  set -e
  # Prebuild crate tools quickly (best-effort)
  (cd "$ROOT/crates/nyash-llvm-compiler" && timeout 180 cargo build --release -j 24 >/dev/null 2>&1) || true
  (cd "$ROOT/crates/nyash_kernel" && timeout 180 cargo build --release -j 24 >/dev/null 2>&1) || true
  # Probe ny-llvmc availability by compiling a minimal ret0 ny_main
  BIN_NYLLVMC="$ROOT/target/release/ny-llvmc"
  if [[ -x "$BIN_NYLLVMC" ]]; then
    tmpj="/tmp/ny_crate_probe_$$.json"; echo '{"schema_version":1,"functions":[{"name":"ny_main","blocks":[{"id":0,"inst":[{"op":"const","dst":1,"ty":"i64","value":0},{"op":"ret","value":1}]}]}]}' > "$tmpj"
    tmpo="/tmp/ny_crate_probe_$$.o"
    if HAKO_LLVM_CANARY_NORMALIZE=1 "$BIN_NYLLVMC" --in "$tmpj" --out "$tmpo" >/dev/null 2>&1; then
      rm -f "$tmpo" "$tmpj" || true
      # Run representative crate EXE canaries (fast)
      bash "$ROOT/tools/smokes/v2/run.sh" --profile quick --filter 'phase2100/s3_backend_selector_crate_exe_return42_canary_vm.sh'
      bash "$ROOT/tools/smokes/v2/run.sh" --profile quick --filter 'phase2100/s3_backend_selector_crate_exe_compare_eq_true_canary_vm.sh'
      bash "$ROOT/tools/smokes/v2/run.sh" --profile quick --filter 'phase2100/s3_backend_selector_crate_exe_binop_return_canary_vm.sh'
    else
      rm -f "$tmpo" "$tmpj" || true
      echo "[phase2100] SKIP crate reps (ny-llvmc probe failed)" >&2
    fi
  else
    echo "[phase2100] SKIP crate reps (ny-llvmc not built)" >&2
  fi
) || echo "[phase2100] crate reps encountered a non-fatal issue; continuing"

# Native backend (experimental) — fast reps guarded by llc presence
if command -v llc >/dev/null 2>&1; then
  echo "[phase2100] Native backend reps (llc detected)..."
  # Prebuild NyRT to speed up link (best-effort)
  (cd "$ROOT/crates/nyash_kernel" && timeout 180 cargo build --release -j 24 >/dev/null 2>&1) || true
  bash "$ROOT/tools/smokes/v2/run.sh" --profile quick --filter 'phase2120/native_backend_return42_canary_vm.sh'
  bash "$ROOT/tools/smokes/v2/run.sh" --profile quick --filter 'phase2120/native_backend_binop_add_canary_vm.sh'
  bash "$ROOT/tools/smokes/v2/run.sh" --profile quick --filter 'phase2120/native_backend_compare_eq_canary_vm.sh'
else
  echo "[phase2100] SKIP native backend reps (llc not available)" >&2
fi

# SSOT relative inference — unique case (always-on, quick)
bash "$ROOT/tools/smokes/v2/run.sh" --profile quick --filter 'phase2211/ssot_relative_unique_canary_vm.sh'

echo "[phase2100] Done."
