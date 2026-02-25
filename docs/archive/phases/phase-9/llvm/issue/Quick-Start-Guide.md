# 🚀 LLVM実装クイックスタートガイド

## 📋 今すぐ始める手順

### 1. **環境準備**（5分）
```bash
# LLVM 17インストール確認
llvm-config --version  # 17.x.x が表示されること

# Nyashプロジェクトで作業
cd /path/to/nyash
git checkout -b feature/llvm-poc
```

### 2. **最初のコミット**（10分）
```bash
# Cargo.tomlを編集
echo '[dependencies]
inkwell = { version = "0.5", features = ["llvm17-0"] }

[features]
llvm = ["inkwell"]' >> Cargo.toml

# ディレクトリ作成
mkdir -p src/backend/llvm

# 最初のファイル作成
touch src/backend/llvm/mod.rs
touch src/backend/llvm/context.rs
touch src/backend/llvm/compiler.rs

# コミット
git add .
git commit -m "feat(llvm): Add inkwell dependency and basic structure"
```

### 3. **最小実装のコピペ**（20分）

**src/backend/llvm/mod.rs**:
```rust
pub mod context;
pub mod compiler;

pub use compiler::compile_to_object;
```

**動作確認**:
```bash
cargo build --features llvm
```

### 4. **テストプログラム作成**（5分）
```bash
# テスト用Nyashファイル
cat > test_return_42.hako << 'EOF'
static box Main {
    main() {
        return 42
    }
}
EOF
```

## 🔍 詰まったときの確認ポイント

### **ビルドエラーの場合**
```bash
# LLVM関連の環境変数確認
echo $LLVM_SYS_170_PREFIX

# 設定されていない場合
export LLVM_SYS_170_PREFIX=$(llvm-config --prefix)
```

### **inkwellのバージョン問題**
```toml
# 代替バージョン
inkwell = { git = "https://github.com/TheDan64/inkwell", branch = "master", features = ["llvm17-0"] }
```

### **リンクエラーの場合**
```bash
# pkg-configの確認
pkg-config --libs --cflags llvm
```

## 📞 ヘルプが必要な場合

1. **GitHub Issue**にコメント
2. **具体的なエラーメッセージ**を貼る
3. **実行したコマンド**を記載

例:
```
inkwellのビルドでエラーが発生しました。

エラー:
```
error: failed to run custom build command for `llvm-sys v170.0.1`
```

実行コマンド:
```
cargo build --features llvm
```

環境:
- OS: Ubuntu 22.04
- LLVM: 17.0.6
- Rust: 1.75.0
```

## ✅ 最初の成功確認

以下が動けば第一歩成功！
```bash
# ビルドが通る
cargo build --features llvm

# テストが実行できる（まだ失敗してOK）
cargo test --features llvm test_llvm
```

## 🎯 次のステップ

1. **context.rs**の実装
2. **compiler.rs**の実装  
3. **return 42**の動作確認

詳細は[001-setup-inkwell-hello-world.md](./001-setup-inkwell-hello-world.md)を参照！

---

**Remember**: 完璧より進捗！最初は動くことが最優先です。🚀