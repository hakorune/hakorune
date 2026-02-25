# ValueId変動エラー修正実装ガイド

**日時**: 2025-11-20
**修正方針**: HashMap/HashSet → BTreeMap/BTreeSet への決定的変更
**推定作業時間**: 1.5時間

---

## 🎯 修正の全体像

### 目標
**PHI生成を完全に決定的にする = 毎回同じValueIdを割り当てる**

### 戦略
1. **`BTreeSet`/`BTreeMap`への置き換え**（決定的ソート保証）
2. **明示的ソート追加**（既存コードへの影響最小化）
3. **テスト検証**（10回実行での一貫性確認）

---

## 📝 実装チェックリスト

### Phase 1: loop_snapshot_merge.rs 修正（最優先）

#### ファイル: `src/mir/phi_core/loop_snapshot_merge.rs`

#### ✅ Change 1: imports修正（Line 19）

```rust
// Before
use std::collections::{HashMap, HashSet};

// After
use std::collections::{HashMap, HashSet, BTreeSet, BTreeMap};
```

#### ✅ Change 2: merge_continue_for_header（Line 65）

```rust
// Before (Line 65)
let mut all_vars: HashSet<String> = HashSet::new();

// After
let mut all_vars: BTreeSet<String> = BTreeSet::new();
```

**影響箇所:**
- Line 66-70: `all_vars.extend()` - API互換（変更不要）
- Line 73: `for var_name in all_vars` - イテレーション順序が決定的に！

#### ✅ Change 3: merge_exit（Line 124）

```rust
// Before (Line 124)
let mut all_vars: HashSet<String> = HashSet::new();

// After
let mut all_vars: BTreeSet<String> = BTreeSet::new();
```

**影響箇所:**
- Line 125-129: `all_vars.extend()` - API互換
- Line 132: `for var_name in all_vars` - 決定的に！

#### ✅ Change 4: merge_exit_with_classification（Line 215）

```rust
// Before (Line 215)
let mut all_vars: HashSet<String> = HashSet::new();

// After
let mut all_vars: BTreeSet<String> = BTreeSet::new();
```

**影響箇所:**
- Line 216-219: `all_vars.extend()` - API互換
- Line 222: `for var_name in all_vars` - 決定的に！

#### ✅ Change 5: sanitize_inputs（Line 323）

```rust
// Before (Line 323)
let mut seen: HashMap<BasicBlockId, ValueId> = HashMap::new();

// After
use std::collections::BTreeMap;
let mut seen: BTreeMap<BasicBlockId, ValueId> = BTreeMap::new();
```

**影響:**
- Line 329: `seen.into_iter().collect()` - BTreeMapから変換、決定的順序保証！
- Line 330: `inputs.sort_by_key()` - 冗長になるが互換性のため残す

---

### Phase 2: if_phi.rs 修正

#### ファイル: `src/mir/phi_core/if_phi.rs`

#### ✅ Change 1: 変数名収集の決定的化

**現在の問題コード:**
```rust
let mut names: HashSet<&str> = HashSet::new();
for k in then_map_end.keys() { names.insert(k.as_str()); }
for k in emap.keys() { names.insert(k.as_str()); }
```

**修正案A: BTreeSet使用**
```rust
let mut names: BTreeSet<&str> = BTreeSet::new();
for k in then_map_end.keys() { names.insert(k.as_str()); }
for k in emap.keys() { names.insert(k.as_str()); }
```

**修正案B: 明示的ソート**
```rust
let mut names: HashSet<&str> = HashSet::new();
for k in then_map_end.keys() { names.insert(k.as_str()); }
for k in emap.keys() { names.insert(k.as_str()); }

// 決定的順序を保証
let mut sorted_names: Vec<&str> = names.into_iter().collect();
sorted_names.sort();
for var_name in sorted_names {
    // PHI生成処理
}
```

---

### Phase 3: loop_phi.rs 修正

#### ファイル: `src/mir/phi_core/loop_phi.rs`

#### ✅ Change 1: all_vars収集の決定的化

**現在の問題コード:**
```rust
let mut all_vars = std::collections::HashSet::new();
for var_name in header_vars.keys() {
    all_vars.insert(var_name.clone());
}
```

**修正案:**
```rust
let mut all_vars = std::collections::BTreeSet::new();
for var_name in header_vars.keys() {
    all_vars.insert(var_name.clone());
}
```

---

### Phase 4: loopform_builder.rs 修正

#### ファイル: `src/mir/phi_core/loopform_builder.rs`

#### ✅ Change 1: snapshot.keys()イテレーションの決定的化

**現在の問題コード:**
```rust
for var_name in snapshot.keys() {
    // 変数処理
}
```

**修正案A: keys()をソート**
```rust
let mut sorted_keys: Vec<_> = snapshot.keys().collect();
sorted_keys.sort();
for var_name in sorted_keys {
    // 変数処理
}
```

**修正案B: snapshot自体をBTreeMapに変更**（より根本的）
```rust
// 関数シグネチャを変更
fn process_snapshot(snapshot: &BTreeMap<String, ValueId>) {
    for var_name in snapshot.keys() {  // 既に決定的！
        // 変数処理
    }
}
```

---

## 🧪 テスト検証手順

### Step 1: コンパイル確認

```bash
cd /home/tomoaki/git/hakorune-selfhost
cargo build --release
```

**期待結果:**
- ✅ コンパイル成功（エラーなし）
- ✅ 警告なし（API互換性確認）

### Step 2: ValueId一貫性テスト（最重要）

```bash
# 10回実行してValueIdが一致するか確認
for i in {1..10}; do
  echo "=== Run $i ==="
  NYASH_MIR_TEST_DUMP=1 ./target/release/nyash apps/selfhost-compiler/test_funcscanner.hako \
    2>&1 | tee /tmp/run_$i.log | grep -E "ValueId\(|Value %"
done

# 差分確認
for i in {2..10}; do
  echo "=== Comparing run 1 vs run $i ==="
  diff /tmp/run_1.log /tmp/run_$i.log
done
```

**期待結果:**
- ✅ 全ての実行で**同じValueId**
- ✅ `diff`で差分なし

### Step 3: MIR検証エラー確認

```bash
NYASH_MIR_TEST_DUMP=1 ./target/release/nyash apps/selfhost-compiler/test_funcscanner.hako \
  2>&1 | grep "verification errors"
```

**期待結果:**
- ✅ 検証エラーなし、または一貫したエラー（非決定的でない）

### Step 4: 267/268テスト回帰確認

```bash
# 既存のパステストが引き続き通ることを確認
./target/release/nyash apps/selfhost-compiler/test_267_simplified.hako
./target/release/nyash apps/selfhost-compiler/test_268_simplified.hako
```

**期待結果:**
- ✅ 両テストPASS

### Step 5: Option Cログ確認

```bash
NYASH_OPTION_C_DEBUG=1 ./target/release/nyash apps/selfhost-compiler/test_funcscanner.hako \
  2>&1 | grep "Option C"
```

**期待結果:**
- ✅ Phantom blockスキップログなし（全blockが正常）
- ✅ 変数分類ログが決定的順序で出力

### Step 6: 全PHIテスト

```bash
cargo test --package nyash --lib mir::phi_core
```

**期待結果:**
- ✅ 全テストPASS

---

## 🔍 デバッグ支援

### トラブルシューティング

#### 問題1: コンパイルエラー「BTreeSet not found」

**原因:** importを忘れた

**修正:**
```rust
use std::collections::{HashMap, HashSet, BTreeSet, BTreeMap};
```

#### 問題2: 依然としてValueIdが変動する

**診断手順:**

1. **全ファイルでBTreeSet使用確認**
```bash
rg "HashSet<String>" src/mir/phi_core/
```

2. **他のHashMapイテレーション発見**
```bash
rg "for \w+ in \w+\.keys\(\)" src/mir/phi_core/
```

3. **snapshot型確認**
```bash
rg "HashMap<String, ValueId>" src/mir/
```

#### 問題3: 267/268テストが失敗

**診断:**
- BTreeSetのソート順序がロジックに影響している可能性
- デバッグログで変数処理順序を確認

```bash
NYASH_OPTION_C_DEBUG=1 ./target/release/nyash test_267_simplified.hako 2>&1 | less
```

---

## 📊 パフォーマンス影響分析

### BTreeSet vs HashSet

| 操作 | HashSet | BTreeSet | 影響 |
|------|---------|----------|------|
| insert | O(1) | O(log n) | 微小（n < 100） |
| contains | O(1) | O(log n) | 微小 |
| iteration | O(n) | O(n) | 同等（ソート済み） |

**実用上の影響:**
- PHI生成での変数数: 通常 < 50
- パフォーマンス低下: < 1%（計測不能レベル）
- **決定性のメリット >> 微小な性能低下**

---

## 🎯 成功基準

### ✅ 修正完了の判定基準

1. **決定性テスト:** 10回実行で全て同じValueId
2. **MIR検証:** エラーなし、または一貫したエラー
3. **回帰テスト:** 267/268テストPASS
4. **全PHIテスト:** `cargo test` PASS
5. **コードレビュー:** BTreeSet使用箇所確認

---

## 📝 コミットメッセージ案

```
fix(phi): Replace HashMap/HashSet with BTreeMap/BTreeSet for deterministic PHI generation

Problem:
- ValueId errors varied between runs (268/271/280)
- HashMap/HashSet iteration order is non-deterministic
- MIR verification errors also varied (%307/%304)

Solution:
- Replace HashSet<String> with BTreeSet<String> in loop_snapshot_merge.rs
- Replace HashMap with BTreeMap in sanitize_inputs()
- Add deterministic sorting to if_phi.rs, loop_phi.rs, loopform_builder.rs

Impact:
- PHI generation is now fully deterministic
- ValueId assignment is consistent across runs
- MIR verification errors are reproducible
- Performance impact < 1% (negligible)

Testing:
- 10 consecutive runs produce identical ValueIds
- 267/268 tests continue to PASS
- All PHI tests PASS

Fixes: ValueId non-determinism bug
Refs: Step 5-5-H (phantom block check working as expected)
```

---

## 🚀 実装開始コマンド

```bash
# 1. バックアップ
cp src/mir/phi_core/loop_snapshot_merge.rs src/mir/phi_core/loop_snapshot_merge.rs.backup

# 2. ファイル編集
cd /home/tomoaki/git/hakorune-selfhost
# Edit: src/mir/phi_core/loop_snapshot_merge.rs

# 3. ビルド
cargo build --release

# 4. テスト
for i in {1..10}; do
  NYASH_MIR_TEST_DUMP=1 ./target/release/nyash apps/selfhost-compiler/test_funcscanner.hako \
    2>&1 | tee /tmp/run_$i.log | grep -E "ValueId\(|Value %"
done

# 5. 差分確認
for i in {2..10}; do
  diff /tmp/run_1.log /tmp/run_$i.log && echo "Run $i: IDENTICAL ✅"
done
```

---

**準備完了！修正を開始してください。**

*Implementation Guide by Claude Code - Ready to Fix!*
