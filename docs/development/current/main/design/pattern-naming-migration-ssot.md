# Pattern Naming Migration SSOT

**Purpose**: Pattern*（数値ベース）命名から Recipe/Lego（意味ベース）命名への移行計画

**Status**: Completed (Phases A/B/C done; 2026-01-29)
**Last updated**: 2026-01-29

---

## Scope

### 対象 (In Scope)

- **コード内の型名**: `Pattern1SimpleWhilePlan` などの旧 planner payload 型名
- **関数名**: `try_extract_pattern1_simplewhile` などの関数名
- **モジュール名**: `pattern1_simple_while.rs` などのモジュール名
- **構造体フィールド**: Pattern 構造体のフィールド名

### 非対象 (Out of Scope)

- **ログタグ**: `pattern1`, `pattern2` などのログ出力（当面維持、別途検討）
- **TSV期待値**: テスト用 TSV ファイル内のパターン識別子（当面維持）
- **エラーメッセージ**: ユーザー向けエラーメッセージ内の Pattern 言及（当面維持）
- **ドキュメント内の説明**: 既存ドキュメントでの Pattern* 表現（追って更新）

---

## Goals / Non-goals

### Goals (達成目標)

1. **意味ベース命名への統一**: Pattern1/2/3 という数値命名を、意味を表す名前に置換
2. **Recipe/Lego 命名との整合性**: ドメインモデル（Recipe/Composer/Lego）との命名一貫性
3. **コード可読性向上**: 新規開発者が Pattern* の意味を即座に理解できるようにする
4. **段階的移行**: 型別名を活用し、漸進的な移行を可能にする

### Non-goals (対象外)

1. **ログ/TSV の一斉変更**: ログタグや TSV ファイルの変更は別フェーズで検討
2. **既存ゲートの挙動変更**: テストゲートや検証ロジックの変更を伴わない
3. **パターン機能の追加/削除**: 移行のみで、機能の増減なし
4. **パフォーマンス向上**: 命名変更のみで、最適化は対象外

---

## Naming Rules (SSOT)

### 基本命名規則

| カテゴリ | Suffix | 例 | 説明 |
|---------|--------|------|------|
| **Facts** | `*Facts` | `SimpleWhileFacts` | 抽出された事実 |
| **Recipe** | `*Recipe` | `SimpleWhileRecipe` | レシピ（検証済みFacts） |
| **Composer** | `*Composer` | `BreakConditionComposer` | レシピ組み立て |
| **Verifier** | `*Verifier` | `BreakConditionVerifier` | レシピ検証 |
| **Builder** | `*Builder` | `SimpleWhileBuilder` | Facts→Recipe 変換 |
| **Normalizer** | `*Normalizer` | `SimpleWhileNormalizer` | Recipe→CorePlan 変換 |

### 意味ベース命名規則

| Pattern 機能 | 推奨名 | 説明 |
|-------------|--------|------|
| **単純ループ** | `LoopSimpleWhileRecipe` | break/continue なしの単純な while ループ |
| **文字変換** | `CharMapRecipe` | 文字変換ループ (to_lower/to_upper) |
| **配列結合** | `ArrayJoinRecipe` | 配列要素の結合ループ |
| **条件付き break** | `LoopBreakRecipe` | break 条件があるループ |
| **条件付き更新 (if-phi)** | `IfPhiJoinRecipe` | if-else でキャリア変数を更新 |
| **条件付き continue** | `LoopContinueOnlyRecipe` | continue 文を含むループ |
| **無限ループ脱出** | `LoopTrueEarlyExitRecipe` | loop(true) + early exit |
| **スキャン (索引)** | `ScanWithInitRecipe` | 索引ベースの文字列スキャン |
| **スキャン (split)** | `SplitScanRecipe` | split ベースのスキャン |
| **述語スキャン** | `BoolPredicateScanRecipe` | 述語関数を使うスキャン |
| **累積ループ** | `AccumConstLoopRecipe` | 定数累積ループ |

### 命名規則の階層

```
[機能/形状] + [Recipe]

例:
- LoopSimpleWhileRecipe     : 機能=Loop, 形状=SimpleWhile
- CharMapRecipe             : 機能=CharMap
- ArrayJoinRecipe           : 機能=ArrayJoin
- LoopBreakRecipe           : 機能=Loop, 形状=Break
- IfPhiJoinRecipe           : 機能=IfPhi, 形状=Join
- LoopContinueOnlyRecipe    : 機能=Loop, 形状=ContinueOnly
- LoopTrueEarlyExitRecipe   : 機能=Loop, 形状=TrueEarlyExit
- ScanWithInitRecipe        : 機能=Scan, 形状=WithInit
- SplitScanRecipe           : 機能=SplitScan
- BoolPredicateScanRecipe   : 機能=BoolPredicateScan
- AccumConstLoopRecipe      : 機能=AccumConstLoop
```

### 命名のポイント

1. **Recipe を明示**: すべての型名に `Recipe` サフィックスを付ける
2. **既存用語を維持**: Loop/Scan/Join など既存の用語を残す
3. **形状を明示**: SimpleWhile/Break/ContinueOnly など形状を明確にする

---

## Mapping Table (SSOT)

### 主要 Pattern の対応表

| 現在 (Pattern*) | 新名 (Recipe/Lego 明示) | カテゴリ | 概要 |
|----------------|----------------------|---------|------|
| `Pattern1SimpleWhile` | `LoopSimpleWhileRecipe` | Basic | 単純な while ループ（break/continue なし） |
| `Pattern1CharMap` | `CharMapRecipe` | Transform | 文字変換ループ (to_lower/to_upper) |
| `Pattern1ArrayJoin` | `ArrayJoinRecipe` | Transform | 配列要素の結合ループ |
| `Pattern2Break` | `LoopBreakRecipe` | Control Flow | 条件付き break ループ |
| `Pattern3IfPhi` | `IfPhiJoinRecipe` | Control Flow | if-else でのキャリア更新 (PHI join) |
| `Pattern4Continue` | `LoopContinueOnlyRecipe` | Control Flow | continue 文を含むループ |
| `Pattern5InfiniteEarlyExit` | `LoopTrueEarlyExitRecipe` | Control Flow | loop(true) + early exit |
| `Pattern6` (ScanWithInit) | `ScanWithInitRecipe` | Scan | 索引ベースのスキャン |
| `Pattern7` (SplitScan) | `SplitScanRecipe` | Scan | split ベースのスキャン |
| `Pattern8BoolPredicateScan` | `BoolPredicateScanRecipe` | Scan | 述語関数を使うスキャン |
| `Pattern9AccumConstLoop` | `AccumConstLoopRecipe` | Accumulator | 定数累積ループ |

### Historical planner-payload 型対応表

Note:
- runtime 主経路では legacy planner payload を使わない。
- この表は移行履歴の対応づけとして残す。

| 現在 | 新名 | 用途 |
|------|------|------|
| `Pattern1SimpleWhilePlan` | `LoopSimpleWhilePlan` | ドメイン計画型 |
| `Pattern2BreakPlan` | `LoopBreakPlan` | historical mapping; current test harness retired in 291x-711 |
| `Pattern3IfPhiPlan` | `IfPhiJoinPlan` | ドメイン計画型 |
| `Pattern4ContinuePlan` | `LoopContinueOnlyPlan` | ドメイン計画型 |
| `Pattern5InfiniteEarlyExitPlan` | `LoopTrueEarlyExitPlan` | ドメイン計画型 |

### Facts 型対応表

| 現在 | 新名 | ファイル位置 |
|------|------|-------------|
| `Pattern1SimpleWhileFacts` | `LoopSimpleWhileFacts` | `facts/loop_simple_while_facts.rs` |
| `Pattern2BreakFacts` | `LoopBreakFacts` | `facts/loop_break_{core,types,...}.rs` |
| `Pattern3IfPhiFacts` | `IfPhiJoinFacts` | `facts/if_phi_join_facts.rs` |
| `Pattern4ContinueFacts` | `LoopContinueOnlyFacts` | `facts/loop_continue_only_facts.rs` |
| `Pattern5InfiniteEarlyExitFacts` | `LoopTrueEarlyExitFacts` | `facts/loop_true_early_exit_facts.rs` |
| `Pattern6NestedMinimalFacts` | `NestedLoopMinimalFacts` | `facts/nested_loop_minimal_facts.rs` |
| `Pattern8BoolPredicateScanFacts` | `BoolPredicateScanFacts` | `facts/bool_predicate_scan_facts.rs` |
| `Pattern9AccumConstLoopFacts` | `AccumConstLoopFacts` | `facts/accum_const_loop_facts.rs` |

### Recipe 型対応表

| 現在 | 新名 | ファイル位置 |
|------|------|-------------|
| `Pattern1SimpleWhileRecipe` | `LoopSimpleWhileRecipe` | `recipe_tree/pattern1_simple_while_builder.rs` |
| `Pattern2BreakRecipe` | `LoopBreakRecipe` | `recipe_tree/loop_break_builder.rs` |
| `Pattern3IfPhiRecipe` | `IfPhiJoinRecipe` | `recipe_tree/pattern3_ifphi_builder.rs` |
| `Pattern4ContinueRecipe` | `LoopContinueOnlyRecipe` | `recipe_tree/pattern4_continue_builder.rs` |
| `Pattern5InfiniteEarlyExitRecipe` | `LoopTrueEarlyExitRecipe` | `recipe_tree/pattern5_infinite_early_exit_builder.rs` |

---

## Migration Phases

**Completion (2026-01-29)**:
- Phase A (aliases), Phase B (internal references), Phase C (legacy names removed) are complete.
- Logs/TSV are unchanged; Facts naming remains as-is.
- File-name renaming is deferred to the historical mapping ledger `docs/development/current/main/design/recipe-file-naming-unification-ssot.md`.

### Phase A: 型別名で互換維持

**Status**: Completed (2026-01-29)

**目的**: 新名を導入しつつ、旧名も維持

**作業内容**:
1. 新名の型を定義（旧名の型別名として）
   ```rust
   // 新定義
   pub type SimpleWhileFacts = Pattern1SimpleWhileFacts;
   pub type SimpleWhileRecipe = Pattern1SimpleWhileRecipe;
   ```
2. 新名のモジュールを re-export
   ```rust
   // plan/mod.rs
   pub use pattern1_simple_while as simple_while;
   pub use pattern2_break as conditional_break;
   ```
3. 新規コードでは新名を使用開始

**完了基準**:
- `cargo build --release --bin hakorune` 成功
- すべての新旧型が使用可能
- 既存テストが PASS

### Phase B: 内部参照を新名へ置換

**Status**: Completed (2026-01-29)

**目的**: コードベース内の参照を新名へ移行

**作業内容**:
1. 内部参照を新名へ置換（型別名は残す）
2. 新規コードでは新名のみ使用
3. 旧名の使用箇所を特定し、新名へ置換

**完了基準**:
- `cargo build --release --bin hakorune` 成功
- 新名がデフォルト使用
- 旧名は型別名のみ

### Phase C: 旧名の削除（別PR）

**Status**: Completed (2026-01-29)

**目的**: 型別名を削除し、新名に完全移行

**作業内容**:
1. 型別名を削除
2. 旧名の re-export を削除
3. モジュール名を新名へ変更

**完了基準**:
- `cargo build --release --bin hakorune` 成功
- 旧名が完全に削除
- ログ/TSV以外は新名に統一

---

## Compatibility Policy

### 旧名の保持方針

1. **型別名で維持**: Phase A-B では `pub type OldName = NewName;` で互換維持
2. **pub use で re-export**: モジュールレベルでも新旧両方を公開
3. **段階的削除**: Phase C でまとめて削除（別PR）

```rust
// Phase A-B の例
pub type Pattern1SimpleWhileFacts = SimpleWhileFacts;
pub type Pattern2BreakFacts = LoopBreakFacts;

pub use pattern1_simple_while as loop_simple_while;
pub use pattern2_break as loop_break;
```

### ログタグの扱い

- **当面維持**: `pattern1`, `pattern2` などのログタグは維持
- **別途検討**: ログ出力の命名規則は別途検討
- **TSV は維持**: テスト用 TSV のパターン識別子は変更なし

### 非破壊的移行の保証

1. **ゲート維持**: 既存のテストゲートは変更なし
2. **TSV 互換**: TSV ファイルのフォーマットは維持
3. **エラーメッセージ**: ユーザー向けメッセージは Pattern* 表現維持

---

## Verification

### ビルド検証

```bash
# Phase A 完了後
cargo build --release --bin hakorune

# 期待結果: 成功（新旧両方の型が使用可能）
```

### テスト検証

```bash
# 既存テストが PASS
cargo test --release

# 期待結果: 既存テストが変更なしで PASS
```

### ゲート検証

```bash
# TSV ゲートが維持
./tools/run_some_gate.sh

# 期待結果: TSV ファイルの変更なし
```

### 移行進捗の確認

```bash
# 旧名の使用箇所をカウント
rg "Pattern1SimpleWhile" --type rust src/
rg "Pattern2Break" --type rust src/

# 期待結果: Phase B 完了後は大幅減少
```

---

## Implementation Notes

### ファイル名の変更（別 SSOT へ移管）

- ファイル名の語彙統一は本 SSOT から切り出し、別設計として管理する。
- Historical mapping SSOT: `docs/development/current/main/design/recipe-file-naming-unification-ssot.md`

### 命名規則の例外

- **PatternN のサブバリアント**: `Pattern2Break` → `LoopBreakRecipe` （サフィックス省略）
- **Fact抽出ヘルパー**: `pattern_break_core.rs` → `loop_break_core.rs`
- **Composer/Verifier**: パターン名を前置詞として使用

---

## References

- **関連 SSOT**:
  - `docs/development/current/main/design/plan-dir-shallowing-ssot.md` - plan/ 浅層化
  - `docs/development/current/main/design/recipe-tree-and-parts-ssot.md` - Recipe/Parts
  - `docs/development/current/main/design/entry-name-map-ssot.md` - 入口名マップ

- **主要ファイル**:
  - `src/mir/builder/control_flow/plan/mod.rs` - Pattern 宣言
  - `src/mir/builder/control_flow/plan/facts/` - Facts モジュール
  - `src/mir/builder/control_flow/plan/recipe_tree/` - Recipe builders

---

## Changelog

| 日付 | 変更内容 |
|------|---------|
| 2025-01-29 | 初版作成 - SSOT ドキュメント作成 |
| 2026-01-29 | Phases A/B/C 完了の反映、Status 更新 |
