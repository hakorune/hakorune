---
Status: Accepted (queued)
Decision: accepted
Date: 2026-03-14
Scope: backend-zero を独立 phase に切り、bootstrap seam と thin backend boundary cutover の fixed order を docs-ready な形で固定する。
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/design/de-rust-full-rust-zero-roadmap-ssot.md
  - docs/development/current/main/design/de-rust-backend-zero-boundary-lock-ssot.md
  - docs/development/current/main/design/de-rust-backend-zero-provisional-inventory-ssot.md
  - docs/reference/abi/ABI_BOUNDARY_MATRIX.md
  - docs/reference/plugin-abi/nyash_abi_v2.md
  - docs/development/current/main/phases/phase-29x/29x-63-llvm-cabi-link-gate-ssot.md
  - docs/development/current/main/phases/phase-29ck/P3-THIN-BACKEND-CUTOVER-LOCK.md
  - crates/nyash-llvm-compiler/src/main.rs
  - src/runner/modes/common_util/exec.rs
---

# Phase 29ck: Backend-Zero Boundary Cutover Preparation

## Goal

- backend-zero を `future idea` ではなく、queued phase として読めるようにする。
- `native_driver.rs` を final owner にせず、最終 target を `.hako -> thin backend C ABI/plugin boundary -> object/exe` に固定する。
- current bootstrap seam と final cutover target を混線させない。
- current compiler authority blocker と混線させず、`inventory -> task pack -> acceptance/reopen rule` を phase 内に閉じる。

## Entry Conditions

1. immediate blocker は引き続き pure `.hako`-only hakorune build の compiler authority removal である
2. canonical ABI surface は引き続き 2 面固定である
   - Core C ABI
   - TypeBox ABI v2
3. `Cranelift` は explicit keep のままであり、この phase では置換対象にしない
4. runtime-zero daily policy（`LLVM-first / vm-hako monitor-only`）はこの phase で変更しない

## Fixed Order

1. `P0-BACKEND-ZERO-OWNER-INVENTORY.md`
2. `P1-NY-LLVMC-NATIVE-EMITTER-TASK-PACK.md`
3. `P2-NATIVE-EMITTER-ACCEPTANCE-AND-REOPEN-RULE.md`
4. `P3-THIN-BACKEND-CUTOVER-LOCK.md`
5. 上記 contract を満たしてからだけ、backend-zero の blocker 昇格可否を再判定する

## Current Snapshot (2026-03-14)

1. current LLVM route は `hakorune -> ny-llvmc -> llvmlite -> object/exe`
2. `ny-llvmc` はすでに CLI / normalize / opt-level env / static-first link を owner している
3. missing leg は `MIR -> .o` の native object emission である
4. landed first docs/code slice:
   - `BE0-min1` CLI contract freeze
   - stable caller contract is now pinned in `crates/nyash-llvm-compiler/README.md`
   - `clap` parse contract is pinned by unit tests in `crates/nyash-llvm-compiler/src/main.rs`
5. landed second seam slice:
   - `BE0-min2` native driver selector
   - `--driver {harness|native}` now exists as implementation-detail opt-in
   - default route stays `harness`
6. landed canary slice:
   - `BE0-min3` native object canary is green for `apps/tests/mir_shape_guard/collapsed_min.mir.json`
   - `BE0-min4` same-seed native executable parity is green on the existing static-first link line
7. landed app-seed opt-in parity:
   - `BE0-min5` is green for `apps/tests/hello_simple_llvm.hako`
   - `tools/build_llvm.sh` now honors `NYASH_LLVM_COMPILER=crate` + `NYASH_LLVM_BACKEND=native`
   - acceptance smoke is `tools/smokes/v2/profiles/integration/apps/phase29ck_native_llvm_cabi_link_min.sh`
8. landed direct runner opt-in parity:
   - `src/runner/modes/common_util/exec.rs` now forwards `NYASH_LLVM_BACKEND=native` to `ny-llvmc --driver native`
   - `NYASH_LLVM_USE_HARNESS=1 ./target/release/hakorune --backend llvm apps/tests/hello_simple_llvm.hako` is green under the same native selector
   - argv capture confirms the runner now invokes `ny-llvmc ... --driver native`
9. boundary lock:
   - `docs/development/current/main/design/de-rust-backend-zero-boundary-lock-ssot.md`
   - `native_driver.rs` is bootstrap seam only
   - final caller target is `LlvmBackendBox` / `hako_aot` style thin boundary
10. legacy route park:
   - historical `llvm_ir` script-builder / AotFacade route is archived under `lang/src/llvm_ir/archive/legacy_script_builder/**`
   - live `llvm_ir` keeps only `AotPrep` / `normalize` / compat `emit`
11. `.hako` backend caller wave や optimization handoff は、この boundary cutover の後段で扱う

## Non-goals

- linker を作り直すこと
- LLVM backend 本体を TypeBox ABI に載せること
- `native_driver.rs` を final owner にすること
- `Cranelift` を de-Rust 対象へ広げること
- backend-zero を inventory なしで current blocker に昇格させること

## Immediate Next

1. `BE0-min6`
   - thin backend boundary cutover lock
   - target owners are `LlvmBackendBox` / `lang/c-abi` / caller wiring
   - legacy `llvm_ir` script-builder route is archive keep, not daily owner
2. native subset widening
   - next widening target is phase2120 old native canary set (`const/binop(Add)/compare(Eq/Lt)/ret/branch`) only when boundary cutover needs more seam evidence
3. post-cutover follow-up
   - optimization handoff と llvmlite demotion lock
4. `P2` の promotion gate はまだ未達なので、current compiler authority wave は上書きしない

## Acceptance

- phase だけで `owner / first code slice / acceptance / reopen rule` が辿れる
- `native_driver.rs` が bootstrap seam であり、final owner ではないと一意に読める
- docs はもう「backend-zero は task pack 未整備だから provisional」の状態ではない
