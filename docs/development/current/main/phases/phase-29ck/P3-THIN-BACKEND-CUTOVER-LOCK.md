---
Status: SSOT
Decision: accepted
Date: 2026-03-14
Scope: backend-zero の cutover 前に official caller / official C owner / archive keep route / env keep-retire を 1 枚で固定する。
Related:
  - docs/development/current/main/design/de-rust-backend-zero-boundary-lock-ssot.md
  - docs/development/current/main/design/de-rust-backend-zero-provisional-inventory-ssot.md
  - docs/development/current/main/phases/phase-29ck/README.md
  - docs/development/current/main/phases/phase-29ck/P1-NY-LLVMC-NATIVE-EMITTER-TASK-PACK.md
  - docs/development/current/main/phases/phase-29ck/P2-NATIVE-EMITTER-ACCEPTANCE-AND-REOPEN-RULE.md
  - lang/src/shared/backend/llvm_backend_box.hako
  - lang/c-abi/include/hako_aot.h
  - lang/c-abi/shims/hako_aot.c
  - lang/src/llvm_ir/README.md
---

# P3: Thin Backend Cutover Lock

## Purpose

- BE0-min6 の前提を 1 枚で読めるようにする。
- official route と legacy route を混ぜない。
- `.hako` caller がどこで止まるか、C 側 helper がどこか、何を archive keep にするかを固定する。

## 1. Official Route

backend-zero の official caller route は次だけだよ。

1. `.hako` caller:
   - `lang/src/shared/backend/llvm_backend_box.hako`
2. thin C helper:
   - `lang/c-abi/include/hako_aot.h`
   - `lang/c-abi/shims/hako_aot.c`
3. backend helper internals:
   - `ny-llvmc`
   - optional FFI helper behind `hako_aot`

rule:
- new daily caller は `llvm_ir` 側へ増やさない
- `.hako` は raw LLVM API へ直接結合しない
- `native_driver.rs` は bootstrap seam only

## 2. Live Compat Keep

次は live compat keep として残す。

1. `lang/src/llvm_ir/boxes/aot_prep.hako`
2. `lang/src/llvm_ir/boxes/aot_prep/**`
3. `lang/src/llvm_ir/boxes/normalize/**`
4. `lang/src/llvm_ir/emit/LLVMEmitBox.hako`

これらは perf / normalize / emit canary 用の keep であり、backend-zero daily caller owner ではない。

## 3. Archived Legacy Route

次は archive keep に固定する。

1. `lang/src/llvm_ir/archive/legacy_script_builder/**`
   - historical script-builder boxes
   - historical `LLVMAotFacadeBox`
2. `lang/src/llvm_ir/archive/examples/**`
   - historical AotBox example

rule:
- new caller wiring をここへ追加しない
- archive path は history/compat 参照だけに使う
- 復活が必要なら phase docs を reopen してから扱う

## 4. Env Contract

### 4.1 Keep

- `NYASH_LLVM_USE_CAPI`
- `HAKO_V1_EXTERN_PROVIDER_C_ABI`
- `NYASH_LLVM_COMPILER`
- `NYASH_LLVM_BACKEND`
- `HAKO_AOT_USE_FFI`
- `HAKO_AOT_FFI_LIB`
- `HAKO_AOT_LDFLAGS`

### 4.2 Temporary Proof Env

- none
  - phase-29ck runtime proof is now pinned only by:
    - `NYASH_LLVM_USE_CAPI=1`
    - `HAKO_V1_EXTERN_PROVIDER_C_ABI=1`

### 4.3 Compat-only keep

- `HAKO_CAPI_PURE`
  - role:
    - legacy pure-lowering selector for phase2120 / historical helper routes
    - official compat-pack lock is `P5-COMPAT-PURE-PACK-LOCK.md`
  - non-goal:
    - do not treat as phase-29ck runtime-proof pin or long-term caller selector
  - retirement trigger:
    - retire once historical pure-lowering canaries and helper routes no longer need an explicit toggle
- `HAKO_LLVM_EMIT_PROVIDER`
  - `LLVMEmitBox` canary 用
  - thin backend cutover の selector にはしない

### 4.4 Retire / no-new-work

- `HAKO_AOT_USE_PLUGIN`
- `HAKO_LLVM_SCRIPT_BUILDER`
- `HAKO_LLVM_SCRIPT_BUILDER_STRICT`

rule:
- retire/no-new-work env で new mainline route を作らない
- new hidden env は足さない

## 5. First Cutover Shape

最初に固定する `.hako` 側 contract はこれだよ。

1. `compile_obj(json_path)`
   - input: MIR(JSON) file path
   - output: object path
2. `link_exe(obj_path, out_path, libs)`
   - input: object path / output exe path / optional libs
   - output: success/fail

補足:
- temp file ownership と error wording は `LlvmBackendBox` 実装 slice で lock する
- ただし owner path はこの文書で先に lock 済みとする
- landed first cut:
  - `lang/c-abi/include/hako_aot.h` is the canonical AOT compile/link header; `hako_hostbridge.h` keeps only thin-shim inclusion for those declarations
  - `lang/c-abi/shims/hako_aot_shared_impl.inc` is the shared compile/link source truth used by both `hako_aot.c` and `hako_kernel.c`
  - `LlvmBackendBox.compile_obj(json_path)` reads file content, injects `schema_version: "1.0"` via `MirV1MetaInjectBox`, then calls `CodegenBridgeBox.emit_object_args(...)`
  - `LlvmBackendBox.link_exe(obj_path, out_path, libs)` calls `CodegenBridgeBox.link_object_args(...)`
  - non-empty `libs` is now forwarded as the third `env.codegen.link_object` arg
  - current canonical encoding is `libs -> single extra_ldflags string`
  - empty `libs` still falls back to `HAKO_AOT_LDFLAGS` under `llvm_codegen::link_object_capi(...)` / `hako_aot_link_obj(...)`
- `.hako` surface parser does not accept `throw`, so failure contract is stable tag print (`[llvmbackend/*]`) + `null`
- current proof shape is:
    - direct MIR emit accepts a `.hako` caller that imports `selfhost.shared.backend.llvm_backend`
    - `LlvmBackendBox` source owner is pinned to `MirV1MetaInjectBox` + `CodegenBridgeBox.emit_object_args/link_object_args`
    - downstream native app parity stays green on `phase29ck_native_llvm_cabi_link_min.sh`
    - non-empty `libs` is pinned by `phase29ck_llvm_backend_box_capi_link_min.sh`
  - final runtime-proof owner is `.hako VM`, not regular VM
  - runtime proof through `LlvmBackendBox` itself is now pinned by `phase29ck_vmhako_llvm_backend_runtime_proof.sh` with non-empty `libs`

## 6. Temporary Seam Retirement

phase-29ck runtime proof で一時的に必要だった seam は retire 済みだよ。

1. retired seam
   - `src/backend/mir_interpreter/handlers/calls/method.rs`
   - `src/backend/mir_interpreter/handlers/boxes.rs`
   - retired shape:
     - receiver-less `hostbridge.extern_invoke`
     - placeholder `newbox(hostbridge)`
   - replacement:
     - `lang/src/vm/boxes/mir_vm_s0_boxcall_exec.hako` now uses owner-local helpers that lower to canonical `Callee::Extern(env.codegen.*)`
2. rule:
   - no new mainline behavior may reintroduce these seams
   - any widening requires reopening `phase-29ck`

## 7. Immediate Cleanup Rule

cutover 実装前に許される cleanup は次だけだよ。

1. legacy `.hako` route の archive 化
2. docs / README / layer guard の同期
3. duplicate truth を減らす準備

まだこの段階で混ぜないもの:

1. `native_driver.rs` の widening
2. llvmlite demotion
3. optimization handoff
4. Cranelift lane 変更
