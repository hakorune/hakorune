Status: Historical

# Hakorune コードベース重複・共通化調査 - 実行サマリー

**調査日**: 2025-11-06
**調査実施**: Claude Code Agent

---

## クイックサマリー

### 主要発見
- **重複パターン総数**: 260箇所以上
- **削減可能行数**: 500-800行（全体の15-20%）
- **即効対応可能**: Phase 1で270-380行削減（実装時間5-8時間）

### 優先度トップ3
1. **Destination書き込み**: 49箇所 → ヘルパー関数化
2. **引数検証**: 55箇所 → 統一検証関数
3. **エラー生成**: 95箇所 → エラービルダー

---

## ドキュメント構成

### 1. 詳細分析レポート
**ファイル**: [`DUPLICATION_ANALYSIS_REPORT.md`](./DUPLICATION_ANALYSIS_REPORT.md)

**内容**:
- 9章構成の包括的レポート
- 全重複パターンの詳細分析
- 3段階のアクションプラン（Phase 1-3）
- リスク評価と期待効果

**主要セクション**:
1. MIR Interpreter Handlers の重複パターン（5種類）
2. MIR Builder の重複パターン（PHI挿入など）
3. 統合可能なモジュール（Box Handler群）
4. 優先度付きアクションプラン
5. 実装ガイドライン
6. 期待される効果（定量・定性）
7. リスク評価
8. 追加調査項目
9. 結論と推奨事項

### 2. Phase 1実装ガイド
**ファイル**: [`PHASE1_IMPLEMENTATION_GUIDE.md`](./PHASE1_IMPLEMENTATION_GUIDE.md)

**内容**:
- Step-by-Step実装手順
- コード例（Before/After）
- テスト戦略
- トラブルシューティング

**実装項目**:
1. `utils/register_ops.rs` - Destination書き込みヘルパー
2. `utils/validation.rs` - 引数検証ヘルパー
3. `utils/conversions.rs` - Receiver変換ヘルパー

---

## 数値で見る重複の実態

### パターン別重複数

| パターン | 箇所数 | 削減見込み行数 | 優先度 |
|---------|--------|--------------|--------|
| Destination書き込み | 49 | 150-200 | 最高 ⭐⭐⭐ |
| 引数検証 | 55 | 100-150 | 高 ⭐⭐ |
| エラー生成 | 95 | 200-300 | 最高 ⭐⭐⭐ |
| Receiver変換 | 5 | 20-30 | 高 ⭐⭐ |
| PHI挿入 | 13 | 50-100 | 中 ⭐ |
| Box downcast | 57 | 将来検討 | 低 |
| **合計** | **274** | **520-780** | - |

### ファイル別行数（上位10）

```
   907行  handlers/calls.rs              ← 最大ファイル
   399行  handlers/boxes_object_fields.rs
   307行  handlers/boxes.rs
   298行  handlers/extern_provider.rs
   218行  handlers/externals.rs
   217行  handlers/boxes_plugin.rs
   208行  handlers/boxes_string.rs
   153行  handlers/boxes_instance.rs
   136行  handlers/arithmetic.rs
   134行  handlers/boxes_map.rs
-------
 3,335行  handlers/ 合計
```

---

## アクションプラン概要

### Phase 1: 即効対応（推奨: 即時実施）
**目標**: 低リスク・高効果のヘルパー関数実装
**期間**: 5-8時間
**削減**: 270-380行

**実装項目**:
1. Destination書き込みヘルパー
   ```rust
   // Before (4行)
   if let Some(d) = dst {
       this.regs.insert(d, VMValue::from_nyash_box(ret));
   }

   // After (1行)
   this.write_box_result(dst, ret);
   ```

2. 引数検証ヘルパー
   ```rust
   // Before (3行)
   if args.len() != 1 {
       return Err(VMError::InvalidInstruction("push expects 1 arg".into()));
   }

   // After (1行)
   this.validate_args_exact("push", args, 1)?;
   ```

3. Receiver変換ヘルパー
   ```rust
   // Before (4行)
   let recv_box = match recv.clone() {
       VMValue::BoxRef(b) => b.share_box(),
       other => other.to_nyash_box(),
   };

   // After (1行)
   let recv_box = this.convert_to_box(&recv);
   ```

### Phase 2: 基盤整備（Phase 1完了後）
**目標**: エラー処理とPHI挿入の統一
**期間**: 5-7時間
**削減**: 250-400行

**実装項目**:
- エラー生成ヘルパー（95箇所）
- PHI挿入ヘルパー（13箇所）

### Phase 3: 抜本改革（将来課題）
**目標**: Box Handler群の統合
**期間**: 1-2週間
**削減**: 300-400行
**リスク**: 中-高

**備考**: Phase 1-2の効果を見てから判断

---

## 期待される効果

### 定量的効果

| 指標 | 現状 | Phase 1後 | Phase 2後 | Phase 3後 |
|-----|------|----------|----------|----------|
| Handler総行数 | 3,335 | 2,965 (-11%) | 2,715 (-19%) | 2,415 (-28%) |
| 重複パターン | 260箇所 | 109箇所 (-58%) | 46箇所 (-82%) | <20箇所 (-92%) |
| 保守対象箇所 | 散在 | 3ヶ所集約 | 5ヶ所集約 | 統一API |

### 定性的効果

✅ **保守性向上**: ロジック変更が1箇所で完結
✅ **可読性向上**: 意図が明確な関数名
✅ **バグ削減**: 共通ロジックのバグ修正が全体に波及
✅ **開発速度向上**: 新機能追加時のボイラープレート削減
✅ **テスト容易性**: ユーティリティ関数の単体テストで広範囲カバー
✅ **一貫性**: エラーメッセージの統一

---

## リスクと対策

### Phase 1リスク: 極めて低

| リスク | 確率 | 影響 | 対策 |
|-------|------|------|------|
| ヘルパー関数のバグ | 低 | 高 | 単体テスト、段階的移行 |
| パフォーマンス劣化 | 極低 | 低 | インライン化、ベンチマーク |
| 既存動作の変更 | 低 | 高 | 1ファイルずつ、回帰テスト |

### 推奨移行戦略

1. **テストファースト**: ユーティリティ関数のテストを先に書く
2. **並行期間**: 新旧コードを共存させる
3. **段階的移行**: 1ファイルずつ確実に
4. **回帰テスト**: 各ファイル移行後にスモークテスト実行

---

## 実装の流れ（Phase 1）

### Week 1: インフラ構築（1-2時間）
```
1. utils/ ディレクトリ作成
2. register_ops.rs, validation.rs, conversions.rs 実装
3. ユニットテスト追加
4. 全テスト確認
```

### Week 1-2: Handler更新（3-5時間）
```
小さいファイルから順番に：
1. boxes_array.rs (63行 → 50行)
2. boxes_map.rs (134行 → 110行)
3. boxes_string.rs (208行 → 170行)
4. boxes_plugin.rs (217行 → 180行)
5. boxes_instance.rs (153行 → 125行)
6. boxes_object_fields.rs (399行 → 330行)
7. boxes.rs (307行 → 250行)
8. calls.rs (907行 → 750行)

各ファイル更新後:
- コンパイル確認
- テスト実行
- コミット（小さなPR）
```

### Week 2: 検証・ドキュメント（1時間）
```
1. 全ハンドラーで古いパターン検索
2. スモークテスト実行
3. ドキュメント更新
4. Phase 2計画開始
```

---

## コード例：Before/After

### Example 1: ArrayBox.push

**Before** (6行):
```rust
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
```

**After** (4行, **33%削減**):
```rust
"push" => {
    this.validate_args_exact("push", args, 1)?;
    let val = this.reg_load(args[0])?.to_nyash_box();
    let _ = ab.push(val);
    this.write_void(dst);
    return Ok(true);
}
```

### Example 2: StringBox.indexOf

**Before** (19行):
```rust
"indexOf" => {
    let (needle, from_index) = match args.len() {
        1 => {
            let n = this.reg_load(args[0])?.to_string();
            (n, 0)
        }
        2 => {
            let n = this.reg_load(args[0])?.to_string();
            let from = this.reg_load(args[1])?.as_integer().unwrap_or(0);
            (n, from.max(0) as usize)
        }
        _ => {
            return Err(VMError::InvalidInstruction(
                "indexOf expects 1 or 2 args (search [, fromIndex])".into(),
            ));
        }
    };
    // ... implementation
    if let Some(d) = dst {
        this.regs.insert(d, VMValue::Integer(idx));
    }
    return Ok(true);
}
```

**After** (14行, **26%削減**):
```rust
"indexOf" => {
    this.validate_args_range("indexOf", args, 1, 2)?;

    let needle = this.reg_load(args[0])?.to_string();
    let from_index = if args.len() >= 2 {
        this.reg_load(args[1])?.as_integer().unwrap_or(0).max(0) as usize
    } else {
        0
    };

    // ... implementation

    this.write_integer(dst, idx);
    return Ok(true);
}
```

---

## 推奨事項

### 🚀 即時実施推奨
**Phase 1の実装**: 低リスク・高効果・短時間で完了可能

### 理由
1. **投資対効果が極めて高い**: 5-8時間で270-380行削減
2. **リスクが低い**: 既存パターンの単純な抽出
3. **即効性がある**: 実装後すぐに効果が現れる
4. **基盤になる**: Phase 2-3の土台

### 実施手順
1. [`PHASE1_IMPLEMENTATION_GUIDE.md`](./PHASE1_IMPLEMENTATION_GUIDE.md) を読む
2. Step 1-3でユーティリティ関数を実装（1-2時間）
3. Step 6で1ファイルずつ更新（3-5時間）
4. 完了後、効果測定とPhase 2計画

---

## 参考資料

### 調査で作成したドキュメント
1. **詳細分析レポート**: [`DUPLICATION_ANALYSIS_REPORT.md`](./DUPLICATION_ANALYSIS_REPORT.md)
   - 全重複パターンの詳細分析
   - リスク評価と期待効果
   - 付録（ファイルサイズ一覧など）

2. **Phase 1実装ガイド**: [`PHASE1_IMPLEMENTATION_GUIDE.md`](./PHASE1_IMPLEMENTATION_GUIDE.md)
   - コピペで使えるコード例
   - トラブルシューティング
   - テスト戦略

3. **このサマリー**: [`CLEANUP_SUMMARY_2025-11-06.md`](./CLEANUP_SUMMARY_2025-11-06.md)
   - 全体像の把握
   - クイックリファレンス

### 調査実行コマンド
```bash
# 重複パターン確認
grep -rn "if let Some(d) = dst { this.regs.insert" src/backend/mir_interpreter/handlers/
grep -rn "args.len()" src/backend/mir_interpreter/handlers/
grep -rn "match recv.clone()" src/backend/mir_interpreter/handlers/

# ファイルサイズ確認
wc -l src/backend/mir_interpreter/handlers/*.rs | sort -rn

# Phase 1実装後の確認
git diff --stat  # 変更行数確認
./tools/jit_smoke.sh  # 回帰テスト
```

---

## Q&A

### Q1: なぜPhase 1を優先？
**A**: 低リスク・高効果・短時間で、Phase 2-3の土台になるため。

### Q2: 既存テストは通る？
**A**: はい。ヘルパー関数は既存ロジックの抽出なので、動作は同一です。

### Q3: パフォーマンスへの影響は？
**A**: ほぼゼロ。コンパイラの最適化でインライン化されます。

### Q4: Phase 2-3はいつ実施？
**A**: Phase 1完了後、効果を測定してから判断します。

### Q5: 他のメンバーへの影響は？
**A**: 最小限。小さなPRに分割し、1ファイルずつレビューします。

---

## 次のステップ

1. ✅ **調査完了** - このドキュメント
2. ⏭️ **Phase 1実装開始** - [`PHASE1_IMPLEMENTATION_GUIDE.md`](./PHASE1_IMPLEMENTATION_GUIDE.md) 参照
3. ⏳ **Phase 2計画** - Phase 1完了後
4. ⏳ **Phase 3検討** - Phase 1-2の効果測定後

---

**作成者**: Claude Code Agent
**最終更新**: 2025-11-06
**次回更新**: Phase 1完了時
