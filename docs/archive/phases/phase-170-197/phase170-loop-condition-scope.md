# Phase 170‑D: LoopConditionScopeBox 設計メモ

日付: 2025‑12‑07  
状態: 設計完了（実装は今後の Phase で）

## 背景

Pattern2/Pattern4（Loop with Break / Loop with Continue）は、もともと

- ループパラメータ（例: `i`）
- ループ外のローカル・引数（例: `start`, `end`, `len`）

のみを条件式に使う前提で設計されていた。

しかし JsonParserBox / trim などでは、

```hako
local ch = …
if (ch != " ") { break }
```

のように「ループ本体ローカル」を条件に使うパターンが現れ、この範囲を超えると

- バグが出たり（ValueId 伝播ミス）
- たまたま動いたり

という **曖昧な状態** になっていた。

これを箱理論的に整理するために、条件で使われる変数の「スコープ」を明示的に分類する
LoopConditionScopeBox を導入する。

## LoopConditionScopeBox の責務

責務は 1 つだけ：

> 「条件式に登場する変数が、どのスコープで定義されたか」を教える

### 型イメージ

```rust
pub enum CondVarScope {
    LoopParam,     // ループパラメータ (i)
    OuterLocal,    // ループ外のローカル/引数 (start, end, len)
    LoopBodyLocal, // ループ本体で定義された変数 (ch)
}

pub struct CondVarInfo {
    pub name: String,
    pub scope: CondVarScope,
}

pub struct LoopConditionScope {
    pub vars: Vec<CondVarInfo>,
}
```

主なメソッド例：

```rust
impl LoopConditionScope {
    pub fn has_loop_body_local(&self) -> bool { … }

    pub fn all_in(&self, allowed: &[CondVarScope]) -> bool { … }

    pub fn vars_in(&self, scope: CondVarScope) -> impl Iterator<Item = &CondVarInfo> { … }
}
```

### 入力 / 出力

入力：

- ループヘッダ条件 AST
- break/continue 条件 AST（Pattern2/4）
- LoopScopeShape（どの変数がどのスコープで定義されたか）

出力：

- `LoopConditionScope`（CondVarInfo の集合）

## Pattern2/Pattern4 との関係

Pattern2/4 は LoopConditionScopeBox の結果だけを見て「対応可否」を決める：

```rust
let cond_scope = LoopConditionScopeBox::analyze(&loop_ast, &loop_scope);

// 対応範囲：LoopParam + OuterLocal のみ
if !cond_scope.all_in(&[CondVarScope::LoopParam, CondVarScope::OuterLocal]) {
    return Err(JoinIrError::UnsupportedPattern { … });
}
```

これにより、

- いままで暗黙だった「対応範囲」が **設計として明示**される
- LoopBodyLocal を条件に含む trim/JsonParser 系ループは
  - 現状は `[joinir/freeze] UnsupportedPattern` にする
  - 将来 Pattern5+ で扱いたくなったときに、LoopConditionScopeBox の結果を使って設計できる

## 将来の拡張

LoopBodyLocal を含む条件式を扱いたくなった場合は：

- LoopConditionScopeBox の結果から `vars_in(LoopBodyLocal)` を取り出し、
  - その変数を carrier に昇格させる
  - もしくは LoopHeader に「状態保持用」の追加パラメータを生やす
- それを新しい Pattern5 として設計すれば、既存 Pattern2/4 の仕様を崩さずに拡張できる。

このドキュメントは設計メモのみであり、実装は別フェーズ（Phase 170‑D‑impl など）で行う。
Status: Historical
