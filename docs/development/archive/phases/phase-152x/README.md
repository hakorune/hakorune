# Phase 152x: llvmlite object emit cutover

- Status: Landed
- 目的: LLVM object emit の current authority を `llvmlite` keep lane から `ny-llvmc --emit obj` mainline lane へ切り替える。
- 対象:
  - `src/runner/modes/common_util/exec.rs`
  - `src/runner/product/llvm/mod.rs`
  - `src/bin/ny_mir_builder.rs`
  - `tools/build_llvm.sh`
  - `CURRENT_TASK.md`

## Decision Now

- `--backend llvm` daily mainline と `--emit-exe` はすでに `ny-llvmc`
- 残り mismatch は `.o` emit lane
- current cut order:
  1. runner object emit cutover
  2. `ny_mir_builder obj|exe` harness drop
  3. llvmlite keep/archive lock
  4. then reopen `phase-137x`

## Exact Focus

- `llvmlite_emit_obj_lib(...)` を current owner から外す
- `ny_llvmc_emit_obj_lib(...)` を実名どおり `ny-llvmc --emit obj` に揃える
- `emit_requested_object_or_exit(...)` が `NYASH_LLVM_USE_HARNESS=1` でも daily mainline object emit を読めるようにする

## Current Slice

- landed in-source:
  - `src/runner/modes/common_util/exec.rs::ny_llvmc_emit_obj_lib(...)` now emits via `ny-llvmc --emit obj`
  - `src/runner/product/llvm/mod.rs::emit_requested_object_or_exit(...)` now reads the mainline object emit path
  - `src/bin/ny_mir_builder.rs` `obj` / `exe` no longer force `NYASH_LLVM_USE_HARNESS=1`
- verified:
  - `target/release/hakorune --backend llvm apps/tests/hello_simple_llvm.hako` with `NYASH_LLVM_OBJ_OUT=/tmp/hakorune_obj_canary.o`
  - `tools/ny_mir_builder.sh --in apps/tests/mir_shape_guard/collapsed_min.mir.json --emit obj -o /tmp/ny_mir_builder_obj_canary.o`
