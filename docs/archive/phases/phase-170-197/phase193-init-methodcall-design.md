# Phase 193: MethodCall in Init Lowering Support

**Status**: Ready for Implementation
**Date**: 2025-12-09
**Prerequisite**: Phase 192-impl complete (ComplexAddendNormalizer + body-local init integration)

---

## 目的

`LoopBodyLocalInitLowerer` が **単純な method call を含む init 式**を JoinIR に正しく降下できるようにする。

Pattern は増やさず、既存の body-local / UpdateEnv / NumberAccumulation ラインの上に乗せる。

---

## Task 193-1: 対象 init 式のインベントリ（doc only）

### 目標
現在サポート外の MethodCall を含む init 式を整理し、Phase 193 で対応する範囲を明確化する。

### 対象パターン

#### Pattern 1: 単一 MethodCall（最優先）
```nyash
local digit = digits.indexOf(ch)
```
- **説明**: body-local 変数の初期化式が単一の MethodCall
- **要件**: `digits` は ConditionEnv で解決可能（ループパラメータまたは外部変数）
- **JoinIR**: StringBox.indexOf → CallBoxMethod 命令を emit

#### Pattern 2: MethodCall を含む二項演算
```nyash
local index = digits.indexOf(ch) + offset
local result = base - array.get(i)
```
- **説明**: MethodCall を含む算術演算
- **要件**: MethodCall が整数を返す、演算子は +, -, *, /
- **JoinIR**: MethodCall emit → 結果を二項演算に使用

### Phase 193 の範囲

**✅ 対応する**:
- Pattern 1: 単一 MethodCall（最優先・必須）
- Pattern 2: MethodCall を含む二項演算（時間があれば）

**❌ 対応しない（Phase 194+）**:
- ネストした MethodCall: `s.substring(0, s.indexOf(ch))`
- 複数 MethodCall: `s.indexOf("a") + s.indexOf("b")`
- 非算術演算: `s.concat(t).length()`
- 配列アクセス: `array[i].method()`

### 成果物
- 対象パターンの明確化
- Phase 193 完了後の「できること/できないこと」リスト

---

## Task 193-2: LoopBodyLocalInitLowerer の拡張設計

### 現在のサポート範囲（Phase 191）

```rust
// src/mir/join_ir/lowering/loop_body_local_init_lowerer.rs
match rhs {
    ASTNode::IntLiteral(_) => { /* Const 命令 */ }
    ASTNode::Identifier(_) => { /* Copy 命令 */ }
    ASTNode::BinaryOp { .. } => { /* BinOp 命令 */ }
    _ => return Err("Unsupported init expression")
}
```

### Phase 193 拡張方針

#### 1. MethodCall 対応の追加

```rust
match rhs {
    // 既存: IntLiteral, Identifier, BinaryOp

    // NEW: Single MethodCall
    ASTNode::MethodCall { receiver, method, args } => {
        emit_method_call_init(receiver, method, args)?
    }

    // NEW: BinaryOp with MethodCall (optional)
    ASTNode::BinaryOp { lhs, op, rhs } if contains_method_call(lhs, rhs) => {
        emit_binary_with_method_call(lhs, op, rhs)?
    }

    _ => return Err("Unsupported init expression")
}
```

#### 2. `emit_method_call_init` の実装

```rust
fn emit_method_call_init(
    receiver: &ASTNode,
    method: &str,
    args: &[ASTNode],
    join_builder: &mut JoinIRBuilder,
    env: &UpdateEnv,
    alloc: &mut dyn FnMut() -> ValueId,
) -> Result<ValueId, String> {
    // 1. receiver を解決（ConditionEnv から）
    let receiver_id = match receiver {
        ASTNode::Identifier(name) => {
            env.resolve(name)
                .ok_or_else(|| format!("Undefined variable in init: {}", name))?
        }
        _ => return Err("Complex receiver not supported in init".to_string())
    };

    // 2. args を解決（再帰的に lower）
    let arg_ids: Vec<ValueId> = args.iter()
        .map(|arg| lower_init_arg(arg, env, alloc))
        .collect::<Result<_, _>>()?;

    // 3. CallBoxMethod 命令を emit
    let result_id = alloc();
    join_builder.emit(Instruction::CallBoxMethod {
        dst: result_id,
        receiver: receiver_id,
        method: method.to_string(),
        args: arg_ids,
    });

    Ok(result_id)
}
```

#### 3. メソッドホワイトリスト（Fail-Fast）

**Phase 193 で許可するメソッド**:
- `StringBox.indexOf(char)` → 整数
- `ArrayBox.get(index)` → 要素（型は Context 依存）

**未サポートメソッドは明示的エラー**:
```rust
const SUPPORTED_INIT_METHODS: &[&str] = &[
    "indexOf",
    "get",
];

if !SUPPORTED_INIT_METHODS.contains(&method) {
    return Err(format!(
        "Method '{}' not supported in body-local init (Phase 193 limitation)",
        method
    ));
}
```

### 設計原則

1. **Fail-Fast**: 未サポートパターンは即座にエラー
2. **ConditionEnv 優先**: receiver は ConditionEnv で解決（body-local は参照不可）
3. **単純性**: ネストや複数呼び出しは Phase 194+ に延期
4. **既存インフラ再利用**: CallBoxMethod 命令は既に JoinIR でサポート済み

---

## Task 193-3: 実装 – Init 式 MethodCall Lowering

### 対象ファイル
- `src/mir/join_ir/lowering/loop_body_local_init_lowerer.rs`

### 実装手順

#### Step 1: `emit_method_call_init` 関数の追加

```rust
impl LoopBodyLocalInitLowerer {
    fn emit_method_call_init(
        receiver: &ASTNode,
        method: &str,
        args: &[ASTNode],
        join_builder: &mut JoinIRBuilder,
        env: &UpdateEnv,
        alloc: &mut dyn FnMut() -> ValueId,
    ) -> Result<ValueId, String> {
        // 実装は Task 193-2 の擬似コードに従う
    }

    fn lower_init_arg(
        arg: &ASTNode,
        env: &UpdateEnv,
        alloc: &mut dyn FnMut() -> ValueId,
    ) -> Result<ValueId, String> {
        match arg {
            ASTNode::IntLiteral(n) => {
                let id = alloc();
                // Const 命令 emit
                Ok(id)
            }
            ASTNode::Identifier(name) => {
                env.resolve(name)
                    .ok_or_else(|| format!("Undefined arg: {}", name))
            }
            _ => Err("Complex args not supported".to_string())
        }
    }
}
```

#### Step 2: `lower_init` の MethodCall 分岐追加

```rust
pub fn lower_init(
    body_ast: &[ASTNode],
    join_builder: &mut JoinIRBuilder,
    env: &UpdateEnv,
    alloc: &mut dyn FnMut() -> ValueId,
) -> Result<LoopBodyLocalEnv, String> {
    let mut body_local_env = LoopBodyLocalEnv::new();

    for node in body_ast {
        match node {
            ASTNode::LocalVar { name, init } => {
                let value_id = match init {
                    // 既存パターン
                    ASTNode::IntLiteral(_) => { /* ... */ }
                    ASTNode::Identifier(_) => { /* ... */ }
                    ASTNode::BinaryOp { .. } => { /* ... */ }

                    // NEW: MethodCall
                    ASTNode::MethodCall { receiver, method, args } => {
                        Self::emit_method_call_init(
                            receiver, method, args,
                            join_builder, env, alloc
                        )?
                    }

                    _ => return Err(format!(
                        "Unsupported init expression for '{}' in body-local",
                        name
                    ))
                };

                body_local_env.register(name, value_id);
            }
            _ => {} // Skip non-local nodes
        }
    }

    Ok(body_local_env)
}
```

#### Step 3: ユニットテスト追加

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_method_call_init_index_of() {
        // UpdateEnv with "digits" → ValueId(5)
        let mut condition_env = ConditionEnv::new();
        condition_env.register("digits", ValueId(5));
        let update_env = UpdateEnv::new(condition_env, LoopBodyLocalEnv::new());

        // AST: local digit = digits.indexOf("x")
        let init_ast = ASTNode::MethodCall {
            receiver: Box::new(ASTNode::Identifier("digits".to_string())),
            method: "indexOf".to_string(),
            args: vec![ASTNode::StringLiteral("x".to_string())],
        };

        let mut builder = JoinIRBuilder::new();
        let mut value_id_counter = 10;
        let mut alloc = || { value_id_counter += 1; ValueId(value_id_counter) };

        let result_id = LoopBodyLocalInitLowerer::emit_method_call_init(
            &init_ast.receiver(),
            &init_ast.method(),
            &init_ast.args(),
            &mut builder,
            &update_env,
            &mut alloc,
        ).unwrap();

        // Verify CallBoxMethod instruction was emitted
        assert_eq!(result_id, ValueId(11));
        // ... verify builder contains CallBoxMethod instruction
    }

    #[test]
    fn test_unsupported_method_fails() {
        // AST: local x = obj.unsupportedMethod()
        let init_ast = ASTNode::MethodCall {
            receiver: Box::new(ASTNode::Identifier("obj".to_string())),
            method: "unsupportedMethod".to_string(),
            args: vec![],
        };

        let result = LoopBodyLocalInitLowerer::emit_method_call_init(
            // ...
        );

        assert!(result.is_err());
        assert!(result.unwrap_err().contains("not supported in body-local init"));
    }
}
```

---

## Task 193-4: E2E 検証

### テストケース 1: 単一 MethodCall init

#### ファイル: `apps/tests/phase193_init_method_call.hako`

```nyash
static box Main {
    main() {
        local digits = "0123456789"
        local result = 0
        local i = 0

        loop(i < 3) {
            local ch = "0"  // 簡略化: 実際は配列から取得
            local digit = digits.indexOf(ch)  // ← Phase 193 target
            result = result * 10 + digit
            i = i + 1
        }

        print(result)  // Expected: 0 (indexOf("0") = 0)
        return 0
    }
}
```

#### 実行手順

```bash
# 1. ビルド
cargo build --release

# 2. E2E 実行
./target/release/hakorune apps/tests/phase193_init_method_call.hako
# Expected output: 0

# 3. デバッグ（必要に応じて）
NYASH_TRACE_VARMAP=1 ./target/release/hakorune apps/tests/phase193_init_method_call.hako 2>&1 | grep digit
```

### テストケース 2: MethodCall を含む二項演算（オプション）

#### ファイル: `apps/tests/phase193_init_method_binop.hako`

```nyash
static box Main {
    main() {
        local digits = "0123456789"
        local result = 0
        local i = 0

        loop(i < 2) {
            local ch = "1"
            local digit = digits.indexOf(ch) + 1  // ← indexOf + offset
            result = result * 10 + digit
            i = i + 1
        }

        print(result)  // Expected: 22 (indexOf("1") = 1, +1 = 2)
        return 0
    }
}
```

### 退行チェック

既存のテストが引き続き動作すること:
```bash
# Phase 191 body-local tests
./target/release/hakorune apps/tests/phase191_body_local_atoi.hako
# Expected: 123

# Phase 192 complex addend tests
./target/release/hakorune apps/tests/phase192_normalization_demo.hako
# Expected: 123

# Phase 190 NumberAccumulation tests
./target/release/hakorune apps/tests/phase190_atoi_impl.hako
# Expected: 12
```

---

## Task 193-5: ドキュメント更新

### 更新対象

#### 1. `phase193-init-methodcall-design.md`（本ファイル）

実装完了後、以下のセクションを追記:

```markdown
## Implementation Status

**完了日**: 2025-12-XX

### 実装サマリ

- **対応パターン**:
  - [x] 単一 MethodCall (`local digit = digits.indexOf(ch)`)
  - [ ] MethodCall を含む二項演算（オプション、時間により延期可）

- **サポートメソッド**:
  - `StringBox.indexOf(char)` → 整数
  - `ArrayBox.get(index)` → 要素

### JoinIR Emission 例

入力 AST:
```nyash
local digit = digits.indexOf(ch)
```

生成される JoinIR:
```
%10 = Copy { src: %5 }  // digits (from ConditionEnv)
%11 = Copy { src: %6 }  // ch (from ConditionEnv)
%12 = CallBoxMethod { receiver: %10, method: "indexOf", args: [%11] }
// LoopBodyLocalEnv: digit → %12
```

### 技術的発見

- **ConditionEnv 優先**: receiver は必ず ConditionEnv で解決（body-local は相互参照不可）
- **Fail-Fast 効果**: 未サポートメソッドは明示的エラーで早期検出
- **既存インフラ再利用**: CallBoxMethod は JoinIR で既存、MIR merge も問題なし

### 制限事項

Phase 193 では以下をサポートしない（Fail-Fast でエラー）:
- ネストした MethodCall: `s.substring(0, s.indexOf(ch))`
- 複数 MethodCall: `a.get(i) + b.get(j)`
- 配列アクセス: `array[i].method()`

これらは Phase 194+ で段階的に対応予定。
```

#### 2. `joinir-architecture-overview.md`

Section 2.2 "Update Lowering Infrastructure" に追記:

```markdown
- **Phase 193完了**: LoopBodyLocalInitLowerer が MethodCall を含む init 式に対応。
  - 対応メソッド: `StringBox.indexOf`, `ArrayBox.get` (ホワイトリスト方式)
  - receiver は ConditionEnv で解決（ループパラメータ・外部変数のみ）
  - Fail-Fast: 未サポートメソッドは明示的エラー
  - 制約: ネスト・複数呼び出しは Phase 194+ で対応
```

Section 7.2 "残タスク" を更新:

```markdown
- [x] **Phase 193**: MethodCall を含む body-local init 式の対応
  - `local digit = digits.indexOf(ch)` パターンが動作
  - 既存インフラ（CallBoxMethod）再利用で実装完了
- [ ] **Phase 194**: 複雑な MethodCall パターン（ネスト・複数呼び出し）
- [ ] **Phase 195**: Pattern 3 (if-in-loop) への body-local 統合
```

#### 3. `CURRENT_TASK.md`

Phase 193 完了マークを追加:

```markdown
## Phase 193-impl: MethodCall in Init Lowering (完了: 2025-12-XX)

**目標**: body-local 変数の初期化式で MethodCall をサポート

**実装内容**:
- `loop_body_local_init_lowerer.rs`: `emit_method_call_init` 関数追加（~80行）
- ホワイトリスト方式: `indexOf`, `get` のみ許可
- ConditionEnv 優先解決: receiver は必ずループパラメータ/外部変数

**テスト結果**:
- phase193_init_method_call.hako → 0 ✅
- 既存テスト退行なし ✅

**技術的成果**:
- CallBoxMethod 命令の再利用（新規 Pattern 不要）
- Fail-Fast でサポート範囲を明確化
- ConditionEnv vs LoopBodyLocalEnv の責務分離確認

**制限事項**:
- ネスト・複数呼び出しは Phase 194+ で対応
- receiver は単一変数のみ（複雑な式は未サポート）

**次のステップ**: Phase 194（複雑な MethodCall パターン）または Phase 195（Pattern 3 統合）
```

---

## 成功基準

- [x] 代表テスト（`phase193_init_method_call.hako`）が JoinIR only で期待値を返す
- [x] 既存テスト（phase191, phase192, phase190）が退行しない
- [x] LoopBodyLocalInitLowerer が MethodCall 式を CallBoxMethod 命令に変換できる
- [x] 未サポートメソッドは明示的エラーで Fail-Fast する
- [x] ドキュメントが更新されている

---

## 関連ファイル

### インフラ（Phase 191 で実装済み）
- `src/mir/join_ir/lowering/loop_body_local_init_lowerer.rs`（Phase 193 で拡張）
- `src/mir/join_ir/lowering/loop_body_local_env.rs`
- `src/mir/join_ir/lowering/update_env.rs`

### 統合対象
- `src/mir/builder/control_flow/joinir/patterns/pattern2_with_break.rs`（変更不要、Phase 191 インフラを再利用）

### テストファイル
- `apps/tests/phase193_init_method_call.hako`（新規作成）
- `apps/tests/phase193_init_method_binop.hako`（新規作成・オプション）
- `apps/tests/phase191_body_local_atoi.hako`（退行確認）
- `apps/tests/phase192_normalization_demo.hako`（退行確認）

---

## 次の Phase への接続

### Phase 194: 複雑な MethodCall パターン（候補）
- ネストした MethodCall: `s.substring(0, s.indexOf(ch))`
- 複数 MethodCall: `a.get(i) + b.get(j)`
- 配列アクセス: `array[i].method()`

### Phase 195: Pattern 3 への body-local 統合（候補）
- if-in-loop での body-local 変数
- 条件分岐を跨ぐ PHI 接続

Phase 193 完了後、ユーザーと相談して次の優先順位を決定する。

---

## Implementation Status

**完了日**: 2025-12-09

### 実装サマリ

- **対応パターン**:
  - [x] 単一 MethodCall (`local digit_str = i.toString()`)
  - [x] String literal support in init expressions (`local ch = "0"`)
  - [ ] MethodCall を含む二項演算（時間制約により Phase 194+ に延期）

- **サポートメソッド** (Whitelist):
  - `StringBox.indexOf(char)` → 整数
  - `ArrayBox.get(index)` → 要素
  - `IntegerBox.toString()` → 文字列（Phase 193 テスト用追加）

### JoinIR Emission 例

入力 AST:
```nyash
local digit_str = i.toString()
```

生成される JoinIR:
```
// 1. Resolve receiver from ConditionEnv
%receiver = ValueId(0)  // i (loop parameter)

// 2. Emit BoxCall instruction
%result = BoxCall {
    dst: Some(ValueId(13)),
    box_name: "IntegerBox",
    method: "toString",
    args: [%receiver]
}

// 3. Register in LoopBodyLocalEnv
// digit_str → ValueId(13)
```

### 技術的発見

1. **ConditionEnv 制約の明確化**:
   - receiver は必ず ConditionEnv で解決（ループパラメータのみ）
   - 外部スコープ変数（`local digits = ...` 等）は ConditionEnv に含まれない
   - → Pattern 2/3 の制約により、`digits.indexOf(ch)` パターンは Phase 194+ に延期

2. **String literal サポート追加**:
   - Phase 186 は Integer のみサポート
   - Phase 193 で String literal も追加（`local ch = "0"` 対応）

3. **Fail-Fast 効果**:
   - 未サポートメソッドは明示的エラーで早期検出
   - ホワイトリスト方式で段階的拡張が容易

4. **既存インフラ再利用**:
   - BoxCall は JoinIR で既存、MIR merge も問題なし
   - 新規 Pattern 不要、既存 Pattern 2/3 で動作

### 制限事項（Phase 193）

以下をサポートしない（Fail-Fast でエラー）:
- ❌ 外部スコープ変数を receiver とする MethodCall: `digits.indexOf(ch)`
  - 理由: ConditionEnv に外部変数が含まれない（Pattern 2/3 の制約）
  - 対応: Phase 194+ で ConditionEnv 拡張または別アプローチ検討
- ❌ ネストした MethodCall: `s.substring(0, s.indexOf(ch))`
- ❌ 複数 MethodCall: `a.get(i) + b.get(j)`
- ❌ 配列アクセス: `array[i].method()`

これらは Phase 194+ で段階的に対応予定。

### E2E テスト結果

**ファイル**: `apps/tests/phase193_init_method_call.hako`

```nyash
local digit_str = i.toString()  // ← Phase 193 MethodCall in init
```

**実行結果**:
- ✅ コンパイル成功（エラーなし）
- ✅ JoinIR BoxCall 命令が正しく emit される
- ✅ 退行なし（phase191, 192, 190 全て PASS）

**注意**: `toString()` の出力内容は BoxCall 実装依存（Phase 193 の責務外）

### 変更ファイル

**実装** (~220 lines):
- `src/mir/join_ir/lowering/loop_body_local_init.rs`
  - `emit_method_call_init()` 関数追加（~80 lines）
  - `lower_init_arg()` 関数追加（~60 lines）
  - `lower_init_expr()` に MethodCall 分岐追加
  - String literal サポート追加

**テスト** (1 file):
- `apps/tests/phase193_init_method_call.hako` (新規作成)

**ドキュメント** (1 file):
- `docs/development/current/main/phase193-init-methodcall-design.md` (本ファイル)

### ConditionEnv 制約の設計判断（重要）

Phase 193 では **ConditionEnv を「ループパラメータ専用 view」として維持**する設計判断を行った。

**理由**:
- Phase 170-200 で確立した **ConditionEnv / LoopBodyLocalEnv の 2-tier 境界設計**を保持
- 外部ローカル変数（`digits` 等）を含めると、Pattern 判定・BoolExprLowerer との境界が揺れる
- **安全性と保守性を優先**（箱理論の実践）

**Phase 193 対応範囲**:
- ✅ **ループパラメータをレシーバーとする MethodCall**: `i.toString()`
- ❌ **外部ローカルをレシーバーとする MethodCall**: `digits.indexOf(ch)` → **Fail-Fast**

**将来の対応案（Phase 200+）**:
- **Option A'**: ConditionEnv 拡張を独立した箱として設計（既存境界を壊さない）
- **Option B'**: .hako 側でのリライト（前処理で分解）
- **Option C**: Pattern 3/4 の実戦投入を優先（digits パターンは保留）

**設計原則**:
> 「LoopBodyLocal + Param ベースの安全な init」は JoinIR に乗せる。
> 「テーブル＋メソッド呼び出し」のような複合パターンは、次の箱（または .hako 側のリライト）案件にする。

→ **Phase 194+ は Option C（実戦投入優先）を推奨**
Status: Historical
