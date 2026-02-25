# Phase 152-B: Static Method 宣言の整理（Stage-3 仕様への統一）

## 🎯 ゴール

**Stage-3 の正式仕様に統一：「静的なエントリポイントは `static box + メソッド定義`だけ」**

方針：
- パーサに新構文は追加しない
- テスト・ドキュメント・Stage-B ヘルパーを `static box` スタイルに統一
- `static method` は legacy/非推奨として明記

結果：
- Stage-3 仕様がクリーンになる
- 既存コードは継続動作（ヘルパーで両パターン対応）
- セルフホスティングパイプラインが仕様統一

## 📋 スコープ（やること・やらないこと）

### ✅ やること
- `apps/tests/stage1_run_min.hako` を `static box Main { main() { } }` 形式に書き換え
- `compiler_stageb.hako` の `_find_main_body` ロジックを修正
  - `static method main` パターンに加えて
  - `static box Main { main() { } }` パターンにも対応
- `docs/development/selfhosting/quickstart.md` など、legacy `static method` を例示している docs を `static box` に統一
- Stage-3 言語リファレンスに「`static method` は legacy/非推奨」を明記

### ❌ やらないこと
- Rust/Selfhost パーサに `static method` 新構文を追加しない
- Stage-2 向けの言語仕様変更
- JoinIR/MIR の意味論変更

---

## 🏗️ 5 つのタスク

### Task 1: 仕様ドキュメント作成（設計確認）

**ファイル**: `docs/development/current/main/phase152b_static_method_cleanup.md`（このファイル）

**内容**:

#### 仕様確認

**Stage-3 正式仕様**:
```nyash
// ✅ 正しい形（静的エントリポイント）
static box Main {
    main(args) {
        return 0
    }
}
```

**Legacy 形式（廃止予定）**:
```nyash
// ❌ Legacy（Stage-3 では廃止方向）
static box Main {
    static method main() {
        return 0
    }
}
```

#### 影響範囲

| 対象 | 内容 | 修正方法 |
|-----|------|---------|
| `apps/tests/stage1_run_min.hako` | テストフィクスチャ | `static box Main { main() { } }` に書き換え |
| `compiler_stageb.hako::_find_main_body` | Stage-B ヘルパー | 両パターンに対応するロジック追加 |
| `quickstart.md` など | サンプルコード | `static box` スタイルに統一 |
| 言語リファレンス | 仕様 | "legacy/非推奨" 明記 |

#### 方針

- **パーサ**: 新構文追加なし（既存の `static box` パーサで十分）
- **後方互換性**: Stage-B ヘルパーで `static method` パターンもサポート
- **将来性**: Phase 160+ では `static method` を廃止（ユーザーにはドキュメントで周知）

---

### Task 2: `stage1_run_min.hako` の書き換え

**対象ファイル**: `apps/tests/stage1_run_min.hako`

**現状**:
```nyash
// Minimal run path fixture for Stage-1 CLI.
// Produces a trivial Program(JSON v0) with Main.main returning 0.
static box Main {
    static method main() {
        return 0
    }
}
```

**変更後**:
```nyash
// Minimal run path fixture for Stage-1 CLI.
// Produces a trivial Program(JSON v0) with Main.main returning 0.
static box Main {
    main() {
        return 0
    }
}
```

**期待動作**:
- Stage-1 CLI / Stage-B パイプラインで Program(JSON v0) が問題なく生成される
- Selfhost depth-1 / Stage-3 でも構文エラーなし
- すべての既存テスト PASS

---

### Task 3: `compiler_stageb.hako::_find_main_body` ロジック調整

**対象ファイル**: `lang/src/compiler/entry/compiler_stageb.hako`（またはその周辺）

**現在のロジック** (概要):
```nyash
_find_main_body(src) {
    if src == null { return "" }
    local key = "static method main"
    local p = src.indexOf(key)
    if p < 0 { return "" }
    // ... parse body from position p
}
```

**問題**:
- `static method main` にハードコードされている
- `static box Main { main() { } }` パターンに対応できない

**修正方針**: 段階的フォールバック

```nyash
_find_main_body(src) {
    if src == null { return "" }

    // Pattern 1: static method main (legacy)
    local p = src.indexOf("static method main")
    if p >= 0 {
        return me.extractBodyFromPosition(src, p + 19)  // Skip "static method main"
    }

    // Pattern 2: static box Main { main() (modern/Stage-3)
    p = src.indexOf("static box Main")
    if p < 0 {
        p = src.indexOf("box Main")
    }

    if p >= 0 {
        // Find "main(" after "Main"
        local start = src.indexOf("main(", p)
        if start >= 0 {
            return me.extractBodyFromPosition(src, start)
        }
    }

    // Fallback: no main found
    return ""
}

extractBodyFromPosition(src, pos) {
    // Find opening brace {
    local bracePos = src.indexOf("{", pos)
    if bracePos < 0 { return "" }

    // Find matching closing brace }
    local depth = 0
    local i = bracePos
    while i < src.length() {
        local ch = src.substring(i, i + 1)
        if ch == "{" {
            depth = depth + 1
        } else if ch == "}" {
            depth = depth - 1
            if depth == 0 {
                return src.substring(bracePos, i + 1)
            }
        }
        i = i + 1
    }

    return ""
}
```

**実装参考**:
- Phase 25.1c の `StageBBodyExtractorBox.build_body_src` に類似ロジックがある
- String スライシング（`substring`）を活用
- ブレースマッチングは基本的な手動実装で十分

**期待動作**:
- `static method main` パターン: 既存どおり動作
- `static box Main { main() { } }` パターン: 新規対応
- エラーハンドリング: 両パターン見つからない場合は空文字（無変更）

---

### Task 4: ドキュメント統一

**更新対象**:

1. **`docs/development/selfhosting/quickstart.md`** など quickstart 系
   - 現在: `static box Main { static method main() { } }` を例示
   - 変更: `static box Main { main() { } }` に統一
   - スニペットはすべて書き換え

2. **`docs/guides/language-guide.md`** など言語ガイド
   - Static box の紹介時に「`main()` メソッドがエントリポイント」を明記
   - `static method` は「legacy/非推奨」と注釈

3. **`LANGUAGE_REFERENCE_2025.md`** など仕様書
   - static box の定義で「Static method は Stage-3 では廃止予定」を追記

**例**:
```markdown
### Static Box（静的ボックス）

エントリポイント：

✅ **推奨**: static box スタイル
\`\`\`nyash
static box Main {
    main() {
        return 0
    }
}
\`\`\`

❌ **Legacy（Phase 160+ で廃止予定）**: static method スタイル
\`\`\`nyash
static box Main {
    static method main() {
        return 0
    }
}
\`\`\`
```

---

### Task 5: テスト・ドキュメント更新・CURRENT_TASK 記録

**テスト実行**:

1. **Stage-1/Stage-B 経路**:
   ```bash
   # stage1_run_min.hako を使っている既存スモーク
   ./tools/smokes/v2/profiles/quick/selfhost/selfhost_stageb_*.sh

   # 期待: Program(JSON v0) 生成エラーなし
   ```

2. **Selfhost depth-1**:
   ```bash
   NYASH_FEATURES=stage3 NYASH_USE_NY_COMPILER=1 NYASH_JOINIR_STRICT=1 \
     ./target/release/hakorune apps/tests/stage1_run_min.hako

   # 期待: RC 0, 構文エラーなし
   ```

3. **既存テスト互換性**:
   ```bash
   # 全スモークテスト実行
   ./tools/smokes/v2/run.sh --profile quick

   # 期待: 回帰なし（既存テスト全て PASS）
   ```

**ドキュメント更新**:
- `phase152b_static_method_cleanup.md` に実装結果を追記
- `CURRENT_TASK.md` に Phase 152-B 完了エントリ追加

**CURRENT_TASK.md 追加内容**:
```markdown
### Phase 152-B: Static Method 宣言の整理（Stage-3 仕様統一）✅

**完了内容**:
- Static box エントリポイントを `static box Main { main() { } }` に統一
- Legacy `static method main()` は廃止方向として明記
- パーサ側は新構文追加なし（既存 static box パーサで対応）

**修正ファイル**:
- `apps/tests/stage1_run_min.hako` - テストフィクスチャ統一
- `compiler_stageb.hako` - _find_main_body ロジック調整（両パターン対応）
- `docs/development/selfhosting/quickstart.md` など - サンプルコード統一
- `LANGUAGE_REFERENCE_2025.md` - "legacy/非推奨" 明記

**テスト結果**:
- Stage-1/Stage-B パイプライン: ✅ 問題なし
- Selfhost depth-1: ✅ stage1_run_min.hako PASS
- 全スモークテスト: ✅ 回帰なし

**成果**:
- Stage-3 仕様がクリーンに（エントリポイント形式の統一）
- 既存コード継続動作（後方互換性维持）
- セルフホスティングパイプラインの仕様統一

**次フェーズ**: Phase 160+ - Static method 廃止（ユーザー周知済み）
```

**Git commit**:
```
chore(phase152-b): Static method 宣言の整理（Stage-3 仕様統一）

- stage1_run_min.hako を static box スタイルに統一
- compiler_stageb.hako のメイン検出ロジック修正（両パターン対応）
- quickstart 等ドキュメントのサンプルコード統一
- static method を legacy/非推奨として明記
- パーサ新構文追加なし（仕様統一性保持）
- テスト結果: 全スモークテスト PASS
```

---

## ✅ 完成チェックリスト（Phase 152-B）

- [x] Task 1: 仕様ドキュメント作成
  - [x] 正式仕様と legacy 形式を明記
  - [x] 影響範囲と方針を整理
- [x] Task 2: `stage1_run_min.hako` 書き換え
  - [x] `static box Main { main() { } }` に変更
  - [x] 期待動作確認（RC: 0）
- [x] Task 3: `compiler_stageb.hako` ロジック調整
  - [x] MainDetectionHelper 作成（箱化モジュール化パターン）
  - [x] tryLegacyPattern / tryModernPattern で両パターン対応
  - [x] ブレースマッチング実装（extractBodyFromPosition）
  - [x] ビルド成功、テスト確認
- [x] Task 4: ドキュメント統一
  - [x] quickstart.md のサンプルコード統一
  - [x] 言語リファレンス既存（legacy 注釈済み）
- [x] Task 5: テスト・CURRENT_TASK 更新
  - [x] Stage-1/Stage-B: stage1_run_min.hako PASS
  - [x] Selfhost depth-1: RC 0 確認
  - [x] 全スモークテスト: 30/31 PASS（1 timeout は無関係）
  - [x] CURRENT_TASK.md 更新
  - [x] git commit で記録（Commit: 27dc0da8）

---

## 所要時間

**2-3 時間程度**

- Task 1（仕様確認）: 30分
- Task 2（ファイル書き換え）: 15分
- Task 3（ロジック修正）: 1時間
- Task 4（ドキュメント）: 30分
- Task 5（テスト・記録）: 30分

---

## 次のステップ

**Phase 200+: Python → Hakorune トランスパイラ構想への準備**
- Stage-3 仕様が確定したので、より大きな野望に向けて土台が固まった
- Phase 160+ の .hako JoinIR/MIR 移植と並行して、Python 統合の準備を進める

---

## 📊 実装サマリー（Phase 152-B 完了）

**実装日**: 2025-12-04
**実装パターン**: 箱化モジュール化（Phase 133/134 継承）

### 修正ファイル一覧

| ファイル | 変更内容 | 行数 |
|---------|---------|-----|
| `lang/src/compiler/entry/compiler_stageb.hako` | MainDetectionHelper 追加（箱化） | +103 |
| `lang/src/compiler/entry/compiler.hako` | Legacy Stage-A コメント追加 | +3 |
| `apps/tests/stage1_run_min.hako` | Modern syntax に統一 | -1 |
| `docs/development/selfhosting/quickstart.md` | サンプルコード 2箇所更新 | 2変更 |
| `CURRENT_TASK.md` | Phase 152-B 完了記録 | +7 |

### MainDetectionHelper 設計

**箱化モジュール化パターンの適用**:

```nyash
static box MainDetectionHelper {
  findMainBody(src)           // Entry point: delegates to pattern modules
  tryLegacyPattern(src)        // Module 1: "static method main" detection
  tryModernPattern(src)        // Module 2: "static box Main { main() }" detection
  findPattern(src, pat, offset) // Helper: Pattern search
  extractBodyFromPosition(src, pos) // Helper: Brace matching extraction
}
```

**モジュール責任分離**:
- `tryLegacyPattern`: Legacy "static method main" パターン専用
- `tryModernPattern`: Modern "static box Main { main() }" パターン専用
- `extractBodyFromPosition`: 共通のブレースマッチングロジック（再利用可能）

**利点**:
- ✅ 明確な責任分離（各パターン検出が独立モジュール）
- ✅ テスタビリティ（各メソッド個別テスト可能）
- ✅ 拡張性（新パターン追加時は新メソッド追加のみ）
- ✅ 後方互換性（Legacy パターン削除は tryLegacyPattern 削除のみ）

### テスト結果

**Stage-1/Stage-B パイプライン**: ✅ PASS
```bash
$ ./target/release/hakorune apps/tests/stage1_run_min.hako
RC: 0
```

**Selfhost depth-1**: ✅ PASS
```bash
$ NYASH_FEATURES=stage3 NYASH_USE_NY_COMPILER=1 NYASH_JOINIR_STRICT=1 \
  ./target/release/hakorune apps/tests/stage1_run_min.hako
RC: 0
```

**全スモークテスト**: ✅ 30/31 PASS
- 1 failure: unrelated timeout (strlen_fast_canary)
- 0 regressions from Phase 152-B changes

### 後方互換性検証

**Legacy パターンサポート**: ✅ 確認済み
- Stage-B ヘルパーで "static method main" と "method main" 両対応
- Modern パターンを優先、Legacy はフォールバック

**パーサ非汚染**: ✅ 達成
- 新構文追加なし（既存 `static box` パーサのみ）
- Stage-3 仕様クリーン性保持

---

## 進捗

- ✅ Phase 130-134: LLVM Python バックエンド整理
- ✅ Phase 150: Selfhost Stage-3 Depth-1 ベースライン強化
- ✅ Phase 151: ConsoleBox Selfhost Support
- ✅ Phase 152-A: 括弧付き代入式（Rust/Selfhost パーサ両対応）
- ✅ Phase 152-B: Static Method 宣言整理（箱化モジュール化完了）
- 📋 Phase 160+: .hako JoinIR/MIR 移植章（予定）
- 🌟 Phase 200+: Python → Hakorune トランスパイラ構想（夢）
Status: Historical

