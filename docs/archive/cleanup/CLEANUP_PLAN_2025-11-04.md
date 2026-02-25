Status: Historical

# 🧹 プロジェクト大掃除計画 2025-11-04

**作成日**: 2025-11-04
**作成者**: Claude Code
**対象**: プロジェクトルート + docsフォルダ

---

## 📊 現状分析サマリー

### 🚨 深刻な問題
- **プロジェクトルート**: 55個の不要バイナリファイル（100MB以上）
- **docs/トップレベル**: 12個のリダイレクト専用ファイル（検索ノイズ）
- **重複ドキュメント**: CURRENT_TASK系3ファイル、CODEX_QUESTION系2ファイル

### 📈 統計
```
プロジェクトルート不要ファイル: 70+個
docs/ Markdownファイル総数: 1,632個
docs/ サイズ: 35MB
  ├── private/: 21MB (適切)
  ├── archive/: 8.9MB (適切)
  └── development/: 4.6MB (適切)
```

---

## 🎯 お掃除計画（3段階）

---

## 🔴 Phase 1: 即削除（安全確認済み）

### 1-A. バイナリファイル削除（55個）

**削除対象**:
```bash
./app*                      # 55個のビルド成果物
./__mir_builder_out.o       # オブジェクトファイル
```

**削除コマンド**:
```bash
# 安全確認（正式な実行ファイルがあることを確認）
ls -lh target/release/nyash target/release/hakorune

# 削除実行
rm -f ./app* ./__mir_builder_out.o

# 確認
ls -1 . | grep -E '^app|\.o$' | wc -l  # → 0になるはず
```

**削減効果**: 約100MB削減

**リスク**: なし（cargo buildで再生成可能）

---

### 1-B. 一時commitメッセージファイル削除

**削除対象**:
```bash
./commit_message.txt
./commit_message2.txt
```

**削除コマンド**:
```bash
rm -f ./commit_message.txt ./commit_message2.txt
```

**削減効果**: 数KB

**リスク**: なし（git履歴に残っている）

---

### 1-C. docs/リダイレクト専用ファイル削除（11個）

**削除対象**: すべて「Moved: ...」のみのファイル
```
docs/CONTRIBUTING-MERGE.md
docs/DEV_QUICKSTART.md
docs/EXTERNCALL.md
docs/LLVM_HARNESS.md
docs/PLUGIN_ABI.md
docs/VM_README.md
docs/CURRENT_TASK.md
docs/DOCUMENTATION_REORGANIZATION_PLAN.md
docs/REORGANIZATION_REPORT.md
docs/execution-backends.md
docs/refactor-roadmap.md
```

**事前確認（重要！）**:
```bash
# これらへのリンクがないか確認
for file in CONTRIBUTING-MERGE DEV_QUICKSTART EXTERNCALL LLVM_HARNESS PLUGIN_ABI VM_README CURRENT_TASK DOCUMENTATION_REORGANIZATION_PLAN REORGANIZATION_REPORT execution-backends refactor-roadmap; do
  echo "=== Checking docs/$file.md ==="
  grep -r "docs/$file\.md" . --include="*.md" 2>/dev/null | grep -v "^docs/$file.md:" || echo "  No references found"
done
```

**削除コマンド**:
```bash
cd docs/
rm -f CONTRIBUTING-MERGE.md DEV_QUICKSTART.md EXTERNCALL.md LLVM_HARNESS.md \
      PLUGIN_ABI.md VM_README.md CURRENT_TASK.md \
      DOCUMENTATION_REORGANIZATION_PLAN.md REORGANIZATION_REPORT.md \
      execution-backends.md refactor-roadmap.md
cd ..
```

**削減効果**: ノイズ削減（検索結果がクリーンに）

**リスク**: 低（リンク確認済みなら安全）

---

## 🟡 Phase 2: 整理・統合（要判断）

### 2-A. CURRENT_TASK系の整理

**現状**:
```
./CURRENT_TASK.md                           ← 最新（保持）
./CURRENT_TASK_ARCHIVE_2025-09-27.md       ← アーカイブ（移動）
./CURRENT_TASK_restored.md                 ← 古いバックアップ（削除）
docs/development/current_task_archive/CURRENT_TASK_2025-09-27.md  ← 重複
```

**推奨アクション**:
```bash
# 1. restored版を削除（古いバックアップ）
rm -f ./CURRENT_TASK_restored.md

# 2. アーカイブ版をdocs/development/archive/に統一
mv ./CURRENT_TASK_ARCHIVE_2025-09-27.md \
   docs/development/archive/current_task/CURRENT_TASK_ARCHIVE_2025-09-27.md

# 3. 重複チェック
ls -lh docs/development/current_task_archive/CURRENT_TASK_2025-09-27.md \
       docs/development/archive/current_task/CURRENT_TASK_2025-09-27.md
# → 重複なら片方削除
```

---

### 2-B. CODEX_QUESTION系の整理

**現状**:
```
./CODEX_QUESTION.md          ← 最新（保持）
./CODEX_QUESTION_backup.md   ← バックアップ（削除推奨）
```

**推奨アクション**:
```bash
# バックアップ版を削除
rm -f ./CODEX_QUESTION_backup.md
```

**理由**: git履歴があるのでバックアップ不要

---

### 2-C. 古いレポートの移動

**移動対象**:
```
./REFACTORING_ANALYSIS_REPORT.md
./analysis_report.md
```

**推奨アクション**:
```bash
# docs/archive/reports/に移動
mkdir -p docs/archive/reports/
mv ./REFACTORING_ANALYSIS_REPORT.md ./analysis_report.md docs/archive/reports/

# READMEに記録
cat >> docs/archive/reports/README.md <<'EOF'
# Archived Reports

- REFACTORING_ANALYSIS_REPORT.md: 古いリファクタリング分析（2025-09前）
- analysis_report.md: 古い分析レポート（2025-09前）

これらは歴史的記録として保持。最新の分析は docs/development/ を参照。
EOF
```

---

## 🟢 Phase 3: 検討・要確認

### 3-A. AGENTS.md の扱い

**現状**: 508行、Codex用の人格定義＋開発原則

**内容分析**:
- L1-14: Codex用人格設定（みらいちゃん設定）
- L15-508: 開発原則・構造設計指針（普遍的内容）

**推奨アクション** (3択):

#### 選択肢A: 分割（推奨）
```bash
# 1. 開発原則部分を docs/development/philosophy/DEVELOPMENT_PRINCIPLES.md に抽出
# 2. AGENTS.md は人格設定のみに縮小（100行以下）
# 3. CLAUDE.md から DEVELOPMENT_PRINCIPLES.md へリンク
```

**メリット**: 検索性向上、開発原則が独立文書に

#### 選択肢B: 保持（現状維持）
```bash
# そのまま保持
```

**メリット**: Codex用設定が一箇所に集約

#### 選択肢C: 非表示化
```bash
# .claude/ に移動（Claude Code検索対象外）
mv AGENTS.md .claude/AGENTS.md
```

**メリット**: ルートがすっきり、Codexからは参照可能

**判断基準**: ユーザーに確認

---

### 3-B. CHANGELOG.md の扱い

**現状**: 28行、最終更新2025-09-11（Phase 15）

**内容**:
- 2025-09-06: Core-13 flip
- 2025-09-04: Phase 12.7完了
- 2025-09-03: ABI TypeBox統合
- 2025-09-11: Phase 15開始

**問題点**:
- Phase 20.38まで進んでいるのに更新なし
- 「Work in progress」のまま放置

**推奨アクション** (2択):

#### 選択肢A: 廃止してREADME.mdに統合
```bash
# 1. 重要マイルストーンのみREADME.mdに記載
# 2. CHANGELOG.mdを削除
# 3. 詳細はgit logとdocs/private/roadmap2/phases/で管理
```

**メリット**: メンテナンス負荷削減

#### 選択肢B: 自動生成化
```bash
# git logから自動生成するスクリプト作成
# tools/generate_changelog.sh
```

**メリット**: 正確性担保

**判断基準**: ユーザーに確認

---

### 3-C. paper_review_prompts.md の扱い

**現状**: 76行、Gemini/Codex向け論文レビュー用プロンプト集

**内容**:
- MIR13論文レビュー用プロンプト
- Nyash言語論文レビュー用プロンプト
- 統合的レビュー用タスク

**推奨アクション** (2択):

#### 選択肢A: docs/private/papers/に移動
```bash
mv paper_review_prompts.md docs/private/papers/REVIEW_PROMPTS.md
```

**メリット**: 論文関連が一箇所に集約

#### 選択肢B: 保持（現状維持）
```bash
# ルートに保持（頻繁に使うツールとして）
```

**メリット**: アクセスしやすい

**判断基準**: 使用頻度次第

---

## 📋 実行チェックリスト

### ✅ Phase 1（即実行可能）

```bash
# 1. バイナリファイル削除
[ ] 正式実行ファイル存在確認
    ls -lh target/release/nyash target/release/hakorune
[ ] 削除実行
    rm -f ./app* ./__mir_builder_out.o
[ ] 削除確認
    ls -1 . | grep -E '^app|\.o$' | wc -l  # → 0

# 2. 一時commitメッセージ削除
[ ] rm -f ./commit_message.txt ./commit_message2.txt

# 3. docs/リダイレクト削除
[ ] リンク確認実行（上記コマンド）
[ ] リンクなし確認後、削除実行
```

**削減効果**: 約100MB + ノイズ削減

---

### ⚠️ Phase 2（要判断）

```bash
# 1. CURRENT_TASK系整理
[ ] CURRENT_TASK_restored.md 削除確認
[ ] アーカイブ統一先確認
[ ] 実行

# 2. CODEX_QUESTION系整理
[ ] バックアップ削除確認
[ ] 実行

# 3. 古いレポート移動
[ ] 移動先フォルダ作成
[ ] README.md作成
[ ] 実行
```

---

### 🤔 Phase 3（ユーザー確認必要）

```bash
# 1. AGENTS.md
[ ] 選択肢を提示してユーザー確認
    A: 分割（推奨）
    B: 保持
    C: 非表示化

# 2. CHANGELOG.md
[ ] 選択肢を提示してユーザー確認
    A: 廃止＋README.md統合
    B: 自動生成化

# 3. paper_review_prompts.md
[ ] 選択肢を提示してユーザー確認
    A: docs/private/papers/に移動
    B: 保持
```

---

## 📊 期待効果

### 削減効果
- **容量削減**: 約100MB
- **ファイル削減**: 約80個
- **検索ノイズ削減**: リダイレクト11個削除

### 改善効果
- ルートディレクトリのクリーン化
- docs/検索結果の改善
- 重複ドキュメント解消
- アーカイブ構造の整理

---

## 🚨 リスク管理

### Phase 1（低リスク）
- バイナリは再生成可能
- リダイレクトはリンク確認済み
- git履歴で復元可能

### Phase 2（中リスク）
- アーカイブ移動前にバックアップ推奨
- 重複確認を慎重に

### Phase 3（要確認）
- ユーザー確認必須
- 誤削除防止のため慎重判断

---

## 📝 実行記録テンプレート

```bash
# 実行日時: YYYY-MM-DD HH:MM
# 実行者:

## Phase 1
- [ ] バイナリ削除完了 (削減: XXX MB)
- [ ] commit message削除完了
- [ ] docs/リダイレクト削除完了

## Phase 2
- [ ] CURRENT_TASK系整理完了
- [ ] CODEX_QUESTION系整理完了
- [ ] 古いレポート移動完了

## Phase 3
- [ ] AGENTS.md: [選択肢] 実行完了
- [ ] CHANGELOG.md: [選択肢] 実行完了
- [ ] paper_review_prompts.md: [選択肢] 実行完了

## 最終確認
- [ ] ビルド成功確認 (cargo build --release)
- [ ] テスト成功確認 (tools/smokes/v2/run.sh --profile quick)
- [ ] git status確認
- [ ] コミット作成
```

---

## 🎯 まとめ

この計画により：
- ✅ プロジェクトルートが大幅にクリーン化
- ✅ docs/検索性が向上
- ✅ 重複ドキュメント解消
- ✅ 約100MB容量削減

**推奨実行順序**: Phase 1 → Phase 2 → Phase 3（ユーザー確認後）

---

**次のステップ**: ユーザーに確認を取り、Phase 1から実行開始！
