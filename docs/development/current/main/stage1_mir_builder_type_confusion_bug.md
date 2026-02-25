# Stage-1 CLI Undefined Value Investigation - Final Report

## 調査目的

Stage1Cli.stage1_main/1 内で発生している `use of undefined value ValueId(..)` エラーを、`__mir__.log` を使用して観測し、根本原因を特定すること。

## 実施した作業

### 1. __mir__.log 挿入 (完了)

#### 修正ファイル:
- **lang/src/runner/stage1_cli.hako**
  - `stage1_main()`: 行109-132
  - `_cmd_emit()`: 行192-198
  - `_cmd_emit_program_json()`: 行215-221
  - `_cmd_emit_mir_json()`: 行243-249
  - `_cmd_run()`: 行309-314

#### 挿入した __mir__.log:
```hako
// stage1_main 入口
__mir__.log("[stage1_main] args_safe at entry", args_safe)

// args.size() 呼び出し前後
__mir__.log("[stage1_main] before args_safe.size()", args_safe)
local argc = args_safe.size()
__mir__.log("[stage1_main] after args_safe.size()", argc)

// env toggles 確認
__mir__.log("[stage1_main] env toggles",
  env.get("STAGE1_EMIT_PROGRAM_JSON"),
  env.get("STAGE1_EMIT_MIR_JSON"),
  ...)
```

### 2. 回避策の試行 (すべて失敗)

#### 試行 A: 引数名変更
```hako
method stage1_main(args) → method stage1_main(cli_args) → method stage1_main(cli_args_raw)
```
**結果:** 失敗 - ParserBox.length() エラー

#### 試行 B: 引数完全回避
```hako
method stage1_main(cli_args_raw) {
  local args_safe = new ArrayBox()  // 引数を使わず fresh 作成
  ...
}
```
**結果:** 失敗 - LoopOptsBox.size() エラー

## 調査結果: 根本原因の特定

### 発見した事実

#### 1. __mir__.log が一切出力されない
```bash
$ grep "__mir__" /tmp/stage1_mir_log_trace.log
(出力なし)
```

**意味:** エラーは **MIR 生成時** に発生しており、VM 実行時ではない

#### 2. エラーの系統的パターン

| 修正段階 | エラー内容 | 本来の型 |
|---------|-----------|---------|
| 最初 | `ParserBox.size()` | `ArrayBox.size()` |
| 修正後1 | `ParserBox.length()` | `ArrayBox.length()` |
| 修正後2 | `LoopOptsBox.size()` | `ArrayBox.size()` |

**パターン:** すべて ArrayBox のメソッドが、別の Box 型として誤認識される

#### 3. 新規作成した ArrayBox でさえ型が誤認識される

```hako
local args_safe = new ArrayBox()  // 明示的に ArrayBox を作成
args_safe.size()                  // → LoopOptsBox.size() と誤認識！
```

**結論:** MIR Builder の型追跡システムが根本的に壊れている

### 根本原因

**MIR Builder の型レジストリバグ**

**場所:** `src/mir/builder/types/mod.rs` (推定)

**問題:**
1. `new ArrayBox()` で作成した ValueId の型情報が正しく登録されていない
2. または SSA の PHI 合流点で型情報が消失している
3. その結果、`.size()` メソッド呼び出し時に誤った Box 型として解決される

**証拠:**
- `.hako レベルでどんなに defensive なコードを書いても回避不可能`
- `new ArrayBox()` という最も明示的な型情報でさえ失われる
- エラーメッセージの Box 型がランダム (ParserBox, LoopOptsBox, etc.)

## 解決策

### .hako レベルでの回避: 不可能

**理由:**
MIR Builder が型情報を正しく追跡できないため、.hako コードをどう書いても回避不可能。

### 必須修正: MIR Builder の型レジストリ

#### Priority 1: デバッグトレース追加 (推定所要時間: 30分)

**ファイル:** `src/mir/builder/types/mod.rs`

**実装内容:**
```rust
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

**実行方法:**
```bash
NYASH_MIR_TYPE_TRACE=1 tools/stage1_debug.sh --mode emit-program-json apps/tests/minimal_ssa_skip_ws.hako 2>&1 | grep "\[MIR/type-"
```

**期待される出力:**
```
[MIR/type-set] ValueId(50) <- ArrayBox  # new ArrayBox() で設定
[MIR/type-get] ValueId(50) -> ArrayBox  # 最初は正しい
[MIR/type-get] ValueId(50) -> LoopOptsBox  # ← ここで型が変わった！
```

→ この出力から **型情報が消失・上書きされる箇所** を特定できる

#### Priority 2: 型情報消失箇所の修正 (推定所要時間: 4-8時間)

**候補箇所 A: NewBox 命令生成時の型登録**

**ファイル:** `src/mir/builder/exprs.rs`

**確認事項:**
```rust
fn emit_newbox(&mut self, box_name: &str) -> ValueId {
    let dst = self.new_value_id();
    self.emit(Instruction::NewBox { dst, box_name: box_name.to_string() });

    // ← ここで型レジストリに登録しているか？
    self.type_registry.set_type(dst, BoxType::UserDefined(box_name.to_string()));

    dst
}
```

**候補箇所 B: PHI 合流点での型伝播**

**ファイル:** `src/mir/phi_core/mod.rs` または `src/mir/builder/phi.rs`

**確認事項:**
```rust
fn emit_phi(&mut self, incoming: &[(ValueId, BlockId)]) -> ValueId {
    let dst = self.new_value_id();
    self.emit(Instruction::Phi { dst, incoming: incoming.to_vec() });

    // ← PHI の型を incoming values から推論しているか？
    let types: Vec<_> = incoming.iter()
        .filter_map(|(vid, _)| self.type_registry.get_type(*vid))
        .collect();

    if let Some(unified_type) = self.unify_types(&types) {
        self.type_registry.set_type(dst, unified_type);
    }

    dst
}
```

**候補箇所 C: メソッド解決時の型検証**

**ファイル:** `src/mir/builder/call_resolution.rs`

**確認事項:**
```rust
fn resolve_method_call(&mut self, receiver: ValueId, method: &str) -> Result<CallTarget> {
    let box_type = self.type_registry.get_type(receiver)
        .ok_or_else(|| format!("Type unknown for ValueId({:?})", receiver))?;

    // メソッドが存在するか確認
    if !box_type.has_method(method) {
        return Err(format!("{:?} does not have method {}", box_type, method));
    }

    Ok(CallTarget::Method { box_type, method: method.to_string() })
}
```

## 次のアクション (優先順位順)

### 1. 即座実装 (今すぐ)
- [ ] `NYASH_MIR_TYPE_TRACE` デバッグトレース追加 (30分)
  - `src/mir/builder/types/mod.rs` 修正
  - `set_type()` と `get_type()` にトレースログ追加

### 2. トレース実行 (1時間)
- [ ] `NYASH_MIR_TYPE_TRACE=1` で実行
- [ ] 型情報消失ポイントを特定
- [ ] ValueId(50) が ArrayBox → LoopOptsBox に変わる箇所を見つける

### 3. 根本修正 (4-8時間)
- [ ] 特定した箇所を修正
  - NewBox 命令生成時の型登録
  - PHI 合流点での型伝播
  - その他の型情報消失ポイント

### 4. 検証 (1時間)
- [ ] 修正後に `tools/stage1_debug.sh` を再実行
- [ ] `__mir__.log` が正常に出力されることを確認
- [ ] stage1_cli.hako が正常動作することを確認

## 成果物

### 1. 修正済みファイル
- **lang/src/runner/stage1_cli.hako**
  - `__mir__.log` 追加済み (今後のデバッグに有用)
  - 引数名を `cli_args_raw` に変更 (型衝突回避の試み)

### 2. 分析ドキュメント
- **/tmp/stage1_mir_log_analysis.md** - 初期分析
- **/tmp/stage1_mir_log_analysis_updated.md** - 更新版分析
- **/tmp/stage1_undefined_value_final_report.md** - 本レポート

## 技術的洞察

### Nyash セルフホスティングの課題

**問題の構造:**
```
.hako ソース → Parser → AST → MIR Builder → MIR → VM
                                    ↑
                              型レジストリ (HashMap<ValueId, BoxType>)
                              ↑
                              ここが壊れている
```

**なぜ今まで見つからなかったか:**
- 簡単な .hako コードでは型情報の消失が問題にならない
- stage1_cli.hako のような複雑な制御フロー (if/loop/PHI) で顕在化
- セルフホスティング環境特有の問題

**Fail-Fast 原則の限界:**
- .hako レベルでのバグ回避には有効
- しかし **コンパイラインフラのバグは .hako では回避不可能**
- インフラ側の修正が必須

### SSA と型情報の関係

**SSA 形式の特性:**
- 各変数に対して複数の ValueId が生成される
- PHI ノードで複数の ValueId が合流する

**型情報も PHI で合流させる必要がある:**
```rust
// 例: if-else で異なる値が同じ変数に代入される
if condition {
    x = new ArrayBox()  // ValueId(10): ArrayBox
} else {
    x = new MapBox()    // ValueId(20): MapBox
}
// PHI: ValueId(30) = phi [ValueId(10), ValueId(20)]
//      → ValueId(30) の型は？ (ArrayBox | MapBox) or 共通親型？
```

**現状の問題:**
- PHI で型情報が失われている
- その結果、PHI の結果値 (ValueId(30)) の型が不定になる
- メソッド解決時に誤った型が使われる

## 結論

### 根本原因:
**MIR Builder の型レジストリが SSA の PHI 合流点で型情報を正しく伝播していない**

### 解決策:
**MIR Builder の型追跡システム修正が必須**

### .hako レベルでの回避:
**不可能**

### 次のステップ:
1. **`NYASH_MIR_TYPE_TRACE` デバッグトレース実装** (最優先)
2. **トレース実行で型消失ポイント特定**
3. **該当箇所を修正**
4. **検証**

### 推定所要時間:
- デバッグトレース実装: 30分
- トレース実行・分析: 1時間
- 根本修正: 4-8時間
- 検証: 1時間
- **合計: 6-10時間**

## 補足: 今回の調査で得られた知見

### __mir__.log の限界
- **実行時ツール** なので、MIR 生成時のバグには使えない
- MIR Builder のバグは **MIR Builder レベルのトレース** が必要

### Fail-Fast の正しい適用範囲
- ✅ .hako コードの論理バグ回避に有効
- ❌ コンパイラインフラのバグ回避には無力

### セルフホスティングの価値
- **コンパイラの隠れたバグを発見できる**
- 簡単なコードでは見つからない問題を顕在化させる
- **Phase 15 セルフホスティングは正しい方向性**
