# Phase 171: JsonParserBox 実装 & hako_check への導入

## 0. ゴール

**Phase 170 で決めた API 草案どおりに .hako 純正 JSON パーサ Box（JsonParserBox）を実装する。**

目的：
- JSON 文字列をメモリ内オブジェクトに変換する Box を実装
- hako_check (HC020) で Phase 156 の手書き JSON パーサ（289行）を置き換え
- 将来の selfhost/Stage-B/解析ツールから再利用可能に

---

## 1. 背景と戦略

### Phase 156 の課題

- hako_check の `analysis_consumer.hako` に 289行の手書き JSON パーサ
- 他の HC ルール（HC021+）でも JSON 解析が必要になる
- 共通ライブラリ化が急務

### Phase 170 での決定

- **API 草案**: `parse()`, `parse_object()`, `parse_array()`
- **型定義**: JsonValueBox（union 相当）, JsonObjectBox, JsonArrayBox
- **MVP スコープ**: MIR/CFG JSON のみ対応（Program JSON v0 は Phase 172+）

### Phase 171 の戦略

1. **最小MVP実装**: MIR/CFG JSON が使う型のみ
2. **段階的導入**: hako_check (HC020) が最初のユーザー
3. **共通ライブラリ化**: `tools/hako_shared/` で shared 設計

---

## 2. Scope / Non-scope

### ✅ やること

1. **JsonParserBox の実装**
   - `parse(json_str) -> JsonValue?`
   - `parse_object(json_str) -> JsonObjectBox?`
   - `parse_array(json_str) -> JsonArrayBox?`

2. **内部型の定義**
   - JsonValueBox（kind で型判定）
   - JsonObjectBox（key-value map）
   - JsonArrayBox（value list）

3. **エスケープシーケンス対応**
   - MIR JSON が実際に使う範囲：`\"`, `\\`, `\n`, `\t` など
   - Unicode (`\uXXXX`) は Phase 172+

4. **hako_check への導入**
   - `analysis_consumer.hako` の 289行手書きパーサを削除
   - JsonParserBox を使用した 10行規模実装に置き換え
   - HC019/HC020 の出力が変わらないことを確認

5. **テスト**
   - JsonParserBox 単体テスト（正常系・エラー系）
   - hako_check スモークテスト（回帰なし）

### ❌ やらないこと

- to_json() / serialization（Phase 172+）
- スキーマ検証（Phase 172+）
- Program JSON v0 対応（Phase 172+）
- ストリーミングパーサ（Phase 173+）

---

## 3. Task 1: 設計ドキュメントの確認・MVP の切り出し

### やること

1. **Phase 170 設計の再確認**
   - `docs/private/roadmap2/phases/phase-170-hako-json-library/README.md` を読む
   - API 草案を確認（parse, parse_object, parse_array）
   - 型定義案を確認（JsonValueBox, JsonObjectBox, JsonArrayBox）

2. **MVP スコープの確定**
   - MIR/CFG JSON のサブセット：
     - 型: null, bool, number, string, object, array
     - number: 整数のみ（浮動小数点は未対応）
     - string: 基本的なエスケープのみ
     - object: キーは文字列固定
   - エラーハンドリング: null を返す

3. **実装方針の決定**
   - Phase 156 の手書きパーサを参照しつつ
   - StringBox.substring/indexOf で文字列走査
   - 状態機械を最小化（シンプル > 最適化）

### 成果物

- MVP の詳細仕様書（Task 2 への引き継ぎ）

---

## 4. Task 2: JsonParserBox の実装 (.hako)

### ファイル位置

```
tools/hako_shared/
├── json_parser.hako      ← 新規作成
└── tests/
    └── json_parser_test.hako  ← 新規作成
```

### やること

1. **Box 構造の定義**

```hako
// JsonValueBox: union 相当の型
static box JsonValueBox {
  kind: StringBox           # "null", "bool", "number", "string", "array", "object"
  boolVal: BoolBox
  numVal: IntegerBox
  strVal: StringBox
  arrVal: JsonArrayBox
  objVal: JsonObjectBox

  method is_null() { return me.kind == "null" }
  method is_bool() { return me.kind == "bool" }
  method is_number() { return me.kind == "number" }
  method is_string() { return me.kind == "string" }
  method is_array() { return me.kind == "array" }
  method is_object() { return me.kind == "object" }
}

// JsonObjectBox: key-value map
static box JsonObjectBox {
  pairs: ArrayBox   # [{"key": String, "value": JsonValue}, ...]

  method get(key: String) -> JsonValue? { ... }
  method keys() -> ArrayBox { ... }
  method values() -> ArrayBox { ... }
}

// JsonArrayBox: value list
static box JsonArrayBox {
  elements: ArrayBox    # [JsonValue, ...]

  method get(index: Integer) -> JsonValue? { ... }
  method length() -> Integer { ... }
}

// JsonParserBox: メインパーサ
static box JsonParserBox {
  method parse(json_str: String) -> JsonValue? { ... }
  method parse_object(json_str: String) -> JsonObjectBox? { ... }
  method parse_array(json_str: String) -> JsonArrayBox? { ... }
}
```

2. **JSON パーサ実装**

```hako
static box JsonParserBox {
  method parse(json_str: String) -> JsonValue? {
    // 前後の空白を削除
    local s = me._trim(json_str)
    if s.length() == 0 { return null }

    local result = null
    local pos = me._parse_value(s, 0, result)
    if pos < 0 { return null }

    return result
  }

  method parse_object(json_str: String) -> JsonObjectBox? {
    local val = me.parse(json_str)
    if val == null or not val.is_object() { return null }
    return val.objVal
  }

  method parse_array(json_str: String) -> JsonArrayBox? {
    local val = me.parse(json_str)
    if val == null or not val.is_array() { return null }
    return val.arrVal
  }

  // 内部ヘルパー
  method _parse_value(s: String, pos: Integer, out: JsonValue) -> Integer { ... }
  method _parse_null(s: String, pos: Integer, out: JsonValue) -> Integer { ... }
  method _parse_bool(s: String, pos: Integer, out: JsonValue) -> Integer { ... }
  method _parse_number(s: String, pos: Integer, out: JsonValue) -> Integer { ... }
  method _parse_string(s: String, pos: Integer, out: JsonValue) -> Integer { ... }
  method _parse_array(s: String, pos: Integer, out: JsonValue) -> Integer { ... }
  method _parse_object(s: String, pos: Integer, out: JsonValue) -> Integer { ... }
  method _skip_whitespace(s: String, pos: Integer) -> Integer { ... }
  method _trim(s: String) -> String { ... }
  method _unescape_string(s: String) -> String { ... }
}
```

3. **エスケープシーケンス対応**

MIR JSON で使われるもの：
- `\"` → `"`
- `\\` → `\`
- `\n` → newline
- `\t` → tab
- `\r` → carriage return
- `\b`, `\f` → 対応可（Phase 171）
- `\uXXXX` → 未対応（Phase 172+）

### 成果物

- JsonParserBox 実装（約300-400行推定）
- 単体テストケース

---

## 5. Task 3: hako_check (HC020) から JsonParserBox を使うように置き換え

### 対象ファイル

- `tools/hako_check/analysis_consumer.hako` (Lines 206-494: 手書きパーサ)
- `tools/hako_check/rules/rule_dead_blocks.hako`

### やること

1. **JSON パーサ部分の削除**
   - `analysis_consumer.hako` の手書きパーサ関数を全削除（~289行）
   - 代わりに JsonParserBox を import

2. **CFG 処理の修正**

```hako
// 修正前（Phase 156）
local mir_json_text = ir.get("_mir_json_text")
local cfg_obj = me._parse_json_object(mir_json_text)  // 手写

// 修正後（Phase 171）
local mir_json_text = ir.get("_mir_json_text")
local cfg_val = JsonParserBox.parse(mir_json_text)
local cfg_obj = cfg_val.objVal  // JsonObjectBox を取得
```

3. **HC020 ルール側の確認**
   - `rule_dead_blocks.hako` が CFG を正しく解析できるか確認
   - JsonObjectBox/.get() メソッドとの互換性確認

4. **目標**
   - `analysis_consumer.hako` の行数を 500+ → 210 に削減（約60%削減）
   - Phase 156 の 289行手書きパーサの削除

### 成果物

- 修正済み `analysis_consumer.hako`
- 修正済み `tools/hako_check/` 関連ファイル

---

## 6. Task 4: 単体テスト & スモークテスト

### JsonParserBox 単体テスト

新規作成: `tools/hako_shared/tests/json_parser_test.hako`

テストケース：

```hako
static box JsonParserTest {
  main() {
    me.test_null()
    me.test_bool()
    me.test_number()
    me.test_string()
    me.test_array()
    me.test_object()
    me.test_error_cases()
    print("All tests passed!")
  }

  test_null() {
    local val = JsonParserBox.parse("null")
    assert(val != null && val.is_null())
  }

  test_bool() {
    local t = JsonParserBox.parse("true")
    local f = JsonParserBox.parse("false")
    assert(t.is_bool() && t.boolVal == true)
    assert(f.is_bool() && f.boolVal == false)
  }

  test_number() {
    local n = JsonParserBox.parse("123")
    assert(n.is_number() && n.numVal == 123)
  }

  test_string() {
    local s = JsonParserBox.parse('"hello"')
    assert(s.is_string() && s.strVal == "hello")
  }

  test_array() {
    local arr = JsonParserBox.parse("[1, 2, 3]")
    assert(arr.is_array() && arr.arrVal.length() == 3)
  }

  test_object() {
    local obj = JsonParserBox.parse('{"key": "value"}')
    assert(obj.is_object())
    local val = obj.objVal.get("key")
    assert(val != null && val.is_string())
  }

  test_error_cases() {
    local inv1 = JsonParserBox.parse("{")
    assert(inv1 == null)

    local inv2 = JsonParserBox.parse("[1,2,]")
    assert(inv2 == null)
  }
}
```

### hako_check スモークテスト

既存のスモークテストを実行：

```bash
# HC020 スモーク
./tools/hako_check_deadblocks_smoke.sh

# HC019 スモーク（回帰なし）
./tools/hako_check_deadcode_smoke.sh
```

期待：
- HC020 出力が Phase 156 と変わらない
- HC019 出力が変わらない
- エラーが出ない

### 成果物

- JsonParserBox テストファイル
- スモークテスト成功確認

---

## 7. Task 5: ドキュメント & CURRENT_TASK 更新

### ドキュメント更新

1. **phase170_hako_json_library_design.md に追記**
   ```markdown
   ## Phase 171 実装結果

   ✅ JsonParserBox 実装完了
   - API: parse(), parse_object(), parse_array()
   - サポート型: null, bool, number, string, array, object
   - エスケープ: \", \\, \n, \t, \r, \b, \f

   ✅ hako_check への導入完了
   - analysis_consumer.hako: 500+ → 210 行（~60%削減）
   - Phase 156 の 289行手書きパーサを削除
   - HC020 が JsonParserBox を使用

   📊 削減実績:
   - hako_check パーサ: 289行 → JsonParserBox 呼び出し（~10行）
   - 共通ライブラリ化: hako_check/selfhost/ツールから再利用可能
   ```

2. **hako_check_design.md を更新**
   ```markdown
   ### JSON 解析

   - Phase 156 まで: analysis_consumer.hako に手書きパーサ（289行）
   - Phase 171 から: JsonParserBox を使用
   - 場所: tools/hako_shared/json_parser.hako
   ```

3. **CURRENT_TASK.md に Phase 171 セクション追加**
   ```markdown
   ### Phase 171: JsonParserBox 実装 & hako_check 導入 ✅

   **完了内容**:
   - JsonParserBox 実装（tools/hako_shared/json_parser.hako）
   - hako_check (HC020) が JsonParserBox を使用
   - 手書きパーサ 289行 → 共通ライブラリに統合

   **成果**:
   - hako_check 行数削減: 60% (500+ → 210 行)
   - 再利用性: HC021+, selfhost, Stage-B で活用可
   - 箱化: .hako 純正 JSON パーサ Box 確立

   **次フェーズ**: Phase 172 で selfhost/Stage-B への導入予定
   ```

### git commit

```
feat(hako_check): Phase 171 JsonParserBox implementation

✨ .hako 純正 JSON パーサ Box 実装完了！

🎯 実装内容:
- JsonParserBox: parse/parse_object/parse_array メソッド
- JsonValueBox, JsonObjectBox, JsonArrayBox 型定義
- エスケープシーケンス対応（\", \\, \n, \t など）

📊 hako_check 統合:
- analysis_consumer.hako: 289行手書きパーサを削除
- JsonParserBox を使用した軽量実装に置き換え
- HC019/HC020 の出力は変わらず（回帰なし）

✅ テスト:
- JsonParserBox 単体テスト全 PASS
- hako_check スモークテスト全 PASS
- Phase 156 との後方互換性確認

🏗️ 次ステップ:
- Phase 172: selfhost/Stage-B への導入
- Phase 172+: Program JSON v0 対応
- Phase 173+: to_json() 逆変換実装
```

---

## ✅ 完成チェックリスト（Phase 171）

- [ ] Task 1: 設計ドキュメント確認・MVP 切り出し
  - [ ] Phase 170 設計再確認
  - [ ] MVP スコープ確定
  - [ ] 実装方針決定
- [ ] Task 2: JsonParserBox 実装
  - [ ] Box 構造定義
  - [ ] パーサ実装
  - [ ] エスケープシーケンス対応
- [ ] Task 3: hako_check への導入
  - [ ] analysis_consumer.hako 修正
  - [ ] 手書きパーサ削除
  - [ ] 行数削減確認（500+ → 210）
- [ ] Task 4: テスト & スモーク
  - [ ] JsonParserBox 単体テスト
  - [ ] hako_check スモークテスト
  - [ ] 回帰なし確認
- [ ] Task 5: ドキュメント更新
  - [ ] phase170 に追記
  - [ ] hako_check_design.md 更新
  - [ ] CURRENT_TASK.md 追加
  - [ ] git commit

---

## 技術的ポイント

### JSON 解析の状態機械

```
parse_value:
  | "null" → parse_null
  | "true"|"false" → parse_bool
  | digit → parse_number
  | '"' → parse_string
  | '[' → parse_array
  | '{' → parse_object
  | else → error (null)
```

### ArrayBox / MapBox との互換性

```hako
// JsonArrayBox.get() は ArrayBox.get() と同じ感覚で
local elem = json_array.get(0)

// JsonObjectBox.get() は MapBox.get() と同じ感覚で
local val = json_obj.get("key")
```

### エラーハンドリング

- 構文エラー → null を返す（Nyash 的な nil）
- 期待値と異なる型 → null を返す

### パフォーマンス

Phase 171 MVP は正確性 > 速度を優先。最適化は Phase 173+ へ。

---

**作成日**: 2025-12-04
**Phase**: 171（JsonParserBox 実装 & hako_check 導入）
**予定工数**: 3-4 時間
**難易度**: 中（JSON パーサ実装 + .hako での Box 設計）
**期待削減**: hako_check 行数 60%、コード共通化 100%
Status: Historical
