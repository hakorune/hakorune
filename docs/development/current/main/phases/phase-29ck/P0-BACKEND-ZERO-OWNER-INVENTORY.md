---
Status: SSOT
Decision: accepted
Date: 2026-03-14
Scope: backend-zero の current owner / keep / replace target を exact path で固定し、次の task pack が迷走しないようにする。
Related:
  - docs/development/current/main/phases/phase-29ck/README.md
  - docs/development/current/main/design/de-rust-backend-zero-boundary-lock-ssot.md
  - docs/development/current/main/design/de-rust-backend-zero-provisional-inventory-ssot.md
  - docs/reference/abi/ABI_BOUNDARY_MATRIX.md
  - crates/nyash-llvm-compiler/src/main.rs
  - src/runner/modes/common_util/exec.rs
---

# P0: Backend-Zero Owner Inventory

## Purpose

- `backend-zero` で何を replace し、何を keep するかを exact path で固定する。
- bootstrap seam と final `.hako -> thin backend boundary` target を同じ表で読み分けられるようにする。
- `Cranelift keep` と `ABI 2 面固定` を phase 冒頭でロックする。

## 1. Current Observable Facts

1. `crates/nyash-llvm-compiler/src/main.rs` は自分自身を backend helper CLI として名乗るべきで、`llvmlite harness wrapper` wording は demotion target である
2. `run_harness_in(...)` / `run_harness_dummy(...)` は `python3` を必須にしている
3. `link_executable(...)` はすでに native/static-first link line を owner している
4. canonical ABI surface は引き続き次の 2 面だけである
   - Core C ABI
   - TypeBox ABI v2
5. future caller boundary seeds はすでに存在する
   - `lang/src/shared/backend/llvm_backend_box.hako`
   - `lang/c-abi/shims/hako_aot.c`

## 2. Owner Matrix (fixed)

| surface | exact paths | classification | note |
| --- | --- | --- | --- |
| LLVM object emission (`MIR -> .o`) | mainline: `tools/llvmlite_harness.py`, `src/llvm_py/**`; bootstrap seam: `crates/nyash-llvm-compiler/src/native_driver.rs` | primary replace target | `native_driver.rs` は canary owner だが final owner ではない |
| thin backend cutover boundary | `lang/src/shared/backend/llvm_backend_box.hako`, `lang/c-abi/shims/hako_aot.c` | future mainline target | `.hako` call-site と C helper に寄せる。第 3 canonical ABI は作らない |
| `ny-llvmc` CLI / stdin normalize / canary normalize / hint / opt-level env | `crates/nyash-llvm-compiler/src/main.rs` | keep while growing | internal tool / helper keep。caller-owned final boundary ではない |
| native EXE link (`.o + libnyash_kernel.a`) | `crates/nyash-llvm-compiler/src/main.rs`, `src/runner/modes/common_util/exec.rs` | explicit keep | static-first link boundary は維持する。runner-side native opt-in selector もここに閉じる |
| LLVM runner route glue | `src/runner/modes/llvm/**` | thin keep | route selection / child process / diagnostics |
| runtime ABI surface | `docs/reference/abi/ABI_BOUNDARY_MATRIX.md`, `include/nyrt.h`, `include/nyash_abi.h` | explicit keep | backend 専用の第 3 ABI は作らない |
| Cranelift backend | `src/backend/cranelift/**`, `src/jit/**` | explicit keep | この phase では置換対象にしない |
| current harness route select | `NYASH_LLVM_USE_HARNESS` plus caller docs/scripts | compat keep until demotion | silent semantics change は禁止 |

## 2.5 Ownership Split (B0 detail lock)

### 2.5.1 Rust mainline owners to demote

1. `crates/nyash-llvm-compiler/src/main.rs`
   - current CLI / normalize / static-first link owner
2. `crates/nyash-llvm-compiler/src/native_driver.rs`
   - bootstrap seam only
3. `src/runner/modes/common_util/exec.rs`
   - runner-side backend selector / child-process launch owner
   - latest tightening: lib/bin EXE routes now share `run_ny_llvmc_emit_exe(...)`, and the invoke path itself is split into `build_ny_llvmc_emit_exe_command(...)` -> `spawn_ny_llvmc_emit_exe_command(...)`, so this owner reads as `MIR JSON emit -> command build -> shared ny-llvmc invoke`
4. `src/runner/modes/llvm/{mod.rs,harness_executor.rs,object_emitter.rs,mir_compiler.rs,pyvm_executor.rs,fallback_executor.rs,error.rs,report.rs,plugin_init.rs,using_resolver.rs,method_id_injector.rs,exit_reporter.rs}`
   - route selection / diagnostics / fallback glue keep
   - latest tightening: `object_emitter.rs` now reaches `src/host_providers/llvm_codegen.rs::mir_json_to_object(...)` without pinning `llvmlite`; explicit `llvmlite` selection stays compat keep behind `HAKO_LLVM_EMIT_PROVIDER=llvmlite`, instead of the runner owning a direct llvmlite harness call
5. `lang/src/shared/backend/llvm_backend_box.hako`
   - latest tightening: daily compile/link owner now stops directly at canonical `env.codegen.compile_json_path(...)` / `env.codegen.link_object(...)`; `CodegenBridgeBox` remains temporary compat helper only

### 2.5.2 Python mainline owners to demote

1. `tools/llvmlite_harness.py`
   - current harness entry
2. `src/llvm_py/llvm_builder.py`
   - current codegen / pass owner
3. `src/llvm_py/{mir_reader.py,build_ctx.py,resolver.py,mir_analysis.py,build_opts.py,phi_manager.py,phi_placement.py,type_facts.py}`
   - reader / build context / analysis support owner
4. `src/llvm_py/instructions/**`
   - instruction-lowering owner

### 2.5.3 Thin backend boundary targets

1. `lang/src/shared/backend/llvm_backend_box.hako`
   - `.hako` caller boundary
2. `lang/c-abi/include/hako_aot.h`
   - canonical C header
3. `lang/c-abi/shims/{hako_aot.c,hako_diag_mem_shared_impl.inc,hako_aot_shared_impl.inc,hako_llvmc_ffi.c}`
   - thin helper / shared TLS+memory foundation / compile-link source truth / optional FFI support

### 2.5.4 Compat / archive keeps

1. `lang/src/llvm_ir/boxes/{aot_prep.hako,normalize/**}`
2. `lang/src/llvm_ir/emit/LLVMEmitBox.hako`
3. `lang/src/llvm_ir/archive/legacy_script_builder/**`
4. `HAKO_CAPI_PURE=1` historical pure-lowering routes
5. `NYASH_LLVM_USE_HARNESS` caller-side compat route select

### 2.5.5 Explicit keep

1. `src/backend/cranelift/**`
2. `src/jit/**`
   - outside backend-zero replace work

## 3. First Code Slice Boundary

- first implementation owner は bootstrap seam として `crates/nyash-llvm-compiler/src/**` に閉じてよい
- default route をいきなり切り替えない
- final ownership target は `lang/src/shared/backend/llvm_backend_box.hako` / `lang/c-abi/**` である
- `src/runner/modes/llvm/**` や `.hako` builder line は、bootstrap evidence 前に reopen しない

## 4. Explicit Keep Locks

### 4.1 ABI lock

- Core C ABI は runtime/bootstrap/load/exec の canonical surface
- TypeBox ABI v2 は plugin Box method dispatch の canonical surface
- backend-zero のために第 3 ABI を増やさない

### 4.2 Cranelift lock

- `Cranelift` は keep
- backend-zero の blocker / acceptance / demotion 判定に含めない

### 4.3 Link boundary lock

- native executable link は既存の static-first line を再利用する
- `link_executable(...)` を main target にしない

### 4.4 Temporary seam lock

- `crates/nyash-llvm-compiler/src/native_driver.rs` は bootstrap seam only
- final owner として docs に書かない
- thin backend boundary が landed したら canary-only か retire 対象にする

## 5. Downstream, but not first

次は phase の中に含まれるが、first code slice ではない。

1. `.hako` builder / selfhost caller parity via thin backend boundary
2. backend-specific optimization handoff
3. llvmlite demotion

## 6. Not in P0

- acceptance command の最終 lock
- blocker 昇格条件
- optimization の owner handoff 実行
