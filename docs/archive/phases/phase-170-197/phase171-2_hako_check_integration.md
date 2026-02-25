# Phase 171-2: hako_check の JSON パーサ完全置き換え

## 0. ゴール

**analysis_consumer.hako に残っている手書き JSON パーサ（約289行）を JsonParserBox 呼び出しに全面置き換えする。**

目的：
- Phase 171 で作成した JsonParserBox を hako_check に統合
- 手書き JSON パーサの完全削除（289行 → ~10-30行）
- HC019/HC020 の結果が変わらないことを確認
- Phase 171 を「実装＋統合まで完了」扱いにする

---

## 1. 背景

### Phase 171 の現状
- ✅ JsonParserBox 実装完了（`tools/hako_shared/json_parser.hako`, 454行）
- ✅ ProgramJSONBox 実装完了（Phase 172）
- 🔄 hako_check への統合が未完了（289行の手書きパーサが残存）

### この Phase でやること
- `tools/hako_check/analysis_consumer.hako` の手書き JSON パーサを削除
- JsonParserBox を using で読み込み、parse() / parse_object() / parse_array() を使用
- HC019/HC020 の出力が変わらないことを確認

---

## 2. Scope / Non-scope

### ✅ やること

1. **現状の JSON 解析経路を再確認**
   - analysis_consumer.hako の手書きパーサ構造を把握
   - JsonParserBox で同じ構造を読めるか確認

2. **analysis_consumer.hako を JsonParserBox に置き換え**
   - 手書き JSON パーサ本体を全削除
   - JsonParserBox を using で読み込み
   - MIR JSON → JsonParserBox.parse() → CFG 抽出

3. **hako_check ルール側の微調整（必要なら）**
   - HC019/HC020 からの参照先フィールド調整
   - ロジック自体は変更しない

4. **スモーク・回帰テスト**
   - HC019/HC020 のスモークテスト実行
   - 以前と同じ警告が出ているか確認

5. **ドキュメント & CURRENT_TASK 更新**
   - Phase 171 完了記録
   - 96%削減達成の記録

### ❌ やらないこと

- HC019/HC020 のロジック変更
- 新しい解析ルール追加
- JsonParserBox の機能追加

---

## 3. Task 1: 現状の JSON 解析経路を再確認

### 対象ファイル
- `tools/hako_check/analysis_consumer.hako`
- `tools/hako_shared/json_parser.hako`（JsonParserBox & ProgramJSONBox）

### やること

1. **手書き JSON パーサの構造確認**
   - analysis_consumer.hako の中で「文字列を手で舐めて JSON を構築している部分」を特定
   - どの JSON（MIR/CFG/Program）のどのフィールドを読んでいるか簡単にメモ

2. **JsonParserBox との互換性確認**
   - JsonParserBox 側で同じ構造を読めるか（MVPとして足りているか）を確認
   - 不足している機能があれば、Phase 173+ で対応予定としてメモ

### 成果物
- JSON 解析経路のメモ
- JsonParserBox 互換性確認結果

---

## 4. Task 2: analysis_consumer.hako を JsonParserBox に置き換え

### 方針
- 文字列操作ベースの JSON パーサ本体は**全部削除**
- 「MIR JSON の文字列を受け取る」までは今のまま
- そこから先を JsonParserBox に委譲する

### 具体的な実装

1. **JsonParserBox を using で読み込む**
   ```hako
   # analysis_consumer.hako の先頭に追加
   # （現在の using 方式に合わせて調整）
   ```

2. **MIR JSON 解析処理を書き換え**
   ```hako
   # 修正前（手書きパーサー、約289行）
   method parse_mir_json_manually(mir_json_str) {
       # 文字列操作ベースの手動解析...
       # 約289行
   }

   # 修正後（JsonParserBox 使用、~10-30行）
   method parse_mir_json_with_parser(mir_json_str) {
       local parser = new JsonParserBox()
       local root = parser.parse(mir_json_str)

       if root == null {
           # エラーハンドリング
           return me.create_empty_cfg()
       }

       # CFG/functions/blocks の抽出
       local cfg = root.get("cfg")
       if cfg == null {
           return me.create_empty_cfg()
       }

       local functions = cfg.get("functions")
       # ... 以下、既存の CFG 抽出処理に接続
   }
   ```

3. **既存の CFG 抽出処理と接続**
   - JsonParserBox が返す DOM から cfg / functions / blocks を拾う形に書き換え
   - 既存の Analysis IR への統合処理はそのまま使用

### 目標
- JSON 解析部分の行数を **289行 → ~10-30行レベル**（96%削減）

### 成果物
- `analysis_consumer.hako` 修正完了
- 手書き JSON パーサ完全削除
- JsonParserBox 統合完了

---

## 5. Task 3: hako_check ルール側の微調整（必要なら）

### 対象ファイル
- `tools/hako_check/rules/rule_dead_blocks.hako`（HC020）
- `tools/hako_check/rules/rule_dead_code.hako`（HC019）

### やること

1. **analysis_consumer 側の CFG/関数情報の形が変わった場合**
   - HC020/HC019 からの参照先フィールドを合わせる
   - ロジック自体（reachability 判定、未使用メソッド判定）は変えない

2. **動作確認**
   - 簡単なテストケースで HC020/HC019 が動作するか確認

### 成果物
- HC020/HC019 の微調整完了（必要な場合のみ）

---

## 6. Task 4: スモーク・回帰テスト

### 必須スクリプト
- `tools/hako_check_deadcode_smoke.sh`（HC019）
- `tools/hako_check_deadblocks_smoke.sh`（HC020）

### やること

1. **JsonParserBox 統合後に両方を実行**
   ```bash
   ./tools/hako_check_deadcode_smoke.sh
   ./tools/hako_check_deadblocks_smoke.sh
   ```

2. **出力確認**
   - 以前と同じケースで同じ警告が出ているか確認
   - 少なくとも「件数」と代表メッセージを確認

3. **追加の手動確認**
   ```bash
   # 1本 .hako を選んで手動実行
   ./tools/hako_check.sh --dead-code apps/tests/XXX.hako
   ./tools/hako_check.sh --dead-blocks apps/tests/XXX.hako
   ```
   - 眼でざっと確認

### 期待される結果
- HC019/HC020 の出力が JsonParserBox 統合前後で変わらない
- スモークテスト全て PASS

### 成果物
- スモークテスト成功確認
- 回帰なし確認

---

## 7. Task 5: ドキュメント & CURRENT_TASK 更新

### ドキュメント更新

1. **phase170_hako_json_library_design.md / phase171_*:**
   - 「hako_check 側の JSON パーサが完全に JsonParserBox に乗ったこと」を追記
   - 「旧手書きパーサは削除済み（約 96% 削減）」を追記

2. **hako_check_design.md:**
   - 「JSON 解析は JsonParserBox を経由」と明記

3. **CURRENT_TASK.md:**
   - Phase 171 セクションに以下を追加：
     ```markdown
     ### Phase 171: JsonParserBox 実装 ✅
     - JsonParserBox 実装完了（454行）
     - hako_check 統合完了（289行 → ~10-30行、96%削減達成）
     - HC019/HC020 正常動作確認
     - Phase 171 完全完了！
     ```
   - この章をクローズ扱いにする

### git commit

```bash
git add .
git commit -m "feat(hako_check): Phase 171-2 JsonParserBox integration complete

🎉 hako_check の JSON パーサ完全置き換え成功！

🔧 実装内容:
- analysis_consumer.hako: 手書き JSON パーサ削除（289行）
- JsonParserBox 統合（~10-30行）
- 96%削減達成！

✅ テスト結果:
- HC019 スモークテスト: PASS
- HC020 スモークテスト: PASS
- 回帰なし確認

🎯 Phase 171 完全完了！
次は JSON 逆方向（to_json）や selfhost depth-2 / .hako JoinIR/MIR へ

🤖 Generated with Claude Code

Co-Authored-By: Claude <noreply@anthropic.com>"
```

---

## ✅ 完成チェックリスト（Phase 171-2）

- [ ] Task 1: JSON 解析経路再確認
  - [ ] 手書きパーサー構造把握
  - [ ] JsonParserBox 互換性確認
- [ ] Task 2: analysis_consumer.hako 置き換え
  - [ ] 手書き JSON パーサ削除（289行）
  - [ ] JsonParserBox 統合（~10-30行）
  - [ ] CFG 抽出処理接続
- [ ] Task 3: HC019/HC020 微調整（必要なら）
  - [ ] 参照先フィールド調整
  - [ ] 動作確認
- [ ] Task 4: スモーク・回帰テスト
  - [ ] HC019 スモーク PASS
  - [ ] HC020 スモーク PASS
  - [ ] 回帰なし確認
- [ ] Task 5: ドキュメント更新
  - [ ] phase170/171 更新
  - [ ] hako_check_design.md 更新
  - [ ] CURRENT_TASK.md 更新
  - [ ] git commit

---

## 技術的注意点

### using statement の使用
```hako
# 現在の hako_check での using 方式に合わせる
# （プロジェクトの using 実装状況に依存）
```

### エラーハンドリング
```hako
# JsonParserBox.parse() が null を返した場合
# 空の CFG 構造体にフォールバック
method create_empty_cfg() {
    local cfg = new MapBox()
    local functions = new ArrayBox()
    cfg.set("functions", functions)
    return cfg
}
```

### CFG フィールド抽出
```hako
# MIR JSON から CFG を抽出する例
local cfg = root.get("cfg")
local functions = cfg.get("functions")  # ArrayBox

# 各 function から entry_block, blocks を取得
for fn in functions {
    local entry_block = fn.get("entry_block")
    local blocks = fn.get("blocks")  # ArrayBox

    # 各 block から id, successors, reachable を取得
    for block in blocks {
        local id = block.get("id")
        local successors = block.get("successors")  # ArrayBox
        local reachable = block.get("reachable")  # BoolBox
    }
}
```

---

## 次のステップ

Phase 171-2 完了後：
- **Phase 173**: to_json() 逆変換実装
- **Phase 174**: selfhost depth-2 JSON 統一化
- **Phase 160+**: .hako JoinIR/MIR 移植

これで Phase 171 をきっちり締めておけば、次のフェーズに気持ちよく移れる！

---

## 実装結果（2025-12-04）

### ✅ 実装完了内容

#### 1. 手書き JSON パーサの削除
- **削減量**: 708行 → 442行（266行削減、37.6%削減達成）
- **削除した機能**:
  - `_parse_cfg_functions` (約40行)
  - `_parse_single_function` (約30行)
  - `_parse_blocks_array` (約40行)
  - `_parse_single_block` (約25行)
  - `_extract_json_string_value` (約15行)
  - `_extract_json_int_value` (約30行)
  - `_extract_json_bool_value` (約20行)
  - `_extract_json_int_array` (約45行)

#### 2. JsonParserBox 統合実装
- **新しい `_extract_cfg_from_mir_json` 実装**:
  ```hako
  _extract_cfg_from_mir_json(json_text) {
    if json_text == null { return me._empty_cfg() }

    // Use JsonParserBox to parse the MIR JSON
    local root = JsonParserBox.parse(json_text)
    if root == null { return me._empty_cfg() }

    // Extract cfg object: {"cfg": {"functions": [...]}}
    local cfg = root.get("cfg")
    if cfg == null { return me._empty_cfg() }

    // Return the cfg object directly
    return cfg
  }
  ```
- **行数**: 約15行（対して元の289行）
- **削減率**: 95%削減達成！

#### 3. using 文の追加
- `using tools.hako_shared.json_parser as JsonParserBox`
- **配置**: analysis_consumer.hako の先頭（line 14）

### ⚠️ 発見された重大な問題

#### using statement の静的 Box メソッド解決問題

**症状**:
```
[ERROR] ❌ [rust-vm] VM error: Invalid instruction: Unknown method '_skip_whitespace' on InstanceBox
```

**根本原因**:
- `using` statement が静的 Box のメソッドを正しく解決できていない
- `JsonParserBox.parse()` は静的メソッドだが、VM が InstanceBox として扱っている
- 内部メソッド `_skip_whitespace` 等が見つからない

**証拠**:
- `tools/hako_shared/tests/json_parser_simple_test.hako` は using を使わず、JsonParserBox のコード全体をインライン化している
- コメント: "Test JsonParserBox without using statement"

**影響範囲**:
- hako_check での JsonParserBox 使用が現時点で不可能
- using statement の静的 Box 対応が Phase 15.5+ で必要

### 🔄 対応方針（2つの選択肢）

#### Option A: Phase 173 で using system 修正を待つ（推奨）
**メリット**:
- 正しいアーキテクチャ（SSOT: JsonParserBox）
- 将来的な保守性向上
- Phase 171-2 の本来の目標達成

**デメリット**:
- すぐには動作しない
- using system の修正が必要（Phase 15.5+）

**実装済み状態**:
- ✅ コード統合完了（37.6%削減）
- ✅ using 文追加完了
- ⚠️ 実行時エラー（using 制限のため）

#### Option B: 一時的にインライン化する
**メリット**:
- 即座に動作する
- テスト可能

**デメリット**:
- SSOT 原則違反
- 454行のコード重複
- Phase 171 の目標不達成

### 📊 現在の状態

| 項目 | 状態 | 備考 |
|-----|------|------|
| 手書きパーサ削除 | ✅ | 266行削除（37.6%） |
| JsonParserBox 統合 | ✅ | 15行実装（95%削減） |
| using 文追加 | ✅ | line 14 |
| コンパイル | ✅ | エラーなし |
| 実行 | ❌ | using 制限のため VM エラー |
| HC019 テスト | ⚠️ | 未実行（実行時エラーのため） |
| HC020 テスト | ⚠️ | 未実行（実行時エラーのため） |

### 🎯 推奨アクション

1. **現在の実装をコミット** (Option A の準備)
   - 37.6%削減の成果を記録
   - using 制限を明記

2. **Phase 173 で using system 修正**
   - 静的 Box のメソッド解決を修正
   - JsonParserBox 統合を完全動作させる

3. **Phase 171 完了判定**
   - 「実装完了、using 制限により Phase 173 で動作確認」とする
   - アーキテクチャ的には正しい方向性

---

**作成日**: 2025-12-04
**更新日**: 2025-12-04 (実装結果追記)
**Phase**: 171-2（hako_check JSON パーサ完全置き換え）
**予定工数**: 2-3 時間
**実工数**: 2 時間（実装完了、using 制限発見）
**難易度**: 中（統合 + テスト確認）
**状態**: 実装完了（using 制限により Phase 173 で動作確認予定）
Status: Historical
