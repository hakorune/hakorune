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
  - docs/development/current/main/phases/phase-29cl/README.md
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
     - landed: `lang/src/runner/launcher.hako` moved from direct `CodegenBridgeBox` build-exe calls to `LlvmBackendBox`
     - keep `lang/src/runner/stage1_cli.hako` as compat keep until after launcher migration
     - follow-up: launcher Program(JSON)->MIR now preserves `user_box_decls`, and compiled-stage1 module dispatch now carries temporary `selfhost.shared.backend.llvm_backend::{compile_obj,link_exe}` surrogate handling
     - retire the old `Unknown Box type: HakoCli` and `LlvmBackendBox.compile_obj failed` blockers; keep `stage1_cli.hako` as the visible compat keep while B1c/B1d lock the boundary contract
  3. `B1c` compile contract lock
     - freeze normalized JSON temp ownership and object-output temp ownership between `LlvmBackendBox` and `hako_aot`
     - remove ambiguity between `compile_obj(json_path)` and `hako_aot_compile_json(json_in, obj_out, ...)`
     - landed: daily compile owner is now path-based `CodegenBridgeBox.compile_json_path_args(...)`, and Rust boundary normalization moved into `src/host_providers/llvm_codegen.rs::normalize_mir_json_for_backend(...)`
  4. `B1d` env truth lock
     - unify `NYASH_LLVM_COMPILER` / `NYASH_NY_LLVM_COMPILER` wording and boundary docs
     - landed: `NYASH_NY_LLVM_COMPILER` is the ny-llvmc path truth; `NYASH_LLVM_COMPILER` remains `tools/build_llvm.sh` selector only
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
     - landed first shell demotion:
       - `tools/llvmlite_harness.py` keeps repo-root bootstrap, CLI parse, and `runpy` delegation behind owner-local helpers
       - `src/llvm_py/llvm_builder.py` keeps CLI parse, MIR file load, and output-file write behind owner-local helpers
       - `NyashLLVMBuilder` / lowering/support stay out of scope for B3a
  2. `B3b` MIR ingest/context demotion inventory
     - classify `src/llvm_py/mir_reader.py` and `src/llvm_py/{build_ctx.py,build_opts.py}` as decode/context owners
     - landed first slice:
       - `src/llvm_py/mir_reader.py` now owns normalized `BuilderInput` ingest for `llvm_builder.py`
       - `src/llvm_py/build_opts.py` now owns `BuildOptions` env/codegen context for object emission
       - `src/llvm_py/build_ctx.py` now owns lowering-side aggregated context through `build_ctx_from_owner(...)`, consumed by `src/llvm_py/builders/instruction_lower.py`
  3. `B3c` opcode-lowering demotion inventory
     - classify `src/llvm_py/instructions/**` as opcode owner pack
     - split “mainline must replace” from “compat canary can remain”
     - landed first slice:
       - generic by-name method fallback now lives in `src/llvm_py/instructions/by_name_method.py`
       - `boxcall.py`, `mir_call/method_call.py`, and `mir_call_legacy.py` no longer each own their own `nyash.plugin.invoke_by_name_i64` wiring
     - landed second slice:
       - `src/llvm_py/instructions/boxcall_runtime_data.py` now owns collection/runtime-data style `size/get/push/set/has` lowering for generic BoxCall
       - `src/llvm_py/instructions/boxcall.py` now consumes the shared helper instead of keeping the collection route table inline
     - landed third slice:
       - `src/llvm_py/instructions/mir_call/collection_method_call.py` now owns shared `get/push/set/has` route order
       - `src/llvm_py/instructions/mir_call/method_call.py` and `src/llvm_py/instructions/mir_call_legacy.py` now consume that helper instead of carrying duplicate collection tails
     - landed fourth slice:
       - `src/llvm_py/instructions/mir_call/method_fallback_tail.py` now owns the final `direct known-box -> by-name plugin` route order
       - `src/llvm_py/instructions/mir_call/method_call.py` and `src/llvm_py/instructions/mir_call_legacy.py` now consume that helper instead of carrying duplicate fallback tails
     - landed fifth slice:
       - `src/llvm_py/instructions/mir_call/string_console_method_call.py` now owns shared `substring/indexOf/lastIndexOf/log` route order
       - `src/llvm_py/instructions/mir_call/method_call.py` and `src/llvm_py/instructions/mir_call_legacy.py` now consume that helper instead of carrying duplicate string/console branches
       - `length/size` specialization intentionally stays owner-local to `method_call.py`
  4. `B3d` analysis/support demotion inventory
     - classify `src/llvm_py/{builders/**,resolver.py,mir_analysis.py,phi_manager.py,phi_placement.py,phi_wiring/**,type_facts.py}`
     - prefer early compat/canary demotion instead of treating the whole tree as one owner
     - landed first slice:
       - `src/llvm_py/build_ctx.py` now owns `current_vmap` / `lower_ctx` as part of the lowering-side aggregated context
       - `src/llvm_py/builders/instruction_lower.py` now consumes those seams instead of reading `_current_vmap` / `ctx` off the builder owner inline
     - landed second slice:
       - `src/llvm_py/type_facts.py` now owns shared `StringBox` / `ArrayBox` fact predicates and handle-fact construction
       - `src/llvm_py/resolver.py` now consumes those helpers through owner-local `value_types` accessors instead of keeping ad-hoc fact-shape checks inline
       - proof is pinned by `src/llvm_py/tests/test_resolver_type_tags.py` and `src/llvm_py/tests/test_type_facts.py`
     - landed third slice:
       - `src/llvm_py/phi_manager.py` now owns cross-block safety helpers for global-safe / PHI-owner / single-def dominance checks
       - `filter_vmap_preserve_phis(...)` now consumes those helpers instead of keeping all dominance cases inline
       - proof is pinned by `src/llvm_py/tests/test_phi_manager_snapshot_filter.py`
     - landed fourth slice:
       - `src/llvm_py/mir_analysis.py` now owns helper-local const-string scan and call-arity record helpers
       - `scan_call_arities(...)` now consumes those helpers instead of mixing seed collection and max-arity update inline
       - proof is pinned by `src/llvm_py/tests/test_mir_analysis.py`
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

### B5. by-name retirement cutover

- phase owner:
  - `docs/development/current/main/phases/phase-29cl/README.md`
- exact work:
  - lock `no-new-mainline` on `nyash.plugin.invoke_by_name_i64`
  - move visible daily callers off module-string / method-name by-name routes before kernel delete
  - shrink compiled-stage1 surrogates only after replacement proof exists
  - demote hook/registry residue to compat-only
- landed first slice:
  - `tools/checks/phase29cl_by_name_mainline_guard.sh`
  - `tools/checks/phase29cl_by_name_mainline_allowlist.txt`
  - `tools/smokes/v2/profiles/integration/apps/phase29cl_by_name_lock_vm.sh`
- current worker-backed inventory:
  - kernel entry owner: `crates/nyash_kernel/src/plugin/invoke/by_name.rs`
  - upstream daily caller/dependency pack:
    - `src/llvm_py/instructions/mir_call/method_call.py`
    - `src/backend/mir_interpreter/handlers/calls/method.rs`
    - `src/runtime/type_registry.rs`
    - `src/backend/wasm_v2/unified_dispatch.rs`
  - compiled-stage1 temporary keeps:
    - `crates/nyash_kernel/src/plugin/module_string_dispatch.rs`
    - `crates/nyash_kernel/src/plugin/module_string_dispatch/build_surrogate.rs`
    - `crates/nyash_kernel/src/plugin/module_string_dispatch/llvm_backend_surrogate.rs`
- done shape:
  - `by_name` is no longer a daily owner
  - remaining `by_name` surface is explicit compat/archive only
- rule:
  - do not mix this pack with `phase-29ce`; frontend fixture-key / semantic by-name retirement stays there

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
9. `B5`
10. `R0` / `R1` remain monitor-only documentation unless reopen trigger fires

## 6. Non-goals

1. turning runtime-zero or backend-zero into immediate blocker while compiler stop-line is still active
2. calling `native_driver.rs` the final backend owner
3. counting `Cranelift keep` as a de-Rust migration task
4. reopening bridge/surrogate/helper waves that are already frozen
