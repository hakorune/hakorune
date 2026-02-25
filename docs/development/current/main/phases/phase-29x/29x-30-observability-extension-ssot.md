---
Status: Active
Decision: accepted
Date: 2026-02-13
Scope: Phase 29x X13-X17 observability 拡張（5カテゴリ観測 + summary 契約固定）。
Related:
  - docs/development/current/main/phases/phase-29y/30-OBSERVABILITY-SSOT.md
  - docs/development/current/main/design/runtime-gc-policy-and-order-ssot.md
  - src/runtime/leak_tracker.rs
---

# Phase 29x X13-X17: Observability Extension SSOT

## 0. Goal

`locals / temps / heap_fields / handles / singletons` の root surface 語彙を
Phase 29x の実装順序（X14-X17）に接続し、実装時の判断分岐を減らす。

本ドキュメントは「語彙と観測契約の固定」が目的であり、
GC 挙動や意味論は変更しない。

## 1. Vocabulary Lock（5 categories）

root surface の語彙は以下 5 つで固定する。

1. `locals`
2. `temps`
3. `heap_fields`
4. `handles`
5. `singletons`

禁止:
- 上記以外の別名カテゴリ追加
- backend 固有の ad-hoc カテゴリ名を外部契約へ露出

## 2. Baseline after X16 (2026-02-13)

現行 `src/runtime/leak_tracker.rs` では次の baseline を観測する。

- `handles`: 実カウントあり（`modules + host_handles` を集計）
- `locals`: baseline として `0` 固定
- `temps`: 実カウントあり（VM 実行中の strong temp root 最大値）
- `heap_fields`: 実カウントあり（object field 由来 strong root の最大値）
- `singletons`: 実カウントあり（runtime module globals を singleton roots として集計）

既存ログ契約:
- `[lifecycle/leak] Root categories:`
- `[lifecycle/leak]   handles: <n>`
- `[lifecycle/leak]   locals: <n>`
- `[lifecycle/leak]   temps: <n>`
- `[lifecycle/leak]   heap_fields: <n>`
- `[lifecycle/leak]   singletons: <n>`
- fallback（source 未観測時）:
  `[lifecycle/leak]   (Phase 1 limitation: temps source unavailable; heap_fields source unavailable; singletons source unavailable)`

## 3. X14-X17 Execution Contract

### X14: `temps` 実装 + smoke

- 目的: `temps` を limitation 固定値から実測へ移す。
- 受け入れ基準: `phase29x_observability_temps_vm.sh` PASS。
- 失敗方針: 観測不可条件は silent skip せず、limitation 理由を 1 行で出す。
- 状態: done（2026-02-13）
- 証跡: `bash tools/smokes/v2/profiles/integration/apps/phase29x_observability_temps_vm.sh`

### X15: `heap_fields` 実装 + smoke

- 目的: object field 由来 strong root を `heap_fields` に集約する。
- 受け入れ基準: `phase29x_observability_heap_fields_vm.sh` PASS。
- 失敗方針: 集計不能を 0 埋めで隠さない（理由付き limitation 表示）。
- 状態: done（2026-02-13）
- 証跡: `bash tools/smokes/v2/profiles/integration/apps/phase29x_observability_heap_fields_vm.sh`

### X16: `singletons` 実装 + smoke

- 目的: runtime/plugin 側 singleton root を `singletons` へ集約する。
- 受け入れ基準: `phase29x_observability_singletons_vm.sh` PASS。
- 失敗方針: singleton source が未接続なら明示タグで fail-fast か limitation 表示。
- 状態: done（2026-02-13）
- 証跡: `bash tools/smokes/v2/profiles/integration/apps/phase29x_observability_singletons_vm.sh`

### X17: `debug_root_summary` 契約固定

- 目的: 5カテゴリを `debug_root_summary` 契約で安定化する。
- 受け入れ基準: `phase29x_observability_summary_vm.sh` PASS。
- 契約: カテゴリ欠落・別名化・順序不定を禁止し、1出力契約に固定する。
- 状態: done（2026-02-13）
- 証跡: `bash tools/smokes/v2/profiles/integration/apps/phase29x_observability_summary_vm.sh`

## 4. Non-goals (Phase 29x)

- GC algorithm 実装
- finalizer 意味論の追加
- 診断ON/OFFによる実行意味の変化

## 5. Evidence for X13-X17

Docs:
- このファイル
- `docs/development/current/main/phases/phase-29x/README.md`（Week3 contract pin）
- `docs/development/current/main/phases/phase-29x/29x-90-integration-checklist.md`
- `docs/development/current/main/phases/phase-29x/29x-91-task-board.md`

Code / smoke:
- `src/runtime/leak_tracker.rs`（temps / heap_fields 観測値を root summary へ接続）
- `src/runtime/leak_tracker.rs`（`debug_root_summary()` API を公開し 5カテゴリ語彙を固定）
- `src/backend/mir_interpreter/mod.rs`（observability reset + strong root counters）
- `src/backend/mir_interpreter/exec/mod.rs`（return 時 observe_temps / observe_heap_fields 配線）
- `tools/smokes/v2/profiles/integration/apps/phase29x_observability_temps_vm.sh`（X14 受け入れスモーク）
- `tools/smokes/v2/profiles/integration/apps/phase29x_observability_heap_fields_vm.sh`（X15 受け入れスモーク）
- `tools/smokes/v2/profiles/integration/apps/phase29x_observability_singletons_vm.sh`（X16 受け入れスモーク）
- `tools/smokes/v2/profiles/integration/apps/phase29x_observability_summary_vm.sh`（X17 受け入れスモーク）
