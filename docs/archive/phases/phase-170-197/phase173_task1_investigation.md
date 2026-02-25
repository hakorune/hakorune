# Phase 173 Task 1: 名前解決経路調査結果

## 調査日時
2025-12-04

## 1. 現状の名前解決経路

### 1.1 AST表現の確認

#### using文の処理
- **ファイル**: `lang/src/compiler/pipeline_v2/using_resolver_box.hako`
- **機能**: `using tools.hako_shared.json_parser as JsonParserBox` を解決
- **実装**:
  - `resolve_path_alias()`: エイリアスからファイルパスを解決
  - `resolve_namespace_alias()`: 名前空間エイリアスを tail マッチングで解決
  - JSON文字列ベースの単純検索（正規表現不使用）

#### 静的Boxメソッド呼び出しの現状
- **呼び出し形式**: `JsonParserBox.parse(json_text)`
- **問題点**:
  - `JsonParserBox` は `using ... as JsonParserBox` で import されたエイリアス
  - パーサーは `JsonParserBox.parse()` を「インスタンスメソッド呼び出し」として解釈
  - 実際には「静的Boxの静的メソッド呼び出し」であるべき

### 1.2 MIR Loweringの確認

#### Callee解決システム
- **ファイル**: `src/mir/builder/calls/resolver.rs`
- **実装**: `CalleeResolverBox` によるCallee解決
- **処理フロー**:
  1. `CallTarget::Method { box_type, method, receiver }` として受け取る
  2. `infer_box_type()` で Box 名を推論
  3. `classify_box_kind()` で Box 種別を分類（StaticCompiler/RuntimeData/UserDefined）
  4. `Callee::Method { box_name, method, receiver, box_kind, certainty }` に変換

#### Box種別分類
- **ファイル**: `src/mir/builder/calls/call_unified.rs`
- **関数**: `classify_box_kind()`
- **分類**:
  - `StaticCompiler`: StageBArgsBox, ParserBox, UsingResolverBox 等
  - `RuntimeData`: MapBox, ArrayBox, StringBox 等
  - `UserDefined`: その他すべて

**問題点**:
- `JsonParserBox` は `StaticCompiler` でも `RuntimeData` でもなく `UserDefined` に分類される
- しかし実際には **static box** として扱うべき

### 1.3 エラー発生箇所の特定

#### Phase 171-2 で発見されたエラー
```
[ERROR] ❌ [rust-vm] VM error: Invalid instruction: Unknown method '_skip_whitespace' on InstanceBox
```

**根本原因**:
1. `using tools.hako_shared.json_parser as JsonParserBox` で import
2. `JsonParserBox.parse(json_text)` として呼び出し
3. MIR lowering で「インスタンスメソッド呼び出し」として解釈
4. VM 実行時に `InstanceBox` として扱われる
5. 内部メソッド `_skip_whitespace` が見つからない

**証拠**:
- `tools/hako_shared/tests/json_parser_simple_test.hako` は using を使わず、JsonParserBox 全体をインライン化
- コメント: "Test JsonParserBox without using statement"
- → using statement 経由だと動作しないことが明示的に回避されている

## 2. AST構造の詳細

### 2.1 静的Boxの定義
```hako
static box JsonParserBox {
  method parse(json_str) {
    // ...
  }

  method _skip_whitespace(s, pos) {
    // 内部メソッド
  }
}
```

- `static box` として定義
- すべてのメソッドは静的メソッド（`me.` で自己参照）
- インスタンス化不要（シングルトン的動作）

### 2.2 using statement
```hako
using tools.hako_shared.json_parser as JsonParserBox
```

- `tools.hako_shared.json_parser`: ファイルパス（.hako 省略）
- `JsonParserBox`: エイリアス名
- **期待動作**: エイリアスを **static box の型** として登録

### 2.3 呼び出し形式
```hako
local v = JsonParserBox.parse("{\"x\":1}")
```

- **期待**: `JsonParserBox` は静的Box名として解決
- **現状**: 変数名として解決 → インスタンスメソッド呼び出しとして処理
- **問題**: TypeRef（型参照）と VarRef（変数参照）の区別がない

## 3. 問題の構造化

### 3.1 名前解決の段階
1. **using 解決** (lang/src/compiler/pipeline_v2/using_resolver_box.hako)
   - ✅ ファイルパスの解決は動作
   - ❌ エイリアスの「型としての登録」がない

2. **パーサー** (lang/src/compiler/parser/*)
   - ❌ `Alias.method()` を TypeRef として認識しない
   - ❌ VarRef として処理される

3. **MIR lowering** (src/mir/builder/calls/resolver.rs)
   - ❌ `CallTarget::Method` で receiver が ValueId
   - ❌ 静的Box呼び出しの判別条件がない

### 3.2 必要な修正箇所

#### A. using resolver (.hako 側)
**ファイル**: `lang/src/compiler/pipeline_v2/using_resolver_box.hako`

**必要な機能**:
- 静的Box名を「型」として環境に登録
- エイリアス → 静的Box の対応を保持
- パーサーに引き渡す context に型情報を含める

#### B. パーサー (.hako 側)
**ファイル**: `lang/src/compiler/parser/parser_*.hako`

**必要な機能**:
- `Alias.method()` の AST 表現を追加
- フラグ: `is_static_box_call: true` を付与
- ノード種別: `StaticBoxMethodCall` 的な kind を追加

#### C. MIR lowering (Rust 側)
**ファイル**: `src/mir/builder/calls/resolver.rs`, `call_unified.rs`

**必要な機能**:
- AST の `is_static_box_call` フラグを確認
- 静的Box呼び出しの場合、receiver を無視
- `Callee::Global("BoxName.method/arity")` として解決
- または `Callee::Method { box_name: "JsonParserBox", ... }` で box_kind を StaticCompiler に設定

## 4. 追加の問題発見

### 4.1 JsonParserBox の無限ループバグ
**症状**:
```
[ERROR] ❌ [rust-vm] VM error: vm step budget exceeded (max_steps=1000000, steps=1000001)
at bb=bb338 fn=JsonParserBox._parse_number/2
```

**原因**: `_parse_number()` メソッドに無限ループがある可能性

**影響**:
- 現時点では簡単なJSON (`{\"x\":1}`) でも動作しない
- Phase 171-2 の実装確認が困難

**対応**:
- Phase 173 とは別に JsonParserBox のバグ修正が必要
- または Phase 173 では別の静的Box（より単純なもの）でテスト

### 4.2 パーサーの構文制限
**症状**:
```
❌ Parse error: Unexpected token DOT, expected LPAREN at line 5
local parser = new JsonLib.JsonParserBox()
                          ^^^
```

**原因**: `new Alias.BoxName()` 構文がサポートされていない

**影響**:
- 静的Box内の Box 定義をインスタンス化できない
- 名前空間的な使用ができない

**対応**:
- Phase 173 では `Alias.method()` の直接呼び出しのみに集中
- `new Alias.BoxName()` は Phase 174+ で対応

## 5. 推奨される実装戦略

### 戦略A: 最小限の修正（推奨）
1. **JsonParserBox のバグ修正を先行**
   - `_parse_number()` の無限ループを修正
   - 簡単な JSON で動作確認

2. **using resolver に型登録を追加**
   - `load_modules_json()` で静的Box情報も保持
   - `to_context_json()` に型情報を含める

3. **パーサーに最小限のフラグ追加**
   - `Alias.method()` を検出時に `is_static_box_call: true`
   - AST ノードに追加

4. **MIR lowering で判別処理追加**
   - `is_static_box_call` フラグを確認
   - `Callee::Global("BoxName.method/arity")` に変換

### 戦略B: 包括的な対応（Phase 174+）
1. HIR層の導入
2. 型システムの拡張
3. 明示的スコープ演算子（`::`）のサポート

## 6. 次のステップ

### Task 2: 仕様固定
- [ ] `using.md` に静的Box using のパターンを追記
- [ ] `LANGUAGE_REFERENCE_2025.md` に static box ライブラリ利用方針を追加
- [ ] 許容する呼び方を明確化（`Alias.method()` のみ、`new Alias.Box()` は Phase 174+）

### Task 3: JsonParserBox バグ修正
- [ ] `_parse_number()` の無限ループ原因を特定
- [ ] 修正実装
- [ ] 簡単な JSON (`{"x":1}`) で動作確認

### Task 4: using resolver 修正
- [ ] 静的Box型情報の登録実装
- [ ] context JSON への型情報含め
- [ ] テスト確認

### Task 5: パーサー修正
- [ ] `Alias.method()` 検出実装
- [ ] AST フラグ追加
- [ ] テスト確認

### Task 6: MIR lowering 修正
- [ ] 静的Box呼び出し判別条件追加
- [ ] Callee 解決実装
- [ ] テスト確認

## 7. リスク評価

### 高リスク
- JsonParserBox の無限ループバグ（Task 3 で対応必要）
- パーサー変更による既存コードへの影響
- using resolver の型登録による互換性問題

### 中リスク
- MIR lowering の複雑化
- VM 実行時の予期しないエラー
- テストケース不足

### 低リスク
- ドキュメント更新のみ
- 段階的な実装による影響範囲の限定

## 8. 成果物チェックリスト

- [x] AST 表現の確認（using statement, 静的Box呼び出し）
- [x] MIR lowering の確認（CalleeResolverBox, classify_box_kind）
- [x] エラー発生箇所の特定（Phase 171-2 エラー分析）
- [x] 追加問題の発見（JsonParserBox 無限ループ、パーサー構文制限）
- [x] 推奨戦略の提案（戦略A: 最小限、戦略B: 包括的）
- [ ] JsonParserBox バグ修正（次タスク）
- [ ] 仕様ドキュメント作成（Task 2）

---

**作成日**: 2025-12-04
**調査時間**: 約2時間
**次のタスク**: Task 3（JsonParserBox バグ修正）または Task 2（仕様固定）
Status: Historical
