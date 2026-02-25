# 🔄 wasmtime互換性マトリクス

## 📅 最終更新: 2025-08-15

## 🎯 **現在の状況**

### 開発環境
```toml
# Cargo.toml
wasmtime = "18.0"
wabt = "0.10"
```

### 実行環境
```bash
# システムインストール
wasmtime 35.0.0 (509af9e5f 2025-07-22)
```

### 互換性状況
❌ **非互換**: 18.0.4 vs 35.0.0 - 実行不可

---

## 📊 **バージョン互換性マトリクス**

| Nyash wasmtime | System wasmtime | 互換性 | 状況 | 対応 |
|----------------|-----------------|--------|------|------|
| **18.0.4** | **35.0.0** | ❌ | 現在 | 要修正 |
| 35.0.x | 35.0.x | ✅ | 目標 | 推奨 |
| 34.0.x | 35.0.x | ⚠️ | 検証必要 | テスト |
| 33.0.x | 35.0.x | ❌ | 古すぎ | 非推奨 |

---

## 🔧 **修正オプション**

### Option A: Nyash側更新 (推奨)
```toml
# Cargo.toml - 更新案
wasmtime = "35.0"
wabt = "0.10"  # 互換性確認必要
```

**メリット**:
- ✅ 最新機能・性能向上
- ✅ セキュリティ修正取り込み
- ✅ 将来性

**リスク**:
- ⚠️ API変更による修正必要
- ⚠️ 既存.cwasmファイル互換性喪失

### Option B: システム側ダウングレード
```bash
# wasmtime 18.0.4 をインストール
curl -sSf https://wasmtime.dev/install.sh | bash -s -- --version 18.0.4
```

**メリット**:
- ✅ Nyashコード修正不要
- ✅ 即座対応可能

**デメリット**:
- ❌ 古いバージョン使用
- ❌ セキュリティリスク
- ❌ 他プロジェクトへの影響

---

## 🎯 **推奨対応手順**

### Step 1: 依存関係調査 (30分)
```bash
# 現在の依存関係確認
cargo tree | grep wasmtime
cargo tree | grep wabt

# API変更点調査
# https://github.com/bytecodealliance/wasmtime/releases
```

### Step 2: テスト環境構築 (30分)
```bash
# ブランチ作成
git checkout -b feature/wasmtime-35-upgrade

# Cargo.toml更新
# wasmtime = "35.0"

# 依存関係更新
cargo update
```

### Step 3: ビルド修正 (2-4時間)
予想される修正箇所：
- `src/backend/aot/compiler.rs`: Engine設定API
- `src/backend/wasm/mod.rs`: Module生成API
- `src/backend/aot/config.rs`: Config構造変更

### Step 4: 動作確認 (1時間)
```bash
# 基本コンパイル
cargo build --release

# WASM/AOT テスト
./target/release/hakorune --aot test_simple.hako
wasmtime --allow-precompiled test_simple.cwasm
```

---

## 📋 **wasmtime API変更予想箇所**

### 18.x → 35.x 主要変更点

#### Engine/Store API
```rust
// 18.x (予想)
let engine = Engine::default();
let store = Store::new(&engine, ());

// 35.x (要確認)
let engine = Engine::new(&Config::default())?;
let mut store = Store::new(&engine, ());
```

#### Module serialize/deserialize
```rust  
// 18.x
module.serialize()?;
Module::deserialize(&engine, bytes)?;

// 35.x (API変更可能性)
module.serialize()?;  // 戻り値型変更？
unsafe { Module::deserialize(&engine, bytes)? }  // unsafe要求？
```

#### Config API
```rust
// 18.x
let config = Config::new();

// 35.x  
let mut config = Config::new();
config.cranelift_opt_level(OptLevel::Speed)?;
```

---

## ✅ **アクションアイテム**

### 緊急 (今日)
- [ ] wasmtime 35.0 API ドキュメント確認
- [ ] 修正工数見積もり (2-8時間予想)

### 短期 (今週)
- [ ] **wasmtime 35.0 への更新実装**
- [ ] 全WASM/AOT機能のテスト実行
- [ ] 互換性問題解決

### 中期 (来週)
- [ ] wasmtime自動バージョン検知機能
- [ ] CI/CDでの互換性テスト自動化

---

## 🎯 **成功指標**

### 技術指標
```bash
# ✅ 成功条件
./target/release/hakorune --aot test.hako      # コンパイル成功
wasmtime --allow-precompiled test.cwasm      # 実行成功
echo $?                                      # 0 (正常終了)
```

### 性能指標
- コンパイル時間: 18.x と同等以上
- 実行速度: 18.x と同等以上  
- メモリ使用量: 18.x と同等以下

---

**🚀 Next Action**: wasmtime 35.0 へのアップグレード実装を最優先で開始