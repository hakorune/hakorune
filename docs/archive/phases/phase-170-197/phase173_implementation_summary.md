# Phase 173: using + 静的 Box メソッド解決 - 実装サマリー

## 実装日時
2025-12-04

## 実装状況
**Phase 173 Task 1-2 完了**: 調査完了・仕様固定完了

## 完了タスク

### ✅ Task 1: 名前解決経路調査
**成果物**: `phase173_task1_investigation.md`

**調査結果サマリー**:
1. **現状の名前解決システム確認**
   - `.hako` 側: `UsingResolverBox` (pipeline_v2/using_resolver_box.hako)
   - Rust 側: `CalleeResolverBox` (src/mir/builder/calls/resolver.rs)
   - Box 種別分類: `classify_box_kind()` (src/mir/builder/calls/call_unified.rs)

2. **問題の特定**
   - using で import した静的 Box が「インスタンス」として扱われる
   - `JsonParserBox.parse()` → `InstanceBox.parse()` として解釈
   - 内部メソッド `_skip_whitespace` が解決できずエラー

3. **AST 構造の確認**
   - `using tools.hako_shared.json_parser as JsonParserBox`
   - 静的 Box: `static box JsonParserBox { method parse(...) }`
   - 呼び出し: `JsonParserBox.parse("{...}")`
   - 問題: TypeRef（型参照）と VarRef（変数参照）の区別がない

4. **追加の問題発見**
   - JsonParserBox の `_parse_number()` に無限ループバグ
   - `new Alias.BoxName()` 構文が未サポート
   - VM step budget exceeded エラー

**技術的洞察**:
- 名前解決の段階: using 解決 → パーサー → MIR lowering
- 各段階で静的 Box を「型」として扱う仕組みが不足
- Rust VM 自体は変更不要（MIR が正しければ動作する前提）

### ✅ Task 2: 仕様固定（docs）
**成果物**:
- `docs/reference/language/using.md` 更新（179行追加）
- `docs/reference/language/LANGUAGE_REFERENCE_2025.md` 更新（54行追加）

**追加された仕様**:

#### using.md の新セクション「📦 静的 Box の using（Phase 173+）」
- **基本概念**: 静的 Box をライブラリとして using で import
- **呼び出しパターン**:
  - ✅ Phase 173: `JsonParserBox.parse()` 静的メソッド直接呼び出し
  - 🚧 Phase 174+: `new Alias.BoxName()` 名前空間的アクセス
- **静的 Box vs インスタンス Box の比較表**
- **実装の技術的詳細**: using 解決 → AST → MIR lowering の流れ
- **使用例**: JsonParserBox / ProgramJSONBox の実例
- **制限事項**: パーサー制限、名前空間階層、型推論
- **Phase 174+ 拡張予定**: HIR 層、型システム、明示的スコープ

#### LANGUAGE_REFERENCE_2025.md の新セクション
- **Static Box のライブラリ利用（Phase 173+）**
- 基本的な使用例（JsonParserBox）
- 特徴・制限事項の明記
- using.md へのクロスリファレンス

**仕様のポイント**:
1. Phase 173 では `Alias.method()` 直接呼び出しのみ
2. `new Alias.BoxName()` は Phase 174+ で対応
3. 静的 Box = シングルトン・ライブラリ的用途
4. インスタンス化不要で直接メソッド呼び出し

## 残タスク

### 🔄 Task 3: JsonParserBox バグ修正（高優先度）
**問題**: `_parse_number()` の無限ループ
**影響**: Phase 173 の動作確認が困難
**対応**:
- [ ] `_parse_number()` の実装確認
- [ ] 無限ループの原因特定
- [ ] 修正実装
- [ ] 簡単な JSON (`{"x":1}`) で動作確認

### 🔄 Task 4: using resolver 修正
**ファイル**: `lang/src/compiler/pipeline_v2/using_resolver_box.hako`
**必要な実装**:
- [ ] 静的 Box 型情報の登録
- [ ] `load_modules_json()` で Box 種別も保持
- [ ] `to_context_json()` に型情報を含める
- [ ] パーサーに型情報を引き渡す

### 🔄 Task 5: パーサー修正
**ファイル**: `lang/src/compiler/parser/parser_*.hako`
**必要な実装**:
- [ ] `Alias.method()` 検出ロジック
- [ ] AST ノードに `is_static_box_call: true` フラグ追加
- [ ] ノード種別追加（必要なら）
- [ ] テスト確認

### 🔄 Task 6: MIR lowering 修正
**ファイル**: `src/mir/builder/calls/resolver.rs`, `call_unified.rs`
**必要な実装**:
- [ ] `is_static_box_call` フラグの確認処理
- [ ] 静的 Box 呼び出し判別条件追加
- [ ] `Callee::Global("BoxName.method/arity")` への変換
- [ ] または `Callee::Method { box_kind: StaticCompiler }` の設定
- [ ] テスト確認

### 🔄 Task 7: 統合テスト
**テストファイル**: `apps/tests/json_parser_min.hako`
**内容**:
```hako
using tools.hako_shared.json_parser as JsonParserBox

static box Main {
    main() {
        local v = JsonParserBox.parse("{\"x\":1}")
        return 0
    }
}
```
**実行確認**:
- [ ] `./target/release/hakorune apps/tests/json_parser_min.hako` が RC 0
- [ ] Unknown method エラーなし
- [ ] hako_check スモーク（HC019/HC020）PASS

### 🔄 Task 8: ドキュメント更新＆ git commit
- [ ] phase171-2 ドキュメント更新（JsonParserBox 正式化）
- [ ] CURRENT_TASK.md 更新（Phase 173 完了記録）
- [ ] git commit

## 推奨実装戦略

### 戦略A: 最小限の修正（推奨）
1. **JsonParserBox バグ修正** → Task 3
2. **using resolver に型登録** → Task 4
3. **パーサーに最小限のフラグ** → Task 5
4. **MIR lowering で判別処理** → Task 6
5. **統合テスト** → Task 7

**理由**:
- 段階的な実装で影響範囲を限定
- 既存コードへの影響最小化
- 各タスクで動作確認可能

### 戦略B: 包括的な対応（Phase 174+）
1. HIR 層の導入
2. 型システムの拡張
3. 明示的スコープ演算子（`::`）

**理由**: より根本的な解決だが、Phase 173 のスコープを超える

## 技術的な注意点

### 箱化モジュール化の原則
1. **Rust VM コア不変**: `.hako` / using 側のみで解決
2. **段階的確認**: AST → MIR → VM の順で確認
3. **既存コード保護**: instance call / plugin call の分岐を壊さない
4. **仕様先行**: まずドキュメントで仕様を固定（Task 2 完了）

### デバッグ環境変数
```bash
# Callee 解決トレース
NYASH_CALLEE_RESOLVE_TRACE=1

# MIR ダンプ
./target/release/hakorune --dump-mir program.hako

# MIR JSON 出力
./target/release/hakorune --emit-mir-json output.json program.hako

# VM 詳細ログ
NYASH_CLI_VERBOSE=1
```

## リスク評価

### 高リスク
- **JsonParserBox 無限ループ**: 動作確認を阻害（Task 3 で対応）
- **パーサー変更影響**: 既存コードの互換性（慎重な実装必要）
- **using resolver 型登録**: 互換性問題の可能性

### 中リスク
- MIR lowering の複雑化
- VM 実行時の予期しないエラー
- テストケース不足

### 低リスク
- ドキュメント更新のみ（Task 2 完了）
- 段階的実装による影響範囲の限定

## 成果物チェックリスト

- [x] Task 1: 名前解決経路調査完了
  - [x] AST 表現確認
  - [x] MIR lowering 確認
  - [x] エラー発生箇所特定
  - [x] 追加問題発見（JsonParserBox 無限ループ、パーサー構文制限）
- [x] Task 2: 仕様固定完了
  - [x] using.md 更新（179行追加）
  - [x] LANGUAGE_REFERENCE_2025.md 更新（54行追加）
- [ ] Task 3: JsonParserBox バグ修正
- [ ] Task 4: using resolver 修正
- [ ] Task 5: パーサー修正
- [ ] Task 6: MIR lowering 修正
- [ ] Task 7: 統合テスト
- [ ] Task 8: ドキュメント更新＆ git commit

## 次のステップ

**immediate**: Task 3（JsonParserBox バグ修正）または Task 4（using resolver 修正）

**理由**:
- Task 3: 動作確認を可能にするための前提条件
- Task 4: 実装の核心部分、他タスクの基盤

**推奨**: Task 3 → Task 4 → Task 5 → Task 6 → Task 7 → Task 8 の順で実装

## 関連ドキュメント

- **指示書**: `phase173_using_static_box_resolution.md`
- **調査結果**: `phase173_task1_investigation.md`
- **仕様**: `docs/reference/language/using.md`, `LANGUAGE_REFERENCE_2025.md`
- **Phase 171-2**: `phase171-2_hako_check_integration.md`
- **Phase 172**: `phase172_implementation_results.md`

---

**作成日**: 2025-12-04
**更新日**: 2025-12-04
**Phase**: 173（using + 静的 Box メソッド解決）
**進捗**: Task 1-2 完了（25%）/ Task 3-8 残り（75%）
Status: Historical
