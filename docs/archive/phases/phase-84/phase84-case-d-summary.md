# Phase 84: Case D 分析サマリー（エグゼクティブサマリー）

## TL;DR

**現状**: Case D 失敗 0 件（Phase 83〜84-4 実装後、dev ガード付きテストで panic なし）

**主要原因（解消済み）**:
- Const 命令の型アノテーション欠如（Phase 84-1 で修正）
- Copy チェーンでの型伝播不足（Phase 84-2 で修正）
- PHI + Copy グラフ上の型集約不足（Phase 84-3 で PhiTypeResolver 導入）
- BoxCall/Await/QMark の戻り値型未登録（Phase 84-4-B で修正）

**対応状況**:
- Phase 83: MethodReturnHintBox（P3-D）実装で 20 件 → 15 件
- Phase 84-1: Const 命令型アノテーション追加で 15 件 → 12 件
- Phase 84-2: CopyTypePropagator 導入で 12 件 → 9 件
- Phase 84-3: PhiTypeResolver 導入で 9 件 → 4 件
- Phase 84-4-B: BoxCall 戻り値型登録で 4 件 → 0 件

**次タスク候補**: if_phi フォールバックの完全削除（Phase 84-5 / Phase 82 最終仕上げ）

---

## 問題の核心

### （初期段階）Const 命令の型アノテーション欠如

```rust
// Integer/Bool/Float/Null/Void は型を登録しない
pub fn emit_integer(b: &mut MirBuilder, val: i64) -> ValueId {
    let dst = b.next_value_id();
    b.emit_instruction(MirInstruction::Const {
        dst,
        value: ConstValue::Integer(val),
    });
    dst  // ← value_types に何も登録していない！
}
```

### ✅ 修正版

```rust
pub fn emit_integer(b: &mut MirBuilder, val: i64) -> ValueId {
    let dst = b.next_value_id();
    b.emit_instruction(MirInstruction::Const {
        dst,
        value: ConstValue::Integer(val),
    });
    b.value_types.insert(dst, MirType::Integer);  // ← この1行を追加
    dst
}
```

**同様の修正を `emit_bool`/`emit_float`/`emit_null`/`emit_void` にも適用**

---

## 影響範囲

### 修正されるテスト（推定 14-16 件）

- `mir_locals_uninitialized` - `return 0` の型
- `mir_stageb_like_*_verifies` (7件) - 全て return 系
- `mir_stage1_cli_entry_like_pattern_verifies` - return 系
- 他の return リテラルを含むテスト

### 残存する問題（Phase 84 終了時点）

- Case D panic は dev ガード付きテストでも 0 件。  
- 残っている課題は「if_phi フォールバックそのものの削除」と、その前提となる `infer_type_from_phi*` callsite の整理のみ。

---

## 実装計画

### Phase 84-1: Const命令型アノテーション（完了）

**Status**: ✅ 実装完了（40dfbc68）

**ファイル**: `src/mir/builder/emission/constant.rs`

**変更箇所**: 5 関数 × 1 行 = **5 行追加**

**効果**: Case D が 15 件 → 12 件（Const 欠如グループは解消）

**所要時間**: 1-2 時間（テスト含む）

**リスク**: 極めて低い（String は既に実装済み）

### Phase 84-2: Copy命令型伝播（完了）

**Status**: ✅ 実装完了（CopyTypePropagator 導入）

**ファイル**:
- `src/mir/phi_core/copy_type_propagator.rs`（新規箱）
- `src/mir/phi_core/mod.rs`
- `src/mir/builder/lifecycle.rs`

**内容**:
- `CopyTypePropagator` が MIR 関数内の `Copy { dst, src }` を固定点ループで走査し、
  `value_types[src]` の型を `value_types[dst]` に伝播（Unknown のみ上書き）。
- `finalize_module` 内で return 型推論の前に実行。

**効果**:
- ベースラインテスト: 489 passed, 34 failed → 494 passed, 33 failed（+5/-1）。
- Case D: 12 件 → 9 件（約 25% 削減）。

**箱理論チェック**:
- 単一責務（Copy の型伝播のみ）、副作用は `value_types` 更新に限定、PHI/JoinIR には非依存。

### Phase 84-3: PHI型推論強化（長期）

**Status**: ✅ PhiTypeResolver 導入完了（PHI + Copy グラフの安全な型推論）

**内容**:
- `PhiTypeResolver` が `Copy`/`Phi` グラフを DFS/BFS で辿り、末端の base 定義型が 1 種類に揃う場合にのみ MirType を返す。
- lifecycle.rs の return 型推論フローに統合し、P3-D/Const/CopyTypePropagator で埋まらないケースの一部を吸収。

**効果**:
- Case D: 9 件 → 4 件（約 56% 削減）。
- 残り 4 件は BoxCall/Await/QMark 戻り値型が `value_types` に登録されていないため、PhiTypeResolver から見ても「base 型が不明」のケースとして扱われている。

### Phase 84-4: BoxCall/Await/QMark 戻り値型登録（完了）

**Status**: ✅ 実装完了（Phase 84-4-B）

**ファイル**:
- `src/mir/builder/utils.rs`（新規）  
  - `infer_boxcall_return_type()` ヘルパー関数を追加（約 75 行）
  - 27 個のビルトイン Box メソッドに対する戻り値型マッピングを集約
- BoxCall lowering 呼び出し元（`emit_box_or_plugin_call` 相当）で、戻り値型を `value_types` に登録

**対応メソッド**（抜粋）:
- StringBox / IntegerBox / BoolBox / ArrayBox / MapBox / Result-like（QMark 相当）/ Stage1CliBox など、計 27 メソッド。

**効果**:
- `NYASH_PHI_FALLBACK_DISABLED=1 cargo test --release --lib` 実行時の Case D panic が 4 件 → 0 件に。
- Await/QMark 系テストは BoxCall 経路の型登録で全て解消され、追加の Await 専用実装は不要となった。

---

## 推奨アクション

1. **Phase 84-1 は完了済み**
   - Const 命令の型アノテーション欠如グループは解消済み。

2. **Phase 84-2 も完了済み**
   - Copy チェーンだけで説明できる Case D は削減済みで、残りは PHI 主体の複雑ケースに集中。

3. **Phase 84-3（PhiTypeResolver）は導入済み**
   - PHI + Copy グラフ上で安全に決められるケースは吸収済みで、残り 4 件は「base 定義側に型がない」という別レイヤの問題に集約された。

---

## 期待される最終結果

| Phase        | Case D 件数 | 修正率 | 備考 |
|--------------|------------|--------|-----|
| Phase 82 終了時 | 20 件       | -      | lifecycle 修正後 |
| Phase 83 後     | 15 件       | 25%   | MethodReturnHintBox（P3-D） |
| Phase 84-1 後   | 12 件       | 40%   | Const 型アノテーション |
| Phase 84-2 後   | 9 件        | 55%   | CopyTypePropagator |
| Phase 84-3 後   | 4 件        | 80%   | PhiTypeResolver（PHI + Copy グラフ） |
| Phase 84-4 後   | 0 件        | 100%  | BoxCall/Await/QMark 型登録 |

**最終目標**: Case D を 0 件にし、`infer_type_from_phi*` を本線から外せる状態を達成済み。次ステップで if_phi フォールバック（約 300 行）を構造的に削除する。

---

## Phase 84-4 方針（案）

- Phase 84-4-A: dev フォールバック
  - 開発時のみ PHI fallback を許可するガードを追加し、自分用のデバッグラインを確保。
- Phase 84-4-B: BoxCall 戻り値型の登録
  - BoxCall lowering で戻り値型を `value_types` に登録し、Stage1 CLI 系 2 テストを根本解決。
- Phase 84-4-C: Await/QMark 戻り値型の処理
  - await/QMark lowering で中間値の型を登録し、await/QMark テスト 2 件を解消。

---

## 詳細分析

完全な分析レポートは以下を参照:
- [phase84-case-d-detailed-analysis.md](./phase84-case-d-detailed-analysis.md)
Status: Historical
