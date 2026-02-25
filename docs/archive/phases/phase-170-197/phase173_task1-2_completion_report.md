# Phase 173 Task 1-2 完了報告書

## 実施日時
2025-12-04

## 実施内容
Phase 173「using + 静的 Box メソッド解決の整備」のうち、Task 1（調査）と Task 2（仕様固定）を完了しました。

---

## ✅ Task 1: 名前解決経路調査 - 完了

### 調査対象

#### .hako 側（using resolver）
- **ファイル**: `lang/src/compiler/pipeline_v2/using_resolver_box.hako`
- **機能**: `using tools.hako_shared.json_parser as JsonParserBox` を解決
- **実装**: JSON 文字列ベースの単純検索で alias → file path を解決

#### Rust 側（MIR lowering）
- **ファイル**: `src/mir/builder/calls/resolver.rs`（CalleeResolverBox）
- **ファイル**: `src/mir/builder/calls/call_unified.rs`（classify_box_kind）
- **機能**: CallTarget → Callee の型安全な解決

### 発見した問題

#### 1. 静的 Box が InstanceBox として扱われる
**症状**:
```
[ERROR] VM error: Unknown method '_skip_whitespace' on InstanceBox
```

**根本原因**:
- `using ... as JsonParserBox` で import した静的 Box
- `JsonParserBox.parse()` を「インスタンスメソッド呼び出し」として解釈
- 実際には「静的 Box の静的メソッド呼び出し」であるべき

**名前解決の段階的問題**:
1. **using 解決**: エイリアスを「型」として登録する仕組みがない
2. **パーサー**: `Alias.method()` を TypeRef として認識しない（VarRef として処理）
3. **MIR lowering**: `CallTarget::Method` で receiver が ValueId（静的 Box 判別不可）

#### 2. JsonParserBox の無限ループバグ
**症状**:
```
[ERROR] VM error: vm step budget exceeded (max_steps=1000000, steps=1000001)
at bb=bb338 fn=JsonParserBox._parse_number/2
```

**影響**: Phase 173 の動作確認が困難

#### 3. パーサーの構文制限
**症状**:
```
❌ Parse error: Unexpected token DOT, expected LPAREN
local parser = new JsonLib.JsonParserBox()
                          ^^^
```

**影響**: `new Alias.BoxName()` 構文が未サポート

### 調査成果物
- **ドキュメント**: `phase173_task1_investigation.md`（220行）
- **内容**:
  - AST 表現の詳細分析
  - MIR lowering の処理フロー確認
  - エラー発生箇所の特定
  - 問題の構造化（3段階の名前解決）
  - 推奨実装戦略（戦略A: 最小限、戦略B: 包括的）

---

## ✅ Task 2: 仕様固定（docs）- 完了

### 更新ドキュメント

#### 1. using.md への追加
**ファイル**: `docs/reference/language/using.md`
**追加行数**: 179行
**追加セクション**: 「📦 静的 Box の using（Phase 173+）」

**内容**:
1. **基本概念**
   - 静的 Box をライブラリとして using で import
   - 静的メソッドの直接呼び出し（インスタンス化不要）

2. **許容される呼び出しパターン**
   - ✅ Phase 173: `JsonParserBox.parse()` 直接呼び出し
   - 🚧 Phase 174+: `new Alias.BoxName()` 名前空間的アクセス

3. **静的 Box vs インスタンス Box 比較表**
   - 定義、インスタンス化、メソッド呼び出し、状態保持、用途を比較

4. **実装の技術的詳細**
   - 名前解決の流れ（using 解決 → AST → MIR lowering）
   - Phase 171-2 で発見された問題の説明

5. **使用例**
   - JsonParserBox の利用例
   - ProgramJSONBox の利用例（Phase 172）

6. **制限事項（Phase 173 時点）**
   - パーサー制限
   - 名前空間階層制限
   - 型推論制限

7. **Phase 174+ での拡張予定**
   - 名前空間的 Box アクセス
   - 型システム統合
   - 明示的スコープ（`::`）

#### 2. LANGUAGE_REFERENCE_2025.md への追加
**ファイル**: `docs/reference/language/LANGUAGE_REFERENCE_2025.md`
**追加行数**: 54行
**追加セクション**: 「Static Box のライブラリ利用（Phase 173+）」

**内容**:
1. **基本的な使用例**（JsonParserBox）
2. **特徴**（インスタンス化不要、シングルトン動作、名前空間的利用）
3. **静的 Box vs インスタンス Box**（簡潔版）
4. **制限事項**（Phase 173）
5. **using.md へのクロスリファレンス**

### 仕様の要点

#### Phase 173 で実装する範囲
- ✅ `Alias.method()` 静的メソッド直接呼び出し
- ✅ using statement での静的 Box import
- ✅ 型としての静的 Box 登録

#### Phase 174+ に繰り越す範囲
- 🚧 `new Alias.BoxName()` 名前空間的アクセス
- 🚧 HIR 層の導入
- 🚧 型システムの拡張
- 🚧 明示的スコープ演算子（`::`）

### 仕様固定の効果
1. **開発者への明確なガイド**: 何ができて何ができないかが明確
2. **実装範囲の明確化**: Phase 173 vs Phase 174+ の境界を定義
3. **将来拡張の道筋**: Phase 174+ での拡張方針を提示
4. **箱化モジュール化の原則**: Rust VM 不変、段階的実装の方針明記

---

## 成果物サマリー

### 作成ドキュメント（3件）
1. **phase173_task1_investigation.md**（220行）
   - 調査結果の詳細
   - 問題の構造化
   - 推奨実装戦略

2. **phase173_implementation_summary.md**（comprehensive）
   - 実装サマリー
   - タスク一覧
   - リスク評価
   - チェックリスト

3. **phase173_task1-2_completion_report.md**（本ドキュメント）
   - Task 1-2 の完了報告
   - 成果物の詳細
   - 次のステップ

### 更新ドキュメント（3件）
1. **docs/reference/language/using.md**（+179行）
   - 静的 Box の using セクション追加

2. **docs/reference/language/LANGUAGE_REFERENCE_2025.md**（+54行）
   - Static Box ライブラリ利用セクション追加

3. **CURRENT_TASK.md**
   - Phase 173 進捗セクション追加
   - Task 1-2 完了記録

### テストファイル（1件）
1. **apps/tests/json_parser_min.hako**
   - Phase 173 の統合テスト用

### 合計
- **新規ドキュメント**: 3件
- **更新ドキュメント**: 3件
- **新規テストファイル**: 1件
- **追加行数**: 233行（using.md 179行 + LANGUAGE_REFERENCE_2025.md 54行）
- **調査ドキュメント**: 220行

---

## 技術的洞察

### 箱化モジュール化の実践
1. **Rust VM コア不変**: `.hako` / using 側のみで解決
2. **段階的確認**: AST → MIR → VM の順で確認
3. **既存コード保護**: instance call / plugin call の分岐を壊さない
4. **仕様先行**: まずドキュメントで仕様を固定（Task 2 完了）

### 発見した設計上の問題点
1. **TypeRef vs VarRef の未分離**
   - パーサーレベルで型参照と変数参照の区別がない
   - `Alias.method()` が変数参照として扱われる

2. **using resolver の型情報不足**
   - エイリアスとファイルパスのマッピングのみ
   - Box 種別（static / instance）の情報がない

3. **MIR lowering の判別条件不足**
   - 静的 Box 呼び出しの判別条件がない
   - receiver の有無のみで判断（不十分）

### 推奨実装戦略
**戦略A: 最小限の修正**（推奨）
- JsonParserBox バグ修正（Task 3）
- using resolver に型登録（Task 4）
- パーサーに最小限のフラグ（Task 5）
- MIR lowering で判別処理（Task 6）

**理由**: 段階的な実装で影響範囲を限定、既存コードへの影響最小化

---

## 次のステップ

### immediate: Task 3 または Task 4

#### Option A: Task 3（JsonParserBox バグ修正）先行
**理由**: 動作確認を可能にするための前提条件
**作業**:
1. `tools/hako_shared/json_parser.hako` の `_parse_number()` 確認
2. 無限ループの原因特定
3. 修正実装
4. 簡単な JSON (`{"x":1}`) で動作確認

#### Option B: Task 4（using resolver 修正）先行
**理由**: 実装の核心部分、他タスクの基盤
**作業**:
1. `using_resolver_box.hako` に静的 Box 型情報登録
2. `load_modules_json()` で Box 種別も保持
3. `to_context_json()` に型情報を含める
4. パーサーに型情報を引き渡す

### 推奨: Option A → Option B の順で実装
- Task 3 で JsonParserBox を修正して動作確認可能に
- Task 4-6 で using system の根本修正
- Task 7 で統合テスト
- Task 8 でドキュメント更新＆ git commit

---

## リスク評価

### 高リスク（Task 3 で対応）
- JsonParserBox 無限ループバグ: 動作確認を阻害

### 中リスク（慎重な実装で対応）
- パーサー変更による既存コードへの影響
- using resolver の型登録による互換性問題
- MIR lowering の複雑化

### 低リスク（完了済み）
- ドキュメント更新のみ（Task 2 完了）
- 段階的実装による影響範囲の限定

---

## チェックリスト

### Task 1-2（完了）
- [x] Task 1: 名前解決経路調査
  - [x] AST 表現確認
  - [x] MIR lowering 確認
  - [x] エラー発生箇所特定
  - [x] 追加問題発見
  - [x] 調査ドキュメント作成（220行）
- [x] Task 2: 仕様固定
  - [x] using.md 更新（179行追加）
  - [x] LANGUAGE_REFERENCE_2025.md 更新（54行追加）
  - [x] 実装サマリードキュメント作成
  - [x] CURRENT_TASK.md 更新

### Task 3-8（残り）
- [ ] Task 3: JsonParserBox バグ修正
- [ ] Task 4: using resolver 修正
- [ ] Task 5: パーサー修正
- [ ] Task 6: MIR lowering 修正
- [ ] Task 7: 統合テスト
- [ ] Task 8: ドキュメント更新＆ git commit

---

## 関連 Phase

### Phase 171-2（一時ブロック中）
- JsonParserBox 統合（using 制限で Runtime Error）
- hako_check の 37.6% コード削減達成
- Phase 173 完了後に再開予定

### Phase 172（完了）
- ProgramJSONBox 実装完了
- parse_program() メソッド追加
- JSON 処理 SSOT 基盤確立

### Phase 174+（将来拡張）
- 名前空間的 Box アクセス（`new Alias.BoxName()`）
- HIR 層の導入
- 型システムの拡張
- 明示的スコープ演算子（`::`）

---

## まとめ

### 達成内容
1. ✅ 名前解決経路の完全調査（220行ドキュメント）
2. ✅ 問題の構造化と根本原因の特定
3. ✅ 仕様の明確化（233行追加）
4. ✅ 実装戦略の策定

### 次のマイルストーン
- Task 3: JsonParserBox バグ修正（immediate）
- Task 4-6: using system の根本修正（core）
- Task 7-8: 統合テスト＆完了処理（verification）

### Phase 173 の意義
- **Phase 171-2 のブロック解除**: using system 修正で JsonParserBox が正式なライブラリとして使用可能に
- **selfhost 基盤強化**: 静的 Box ライブラリパターンの確立
- **将来拡張への道筋**: Phase 174+ での名前空間・型システム統合への基盤

---

**作成日**: 2025-12-04
**Phase**: 173（using + 静的 Box メソッド解決）
**進捗**: Task 1-2 完了（25%）
**次タスク**: Task 3（JsonParserBox バグ修正）または Task 4（using resolver 修正）
Status: Historical
