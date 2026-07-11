# Phase 287 P5: `rewriter/stages` 可視性引き締め指示書（意味論不変）

**Date**: 2025-12-27  
**Status**: Completed ✅  
**Scope**: `src/mir/builder/control_flow/joinir/merge/rewriter/stages/` の stage 関数（scan/plan/apply）を `stages/mod.rs` 経由の re-export に統一し、呼び出し側は `stages::{...}` のみを使用する（単一入口）。  
**Non-goals**: 仕様変更、エラータグ/ヒント文の変更、ログ出力変更、pipeline の順序/条件変更、ファイル分割の追加（P5は “可視性と入口” のみ）

---

## 目的（SSOT）

- “誰が stage 関数を呼べるか” を構造で明示し、API 面を薄くする（カプセル化）。
- `instruction_rewriter.rs` 側の import を単純化し、`stages/mod.rs` を pipeline の **唯一の入口**にする。
- 将来のリファクタでも “入口を 1 箇所” に保ち、迷子を防ぐ。

---

## 現状

- `stages/{scan,plan,apply}.rs` の関数が `pub(in crate::mir::builder::control_flow::joinir::merge)` になっており、実装側が外へ露出している。
- `instruction_rewriter.rs` が `stages::scan::scan_blocks` のように “実装ファイル名” を知っている。

---

## 目標（構造）

```
rewriter/stages/
├── mod.rs        # facade（re-export のみ）
├── scan.rs       # pub(super) fn scan_blocks(...)
├── plan/         # pub(super) fn plan_rewrites(...)
└── apply.rs      # pub(super) fn apply_rewrites(...)
```

- `instruction_rewriter.rs` は `use super::rewriter::stages::{scan_blocks, plan_rewrites, apply_rewrites};` のみ。

---

## 手順（安全な順序）

### Step 1: `stages/mod.rs` に re-export を追加

- `pub(in crate::mir::builder::control_flow::joinir::merge) use ...;` で 3 関数を re-export する。
  - `scan::scan_blocks`
  - `plan::plan_rewrites`
  - `apply::apply_rewrites`

### Step 2: stage 関数の可視性を縮退

- `scan.rs` / `apply.rs` / `plan/mod.rs` の関数を `pub(super) fn ...` に変更。

### Step 3: `instruction_rewriter.rs` の import を単純化

- `stages::scan::...` などの “ファイル名 import” を削除して、`stages::{...}` に統一。

### Step 4: compile/check

- `cargo check -p nyash-rust --lib` が通ること。
- warnings は増やさない（既存は許容）。

---

## 検証手順（受け入れ基準）

```bash
cargo build --release
./tools/smokes/v2/run.sh --profile quick
./target/release/hakorune --backend vm apps/tests/phase1883_nested_minimal.hako   # RC=9
```

受け入れ:
- Build: 0 errors
- quick: 154/154 PASS
- Pattern6: RC=9 維持
- 恒常ログ増加なし
