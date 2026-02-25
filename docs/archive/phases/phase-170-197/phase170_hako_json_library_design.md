# Phase 170: .hako 純正 JSON ライブラリ設計 & インベントリ

## 0. ゴール

**これから実装する .hako JSON ライブラリ（JsonParserBox）の範囲と責務をはっきり決める。**

目的：
- いま散らばっている JSON 解析コード（特に hako_check 内部）を棚卸し
- 「何を共通箱に移すか」を見える化
- Phase 171+ の実装に向けたAPI設計を固定

---

## 1. 背景

### Phase 156 の成果と課題

| 内容 | 状態 | 課題 |
|------|------|------|
| hako_check で MIR JSON を解析 | ✅ 完了 | 320行の手動 JSON パーサ実装 |
| HC020 が unreachable block を検出 | ✅ 完了 | 他ルールでも JSON 解析が必要 |
| Rust 層の変更なし | ✅ | .hako に builtin JSON パーサが欲しい |

### 今後の課題

- Phase 157+ で HC021, HC022... を追加するたび JSON 解析コードが増殖
- Program JSON (v0) を読む selfhost/Stage-B も JSON パーサが欲しい
- 共通ライブラリ化が必須

---

## 2. Scope / Non-scope

### ✅ やること

1. **既存 JSON 利用箇所のインベントリ**
   - どこで JSON を使っているか棚卸し
   - 読み手と書き手を把握

2. **hako_check 内部 JSON パーサの構造把握**
   - Phase 156 の実装を分析
   - 対応範囲と TODO を整理

3. **JsonParserBox の API 草案決定**
   - 最低限必要な型・関数を決定
   - MVP と Phase 172+ の区切りを決定

4. **利用予定箇所のマッピング**
   - hako_check, selfhost, 将来ツール
   - 誰が JsonParserBox を使うか明確化

5. **CURRENT_TASK 更新**
   - Phase 170 の完了記録

### ❌ やらないこと

- JsonParserBox の実装（Phase 171 へ）
- 既存コードのリファクタリング（Phase 172 へ）
- エスケープシーケンスの全対応（段階的に）

---

## 3. Task 1: 既存 JSON 利用箇所のインベントリ

### 調査対象

1. **Program JSON (v0) の読み手**
   - `src/runner/modes/stage_b.rs` - Stage-B 実行
   - `src/runner/modes/` 内の自動実行処理
   - `.hako` コード内の selfhost 解析

2. **MIR JSON の読み手**
   - `tools/hako_check/analysis_consumer.hako` (Phase 156)
   - 将来: 開発ツール / デバッグスクリプト

3. **その他 JSON**
   - JoinIR debug JSON
   - CFG JSON

### やること

1. **JSON 利用箇所を洗い出す**
   ```bash
   rg '"version".*0' --type rust     # Program JSON v0
   rg 'MIR JSON' --type rust          # MIR JSON
   rg 'parse_json\|JsonBox' tools/    # .hako 内
   ```

2. **「誰が何を読むか」を表にまとめる**

   | JSON 形式 | 読み手 | 用途 | 書き手 |
   |----------|-------|------|--------|
   | Program v0 | Stage-B, selfhost | コンパイラ入力 | Rust コンパイラ |
   | MIR JSON | hako_check HC020 | CFG 解析 | `emit_mir_json_for_harness()` |
   | CFG JSON | hako_check HC020+ | ブロック到達可能性 | `extract_cfg_info()` |
   | JoinIR JSON | デバッグツール | 構造確認 | JoinIR dumper |

3. **README に「既存 JSON 利用一覧」として追記**

### 成果物

- JSON 利用箇所の詳細リスト
- 「読み手と書き手」の対応表

---

## 4. Task 2: hako_check 内部 JSON パーサの構造を把握

### 対象ファイル

- `tools/hako_check/analysis_consumer.hako` (Phase 156 実装)

### やること

1. **対応範囲の確認**
   - どの JSON 型に対応しているか
     - ✅ 文字列 / 数値 / bool / null
     - ✅ 配列 / オブジェクト
     - ❓ エスケープ（\", \n 等）
   - サンプル JSON での動作確認

2. **現状と TODO を整理**
   ```
   ✅ できること:
   - オブジェクトの key-value 解析
   - 配列のループ処理
   - 文字列・数値の抽出

   ❓ 改善したいこと:
   - エスケープシーケンス完全対応
   - ネストされた構造の処理
   - エラーハンドリング詳細化
   - パフォーマンス（素朴な実装）

   ❌ 未対応:
   - Unicode エスケープ (\uXXXX)
   - 特殊数値 (Infinity, NaN)
   ```

3. **README に「hako_check 内部 JSON パーサの現状」として整理**
   - 実装行数（約320行）
   - 対応範囲
   - 改善候補

### 成果物

- 対応範囲の詳細ドキュメント
- TODO リスト

---

## 5. Task 3: JsonParserBox の API 草案を決める

### 設計方針

**最小限の責務分離**:
- JsonParserBox: JSON 文字列 → メモリ内オブジェクト
- JsonObjectBox, JsonArrayBox: メモリ内表現
- 書き込み（to_json）は Phase 172 以降

### 草案 API

#### 型定義案

```hako
// 使用側の視点: JsonParserBox.parse() で JSON をパース

// 例1: 単純オブジェクト
local json_str = '{"name": "Main", "entry_block": 0}'
local obj = JsonParserBox.parse(json_str)
// obj は MapBox のように .get("name") 等が使える

// 例2: 配列含む
local json_str = '{"functions": [{"id": 0}, {"id": 1}]}'
local obj = JsonParserBox.parse(json_str)
local funcs = obj.get("functions")  // ArrayBox のように
```

#### 関数シグネチャ案

```
JsonParserBox
  method parse(json_str: String) -> JsonValue?
    // 汎用パーサ（最初はこれだけ）
    // 成功時: JsonValue（実体は MapBox/ArrayBox/String/Number/Bool/Null）
    // 失敗時: null

  method parse_object(json_str: String) -> MapBox?
    // オブジェクト専用（便利メソッド）

  method parse_array(json_str: String) -> ArrayBox?
    // 配列専用（便利メソッド）

JsonValue (Union 型代わり)
  // 実装: MapBox / ArrayBox / String / Integer / Bool / Null を
  //      型タグで区別する仕組み（または each に分岐）
```

### MVP vs Phase 172+

**Phase 171 MVP**:
- ✅ `parse(json_str) -> JsonValue?`
- ✅ オブジェクト / 配列 / 基本型対応
- ✅ Phase 156 の手動パーサを置き換え可能な水準

**Phase 172+**:
- 📋 エスケープシーケンス完全対応
- 📋 スキーマ検証
- 📋 to_json() 逆変換
- 📋 ストリーミングパーサ

### 成果物

- API 草案ドキュメント
- 型定義案
- MVP / Phase 172+ の区切り明記

---

## 6. Task 4: 利用予定箇所のマッピング（誰が JsonParserBox を使うか）

### やること

1. **Task 1 のインベントリから利用候補を抽出**

   | 利用箇所 | 用途 | 置き換え対象 | 優先度 |
   |---------|------|-----------|--------|
   | hako_check HC020 | CFG 解析 | analysis_consumer.hako の JSON パーサ | 高 |
   | hako_check HC021+ | 定数畳み込み等 | 新規実装時に JsonParserBox 利用 | 高 |
   | selfhost Stage-B | Program v0 読み込み | 新規実装 | 中 |
   | 開発ツール | MIR/CFG デバッグ | 新規実装 | 低 |

2. **各箇所の「置き換え内容」をメモ**
   ```
   hako_check HC020:
   - 現状: analysis_consumer.hako に 320行の手動パーサ
   - 修正: JsonParserBox.parse(mir_json_str).get_cfg() 的なワンステップに

   selfhost Stage-B:
   - 現状: Rust で serde_json で JSON パース
   - 将来: Program v0 を .hako で読み込む場合に JsonParserBox 使用
   ```

3. **README に「利用予定マップ」として追記**

### 成果物

- 利用予定マッピング表
- 各用途の「置き換え内容」

---

## 7. Task 5: CURRENT_TASK 更新

### 追加内容

CURRENT_TASK.md に Phase 170 セクションを追加（1-2 段落）:

```markdown
### Phase 170: .hako JSON ライブラリ設計 & インベントリ ✅

**完了内容**:
- 既存 JSON 利用箇所のインベントリ
- hako_check 内部 JSON パーサの構造把握
- JsonParserBox API 草案決定
- 利用予定箇所のマッピング

**設計内容**:
- JsonParserBox: JSON 文字列 → オブジェクト
- Phase 171 MVP: parse() メソッド1つで汎用対応
- Phase 172+: エスケープ完全対応、スキーマ検証、to_json()

**次フェーズ**: Phase 171 で JsonParserBox 実装開始、その後順次 hako_check/selfhost から置き換え開始
```

### 成果物

- CURRENT_TASK.md 更新

---

## ✅ 完成チェックリスト（Phase 170）

- [ ] Task 1: 既存 JSON 利用箇所のインベントリ
  - [ ] JSON 利用箇所洗い出し
  - [ ] 「読み手と書き手」対応表作成
- [ ] Task 2: hako_check 内部 JSON パーサ把握
  - [ ] 対応範囲確認
  - [ ] TODO リスト作成
- [ ] Task 3: JsonParserBox API 草案決定
  - [ ] 関数シグネチャ決定
  - [ ] 型定義案確定
  - [ ] MVP / Phase 172+ 区切り明記
- [ ] Task 4: 利用予定マッピング
  - [ ] 利用候補箇所抽出
  - [ ] 「置き換え内容」メモ
- [ ] Task 5: CURRENT_TASK 更新
  - [ ] Phase 170 セクション追加
  - [ ] git commit

---

## 出力ファイル

`docs/private/roadmap2/phases/phase-170-hako-json-library/README.md`

構成:
1. 既存 JSON 利用一覧（Task 1）
2. hako_check 内部 JSON パーサの現状（Task 2）
3. JsonParserBox API 草案（Task 3）
4. 利用予定マップ（Task 4）

---

**作成日**: 2025-12-04
**Phase**: 170（.hako JSON ライブラリ設計）
**予定工数**: 2-3 時間
**難易度**: 低（調査 + ドキュメント化）
Status: Historical
