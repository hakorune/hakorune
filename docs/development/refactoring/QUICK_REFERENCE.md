# モジュール構造改善 - クイックリファレンス

**最終更新**: 2025-11-06

## 📋 3つのドキュメント

| ドキュメント | 用途 | 対象読者 |
|------------|------|---------|
| **MODULE_STRUCTURE_ANALYSIS.md** | 全体分析・戦略 | チームリーダー・アーキテクト |
| **PHASE1_IMPLEMENTATION_GUIDE.md** | Phase 1実装手順 | 開発者 |
| **このファイル** | クイックリファレンス | 全員 |

---

## 🎯 緊急対応（Phase 1）- 1-2週間

### 対象ファイル

| ファイル | 現在行数 | 目標 | 優先度 |
|---------|---------|------|--------|
| `handlers/calls.rs` | 49,750 | 6ファイルに分割 | 🔥最高 |
| `runner/modes/common.rs` | 14,000 | 1,000行以下 | 🔥最高 |
| `runtime/plugin_loader_v2/` | 5階層 | 4階層に削減 | 🔥最高 |

### クイック実装手順

#### 1. calls.rs分割
```bash
# 1. ディレクトリ作成
mkdir -p src/backend/mir_interpreter/handlers/calls

# 2. 責務別に分割
# - resolution.rs (呼び出し解決)
# - dispatch.rs (ディスパッチ)
# - arguments.rs (引数処理)
# - returns.rs (戻り値処理)
# - errors.rs (エラー)
# - shared.rs (共有型)

# 3. テスト
cargo build --release && cargo test
```

#### 2. common.rs分割
```bash
# 1. 新構造作成
mkdir -p src/runner/{execution,pipeline,resolution}

# 2. 責務別に移動
# execution/ - VM/LLVM/PyVM実行
# pipeline/ - MIRコンパイル・前処理
# resolution/ - using/namespace解決

# 3. modesを薄いラッパーに
# modes/*.rs → execution/*.rs を呼ぶだけ

# 4. テスト
./tools/smokes/v2/run.sh --profile quick
```

#### 3. plugin_loader_v2階層整理
```bash
# 1. 新構造作成
mkdir -p src/runtime/plugins/{core,loader,bridge}

# 2. ファイル移動
# enabled/ → core/
# enabled/loader/ → loader/
# enabled/*_bridge.rs → bridge/

# 3. runtime/mod.rs更新
# pub mod plugins;

# 4. テスト
NYASH_SKIP_TOML_ENV=1 ./tools/smoke_plugins.sh
```

---

## 📊 統計サマリー

### 現状
- **総ファイル数**: 500+
- **最大ファイル**: calls.rs (49,750行)
- **平均ファイル**: 500-1000行
- **最深階層**: 5階層

### 目標（Phase 1完了後）
- **1,000行超ファイル**: 20+ → 10以下
- **最大ファイル**: 10,000行以下
- **平均ファイル**: 300-500行
- **最深階層**: 4階層以下

---

## 🚨 重要な注意点

### やってはいけないこと
1. ❌ 一度に全ての変更を行う（段階的に！）
2. ❌ テストをスキップする（毎回確認！）
3. ❌ 後方互換性を無視する（deprecation使用）
4. ❌ ドキュメント更新を忘れる

### 必ずやること
1. ✅ Gitブランチを作成してから作業
2. ✅ 各ステップでコンパイル確認
3. ✅ 全テストが通ることを確認
4. ✅ 進捗をチームに共有

---

## 🔧 便利なコマンド

### ファイル分析
```bash
# ファイル行数ランキング
find src -name "*.rs" -exec wc -l {} \; | sort -rn | head -20

# 特定モジュールの行数
wc -l src/backend/mir_interpreter/handlers/*.rs | sort -rn

# 関数一覧抽出
rg "^pub fn|^fn " src/path/to/file.rs
```

### 依存関係分析
```bash
# インポート分析
rg "^use crate::" src/module/ | sort | uniq -c | sort -rn

# 特定モジュールへの依存を検索
rg "use crate::runtime::plugin_loader_v2" src/
```

### テスト実行
```bash
# フルビルド
cargo build --release

# 特定モジュールのテスト
cargo test --package nyash_rust --lib backend::mir_interpreter

# スモークテスト（VM）
./tools/smokes/v2/run.sh --profile quick

# スモークテスト（LLVM統合）
./tools/smokes/v2/run.sh --profile integration

# プラグインテスト
NYASH_SKIP_TOML_ENV=1 ./tools/smoke_plugins.sh
```

---

## 📚 追加リソース

### Rustのベストプラクティス
- [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- [Rust Book - Modules](https://doc.rust-lang.org/book/ch07-00-managing-growing-projects-with-packages-crates-and-modules.html)

### プロジェクト内ドキュメント
- `/docs/development/architecture/` - アーキテクチャ設計
- `/docs/development/roadmap/` - ロードマップ
- `/CURRENT_TASK.md` - 現在のタスク

---

## 🎯 次のアクション

### 今すぐやること
1. [ ] MODULE_STRUCTURE_ANALYSIS.mdを読む
2. [ ] PHASE1_IMPLEMENTATION_GUIDE.mdを読む
3. [ ] Gitブランチを作成
4. [ ] calls.rs分析を開始

### 1週間後
1. [ ] calls.rs分割完了
2. [ ] common.rs分析・分割開始

### 2週間後
1. [ ] Phase 1全項目完了
2. [ ] PR作成・レビュー
3. [ ] Phase 2計画開始

---

## 📞 サポート

質問・相談は以下へ：
- **Issue**: GitHub Issueで質問
- **PR**: レビューリクエスト
- **CURRENT_TASK.md**: 進捗報告

---

**頑張って！段階的に、確実に進めよう！** 🚀
