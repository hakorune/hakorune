# If Handler 箱化モジュール化 実装計画書

## 目次
1. [現状分析](#1-現状分析)
2. [ファイル構成案](#2-ファイル構成案)
3. [IfPattern enum 設計](#3-ifpattern-enum-設計)
4. [段階的実装手順](#4-段階的実装手順)
5. [テスト戦略](#5-テスト戦略)
6. [リスク評価](#6-リスク評価)
7. [タイムライン](#7-タイムライン)
8. [チェックリスト](#8-チェックリスト)

---

## 1. 現状分析

### 1.1 対象コード概要

**ファイル**: `/home/tomoaki/git/hakorune-selfhost/src/mir/join_ir/frontend/ast_lowerer/stmt_handlers.rs`

**対象関数**: `lower_if_stmt_in_loop()` (147-300行)

**現在の実装**: 5段階のケース分岐による If 処理
- **ケース 1**: 空の If（164-166行、3行）
- **ケース 2**: 単一変数更新 then のみ（169-202行、34行）
- **ケース 3**: 両方に単一変数更新（204-234行、31行）
- **ケース 4**: Phase 56 条件付き側効果パターン（236-292行、57行）
- **ケース 5**: 複雑なケース・未対応（294-300行、7行）

**合計**: 154行（ロジック部分）

### 1.2 既存のパターン分類システム

プロジェクトには既に2つのパターン分類システムが存在：

1. **IfSelectLowerer** (`src/mir/join_ir/lowering/if_select.rs`)
   - MIR レベルの If パターン検出
   - `IfPatternType`: Simple/Local の2パターン
   - PHI 早期チェック機能
   - 176行、箱化済み

2. **IfLoweringDryRunner** (`src/mir/join_ir/lowering/if_dry_runner.rs`)
   - MIR モジュール全体スキャナー
   - パフォーマンス計測機能
   - 統計情報収集
   - 166行、箱化済み

### 1.3 設計原則（Phase 33-10）

**JoinIR は PHI 生成器（SSOT）、PHI 変換器ではない**

この原則により：
- 既存 PHI を持つ MIR は JoinIR 変換をスキップ
- JoinIR は新規 PHI のみを生成
- 既存経路との共存が可能

---

## 2. ファイル構成案

### 2.1 新規ファイル構成

```
src/mir/join_ir/frontend/ast_lowerer/
├── stmt_handlers.rs (縮小版: ~100行)
│   └── lower_statement() - エントリーポイント
│   └── lower_local_stmt() - Local 処理
│   └── lower_assignment_stmt() - Assignment 処理
│   └── lower_print_stmt() - Print 処理
│   └── lower_method_stmt() - Method 処理
│   └── lower_if_stmt_in_loop() - If ディスパッチャー（リファクタ後）
│
└── if_in_loop/
    ├── mod.rs (~40行)
    │   └── pub use declarations
    │
    ├── pattern.rs (~80行)
    │   ├── IfInLoopPattern enum
    │   ├── detect() 関数
    │   └── ヘルパー関数
    │
    ├── lowering/
    │   ├── mod.rs (~20行)
    │   ├── empty.rs (~20行) - ケース 1
    │   ├── single_var_then.rs (~50行) - ケース 2
    │   ├── single_var_both.rs (~50行) - ケース 3
    │   ├── conditional_effect.rs (~80行) - ケース 4
    │   └── unsupported.rs (~20行) - ケース 5
    │
    └── tests.rs (~120行)
        └── 各パターンのユニットテスト
```

**新規ファイル合計**: ~480行（現在154行 → +326行、構造化コストを含む）

### 2.2 ファイル役割分担

| ファイル | 責務 | 行数見積 |
|---------|------|---------|
| `if_in_loop/mod.rs` | 公開API、再エクスポート | 40 |
| `if_in_loop/pattern.rs` | パターン検出ロジック | 80 |
| `if_in_loop/lowering/mod.rs` | lowering 統合 | 20 |
| `if_in_loop/lowering/empty.rs` | 空If処理 | 20 |
| `if_in_loop/lowering/single_var_then.rs` | Then単一変数 | 50 |
| `if_in_loop/lowering/single_var_both.rs` | 両方単一変数 | 50 |
| `if_in_loop/lowering/conditional_effect.rs` | 条件付き側効果 | 80 |
| `if_in_loop/lowering/unsupported.rs` | 未対応処理 | 20 |
| `if_in_loop/tests.rs` | テストスイート | 120 |
| **合計** | | **480** |

---

## 3. IfPattern enum 設計

### 3.1 パターン定義

```rust
//! if_in_loop/pattern.rs

use crate::mir::ValueId;

/// ループ内 If 文のパターン分類
#[derive(Debug, Clone, PartialEq)]
pub enum IfInLoopPattern {
    /// ケース 1: 空の If（条件チェックのみ）
    /// if cond { } else { }
    Empty {
        cond: ValueId,
    },

    /// ケース 2: 単一変数更新 then のみ
    /// if cond { x = expr }
    SingleVarThen {
        cond: ValueId,
        var_name: String,
        then_expr: serde_json::Value,
        else_val: ValueId,  // 元の値
    },

    /// ケース 3: 両方に単一変数更新（同じ変数）
    /// if cond { x = a } else { x = b }
    SingleVarBoth {
        cond: ValueId,
        var_name: String,
        then_expr: serde_json::Value,
        else_expr: serde_json::Value,
    },

    /// ケース 4: 条件付き側効果パターン（filter 用）
    /// if pred(v) { acc.push(v) }
    ConditionalEffect {
        cond: ValueId,
        receiver_expr: serde_json::Value,
        method: String,
        args: Vec<serde_json::Value>,
    },

    /// ケース 5: 複雑なケース（未対応）
    Unsupported {
        then_stmts_len: usize,
        else_stmts_len: usize,
    },
}

impl IfInLoopPattern {
    /// If 文からパターンを検出
    pub fn detect(
        stmt: &serde_json::Value,
        ctx: &ExtractCtx,
    ) -> Result<Self, PatternDetectionError> {
        // 実装詳細は計画書本編参照
    }
}
```

---

## 4. 段階的実装手順

### Step 1: 基礎インフラ構築（2-3時間）

**目標**: ファイル構成とパターン検出ロジックの実装

**作業内容**:
1. ディレクトリ構造作成
2. `if_in_loop/mod.rs` 作成
3. `if_in_loop/pattern.rs` 実装
4. `if_in_loop/lowering/mod.rs` 作成（スケルトン）

**完了条件**:
- ✅ ファイル構造完成
- ✅ パターン検出ロジックがコンパイル可能
- ✅ 基本テストが通る

### Step 2: 各パターンの lowering 実装（4-6時間）

**ケース実装順序**:
1. ケース 1: Empty (30分)
2. ケース 2: SingleVarThen (1時間)
3. ケース 3: SingleVarBoth (1時間)
4. ケース 4: ConditionalEffect (2時間) - 最複雑
5. ケース 5: Unsupported (30分)
6. lowering/mod.rs 統合 (1時間)

**完了条件**:
- ✅ 5つのケース全て実装完了
- ✅ lowering/mod.rs で統合完了
- ✅ ユニットテストが通る

### Step 3: stmt_handlers.rs のリファクタリング（1-2時間）

**変更内容**:
```rust
// stmt_handlers.rs (リファクタ後)

use super::if_in_loop::{IfInLoopPattern, lower_pattern};

impl AstToJoinIrLowerer {
    fn lower_if_stmt_in_loop(
        &mut self,
        stmt: &serde_json::Value,
        ctx: &mut ExtractCtx,
    ) -> (Vec<JoinInst>, StatementEffect) {
        // パターン検出
        let pattern = IfInLoopPattern::detect(stmt, ctx)
            .expect("Failed to detect If pattern");

        // パターンに応じた lowering
        lower_pattern(&pattern, self, ctx)
    }
}
```

**完了条件**:
- ✅ stmt_handlers.rs が簡潔になった（154行 → 10行）
- ✅ 既存テストが全て通る
- ✅ 動作に変更がない（リファクタリングのみ）

### Step 4: テスト追加（2-3時間）

**テストケース構成**:
- 各パターンの基本ケース: 5個
- 複雑な式の処理: 5個
- エッジケース: 5個
- エラーハンドリング: 3個
- パターン検出テスト: 5個
- **合計**: 30-40個のテスト

**完了条件**:
- ✅ 各パターンごとに最低2つのテスト
- ✅ エラーケースのテスト
- ✅ 全テストが通る
- ✅ カバレッジ80%以上（理想）

### Step 5: 統合・検証（1-2時間）

**検証項目**:
- ✅ 既存テスト全て通過
- ✅ 新規テスト全て通過
- ✅ 実際の .hako ファイルで動作確認
- ✅ パフォーマンス劣化なし
- ✅ コンパイル警告ゼロ

---

## 5. テスト戦略

### 5.1 テストレベル

| レベル | 対象 | 目的 | テスト数 |
|--------|------|------|---------|
| **ユニットテスト** | パターン検出ロジック | 各パターンの正確な検出 | 10-15 |
| **統合テスト** | lowering 処理 | 正しい JoinInst 生成 | 10-15 |
| **E2Eテスト** | .hako ファイル実行 | 実際の動作確認 | 3-5 |
| **回帰テスト** | 既存テスト | 変更による影響確認 | 既存全て |

### 5.2 回帰テストリスト

既存テストケースで動作確認が必要なもの：

```bash
# Loop内If テスト
apps/tests/loop_if_phi.hako
apps/tests/loop_if_phi_continue.hako
apps/tests/loop_phi_one_sided.hako

# Macro If テスト
apps/tests/macro/if/assign.hako
apps/tests/macro/if/assign_both_branches.hako
```

---

## 6. リスク評価

### 6.1 技術的リスク

| リスク | 深刻度 | 確率 | 対策 |
|--------|--------|------|------|
| **既存テストの破損** | 高 | 中 | Step 3 で段階的移行、既存ロジック保持 |
| **パフォーマンス劣化** | 中 | 低 | 関数呼び出しオーバーヘッド最小化 |
| **新バグの混入** | 高 | 中 | 包括的テストスイート、A/Bテスト |

### 6.2 回避策：段階的移行

```rust
fn lower_if_stmt_in_loop(...) -> ... {
    if std::env::var("HAKO_USE_OLD_IF_HANDLER").is_ok() {
        // 旧実装（保持）
        self.lower_if_stmt_in_loop_legacy(stmt, ctx)
    } else {
        // 新実装
        let pattern = IfInLoopPattern::detect(stmt, ctx)?;
        lower_pattern(&pattern, self, ctx)
    }
}
```

---

## 7. タイムライン

### 7.1 スケジュール概要

| フェーズ | 期間 | 作業内容 | 完了判定 |
|---------|------|---------|---------|
| **準備** | 0.5時間 | 要件確認、設計レビュー | ✅ 計画書承認 |
| **Step 1** | 2-3時間 | 基礎インフラ構築 | ✅ パターン検出実装 |
| **Step 2** | 4-6時間 | 各パターン lowering 実装 | ✅ 5ケース全実装 |
| **Step 3** | 1-2時間 | stmt_handlers リファクタ | ✅ 委譲完了 |
| **Step 4** | 2-3時間 | テスト追加 | ✅ カバレッジ80%+ |
| **Step 5** | 1-2時間 | 統合・検証 | ✅ CI通過 |
| **合計** | **10.5-16.5時間** | | |

### 7.2 詳細スケジュール（3日間想定）

#### Day 1（4-6時間）
- **午前** (2-3時間): Step 1 基礎インフラ構築
- **午後** (2-3時間): Step 2-1, 2-2 開始

#### Day 2（4-6時間）
- **午前** (2-3時間): Step 2-3, 2-4
- **午後** (2-3時間): Step 2-5, 2-6

#### Day 3（2.5-4.5時間）
- **午前** (1-2時間): Step 3
- **午後** (1.5-2.5時間): Step 4, 5

---

## 8. チェックリスト

### 8.1 実装完了チェックリスト

#### 基礎インフラ (Step 1)
- [ ] ディレクトリ構造作成完了
- [ ] `if_in_loop/mod.rs` 作成完了
- [ ] `if_in_loop/pattern.rs` 実装完了
- [ ] コンパイル成功
- [ ] 基本パターン検出テスト通過

#### パターン lowering (Step 2)
- [ ] `lowering/empty.rs` 実装完了
- [ ] `lowering/single_var_then.rs` 実装完了
- [ ] `lowering/single_var_both.rs` 実装完了
- [ ] `lowering/conditional_effect.rs` 実装完了
- [ ] `lowering/unsupported.rs` 実装完了
- [ ] `lowering/mod.rs` 統合完了

#### リファクタリング (Step 3)
- [ ] `stmt_handlers.rs` 修正完了
- [ ] コンパイル成功
- [ ] 既存テスト全通過

#### テスト (Step 4)
- [ ] `if_in_loop/tests.rs` 作成完了
- [ ] ケース 1-5 各テスト作成
- [ ] パターン検出テスト
- [ ] カバレッジ80%以上達成

#### 統合・検証 (Step 5)
- [ ] 全ユニットテスト通過
- [ ] 全統合テスト通過
- [ ] 既存回帰テスト通過
- [ ] E2Eテスト通過
- [ ] CI/CD通過

---

## 9. 成果物

### 9.1 コード成果物

1. **新規ファイル** (9ファイル、~480行)
2. **修正ファイル** (1ファイル)
   - `stmt_handlers.rs` (330行 → 186行、-144行)

3. **純増減**
   - 新規: +480行
   - 削減: -144行
   - **実質: +336行**（構造化コスト含む）

---

## 10. 次のステップ

### 10.1 実装後の展開

1. **Phase 33-12**: IfMerge 実装（複数変数PHI対応）
2. **Phase 34**: return/break を含む If の JoinIR 表現
3. **Phase 35**: If/PHI レガシー箱の完全削除

---

**作成日**: 2025-11-29
**Phase**: 56 後のリファクタリング計画
**作成者**: Task agent (Plan mode) + Claude Code
**目的**: stmt_handlers.rs の If 処理を箱化モジュール化し、保守性と拡張性を向上
