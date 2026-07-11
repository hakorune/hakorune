# Phase 154x: llvmlite archive lock

- Status: Active
- 目的: llvmlite / `tools/llvmlite_harness.py` / `src/llvm_py/**` を explicit compat/archive keep に押し込む。
- 対象:
  - `tools/llvmlite_harness.py`
  - `src/llvm_py/**`
  - docs / smoke callers
  - `tools/selfhost/lib/selfhost_build_exe.sh`
  - `src/host_providers/llvm_codegen/README.md`
  - `docs/guides/exe-first-wsl.md`
  - `docs/guides/selfhost-pilot.md`
  - `docs/reference/environment-variables.md`

## Current slice

- current-facing EXE / object helper docs must read `ny-llvmc` as mainline owner
- forced `NYASH_LLVM_USE_HARNESS=1` exports must not survive in daily EXE helper paths
- llvmlite must remain explicit compat/archive keep only

## Landed slice

- `tools/selfhost/lib/selfhost_build_exe.sh`
  - daily EXE helper no longer exports `NYASH_LLVM_USE_HARNESS=1`
- `tools/build_llvm.sh`
  - explicit `NYASH_LLVM_COMPILER=harness` now routes through `ny-llvmc --driver harness`
- `tools/archive/manual-smokes/llvm_smoke.sh`
  - historical LLVM smoke is now an explicit llvmlite compat/probe keep script
- `src/host_providers/llvm_codegen/README.md`
  - mainline object emit owner is documented as `ny-llvmc --emit obj`
- `docs/guides/exe-first-wsl.md`
  - WSL quickstart now treats `ny-llvmc` as the daily EXE-first owner; llvmlite is keep-only troubleshooting
- `docs/guides/selfhost-pilot.md`
  - troubleshooting no longer presents llvmlite as a daily requirement
- `docs/reference/environment-variables.md`
  - `NYASH_LLVM_USE_HARNESS=1` examples are labeled explicit keep-lane, not current owner
