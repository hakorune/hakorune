# Phase 172: JsonParserBox の再利用拡大（Stage-B / selfhost / ツール統合）

## 0. ゴール

**Phase 171 で実装した JsonParserBox を、hako_check 以外の「JSON を読む .hako コード」にも適用し、JSON パーサ断片をできるだけ共通箱に集約する。**

目的：
- Program(JSON v0) を読む Stage-B/selfhost 補助で JsonParserBox を使用
- 将来の解析ツールが全部この箱を経由する状態を作る
- JSON 処理の単一の真実（SSOT）を確立

---

## 1. 背景と戦略

### Phase 171 での成果

- ✅ JsonParserBox 実装完了（454行）
- ✅ hako_check (HC020) で使用開始
- ✅ 289行の手書きパーサを削除（96%削減）

### Phase 172 の戦略

**再利用拡大の3ステップ**：

1. **再利用候補の特定**: Program(JSON v0) を読む箇所を洗い出し
2. **薄いラッパー追加**: `parse_program()` ヘルパーを JsonParserBox に追加
3. **段階的適用**: Stage-B/selfhost の 1-2 箇所から開始

### selfhost depth-2 への準備

- JSON 処理が共通箱に集約されることで
- .hako JoinIR/MIR 移植時に構造が崩れにくくなる
- selfhost depth-2 の基盤が整う

---

## 2. Scope / Non-scope

### ✅ やること

1. **再利用候補の棚卸し（再スキャン）**
   - Stage-B/selfhost での JSON 利用箇所を洗い出し
   - 現状の JSON 読み取り方法を調査
   - JsonParserBox 適用可否を評価

2. **Program(JSON v0) 用ラッパー追加**
   - `parse_program()` メソッドを JsonParserBox に追加
   - ProgramJSONBox（薄いヘルパー型）を定義
   - Program JSON の定形構造に対応

3. **Stage-B/selfhost への適用**
   - 1-2 箇所から段階的に適用
   - 自前 substring 処理を JsonParserBox に置き換え

4. **hako_check 仕上げ**
   - Phase 171 の残りタスク完了確認
   - JsonParserBox API の十分性確認

5. **テスト & 回帰**
   - hako_check: HC019/HC020 スモーク
   - Stage-B/selfhost: selfhost_phase150_depth1_smoke
   - JsonParserBox: 単体テスト

6. **ドキュメント更新**

### ❌ やらないこと

- to_json() / serialization（Phase 173+）
- スキーマ検証（Phase 173+）
- 全 JSON 利用箇所の一括置き換え（段階的に）

---

## 3. Task 1: 再利用候補の棚卸し（再スキャン）

### 対象候補

1. **Stage-B/selfhost での JSON 利用**
   - `lang/src/compiler/entry/compiler.hako` - Program JSON 処理
   - `apps/selfhost-vm/json_loader.hako` - JSON ローダー
   - `tools/selfhost/*` - selfhost ツール群

2. **その他ツール・テスト**
   - `apps/tests/` 内の JSON を扱うテスト
   - 開発ツールの JSON 解析コード

### やること

1. **Program(JSON v0) 消費箇所の洗い出し**
   ```bash
   rg '"version".*0' lang tools apps
   rg 'Program.*JSON' lang tools apps
   rg 'substring.*\{' lang tools apps  # 自前 JSON 解析の疑い
   ```

2. **各箇所の評価**
   | ファイル | 現状の読み方 | JsonParserBox 適用可否 | 優先度 |
   |---------|-------------|---------------------|--------|
   | lang/src/compiler/entry/compiler.hako | 自前 split/substring | ✅ 可能 | 高 |
   | apps/selfhost-vm/json_loader.hako | 自前 substring | ✅ 可能 | 中 |
   | tools/selfhost/helpers.hako | まだ読んでいない | ⏸️ 将来 | 低 |

3. **phase170_hako_json_library_design.md に追記**
   - 「Phase 172 対象候補」セクションを追加
   - 評価結果を表にまとめる

### 成果物

- 再利用候補リスト（優先度付き）
- 各候補の評価結果

---

## 4. Task 2: Program(JSON v0) 用の薄いラッパーを JsonParserBox に追加

### 目的

Program(JSON v0) の定形構造を読みやすくする

```json
{
  "version": 0,
  "kind": "Program",
  "defs": [...],
  "meta": {
    "usings": [...]
  }
}
```

### やること

1. **JsonParserBox に Program JSON ヘルパーを追加**

```hako
static box JsonParserBox {
  // 既存
  method parse(json_str) { ... }
  method parse_object(json_str) { ... }
  method parse_array(json_str) { ... }

  // 新規（Phase 172）
  method parse_program(json_str) {
    local obj = me.parse_object(json_str)
    if obj == null { return null }

    // Program JSON の必須フィールド確認
    local version = obj.get("version")
    local kind = obj.get("kind")
    if version == null or kind != "Program" { return null }

    // ProgramJSONBox を返す
    local prog = new ProgramJSONBox()
    prog._obj = obj
    return prog
  }
}
```

2. **ProgramJSONBox の定義**

```hako
// 薄いラッパー: Program JSON の構造に特化
static box ProgramJSONBox {
  _obj: MapBox  # 内部で parse_object() の結果を保持

  method get_version() {
    return me._obj.get("version")
  }

  method get_kind() {
    return me._obj.get("kind")
  }

  method get_defs() {
    // ArrayBox を返す
    return me._obj.get("defs")
  }

  method get_meta() {
    // MapBox を返す
    return me._obj.get("meta")
  }

  method get_usings() {
    local meta = me.get_meta()
    if meta == null { return null }
    return meta.get("usings")
  }
}
```

3. **実装場所**
   - `tools/hako_shared/json_parser.hako` に追加
   - Phase 171 実装の拡張として統合

### 成果物

- `parse_program()` メソッド実装
- ProgramJSONBox 型定義

---

## 5. Task 3: Stage-B / selfhost 補助から JsonParserBox に寄せる

### 対象ファイル

1. **第一候補**: `lang/src/compiler/entry/compiler.hako`
   - Program(JSON v0) を読む主要箇所
   - 置き換え効果が大きい

2. **第二候補**: `apps/selfhost-vm/json_loader.hako`
   - JSON ローダー補助
   - 再利用性確認に適切

### やること

1. **現状の JSON 読み取り処理を特定**
   ```hako
   // 修正前の例（自前 substring）
   local json_str = file.read("program.json")
   local version_pos = json_str.indexOf('"version"')
   local defs_pos = json_str.indexOf('"defs"')
   // ... 手作業の解析 ...
   ```

2. **JsonParserBox を使用した実装に置き換え**
   ```hako
   // 修正後
   local json_str = file.read("program.json")
   local prog = JsonParserBox.parse_program(json_str)
   if prog == null {
     print("[ERROR] Invalid Program JSON")
     return null
   }

   local version = prog.get_version()
   local defs = prog.get_defs()
   local usings = prog.get_usings()
   // 構造化されたアクセス
   ```

3. **段階的適用**
   - 初回は 1 箇所のみ（lang/src/compiler/entry/compiler.hako）
   - テスト成功後、2 箇所目（apps/selfhost-vm/json_loader.hako）
   - 全箇所の一括置き換えはしない

### 成果物

- 修正済み `lang/src/compiler/entry/compiler.hako`
- 修正済み `apps/selfhost-vm/json_loader.hako`（オプション）

---

## 6. Task 4: hako_check 側の JsonParserBox 利用を仕上げる

### やること

1. **Phase 171 の残りタスク確認**
   - `analysis_consumer.hako` から手書き JSON パーサが完全に削除されているか
   - `rule_dead_blocks.hako` が JsonParserBox を正しく使用しているか

2. **未置き換えコードの確認**
   ```bash
   rg 'substring.*\{' tools/hako_check/
   rg 'indexOf.*\"' tools/hako_check/
   ```

3. **JsonParserBox API の十分性確認**
   - hako_check が必要とする API は揃っているか
   - 追加が必要なメソッドはあるか
   - 重い拡張は Phase 173 backlog へ

### 成果物

- hako_check における JsonParserBox 利用の完成
- API 拡張候補リスト（Phase 173 へ）

---

## 7. Task 5: テストと回帰チェック

### テストマトリックス

| 対象 | テストスクリプト | 確認内容 |
|------|---------------|---------|
| hako_check HC019 | `tools/hako_check_deadcode_smoke.sh` | dead code 検出（回帰なし） |
| hako_check HC020 | `tools/hako_check_deadblocks_smoke.sh` | dead block 検出（回帰なし） |
| Stage-B/selfhost | `tools/selfhost/selfhost_phase150_depth1_smoke.sh` | Program JSON 読み込み動作 |
| JsonParserBox | `tools/hako_shared/tests/json_parser_simple_test.hako` | 単体テスト全 PASS |

### やること

1. **hako_check スモークテスト**
   ```bash
   ./tools/hako_check_deadcode_smoke.sh
   ./tools/hako_check_deadblocks_smoke.sh
   ```
   期待: HC019/HC020 の出力が変わらない

2. **Stage-B/selfhost スモークテスト**
   ```bash
   ./tools/selfhost/selfhost_phase150_depth1_smoke.sh
   ```
   期待: Program JSON 読み込みで回帰なし

3. **JsonParserBox 単体テスト**
   ```bash
   NYASH_USE_NY_COMPILER=1 ./target/release/hakorune tools/hako_shared/tests/json_parser_simple_test.hako
   ```
   期待: 全テスト PASS

### 成果物

- 全スモークテスト成功
- 回帰なし確認

---

## 8. Task 6: ドキュメント & CURRENT_TASK 更新

### ドキュメント更新

1. **phase170_hako_json_library_design.md に追記**
   ```markdown
   ## Phase 172 実装結果

   ✅ JsonParserBox 再利用拡大完了
   - Program(JSON v0) 対応: parse_program() + ProgramJSONBox
   - Stage-B 統合: lang/src/compiler/entry/compiler.hako
   - selfhost 統合: apps/selfhost-vm/json_loader.hako
   - hako_check 仕上げ: Phase 171 の残りタスク完了

   📊 統合実績:
   - hako_check: HC019/HC020 で使用（Phase 171）
   - Stage-B: Program JSON 読み込みで使用（Phase 172）
   - selfhost: JSON ローダーで使用（Phase 172）

   📋 未統合箇所（Phase 173+ 候補）:
   - tools/selfhost/helpers.hako（低優先度）
   - apps/tests/ 内の一部テスト
   ```

2. **hako_check_design.md / selfhost 関連 docs を更新**
   ```markdown
   ### JSON 解析の実装場所

   **Phase 172 から全て JsonParserBox に集約**:
   - 場所: tools/hako_shared/json_parser.hako
   - 利用者:
     - hako_check: MIR/CFG JSON 解析
     - Stage-B: Program(JSON v0) 読み込み
     - selfhost: JSON ローディング
   - 単一の真実（SSOT）確立
   ```

3. **CURRENT_TASK.md に Phase 172 セクション追加**
   ```markdown
   ### Phase 172: JsonParserBox 再利用拡大 ✅

   **完了内容**:
   - Program(JSON v0) 対応: parse_program() + ProgramJSONBox
   - Stage-B 統合: lang/src/compiler/entry/compiler.hako で使用開始
   - selfhost 統合: apps/selfhost-vm/json_loader.hako で使用開始
   - hako_check 仕上げ: Phase 171 の残りタスク完了

   **成果**:
   - JSON 処理の単一の真実（SSOT）確立
   - hako_check/Stage-B/selfhost が同じ箱を使用
   - selfhost depth-2 への基盤整備完了

   **次フェーズ**: Phase 173 で to_json() 逆変換、Phase 160+ で .hako JoinIR/MIR 移植
   ```

### git commit

```
feat(json): Phase 172 JsonParserBox reuse expansion

✨ JsonParserBox を Stage-B/selfhost/ツールに統合！

🎯 Program(JSON v0) 対応:
- parse_program() メソッド追加
- ProgramJSONBox 薄いヘルパー定義
- version/kind/defs/meta/usings アクセサ

📦 統合実績:
- lang/src/compiler/entry/compiler.hako: Program JSON 読み込み
- apps/selfhost-vm/json_loader.hako: JSON ローダー
- tools/hako_check/: Phase 171 仕上げ完了

✅ テスト:
- hako_check スモーク: HC019/HC020 回帰なし
- Stage-B/selfhost スモーク: Program JSON 読み込み OK
- JsonParserBox 単体テスト: 全 PASS

🏗️ SSOT 確立:
- JSON 処理が JsonParserBox に集約
- hako_check/Stage-B/selfhost が同じ箱を使用
- selfhost depth-2 への基盤完成
```

---

## ✅ 完成チェックリスト（Phase 172）

- [ ] Task 1: 再利用候補の棚卸し
  - [ ] Program(JSON v0) 消費箇所の洗い出し
  - [ ] 各箇所の評価（適用可否・優先度）
  - [ ] phase170 に追記
- [ ] Task 2: Program(JSON v0) ラッパー追加
  - [ ] parse_program() 実装
  - [ ] ProgramJSONBox 定義
  - [ ] json_parser.hako に統合
- [ ] Task 3: Stage-B/selfhost への適用
  - [ ] lang/src/compiler/entry/compiler.hako 修正
  - [ ] apps/selfhost-vm/json_loader.hako 修正（オプション）
  - [ ] 自前 substring 処理を削除
- [ ] Task 4: hako_check 仕上げ
  - [ ] Phase 171 残りタスク確認
  - [ ] API 十分性確認
  - [ ] 拡張候補リスト作成
- [ ] Task 5: テスト & 回帰
  - [ ] hako_check スモーク
  - [ ] Stage-B/selfhost スモーク
  - [ ] JsonParserBox 単体テスト
- [ ] Task 6: ドキュメント更新
  - [ ] phase170 に追記
  - [ ] hako_check_design.md 更新
  - [ ] selfhost 関連 docs 更新
  - [ ] CURRENT_TASK.md 追加
  - [ ] git commit

---

## 技術的ポイント

### Program(JSON v0) の構造

```json
{
  "version": 0,
  "kind": "Program",
  "defs": [
    {"kind": "Box", "name": "Main", ...},
    {"kind": "Method", "name": "main", ...}
  ],
  "meta": {
    "usings": ["nyashstd", "mylib"]
  }
}
```

### ProgramJSONBox の使い方

```hako
local prog = JsonParserBox.parse_program(json_str)
if prog == null {
  print("[ERROR] Invalid Program JSON")
  return
}

// 型安全なアクセス
local version = prog.get_version()  // Integer
local defs = prog.get_defs()        // ArrayBox
local usings = prog.get_usings()    // ArrayBox?

// defs をループ
for def in defs {
  local kind = def.get("kind")
  if kind == "Box" {
    local name = def.get("name")
    print("Box: " + name)
  }
}
```

### 段階的適用の重要性

- 一気に全箇所を変更しない
- 1 箇所 → テスト → 2 箇所目 の順序
- 問題があれば早期発見

---

**作成日**: 2025-12-04
**Phase**: 172（JsonParserBox 再利用拡大）
**予定工数**: 2-3 時間
**難易度**: 低-中（既存実装の適用 + 薄いラッパー追加）
**期待効果**: JSON 処理 SSOT 確立、selfhost depth-2 基盤完成
Status: Historical
