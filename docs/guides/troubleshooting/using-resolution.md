# Using 解決トラブルシューティング

**作成日**: 2025-11-22
**対象**: Phase 21.7++ NamingBox SSOT 統一化後のトラブルシューティング

---

## エラーパターン別対処法

### 1. `[using] Module not found: 'ModuleName'`

**原因**:
- hako.toml / nyash.toml に alias が定義されていない
- TOML parse エラーで alias が読み込めていない
- タイポ（モジュール名の綴り間違い）

**対処法**:

1. **TOML ファイルを確認**
   ```bash
   cat hako.toml | grep "ModuleName"
   # hako.toml が無い場合は nyash.toml を確認
   cat nyash.toml | grep "ModuleName"
   ```

2. **TOML parse エラーを確認**
   - Phase 0 で実装した即座表示機能により、TOML エラーは起動時に表示されます
   - エラーメッセージに修正方法が含まれています

3. **詳細ログで確認**
   ```bash
   NYASH_DEBUG_USING=1 ./target/release/hakorune program.hako
   ```

4. **"Did you mean?" 提案を確認**
   - Phase 0.3 で実装された機能により、類似モジュール名が自動提案されます
   - 提案に正しい綴りが含まれているか確認

**修正例**:
```toml
# hako.toml (preferred)
[using.aliases]
StringUtils = "lang/src/shared/common/string_helpers.hako"  # ← 追加
```

---

### 2. `Function not found: Box.method/N`

**原因**:
- arity（引数の数）が一致していない
- 関数名の綴り間違い
- 関数が定義されていない

**対処法**:

1. **"Did you mean?" 提案を確認**
   - Phase 0.2 で実装された機能により、類似関数名が自動提案されます
   ```
   Function not found: StringUtils.starts_with/1

   💡 Did you mean:
      - StringUtils.starts_with/2
      - StringUtils.starts_with/3
   ```

2. **デバッグログで詳細確認**
   ```bash
   NYASH_DEBUG_FUNCTION_LOOKUP=1 ./target/release/hakorune program.hako
   ```

   出力例:
   ```
   [DEBUG/vm] Looking up function: 'StringUtils.starts_with'
   [DEBUG/vm]   Parsed: box='StringUtils', method='starts_with', arity=None
   [DEBUG/vm]   canonical: 'StringUtils.starts_with/2'
   [DEBUG/vm]   ✅ 'StringUtils.starts_with/2' found
   ```

3. **arity を確認**
   - 関数定義と呼び出しの引数の数が一致しているか確認
   - Phase 2 で実装された自動補完により、arity なし呼び出しは自動補完されます

**修正例**:
```nyash
// 誤: arity が一致しない
StringUtils.starts_with("hello")  // arity 1 → "starts_with/1" を探す

// 正: arity が一致
StringUtils.starts_with("hello", "he")  // arity 2 → "starts_with/2" を探す
```

---

### 3. `VM Runtime Error: arity mismatch`

**原因**:
- MIR Builder 側と VM 側で arity の扱いが不一致
- methodization で receiver が二重に追加された
- static box method で receiver が誤って追加された

**対処法**:

1. **Methodization トレースログを確認**
   ```bash
   NYASH_METHODIZE_TRACE=1 ./target/release/hakorune program.hako
   ```

   出力例:
   ```
   [methodize] Global(Calculator.add/2) → Method{Calculator.add, recv=%5}
   ```

2. **StaticMethodId パース結果を確認**
   - Phase 2/3 で実装された構造化表現により、パース結果が明示的に表示されます

3. **Methodization を無効化して確認**
   ```bash
   HAKO_MIR_BUILDER_METHODIZE=0 ./target/release/hakorune program.hako
   ```

**Phase 21.7++ での修正**:
- Phase 3 で known.rs と unified_emitter.rs の receiver 追加ロジックを修正済み
- static box method では receiver を追加しないようガード実装済み

---

### 4. Silent Failure（エラーが表示されない）

**原因**:
- Phase 0 以前のバージョンを使用している
- エラーメッセージが stderr に出力されているが見逃している

**対処法**:

1. **Phase 0 実装済みか確認**
   ```bash
   git log --oneline | grep "Phase 0"
   ```

   Phase 0 commit (63012932) 以降であれば、Silent Failure は根絶されています

2. **詳細診断モードで実行**
   ```bash
   NYASH_CLI_VERBOSE=1 ./target/release/hakorune program.hako 2>&1 | tee debug.log
   ```

3. **すべてのデバッグフラグを有効化**
   ```bash
   NYASH_DEBUG_FUNCTION_LOOKUP=1 \
   NYASH_DEBUG_USING=1 \
   NYASH_METHODIZE_TRACE=1 \
   NYASH_CLI_VERBOSE=1 \
   ./target/release/hakorune program.hako
   ```

**Phase 21.7++ での修正**:
- ✅ TOML parse エラー即座表示（pipeline.rs）
- ✅ 関数ルックアップ「Did you mean?」提案（global.rs）
- ✅ using not found 詳細化（strip.rs）

---

## デバッグフローチャート

```
エラー発生
    ↓
1. エラーメッセージを確認
    ├─ "Did you mean?" 提案あり → 提案に従う
    ├─ "Module not found" → nyash.toml 確認
    └─ "Function not found" → 以下へ
    ↓
2. デバッグログ有効化
    NYASH_DEBUG_FUNCTION_LOOKUP=1 実行
    ↓
3. パース結果を確認
    box/method/arity が正しいか確認
    ↓
4. 関数定義を確認
    定義されている関数名と arity を確認
    ↓
5. Methodization を確認
    NYASH_METHODIZE_TRACE=1 で変換ログ確認
    ↓
6. それでも解決しない場合
    - Issue 報告（GitHub）
    - チームに相談
```

---

## よくある質問

### Q1: `StringUtils.starts_with` は動くが `StringUtils.starts_with/2` で呼ぶとエラーになる

**A1**: Phase 2 実装により、arity なし呼び出しは自動補完されます。明示的に arity を指定する必要はありません。

```nyash
// ✅ 推奨: arity なし（自動補完）
StringUtils.starts_with("hello", "he")

// ✅ 動作するが冗長
// MIR レベルでは "StringUtils.starts_with/2" になる
```

### Q2: `main._nop/0` が `Main._nop/0` に normalize される理由は？

**A2**: NamingBox の canonical_box_name() により、`main` → `Main` に正規化されます（src/mir/naming.rs:22-27）。

これは static box 名の一貫性を保つための仕様です。

### Q3: Methodization ON/OFF でどう動作が変わる？

**A3**:

**Methodization OFF** (HAKO_MIR_BUILDER_METHODIZE=0):
```
Global("Calculator.add/2") → VM が Global 関数として実行
```

**Methodization ON** (既定):
```
Global("Calculator.add/2")
  → NewBox(Calculator) → Method{Calculator.add, recv=singleton}
  → VM が Method として実行
```

どちらも最終的な動作は同じですが、MIR の表現が異なります。

### Q4: Phase 21.7++ で何が変わった？

**A4**: 主な変更点:

| 項目 | Phase 0-3 前 | Phase 0-3 後 |
|------|-------------|-------------|
| エラー表示 | Silent Failure 多数 | 即座表示 + 提案 |
| 名前パース | ad-hoc（素手 split） | SSOT（StaticMethodId） |
| arity 補完 | Hotfix（文字列操作） | 正式実装（構造化） |
| デバッグ | 難しい | 環境変数で詳細表示 |

---

## 関連ドキュメント

- **Phase 21.7 README**: [docs/private/roadmap2/phases/phase-21.7-normalization/README.md](../roadmap/phases/phase-21.7-normalization/README.md)
- **Phase 21.7++ チェックリスト**: [docs/development/current/main/phase-21.7-naming-ssot-checklist.md](../current/main/phase-21.7-naming-ssot-checklist.md)
- **NamingBox 実装**: [src/mir/naming.rs](../../src/mir/naming.rs)
- **StaticMethodId テスト**: [src/tests/namingbox_static_method_id.rs](../../src/tests/namingbox_static_method_id.rs)

---

## 更新履歴

- 2025-11-22: Phase 21.7++ Phase 4 完了時に作成
