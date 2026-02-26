#!/usr/bin/env bash
set -euo pipefail

# dev_gate.sh
# Purpose: single-entry developer gate with tiered profiles.
#
# Usage:
#   tools/checks/dev_gate.sh [quick|hotpath|wasm-boundary-lite|portability|milestone|milestone-runtime|milestone-perf]
#   tools/checks/dev_gate.sh --list
#
# Profiles:
#   quick     : day-to-day lightweight checks (default)
#   hotpath   : quick + phase21.5 perf hotpath contract bundle
#   wasm-boundary-lite : quick + wasm-backend compile + boundary fast-fail unit locks
#   portability : cross-platform maintenance guards (Windows WSL/CMD + macOS readiness)
#   milestone-runtime : hotpath + runtime/selfhost milestone smoke
#   milestone-perf    : hotpath + phase21.5 full perf milestone checks
#   milestone         : milestone-runtime + milestone-perf (backward compatible)

PROFILE="${1:-quick}"
ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"

usage() {
  cat <<'USAGE'
Usage:
  tools/checks/dev_gate.sh [quick|hotpath|wasm-boundary-lite|portability|milestone|milestone-runtime|milestone-perf]
  tools/checks/dev_gate.sh --list
USAGE
}

list_profiles() {
  cat <<'LIST'
[dev-gate] profiles:
  quick:
    - cargo check --bin hakorune
    - PYTHONPATH=src/llvm_py:. python3 -m unittest src/llvm_py/tests/test_strlen_fast.py
    - phase21_5_perf_chip8_kernel_crosslang_contract_vm.sh
  hotpath:
    - quick
    - tools/perf/run_phase21_5_perf_gate_bundle.sh hotpath
  wasm-boundary-lite:
    - quick
    - cargo check --features wasm-backend --bin hakorune
    - cargo test --features wasm-backend extern_contract_supported_name_maps_to_import -- --nocapture
    - cargo test --features wasm-backend test_unsupported_extern_call_fails_fast_with_supported_list -- --nocapture
    - cargo test --features wasm-backend test_unsupported_boxcall_method_fails_fast_with_supported_list -- --nocapture
    - phase29cc_wsm02d_demo_min_boundary_vm.sh
    - phase29cc_wsm02d_demo_unsupported_boundary_vm.sh
  portability:
    - tools/checks/windows_wsl_cmd_smoke.sh (preflight by default)
    - tools/checks/macos_portability_guard.sh
  milestone-runtime:
    - hotpath
    - phase29y_no_compat_mainline_vm.sh
    - phase29x_derust_done_matrix_vm.sh
  milestone-perf:
    - hotpath
    - tools/perf/run_phase21_5_perf_gate_bundle.sh full
  milestone:
    - milestone-runtime
    - milestone-perf
LIST
}

run_step() {
  local label="$1"
  shift
  echo "[dev-gate] >>> ${label}"
  "$@"
}

run_quick() {
  run_step "cargo check" \
    cargo check --bin hakorune

  run_step "llvm_py unittest (strlen_fast)" \
    env PYTHONPATH=src/llvm_py:. python3 -m unittest src/llvm_py/tests/test_strlen_fast.py

  run_step "chip8 crosslang contract smoke" \
    env NYASH_LLVM_SKIP_BUILD="${NYASH_LLVM_SKIP_BUILD:-1}" \
      bash tools/smokes/v2/profiles/integration/apps/phase21_5_perf_chip8_kernel_crosslang_contract_vm.sh
}

run_hotpath() {
  run_quick
  run_step "phase21.5 perf gate bundle (hotpath)" \
    env NYASH_LLVM_SKIP_BUILD="${NYASH_LLVM_SKIP_BUILD:-1}" \
      tools/perf/run_phase21_5_perf_gate_bundle.sh hotpath
}

run_wasm_boundary_lite() {
  run_quick
  run_step "cargo check (wasm-backend)" \
    cargo check --features wasm-backend --bin hakorune
  run_step "wasm extern contract lock" \
    cargo test --features wasm-backend extern_contract_supported_name_maps_to_import -- --nocapture
  run_step "wasm extern unsupported boundary lock" \
    cargo test --features wasm-backend test_unsupported_extern_call_fails_fast_with_supported_list -- --nocapture
  run_step "wasm boxcall unsupported boundary lock" \
    cargo test --features wasm-backend test_unsupported_boxcall_method_fails_fast_with_supported_list -- --nocapture
  run_step "wasm demo-min fixture boundary lock" \
    bash tools/smokes/v2/profiles/integration/apps/phase29cc_wsm02d_demo_min_boundary_vm.sh
  run_step "wasm demo unsupported boundary lock" \
    bash tools/smokes/v2/profiles/integration/apps/phase29cc_wsm02d_demo_unsupported_boundary_vm.sh
}

run_portability() {
  run_step "windows WSL/CMD smoke (preflight)" \
    bash tools/checks/windows_wsl_cmd_smoke.sh
  run_step "macOS portability guard" \
    bash tools/checks/macos_portability_guard.sh
}

run_milestone_runtime() {
  run_hotpath
  run_step "phase29y no-compat mainline smoke" \
    bash tools/smokes/v2/profiles/integration/apps/phase29y_no_compat_mainline_vm.sh
  run_step "phase29x de-rust done matrix smoke" \
    bash tools/smokes/v2/profiles/integration/apps/phase29x_derust_done_matrix_vm.sh
}

run_milestone_perf() {
  run_hotpath
  run_step "phase21.5 perf gate bundle (full)" \
    env NYASH_LLVM_SKIP_BUILD="${NYASH_LLVM_SKIP_BUILD:-1}" \
      tools/perf/run_phase21_5_perf_gate_bundle.sh full
}

run_milestone() {
  run_milestone_runtime
  run_milestone_perf
}

cd "${ROOT_DIR}"

case "${PROFILE}" in
  -h|--help)
    usage
    exit 0
    ;;
  --list)
    list_profiles
    exit 0
    ;;
  quick)
    run_quick
    ;;
  hotpath)
    run_hotpath
    ;;
  wasm-boundary-lite)
    run_wasm_boundary_lite
    ;;
  portability)
    run_portability
    ;;
  milestone-runtime)
    run_milestone_runtime
    ;;
  milestone-perf)
    run_milestone_perf
    ;;
  milestone)
    run_milestone
    ;;
  *)
    usage >&2
    exit 2
    ;;
esac

echo "[dev-gate] profile=${PROFILE} ok"
