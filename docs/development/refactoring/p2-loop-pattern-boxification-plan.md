# Phase P2: Loop Pattern enum化 実装計画書

## 目次
1. [現状分析](#1-現状分析)
2. [LoopPattern enum 設計案](#2-looppattern-enum-設計案)
3. [モジュール構造案](#3-モジュール構造案)
4. [削減見込み](#4-削減見込み)
5. [実装ステップ](#5-実装ステップ)
6. [リスク評価](#6-リスク評価)
7. [テスト戦略](#7-テスト戦略)
8. [チェックリスト](#8-チェックリスト)

---

## 1. 現状分析

### 1.1 対象ファイル概要

**ファイル**: `src/mir/join_ir/frontend/ast_lowerer/loop_patterns.rs`

**行数**: 895行

**主要関数**:
1. `lower_loop_with_break_continue()` (43行) - **ディスパッチャー**
2. `lower_loop_case_a_simple()` (329行) - **Simple pattern**
3. `lower_loop_break_pattern()` (202行) - **Break pattern**
4. `lower_loop_continue_pattern()` (241行) - **Continue pattern**

### 1.2 ループパターンの分類

| パターン | 条件 | 関数 | 行数 | 複雑度 |
|---------|------|------|------|--------|
| **Simple** | `!has_break && !has_continue` | `lower_loop_case_a_simple()` | 329 | 高 |
| **Break** | `has_break && !has_continue` | `lower_loop_break_pattern()` | 202 | 中 |
| **Continue** | `has_continue && !has_break` | `lower_loop_continue_pattern()` | 241 | 中 |
| **Mixed** | `has_break && has_continue` | panic! | 0 | 未実装 |

### 1.3 P1 との比較

| 項目 | P1 (If Handler) | P2 (Loop Patterns) |
|------|-----------------|-------------------|
| **対象ファイル** | `stmt_handlers.rs` | `loop_patterns.rs` |
| **元の行数** | 154行 | 895行 |
| **パターン数** | 5個 | 4個（1個未実装） |
| **最大パターン** | 57行 | 329行 |
| **共通処理** | 少ない | 多い（~85行） |

---

## 2. LoopPattern enum 設計案

```rust
/// ループパターンの分類
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LoopPattern {
    /// Simple pattern: 条件付きループ（break/continue なし）
    Simple {
        cond_expr: Value,
        has_me: bool,
        external_refs: Vec<(String, ValueId)>,
    },

    /// Break pattern: 早期 return ループ
    Break {
        break_cond_expr: Value,
    },

    /// Continue pattern: 条件付きスキップループ
    Continue {
        loop_cond_expr: Value,
        continue_cond_expr: Value,
    },

    /// Mixed pattern: break と continue 両方（未実装）
    Mixed {
        _marker: (),
    },
}
```

---

## 3. モジュール構造案

```
src/mir/join_ir/frontend/ast_lowerer/
├── loop_patterns.rs (縮小版: ~60行)
└── loop_patterns/
    ├── mod.rs (~50行)
    ├── pattern.rs (~180行) - パターン検出ロジック
    ├── common.rs (~150行) - 共通処理ヘルパー
    ├── lowering/
    │   ├── mod.rs (~30行)
    │   ├── simple.rs (~180行)
    │   ├── break_pattern.rs (~120行)
    │   ├── continue_pattern.rs (~140行)
    │   └── mixed.rs (~30行)
    └── tests.rs (~150行)
```

---

## 4. 削減見込み

| カテゴリ | Before | After | 削減 | 削減率 |
|---------|--------|-------|------|--------|
| **loop_patterns.rs** | 895 | 60 | -835 | **-93.3%** |
| **新規モジュール** | 0 | 1,090 | +1,090 | - |
| **純増減** | 895 | 1,150 | +255 | +28.5% |

### P1 との比較

| 項目 | P1 (If Handler) | P2 (Loop Patterns) |
|------|-----------------|-------------------|
| **削減率** | 93.5% | 93.3% |
| **新規コスト** | +336行 (+218%) | +255行 (+28.5%) |

**P2 の優位性**: 共通処理の再利用により、新規コストが P1 より効率的

---

## 5. 実装ステップ

### Step 1: 基礎インフラ構築（3-4時間）
- ディレクトリ構造作成
- `loop_patterns/mod.rs` 作成
- `loop_patterns/pattern.rs` 実装（LoopPattern enum + 検出ロジック）
- `loop_patterns/common.rs` 実装（共通処理ヘルパー）

### Step 2: 各パターン lowering 実装（6-8時間）
1. Break pattern (2時間)
2. Continue pattern (2.5時間)
3. Simple pattern (3時間) - 最複雑
4. Mixed pattern (0.5時間)
5. lowering/mod.rs 統合 (1時間)

### Step 3: loop_patterns.rs リファクタリング（1-2時間）
- 895行 → 60行に削減
- パターン検出 → lowering 委譲

### Step 4: テスト追加（2-3時間）
- パターン検出テスト: 10個
- 共通処理テスト: 5個
- lowering テスト: 12個
- 統合テスト: 5個

### Step 5: 統合・検証（1-2時間）
- 既存テスト全通過
- 回帰テスト（11個の .hako ファイル）

**総推定時間**: 13-19時間

---

## 6. リスク評価

### 6.1 P1 より複雑な点

1. **Simple pattern の複雑性**
   - me/external_refs の動的パラメータ計算
   - P1 最大57行 vs P2 最大329行（5.8倍）

2. **共通処理の量**
   - 3関数構造生成（entry/loop_step/k_exit）
   - P1: 共通処理ほぼなし vs P2: 150行

3. **Phase 52/56 機能の保持**
   - me パラメータ対応
   - external_refs 対応（filter/map）

### 6.2 回避策

環境変数制御による段階的移行（P1 と同じ戦略）

---

## 7. テスト戦略

### 7.1 回帰テストリスト

```bash
# 基本ループ
apps/tests/loop_min_while.hako
apps/tests/joinir_min_loop.hako

# Break pattern
apps/tests/loopform_break_and_return.hako
apps/tests/nested_loop_inner_break_isolated.hako

# Continue pattern
apps/tests/loop-continue-demo/main.hako
apps/tests/loopform_continue_break_scan.hako
```

---

## 8. チェックリスト

### 基礎インフラ (Step 1)
- [ ] ディレクトリ構造作成完了
- [ ] `loop_patterns/mod.rs` 作成完了
- [ ] `loop_patterns/pattern.rs` 実装完了
- [ ] `loop_patterns/common.rs` 実装完了

### パターン lowering (Step 2)
- [ ] `lowering/break_pattern.rs` 実装完了
- [ ] `lowering/continue_pattern.rs` 実装完了
- [ ] `lowering/simple.rs` 実装完了
- [ ] `lowering/mixed.rs` 実装完了

### リファクタリング (Step 3)
- [ ] `loop_patterns.rs` 修正完了（895行 → 60行）
- [ ] 既存テスト全通過

### テスト (Step 4)
- [ ] パターン検出テスト（10個）
- [ ] 共通処理テスト（5個）
- [ ] lowering テスト（12個）

### 統合・検証 (Step 5)
- [ ] 全ユニットテスト通過
- [ ] 既存回帰テスト通過
- [ ] パフォーマンス劣化なし

---

**作成日**: 2025-11-29
**Phase**: P2 (Loop Pattern enum化)
**前提**: P1 (If Handler 箱化モジュール化) 完了
**作成者**: Claude Code + Task agent (Plan mode)
