---
Status: SSOT
Decision: accepted
Date: 2026-03-15
Scope: `full Rust 0` に向けた remaining Rust-owned buckets を compiler / runtime / backend の 3 レーンで固定し、current blocker と future queued work の混線を防ぐ。
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/design/de-rust-full-rust-zero-roadmap-ssot.md
  - docs/development/current/main/design/de-rust-master-task-map-ssot.md
  - docs/development/current/main/design/de-rust-post-g1-runtime-plan-ssot.md
  - docs/development/current/main/design/de-rust-backend-zero-boundary-lock-ssot.md
  - docs/development/current/main/design/de-rust-backend-zero-provisional-inventory-ssot.md
  - docs/development/current/main/phases/phase-29cj/README.md
  - docs/development/current/main/phases/phase-29ck/README.md
  - docs/development/current/main/phases/phase-29y/60-NEXT-TASK-PLAN.md
---

# De-Rust Full Rust 0 Remaining Rust Inventory (SSOT)

## Purpose

- `0 Rust` の残りを compiler / runtime / backend の 3 レーンで exact path まで固定する。
- current blocker と queued phase を同じ表現で混ぜない。
- 「active front」「frozen exact owner」「explicit keep / temporary seam」を分けて、次にどこを削るか迷わないようにする。

## 1. Boundary Lock

1. immediate blocker は引き続き pure `.hako`-only hakorune build の compiler authority removal である。
2. runtime-zero / backend-zero は full Rust 0 の subordinate lane であり、compiler blocker を上書きしない。
3. `0 Rust` は repository から Rust source を一掃する意味ではない。
   - runtime-zero done でも portability / build scaffolding / ABI glue は残り得る。
   - backend-zero done でも C ABI / plugin boundary / archived compat keep は残り得る。
4. `Cranelift` は explicit keep とする。
   - full Rust 0 inventory に載せても、backend-zero の primary replace target にはしない。

## 2. Current Snapshot (2026-03-15)

1. compiler lane:
   - `phase-29cj` は closeout-ready
   - active exact owner は `src/host_providers/mir_builder.rs::module_to_mir_json(...)`
2. runtime lane:
   - `phase-29y` blocker は `none`
   - `.hako VM` parity / backend-zero runtime proof は landed
   - remaining Rust runtime is mostly monitor-only / portability / ABI keep
3. backend lane:
   - `phase-29ck` は active queued phase
   - `.hako VM -> LlvmBackendBox -> env.codegen C-API -> exe` proof は landed
   - still-Rust / still-Python ownership is substantial, especially `ny-llvmc`, runner glue, and `llvmlite`
   - latest tightening: `src/runner/modes/llvm/object_emitter.rs` is no longer a direct llvmlite caller; it now reaches the backend boundary through `src/host_providers/llvm_codegen.rs` while pinning `llvmlite` as compat keep

## 3. Remaining Rust Inventory

### 3.1 Compiler lane

#### active exact owner

1. `src/host_providers/mir_builder.rs`
   - active stop-line:
     - `module_to_mir_json(...)`
   - role:
     - Rust host seam for canonical MIR(JSON) emission
     - source/explicit Program(JSON) handoff glue above that seam
   - rule:
     - continue shrinking only same-file handoff/finalize leaves
     - do not reopen broad bridge / shell / `.hako` helper cleanup

#### frozen exact owners

1. `src/stage1/program_json_v0/authority.rs`
   - strict source-authority core
   - frozen, not the active front
2. `crates/nyash_kernel/src/plugin/module_string_dispatch/build_surrogate.rs`
   - compiled-stage1 keep
   - near thin floor, frozen
3. `src/runner/stage1_bridge/program_json/mod.rs`
4. `src/runner/stage1_bridge/program_json_entry/mod.rs`
5. `src/runner/stage1_bridge/program_json_entry/request.rs`
   - future-retire bridge cluster
   - near thin floor, frozen
6. `src/host_providers/mir_builder/lowering.rs`
   - test-only evidence seam
   - not a live blocker surface

#### already closeout-ready / do not reopen

1. `lang/src/mir/builder/MirBuilderBox.hako`
2. `lang/src/runner/{stage1_cli_env.hako,stage1_cli.hako,launcher.hako}`
3. `lang/src/compiler/build/build_box.hako`
4. `tools/hakorune_emit_mir.sh`
5. `tools/selfhost/selfhost_build.sh`
6. `tools/smokes/v2/lib/test_runner.sh`

### 3.2 Runtime lane

#### monitor-only / explicit keep

1. `src/backend/mir_interpreter/**`
   - regular Rust VM / interpreter keep
   - no longer the final proof owner for backend-zero runtime proof
   - remains the main Rust runtime execution substrate outside `.hako VM`
2. `src/runtime/**`
   - runtime bridge / ABI / host-side glue keep
   - includes `src/runtime/mirbuilder_emit.rs` which still calls the Rust host stop-line directly
3. portability / build scaffolding lane
   - Windows/macOS guards and ABI/build support remain in Rust by policy
   - tracked by `de-rust-post-g1-runtime-plan-ssot.md`

#### landed / not current blocker

1. `lang/src/vm/**`
   - `.hako VM` final proof owner for phase-29ck backend runtime proof
   - monitor-only, not the current blocker
2. `phase-29y`
   - blocker is `none`
   - failure-driven reopen only

#### runtime-zero meaning of “remaining Rust”

1. remaining Rust is mostly:
   - regular VM keep
   - ABI / runner / portability keep
   - runtime/plugin source-zero follow-up not yet promoted as active blocker
2. runtime-zero is closer than backend-zero, but still not “0 Rust now”

### 3.3 Backend lane

#### active queued backend-zero buckets

1. `crates/nyash-llvm-compiler/**`
   - current `ny-llvmc` CLI / normalize / static-first link owner
   - `src/native_driver.rs` is temporary seam only
   - final owner target is not Rust CLI, but thin backend boundary
2. `src/runner/modes/common_util/exec.rs`
   - backend selector / child-process launch owner
   - latest tightening: `ny-llvmc` EXE routes now read as `MIR JSON emit -> command build -> shared ny-llvmc invoke`
3. `src/runner/modes/llvm/{mod.rs,harness_executor.rs,object_emitter.rs,mir_compiler.rs,pyvm_executor.rs,fallback_executor.rs,error.rs,report.rs,plugin_init.rs,using_resolver.rs,method_id_injector.rs,exit_reporter.rs}`
   - Rust runner glue / route selection / diagnostics keep
   - still mainline-owned for LLVM route orchestration
4. `src/llvm_py/llvm_builder.py`
   - Python mainline entry/orchestration owner for `MIR -> object`
   - still owns the broadest llvmlite-facing route
5. `src/llvm_py/mir_reader.py`
   - Python MIR ingest / decode owner
6. `src/llvm_py/{build_ctx.py,build_opts.py}`
   - Python build-context / opt-level / output-shape owner
7. `src/llvm_py/instructions/**`
   - Python opcode-lowering / emit owner
8. `src/llvm_py/{builders/**,resolver.py,mir_analysis.py,phi_manager.py,phi_placement.py,phi_wiring/**,type_facts.py}`
   - Python lowering-analysis support owners
   - likely to become compat/canary keep earlier than the mainline entry path
9. `tools/llvmlite_harness.py`
   - Python harness/CLI glue owner
   - still Python-owned mainline for `MIR -> object`

#### official future target / non-Rust boundary

1. `lang/src/shared/backend/llvm_backend_box.hako`
2. `lang/c-abi/include/hako_aot.h`
3. `lang/c-abi/shims/{hako_aot.c,hako_aot_shared_impl.inc,hako_llvmc_ffi.c}`
   - official thin backend boundary
   - final shape is `.hako -> thin backend C ABI/plugin boundary -> object/exe`
4. `lang/src/shared/host_bridge/codegen_bridge_box.hako`
   - temporary bridge owner only
   - do not treat as final daily caller stop-point

#### compat / archive keep

1. `lang/src/llvm_ir/boxes/aot_prep.hako`
2. `lang/src/llvm_ir/boxes/aot_prep/**`
3. `lang/src/llvm_ir/boxes/normalize/**`
4. `lang/src/llvm_ir/emit/LLVMEmitBox.hako`
   - live compat keep, not daily owner
5. `lang/src/llvm_ir/archive/legacy_script_builder/**`
   - archive keep
6. `HAKO_CAPI_PURE=1` routes
   - compat-only pure pack, not mainline proof owner

#### explicit keep

1. `src/backend/cranelift/**`
2. `src/jit/**`
   - explicit keep
   - not part of backend-zero replace work
3. direct `.hako` CodegenBridge callers outside the official boundary
   - `lang/src/runner/launcher.hako`
     - mainline move-now candidate; currently bypasses `LlvmBackendBox`
   - `lang/src/runner/stage1_cli.hako`
     - compat keep for now; not the first daily caller migration
   - `lang/src/vm/hakorune-vm/extern_provider.hako`
     - explicit keep / runtime-side bridge surface

## 4. Fixed Remaining Order

1. compiler:
   - finish the last active Rust stop-line wave at `src/host_providers/mir_builder.rs`
   - then formal-close `phase-29cj`
2. runtime:
   - keep `phase-29y` monitor-only unless a runtime proof gate fails
   - do not invent a new runtime blocker while compiler stop-line is still active
3. backend:
   - continue `phase-29ck` as queued active phase
   - next ownership move is not “grow Rust native_driver more”, but “move daily caller ownership toward `LlvmBackendBox` / `hako_aot`”

## 5. Practical Meaning of 0 Rust

### 5.1 Near-term

- remove Rust as the authority owner for compiler selfhost
- keep Rust where it is still an explicit bridge / portability / execution substrate

### 5.2 Longer-term

- runtime-zero:
  - shrink regular Rust VM / runtime meaning to portability / scaffolding residue
- backend-zero:
  - demote `ny-llvmc` / runner glue / llvmlite from mainline owner
  - keep `.hako -> thin backend boundary` as the daily route

## 6. Non-goals

1. declaring backend-zero or runtime-zero as current blocker right now
2. treating `native_driver.rs` as final owner
3. treating `Cranelift keep` as de-Rust replacement work
4. reopening frozen bridge / surrogate / `.hako` helper waves just because they still exist
