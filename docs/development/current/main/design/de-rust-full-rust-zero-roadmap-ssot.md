---
Status: SSOT
Decision: provisional
Date: 2026-03-14
Scope: `full Rust 0` を runtime-zero / backend-zero に分割して可視化し、current blocker と future vision の混線を防ぐ。
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/10-Now.md
  - docs/development/current/main/design/de-rust-backend-zero-fixed-order-and-buildability-ssot.md
  - docs/development/current/main/design/de-rust-kernel-authority-cutover-ssot.md
  - docs/development/current/main/design/de-rust-zero-buildability-contract-ssot.md
  - docs/development/current/main/design/de-rust-full-rust-zero-remaining-rust-inventory-ssot.md
  - docs/development/current/main/design/de-rust-full-rust-zero-remaining-rust-task-pack-ssot.md
  - docs/development/current/main/design/de-rust-backend-zero-boundary-lock-ssot.md
  - docs/development/current/main/design/de-rust-backend-zero-provisional-inventory-ssot.md
  - docs/development/current/main/phases/phase-29ck/README.md
  - docs/development/current/main/phases/phase-29ck/P1-NY-LLVMC-NATIVE-EMITTER-TASK-PACK.md
  - docs/development/current/main/phases/phase-29ck/P2-NATIVE-EMITTER-ACCEPTANCE-AND-REOPEN-RULE.md
  - docs/development/current/main/design/de-rust-master-task-map-ssot.md
  - docs/development/current/main/design/de-rust-lane-map-ssot.md
  - docs/development/current/main/design/de-rust-post-g1-runtime-plan-ssot.md
  - docs/development/current/main/phases/phase-29cc/29cc-220-runtime-source-zero-cutover-lock-ssot.md
  - docs/development/current/main/phases/phase-29cc/29cc-253-source-zero-static-link-boundary-lock-ssot.md
  - docs/development/current/main/phases/phase-29y/60-NEXT-TASK-PLAN.md
  - docs/development/current/main/design/de-rust-compiler-thin-rust-roadmap-ssot.md
---

# De-Rust Full Rust 0 Roadmap (SSOT)

## Purpose

- `full Rust 0` を 1 本の曖昧な将来案として扱わず、runtime-zero と backend-zero に分けて見通しを固定する。
- 既存の current blocker（compiler authority removal）を future vision で上書きしない。
- runtime 側は inventory-ready、backend 側は phase-cut queued という温度差を明文化する。
- `0rust` は Rust meaning owner zero を意味するが、Rust ベースの build/bootstrap route を壊すことではない。
- operational reading は `stage0 Rust bootstrap keep / stage1 proof / stage2+ 0rust mainline` だと読む。

## 1. Boundary Lock

- この文書は future tracking 用の薄い SSOT であり、daily の blocker 管理そのものは `CURRENT_TASK.md` を正本とする。
- 現在の immediate blocker は引き続き compiler authority removal（pure `.hako`-only hakorune build）である。
- `phase-29y` の runtime daily policy（`LLVM-first / vm-hako monitor-only`）はこの文書では変更しない。
- non-plugin done の判定範囲は `de-rust-scope-decision-ssot.md` を維持し、この文書で広げない。
- buildability は preservation-first で扱い、Rust build route を migration 中に silent delete しない。
- stage0 bootstrap keep と stage2+ selfhost mainline を同じ acceptance に混ぜない。

## 2. Current Snapshot (2026-03-14)

1. compiler authority removal:
   - active
   - `CURRENT_TASK.md` の current blocker / `phase-29ch` / `phase-29ci` / `phase-29cj` を正本とする。
2. runtime-zero:
   - accepted pointer / inventory-ready
   - source-zero と static-link boundary の入口が既に揃っている。
   - `kernel authority zero` は queued pointer として別建てで tracking し、meaning/policy owner の cutover と substrate delete を混ぜない。
3. backend-zero:
   - accepted pointer / phase-cut queued
   - owner inventory と keep-vs-retire の高位境界は固定済み
   - final architecture boundary is locked in `de-rust-backend-zero-boundary-lock-ssot.md`
   - execution order / buildability gate is fixed in `de-rust-backend-zero-fixed-order-and-buildability-ssot.md`
   - phase-29ck が `task pack / acceptance / reopen rule` を owner する
   - post-B1/B3 by-name retirement follow-up is owned by `docs/development/current/main/phases/phase-29cl/README.md`
   - `native_driver.rs` は bootstrap seam only であり、done shape ではない。
   - migration slice は Rust ベースの buildability を保持することを前提にする。
4. remaining Rust bucket snapshot:
   - exact compiler/runtime/backend residue inventory lives in
     `docs/development/current/main/design/de-rust-full-rust-zero-remaining-rust-inventory-ssot.md`
   - fixed-order task pack lives in
     `docs/development/current/main/design/de-rust-full-rust-zero-remaining-rust-task-pack-ssot.md`
   - stage0 first-build / recovery lane is allowed to remain Rust-based while stage2+ mainline is cut over

## 3. Split Tracking Rule

### 3.1 runtime-zero

- 定義:
  - runtime/plugin の Rust meaning 実装を source-zero まで縮退し、残置Rustを portability / build scaffolding に限定する。
- status:
  - accepted pointer / inventory-ready
- primary SSOT:
  - `docs/development/current/main/design/de-rust-post-g1-runtime-plan-ssot.md`
  - `docs/development/current/main/phases/phase-29cc/29cc-220-runtime-source-zero-cutover-lock-ssot.md`
  - `docs/development/current/main/phases/phase-29cc/29cc-253-source-zero-static-link-boundary-lock-ssot.md`
  - `docs/development/current/main/phases/phase-29y/60-NEXT-TASK-PLAN.md`
- operation rule:
  - daily は `phase-29y` の failure-driven monitor-only を維持する。
  - full Rust 0 pointer を作ったこと自体では lane C を reopen しない。

### 3.1b kernel-authority-zero

- 定義:
  - kernel meaning/policy の final owner を `.hako` 側へ移し、Rust runtime を substrate / portability / compat keep に降格する。
- status:
  - queued pointer
- primary SSOT:
  - `docs/development/current/main/design/de-rust-kernel-authority-cutover-ssot.md`
  - `lang/README.md`
  - `docs/development/current/main/design/array-map-owner-and-ring-cutover-ssot.md`
- current rule:
  - current exe optimization wave と混ぜない。
  - `kernel authority zero` は `substrate zero` と同じ task にしない。
  - start trigger を満たすまでは visibility only に留める。

### 3.2 backend-zero

- 定義:
  - LLVM / Cranelift / object emit / exe link / backend runner glue を含む Rust-owned backend surface を棚卸しし、`.hako` ownership へ移せる面と explicit keep 面を分離する。
- status:
  - accepted pointer / phase-cut queued
- primary SSOT:
  - `docs/development/current/main/design/de-rust-backend-zero-boundary-lock-ssot.md`
  - `docs/development/current/main/design/de-rust-backend-zero-provisional-inventory-ssot.md`
  - `docs/development/current/main/phases/phase-29ck/README.md`
  - `docs/development/current/main/phases/phase-29cl/README.md`
  - `docs/development/current/main/phases/phase-29ck/P1-NY-LLVMC-NATIVE-EMITTER-TASK-PACK.md`
  - `docs/development/current/main/phases/phase-29ck/P2-NATIVE-EMITTER-ACCEPTANCE-AND-REOPEN-RULE.md`
- current gap:
  - `.hako` caller が thin backend C ABI/plugin boundary を daily route で使う line はまだ未実装。
  - `native_driver.rs` の bootstrap evidence は final ownership を意味しない。
  - current blocker へ昇格させるには `phase-29ck` の promotion gate を満たす必要がある。
- rule:
  - `phase-29ck` の promotion gate を満たすまでは current blocker に昇格しない。
  - `backend-zero` は現時点では「queued phase」であり、daily 実装順を上書きしない。
  - kernel/plugin/backend boundary の `by_name` retire work は `phase-29cl` を owner にし、frontend fixture-key history (`phase-29ce`) と混線させない。

## 4. Fixed Order (high level)

1. まず current compiler authority wave を完走する。
2. runtime-zero は source-zero docs 群を正本に visibility を上げつつ、failure-driven で reopen する。
3. backend-zero は `phase-29ck` を queued phase として維持し、その promotion gate を満たした後にだけ blocker 昇格可否を判断する。

## 5. Done Shape

- runtime-zero done:
  - runtime/plugin Rust meaning 実装が source-zero まで縮退し、残置Rustは portability / build scaffolding に限定されている。
- backend-zero done:
  - backend surface の keep / retire 境界が別文書で固定され、mainline backend ownership が `.hako -> thin backend boundary` 側へ移っているか、または explicit keep として限定されている。
  - `native_driver.rs` は final owner ではなく canary-only か retired になっている。
- full Rust 0 done:
  - compiler authority removal
  - runtime-zero
  - backend-zero
  - stage2+ daily selfhost build no longer depends on Rust as a normal owner path
  - stage0 Rust bootstrap remains only as first-build / recovery / preservation lane
  - 上記 3 本が docs 間で矛盾なく closeout されている。

## 6. Not in this doc

- `phase-29y` の immediate next を書き換えること。
- backend-zero を inventory なしで active blocker にすること。
- source delete task をこの文書だけで再開すること。
