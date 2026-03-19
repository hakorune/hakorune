---
Status: SSOT
Decision: provisional
Date: 2026-03-14
Scope: backend-zero の provisional inventory を固定しつつ、final target を `.hako -> thin backend boundary`、`native_driver.rs` を temporary seam として切り分ける。
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/10-Now.md
  - docs/development/current/main/design/de-rust-backend-zero-boundary-lock-ssot.md
  - docs/development/current/main/design/de-rust-backend-zero-fixed-order-and-buildability-ssot.md
  - docs/development/current/main/design/de-rust-full-rust-zero-roadmap-ssot.md
  - docs/development/current/main/phases/phase-29ck/README.md
  - docs/development/current/main/phases/phase-29ck/P1-NY-LLVMC-NATIVE-EMITTER-TASK-PACK.md
  - docs/development/current/main/phases/phase-29ck/P2-NATIVE-EMITTER-ACCEPTANCE-AND-REOPEN-RULE.md
  - docs/development/current/main/design/de-rust-lane-map-ssot.md
  - docs/reference/abi/ABI_BOUNDARY_MATRIX.md
  - docs/reference/plugin-abi/nyash_abi_v2.md
  - docs/development/current/main/phases/phase-29cc/29cc-253-source-zero-static-link-boundary-lock-ssot.md
  - docs/private/roadmap/phases/phase-21.10/PLAN.md
---

# De-Rust Backend-Zero Provisional Inventory (SSOT)

## Purpose

- backend-zero を曖昧な将来案ではなく、`keep` と `replace` を切り分けた provisional lane として固定する。
- `native_driver.rs` が accidental final owner にならないよう、final target を `.hako -> thin backend boundary` に固定する。
- `llvmlite` と `ny-llvmc` の責務を分け、最適化の主戦場を取り違えないようにする。
- execution-ready な phase/task pack は `phase-29ck` を正本にする。

## Execution Phase Pointer (2026-03-14)

1. architecture boundary:
   - `docs/development/current/main/design/de-rust-backend-zero-boundary-lock-ssot.md`
2. phase entry:
   - `docs/development/current/main/phases/phase-29ck/README.md`
3. task pack:
   - `docs/development/current/main/phases/phase-29ck/P1-NY-LLVMC-NATIVE-EMITTER-TASK-PACK.md`
4. acceptance / reopen rule:
   - `docs/development/current/main/phases/phase-29ck/P2-NATIVE-EMITTER-ACCEPTANCE-AND-REOPEN-RULE.md`

## 1. Boundary Lock

- この文書は backend-zero の provisional inventory であり、current blocker を上書きしない。
- canonical ABI surface は引き続き 2 面固定:
  - Core C ABI
  - TypeBox ABI v2
- TypeBox ABI v2 は plugin Box method dispatch 用であり、LLVM backend 本体の置換先にはしない。
- backend helper boundary が必要でも、それを第 3 canonical ABI にはしない。
- `.hako` 側の final caller target は `lang/src/shared/backend/llvm_backend_box.hako` / `lang/c-abi/**` の thin boundary であり、`native_driver.rs` ではない。
- `Cranelift` は keep とする。backend-zero の primary replace target ではない。

## 2. Current Snapshot (2026-03-14)

1. `llvmlite`:
   - current LLVM codegen / codegen optimization owner
   - MIR -> object の実体はここにある
2. `ny-llvmc`:
   - current wrapper / normalize / opt-level propagation / link owner
   - native canary owner は landed したが、これは bootstrap seam であり final owner ではない
3. `hakorune` LLVM route:
   - `hakorune -> ny-llvmc -> llvmlite -> object/exe`
   - app-seed opt-in parity is now green both from `tools/build_llvm.sh` and from direct `hakorune --backend llvm` runner glue when `NYASH_LLVM_BACKEND=native`
   - final caller target is still future: `.hako -> LlvmBackendBox / hako_aot -> backend helper -> object/exe`
4. `Cranelift`:
   - keep
   - backend-zero の blocker に含めない
5. legacy `.hako` script-builder route:
   - `lang/src/llvm_ir/archive/legacy_script_builder/**`
   - archive keep
   - daily caller target に戻さない

## 3. Ownership Matrix (fixed for now)

| surface | current owner | target status | note |
| --- | --- | --- | --- |
| LLVM object emission (`MIR -> .o`) | mainline: `src/llvm_py/**`; bootstrap seam: `crates/nyash-llvm-compiler/src/native_driver.rs` | replace target | `native_driver.rs` は temporary seam only。final owner ではない |
| thin backend cutover boundary | `lang/src/shared/backend/llvm_backend_box.hako`, `lang/c-abi/shims/hako_aot.c` | future mainline target | `.hako` から呼べる boundary。第 3 canonical ABI は作らない |
| legacy script-builder / AotFacade route | `lang/src/llvm_ir/archive/legacy_script_builder/**` | archive keep | new daily route は作らない。compat/history 用 |
| `ny-llvmc` CLI / normalize / hint / opt-level env | `crates/nyash-llvm-compiler/src/main.rs` | keep while growing | internal tool / helper keep。caller-owned final boundary ではない |
| EXE link (`.o + libnyash_kernel.a`) | `crates/nyash-llvm-compiler/src/main.rs`, `src/runner/modes/common_util/exec.rs` | keep | static-first boundary を維持 |
| LLVM route orchestration | `src/runner/modes/llvm/**`, `src/runner/modes/common_util/exec.rs` | thin keep | child process / route selection / diagnostics。native opt-in selector forwarding is now owner-local in `exec.rs` |
| Runtime ABI surface | Core C ABI / TypeBox ABI v2 | keep | backend 専用の第3 ABI は作らない |
| Cranelift backend | `src/backend/cranelift/`, `src/jit/**` | explicit keep | この lane では置換しない |
| `llvmlite` harness route | `tools/llvmlite_harness.py`, `src/llvm_py/**` | future compat/canary keep | thin boundary cutover 後に降格候補 |

## 4. Main Task Shape

### 4.1 Final target

- `.hako -> thin backend boundary -> object/exe`
- 定義:
  - `.hako` caller が `LlvmBackendBox` / `hako_aot` などの thin boundary を叩き、その内側で object/exe emission が完結する状態

### 4.2 Temporary bootstrap seam

- `crates/nyash-llvm-compiler/src/native_driver.rs`
- role:
  - Python 非依存 object/exe canary
  - runner/build glue parity の early evidence
- rule:
  - final owner と見なさない
  - thin backend boundary が landed したら canary-only か retire 対象にする

### 4.3 Not the primary task

- linker を作り直すこと
- TypeBox ABI に LLVM backend を載せること
- Cranelift を de-Rust 対象へ広げること

## 5. Fixed Order (high level)

1. inventory lock
   - current owner / keep / replace をこの文書で固定する
2. bootstrap seam evidence
   - `ny-llvmc --driver native` が llvmlite 非依存で最小 canary / runner parity を通す
3. thin backend boundary cutover
   - `LlvmBackendBox` / `hako_aot` / caller wiring を daily route に寄せる
4. native seam demotion
   - `native_driver.rs` を canary-only か retire にする
5. llvmlite demotion
   - mainline owner から外し、compat/canary route へ降格する

## 6. Optimization Placement Rule

- MIR semantics に閉じる最適化:
  - Rust MIR pass に置く
- LLVM lowering / codegen / pass tuning:
  - thin backend boundary cutover 完了までは `llvmlite` 側で行う
  - cutover 完了後は新規 backend-specific optimization を thin boundary の内側へ寄せる
- wrapper / CLI / link / diagnostics:
  - `ny-llvmc` 側で行う

## 7. Exit Condition

- `.hako` caller が thin backend boundary を daily route で使っている
- `native_driver.rs` が final owner ではなく canary-only か retired になっている
- `llvmlite` が mainline owner ではなく compat/canary owner に降格している
- static-first link boundary と Core C ABI / TypeBox ABI v2 の 2 面契約が維持されている
- `Cranelift keep` が docs 間で矛盾していない

## 8. Not in this doc

- current blocker を backend-zero に切り替えること
- `phase-29y` の runtime daily policy を変えること
- Cranelift の置換計画を作ること
