---
Status: Active
Decision: accepted
Date: 2026-02-28
Scope: runtime/plugin de-rust の最終ゴールを source-zero に固定しつつ、現フェーズは no-delete-first（経路切替先行）で進める。
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/10-Now.md
  - docs/development/current/main/phases/phase-29cc/README.md
  - docs/development/current/main/design/hako-runtime-c-abi-cutover-order-ssot.md
  - docs/development/current/main/design/de-rust-post-g1-runtime-plan-ssot.md
  - docs/development/current/main/phases/phase-29cc/29cc-214-runtime-rust-thin-to-zero-execution-path-ssot.md
  - docs/development/current/main/phases/phase-29cc/29cc-215-runtime-execution-path-observability-lock-ssot.md
  - docs/reference/abi/ABI_BOUNDARY_MATRIX.md
---

# 29cc-220 Runtime Source-Zero Cutover Lock

## Purpose

`execution-path-zero` で止めず、runtime/plugin の Rust 実装を source-zero まで進めるための完了条件を固定する。
ただし本フェーズでは Rust source の物理削除を行わず、経路切替を先行して安全性を確保する。

## Source-Zero Definition (fixed)

1. この lane の done は **source-zero** とする。
2. source-zero の定義:
   - `src/runtime/plugin_loader_v2/enabled/*` の runtime/plugin loader 実装が撤去済み。
   - `crates/nyash_kernel/src/plugin/*` の runtime/plugin 実装が撤去済み。
   - mainline/CI 既定で compat route を使用しない（compat default-off）。
3. Rust 側の残置は portability/build scaffolding に限定する（runtime/plugin 意味論実装は残さない）。

## Current Phase Target (fixed)

1. このフェーズの done は **route-zero + stability** とする。
2. route-zero + stability の定義:
   - mainline/CI 既定が `.hako + ABI` のみで実行される（compat default-off）。
   - runtime/plugin Rust route は未使用化され、guard で drift が監査可能。
   - Rust source は削除しない（復元/保険のため残置）。

## Boundary Lock (must keep)

1. Canonical ABI は 2 面のみ:
   - Core C ABI
   - TypeBox ABI v2
2. `args borrowed / return owned` 契約を維持する。
3. silent fallback を禁止し、strict/dev では fail-fast を固定する。
4. 第3 ABI 面は追加しない。

## Fixed Order (1 boundary = 1 commit)

1. docs sync（本 lock + CURRENT_TASK + phase README）
2. compat default-off lock（mainline/CI route）
3. plugin loader residue retire:
   - `method_resolver.rs`
   - `instance_manager.rs`
   - `ffi_bridge.rs` / `host_bridge.rs`
   - `loader/*`
4. kernel plugin residue retire:
   - `invoke_core.rs` / `birth.rs` / `runtime_data.rs` / `semantics.rs`
   - `value_codec/*`
   - `future.rs` / `invoke.rs`
5. no-delete route lock 更新（source delete は将来フェーズへ延期）

## Acceptance

1. `CURRENT_TASK.md` / `10-Now.md` / `phase-29cc/README.md` が source-zero 定義で同期されている。
2. `tools/checks/dev_gate.sh runtime-exec-zero` と route guard が green。
3. mainline で compat route が呼ばれない（ログ監査で drift 無し）。
4. ABI lane guard（Core C ABI + TypeBox ABI v2）が継続 green。

## Execution update

- 2026-03-01: kernel B3 closeout（29cc-241/242）を同期。
  - future/invoke の compat payload encode を `encode_legacy_vm_args_range()` に統一。
  - `plugin/mod.rs` の B3 wiring contract test で entrypoint re-export drift を監視。
  - runtime-exec-zero / no-compat-mainline gate は継続 green。

## Deferred Deletion Gate (future)

Rust source 削除は次の条件を満たした後に別 lock で実施する。

1. mac 実機でのローカルビルド運用が確立している。
2. portability CI（macOS/Windows）が一定期間連続で green。
3. route drift guard で Rust route 未使用が継続確認できる。

## Not in this lock

1. 新規 ABI 面の追加
2. perf 最適化の再開
3. 言語仕様拡張
