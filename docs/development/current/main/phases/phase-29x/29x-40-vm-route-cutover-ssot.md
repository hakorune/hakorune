---
Status: Active
Decision: accepted
Date: 2026-02-13
Scope: Phase 29x X18 VM route cutover 設計（`vm`/`vm-hako`/compat の責務と切替順序を固定）。
Related:
  - docs/development/current/main/phases/phase-29x/README.md
  - docs/development/current/main/phases/phase-29x/29x-91-task-board.md
  - src/runner/dispatch.rs
  - src/config/env/vm_backend_flags.rs
---

# Phase 29x X18: VM Route Cutover SSOT

## 0. Goal

`vm` / `vm-hako` / compat を「どこで選び、どこで失敗させるか」を先に固定し、
X19-X23 の route 実装作業で責務混線を防ぐ。

本ドキュメントは route の**設計境界**を固定する。  
route 優先順の実装変更（strict/dev 既定切替など）は X19 以降で行う。

## 1. Route Surface (as-is, X18 baseline)

Route 選択入口は `src/runner/dispatch.rs` の backend 分岐を SSOT とする。

- `backend=vm`:
  - 既定: `execute_vm_mode()`
  - 例外: `NYASH_VM_USE_FALLBACK=1` のときのみ `execute_vm_fallback_interpreter()`
- `backend=vm-hako`:
  - `execute_vm_hako_mode()`
- `backend=llvm`:
  - `execute_llvm_mode()`

観測タグ（現状）:
- `NYASH_VM_ROUTE_TRACE=1` で `[vm-route/select] ...`（X19）、
  `[vm-route/pre-dispatch] ...`（X26）を出力

## 2. Responsibility Split (X18 contract)

1. `vm` lane:
   Rust VM の主経路。compat を暗黙で抱えない。
2. `vm-hako` lane:
   selfhost/de-rust の主候補経路。未対応は fail-fast で止める。
3. `compat` lane:
   明示 opt-in（例: `NYASH_VM_USE_FALLBACK=1`）時のみ許可する縮退経路。

禁止:
- `vm` lane から silent に compat へ落ちること
- `vm-hako` lane から暗黙に `vm` へ戻ること
- route 判断を複数層へ重複実装すること（dispatch + mode 両方で別判断）

## 3. Cutover Sequence Lock (X19-X23)

1. X19: route observability を追加（選択理由タグを安定化）
2. X20: strict/dev で `vm-hako` 優先化（compat は明示時のみ）
3. X21: non-strict compat lane を限定縮退（撤去ではなく境界固定）
4. X22: 3日連続 gate evidence
5. X23: Rust-optional done docs 同期

この順序を崩して「先に優先順だけ変える」ことを禁止する。

## 4. Fail-Fast Contract

- route 未対応/未実装は fail-fast（理由タグ付き）で停止する。
- compat は “救済経路” ではなく “明示縮退経路” として扱う。
- strict/dev では暗黙 fallback を契約違反として扱う（X20 で実装固定）。

## 5. Evidence for X18

Docs:
- このファイル
- `docs/development/current/main/phases/phase-29x/README.md`
- `docs/development/current/main/phases/phase-29x/29x-90-integration-checklist.md`
- `docs/development/current/main/phases/phase-29x/29x-91-task-board.md`

Code anchors:
- `src/runner/dispatch.rs`（route 選択入口）
- `src/config/env/vm_backend_flags.rs`（compat/fallback/trace フラグ）
