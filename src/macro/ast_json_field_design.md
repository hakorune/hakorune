# AST JSON フィールド設計: "kind" vs "type"

## ⚠️ 重要な警告

**"kind" フィールドは削除不可！**

このファイルは `ast_to_json()` と `json_to_ast()` の往復変換、PHI生成、ループ変数追跡に不可欠です。

---

## フィールドの役割分担

### 🔵 "kind" フィールド（AST内部用）

**目的**: AST ノードタイプの正式名称（Rust enum variant と一致）

**値の例**:
- `"Variable"` - 変数参照
- `"Literal"` - リテラル値
- `"BinaryOp"` - 二項演算
- `"MethodCall"` - メソッド呼び出し
- `"FunctionCall"` - 関数呼び出し

**使用箇所**:
1. **`ast_json.rs:256` - `json_to_ast()`**
   ```rust
   let k = v.get("kind")?.as_str()?;
   Some(match k {
       "Program" => { /* ... */ }
       "Variable" => { /* ... */ }
       // ... 全ノードタイプ
   })
   ```

2. **`if_phi.rs:169-206` - PHI生成・変数抽出**
   ```rust
   match stmt.get("kind").and_then(|k| k.as_str()) {
       Some("Local") => { /* 変数宣言を抽出 */ }
       Some("Assignment") => { /* 代入を処理 */ }
       Some("If") => { /* if分岐を再帰処理 */ }
   }
   ```

3. **`loop_frontend_binding.rs:297-308` - ループ変数処理**
   ```rust
   let is_var = map.get("kind").and_then(|v| v.as_str()) == Some("Variable");
   ```

**依存コンポーネント**:
- AST ↔ JSON 双方向変換
- SSA PHI node 生成（if_phi.rs）
- ループ変数の追跡・置換

---

### 🟢 "type" フィールド（JoinIR Frontend用）

**目的**: JoinIR Frontend が期待する正規化された型名

**値の例**:
- `"Var"` - 変数参照（"Variable" の JoinIR 版）
- `"Int"`, `"Bool"`, `"String"` - リテラル型
- `"Method"` - メソッド呼び出し（"MethodCall" の JoinIR 版）
- `"Call"` - 関数呼び出し（"FunctionCall" の JoinIR 版）
- `"Binary"`, `"Compare"` - 演算子（"BinaryOp" から分離）

**使用箇所**:
1. **`stmt_handlers.rs` - Statement lowering**
   ```rust
   let stmt_type = stmt["type"].as_str().expect("stmt must have 'type' field");
   match stmt_type {
       "Local" => { /* ... */ }
       "Assignment" => { /* ... */ }
   }
   ```

2. **`expr.rs` - Expression lowering**
   ```rust
   let expr_type = expr["type"].as_str().expect("expr must have 'type' field");
   match expr_type {
       "Int" => { /* ... */ }
       "Var" => { /* ... */ }
       "Method" => { /* ... */ }
   }
   ```

**依存コンポーネント**:
- JoinIR Frontend AST lowering
- Phase 34-56 の全 lowering パス

---

## マッピングテーブル

| AST Node (kind) | JoinIR Type (type) | 説明 |
|-----------------|-------------------|------|
| `"Variable"` | `"Var"` | 変数参照 |
| `"Literal"` (Int) | `"Int"` | 整数リテラル |
| `"Literal"` (Bool) | `"Bool"` | 真偽値リテラル |
| `"Literal"` (String) | `"String"` | 文字列リテラル |
| `"MethodCall"` | `"Method"` | メソッド呼び出し |
| `"FunctionCall"` | `"Call"` | 関数呼び出し |
| `"BinaryOp"` (+,-,*,/) | `"Binary"` | 算術演算 |
| `"BinaryOp"` (<,>,==) | `"Compare"` | 比較演算 |
| `"FieldAccess"` | `"Field"` | フィールドアクセス |
| `"New"` | `"NewBox"` | Box インスタンス生成 |
| `"UnaryOp"` | `"Unary"` | 単項演算 |
| `"If"` | `"If"` | 条件分岐（同じ） |
| `"Loop"` | `"Loop"` | ループ（同じ） |
| `"Local"` | `"Local"` | 変数宣言（同じ） |
| `"Assignment"` | `"Assignment"` | 代入（同じ） |

---

## なぜ両方必要か

### 🚫 "kind" だけでは不十分

**問題**: JoinIR Frontend は統一された型名を期待
- `"Variable"` ≠ `"Var"` → 判定ロジックが複雑化
- `"BinaryOp"` では算術/比較が区別できない

### 🚫 "type" だけでは不十分

**問題**: AST 内部処理が破壊される
- `json_to_ast()` が動作不可（判定キーがない）
- PHI生成が失敗（SSA構築不可）
- ループ変数追跡が失敗

---

## 実装例（ast_json.rs）

```rust
// Phase 34: Variable with both "kind" and "type"
ASTNode::Variable { name, .. } => json!({
    "kind": "Variable",      // ← AST内部用（json_to_ast, PHI生成）
    "type": "Var",           // ← JoinIR Frontend用
    "name": name
}),

// Phase 34: MethodCall with both "kind" and "type"
ASTNode::MethodCall { object, method, arguments, .. } => json!({
    "kind": "MethodCall",    // ← AST内部用
    "type": "Method",        // ← JoinIR Frontend用
    "receiver": ast_to_json(&object),
    "method": method,
    "args": arguments.iter().map(|a| ast_to_json(a)).collect::<Vec<_>>()
}),
```

---

## 削除を試みた場合の影響

### ❌ "kind" を削除すると

| コンポーネント | 影響 |
|---------------|------|
| `json_to_ast()` | **完全に動作不可** |
| PHI生成 (if_phi.rs) | **SSA構築失敗** |
| ループ変数処理 | **変数追跡失敗** |
| テスト | 30行以上の修正必要 |

### ❌ "type" を削除すると

| コンポーネント | 影響 |
|---------------|------|
| JoinIR Frontend | **全lowering失敗** |
| Phase 34-56 | **動作不可** |
| stmt_handlers/expr | panic（type field必須） |

---

## 今後の統合計画

### Phase 57+ での統一検討

**可能性**: JoinIR Frontend への完全移行後、"kind" を廃止し "type" へ統一

**前提条件**:
1. ✅ AST ↔ JSON 変換を JoinIR ベースに書き換え
2. ✅ PHI生成を JoinIR ベースに書き換え
3. ✅ ループ変数処理を JoinIR ベースに書き換え
4. ✅ すべてのテストを JoinIR 形式に移行

**時期**: Phase 60+ 以降（大規模アーキテクチャ変更）

---

## まとめ

| 項目 | 結論 |
|------|------|
| "kind" 削除 | ❌ **不可**（AST内部で必須） |
| "type" 削除 | ❌ **不可**（JoinIR Frontendで必須） |
| 両立 | ✅ **必須**（用途が異なる） |
| 冗長性 | ⚠️ 設計上の trade-off |
| 将来統合 | ✅ **Phase 60+で検討可** |

---

**作成日**: 2025-11-29
**Phase**: 56 (ArrayExtBox.filter JoinIR lowering 完了後)
**調査**: Task agent (Explore mode)
**理由**: "kind" フィールド削除の提案が不可能と判明
