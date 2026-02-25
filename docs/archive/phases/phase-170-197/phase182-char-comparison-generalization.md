# Phase 182: CharComparison ループの汎用化（設計・命名調整）

## 概要

Phase 171-180 で実装した "Trim" パターンは、実は「特定の文字と比較するループ」の特例である。
次フェーズ以降、JsonParser などで同様のパターン（例: `ch == '"'`, `ch == ','` など）が出てくる。

Phase 182 では、内部的な実装は変えずに、ドキュメント・コメント・型名を
「Trim 専用」から「CharComparison パターン」として一般化する準備をする。

## 現在の用語体系

### Phase 171-180 での呼称

| 用語 | 場所 | 役割 |
|------|------|------|
| TrimLoopHelper | `src/mir/loop_pattern_detection/trim_loop_helper.rs` | Trim パターンの条件生成 |
| TrimPatternInfo | loop_body_carrier_promoter.rs | Trim 昇格検出結果 |
| TrimLoopLowering module | `src/mir/builder/control_flow/joinir/patterns/trim_loop_lowering.rs` | Trim ループ前処理 lowerer |
| TrimPatternLowerer | trim_pattern_lowerer.rs | JoinIR 条件生成（Trim専用） |
| TrimPatternValidator | trim_pattern_validator.rs | Trim パターン構造検証 |
| LoopBodyCarrierPromoter | loop_pattern_detection/ | キャリア昇格検出（汎用） |

### 将来の呼称（Phase 183+）

| 用語 | 対応する現在名 | 新しい役割 |
|------|---------------|-----------|
| CharComparisonHelper | TrimLoopHelper | **文字比較ループ**の条件生成（汎用） |
| CharComparisonPatternInfo | TrimPatternInfo | **文字比較パターン**の昇格検出結果 |
| CharComparisonLowering module | TrimLoopLowering module | **文字比較パターン**ループ前処理 lowerer |
| CharComparisonPatternLowerer | TrimPatternLowerer | JoinIR 条件生成（文字比較汎用） |
| CharComparisonPatternValidator | TrimPatternValidator | 文字比較パターン構造検証 |
| LoopBodyCarrierPromoter | (変更なし) | キャリア昇格検出（汎用、変更なし） |

## 用語一般化の意図

### Trim → CharComparison へ

**Trim パターン**は、以下の構造を持つ：
```nyash
loop(p < s.length()) {
    local ch = s.substring(p, p+1)
    if ch == ' ' or ch == '\n' { p = p + 1; continue }
    break
}
```

これは本質的に「**文字比較ループ**」であり、以下のパターンでも同じ構造：

#### JsonParser: Quote Check
```nyash
loop(p < s.length()) {
    local ch = s.substring(p, p+1)
    if ch == '"' { break }  // 引用符を見つけたら終了
    p = p + 1
}
```

#### JsonParser: Delimiter Check
```nyash
loop(p < s.length()) {
    local ch = s.substring(p, p+1)
    if ch == ',' or ch == ']' { break }  // デリミタを見つけたら終了
    p = p + 1
}
```

#### 汎用の文字比較ループ
```nyash
loop(p < s.length()) {
    local ch = s.substring(p, p+1)
    if <character_comparison> { <action> }
    p = p + 1
}
```

**共通点**:
- ループ変数: 文字列インデックス（`p`, `i` など）
- キャリア: LoopBodyLocal の文字変数（`ch`）
- 比較対象: 特定の文字（whitespace, quote, delimiter など）
- アクション: break, continue, または複雑な処理

**一般化の価値**:
- Trim は whitespace 比較の特例
- JsonParser では quote, delimiter, escape 文字など多種類の比較が必要
- 同じ lowering ロジックで複数のパターンに対応可能

## Phase 182 作業内容（コード変更なし）

### 1. ドキュメント・docstring 更新

**対象ファイル群**:
- `src/mir/builder/control_flow/joinir/patterns/trim_loop_lowering.rs`
- `src/mir/builder/control_flow/joinir/patterns/trim_pattern_lowerer.rs`
- `src/mir/builder/control_flow/joinir/patterns/trim_pattern_validator.rs`
- `src/mir/loop_pattern_detection/trim_loop_helper.rs`
- `src/mir/loop_pattern_detection/loop_body_carrier_promoter.rs`
- 各ファイルの crate-level comment

**更新内容**:

```rust
// Before
//! Phase 171: Trim pattern detection and lowering for JsonParser

// After
//! Phase 171+: Character Comparison pattern detection and lowering
//!
//! Initially implemented for Trim pattern (whitespace comparison).
//! Generalized for any character comparison patterns (e.g., quoted string parsing, delimiter matching).
//!
//! ## Examples
//! - Trim: `ch == ' '` (whitespace check)
//! - JsonParser: `ch == '"'` (quote check), `ch == ','` (delimiter check)
//! - CSV Parser: `ch == ','` (delimiter), `ch == '\n'` (line break)
```

### 2. 型・関数のコメント更新

#### TrimLoopHelper
```rust
// Before
pub struct TrimLoopHelper { ... }

// After
/// Character comparison loop helper (initially for Trim patterns)
///
/// Used by loops like:
/// - Trim: `ch == ' '` (whitespace check)
/// - JsonParser: `ch == '"'` (quote check), `ch == ','` (delimiter check)
/// - CSV Parser: `ch == ','` (delimiter), `ch == '\n'` (line break)
///
/// ## Renaming Plan (Phase 183+)
/// This struct will be renamed to `CharComparisonHelper` in Phase 183.
pub struct TrimLoopHelper { ... }
```

#### TrimPatternLowerer
```rust
// Before
pub struct TrimPatternLowerer;

// After
/// Trim/CharComparison pattern lowerer (JoinIR condition generation)
///
/// Initially implemented for Trim patterns (whitespace comparison).
/// Generalized for any character comparison patterns.
///
/// ## Renaming Plan (Phase 183+)
/// This struct will be renamed to `CharComparisonPatternLowerer` in Phase 183.
pub struct TrimPatternLowerer;
```

#### TrimPatternValidator
```rust
// Before
pub struct TrimPatternValidator;

// After
/// Trim/CharComparison pattern validator (structure validation)
///
/// Validates character comparison loop structure:
/// - LoopBodyLocal carrier (e.g., `ch`)
/// - Character comparison condition (e.g., `ch == ' '`)
/// - Substring pattern detection
///
/// ## Renaming Plan (Phase 183+)
/// This struct will be renamed to `CharComparisonPatternValidator` in Phase 183.
pub struct TrimPatternValidator;
```

### 3. 用語統一の計画メモ

**Phase 183+ での段階的リネーム**:

#### Step 1: 型エイリアス追加（互換性維持）
```rust
// Phase 183
pub type TrimLoopHelper = CharComparisonHelper;
pub type TrimPatternInfo = CharComparisonPatternInfo;

#[deprecated(note = "Use CharComparisonPatternLowerer instead")]
pub type TrimPatternLowerer = CharComparisonPatternLowerer;
```

#### Step 2: ファイル名変更
```
src/mir/builder/control_flow/joinir/patterns/
  trim_loop_lowering.rs → char_comparison_lowering.rs
  trim_pattern_lowerer.rs → char_comparison_pattern_lowerer.rs
  trim_pattern_validator.rs → char_comparison_pattern_validator.rs

src/mir/loop_pattern_detection/
  trim_loop_helper.rs → char_comparison_helper.rs
```

#### Step 3: 旧名の非推奨化
```rust
#[deprecated(since = "Phase 183", note = "Use CharComparisonHelper instead")]
pub type TrimLoopHelper = CharComparisonHelper;
```

#### Step 4: 完全置換（Phase 184+）
- 型エイリアス削除
- 旧名の完全廃止
- ドキュメント内の "Trim" → "CharComparison" 一斉置換

### 4. 互換性の考慮

**段階的リネーム戦略**:
- **Phase 182**: ドキュメント・コメントのみ更新（実装変更なし）
- **Phase 183**: 型エイリアス追加、ファイル名変更、新名称導入
- **Phase 184**: 旧名削除、完全統一

**移行期間**:
- Phase 183-184 の間、旧名と新名を共存させる
- 型エイリアスで互換性維持
- docstring に `#[deprecated]` マークを付ける

## 実装例（Phase 183 以降向け）

### CharComparisonHelper の使用例

```rust
// Phase 182: ドキュメント更新のみ（この段階では実装なし）
//
// docstring に記載:
// "This will be renamed to CharComparisonHelper in Phase 183"
// "Initially used for Trim patterns; generalized for any char comparison"

// Phase 183+: 実装例（参考）
impl CharComparisonHelper {
    pub fn new(carrier_name: String, comparison_chars: Vec<String>) -> Self {
        // whitespace以外の文字比較にも対応
        Self {
            carrier_name,
            comparison_chars,
        }
    }

    pub fn for_trim() -> Self {
        // Trim パターン専用ヘルパー（後方互換性）
        Self::new("ch".to_string(), vec![" ".to_string(), "\n".to_string(), "\t".to_string()])
    }

    pub fn for_quote_check() -> Self {
        // JsonParser 引用符チェック
        Self::new("ch".to_string(), vec!["\"".to_string()])
    }

    pub fn for_delimiter_check() -> Self {
        // JsonParser デリミタチェック
        Self::new("ch".to_string(), vec![",".to_string(), "]".to_string(), "}".to_string()])
    }
}
```

## 次フェーズへの提案

### Phase 183: 実装・リネーム フェーズ

**目標**: 型名・ファイル名を実際にリネーム、互換性維持

**タスク**:
- [ ] `trim_loop_lowering.rs` → `char_comparison_lowering.rs` リネーム
- [ ] `TrimLoopHelper` → `CharComparisonHelper` リネーム（型エイリアス付き）
- [ ] `TrimPatternLowerer` → `CharComparisonPatternLowerer` リネーム
- [ ] `TrimPatternValidator` → `CharComparisonPatternValidator` リネーム
- [ ] テスト・ドキュメント内の明示的な "Trim" → "CharComparison" 置換
- [ ] 新規の JsonParser ループ（_parse_string 等）でも同モジュール使用

**互換性**:
```rust
// 型エイリアスで旧コード動作保証
pub type TrimLoopHelper = CharComparisonHelper;
pub type TrimPatternInfo = CharComparisonPatternInfo;

#[deprecated(note = "Use CharComparisonHelper instead. Will be removed in Phase 184.")]
pub type TrimLoopHelper = CharComparisonHelper;
```

### Phase 184+: 完全置換フェーズ

**目標**: 旧名完全廃止、用語統一完了

**タスク**:
- [ ] 型エイリアス削除
- [ ] 旧名への参照を全て新名に置換
- [ ] `#[deprecated]` マーク削除
- [ ] ドキュメント最終確認

## Pattern5 (Trim/CharComparison) の拡張性

### 現在のサポート範囲

**Trim パターン**:
- whitespace 比較（`ch == ' '`, `ch == '\n'`, `ch == '\t'`）
- キャリア: `ch` (LoopBodyLocal)
- アクション: continue（whitespace skip）

### Phase 183+ での拡張

**JsonParser パターン**:
- quote 比較（`ch == '"'`）
- delimiter 比較（`ch == ','`, `ch == ']'`, `ch == '}'`）
- escape 文字比較（`ch == '\\'`）
- キャリア: `ch` (LoopBodyLocal)
- アクション: break, continue, 複雑な処理

**CSV Parser パターン**:
- delimiter 比較（`ch == ','`）
- line break 比較（`ch == '\n'`）
- quote 処理（`ch == '"'`）

**汎用文字比較ループ**:
- 任意の文字列との比較
- 複数文字の OR 条件
- 否定条件（`ch != '...'`）

## 実装の安全性

### Conservative Approach

1. **Phase 182**: ドキュメント・コメントのみ更新（実装変更なし）
2. **Phase 183**: 型エイリアス・ファイル名変更（後方互換性維持）
3. **Phase 184**: 旧名削除（完全統一）

### テスト戦略

**既存テストの継続動作保証**:
```bash
# Trim pattern tests
cargo test --release --lib trim

# JsonParser tests (uses Trim pattern)
./target/release/hakorune apps/tests/test_jsonparser_skip_whitespace.hako

# Pattern2 tests
cargo test --release --lib pattern2

# Pattern4 tests
cargo test --release --lib pattern4
```

## アーキテクチャへの影響

### joinir-architecture-overview.md への追記

**更新内容**:
```markdown
### Pattern 5: CharComparison (Trim-like) Loops

**Previously Known As**: Trim Pattern (Phase 171-180)

**Generalized**: Character comparison loops (Phase 182+)

**Examples**:
- Trim: `ch == ' '` (whitespace skip)
- JsonParser: `ch == '"'` (quote detection), `ch == ','` (delimiter detection)
- CSV Parser: `ch == ','` (field delimiter), `ch == '\n'` (line break)

**Components**:
- `CharComparisonHelper` (was `TrimLoopHelper`) - Pattern data structure
- `CharComparisonPatternLowerer` (was `TrimPatternLowerer`) - JoinIR condition generation
- `CharComparisonPatternValidator` (was `TrimPatternValidator`) - Pattern structure validation
- `LoopBodyCarrierPromoter` - Carrier promotion (shared, unchanged)

**Lowering Module**: `src/mir/builder/control_flow/joinir/patterns/char_comparison_lowering.rs`
```

## 関連ドキュメント

- `phase180-trim-module-design.md` - Trim module 当初の設計
- `phase171-c-trim-pattern-lowering.md` - Phase 171 実装詳細
- `phase181-jsonparser-loop-roadmap.md` - JsonParser 次実装予定
- `joinir-architecture-overview.md` - JoinIR アーキテクチャ全体図

## タイムライン

### Phase 182（このフェーズ）
- **Task 182-1**: 設計ドキュメント作成（このファイル） - 完了
- **Task 182-2**: docstring 更新（オプション） - 10-15分

### Phase 183（次フェーズ）
- **Task 183-1**: 型名・ファイル名リネーム - 30分
- **Task 183-2**: 型エイリアス追加 - 10分
- **Task 183-3**: テスト確認 - 15分

### Phase 184+（最終フェーズ）
- **Task 184-1**: 旧名削除 - 20分
- **Task 184-2**: 完全統一確認 - 10分

**Total Estimated Time**: 95-100分（3フェーズ合計）

## 成功基準

### Phase 182
- ✅ 設計ドキュメント作成完了
- ✅ docstring に汎用化の意図を明記
- ✅ Phase 183+ の実装計画を明確化

### Phase 183
- ✅ 型名・ファイル名リネーム完了
- ✅ 型エイリアスで後方互換性維持
- ✅ 既存テスト全てパス（変更なし）

### Phase 184+
- ✅ 旧名完全削除
- ✅ ドキュメント統一
- ✅ "Trim" → "CharComparison" 用語統一完了

---

**作成日**: 2025-12-08
**Phase**: 182（CharComparison 汎用化の準備・設計）
**ステータス**: ドキュメント・計画のみ（コード変更なし）
**次フェーズ**: Phase 183（実装・リネーム）
Status: Historical
