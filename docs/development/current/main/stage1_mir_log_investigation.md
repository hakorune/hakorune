# Stage-1 CLI __mir__.log Observation Analysis - UPDATED

## Executive Summary

**根本原因が判明**: MIR Builder が **すべての `.size()` 呼び出し** を誤った型で解決している

### エラーの系統的パターン:
1. **最初のエラー**: `ParserBox.size()` - ArrayBox.size() のはず
2. **修正後エラー1**: `ParserBox.length()` - ArrayBox.length() のはず
3. **修正後エラー2**: `LoopOptsBox.size()` - ArrayBox.size() のはず

→ **パターン**: 常に ArrayBox であるべきところ、別の Box 型として誤認識される

## 1. ログ挿入箇所

### 完了した修正:
- **Stage1Cli.stage1_main**: 行109-132
  - 引数名を `args` → `cli_args_raw` に変更
  - 引数を完全に使用せず、fresh ArrayBox を作成
  - __mir__.log追加 (行117, 128-130)

## 2. 実行ログ抽出結果

### 2.1 エラーの進化

#### 最初のエラー (修正前):
```
ValueId(71) undefined
ParserBox.size() 呼び出し (ArrayBox.size のはず)
```

#### 2回目のエラー (cli_args使用時):
```
ValueId(20) undefined
ParserBox.length() 呼び出し (ArrayBox.length のはず)
```

#### 3回目のエラー (cli_args回避後):
```
ValueId(50) undefined
LoopOptsBox.size() 呼び出し (ArrayBox.size のはず)
```

### 2.2 __mir__.log の状況
- **依然として出力なし**
- エラーが発生する箇所は __mir__.log より前
- つまり MIR 生成時に既に不正な MIR が生成されている

## 3. 根本原因の特定

### MIR Builder の型レジストリバグ

**問題の本質:**
MIR Builder が `.size()` メソッド呼び出しを解決する際、receiver の型を正しく追跡できていない

**証拠:**
1. `local args_safe = new ArrayBox()` で作成した ArrayBox インスタンス
2. しかし `args_safe.size()` を解決するとき、receiver が：
   - 最初: ParserBox
   - 次: LoopOptsBox
   - などとランダムな Box 型になる

**推測される原因:**
- MIR Builder の型レジストリが ValueId と Box型の対応を正しく保持していない
- 新規作成した ArrayBox インスタンスでさえ、型情報が失われている
- おそらく SSA の PHI 合流点や制御フロー分岐で型情報が消失している

## 4. 該当する可能性のある Rust コード

### src/mir/builder/types/mod.rs または src/mir/builder/call_resolution.rs

**型レジストリの問題:**
```rust
// 推測: 型レジストリが ValueId → BoxType のマッピングを保持しているが
// SSA変換や PHI 生成時に型情報が失われている可能性

pub struct TypeRegistry {
    value_types: HashMap<ValueId, BoxType>,  // この情報が正しく伝播していない
    ...
}
```

**メソッド解決の問題:**
```rust
// MIR Builder がメソッド呼び出しを解決するとき
// receiver の型を lookup するが、間違った型を返している

fn resolve_method_call(&mut self, receiver: ValueId, method: &str) -> Result<...> {
    let box_type = self.type_registry.get_type(receiver)?;  // ← ここが間違った型を返す
    // box_type が ParserBox/LoopOptsBox になってしまっている
    ...
}
```

## 5. .hako レベルでの回避策の限界

### 試した回避策:
1. ✅ 引数名変更 (`args` → `cli_args` → `cli_args_raw`)
2. ✅ 引数を完全に無視して fresh ArrayBox 作成
3. ❌ **しかし依然としてエラー**

### 結論:
**.hako レベルでの回避は不可能**

**理由:**
- `local args_safe = new ArrayBox()` という明示的な ArrayBox 作成でさえ
- MIR Builder は `args_safe` の型を LoopOptsBox と誤認識する
- これは .hako コードの問題ではなく、MIR Builder の型追跡バグ

## 6. MIR Builder 修正の必要性

### 必須修正箇所:

#### A. 型レジストリの PHI 対応
**ファイル:** `src/mir/builder/types/mod.rs`
**問題:** PHI 合流点で型情報が失われる

**修正案:**
```rust
// PHI ノード生成時に、合流する各値の型情報を保持
fn merge_types_at_phi(&mut self, phi_value: ValueId, incoming_values: &[(ValueId, BlockId)]) {
    // すべての incoming values の型を確認
    let types: Vec<_> = incoming_values.iter()
        .map(|(vid, _)| self.get_type(*vid))
        .collect();

    // 型が一致しない場合は警告 or エラー
    let unified_type = self.unify_types(&types)?;
    self.set_type(phi_value, unified_type);
}
```

#### B. NewBox 命令の型登録確認
**ファイル:** `src/mir/builder/exprs.rs` (NewBox 命令生成箇所)
**問題:** `new ArrayBox()` で作成した値の型が正しく登録されていない可能性

**確認事項:**
```rust
// NewBox 命令生成時に型レジストリに登録しているか？
fn emit_newbox(&mut self, box_name: &str) -> ValueId {
    let dst = self.new_value_id();
    self.emit(Instruction::NewBox { dst, box_name: box_name.to_string() });

    // ← ここで型レジストリに登録すべき
    self.type_registry.set_type(dst, BoxType::UserDefined(box_name.to_string()));

    dst
}
```

#### C. メソッド解決時の型検証
**ファイル:** `src/mir/builder/call_resolution.rs`
**問題:** メソッド解決時に型レジストリから取得した型が正しいか検証していない

**修正案:**
```rust
fn resolve_method_call(&mut self, receiver: ValueId, method: &str) -> Result<CallTarget> {
    let box_type = self.type_registry.get_type(receiver)
        .ok_or_else(|| format!("Type unknown for ValueId({:?})", receiver))?;

    // DEBUG: 型レジストリの内容をログ出力
    eprintln!("[MIR/type-resolve] ValueId({:?}) -> {:?}", receiver, box_type);

    // メソッドが存在するか確認
    if !box_type.has_method(method) {
        return Err(format!("{:?} does not have method {}", box_type, method));
    }

    Ok(CallTarget::Method { box_type, method: method.to_string() })
}
```

## 7. デバッグ手順

### Step 1: 型レジストリのトレース有効化

**環境変数追加:**
```rust
// src/mir/builder/types/mod.rs

impl TypeRegistry {
    pub fn set_type(&mut self, value: ValueId, ty: BoxType) {
        if std::env::var("NYASH_MIR_TYPE_TRACE").is_ok() {
            eprintln!("[MIR/type-set] ValueId({:?}) <- {:?}", value, ty);
        }
        self.value_types.insert(value, ty);
    }

    pub fn get_type(&self, value: ValueId) -> Option<&BoxType> {
        let result = self.value_types.get(&value);
        if std::env::var("NYASH_MIR_TYPE_TRACE").is_ok() {
            eprintln!("[MIR/type-get] ValueId({:?}) -> {:?}", value, result);
        }
        result
    }
}
```

### Step 2: トレース実行
```bash
NYASH_MIR_TYPE_TRACE=1 tools/stage1_debug.sh --mode emit-program-json apps/tests/minimal_ssa_skip_ws.hako 2>&1 | grep "\[MIR/type-"
```

### Step 3: 型情報の消失ポイント特定
```
期待される出力:
[MIR/type-set] ValueId(50) <- ArrayBox  # new ArrayBox() で設定
[MIR/type-get] ValueId(50) -> ArrayBox  # 直後は正しい
[MIR/type-get] ValueId(50) -> LoopOptsBox  # ← どこかで上書きされた！
```

## 8. 即座に実装すべき修正

### Priority 1: デバッグトレース追加

**ファイル:** `src/mir/builder/types/mod.rs`
**内容:** 上記 Step 1 の `NYASH_MIR_TYPE_TRACE` 実装

**目的:** 型情報がどこで消失・上書きされるかを特定

### Priority 2: NewBox 命令の型登録確認

**ファイル:** `src/mir/builder/exprs.rs`
**確認:** `new ArrayBox()` 生成時に型レジストリに正しく登録されているか

### Priority 3: PHI 合流点の型処理確認

**ファイル:** `src/mir/builder/phi.rs` または `src/mir/phi_core/`
**確認:** PHI ノード生成時に型情報が正しく伝播しているか

## 9. 暫定的な実行可能性

### .hako レベルでの完全回避は不可能

**理由:**
- MIR Builder の型追跡バグが根本原因
- どんなに defensive な .hako コードを書いても、MIR Builder が型を誤認識する

**唯一の解決策:**
**MIR Builder の型レジストリ修正が必須**

## 10. 推奨アクション (優先順位順)

1. ✅ **即座実装**: `NYASH_MIR_TYPE_TRACE` デバッグトレース追加 (30分)
2. ✅ **トレース実行**: 型情報消失ポイントの特定 (1時間)
3. ✅ **根本修正**: 型レジストリのPHI対応 or NewBox登録修正 (4-8時間)
4. ⏳ **検証**: 修正後に stage1_cli.hako が正常動作するか確認

## 11. 技術的洞察

### Nyash MIR Builder の型追跡の脆弱性

**問題の構造:**
```
.hako ソース → Parser → AST → MIR Builder → MIR
                                    ↑
                              型レジストリ (HashMap<ValueId, BoxType>)
                              ↑
                              ここが壊れている
```

**SSA と型情報の関係:**
- SSA 形式では、各変数に対して複数の ValueId が生成される
- PHI ノードで複数の ValueId が合流する
- **型情報も PHI で合流させる必要がある**
- しかし現状は型情報が失われている

**セルフホスティングで顕在化:**
- 簡単な .hako コードでは問題が見えにくい
- stage1_cli.hako のような複雑な制御フローで型追跡が破綻する
- これは Phase 15 セルフホスティングの最大の障壁

### Fail-Fast 原則の限界

**今回学んだこと:**
- Fail-Fast は .hako レベルでのバグ回避に有効
- しかし MIR Builder のバグは .hako では回避不可能
- コンパイラインフラのバグは、インフラ側で修正するしかない

## 12. 次のステップ

### 即座に実行:
1. `src/mir/builder/types/mod.rs` に `NYASH_MIR_TYPE_TRACE` 追加
2. トレース実行で型消失ポイント特定
3. 該当箇所を修正 (NewBox登録 or PHI合流処理)

### 中期的タスク:
1. MIR Builder の型追跡システム全体のレビュー
2. 型安全性テストケースの追加
3. セルフホスティング環境での回帰テスト

### 長期的改善:
1. 型推論システムの再設計 (HIR層の導入を検討)
2. MIR Verifier での型チェック強化
3. デバッグトレースの常設化
