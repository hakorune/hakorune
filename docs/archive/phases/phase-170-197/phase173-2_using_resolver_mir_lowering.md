# Phase 173-2: using resolver + MIR lowering 修正

## 0. ゴール

**using 経由で静的 Box を正しく解決して VM に渡す。**

目的：
- .hako の using resolver が静的 Box を「型/名前空間」として環境に登録
- MIR lowering が静的 Box メソッドを適切な BoxCall/MethodCall に変換
- JsonParserBox が「正式な標準ライブラリ」として完全動作

---

## 1. 背景

### Phase 173 前半の成果

- ✅ Task 1: 名前解決経路調査完了
- ✅ Task 2: 仕様固定（docs）完了
- ✅ Task 3: JsonParserBox バグ修正完了
  - MIR Nested-If-in-Loop Bug 発見・回避
  - `while` → `loop()` 統一
  - json_parser.hako 正常動作確認

### この Phase 後半でやること

**Task 4-6**: using 経由での静的 Box 解決を完全実装
- using resolver 修正
- MIR lowering 調整
- テスト・ドキュメント整備

---

## 2. Scope / Non-scope

### ✅ やること

1. **using resolver 修正**
   - 静的 Box を「型/名前空間」として環境に登録
   - `Alias.BoxName` を型参照として識別

2. **MIR lowering 調整**
   - 静的 Box 呼び出しを BoxCall/MethodCall に正しく変換
   - 既存の instance Box / plugin Box 呼び出しに影響を与えない

3. **テスト・ドキュメント整備**
   - json_parser_min.hako 動作確認
   - hako_check スモークテスト
   - ドキュメント更新

### ❌ やらないこと

- CoreBoxId / CoreMethodId 周りの変更
- VM コアの変更
- JsonParserBox の機能追加

---

## 3. Task 4: using resolver 修正

### 目的

.hako 側の using 解決が、静的 Box を「型/名前空間」として環境に登録できるようにする。

### 対象ファイル

- `lang/src/compiler/entry/using_resolver*.hako`
- `lang/src/compiler/runner/launcher.hako`（using を扱うエントリ周辺）

### やること

#### 1. 現状の動作確認

```bash
# using resolver の構造確認
rg 'using|resolver|Alias' lang/src/compiler/entry/ --type hako -n

# AST/シンボルテーブルの確認
rg 'symbol|type.*table|namespace' lang/src/compiler/ --type hako -l
```

**確認項目**:
- `using tools.hako_shared.json_parser as JsonLib` のとき：
  - `JsonLib` が AST/内部シンボルテーブル上でどう扱われているか
  - 単なるモジュール名なのか、Box 名（JsonParserBox）まで解決されているか

#### 2. 修正方針

**静的 Box の名前空間登録**:
```
using ... as Alias で static box を含む .hako を読み込んだ場合：
  ↓
Alias を「static box を持つ名前空間」として登録
  ↓
Alias.JsonParserBox のような参照を「型参照」として識別可能に
```

**設計上の決定**:
- `Alias.parse(...)` を「Alias が static box そのもの」な場合に認めるか？
  - 今は `new Alias.JsonParserBox()` パターン優先で良い
  - JsonParserBox はインスタンス Box の方が自然

#### 3. 実装内容

**A. 静的 Box の登録**:
```hako
method register_static_box_alias(alias, box_name) {
    # シンボルテーブルに alias → static box のマッピング登録
    local entry = new SymbolEntry()
    entry.set_kind("static_box_namespace")
    entry.set_box_name(box_name)

    me.symbol_table.register(alias, entry)
}
```

**B. 型参照の識別**:
```hako
method resolve_type_ref(ref) {
    # Alias.BoxName パターンの判定
    if ref.has_namespace() {
        local alias = ref.get_namespace()
        local entry = me.symbol_table.lookup(alias)

        if entry.is_static_box_namespace() {
            # 型参照として解決
            return me.resolve_as_type(entry, ref.get_name())
        }
    }

    # 通常の型参照処理
    return me.resolve_normal_type(ref)
}
```

#### 4. 変更を小さくする

**重要な制約**:
- 既存の using（namespace/file using）を壊さない
- 変更は「静的 Box を alias に結び付ける部分」だけに限定
- 段階的な変更で影響範囲を最小化

### 成果物

- using_resolver*.hako 修正
- 静的 Box の名前空間登録実装
- 型参照識別ロジック追加

---

## 4. Task 5: MIR lowering の調整

### 目的

AST 上で「Alias.JsonParserBox」やそのメソッド呼び出しが正しく解決された前提で、MIR lowering がそれを適切な BoxCall/MethodCall に落とすようにする。

### 対象ファイル

- `src/mir/builder/calls/resolver.rs`
- `src/mir/builder/calls/call_unified.rs`

### やること

#### 1. 具体的なパターンを想定

**パターン A: Box 生成**
```hako
local p = new JsonLib.JsonParserBox()
```
↓ MIR
```rust
NewBox { box_id: JsonParserBox, namespace: Some("JsonLib") }
```

**パターン B: インスタンスメソッド**
```hako
p.parse("123")
```
↓ MIR（既に動いているはず）
```rust
MethodCall { receiver: p, method: "parse", args: ["123"] }
```

**パターン C: 静的メソッド（将来）**
```hako
JsonLib.JsonParserBox.parse_static(...)
```
↓ MIR（今は未対応、余地だけ残す）
```rust
BoxCall { box: JsonParserBox, method: "parse_static", args: [...] }
```

#### 2. ロジック修正

**A. 静的 Box 判定**
```rust
// src/mir/builder/calls/resolver.rs
fn is_static_box_call(ast_node: &AstNode) -> bool {
    // receiver の部分が名前空間付き静的 Box 由来であるか判定
    if let Some(namespace) = ast_node.get_namespace() {
        if let Some(entry) = self.symbol_table.lookup(namespace) {
            return entry.is_static_box_namespace();
        }
    }
    false
}
```

**B. BoxId の正しい解決**
```rust
// src/mir/builder/calls/call_unified.rs
fn resolve_box_id(ast_node: &AstNode) -> Result<BoxId, Error> {
    if self.is_static_box_call(ast_node) {
        // 名前空間付き静的 Box の場合
        let box_name = ast_node.get_box_name();
        return self.unified_box_registry.lookup_box_id(box_name);
    }

    // 通常の Box 解決
    self.resolve_normal_box_id(ast_node)
}
```

#### 3. 安全性の確保

**重要な制約**:
- 既存の instance Box / plugin Box 呼び出しに影響を与えない
- 分岐を追加する形で実装
- CoreBoxId / CoreMethodId 周りの規則には手を出さない

**テスト戦略**:
```bash
# 既存テストが通ることを確認
cargo test --release

# 特に instance Box のテスト
cargo test --release instance_box
cargo test --release plugin_box
```

### 成果物

- resolver.rs 修正
- call_unified.rs 修正
- 静的 Box 判定ロジック追加

---

## 5. Task 6: テスト・docs・CURRENT_TASK でフェーズを締める

### やること

#### 1. JsonParserBox 最小テスト

**テストファイル** (`apps/tests/json_parser_min.hako`):
```hako
using tools.hako_shared.json_parser as JsonLib

static box Main {
    main() {
        local parser = new JsonLib.JsonParserBox()
        local v = parser.parse("{\"x\":1}")
        return 0
    }
}
```

**実行確認**:
```bash
./target/release/nyash apps/tests/json_parser_min.hako

# 期待: RC 0、Unknown Box/method が出ない
```

#### 2. hako_check スモークテスト

```bash
# HC019 スモークテスト
./tools/hako_check_deadcode_smoke.sh

# HC020 スモークテスト
./tools/hako_check_deadblocks_smoke.sh

# 期待: JsonParserBox 差し替え後も HC019/HC020 の挙動に変化なし
```

#### 3. ドキュメント更新

**A. Phase 170-173 ドキュメント**:
- `phase170_hako_json_library_design.md`
- `phase171_jsonparserbox_implementation.md`
- `phase173_using_static_box_resolution.md`

追記内容：
```markdown
### Phase 173 完了（2025-12-04）

**成果**:
- using + static box の解決が整備され、JsonParserBox が正式にライブラリとして使用可能に
- MIR Nested-If-in-Loop Bug を発見・回避
- using resolver + MIR lowering の統合完了

**技術的成果**:
- 静的 Box を名前空間として環境に登録
- BoxCall/MethodCall への正しい変換
- VM で安定して動作確認
```

**B. using.md**:
```markdown
### JsonParserBox を例にした静的 Box の using

\`\`\`hako
using tools.hako_shared.json_parser as JsonLib

static box Main {
    main() {
        # 静的 Box のインスタンス化
        local parser = new JsonLib.JsonParserBox()

        # メソッド呼び出し
        local obj = parser.parse("{\"name\": \"Alice\"}")
        local name = obj.get("name")

        return 0
    }
}
\`\`\`
```

#### 4. CURRENT_TASK 更新

```markdown
### Phase 173: using + 静的 Box メソッド解決 ✅

**完了内容**:
- Task 1-3: 調査・仕様固定・JsonParserBox バグ修正 ✅
- Task 4-6: using resolver + MIR lowering 統合 ✅

**技術的成果**:
- 静的 Box の using 解決と MIR lowering が整備
- JsonParserBox を含む static box library 呼び出しが VM 上で安定動作
- MIR Nested-If-in-Loop Bug を発見・ドキュメント化

**次のステップ**:
- Phase 174: to_json() 逆変換実装
- Phase 175: selfhost depth-2 JSON 統一化
```

#### 5. git commit

```bash
git add .
git commit -m "feat(using): Phase 173-2 using resolver + MIR lowering integration complete

🎉 using + 静的 Box 解決の完全実装！

🔧 実装内容:
- using resolver: 静的 Box を名前空間として環境に登録
- MIR lowering: BoxCall/MethodCall への正しい変換実装
- 安全性: 既存 instance Box / plugin Box 呼び出しに影響なし

✅ テスト結果:
- json_parser_min.hako: RC 0、エラーなし
- hako_check HC019: PASS
- hako_check HC020: PASS
- 回帰テスト: すべて PASS

📚 ドキュメント整備:
- Phase 170-173 完了記録
- using.md に JsonParserBox 実例追加
- CURRENT_TASK.md 更新

🎯 JsonParserBox が正式な標準ライブラリとして完全動作！
Phase 171-2 のブロック解除、hako_check 統合完了

🤖 Generated with Claude Code
Co-Authored-By: Claude <noreply@anthropic.com>"
```

---

## ✅ 完成チェックリスト（Phase 173-2）

- [ ] Task 4: using resolver 修正
  - [ ] 現状動作確認
  - [ ] 静的 Box 名前空間登録実装
  - [ ] 型参照識別ロジック追加
- [ ] Task 5: MIR lowering 調整
  - [ ] 静的 Box 判定ロジック追加
  - [ ] BoxId 解決修正
  - [ ] 安全性確認（既存テスト PASS）
- [ ] Task 6: テスト・ドキュメント整備
  - [ ] json_parser_min.hako 動作確認
  - [ ] hako_check スモーク PASS
  - [ ] ドキュメント更新完了
  - [ ] CURRENT_TASK.md 更新
  - [ ] git commit

---

## 技術的注意点

### 段階的な変更

1. **Phase 1**: using resolver のみ修正 → テスト
2. **Phase 2**: MIR lowering のみ修正 → テスト
3. **Phase 3**: 統合テスト → ドキュメント

### 安全性の確保

- 既存コードへの影響を最小化
- 各段階でテストを実行
- 問題があればロールバック可能に

### デバッグ戦略

```bash
# using resolver のデバッグ
NYASH_DEBUG_USING=1 ./target/release/nyash test.hako

# MIR lowering のデバッグ
./target/release/nyash --dump-mir test.hako

# VM 実行のデバッグ
NYASH_CLI_VERBOSE=1 ./target/release/nyash test.hako
```

---

## 次のステップ

Phase 173-2 完了後：
- **Phase 174**: to_json() 逆変換実装
- **Phase 175**: selfhost depth-2 JSON 統一化
- **Phase 160+**: .hako JoinIR/MIR 移植

JsonParserBox が「正式な標準ライブラリ」として完全に機能するようになり、その上に hako_check のルール追加や .hako JoinIR/MIR を安心して乗せられるようになる！

---

**作成日**: 2025-12-04
**Phase**: 173-2（using resolver + MIR lowering）
**予定工数**: 4-6 時間
**難易度**: 高（名前解決・MIR lowering の統合）
**前提**: Phase 173 前半（Task 1-3）完了
Status: Historical
