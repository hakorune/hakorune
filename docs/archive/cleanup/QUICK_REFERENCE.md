Status: Historical

# 重複コード削減 - クイックリファレンスカード

## 📊 一目で分かる状況

```
現状: 3,335行（Handlers）
  ↓
Phase 1実装後: 2,965行 (-11%)
  ↓
Phase 2実装後: 2,715行 (-19%)
  ↓
Phase 3実装後: 2,415行 (-28%)
```

---

## 🎯 Phase 1: 今すぐできる改善（5-8時間）

### 3つのヘルパー関数

#### 1. Destination書き込み (49箇所 → 1行化)
```rust
// ❌ Before (3-4行)
if let Some(d) = dst {
    this.regs.insert(d, VMValue::from_nyash_box(ret));
}

// ✅ After (1行)
this.write_box_result(dst, ret);
```

#### 2. 引数検証 (55箇所 → 1行化)
```rust
// ❌ Before (3行)
if args.len() != 1 {
    return Err(VMError::InvalidInstruction("push expects 1 arg".into()));
}

// ✅ After (1行)
this.validate_args_exact("push", args, 1)?;
```

#### 3. Receiver変換 (5箇所 → 1行化)
```rust
// ❌ Before (4行)
let recv_box = match recv.clone() {
    VMValue::BoxRef(b) => b.share_box(),
    other => other.to_nyash_box(),
};

// ✅ After (1行)
let recv_box = this.convert_to_box(&recv);
```

---

## 📂 実装する新ファイル

```
src/backend/mir_interpreter/utils/
├── mod.rs                 # モジュール定義
├── register_ops.rs        # write_result, write_void, etc.
├── validation.rs          # validate_args_exact, validate_args_range
└── conversions.rs         # convert_to_box
```

---

## ✅ 実装チェックリスト

### ステップ1: インフラ (1-2時間)
- [ ] `utils/` ディレクトリ作成
- [ ] 3つのファイル実装（register_ops, validation, conversions）
- [ ] ユニットテスト追加
- [ ] コンパイル＆テスト確認

### ステップ2: Handler更新 (3-5時間)
小→大の順で1ファイルずつ：
- [ ] `boxes_array.rs` (63行 → 50行)
- [ ] `boxes_map.rs` (134行 → 110行)
- [ ] `boxes_string.rs` (208行 → 170行)
- [ ] `boxes_plugin.rs` (217行 → 180行)
- [ ] `boxes_instance.rs` (153行 → 125行)
- [ ] `boxes_object_fields.rs` (399行 → 330行)
- [ ] `boxes.rs` (307行 → 250行)
- [ ] `calls.rs` (907行 → 750行)

各更新後: コンパイル → テスト → コミット

### ステップ3: 検証 (1時間)
- [ ] 古いパターン残存確認
- [ ] スモークテスト実行
- [ ] ドキュメント更新

---

## 🔍 確認コマンド

```bash
# 重複パターン検索
grep -rn "if let Some(d) = dst { this.regs.insert" src/backend/mir_interpreter/handlers/
grep -rn "args.len() !=" src/backend/mir_interpreter/handlers/
grep -rn "match recv.clone()" src/backend/mir_interpreter/handlers/

# Phase 1実装後（残存チェック）
grep -rn "if let Some(d) = dst { this.regs.insert" src/backend/mir_interpreter/handlers/ | wc -l
# → 0になるはず

# テスト実行
./tools/jit_smoke.sh

# 変更行数確認
git diff --stat
```

---

## 📈 期待される効果

| 指標 | Before | After | 改善 |
|-----|--------|-------|------|
| 行数 | 3,335 | 2,965 | -11% |
| 重複箇所 | 260 | 109 | -58% |
| 保守対象 | 散在 | 3ヶ所 | 集約 |

---

## 💡 実装のコツ

1. **最小ファイルから**: `boxes_array.rs` (63行) がおすすめ
2. **1ファイルずつ**: 都度テストして確実に
3. **コピペOK**: 実装ガイドのコードをそのまま使える
4. **小さなPR**: レビュー負荷を軽減

---

## 🚨 トラブルシューティング

### コンパイルエラー「method not found」
```rust
// src/backend/mir_interpreter/mod.rs に追加
mod utils;
```

### テストエラー「MirInterpreter::new_for_test not found」
```rust
// テスト用ビルダー関数を追加
#[cfg(test)]
impl MirInterpreter {
    fn new_for_test() -> Self { /* ... */ }
}
```

---

## 📚 詳細ドキュメント

- **実装手順**: [`PHASE1_IMPLEMENTATION_GUIDE.md`](./PHASE1_IMPLEMENTATION_GUIDE.md)
- **詳細分析**: [`DUPLICATION_ANALYSIS_REPORT.md`](./DUPLICATION_ANALYSIS_REPORT.md)
- **全体サマリー**: [`CLEANUP_SUMMARY_2025-11-06.md`](./CLEANUP_SUMMARY_2025-11-06.md)

---

## 🎯 今日から始める

```bash
# 1. ドキュメント確認（5分）
cat docs/development/cleanup/PHASE1_IMPLEMENTATION_GUIDE.md

# 2. ディレクトリ作成（1分）
mkdir -p src/backend/mir_interpreter/utils

# 3. ファイル作成＆実装（1-2時間）
touch src/backend/mir_interpreter/utils/{mod,register_ops,validation,conversions}.rs
# → 実装ガイドからコピペ

# 4. 最小ファイルで試す（30分）
# → boxes_array.rs を更新

# 5. テスト実行（5分）
./tools/jit_smoke.sh

# 6. 残りのファイルも順次更新（2-4時間）
```

---

**所要時間**: 5-8時間
**リスク**: 低
**効果**: 高（270-380行削減）

今すぐ始めましょう！ 🚀
