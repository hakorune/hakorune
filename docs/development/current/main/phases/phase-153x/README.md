# Phase 153x: ny_mir_builder harness drop

- Status: Landed
- 目的: `ny_mir_builder --emit obj|exe` が daily mainline では harness keep を強制しないようにする。
- 対象:
  - `src/bin/ny_mir_builder.rs`
  - `tools/ny_mir_builder.sh`
  - related smoke/tool callers

## Landed slice

- `src/bin/ny_mir_builder.rs`
  - `obj|exe` now prefer the `hakorune` binary and no longer read as harness-owned routes
- `tools/ny_mir_builder.sh`
  - unknown `NYASH_LLVM_BACKEND` values no longer fall through to llvmlite keep
- `tools/build_llvm.sh`
  - daily mainline no longer auto-falls back to `harness`; missing `ny-llvmc` is now a fail-fast mainline error
- `tools/llvm_smoke.sh`
  - daily object emit examples no longer set `NYASH_LLVM_USE_HARNESS=1`
