---
Status: SSOT
Decision: accepted
Date: 2026-03-14
Scope: backend-zero の bootstrap seam / boundary cutover line をいつ acceptance 済みと見なすか、いつ current blocker 候補へ昇格できるかを固定する。
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/design/de-rust-backend-zero-boundary-lock-ssot.md
  - docs/development/current/main/design/backend-recipe-route-profile-ssot.md
  - docs/development/current/main/phases/phase-29ck/README.md
  - docs/development/current/main/phases/phase-29ck/P1-NY-LLVMC-NATIVE-EMITTER-TASK-PACK.md
  - docs/development/current/main/phases/phase-29ck/P3-THIN-BACKEND-CUTOVER-LOCK.md
  - docs/development/current/main/phases/phase-29x/29x-63-llvm-cabi-link-gate-ssot.md
  - docs/reference/abi/ABI_BOUNDARY_MATRIX.md
  - apps/tests/mir_shape_guard/collapsed_min.mir.json
  - apps/tests/hello_simple_llvm.hako
---

# P2: Backend Boundary Acceptance And Reopen Rule

## Purpose

- backend-zero を「いつかやる」で終わらせず、acceptance と blocker promotion を固定する。
- native object / native exe / runner parity の証拠を段階で分ける。
- bootstrap seam evidence と final ownership を混同しない。
- current compiler authority wave を backend-zero docs が誤って上書きしないようにする。

## 1. Acceptance Ladder

### A1. Docs-ready

- `phase-29ck/README.md`
- `P0-BACKEND-ZERO-OWNER-INVENTORY.md`
- `P1-NY-LLVMC-NATIVE-EMITTER-TASK-PACK.md`
- `P2-NATIVE-EMITTER-ACCEPTANCE-AND-REOPEN-RULE.md`
- `P3-THIN-BACKEND-CUTOVER-LOCK.md`

が揃っていること。

### A2. Native object canary

- seed fixture:
  - `apps/tests/mir_shape_guard/collapsed_min.mir.json`
- contract:
  - `ny-llvmc --emit obj` が Python/llvmlite 非依存で `.o` を出す
  - failure は explicit fail-fast
  - silent fallback しない

### A3. Native executable parity

- seed fixture:
  - `apps/tests/mir_shape_guard/collapsed_min.mir.json`
- contract:
  - same native route が既存 static-first link line で EXE まで到達する
  - Core C ABI surface lock を壊さない

### A4. Runner / app parity

- seed fixture:
  - `apps/tests/hello_simple_llvm.hako`
- contract:
  - `phase29x_llvm_cabi_link_min.sh` 相当の意味契約
    - build success
    - linked exe exit 0
    - output `42`
  - native route でも replay できる
  - direct `hakorune --backend llvm` route でも same selector で replay できる

### A5. Thin backend boundary cutover

- contract:
  - `.hako` caller が thin backend boundary を daily route で使う
  - target owner paths are `LlvmBackendBox` / `lang/c-abi`
  - `native_driver.rs` は final owner ではなく canary-only か retire 対象になる
  - canonical ABI surface は 2 面固定のまま維持する
  - final runtime-proof owner は `.hako VM`
  - regular Rust VM は blocker closeout まで temporary proof lane に留める

### A6. Mainline promotion / llvmlite demotion

- contract:
  - mainline backend owner = thin backend boundary
  - `llvmlite` = compat/canary only
  - backend-specific optimization の新規 mainline owner は thin boundary の内側にある

## 2. Evidence Shape

phase docs が要求する evidence は次の順番で積む。

1. direct CLI object evidence
   - `ny-llvmc --emit obj --in apps/tests/mir_shape_guard/collapsed_min.mir.json --out <tmp.o>`
   - default driver must be boundary-owned; no `--driver harness` or `--driver native` required
2. direct CLI executable evidence
   - `ny-llvmc --emit exe --in apps/tests/mir_shape_guard/collapsed_min.mir.json --nyrt target/release --out <tmp.exe>`
   - default driver must be boundary-owned; no `--driver harness` or `--driver native` required
3. native seam evidence
   - `ny-llvmc --driver native --emit obj --in apps/tests/mir_shape_guard/collapsed_min.mir.json --out <tmp.o>`
   - `ny-llvmc --driver native --emit exe --in apps/tests/mir_shape_guard/collapsed_min.mir.json --nyrt target/release --out <tmp.exe>`
4. app fixture parity evidence
   - `apps/tests/hello_simple_llvm.hako`
   - current downstream meaning target は `phase29x_llvm_cabi_link_min.sh`
   - native opt-in smoke:
     - `tools/smokes/v2/profiles/integration/apps/phase29ck_native_llvm_cabi_link_min.sh`
   - direct runner evidence:
     - `NYASH_LLVM_USE_HARNESS=1 NYASH_LLVM_BACKEND=native NYASH_NY_LLVM_COMPILER=target/release/ny-llvmc NYASH_EMIT_EXE_NYRT=target/release ./target/release/hakorune --backend llvm apps/tests/hello_simple_llvm.hako`
5. boundary cutover evidence
   - owner/env/archive contract is locked by `P3-THIN-BACKEND-CUTOVER-LOCK.md`
   - exact replay commands:
     - `bash tools/smokes/v2/profiles/integration/apps/phase29ck_llvm_backend_box_capi_link_min.sh`
     - `SMOKES_FORCE_LLVM=1 bash tools/smokes/v2/profiles/integration/apps/phase29ck_native_llvm_cabi_link_min.sh`
   - owner paths must be:
     - `lang/src/shared/backend/llvm_backend_box.hako`
     - `lang/c-abi/shims/hako_aot.c`
   - parked legacy paths:
     - `lang/src/llvm_ir/archive/legacy_script_builder/**`
   - rule:
     - do not treat `A2`-`A4` alone as final done shape
6. runtime proof owner evidence
   - final owner lane:
     - `.hako VM` route for `LlvmBackendBox` execution
   - exact replay command:
     - `bash tools/smokes/v2/profiles/integration/apps/phase29ck_vmhako_llvm_backend_runtime_proof.sh`
   - pinned env:
     - `NYASH_LLVM_USE_CAPI=1`
     - `HAKO_V1_EXTERN_PROVIDER_C_ABI=1`
   - compat-only env:
     - `HAKO_CAPI_PURE=1`
       - pure-lowering/historical route only; phase-29ck runtime proof must not require it
       - compat-pack entry/meaning is fixed by `P5-COMPAT-PURE-PACK-LOCK.md`
   - temporary seam rule:
     - this proof may depend on `P3` temporary seams, but those seams cannot become silent daily owner paths
   - rule:
     - temporary regular-VM proof may exist during migration, but cannot be promoted as done-shape owner evidence

補足:
- `BE0-min2` で locked selector は `--driver native`
- default route is now `--driver boundary`
- unsupported shapes may still replay `--driver harness` through `hako_aot_compile_json(...)` compat fallback, but that fallback must stay explicit and must not silently re-promote `Harness` as the selector-level default

## 3. Reopen / Promotion Rule

backend-zero を current blocker 候補へ昇格してよいのは、次を全部満たした時だけ。

1. current compiler authority wave が `monitor-only` か、backend-only blocker に収束している
2. `A1` docs-ready が満たされている
3. `A2`-`A4` bootstrap seam evidence が green
4. `A5` thin backend boundary cutover contract が exact owner docs で固定されている
5. promotion 時に `CURRENT_TASK.md` と `de-rust-full-rust-zero-roadmap-ssot.md` を同時更新する

上記を満たさない限り、backend-zero は `accepted pointer / queued phase` のまま保持する。

## 4. Regression Rule

1. native route failure を llvmlite への silent auto-fallback で隠さない
2. `NYASH_LLVM_USE_HARNESS` の意味をこの phase で暗黙変更しない
3. `Cranelift keep` を reopen しない
4. canonical ABI surface を 2 面以外へ増やさない
5. `native_driver.rs` を bootstrap seam から final owner へ黙って昇格させない

## 5. Done Shape For This Phase

- backend-zero の first implementation target が phase docs だけで読める
- acceptance と promotion 条件が phrase ではなく command/evidence 単位で読める
- `native_driver.rs` が bootstrap seam であり final owner ではないと phase docs だけで読める
- current blocker と future queued phase の境界が崩れていない
