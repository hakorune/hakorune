Status: Historical

# コードベース重複・共通化調査 - インデックス

Hakorune Rustコードベースにおける重複コードの特定と、DRY原則に基づく改善計画のドキュメントハブです。

**調査日**: 2025-11-06
**実施**: Claude Code Agent
**調査種別**: 重複パターン分析・共通化機会の特定

---

## 📚 ドキュメント一覧

### 🎯 まず読むべきドキュメント

1. **[クイックリファレンス](./QUICK_REFERENCE.md)** ⭐ おすすめ
   - 1ページで全体像を把握
   - Before/After コード例
   - 今すぐ始められる実装手順

2. **[実行サマリー](./CLEANUP_SUMMARY_2025-11-06.md)**
   - エグゼクティブサマリー
   - 数値で見る重複の実態
   - Q&A

### 📖 詳細ドキュメント

3. **[詳細分析レポート](./DUPLICATION_ANALYSIS_REPORT.md)**
   - 9章構成の包括的分析
   - 全重複パターンの詳細
   - リスク評価と期待効果
   - 付録（ファイルサイズ一覧）

4. **[Phase 1実装ガイド](./PHASE1_IMPLEMENTATION_GUIDE.md)**
   - Step-by-Step実装手順
   - コピペで使えるコード例
   - テスト戦略
   - トラブルシューティング

---

## 🎯 調査結果サマリー

### 重複の規模
- **総重複箇所**: 260箇所以上
- **削減可能行数**: 500-800行（15-20%削減）
- **優先対応**: Phase 1で270-380行削減可能

### 主要な重複パターン（Top 5）

| パターン | 箇所数 | 削減見込み | 優先度 |
|---------|--------|----------|--------|
| Destination書き込み | 49 | 150-200行 | ⭐⭐⭐ 最高 |
| 引数検証 | 55 | 100-150行 | ⭐⭐ 高 |
| エラー生成 | 95 | 200-300行 | ⭐⭐⭐ 最高 |
| Receiver変換 | 5 | 20-30行 | ⭐⭐ 高 |
| PHI挿入 | 13 | 50-100行 | ⭐ 中 |

---

## 🚀 アクションプラン

### Phase 1: 即効対応（推奨: 即時実施）
**目標**: 低リスク・高効果のヘルパー関数実装
- **期間**: 5-8時間
- **削減**: 270-380行
- **リスク**: 低
- **詳細**: [Phase 1実装ガイド](./PHASE1_IMPLEMENTATION_GUIDE.md)

**実装内容**:
1. Destination書き込みヘルパー（`write_result`, `write_void` など）
2. 引数検証ヘルパー（`validate_args_exact`, `validate_args_range`）
3. Receiver変換ヘルパー（`convert_to_box`）

### Phase 2: 基盤整備（Phase 1完了後）
**目標**: エラー処理とPHI挿入の統一
- **期間**: 5-7時間
- **削減**: 250-400行
- **リスク**: 低

### Phase 3: 抜本改革（将来課題）
**目標**: Box Handler群の統合
- **期間**: 1-2週間
- **削減**: 300-400行
- **リスク**: 中-高

---

## 📈 期待される効果

### 定量的効果

```
現状: 3,335行（Handlers）
  ↓ Phase 1
2,965行 (-11%, -370行)
  ↓ Phase 2
2,715行 (-19%, -620行)
  ↓ Phase 3
2,415行 (-28%, -920行)
```

### 定性的効果
- ✅ 保守性向上（変更が1箇所で完結）
- ✅ 可読性向上（意図が明確な関数名）
- ✅ バグ削減（共通ロジックの一元管理）
- ✅ 開発速度向上（ボイラープレート削減）
- ✅ テスト容易性（単体テストで広範囲カバー）

---

## 📂 対象ファイル

### MIR Interpreter Handlers（重複集中エリア）
```
src/backend/mir_interpreter/handlers/
├── arithmetic.rs (136行)
├── boxes.rs (307行) ← 重複多
├── boxes_array.rs (63行) ← 重複多
├── boxes_instance.rs (153行) ← 重複多
├── boxes_map.rs (134行) ← 重複多
├── boxes_object_fields.rs (399行) ← 重複多
├── boxes_plugin.rs (217行) ← 重複多
├── boxes_string.rs (208行) ← 重複多
├── boxes_void_guards.rs (21行)
├── call_resolution.rs (89行)
├── calls.rs (907行) ← 最大ファイル
├── extern_provider.rs (298行)
├── externals.rs (218行)
├── memory.rs (47行)
├── misc.rs (31行)
└── mod.rs (107行)

合計: 3,335行
```

### 新規作成ファイル（Phase 1）
```
src/backend/mir_interpreter/utils/
├── mod.rs
├── register_ops.rs        # Destination書き込み
├── validation.rs          # 引数検証
└── conversions.rs         # 型変換
```

---

## 💡 実装のヒント

### Before/After 例

#### 例1: ArrayBox.push メソッド
```rust
// ❌ Before (6行)
"push" => {
    if args.len() != 1 {
        return Err(VMError::InvalidInstruction("push expects 1 arg".into()));
    }
    let val = this.reg_load(args[0])?.to_nyash_box();
    let _ = ab.push(val);
    if let Some(d) = dst {
        this.regs.insert(d, VMValue::Void);
    }
    return Ok(true);
}

// ✅ After (4行, 33%削減)
"push" => {
    this.validate_args_exact("push", args, 1)?;
    let val = this.reg_load(args[0])?.to_nyash_box();
    let _ = ab.push(val);
    this.write_void(dst);
    return Ok(true);
}
```

### 実装の流れ
1. ユーティリティ関数実装（1-2時間）
2. 最小ファイルで試す（30分）
3. 残りのファイルを順次更新（3-5時間）
4. 検証＆ドキュメント更新（1時間）

---

## 🔍 確認コマンド

```bash
# 重複パターン検索
cd /home/tomoaki/git/hakorune-selfhost
grep -rn "if let Some(d) = dst { this.regs.insert" src/backend/mir_interpreter/handlers/
grep -rn "args.len() !=" src/backend/mir_interpreter/handlers/
grep -rn "match recv.clone()" src/backend/mir_interpreter/handlers/

# ファイルサイズ確認
wc -l src/backend/mir_interpreter/handlers/*.rs | sort -rn

# Phase 1実装後の検証
./tools/jit_smoke.sh
git diff --stat
```

---

## 📊 進捗管理

### Phase 1チェックリスト

#### インフラ構築
- [ ] `utils/` ディレクトリ作成
- [ ] `register_ops.rs` 実装
- [ ] `validation.rs` 実装
- [ ] `conversions.rs` 実装
- [ ] ユニットテスト追加
- [ ] コンパイル＆テスト確認

#### Handler更新（1ファイルずつ）
- [ ] `boxes_array.rs` (63行 → 50行)
- [ ] `boxes_map.rs` (134行 → 110行)
- [ ] `boxes_string.rs` (208行 → 170行)
- [ ] `boxes_plugin.rs` (217行 → 180行)
- [ ] `boxes_instance.rs` (153行 → 125行)
- [ ] `boxes_object_fields.rs` (399行 → 330行)
- [ ] `boxes.rs` (307行 → 250行)
- [ ] `calls.rs` (907行 → 750行)

#### 最終検証
- [ ] 重複パターン残存確認
- [ ] スモークテスト実行
- [ ] ドキュメント更新
- [ ] Phase 2計画

---

## 🎓 学んだこと

### 重複パターンの発見方法
1. 同じようなコードブロックを探す（grep活用）
2. 関数シグネチャの類似性をチェック
3. エラーメッセージパターンを調査
4. ファイルサイズ比較（大きいファイルは重複の宝庫）

### 効果的な共通化のポイント
1. **小さく始める**: 最も単純なパターンから
2. **段階的移行**: 一度に全部変えない
3. **テストファースト**: ヘルパー関数のテストを先に
4. **1ファイルずつ**: 都度テストして確実に

### リスク管理
1. **並行期間**: 新旧コードを共存させる
2. **回帰テスト**: 各変更後にスモークテスト
3. **小さなPR**: レビュー負荷を軽減
4. **ロールバック可能**: いつでも戻せる設計

---

## 📞 サポート

### 質問・相談
- **実装で困ったら**: [Phase 1実装ガイド](./PHASE1_IMPLEMENTATION_GUIDE.md) のトラブルシューティングを参照
- **全体像が知りたい**: [実行サマリー](./CLEANUP_SUMMARY_2025-11-06.md) を参照
- **詳細な分析結果**: [詳細分析レポート](./DUPLICATION_ANALYSIS_REPORT.md) を参照

### 次のステップ
1. [クイックリファレンス](./QUICK_REFERENCE.md) を読む（5分）
2. [Phase 1実装ガイド](./PHASE1_IMPLEMENTATION_GUIDE.md) に従って実装開始（5-8時間）
3. Phase 1完了後、Phase 2を計画

---

## 📅 履歴

- **2025-11-06**: 初版作成（Claude Code Agent による調査完了）
  - 重複パターン260箇所を特定
  - Phase 1-3のアクションプラン策定
  - 実装ガイド作成

---

## 📌 クイックリンク

### 今すぐ始める
1. 📖 [クイックリファレンス](./QUICK_REFERENCE.md) - 1ページで全体把握
2. 🔧 [Phase 1実装ガイド](./PHASE1_IMPLEMENTATION_GUIDE.md) - コピペで実装開始
3. ✅ チェックリスト（上記の「Phase 1チェックリスト」）

### 詳細を知る
4. 📊 [実行サマリー](./CLEANUP_SUMMARY_2025-11-06.md) - 数値と効果
5. 📚 [詳細分析レポート](./DUPLICATION_ANALYSIS_REPORT.md) - 包括的分析

### 関連リソース（別プロジェクト）
- [README.md](./README.md) - レガシーコード削除プロジェクト
- [Phase 2レポート](./PHASE2_REPORT_2025-11-04.md) - 既存の整理作業

---

**次のアクション**: [クイックリファレンス](./QUICK_REFERENCE.md) を読んで、Phase 1実装を開始しましょう！🚀

**注意**: このドキュメントは「重複コード・共通化」に関する調査です。「レガシーコード削除」については [README.md](./README.md) を参照してください。
