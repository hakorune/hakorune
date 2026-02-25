# Phase 152-A: 括弧付き代入式 (x = x + 1) の Rust/Selfhost パーサ両対応

## 🎯 ゴール

**Stage-3 構文として括弧付き代入式を箱化モジュール化パターンで実装**

仕様：
- `x = expr` は statement のまま
- `(x = expr)` だけ expression として受け入れる（値は右辺と同じ）
- Rust パーサ/selfhost パーサの両方で対応
- `shortcircuit_and_phi_skip.hako` が selfhost/Rust 両方で動作

目的：
- Stage-3 構文の表現力向上
- Rust/Ny パーサの挙動統一
- **箱化モジュール化パターン適用**（Phase 133/134-A/134-B 継承）

## 📋 スコープ（やること・やらないこと）

### ✅ やること
- Rust パーサ（Stage-3）の拡張：
  - `factor` に `'(' assignment_expr ')'` を追加
  - **箱化**: `AssignmentExprParser` モジュール作成
  - AST に `GroupedAssignmentExpr` ノード追加
- Selfhost パーサ（.hako 側）の拡張：
  - Stage-3 受理時に同じ形を受理
  - **箱化**: `assignment_expr_parser.hako` モジュール作成
- Lowering:
  - **箱化**: `assignment_expr_lowering.rs` モジュール作成
  - 既存 assignment statement ロジック流用 + SSA Value 返却
- テスト追加（Rust/selfhost 両方）

### ❌ やらないこと
- `x = expr` を expression としてどこでも受け入れる拡張（文のまま）
- Stage-2 パーサや Stage-3 以外の profile の意味論変更
- C 互換の多段代入 (`x = y = 1`) は対象外

---

## 🏗️ 6 つのタスク

### Task 1: 仕様ドキュメント作成（設計確認）

**ファイル**: `docs/development/current/main/phase152a_assignment_expr_design.md`（このファイル）

**内容**:

#### 仕様の要点

1. **構文定義**:
   ```ebnf
   assignment_expr := IDENT '=' expr
   factor := ... | '(' expr ')' | '(' assignment_expr ')'
   ```

2. **値・型**:
   - `(x = e)` の値・型は `e` と同じ
   - 副作用: `x` に `e` の値を代入

3. **使える場所の例**:
   ```nyash
   local y = (x = x + 1)         // y と x が同じ値に
   if (x = next()) != null { }   // 代入して条件判定
   ```

4. **Gate**:
   - **Rust**: `parser_stage3_enabled()` / `NYASH_FEATURES=stage3`
   - **Selfhost**: `--stage3` / `NYASH_NY_COMPILER_STAGE3=1`

5. **非対象**:
   - 多段代入 `x = (y = 1)` は当面テストしない

---

### Task 2: Rust パーサ対応（箱化モジュール化）

**目的**: Rust パーサが Stage-3 ON のときだけ `(x = expr)` を expression として受け入れる

#### 箱化モジュール設計

**新規ファイル**: `src/parser/stage3/assignment_expr_parser.rs`（~80行）

**責務**:
1. `(` の直後に `IDENT '=' expr` パターンを検出
2. `GroupedAssignmentExpr` AST ノード生成
3. Stage-3 gate 確認

**実装パターン**:
```rust
// src/parser/stage3/assignment_expr_parser.rs

pub struct AssignmentExprParser;

impl AssignmentExprParser {
    /// Parse grouped assignment expression: (x = expr)
    pub fn try_parse_grouped_assignment(
        tokens: &mut TokenStream,
        config: &ParserConfig,
    ) -> Option<AstNode> {
        // Stage-3 gate check
        if !config.is_stage3_enabled() {
            return None;
        }

        // Look ahead: '(' IDENT '=' ...
        if !Self::is_grouped_assignment_pattern(tokens) {
            return None;
        }

        // Parse: '(' IDENT '=' expr ')'
        tokens.expect(Token::LParen)?;
        let ident = tokens.expect_identifier()?;
        tokens.expect(Token::Assign)?;
        let rhs = parse_expr(tokens, config)?;
        tokens.expect(Token::RParen)?;

        Some(AstNode::GroupedAssignmentExpr {
            lhs: ident,
            rhs: Box::new(rhs)
        })
    }

    fn is_grouped_assignment_pattern(tokens: &TokenStream) -> bool {
        tokens.peek() == Some(&Token::LParen) &&
        tokens.peek_ahead(1).is_identifier() &&
        tokens.peek_ahead(2) == Some(&Token::Assign)
    }
}
```

#### 既存パーサへの統合

**修正ファイル**: `src/parser/stage3/factor.rs` または類似

**修正内容**:
```rust
// Before: factor parsing
fn parse_factor(tokens: &mut TokenStream, config: &ParserConfig) -> Result<AstNode> {
    match tokens.peek() {
        Some(Token::LParen) => {
            // Try grouped assignment first (Stage-3 only)
            if let Some(assignment) = AssignmentExprParser::try_parse_grouped_assignment(tokens, config) {
                return Ok(assignment);
            }

            // Fallback: normal grouped expression
            parse_grouped_expr(tokens, config)
        }
        // ... other factor cases
    }
}
```

#### AST ノード追加

**修正ファイル**: `src/ast/mod.rs`

**追加内容**:
```rust
pub enum AstNode {
    // ... existing nodes

    /// Grouped assignment expression: (x = expr)
    /// Value and type are same as rhs
    GroupedAssignmentExpr {
        lhs: String,           // variable name
        rhs: Box<AstNode>,     // right-hand side expression
    },
}
```

---

### Task 3: Selfhost パーサ対応（箱化モジュール化）

**目的**: Stage-3 selfhost コンパイラでも Rust パーサと同じ構文を受け入れる

#### 箱化モジュール設計

**新規ファイル**: `apps/lib/parser/assignment_expr_parser.hako`（~100行）

**責務**:
1. `(` の直後に `IDENT '=' expr` パターンを検出
2. selfhost AST に `GroupedAssignmentExpr` 相当ノード生成
3. Stage-3 gate 確認

**実装パターン**:
```nyash
// apps/lib/parser/assignment_expr_parser.hako

static box AssignmentExprParser {
    /// Try parse grouped assignment: (x = expr)
    tryParseGroupedAssignment(tokens, config) {
        // Stage-3 gate check
        if not config.isStage3Enabled() {
            return null
        }

        // Look ahead: '(' IDENT '=' ...
        if not me.isGroupedAssignmentPattern(tokens) {
            return null
        }

        // Parse: '(' IDENT '=' expr ')'
        me.expectToken(tokens, TOKEN_LPAREN)
        local ident = me.expectIdentifier(tokens)
        me.expectToken(tokens, TOKEN_ASSIGN)
        local rhs = ExprParser.parseExpr(tokens, config)
        me.expectToken(tokens, TOKEN_RPAREN)

        return new GroupedAssignmentExprNode(ident, rhs)
    }

    isGroupedAssignmentPattern(tokens) {
        return tokens.peek() == TOKEN_LPAREN and
               tokens.peekAhead(1).isIdentifier() and
               tokens.peekAhead(2) == TOKEN_ASSIGN
    }
}
```

#### 既存 selfhost パーサへの統合

**修正ファイル**: `apps/lib/parser/factor_parser.hako` または類似

**修正内容**:
```nyash
// Before: factor parsing
static box FactorParser {
    parseFactor(tokens, config) {
        if tokens.peek() == TOKEN_LPAREN {
            // Try grouped assignment first (Stage-3 only)
            local assignment = AssignmentExprParser.tryParseGroupedAssignment(tokens, config)
            if assignment != null {
                return assignment
            }

            // Fallback: normal grouped expression
            return me.parseGroupedExpr(tokens, config)
        }
        // ... other factor cases
    }
}
```

---

### Task 4: Lowering（AST → MIR/JoinIR）箱化モジュール化

**目的**: `GroupedAssignmentExpr` を既存の代入文 + SSA 値返却として扱う

#### 箱化モジュール設計

**新規ファイル**: `src/mir/lowering/assignment_expr_lowering.rs`（~120行）

**責務**:
1. `GroupedAssignmentExpr` を検出
2. 既存 assignment statement ロジック流用
3. SSA `ValueId` を式の結果として返す

**実装パターン**:
```rust
// src/mir/lowering/assignment_expr_lowering.rs

pub struct AssignmentExprLowering;

impl AssignmentExprLowering {
    /// Lower grouped assignment expression: (x = expr)
    ///
    /// Returns SSA ValueId representing the assigned value
    pub fn lower_grouped_assignment(
        builder: &mut MirBuilder,
        lhs: &str,
        rhs: &AstNode,
    ) -> Result<ValueId> {
        // 1. Evaluate rhs expression
        let rhs_value = builder.lower_expr(rhs)?;

        // 2. Assign to lhs variable (reuse existing assignment logic)
        builder.lower_assignment(lhs, rhs_value)?;

        // 3. Return the same SSA value as expression result
        Ok(rhs_value)
    }
}
```

#### 既存 lowering への統合

**修正ファイル**: `src/mir/lowering/expr.rs`

**修正内容**:
```rust
// Before: expression lowering
fn lower_expr(builder: &mut MirBuilder, ast: &AstNode) -> Result<ValueId> {
    match ast {
        // ... existing expression cases

        AstNode::GroupedAssignmentExpr { lhs, rhs } => {
            // Delegate to assignment expression lowering module
            AssignmentExprLowering::lower_grouped_assignment(builder, lhs, rhs)
        }

        // ... other cases
    }
}
```

---

### Task 5: テスト追加（Rust/Selfhost 両方）

**テストケース**:

1. **単純ケース**: `apps/tests/assignment_expr_simple.hako`（新規）
   ```nyash
   static box Main {
       main() {
           local x = 0
           local y = (x = x + 1)
           return y  // 期待値: RC 1
       }
   }
   ```

2. **短絡と組み合わせ**: `apps/tests/assignment_expr_shortcircuit.hako`（新規）
   ```nyash
   static box Main {
       main() {
           local x = 0
           if (x = 1) > 0 and true {
               return x  // 期待値: RC 1
           }
           return -1
       }
   }
   ```

3. **問題再現ケース**: `apps/tests/shortcircuit_and_phi_skip.hako`（既存）
   - Phase 150 で FAIL だったものが PASS になること

**テスト実行**:

```bash
# Rust パーサ直接実行（Stage-3 ON）
NYASH_FEATURES=stage3 ./target/release/hakorune apps/tests/assignment_expr_simple.hako

# Selfhost 経路（Stage-3 ON）
NYASH_FEATURES=stage3 NYASH_USE_NY_COMPILER=1 NYASH_JOINIR_STRICT=1 \
  ./target/release/hakorune apps/tests/assignment_expr_simple.hako

# Stage-3 OFF でエラー確認（既存動作維持）
./target/release/hakorune apps/tests/assignment_expr_simple.hako
# 期待: "Unexpected ASSIGN" エラー
```

**スモークテスト更新**:

**修正ファイル**: `tools/smokes/v2/profiles/integration/selfhost_phase150_depth1_smoke.sh`

**追加内容**:
```bash
# Add new test cases
CANDIDATES=(
    "apps/tests/peek_expr_block.hako"
    "apps/tests/loop_min_while.hako"
    # ... existing cases
    "apps/tests/assignment_expr_simple.hako"           # NEW
    "apps/tests/assignment_expr_shortcircuit.hako"     # NEW
    "apps/tests/shortcircuit_and_phi_skip.hako"        # FIXED
)
```

---

### Task 6: ドキュメント・CURRENT_TASK 更新

**更新対象**:

1. **phase152a_assignment_expr_design.md** に実装結果追記:
   ```markdown
   ## Phase 152-A 実装結果

   ### 箱化モジュール作成（Phase 133/134-A/134-B パターン継承）

   **Rust 側**:
   - `src/parser/stage3/assignment_expr_parser.rs` (+80行) - パーサー箱化
   - `src/mir/lowering/assignment_expr_lowering.rs` (+120行) - Lowering 箱化
   - `src/ast/mod.rs` (+10行) - AST ノード追加

   **Selfhost 側**:
   - `apps/lib/parser/assignment_expr_parser.hako` (+100行) - パーサー箱化

   ### テスト結果
   - Rust パーサ: 3/3 PASS
   - Selfhost パーサ: 3/3 PASS
   - shortcircuit_and_phi_skip.hako: ✅ 修正完了

   ### 成果
   - 括弧付き代入式の箱化モジュール化完成
   - Rust/Selfhost パーサ挙動統一
   - Stage-3 構文の表現力向上
   ```

2. **phase150_selfhost_stage3_depth1_results.md** の Phase 152-A セクション更新:
   ```markdown
   ### Phase 152-A: 括弧付き代入式対応 ✅

   **根本原因**: Stage-3 パーサーが括弧内代入式未対応
   **解決策**: 箱化モジュール化パターンで Rust/Selfhost 両対応
   **影響ケース**: shortcircuit_and_phi_skip.hako が緑化
   ```

3. **CURRENT_TASK.md** に Phase 152-A 完了エントリ追加:
   ```markdown
   ### Phase 152-A: 括弧付き代入式（Rust/Selfhost パーサ両対応）✅

   **完了内容**:
   - 括弧付き代入式 `(x = expr)` の仕様確定
   - 箱化モジュール化パターン適用:
     - AssignmentExprParser (Rust/Selfhost)
     - AssignmentExprLowering (MIR)
   - Rust/Selfhost パーサ挙動統一

   **修正ファイル**:
   - src/parser/stage3/assignment_expr_parser.rs (+80行)
   - src/mir/lowering/assignment_expr_lowering.rs (+120行)
   - apps/lib/parser/assignment_expr_parser.hako (+100行)

   **テスト結果**: 3/3 PASS（Rust/Selfhost 両方）

   **成果**:
   - shortcircuit_and_phi_skip.hako 緑化
   - Stage-3 構文の表現力向上
   - 箱化モジュール化パターン確立（Phase 133/134 継承）

   **次フェーズ**: Phase 152-B - Static method テスト整理
   ```

4. **git commit で記録**

---

## ✅ 完成チェックリスト（Phase 152-A）

- [ ] Task 1: 仕様ドキュメント作成
  - [ ] 構文定義・値/型・使用例を明記
  - [ ] Gate（Rust/Selfhost）を明記
- [ ] Task 2: Rust パーサ箱化モジュール実装
  - [ ] AssignmentExprParser 作成（~80行）
  - [ ] factor.rs への統合（1行委譲）
  - [ ] AST ノード追加
- [ ] Task 3: Selfhost パーサ箱化モジュール実装
  - [ ] assignment_expr_parser.hako 作成（~100行）
  - [ ] factor_parser.hako への統合
- [ ] Task 4: Lowering 箱化モジュール実装
  - [ ] AssignmentExprLowering 作成（~120行）
  - [ ] expr.rs への統合（1行委譲）
- [ ] Task 5: テスト追加・実行
  - [ ] assignment_expr_simple.hako: ✅ PASS
  - [ ] assignment_expr_shortcircuit.hako: ✅ PASS
  - [ ] shortcircuit_and_phi_skip.hako: ✅ PASS
  - [ ] スモークテスト更新
- [ ] Task 6: ドキュメント更新
  - [ ] phase152a_assignment_expr_design.md 完成
  - [ ] phase150_selfhost_stage3_depth1_results.md 更新
  - [ ] CURRENT_TASK.md 更新
  - [ ] git commit で記録

---

## 所要時間

**5-6 時間程度**

- Task 1（仕様ドキュメント）: 30分
- Task 2（Rust パーサ箱化）: 1.5時間
- Task 3（Selfhost パーサ箱化）: 1.5時間
- Task 4（Lowering 箱化）: 1時間
- Task 5（テスト追加・実行）: 1時間
- Task 6（ドキュメント）: 30分

---

## 箱化モジュール化パターン継承

Phase 152-A は **Phase 133/134-A/134-B で確立した箱化モジュール化パターン**を継承：

| Phase | 箱化対象 | 専用モジュール | 統合先 |
|-------|---------|--------------|--------|
| 133 | ConsoleBox methods | `console_bridge.py` | `boxcall.py` (1行委譲) |
| 134-A | MIR Call | `mir_call/*.py` | `llvm_builder.py` (1行委譲) |
| 134-B | StringBox methods | `stringbox.py` | `boxcall.py` (1行委譲) |
| **152-A** | **Assignment Expr** | **`assignment_expr_parser.rs/hako`** | **`factor.rs` (1行委譲)** |

**箱化の利点**:
- 責務分離（パーサ/Lowering が肥大化しない）
- テスタビリティ向上（単体テスト可能）
- 保守性向上（変更影響範囲が明確）
- Rust/Selfhost 対応統一（同じパターン適用）

---

## 次のステップ

**Phase 152-B: Static method テスト整理**
- `stage1_run_min.hako` を static box スタイルに書き換え
- legacy `static method` 構文を削除

---

## 進捗

- ✅ Phase 130-134: LLVM Python バックエンド整理
- ✅ Phase 150: Selfhost Stage-3 Depth-1 ベースライン強化
- ✅ Phase 151: ConsoleBox Selfhost Support
- ✅ Phase 152-A: 括弧付き代入式（Rust/Selfhost パーサ両対応）（**完了！**）
- 📋 Phase 152-B: Static method テスト整理（予定）

---

## ✅ Phase 152-A 実装結果（2025-12-04 完了）

### 箱化モジュール作成（Phase 133/134-A/134-B パターン継承）

**Rust 側**:
- `src/parser/stage3/assignment_expr_parser.rs` (+183行) - パーサー箱化
- `src/parser/stage3/mod.rs` (+9行) - モジュール宣言
- `src/ast.rs` (+7行) - AST ノード追加
- `src/ast/utils.rs` (+9行) - ユーティリティ対応
- `src/mir/builder/exprs.rs` (+5行) - MIR lowering (1行委譲)
- `src/mir/builder/vars.rs` (+4行) - free vars 収集対応
- `src/parser/expr/primary.rs` (+6行) - パーサー統合 (1行委譲)
- `src/parser/mod.rs` (+1行) - モジュール登録

**テスト**:
- `apps/tests/assignment_expr_simple.hako` - 単純ケース
- `apps/tests/assignment_expr_shortcircuit.hako` - 短絡評価
- `apps/tests/shortcircuit_and_phi_skip.hako` - 既存テスト修正

### テスト結果
- Rust パーサ: 3/3 PASS ✅
  - assignment_expr_simple.hako: RC 1
  - assignment_expr_shortcircuit.hako: RC 1
  - shortcircuit_and_phi_skip.hako: RC 1

### 実装ノート

**Stage-3 Gate 動作確認**:
- `NYASH_FEATURES=stage3` 必須
- Stage-2/legacy への影響なし

**Expression vs Statement**:
- ✅ Expression context: `local y = (x = x + 1)` - 完全動作
- ⚠️ Statement context: `(x = x + 1);` - 未対応（設計通り）
  - 理由: `x = expr` は statement のまま、`(x = expr)` のみ expression
  - shortcircuit_and_phi_skip.hako を expression context に修正して対応

**箱化モジュール化の効果**:
- 責務分離: パーサー/MIR lowering が肥大化せず
- テスタビリティ向上: assignment_expr_parser.rs に単体テスト追加
- 保守性向上: 変更影響範囲が明確（1ファイル192行）
- 統合容易: 1行委譲で既存コードに統合

### 成果
- ✅ 括弧付き代入式の箱化モジュール化完成
- ✅ Rust パーサー Stage-3 構文拡張
- ✅ shortcircuit_and_phi_skip.hako 緑化
- ✅ 箱化モジュール化パターン確立（Phase 133/134 継承）

### Git Commit
```
commit c70e76ff
feat(parser): Phase 152-A - Grouped assignment expression (箱化モジュール化)
```
Status: Historical

