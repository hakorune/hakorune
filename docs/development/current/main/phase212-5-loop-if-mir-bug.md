# Phase 212.5: ループ内 if → MIR 変換バグ修正（緊急ミニフェーズ）

**Phase**: 212.5
**Date**: 2025-12-09
**Status**: 🔧 In Progress
**Prerequisite**: Phase 212 完了（制約発見）

---

## 🎯 Phase 212.5 の目的

Phase 212 で発見した「ループ内 if/else が MIR に変換されない」問題を修正する。

**戦略**:
- JoinIR に触らない（AST→MIR Builder だけを修正）
- 既存の If lowering 箱を再利用
- 最小限の変更で根治

---

## Task 212.5-1: 現状の AST / MIR を確認 ✅

### テストファイル

**`apps/tests/phase212_if_sum_min.hako`**:

```hako
static box IfSumTest {
    sum_def_count(defs) {
        local sum = 0
        local i = 0
        local len = 3

        loop(i < len) {
            // ← この if がループ本体に含まれるはず
            if i > 0 {
                sum = sum + 1  // ← 条件付き更新
            }
            i = i + 1
        }
        return sum
    }

    main() {
        local result = IfSumTest.sum_def_count(0)
        return result
    }
}
```

### 期待される AST 構造

ループ本体の AST ノードには以下が含まれるはず：

```
Loop {
    condition: BinaryOp(Lt, i, len),
    body: Block [
        // ← If ノードがここにあるはず
        If {
            condition: BinaryOp(Gt, i, 0),
            then_block: Block [
                Assignment(sum, BinOp(Add, sum, 1))
            ],
            else_block: None
        },
        Assignment(i, BinOp(Add, i, 1))
    ]
}
```

### 実際の MIR 出力（Before）

```mir
define i64 @IfSumTest.sum_def_count/1(? %0) effects(read) {
bb1:
    %2 = const 0    ; sum 初期化
    %4 = const 0    ; i 初期化
    br label bb3

bb2:
    ret %2          ; return sum

bb3:
    %7 = phi [%4, bb1], [%16, bb6]  ; ← i の PHI のみ
    br label bb4

bb4:
    %12 = const 3
    %13 = icmp Lt %7, %12
    %14 = Not %13
    br %14, label bb5, label bb6

bb5:
    br label bb2

bb6:
    ; ← ここに if 由来の Compare / Branch が無い！
    extern_call env.console.log(%7) [effects: pure|io]
    %15 = const 1
    %16 = %7 Add %15
    %7 = copy %16
    br label bb3
}
```

### 問題点の詳細

**欠落している MIR 命令**:

bb6 ループ本体ブロックには以下があるべき：

```mir
bb6:
    ; ← if i > 0 の条件チェック
    %const_0 = const 0
    %cond = icmp Gt %7, %const_0
    br %cond, label bb_then, label bb_else

bb_then:
    ; sum = sum + 1
    %sum_phi = phi [%2, bb3], [%sum_updated, bb_else]  ; ← sum の PHI
    %const_1 = const 1
    %sum_updated = %sum_phi Add %const_1
    br label bb_merge

bb_else:
    br label bb_merge

bb_merge:
    %sum_final = phi [%sum_updated, bb_then], [%sum_phi, bb_else]
    ; i = i + 1
    %15 = const 1
    %16 = %7 Add %15
    br label bb3
```

**実際には**:
- if 由来の `Compare` / `Branch` が一切無い
- `sum` 変数に関する処理（PHI・加算）が完全に消失

### 仮説: どの層が壊しているか

#### ✅ Parser 層は OK

理由:
- Phase 212 で print を if 内に追加しても同じ結果
- Parser が if ノードを落としているなら、syntax error になるはず
- → **Parser は正しく AST を生成している可能性が高い**

#### ❌ LoopForm / control_flow builder が怪しい

**仮説 1**: ループ本体の AST ノードが **フラット化** されている
- `loop { stmt1; stmt2; }` の各 stmt を順次処理する際、
- `stmt` が `If` ノードの場合に **match していない** 可能性

**仮説 2**: ループ本体の `build_block()` が If を **スキップ** している
- `build_block()` が Statement を処理する際、
- `Statement::Expr(If)` を **式として評価** せずに無視している可能性

**仮説 3**: If が **Dead Code Elimination (DCE)** で消えている
- `sum` の値が return で使われているから DCE で消えないはず
- でも念のため確認が必要

---

## Task 212.5-2: MIR Builder の責務位置を特定 ✅

### 確認したファイル

1. ✅ **`src/mir/builder/stmts.rs`** - Statement 処理
2. ✅ **`src/mir/builder/exprs.rs`** - Expression 処理
3. ✅ **`src/mir/builder/control_flow/mod.rs`** - cf_if(), cf_loop()

### 問題の根本原因を特定

#### 🚨 **発見した問題**

**`build_statement()` (stmts.rs:215-222)**:

```rust
pub(super) fn build_statement(&mut self, node: ASTNode) -> Result<ValueId, String> {
    self.current_span = node.span();
    match node {
        // 将来ここに While / ForRange / Match / Using など statement 専用分岐を追加する。
        other => self.build_expression(other),  // ← すべて expression として処理
    }
}
```

**問題点**:
- `ASTNode::If` のケースが **match に存在しない**
- すべての Statement が `other =>` で `build_expression()` に投げられる
- **If が式として評価される** → 値が使われない場合に最適化で消える可能性

#### If の処理フロー（現状）

```
build_statement(ASTNode::If)
  ↓
  match { other => build_expression(other) }  ← If ケースなし
  ↓
build_expression(ASTNode::If)
  ↓
  match { ASTNode::If { ... } => self.cf_if(...) }  ← ここで処理
  ↓
cf_if(condition, then_branch, else_branch)
  ↓
lower_if_form(...)  ← JoinIR ベースの PHI 生成
```

#### 新しい仮説

**仮説 1**: If が式として評価され、**値が使われない**ため DCE で消える
- ループ内の `if i > 0 { sum = sum + 1 }` は Statement として書かれている
- でも `build_statement()` は `build_expression()` に投げる
- `build_expression()` は ValueId を返すが、ループ本体では **その値を使わない**
- → 最適化 (DCE) で If ブロック全体が消える？

**仮説 2**: ループ本体の AST が JoinIR 経路で **フラット化** されている
- `cf_loop()` → `try_cf_loop_joinir()` の経路で
- ループ本体の AST ノードが別の形式に変換される際に If が消失

**仮説 3**: `lower_if_form()` がループ内 if を **スキップ** している
- `lower_if_form()` が「ループ外の if のみ対応」の可能性
- ループ内 if は別の処理が必要だが、その処理が未実装

### 次の調査対象

1. **DCE (Dead Code Elimination)** の動作確認
   - If 式の戻り値が使われない場合に DCE で消えるか？

2. **`try_cf_loop_joinir()` の実装確認**
   - ループ本体の AST がどう処理されているか
   - If ノードが JoinIR 変換時に保持されているか

3. **`lower_if_form()` の実装確認**
   - ループ内 if でも正しく動作するか
   - ループコンテキストでの制約があるか

---

## Task 212.5-3: 小さな箱として if-lowering を足す 🔧

### 根本原因の確定

**問題**:
- `build_statement()` が `ASTNode::If` を **expression 経路にだけ流していた**
- Statement としての If（副作用のみが欲しい）が expression として評価される
- → 値が使われないと最適化で消える

**対応方針**:
- **Option A** を採用: `build_statement()` に statement 用の If ケースを追加
- 既存の If lowering 箱 (`cf_if` / `lower_if_form`) を呼ぶだけ

### 設計方針

**原則**:
- 新規巨大箱は作らない
- 既存の If lowering 箱を再利用
- Statement と Expression の If を明確に分離

### 実装戦略（Option A）

#### 修正箇所: `src/mir/builder/stmts.rs`

**Before**:
```rust
pub(super) fn build_statement(&mut self, node: ASTNode) -> Result<ValueId, String> {
    self.current_span = node.span();
    match node {
        // TODO: While / ForRange / Match / Using …
        other => self.build_expression(other),  // ← If も expression 扱い
    }
}
```

**After**:
```rust
pub(super) fn build_statement(&mut self, node: ASTNode) -> Result<ValueId, String> {
    self.current_span = node.span();
    match node {
        ASTNode::If { condition, then_body, else_body, .. } => {
            // Statement としての If - 既存 If lowering を呼ぶ
            self.build_if_statement(*condition, then_body, else_body)?;
            // Statement なので値は使わない（Void を返す）
            Ok(crate::mir::builder::emission::constant::emit_void(self))
        }
        // 将来: While / ForRange / Match / Using など
        other => self.build_expression(other),
    }
}
```

#### 新規関数: `build_if_statement()`

既存の If lowering を薄くラップする小さい箱：

```rust
/// Statement としての If 処理（副作用のみ）
///
/// ループ内 if や top-level statement if はここを通る。
/// Expression としての if（値を使う場合）は build_expression 経由。
pub(super) fn build_if_statement(
    &mut self,
    condition: ASTNode,
    then_body: Vec<ASTNode>,
    else_body: Option<Vec<ASTNode>>,
) -> Result<(), String> {
    use crate::ast::Span;

    // then_body と else_body を ASTNode::Program に変換
    let then_node = ASTNode::Program {
        statements: then_body,
        span: Span::unknown(),
    };
    let else_node = else_body.map(|b| ASTNode::Program {
        statements: b,
        span: Span::unknown(),
    });

    // 既存の If lowering を呼ぶ（cf_if は lower_if_form を呼ぶ）
    self.cf_if(condition, then_node, else_node)?;

    Ok(())
}
```

### Expression vs Statement の分離

**Expression としての If** (既存のまま):
```hako
local x = if cond { 1 } else { 2 }  // ← 値を使う
```
→ `build_expression()` 経由で処理

**Statement としての If** (今回追加):
```hako
if i > 0 { sum = sum + 1 }  // ← 副作用のみ
```
→ `build_statement()` 経由で処理

### 重要なポイント

1. **JoinIR 側には触らない**
   - 今は素の MIR だけ直す
   - JoinIR Pattern3 (IfPHI) は Phase 212.5 完了後に使う

2. **既存 If lowering を再利用**
   - `cf_if()` → `lower_if_form()` の既存パスをそのまま使う
   - **ループ内 if も top-level if と同じ構造**（特別扱いしない）

3. **1 箇所だけで修正**
   - `build_statement()` に If ケースを追加するだけ
   - 複数箇所で同じことをしない（DRY 原則）

---

## Task 212.5-4: phase212_if_sum_min.hako で再検証 🧪

### 検証手順

#### Step 1: 素の MIR ダンプ確認

```bash
./target/release/hakorune --dump-mir apps/tests/phase212_if_sum_min.hako 2>&1 | grep -A 50 "sum_def_count"
```

**期待される MIR**:

- ループ body 内に:
  - ✅ `Compare` 命令: `%cond = icmp Gt %i, 0`
  - ✅ `Branch` 命令: `br %cond, label bb_then, label bb_else`
  - ✅ then ブロック: `sum = sum + 1` 相当の BinOp
  - ✅ PHI 命令: `sum` の merge PHI

#### Step 2: JoinIR 経由の E2E テスト

```bash
NYASH_JOINIR_CORE=1 ./target/release/hakorune apps/tests/phase212_if_sum_min.hako
```

**期待される結果**:

- RC: **2** (i=1, i=2 で sum が increment されるため)
- Pattern 3 (IfPHI) または Pattern 1 + IfPHI が選ばれる
- Carrier: `i` と `sum` の 2 つ

---

## Task 212.5-5: ドキュメント & CURRENT_TASK の更新 📝

### Before/After まとめ

**Before** (Phase 212 時点):
- ループ内 if が MIR に現れない
- `sum` 変数が carrier として認識されない
- RC: 0 (期待: 2)

**After** (Phase 212.5 完了後):
- ループ内 if が正常に MIR に変換される
- `sum` と `i` の 2-carrier が正常動作
- RC: 2 (正常)

### CURRENT_TASK.md 更新内容

```markdown
- [x] **Phase 212.5: ループ内 if の AST→MIR 修正** ✅ (完了: 2025-12-09)
      - **目的**: Phase 212 で発見した「ループ内 if が MIR に変換されない」問題を修正
      - **修正箇所**: [ファイル名・関数名]
      - **修正内容**: [具体的な変更内容]
      - **検証結果**: phase212_if_sum_min.hako で RC=2 を確認
      - **Phase 212 BLOCKED 解消**: ループ内 if の根本問題を解決
```

---

## 📊 Phase 212.5 の進捗

- [x] Task 212.5-1: 現状確認・設計メモ作成 ✅
- [ ] Task 212.5-2: MIR Builder 責務位置特定
- [ ] Task 212.5-3: if-lowering 追加
- [ ] Task 212.5-4: 再検証
- [ ] Task 212.5-5: ドキュメント更新

**次のステップ**: Task 212.5-2（ファイル読み込み・責務特定）
Status: Active  
Scope: loop-if MIR バグ調査（JoinIR v2）
