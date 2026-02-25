Status: Historical

# レガシーコード削除プロジェクト - エグゼクティブサマリー

**調査日**: 2025-11-06
**調査対象**: Phase 21.0に向けたレガシーコード削除可能性

---

## 📊 調査結果サマリー

### 総削減見込み: **約9,500行** (全体の約10%)

| フェーズ | 削減行数 | リスク | 実施時期 | 状態 |
|---------|---------|-------|---------|-----|
| Phase A | 3,900行 | 無し | 今すぐ | ✅ 準備完了 |
| Phase B | 2,200行 | 低 | 1週間 | 📋 調査必要 |
| Phase C | 3,400行 | 中 | Phase 16 | ⏳ 戦略待ち |

---

## 🎯 推奨アクション

### 即実行推奨 (Phase A)
**今すぐ削除可能**: 約3,900行 (リスク無し)

1. **Cranelift/JIT削除** (1,500行)
   - Phase 15でアーカイブ済み
   - feature削除済み、ビルドエラーなし

2. **BID Copilotアーカイブ** (1,900行)
   - 未使用プロトタイプ
   - READMEで「現在は使用していない」と明記

3. **Dead Code削除** (500行)
   - `#[allow(dead_code)]`マーカー付き
   - 明確に未使用な関数

**実行時間**: 約10分
**手順**: [クイック削除ガイド](./QUICK_CLEANUP_GUIDE.md)

---

### 調査後実行 (Phase B)
**1週間以内**: 約2,200行 (低リスク)

1. **JSON v1 Bridge** (734行)
   - 使用状況確認が必要
   - 未使用なら削除 or アーカイブ

2. **Legacy Test Files** (1,000行)
   - cranelift依存テストの整理
   - VM/LLVM比較として有用性評価

3. **Parser Dead Code** (500行)
   - 実使用確認後削除

**実行時間**: 約1週間 (調査含む)

---

### 戦略確定後実行 (Phase C)
**Phase 16以降**: 約3,400行 (中リスク)

1. **WASM Backend** (3,170行)
   - Phase 21.0戦略次第
   - 動作確認後判断

2. **Builtin Box移行** (264行)
   - Phase 15.5-B完了後
   - プラグイン移行戦略に従う

---

## 📚 ドキュメント構成

### 1. [README.md](./README.md)
**役割**: 入口・ナビゲーション

### 2. [レガシーコード調査レポート](./LEGACY_CODE_INVESTIGATION_REPORT.md)
**役割**: 詳細調査結果・分析
**対象**: プロジェクトマネージャー、意思決定者

### 3. [クイック削除ガイド](./QUICK_CLEANUP_GUIDE.md)
**役割**: 実行マニュアル (コピペ可能)
**対象**: 開発者 (実行担当者)

### 4. [詳細ファイルリスト](./LEGACY_FILES_DETAILED_LIST.md)
**役割**: ファイル別完全リスト
**対象**: 開発者 (詳細確認用)

---

## 💡 削除する理由

### 1. Phase 20.46完了
開発の区切りが良く、次フェーズに集中できる

### 2. 保守性向上
約10%のコード削減で可読性・保守性が向上

### 3. AI協働効率化
レガシーコードがAIを混乱させる問題を解消

### 4. Phase 21.0準備
クリーンなコードベースで次フェーズ開始

---

## ⚠️ 削除しない理由 (Phase C)

### WASM Backend (3,170行)
- Phase 21.0でWASM対応の可能性
- 動作確認後に判断

### Builtin Box (264行)
- プラグイン移行戦略が確定していない
- 削除するとコードが大量に壊れる

### PyVM (保持)
- Phase 15セルフホスティングで現役使用中
- JSON v0ブリッジ・using処理で必須

---

## 📈 削減効果

### コードベース削減
- **Phase A**: 約3,900行 (約4%)
- **Phase B**: 約2,200行 (約2%)
- **Phase C**: 約3,400行 (約3.5%)
- **合計**: 約9,500行 (約10%)

### 保守性向上
- レガシーコード除去で可読性向上
- AI協働時の混乱減少
- ビルド時間微減

### リスク評価
- **Phase A**: リスク無し (feature削除済み)
- **Phase B**: 低リスク (調査後判断)
- **Phase C**: 中リスク (戦略依存)

---

## ✅ 次のアクション

### 1. Phase A実行 (今すぐ)
👉 [クイック削除ガイド](./QUICK_CLEANUP_GUIDE.md)

約10分で約3,900行削除可能

### 2. Phase B調査 (1週間以内)
👉 [詳細ファイルリスト](./LEGACY_FILES_DETAILED_LIST.md)

使用状況確認・削除判断

### 3. Phase C戦略確認 (Phase 16)
👉 [レガシーコード調査レポート](./LEGACY_CODE_INVESTIGATION_REPORT.md)

Phase 21.0戦略に従う

---

## 📝 コミットメッセージ例

```
chore: Remove legacy Cranelift/JIT code and BID Copilot prototype

- Remove Cranelift/JIT backend (archived in Phase 15)
  * src/runner/modes/cranelift.rs (46 lines)
  * src/runner/jit_direct.rs (~200 lines)
  * src/tests/core13_smoke_jit*.rs (2 files)
  * Related references in backend/mod.rs, cli/args.rs

- Archive BID Copilot modules (unused prototype)
  * src/bid-codegen-from-copilot/ → archive/
  * src/bid-converter-copilot/ → archive/

- Delete dead code
  * src/mir/builder/exprs_legacy.rs
  * Multiple #[allow(dead_code)] functions

Total reduction: ~3,900 lines (~4%)

Refs: docs/development/cleanup/LEGACY_CODE_INVESTIGATION_REPORT.md
```

---

**作成者**: Claude Code
**最終更新**: 2025-11-06
**推奨アクション**: Phase A実行 (今すぐ)
