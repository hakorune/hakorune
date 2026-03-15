---
Status: SSOT
Decision: accepted
Date: 2026-03-15
Scope: `full Rust 0` の remaining Rust/Python inventory を、compiler / runtime / backend の fixed-order task pack に落として迷走を防ぐ。
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/design/de-rust-full-rust-zero-remaining-rust-inventory-ssot.md
  - docs/development/current/main/design/de-rust-full-rust-zero-roadmap-ssot.md
  - docs/development/current/main/phases/phase-29cj/README.md
  - docs/development/current/main/phases/phase-29ck/README.md
  - docs/development/current/main/phases/phase-29y/60-NEXT-TASK-PLAN.md
---

# De-Rust Full Rust 0 Remaining Rust Task Pack (SSOT)

## Purpose

- remaining Rust/Python residue を「どこにあるか」だけでなく、「どの順で削るか」まで固定する。
- immediate compiler blocker と queued runtime/backend work を同じ優先度で混ぜない。
- `1 owner-local wave = 1 task pack row` の形で次の slice を決めやすくする。

## 1. Priority Lock

1. first priority:
   - compiler stop-line closeout
2. second priority:
   - backend-zero daily-owner cutover prep
3. third priority:
   - runtime-zero reopen only when lane C / source-zero gates fail

rule:
- runtime/backend の inventory は visibility を上げるために持つ。
- ただし compiler stop-line が active な間は、runtime/backend を current blocker に昇格しない。

## 2. Compiler Task Pack

### C0. phase-29cj close sync

- owner:
  - `src/host_providers/mir_builder.rs`
- exact target:
  - `module_to_mir_json(...)` 周辺の same-file handoff/finalize leaf だけ
- done shape:
  - `phase-29cj` wording/status lock is `formal close-sync-ready`
  - the remaining live Rust stop-line is concentrated in `src/host_providers/mir_builder.rs`, with targeted proof centered on `module_to_mir_json(...)`
- acceptance:
  - `cargo test mir_builder -- --nocapture`
- status:
  - landed / frozen; do not reopen bridge-surrogate-helper waves for additional local thinning

### C1. strict source-authority freeze confirmation

- owners:
  - `src/stage1/program_json_v0/authority.rs`
  - `crates/nyash_kernel/src/plugin/module_string_dispatch/build_surrogate.rs`
- exact target:
  - reopen せず frozen exact owner として close sync へ持ち込む
- done shape:
  - compiler lane の “remaining Rust” が stop-line だけに縮む
  - not deletion; frozen exact owners remain present until a later phase removes them
- status:
  - landed / frozen

### C2. post-phase-29cj authority replacement promotion

- next owner wave:
  1. `lang/src/mir/builder/MirBuilderBox.hako`
  2. `lang/src/runner/{stage1_cli_env.hako,stage1_cli.hako,launcher.hako}`
  3. `lang/src/compiler/build/build_box.hako`
- exact target:
  - Rust stop-line above the `.hako` authority wave を formal に切り替える
- note:
  - local helper thinning wave は already closeout-ready なので reopen しない

## 3. Backend Task Pack

### B0. backend-zero ownership demotion inventory

- current owners:
  - Rust:
    - `crates/nyash-llvm-compiler/**`
    - `src/runner/modes/common_util/exec.rs`
    - `src/runner/modes/llvm/**`
  - Python:
    - `src/llvm_py/**`
    - `tools/llvmlite_harness.py`
- target:
  - Rust/Python mainline owner -> thin backend boundary owner
- done shape:
  - backend-zero queue is described as ownership moves, not just native canaries
  - each remaining owner is classified as
    - Rust mainline keep to demote
    - Python mainline keep to demote
    - thin boundary target
    - compat/archive keep
- current active front:
  - `crates/nyash-llvm-compiler/src/main.rs`
  - latest tightening keeps harness-path resolution, object-output resolution, input temp/normalize ownership, compile-mode diagnostics, and emit finalize output behind same-file helpers, and top-level route order now dispatches through `run_dummy_mode(...)` / `run_compile_mode(...)` instead of inline `main()` logic
  - nearby runner-owner tightening now keeps `src/runner/modes/llvm/harness_executor.rs`, `src/runner/modes/llvm/object_emitter.rs`, and `src/runner/modes/llvm/mod.rs` on the same boundary-dispatch shape through owner-local helpers, including the object-only route gate/tails, without changing the current harness/object contract
  - nearby runner-owner tightening also keeps `src/runner/modes/common_util/exec.rs` lib/bin MIR JSON emit + ny-llvmc EXE launch behind shared helper `emit_json_and_run_ny_llvmc_emit_exe(...)`
  - nearby runner-owner tightening now keeps `src/runner/modes/llvm/harness_executor.rs` on the same `guard -> emit -> run` shape through owner-local helpers, without changing the harness route contract

### B1. daily caller cutover prep

- target owners:
  - `lang/src/shared/backend/llvm_backend_box.hako`
  - `lang/c-abi/include/hako_aot.h`
  - `lang/c-abi/shims/hako_aot.c`
- exact work:
  - file-path / temp ownership / diagnostics / arg plumbing contract freeze
  - runner docs must say daily caller target is this boundary, not `native_driver.rs`
- landed first slice:
  - `LlvmBackendBox.link_exe(obj_path, out_path, libs)` now forwards `libs` as the third `env.codegen.link_object` arg
  - vm-hako / regular-VM `env.codegen.link_object` handlers now accept `[obj_path, exe_out?, extra_ldflags?]`
  - canonical encoding remains `libs -> single extra_ldflags string`
  - empty `libs` still falls back to `HAKO_AOT_LDFLAGS` under the C boundary
- remaining fixed order inside B1:
  1. `B1a` temporary bridge freeze
     - freeze `lang/src/shared/host_bridge/codegen_bridge_box.hako` as temporary bridge owner only
     - do not let docs or new callers treat it as the final daily boundary
  2. `B1b` daily caller stop-point unification
     - move mainline direct caller `lang/src/runner/launcher.hako` from `CodegenBridgeBox` to `LlvmBackendBox`
     - keep `lang/src/runner/stage1_cli.hako` as compat keep until after launcher migration
  3. `B1c` compile contract lock
     - freeze normalized JSON temp ownership and object-output temp ownership between `LlvmBackendBox` and `hako_aot`
     - remove ambiguity between `compile_obj(json_path)` and `hako_aot_compile_json(json_in, obj_out, ...)`
  4. `B1d` env truth lock
     - unify `NYASH_LLVM_COMPILER` / `NYASH_NY_LLVM_COMPILER` wording and boundary docs
- acceptance anchor:
  - `tools/smokes/v2/profiles/integration/apps/phase29ck_llvm_backend_box_capi_link_min.sh`
  - `tools/smokes/v2/profiles/integration/apps/phase29ck_vmhako_llvm_backend_runtime_proof.sh`

### B2. Rust CLI / runner glue demotion

- target owners:
  - `crates/nyash-llvm-compiler/src/main.rs`
  - `crates/nyash-llvm-compiler/src/native_driver.rs`
  - `src/runner/modes/common_util/exec.rs`
  - `src/runner/modes/llvm/**`
- exact work:
  - turn `native_driver.rs` into canary-only
  - reduce Rust runner glue to boundary invocation / diagnostics only
- done shape:
  - Rust is no longer the daily backend owner

### B3. Python owner demotion

- target owners:
  - `src/llvm_py/llvm_builder.py`
  - `src/llvm_py/mir_reader.py`
  - `src/llvm_py/build_ctx.py`
  - `src/llvm_py/instructions/**`
  - `tools/llvmlite_harness.py`
- exact work:
  - move mainline `MIR -> object` ownership away from llvmlite
  - keep Python only as compat/canary lane until retired
- remaining fixed order inside B3:
  1. `B3a` harness/entry demotion inventory
     - classify `tools/llvmlite_harness.py` and `src/llvm_py/llvm_builder.py` as entry/orchestration owners
     - define which one remains compat-only first
  2. `B3b` MIR ingest/context demotion inventory
     - classify `src/llvm_py/mir_reader.py` and `src/llvm_py/{build_ctx.py,build_opts.py}` as decode/context owners
  3. `B3c` opcode-lowering demotion inventory
     - classify `src/llvm_py/instructions/**` as opcode owner pack
     - split “mainline must replace” from “compat canary can remain”
  4. `B3d` analysis/support demotion inventory
     - classify `src/llvm_py/{builders/**,resolver.py,mir_analysis.py,phi_manager.py,phi_placement.py,phi_wiring/**,type_facts.py}`
     - prefer early compat/canary demotion instead of treating the whole tree as one owner
- done shape:
  - Python is no longer mainline backend owner

### B4. Compat/legacy pack retirement

- target owners:
  - `lang/src/llvm_ir/boxes/aot_prep/**`
  - `lang/src/llvm_ir/boxes/normalize/**`
  - `lang/src/llvm_ir/emit/LLVMEmitBox.hako`
  - `lang/src/llvm_ir/archive/legacy_script_builder/**`
  - `HAKO_CAPI_PURE=1` pure-pack routes
- exact work:
  - keep explicit compat names until daily route no longer depends on them
  - do not use them as acceptance owner for backend-zero

## 4. Runtime Task Pack

### R0. monitor-only keep

- owners:
  - `phase-29y`
  - `lang/src/vm/**`
- rule:
  - no new runtime blocker while compiler stop-line is active and lane C gates stay green

### R1. remaining Rust runtime inventory lock

- target owners:
  - `src/backend/mir_interpreter/**`
  - `src/runtime/**`
- exact work:
  - split “regular VM keep” from “runtime-zero must eventually shrink”
  - keep portability/build-scaffolding paths explicit

### R2. source-zero reopen trigger pack

- reopen only when:
  - lane C gate fails
  - runtime/plugin source-zero task is explicitly promoted
- target:
  - move runtime meaning out of Rust, not just runtime proof ownership

## 5. Fixed Order

1. `C0`
2. `C1`
3. `C2`
4. `B0`
5. `B1`
6. `B2`
7. `B3`
8. `B4`
9. `R0` / `R1` remain monitor-only documentation unless reopen trigger fires

## 6. Non-goals

1. turning runtime-zero or backend-zero into immediate blocker while compiler stop-line is still active
2. calling `native_driver.rs` the final backend owner
3. counting `Cranelift keep` as a de-Rust migration task
4. reopening bridge/surrogate/helper waves that are already frozen
