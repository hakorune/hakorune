Status: Historical

# クイック削除ガイド - 今すぐ実行可能なレガシーコード削除

**即実行可能**: Safe削除のみ (約3,900行削減)

---

## 🚀 実行手順 (コピペ可能)

### Step 1: Cranelift/JIT削除 (約1,500行)

```bash
cd /home/tomoaki/git/hakorune-selfhost

# Cranelift JITファイル削除
rm src/runner/modes/cranelift.rs
rm src/runner/modes/aot.rs
rm src/runner/jit_direct.rs

# Cranelift JITテスト削除
rm src/tests/core13_smoke_jit.rs
rm src/tests/core13_smoke_jit_map.rs

# backend/mod.rsのcranelift参照削除 (手動編集)
echo "⚠️ src/backend/mod.rs の29-52行を手動削除してください"

# cli/args.rsのcranelift参照削除 (手動編集)
echo "⚠️ src/cli/args.rs の --backend cranelift オプションを削除してください"

# runner/dispatch.rsのcranelift分岐削除 (手動編集)
echo "⚠️ src/runner/dispatch.rs のcranelift分岐を削除してください"
```

### Step 2: BID Copilotアーカイブ (約1,900行)

```bash
# アーカイブディレクトリ作成
mkdir -p archive/bid-copilot-prototype

# BID Copilotコード移動
git mv src/bid-codegen-from-copilot archive/bid-copilot-prototype/
git mv src/bid-converter-copilot archive/bid-copilot-prototype/

# 確認
ls -la archive/bid-copilot-prototype/
```

### Step 3: 明確なDead Code削除 (約500行)

```bash
# Legacy MIR Expression削除
rm src/mir/builder/exprs_legacy.rs

# src/mir/builder.rsから参照削除 (手動編集)
echo "⚠️ src/mir/builder.rs の exprs_legacy モジュール宣言を削除してください"

# src/mir/builder.rs の build_expression_impl_legacy() 呼び出し削除
echo "⚠️ src/mir/builder.rs の build_expression_impl_legacy() を削除してください"
```

### Step 4: ビルド確認

```bash
# ビルドテスト
cargo build --release

# テスト実行
cargo test

# LLVMビルドテスト
cargo build --release --features llvm

# スモークテスト
./tools/smokes/v2/run.sh --profile quick
```

---

## 📋 手動編集が必要なファイル

### 1. `src/backend/mod.rs` (削除: 29-52行)

```rust
// 削除対象
#[cfg(feature = "cranelift-jit")]
pub mod cranelift;
#[cfg(feature = "cranelift-jit")]
pub use cranelift::{
    compile_and_execute as cranelift_compile_and_execute,
    compile_to_object as cranelift_compile_to_object,
};
```

### 2. `src/cli/args.rs`

```rust
// 削除対象: --backend cranelift オプション
// "cranelift" => Backend::Cranelift, の行を削除
```

### 3. `src/runner/dispatch.rs`

```rust
// 削除対象: cranelift分岐
// Backend::Cranelift => { ... } の分岐を削除
```

### 4. `src/mir/builder.rs`

```rust
// 削除対象1: モジュール宣言
// mod exprs_legacy;

// 削除対象2: build_expression_impl_legacy() 呼び出し
// self.build_expression_impl_legacy(ast) の分岐を削除
```

### 5. `src/runner/modes/mod.rs`

```rust
// 削除対象
#[cfg(feature = "cranelift-jit")]
pub mod aot;
```

---

## ✅ 削除確認チェックリスト

- [ ] `cargo build --release` が成功
- [ ] `cargo test` がパス
- [ ] `cargo build --release --features llvm` が成功
- [ ] `./tools/smokes/v2/run.sh --profile quick` がパス
- [ ] `git status` で削除ファイル確認
- [ ] 手動編集箇所の確認完了

---

## 📊 削減結果

| カテゴリ | 削減行数 |
|---------|---------|
| Cranelift/JIT | 約1,500行 |
| BID Copilot | 約1,900行 |
| Dead Code | 約500行 |
| **合計** | **約3,900行** |

---

## 🔍 次のステップ (調査後削除)

### Phase B: 調査が必要な項目
1. **JSON v1 Bridge** (734行) - 使用状況確認
2. **Legacy Test Files** (1,000行) - identical_exec系の整理
3. **Parser Dead Code** (約500行) - 実使用確認

### Phase C: Phase 16以降
1. **WASM Backend** (3,170行) - 動作確認後アーカイブ
2. **Builtin Box移行** (264行) - プラグイン完全移行後

---

## 🚨 問題が発生した場合

### ビルドエラー
```bash
# 変更を戻す
git restore .
git clean -fd

# 詳細レポート確認
cat docs/development/cleanup/LEGACY_CODE_INVESTIGATION_REPORT.md
```

### テスト失敗
- 削除したファイルが実際に使用されている可能性
- 詳細レポートの「要確認 (Investigate)」セクションを参照

---

**実行時間**: 約10分 (手動編集含む)
**リスク**: 無し (Safe削除のみ)
**削減効果**: 約3,900行 (4%)
