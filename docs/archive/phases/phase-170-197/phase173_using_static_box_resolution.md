# Phase 173: using + 静的 Box メソッド解決の整備

## 0. ゴール

**using した静的 Box をライブラリとして正しく呼び出せるようにする。**

目的：
- `.hako` で `using` した静的 Box（JsonParserBox 等）を VM から正常に呼び出せるようにする
- Rust VM コアには触らず、`.hako` の using/resolver + MIR lowering 側で解決
- Phase 171-2 で発見された「Unknown method on InstanceBox」問題を根本解決

---

## 1. 背景

### Phase 171-2 の発見

```
[ERROR] ❌ [rust-vm] VM error: Invalid instruction: Unknown method '_skip_whitespace' on InstanceBox
```

**根本原因**:
- `using tools.hako_shared.json_parser as JsonParserBox` で import した静的 Box が
- VM 側で InstanceBox として扱われ、内部メソッドが解決できない

**影響**:
- hako_check での JsonParserBox 統合が動作しない
- selfhost での JSON 処理統一化がブロック

### この Phase でやること

1. 名前解決経路の調査
2. 仕様の明確化（ドキュメント）
3. using/resolver の修正
4. MIR lowering の修正
5. JsonParserBox 動作確認

---

## 2. Scope / Non-scope

### ✅ やること

1. **現状の名前解決経路を調査**
   - AST 上での静的 Box の表現確認
   - MIR lowering での判別処理確認
   - VM エラー発生箇所の特定

2. **仕様の固定（docs）**
   - using.md に静的 Box の使い方を追記
   - LANGUAGE_REFERENCE_2025.md に方針追加

3. **.hako 側の using/resolver 修正**
   - 静的 Box 名を型として環境に登録
   - AST にフラグ追加

4. **MIR lowering 修正**
   - 静的 Box 呼び出しの判別条件追加
   - BoxCall/MethodCall への正しい解決

5. **テストと回帰確認**
   - JsonParserBox 最小テスト
   - hako_check スモークテスト

### ❌ やらないこと

- Rust VM コアの変更（実行側は既に静的 Box を扱える前提）
- JsonParserBox の機能追加
- 新しい解析ルール追加

---

## 3. Task 1: 現状の名前解決経路を調べる

### 対象ファイル

**.hako 側**:
- `lang/src/compiler/parser/parser_box.hako`
- `lang/src/compiler/entry/using_resolver*.hako`（Stage-1/UsingResolver）

**Rust 側**:
- `src/mir/builder/calls/resolver.rs`
- `src/mir/builder/calls/call_unified.rs`

### やること

1. **AST 表現の確認**
   - `using ... as ...` で静的 Box を import したときの AST 構造
   - `JsonLib.parse(...)` / `JsonLib.JsonParserBox(...)` がどの kind で表現されるか
   - TypeRef / VarRef の区別

2. **MIR lowering の確認**
   - ノードが「インスタンスメソッド」か「静的 Box メソッド」か判別できているか
   - BoxCall vs MethodCall の分岐ロジック

3. **エラー発生箇所の特定**
   - 具体的にどの Box 名 / メソッド名で Unknown になるか
   - `_skip_whitespace` 等の内部メソッドがなぜ解決できないか

### 成果物

- 名前解決経路のメモ
- AST 構造の確認結果
- エラー発生箇所の特定

---

## 4. Task 2: 仕様を固定する（docs）

### docs/reference/language/using.md への追記

```markdown
## 静的 Box の using

静的 Box をライブラリとして使用する場合：

\`\`\`hako
using tools.hako_shared.json_parser as JsonLib

static box Main {
    main() {
        // 方法1: 静的 Box のインスタンス化
        local parser = new JsonLib.JsonParserBox()
        local v = parser.parse("{\"x\":1}")

        // 方法2: 静的メソッド呼び出し（Alias が static box の場合）
        local result = JsonLib.parse("{\"y\":2}")

        return 0
    }
}
\`\`\`

### 許容する呼び方

1. `new Alias.BoxName()` - 静的 Box 定義を namespace 的に使う
2. `Alias.func()` - Alias が static box そのものの場合
3. `instance.method()` - インスタンスメソッド呼び出し
```

### LANGUAGE_REFERENCE_2025.md への追記

```markdown
## static box のライブラリ利用

- `static box` は 1 ファイル 1 個のシングルトン / ライブラリ的な箱として扱う
- instance Box と同じく `using` から名前解決されるべき
- `using ... as Alias` でインポートした静的 Box は：
  - `Alias.method()` で静的メソッド呼び出し
  - `new Alias.BoxName()` で内部 Box のインスタンス化
```

### 成果物

- using.md 更新
- LANGUAGE_REFERENCE_2025.md 更新

---

## 5. Task 3: .hako 側の using/resolver を修正

### 対象ファイル

- `lang/src/compiler/entry/using_resolver*.hako`
- Stage-3 専用の resolver（あれば）

### 方針

1. **using 解決時の型登録**
   - 静的 Box 名を型として環境に登録
   - `Alias → static box / namespace`
   - `Alias.BoxName → type or Box`

2. **AST へのフラグ追加**
   - `JsonLib.JsonParserBox` を「型参照」＋「Box 名」として扱えるようにフラグ付け
   - これにより MIR lowering で判断可能に

3. **パーサの修正（必要なら）**
   - ドット記法の解釈を拡張
   - 静的 Box 参照のノード種別追加

### 成果物

- using_resolver*.hako 修正
- AST フラグ追加

---

## 6. Task 4: MIR lowering の修正

### 対象ファイル

- `src/mir/builder/calls/resolver.rs`
- `src/mir/builder/calls/call_unified.rs`

### やること

1. **静的 Box 呼び出しの判別**
   - Task 3 で付けた AST 情報を使用
   - 「インスタンスメソッド」vs「静的 Box メソッド」の分岐

2. **正しい解決**
   - `JsonLib.JsonParserBox` → 正しい BoxId に解決
   - `JsonLib.parse` → 正しい MethodId に解決
   - BoxCall/MethodCall に正しく落とす

3. **既存分岐の保護**
   - instance call / plugin call の分岐を壊さない
   - carefully な実装

### 期待される動作

```
AST: JsonLib.parse(...)
  ↓ MIR lowering
MIR: BoxCall { box: "JsonParserBox", method: "parse", args: [...] }
  ↓ VM 実行
正常動作（Unknown エラーなし）
```

### 成果物

- resolver.rs 修正
- call_unified.rs 修正（必要なら）

---

## 7. Task 5: JsonParserBox を使った最小ケースで確認

### テストファイル作成

`apps/tests/json_parser_min.hako`:

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

### 実行コマンド

```bash
# 最小テスト
./target/release/nyash apps/tests/json_parser_min.hako

# 期待: RC 0、Unknown box/method エラーなし
```

### hako_check スモークテスト

```bash
# HC019 スモークテスト
./tools/hako_check_deadcode_smoke.sh

# HC020 スモークテスト
./tools/hako_check_deadblocks_smoke.sh
```

### 期待される結果

- 最小テスト: RC 0、エラーなし
- hako_check: JsonParserBox 利用部が正常動作
- 回帰なし

### 成果物

- テストファイル作成
- 動作確認完了

---

## 8. Task 6: ドキュメント & CURRENT_TASK 更新

### ドキュメント更新

1. **phase170_hako_json_library_design.md / phase171_***:
   - 「JsonParserBox を using で読み込んだ静的 Box ライブラリとしても VM から正常に使えるようになった」を追記

2. **using.md**:
   - 静的 Box を using して使う実例を追加（JsonLib パターン）

3. **CURRENT_TASK.md**:
   ```markdown
   ### Phase 173: using + 静的 Box メソッド解決 ✅
   - .hako using + 静的 Box 呼び出しが言語仕様と実装の両方で揃った
   - JsonParserBox が selfhost/hako_check にとって「正式なライブラリ」として使用可能に
   - Phase 171-2 の完全動作達成
   ```

### git commit

```bash
git add .
git commit -m "feat(using): Phase 173 static box method resolution for using statement

🎉 using + 静的 Box メソッド解決の整備完了！

🔧 実装内容:
- using/resolver: 静的 Box 名の型登録実装
- MIR lowering: 静的 Box 呼び出しの正しい解決
- AST: 静的 Box 参照のフラグ追加

📚 仕様整備:
- using.md: 静的 Box の使い方追記
- LANGUAGE_REFERENCE_2025.md: static box ライブラリ利用の方針追加

✅ テスト結果:
- json_parser_min.hako: RC 0、エラーなし
- hako_check スモーク: PASS
- 回帰なし確認

🎯 JsonParserBox が正式なライブラリとして使用可能に！
Phase 171-2 完全動作達成

🤖 Generated with Claude Code
Co-Authored-By: Claude <noreply@anthropic.com>"
```

---

## ✅ 完成チェックリスト（Phase 173）

- [ ] Task 1: 名前解決経路調査
  - [ ] AST 表現確認
  - [ ] MIR lowering 確認
  - [ ] エラー発生箇所特定
- [ ] Task 2: 仕様固定（docs）
  - [ ] using.md 更新
  - [ ] LANGUAGE_REFERENCE_2025.md 更新
- [ ] Task 3: using/resolver 修正
  - [ ] 型登録実装
  - [ ] AST フラグ追加
- [ ] Task 4: MIR lowering 修正
  - [ ] 静的 Box 判別条件追加
  - [ ] 正しい解決実装
- [ ] Task 5: テスト確認
  - [ ] json_parser_min.hako 動作確認
  - [ ] hako_check スモーク PASS
- [ ] Task 6: ドキュメント更新
  - [ ] phase170/171 更新
  - [ ] using.md 更新
  - [ ] CURRENT_TASK.md 更新
  - [ ] git commit

---

## 技術的注意点

### Rust VM は変更しない

VM 側は既に静的 Box のメソッドを扱える前提。MIR が正しく BoxCall/MethodCall になれば VM はそのまま動くはず。

### 段階的確認

1. まず AST レベルで正しく表現されているか確認
2. 次に MIR レベルで正しく変換されているか確認
3. 最後に VM で正しく実行されるか確認

### 既存コードの保護

- instance call / plugin call の分岐を壊さない
- 既存の using 動作に影響を与えない

---

## 次のステップ

Phase 173 完了後：
- **Phase 174**: to_json() 逆変換実装
- **Phase 175**: selfhost depth-2 JSON 統一化
- **Phase 160+**: .hako JoinIR/MIR 移植

これで using + static box の前段整備が完了し、JsonParserBox が正式なライブラリとして使えるようになる！

---

**作成日**: 2025-12-04
**Phase**: 173（using + 静的 Box メソッド解決）
**予定工数**: 4-6 時間
**難易度**: 高（名前解決・MIR lowering の修正）
**Rust VM 変更**: なし（.hako/using 側のみ）
Status: Historical
