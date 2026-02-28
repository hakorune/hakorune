---
Status: Active
Decision: accepted
Date: 2026-02-28
Scope: de-rust の最終ゴールを「実行経路0（execution-path-zero）」で固定し、runtime/plugin loader の薄化順序を docs-first で確定する。
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/10-Now.md
  - docs/development/current/main/phases/phase-29cc/README.md
  - docs/development/current/main/design/hako-runtime-c-abi-cutover-order-ssot.md
  - docs/development/current/main/design/de-rust-post-g1-runtime-plan-ssot.md
  - docs/reference/abi/ABI_BOUNDARY_MATRIX.md
---

# 29cc-214 Runtime Rust Thin-to-Zero Execution Path Lock

## Purpose

`Rust最終0` を曖昧語で運用せず、mainline の完了条件を 1 つに固定する。

## Zero Definition (fixed)

1. この lane の done は **execution-path-zero** とする。
2. execution-path-zero の定義:
   - mainline 実行経路（daily/CI 標準）で Rust runtime/plugin loader を通らない。
   - Rust 実装は portability/recovery 用として残置してよい。
3. ソース完全撤去（repo source-zero）は別フェーズとし、本 lane の done 条件に含めない。

## Boundary Lock (must keep)

1. Canonical ABI は 2 面のみ:
   - Core C ABI
   - TypeBox ABI v2
2. `args borrowed / return owned` 契約を維持する。
3. silent fallback を禁止し、strict/dev では fail-fast を固定する。
4. 第3 ABI 面は追加しない。

## Fixed Order (docs-first -> implementation)

1. Docs boundary sync（this lock）
2. Route drift observability lock（Rust route 使用の検知タグを固定）
3. ABI slice lock（`string_len` / `array_get_i64` / `array_set_i64`）
4. Rust thin wrapper lock（marshal + fail-fast のみ）
5. `.hako` adapter default lock（mainline を `.hako` 優先へ）
6. Vocabulary expansion（1語彙=1契約=1commit）
7. execution-path-zero declaration（mainline/CI 既定が Rust runtime/plugin loader 非依存）

## Acceptance

1. `CURRENT_TASK.md` / `10-Now.md` / `phase-29cc/README.md` が本定義と同期している。
2. `hako-runtime-c-abi-cutover-order-ssot.md` に execution-path-zero 定義が反映されている。
3. plugin lane monitor-only（29cc-213 fixed）と矛盾しない。

## Not in this lock

1. Rustコードの大規模削除
2. plugin loader 実装の即時置換
3. 追加最適化（perf 施策）
