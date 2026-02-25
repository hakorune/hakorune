Status: Historical

# レガシーコード削除プロジェクト

**調査日**: 2025-11-06
**Phase**: 20.46完了後
**目的**: Phase 21.0に向けたコードベース整理

---

## 📚 ドキュメント構成

### 1. [レガシーコード調査レポート](./LEGACY_CODE_INVESTIGATION_REPORT.md) ⭐ メインレポート
**内容**: 包括的な調査結果・削減見込み・段階的削除計画

- カテゴリ別詳細分析 (8カテゴリ)
- 削減見込みサマリー (約5,600〜8,900行)
- 段階的削除計画 (Phase A/B/C)
- リスク評価 (Safe / Investigate / Risky)

**読者**: プロジェクトマネージャー、意思決定者

---

### 2. [クイック削除ガイド](./QUICK_CLEANUP_GUIDE.md) ⚡ 実行マニュアル
**内容**: 今すぐ実行可能なSafe削除手順

- コピペ可能なコマンド
- 手動編集箇所の詳細
- ビルド確認手順
- トラブルシューティング

**読者**: 開発者 (実行担当者)

---

### 3. [詳細ファイルリスト](./LEGACY_FILES_DETAILED_LIST.md) 📋 リファレンス
**内容**: ファイル別・行数別の完全リスト

- Safe削除ファイル一覧 (25+ ファイル, ~3,900行)
- Investigateファイル一覧 (7+ ファイル, ~2,200行)
- Riskyファイル一覧 (18 ファイル, ~3,434行)
- 削除優先順位

**読者**: 開発者 (詳細確認用)

---

## 🎯 クイックスタート

### 今すぐ削除したい場合
👉 **[クイック削除ガイド](./QUICK_CLEANUP_GUIDE.md)** を参照

約10分で約3,900行削除可能 (リスク無し)

---

### 全体像を把握したい場合
👉 **[レガシーコード調査レポート](./LEGACY_CODE_INVESTIGATION_REPORT.md)** を参照

調査結果・削減見込み・段階的計画の全体像

---

### 個別ファイルを確認したい場合
👉 **[詳細ファイルリスト](./LEGACY_FILES_DETAILED_LIST.md)** を参照

ファイル別・カテゴリ別の完全リスト

---

## 📊 削減見込みサマリー

| フェーズ | 対象 | 削減行数 | リスク | 実施時期 |
|---------|-----|---------|-------|---------|
| **Phase A** | Safe削除 | 約3,900行 | 無し | 今すぐ |
| **Phase B** | Investigate | 約2,200行 | 低 | 1週間以内 |
| **Phase C** | Risky | 約3,400行 | 中 | Phase 16以降 |
| **合計** | - | **約9,500行** | - | - |

---

## 🚀 削除実行フロー

### Step 1: Safe削除 (今すぐ)
```bash
# クイック削除ガイドを参照
cd /home/tomoaki/git/hakorune-selfhost
cat docs/development/cleanup/QUICK_CLEANUP_GUIDE.md

# コマンドをコピペして実行
# 約10分で完了
```

**削減**: 約3,900行
**リスク**: 無し

---

### Step 2: Investigate (1週間以内)

#### 2-1. JSON v1 Bridge調査
```bash
# 使用状況確認
grep -r "json_v1_bridge\|try_parse_v1_to_module" src --include="*.rs"
grep -r "schema_version.*1" apps --include="*.json"
```

**判断基準**: 使用されていない → 削除 or アーカイブ

---

#### 2-2. Legacy Tests整理
```bash
# cranelift依存テスト確認
find src/tests -name "*.rs" -exec grep -l "cfg.*cranelift" {} \;
```

**判断基準**: VM/LLVM比較として有用? → 分割 or 削除

---

#### 2-3. Parser Dead Code削除
```bash
# 各関数の使用状況確認
grep -r "unknown_span" src --include="*.rs" | grep -v "def\|fn "
```

**判断基準**: 未使用 → 削除

---

### Step 3: Risky (Phase 16以降)

#### 3-1. WASM Backend評価
```bash
# 動作確認
cargo build --release --features wasm-backend
./target/release/nyash --backend wasm test.hako
```

**判断基準**: 動作しない → アーカイブ

---

#### 3-2. Builtin Box移行
Phase 15.5-B完了後、プラグイン移行戦略に従う

**判断基準**: Plugin完成 → Builtin削除

---

## ✅ 実行チェックリスト

### Phase A: Safe削除
- [ ] Cranelift/JITファイル削除完了
- [ ] BID Copilotアーカイブ完了
- [ ] Dead Code削除完了
- [ ] `cargo build --release` 成功
- [ ] `cargo test` 全テストパス
- [ ] `./tools/smokes/v2/run.sh --profile quick` パス
- [ ] Git commit & push完了

**コミットメッセージ例**:
```
chore: Remove legacy Cranelift/JIT code and BID Copilot prototype

- Remove Cranelift/JIT backend (archived in Phase 15)
- Archive BID Copilot modules (unused prototype)
- Delete dead code (明確に未使用なコード)
- Total reduction: ~3,900 lines (~4%)

Refs: docs/development/cleanup/LEGACY_CODE_INVESTIGATION_REPORT.md
```

---

### Phase B: Investigate
- [ ] JSON v1 Bridge調査完了
- [ ] Legacy Tests整理完了
- [ ] Parser Dead Code確認完了
- [ ] 削除判断・実行完了
- [ ] テスト確認完了

---

### Phase C: Risky
- [ ] WASM Backend動作確認完了
- [ ] Phase 21.0戦略確認完了
- [ ] Builtin Box移行計画確定
- [ ] 段階的削除実施完了

---

## 📈 進捗トラッキング

### 現在の状態 (2025-11-06)
- ✅ 調査完了 (3ドキュメント作成)
- ⏳ Phase A実行待ち (約3,900行削減)
- ⏳ Phase B調査待ち (約2,200行削減)
- ⏳ Phase C戦略待ち (約3,400行削減)

### 目標
- Phase A: 2025-11-07までに完了
- Phase B: 2025-11-14までに完了
- Phase C: Phase 16開始時に実施

---

## 🔍 関連ドキュメント

### プロジェクト全体
- [CURRENT_TASK.md](../../../CURRENT_TASK.md) - 現在のタスク
- [00_MASTER_ROADMAP.md](../roadmap/phases/00_MASTER_ROADMAP.md) - マスタープラン

### 過去の削除実績
- [CLEANUP_REPORT_2025-11-04.md](./CLEANUP_REPORT_2025-11-04.md) - 前回の削除実績
- [PHASE2_REPORT_2025-11-04.md](./PHASE2_REPORT_2025-11-04.md) - Phase 2削除

---

## 📝 備考

### なぜ今削除するのか?
1. **Phase 20.46完了**: 開発の区切りが良い
2. **Phase 21.0準備**: 次フェーズに集中するため
3. **保守性向上**: レガシーコード除去で可読性向上
4. **AI協働効率化**: 不要コードがAIを混乱させる

### なぜ段階的なのか?
1. **リスク管理**: 一度に削除すると問題発見が困難
2. **テスト実施**: 各段階で動作確認
3. **ロールバック容易性**: 問題があれば即座に戻せる

### Archive vs 削除
- **Archive推奨**: 将来的に再利用可能性あり
- **完全削除**: 明確に不要・復活の可能性なし

---

## 🆘 問題が発生した場合

### ビルドエラー
```bash
# 変更を戻す
git restore .
git clean -fd

# 詳細レポート確認
cat docs/development/cleanup/LEGACY_CODE_INVESTIGATION_REPORT.md
```

### テスト失敗
1. 削除したファイルが実際に使用されている
2. 詳細リストの「Investigate」セクションを確認
3. 必要に応じてファイル復元

### 質問・相談
- Claude Code に相談
- ChatGPT に実装依頼
- ドキュメントを再確認

---

**作成者**: Claude Code
**最終更新**: 2025-11-06
**次のアクション**: Phase A実行推奨 (クイック削除ガイド参照)
