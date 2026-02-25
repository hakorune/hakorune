# Phase 1実装ガイド - 緊急対応項目

**対象期間**: 1-2週間
**目標**: 最も影響の大きい3つの巨大ファイル/モジュールを整理

---

## 1. handlers/calls.rs分割（49,750行）

### 1.1 現状分析

**ファイルパス**: `src/backend/mir_interpreter/handlers/calls.rs`
**行数**: 49,750行（プロジェクト最大）

### 1.2 責務の分類

calls.rsを詳細調査し、以下のカテゴリに分類：

```bash
# 調査コマンド
cd /home/tomoaki/git/hakorune-selfhost
rg "^pub\(super\) fn|^fn " src/backend/mir_interpreter/handlers/calls.rs | head -50
```

**予想される責務**:
1. 関数呼び出し解決 (resolution)
2. 呼び出しディスパッチ (dispatch)
3. 引数処理 (argument handling)
4. 戻り値処理 (return value handling)
5. エラーハンドリング (error handling)

### 1.3 分割戦略

#### ステップ1: 分析・マッピング（1日）

```bash
# 関数一覧を抽出
rg "^pub\(super\) fn|^fn " src/backend/mir_interpreter/handlers/calls.rs \
  > /tmp/calls_functions.txt

# 依存関係を分析
rg "self\." src/backend/mir_interpreter/handlers/calls.rs \
  | sort | uniq > /tmp/calls_dependencies.txt
```

**作業内容**:
- [ ] 全関数の責務を分類
- [ ] 関数間の依存関係を可視化
- [ ] 共有される型・定数を特定

#### ステップ2: ディレクトリ構造作成（0.5日）

```
handlers/
└── calls/
    ├── mod.rs           # 公開インターフェース
    ├── resolution.rs    # 呼び出し解決
    ├── dispatch.rs      # ディスパッチロジック
    ├── arguments.rs     # 引数処理
    ├── returns.rs       # 戻り値処理
    ├── errors.rs        # エラーハンドリング
    └── shared.rs        # 共有型・定数
```

**実装**:
```bash
cd /home/tomoaki/git/hakorune-selfhost
mkdir -p src/backend/mir_interpreter/handlers/calls
```

#### ステップ3: 段階的移動（2日）

**移動順序**（依存の少ない順）:
1. `shared.rs` - 共有型・定数
2. `errors.rs` - エラー型
3. `arguments.rs` - 引数処理
4. `returns.rs` - 戻り値処理
5. `resolution.rs` - 解決ロジック
6. `dispatch.rs` - メインディスパッチ
7. `mod.rs` - 統合

**各ファイルのテンプレート**:
```rust
// src/backend/mir_interpreter/handlers/calls/resolution.rs
use super::shared::*;
use super::errors::*;
use crate::backend::mir_interpreter::MirInterpreter;
use crate::backend::vm_types::VMError;

impl MirInterpreter {
    /// 呼び出しターゲットを解決
    pub(in crate::backend::mir_interpreter) fn resolve_call_target(
        &self,
        func_name: &str,
    ) -> Result<CallTarget, VMError> {
        // ... 実装
    }
}
```

#### ステップ4: 統合・テスト（0.5日）

**mod.rsの作成**:
```rust
// src/backend/mir_interpreter/handlers/calls/mod.rs
mod shared;
mod errors;
mod arguments;
mod returns;
mod resolution;
mod dispatch;

// 必要に応じて再エクスポート
pub(super) use resolution::*;
pub(super) use dispatch::*;
// ...
```

**handlers/mod.rsの更新**:
```rust
// src/backend/mir_interpreter/handlers/mod.rs
mod calls;  // calls.rs → calls/mod.rs

impl MirInterpreter {
    pub(super) fn execute_instruction(&mut self, inst: &MirInstruction) -> Result<(), VMError> {
        match inst {
            MirInstruction::Call { ... } => self.handle_call(...)?,
            // ...
        }
        Ok(())
    }
}
```

#### ステップ5: 検証（0.5日）

```bash
# コンパイル確認
cargo build --release

# テスト実行
cargo test --package nyash_rust --lib backend::mir_interpreter

# スモークテスト
./tools/smokes/v2/run.sh --profile quick --filter "call_*"
```

### 1.4 成功基準

- [ ] calls.rsが6つのファイルに分割（各5,000-10,000行以下）
- [ ] コンパイルエラーなし
- [ ] 既存テストが全てパス
- [ ] スモークテストが全てパス

---

## 2. runner/modes/common.rs分割（14,000行）

### 2.1 現状分析

**ファイルパス**: `src/runner/modes/common.rs`
**行数**: 14,000行
**問題**: 「共通処理」という曖昧な責務

### 2.2 責務の分類

```bash
# 関数一覧抽出
rg "^pub fn|^fn " src/runner/modes/common.rs > /tmp/common_functions.txt

# インポート分析
rg "^use " src/runner/modes/common.rs | sort | uniq > /tmp/common_imports.txt
```

**予想される責務**:
1. ファイルI/O処理
2. MIRコンパイル・実行
3. using/namespace解決
4. 環境変数処理
5. エラーハンドリング

### 2.3 分割戦略

#### ステップ1: 責務マッピング（1日）

**分析スクリプト**:
```bash
#!/bin/bash
# analyze_common.sh
file="src/runner/modes/common.rs"

echo "=== 関数分析 ==="
rg "^pub fn|^fn " "$file" | awk '{print $2}' | sed 's/(.*//' | sort

echo "=== use文分析 ==="
rg "^use " "$file" | awk '{print $2}' | sort | uniq -c | sort -rn | head -20

echo "=== MIRに関連する関数 ==="
rg "fn.*mir" "$file" -i

echo "=== 実行に関連する関数 ==="
rg "fn.*execute|fn.*run" "$file" -i

echo "=== 解決に関連する関数 ==="
rg "fn.*resolve" "$file" -i
```

#### ステップ2: 新構造の設計（0.5日）

```
runner/
├── execution/         # 実行関連（common.rsから移動）
│   ├── mod.rs
│   ├── vm.rs          # VM実行
│   ├── llvm.rs        # LLVM実行
│   ├── pyvm.rs        # PyVM実行
│   └── prepare.rs     # 実行準備
├── pipeline/          # パイプライン処理
│   ├── mod.rs
│   ├── compilation.rs # MIRコンパイル
│   ├── preprocessing.rs
│   └── postprocessing.rs
├── resolution/        # using/namespace解決
│   ├── mod.rs
│   ├── using.rs
│   ├── namespace.rs
│   └── prelude.rs
└── modes/             # 既存（execution/等からインポート）
    ├── mod.rs
    ├── vm.rs          # execution::vm を呼ぶだけ
    ├── llvm.rs
    └── pyvm.rs
```

#### ステップ3: 段階的移動（2日）

**移動順序**:
1. **Phase A**: resolution/ に using解決関連を移動
2. **Phase B**: pipeline/ に MIRコンパイル関連を移動
3. **Phase C**: execution/ に実行関連を移動
4. **Phase D**: modes/ を薄いラッパーに変更

**Phase A実装例**:
```rust
// src/runner/resolution/using.rs
use crate::runner::modes::common_util::resolve::using_resolution::*;
use std::path::Path;

/// using target を解決
pub fn resolve_using_target(
    target: &str,
    context: Option<&Path>,
) -> Result<String, String> {
    // ... common.rsから移動した実装
}
```

**Phase D実装例**:
```rust
// src/runner/modes/vm.rs
use crate::runner::execution;
use crate::runner::pipeline;
use crate::cli::CliGroups;

pub fn execute_vm_mode(groups: &CliGroups, filename: &str) -> i32 {
    // 薄いラッパー - 実際の処理は execution/pipeline/ に委譲
    let module = match pipeline::compile_file(filename) {
        Ok(m) => m,
        Err(e) => {
            eprintln!("❌ Compilation error: {}", e);
            return 1;
        }
    };

    execution::run_vm(&module, groups)
}
```

#### ステップ4: common.rsの削除（0.5日）

```bash
# 移動後、common.rsが空になったことを確認
wc -l src/runner/modes/common.rs  # 目標: 50行以下

# 削除または最小限のre-export
```

**最小限のcommon.rs**:
```rust
// src/runner/modes/common.rs
// 後方互換性のための re-export のみ
#[deprecated(note = "Use crate::runner::execution instead")]
pub use crate::runner::execution::*;

#[deprecated(note = "Use crate::runner::pipeline instead")]
pub use crate::runner::pipeline::*;
```

#### ステップ5: テスト・検証（0.5日）

```bash
# コンパイル確認
cargo build --release

# 統合テスト
cargo test --package nyash_rust --lib runner

# E2Eテスト
./tools/smokes/v2/run.sh --profile quick
./tools/smokes/v2/run.sh --profile integration
```

### 2.4 成功基準

- [ ] common.rsが1,000行以下（理想: 100行以下のre-export）
- [ ] 新構造（execution/pipeline/resolution/）が明確
- [ ] 全テストがパス
- [ ] deprecation警告が適切に表示される

---

## 3. runtime/plugin_loader_v2/階層整理

### 3.1 現状分析

**現在の構造**:
```
runtime/
└── plugin_loader_v2/
    ├── mod.rs
    ├── stub.rs
    └── enabled/
        ├── mod.rs
        ├── globals.rs
        ├── method_resolver.rs
        ├── types.rs
        ├── extern_functions.rs
        ├── ffi_bridge.rs
        ├── host_bridge.rs
        ├── instance_manager.rs
        ├── errors.rs
        └── loader/            # 5階層目！
            ├── mod.rs
            ├── specs.rs
            ├── metadata.rs
            ├── singletons.rs
            ├── library.rs
            ├── config.rs
            └── util.rs
```

**問題点**:
- 最深5階層（`runtime/plugin_loader_v2/enabled/loader/specs.rs`）
- `enabled/`という曖昧な名前
- `loader/`が1階層深い

### 3.2 新構造の設計

```
runtime/
└── plugins/              # plugin_loader_v2 → plugins
    ├── mod.rs
    ├── stub.rs           # プラグイン無効時
    ├── core/             # enabled → core
    │   ├── mod.rs
    │   ├── types.rs
    │   ├── globals.rs
    │   ├── method_resolver.rs
    │   └── instance_manager.rs
    ├── loader/           # 1階層上げる
    │   ├── mod.rs
    │   ├── specs.rs
    │   ├── metadata.rs
    │   ├── singletons.rs
    │   └── library.rs
    ├── bridge/           # FFI/ブリッジ処理
    │   ├── mod.rs
    │   ├── ffi.rs        # ffi_bridge.rs → ffi.rs
    │   ├── host.rs       # host_bridge.rs → host.rs
    │   └── extern_functions.rs
    ├── config.rs         # loader/config.rs → トップレベル
    └── errors.rs         # トップレベルへ
```

**改善点**:
- 最深4階層（`runtime/plugins/loader/specs.rs`）
- 明確な責務分離（core/loader/bridge）
- 短いインポートパス

### 3.3 移行戦略

#### ステップ1: 新ディレクトリ作成（0.5日）

```bash
cd /home/tomoaki/git/hakorune-selfhost
mkdir -p src/runtime/plugins/{core,loader,bridge}
```

#### ステップ2: ファイル移動（1日）

**移動計画**:
```bash
# Phase A: 独立ファイル
mv src/runtime/plugin_loader_v2/stub.rs src/runtime/plugins/
mv src/runtime/plugin_loader_v2/enabled/errors.rs src/runtime/plugins/
mv src/runtime/plugin_loader_v2/enabled/loader/config.rs src/runtime/plugins/

# Phase B: core/
mv src/runtime/plugin_loader_v2/enabled/{types,globals,method_resolver,instance_manager}.rs \
   src/runtime/plugins/core/

# Phase C: bridge/
mv src/runtime/plugin_loader_v2/enabled/ffi_bridge.rs src/runtime/plugins/bridge/ffi.rs
mv src/runtime/plugin_loader_v2/enabled/host_bridge.rs src/runtime/plugins/bridge/host.rs
mv src/runtime/plugin_loader_v2/enabled/extern_functions.rs src/runtime/plugins/bridge/

# Phase D: loader/
mv src/runtime/plugin_loader_v2/enabled/loader/*.rs src/runtime/plugins/loader/
```

#### ステップ3: モジュール宣言更新（0.5日）

**plugins/mod.rs**:
```rust
// src/runtime/plugins/mod.rs
#[cfg(not(feature = "disable-plugins"))]
pub mod core;
#[cfg(not(feature = "disable-plugins"))]
pub mod loader;
#[cfg(not(feature = "disable-plugins"))]
pub mod bridge;

#[cfg(feature = "disable-plugins")]
pub mod stub;

pub mod config;
pub mod errors;

// 後方互換性のための re-export
#[cfg(not(feature = "disable-plugins"))]
pub use core::{PluginBoxType, MethodHandle};
#[cfg(not(feature = "disable-plugins"))]
pub use loader::PluginLoaderV2;
```

**runtime/mod.rs**:
```rust
// src/runtime/mod.rs
pub mod plugins;  // plugin_loader_v2 → plugins

// 後方互換性
#[deprecated(note = "Use crate::runtime::plugins instead")]
pub use plugins as plugin_loader_v2;
```

#### ステップ4: インポート更新（1日）

**検索・置換**:
```bash
# 全ファイルでインポートパスを更新
find src -name "*.rs" -type f -exec sed -i \
  's/crate::runtime::plugin_loader_v2/crate::runtime::plugins/g' {} \;

find src -name "*.rs" -type f -exec sed -i \
  's/crate::runtime::plugin_loader_v2::enabled/crate::runtime::plugins::core/g' {} \;
```

**手動確認箇所**:
- `src/runner/plugins.rs`
- `src/backend/mir_interpreter/handlers/boxes_plugin.rs`
- `src/runtime/unified_registry.rs`

#### ステップ5: テスト・検証（0.5日）

```bash
# コンパイル確認（プラグイン有効）
cargo build --release

# コンパイル確認（プラグイン無効）
cargo build --release --features disable-plugins

# プラグインテスト
cargo test --package nyash_rust --lib runtime::plugins

# E2Eテスト
NYASH_SKIP_TOML_ENV=1 ./tools/smoke_plugins.sh
```

#### ステップ6: 旧ディレクトリ削除（0.5日）

```bash
# 移行完了後
rm -rf src/runtime/plugin_loader_v2

# Git確認
git status
git diff --stat
```

### 3.4 成功基準

- [ ] 最深階層が4階層以下
- [ ] `plugin_loader_v2` → `plugins` に完全移行
- [ ] 全テストがパス
- [ ] deprecation警告が適切に機能

---

## 4. Phase 1全体のチェックリスト

### 準備（開始前）

- [ ] Gitブランチ作成: `refactor/phase1-module-structure`
- [ ] ベースラインテスト実行・記録
- [ ] バックアップ取得

### 実装（1-2週間）

#### Week 1
- [ ] Day 1-2: calls.rs分析・分割計画
- [ ] Day 3-4: calls.rs移動・統合
- [ ] Day 5: calls.rs検証・テスト

#### Week 2
- [ ] Day 1-2: common.rs分析・分割計画
- [ ] Day 3-4: common.rs移動・統合
- [ ] Day 5: common.rs検証

#### Week 2 (並行可能)
- [ ] Day 1-2: plugin_loader_v2移行計画
- [ ] Day 3-4: plugin_loader_v2移動・統合
- [ ] Day 5: plugin_loader_v2検証

### 検証（完了後）

- [ ] 全コンパイルエラー解消
- [ ] 全ユニットテストパス
- [ ] スモークテストパス（quick/integration）
- [ ] ドキュメント更新
- [ ] PRレビュー・マージ

---

## 5. ロールバック計画

### トラブル発生時の対応

**軽微な問題**（コンパイルエラー等）:
```bash
# 部分的に戻す
git checkout HEAD -- src/backend/mir_interpreter/handlers/calls/
```

**重大な問題**（実行時エラー・テスト失敗）:
```bash
# ブランチ全体を破棄
git checkout main
git branch -D refactor/phase1-module-structure
```

**再実行の判断基準**:
- 問題の原因が明確で、修正に1日以内
- それ以外はロールバックして再計画

---

## 6. コミュニケーション

### チーム共有

**開始時**:
- [ ] CURRENT_TASK.mdに記載
- [ ] チームに通知（Issue/PR作成）

**進捗報告**（毎日）:
- [ ] 完了した作業
- [ ] 発見した問題
- [ ] 翌日の予定

**完了時**:
- [ ] PR作成
- [ ] レビュー依頼
- [ ] ドキュメント更新を含める

---

## 7. 次のステップ

Phase 1完了後、Phase 2（高優先度項目）に進む：
- BID関連モジュール統一
- boxes/カテゴリ再編
- runner/modes/common_util/再構成

**Phase 2実装ガイド**: `PHASE2_IMPLEMENTATION_GUIDE.md`（別途作成）

---

**このガイドに従って、Phase 1の3項目を確実に完了させましょう！**
