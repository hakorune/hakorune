---
Status: SSOT
Decision: accepted
Date: 2026-03-15
Scope: kernel/plugin/backend boundary に残る `by_name` residue を owner 単位で分類し、delete 順の前に current role を固定する。
Related:
  - docs/development/current/main/phases/phase-29cl/README.md
  - CURRENT_TASK.md
  - docs/development/current/main/design/de-rust-full-rust-zero-remaining-rust-inventory-ssot.md
  - crates/nyash_kernel/src/plugin/invoke/by_name.rs
  - crates/nyash_kernel/src/plugin/module_string_dispatch.rs
---

# P0: By-Name Owner Inventory

Note:
- This inventory was accepted while Python-side generic by-name emitters were still live.
- `src/llvm_py/instructions/boxcall.py`, `src/llvm_py/instructions/mir_call/method_call.py`, and `src/llvm_py/instructions/mir_call_legacy.py` have since been fail-fast retired on the Python side.
- `crates/nyash_kernel/src/plugin/invoke/by_name.rs` and the public `nyash_plugin_invoke_by_name_i64` export have since been retired.
- The remaining live residue is hook-bridge compat glue for future spawn / string dispatch only.

## 1. Mainline Owners To Demote

1. `crates/nyash_kernel/src/plugin/invoke/by_name.rs`
   - current exported `nyash.plugin.invoke_by_name_i64`
   - still resolves named receiver + method strings in the live kernel surface
2. `src/llvm_py/instructions/mir_call/method_call.py`
   - current llvmlite `mir_call` lowering entry that still emits `nyash.plugin.invoke_by_name_i64` on fallback method dispatch
3. `src/backend/mir_interpreter/handlers/calls/method.rs`
   - regular VM still resolves method slots by name before execution
4. `src/runtime/type_registry.rs`
   - current runtime SSOT for `type + method_name + arity -> slot`
5. `src/backend/wasm_v2/unified_dispatch.rs`
   - WASM v2 also still relies on `resolve_slot_by_name(...)`

## 2. Compiled-Stage1 Temporary Keeps

1. `crates/nyash_kernel/src/plugin/module_string_dispatch.rs`
   - current compiled-stage1 string-module route table / probe layer
   - still probed from `by_name.rs` before generic named receiver dispatch
2. `crates/nyash_kernel/src/plugin/module_string_dispatch/build_surrogate.rs`
   - temporary compiled-stage1 `BuildBox.emit_program_json_v0` surrogate
3. `crates/nyash_kernel/src/plugin/module_string_dispatch/llvm_backend_surrogate.rs`
   - temporary compiled-stage1 `selfhost.shared.backend.llvm_backend::{compile_obj,link_exe}` surrogate
4. `lang/src/vm/boxes/mir_call_v1_handler.hako`
   - observation-only `[vm/byname:*]` tag point under `HAKO_VM_DYN_FALLBACK=1`
   - not a final dispatch target

Rule:
- these are temporary proof owners only
- do not promote them into the final backend/runtime architecture

## 3. Compat Keeps

1. `crates/nyash_kernel/src/hako_forward_bridge.rs`
   - current hook registry / hook-miss policy glue for by-name entry
2. `crates/nyash_kernel/src/hako_forward.rs`
3. `crates/nyash_kernel/src/hako_forward_registry.c`
   - current C-side registration / bridge surface for by-name forwarding
4. `lang/c-abi/shims/hako_kernel.c`
   - current C shim registry surface for plugin invoke by name
5. `src/llvm_py/instructions/boxcall.py`
   - legacy `boxcall` lowering still emits `nyash.plugin.invoke_by_name_i64`
6. builtin `FileBox` named-method handling inside `crates/nyash_kernel/src/plugin/invoke/by_name.rs`
   - still a compat path while filebox call-shape migration is incomplete
7. `crates/nyash_kernel/src/tests.rs`
8. `crates/nyash_kernel/src/plugin/invoke/tests.rs`
   - regression coverage for current keep surface

## 4. Archive Candidate / Compat-Only Residue

1. `src/llvm_py/instructions/mir_call_legacy.py`
   - legacy fallback copy; no current mainline import should grow here
2. dynamic-fallback `by_name` path inside `lang/c-abi/shims/hako_llvmc_ffi.c`
   - compat-only / historical route, not a mainline acceptance owner

## 5. Migration Targets

1. plugin dispatch target:
   - TypeBox ABI v2
2. runtime/bootstrap target:
   - Core C ABI
3. backend target:
   - `.hako -> lang/src/shared/backend/llvm_backend_box.hako -> lang/c-abi/include/hako_aot.h / lang/c-abi/shims/hako_aot.c`

## 6. Non-goals

1. deleting `by_name.rs` before daily callers move
2. treating compiled-stage1 surrogates as permanent
3. mixing frontend fixture-key “by-name” history with this inventory
