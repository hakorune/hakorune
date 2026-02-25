# Hakorune Rustコードベース - モジュール構造改善分析レポート

**作成日**: 2025-11-06
**対象**: `/home/tomoaki/git/hakorune-selfhost/src/`

## エグゼクティブサマリー

Hakoruneコードベースの現在のモジュール構造を分析した結果、以下の主要な改善機会を特定しました：

### 🎯 最優先改善項目
1. **handlers分散問題** - mir_interpreter/handlersの3,335行を責務別に整理
2. **runner/modes肥大化** - 41ファイル・14,000行のcommon.rsを分割
3. **boxes整理** - 61ファイルのBox実装を機能別に再グルーピング
4. **BID関連モジュールの命名** - 3つのbid-*ディレクトリを統一
5. **utils/common/helpers散在** - 再利用コードの統一整理

### 📊 統計サマリー

| モジュール | ファイル数 | 主な課題 |
|-----------|----------|---------|
| `src/mir/` | 112 | 整理されているが一部utils分散 |
| `src/backend/` | 69 | mir_interpreter/handlersに集約必要 |
| `src/runtime/` | 46 | plugin_loader_v2が深すぎる |
| `src/parser/` | 52 | 概ね良好（declarations分離済み） |
| `src/boxes/` | 61 | カテゴリ分けが不十分 |
| `src/runner/` | 41+ | modes/common_utilが肥大化 |

---

## 1. モジュールの責務分析

### 1.1 責務が曖昧/複数責務を持つモジュール

#### 🔴 優先度: 高

##### `src/backend/mir_interpreter/handlers/` (3,335行)

**現状の問題**:
```
handlers/
├── arithmetic.rs        (4,881行) - 算術演算
├── boxes.rs             (13,161行) - Box操作全般
├── boxes_array.rs       (2,710行) - Array固有
├── boxes_string.rs      (9,907行) - String固有
├── boxes_map.rs         (6,425行) - Map固有
├── boxes_object_fields.rs (22,559行) - オブジェクトフィールド
├── boxes_instance.rs    (8,217行) - インスタンス処理
├── boxes_plugin.rs      (9,654行) - プラグインBox
├── boxes_void_guards.rs (861行) - Voidガード
├── calls.rs             (49,750行) ⚠️ 巨大ファイル
├── externals.rs         (10,584行) - 外部呼び出し
├── extern_provider.rs   (18,350行) - プロバイダ統合
└── ...
```

**問題点**:
- `calls.rs`が49,750行で巨大すぎる（単一ファイル最大）
- boxes_*が8つに分散しているが、boxes.rsも13,161行で巨大
- 責務の境界が不明瞭（boxes.rsとboxes_*.rsの使い分け）

**改善提案**:
```
handlers/
├── core/              # コア命令処理
│   ├── mod.rs
│   ├── arithmetic.rs
│   ├── memory.rs      # Load/Store
│   └── control_flow.rs
├── boxes/             # Box操作（型別）
│   ├── mod.rs
│   ├── primitives.rs  # String/Integer/Bool統合
│   ├── collections.rs # Array/Map統合
│   ├── instances.rs   # ユーザー定義Box
│   └── plugin.rs      # プラグインBox
├── calls/             # 呼び出し処理（分割）
│   ├── mod.rs
│   ├── resolution.rs  # 呼び出し解決
│   ├── dispatch.rs    # ディスパッチ
│   ├── extern.rs      # ExternCall
│   └── provider.rs    # Provider統合
└── misc.rs
```

**影響範囲**: 中（handlers内部のみ）
**優先度**: 高（可読性・保守性に直結）

---

##### `src/runner/modes/common.rs` (14,000行)

**現状の問題**:
- 単一ファイルに14,000行の共通処理が集約
- common_util/ディレクトリも17ファイルで肥大化
- 責務が不明瞭（共通処理とは何か？）

**改善提案**:
```
runner/
├── execution/         # 実行制御
│   ├── vm.rs
│   ├── llvm.rs
│   ├── pyvm.rs
│   └── mir_interpreter.rs
├── pipeline/          # パイプライン処理
│   ├── resolution.rs  # using解決
│   ├── preprocessing.rs
│   └── compilation.rs
├── io/                # 入出力
│   ├── file.rs
│   └── stream.rs
├── selfhost/          # セルフホスト関連
│   ├── executor.rs
│   └── bridge.rs
└── modes/
    └── (execution/からインポート)
```

**影響範囲**: 大（runner全体に影響）
**優先度**: 高（Phase 15の重要目標）

---

#### 🟡 優先度: 中

##### `src/boxes/` (61ファイル)

**現状の問題**:
```
boxes/
├── basic/             # 基本Box（曖昧）
├── arithmetic/        # 算術Box
├── array/
├── buffer/
├── file/
├── http/
├── json/
├── web/               # Web専用
├── string_box.rs      # なぜディレクトリ外？
├── integer_box.rs
├── math_box.rs
└── ... (61ファイル)
```

**問題点**:
- basic/arithmeticが曖昧
- ディレクトリとファイルが混在（string_box.rsはなぜ外？）
- カテゴリ分けが不統一

**改善提案**:
```
boxes/
├── primitives/        # 基本データ型
│   ├── string.rs
│   ├── integer.rs
│   ├── bool.rs
│   └── null.rs
├── collections/       # コレクション
│   ├── array.rs
│   ├── map.rs
│   └── buffer.rs
├── io/                # 入出力
│   ├── file.rs
│   ├── http.rs
│   └── stream.rs
├── system/            # システム
│   ├── console.rs
│   ├── debug.rs
│   └── time.rs
├── advanced/          # 高度な機能
│   ├── json.rs
│   ├── regex.rs
│   └── future.rs
└── platform/          # プラットフォーム依存
    ├── web/
    ├── audio.rs (not wasm32)
    └── egui.rs (feature = "gui")
```

**影響範囲**: 中（boxes/内部とインポートパス）
**優先度**: 中（可読性向上）

---

### 1.2 名前と実態が合っていないモジュール

#### 🔴 `src/bid-*` (BID関連モジュール)

**現状の問題**:
```
src/
├── bid/                      # コア実装？
├── bid-codegen-from-copilot/ # Copilot生成コード？
└── bid-converter-copilot/    # 変換ツール？
```

**問題点**:
- ディレクトリ名にハイフンを使用（Rust慣習に反する）
- "-from-copilot", "-converter-copilot"など命名が一貫性なし
- 役割が名前から不明瞭

**改善提案**:
```
src/
└── bid/
    ├── core/          # BIDコア実装
    ├── codegen/       # コード生成
    ├── converter/     # 変換ツール
    └── plugins/       # プラグイン実装
```

**影響範囲**: 中（BIDシステム全体）
**優先度**: 高（命名規約違反）

---

#### 🟡 `src/runner/modes/common_util/`

**現状の問題**:
- `common_util`という名前が汎用的すぎる
- 実際には"using解決"と"セルフホスト実行"が主
- 17ファイルあり、役割が不明瞭

**改善提案**:
```
runner/
├── resolution/        # using/namespace解決
│   ├── using.rs
│   ├── prelude.rs
│   └── path_util.rs
├── selfhost/          # セルフホスト実行
│   ├── executor.rs
│   ├── bridge.rs
│   └── pipeline.rs
└── io/                # 入出力ヘルパー
    └── hako.rs
```

**影響範囲**: 中（runner内部）
**優先度**: 中（責務明確化）

---

## 2. 依存関係の分析

### 2.1 循環依存

**調査結果**: 現時点で明確な循環依存は検出されませんでした。

- `src/mir/` → `src/backend/` （一方向のみ）
- `src/runtime/` → `src/boxes/` （一方向のみ）
- `src/parser/` → `src/ast/` （一方向のみ）

**理由**: 各モジュールが`crate::`からのインポートを適切に使用しているため。

---

### 2.2 過度な結合

#### 🔴 `src/backend/mir_interpreter/` → `src/runtime/`

**問題点**:
- MIRインタープリターがランタイムに強く依存
- plugin_loader_v2への直接参照が散在
- Box操作がruntime経由で複雑化

**改善提案**:
```rust
// Before: 直接runtime依存
use crate::runtime::plugin_loader_v2::enabled::...;

// After: トレイト経由で抽象化
use crate::backend::traits::BoxProvider;
```

**影響範囲**: 大（backend/runtimeの境界設計）
**優先度**: 中（テスト容易性・将来の拡張性）

---

#### 🟡 `src/runner/` → 複数モジュール

**問題点**:
```rust
// runner/mod.rsから多数のモジュールをインポート
use nyash_rust::cli::CliConfig;
use nyash_rust::backend::*;
use nyash_rust::runtime::*;
use nyash_rust::parser::*;
use nyash_rust::mir::*;
```

**改善提案**:
- runnerをファサードパターンとして明確化
- 各機能をサブモジュールに委譲（execution/, pipeline/等）

**影響範囲**: 中（runner内部構造）
**優先度**: 中（保守性向上）

---

### 2.3 pub(crate)で十分なのにpubになっているもの

#### 調査方法
```bash
# pub use を含む行数
grep -r "^pub use" src/lib.rs src/backend/mod.rs src/mir/mod.rs src/runtime/mod.rs | wc -l
# 結果: 48行
```

**問題点**:
- `src/lib.rs`で48個の型を再エクスポート
- 実際には内部実装の詳細を公開している可能性

**改善提案**:
```rust
// src/lib.rs
// 公開APIを明確化
pub use ast::{ASTNode, BinaryOperator, LiteralValue};
pub use box_trait::{NyashBox, StringBox, IntegerBox, BoolBox};
pub use mir::{MirModule, MirFunction, MirInstruction};
pub use backend::{VM, VMValue, VMError};

// 内部実装は pub(crate) に変更
pub(crate) use box_arithmetic::*;
pub(crate) use box_operators::*;
```

**影響範囲**: 小（外部APIのみ）
**優先度**: 低（既存コードへの影響小）

---

## 3. モジュール階層の改善

### 3.1 ネストが深すぎる

#### 🔴 `src/runtime/plugin_loader_v2/enabled/loader/`

**現状の問題**:
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
        └── loader/            # ここが深すぎる！
            ├── mod.rs
            ├── specs.rs
            ├── metadata.rs
            ├── singletons.rs
            ├── library.rs
            ├── config.rs
            └── util.rs
```

**問題点**:
- 5階層のネスト（`runtime/plugin_loader_v2/enabled/loader/specs.rs`）
- インポートパスが冗長: `crate::runtime::plugin_loader_v2::enabled::loader::specs`

**改善提案**:
```
runtime/
└── plugins/              # plugin_loader_v2 → plugins
    ├── mod.rs
    ├── stub.rs           # プラグイン無効時
    ├── core/             # enabled → core
    │   ├── globals.rs
    │   ├── method_resolver.rs
    │   ├── types.rs
    │   └── bridge/       # FFI/ブリッジ処理
    │       ├── ffi_bridge.rs
    │       └── host_bridge.rs
    ├── loader/           # 1階層上げる
    │   ├── specs.rs
    │   ├── metadata.rs
    │   ├── singletons.rs
    │   └── library.rs
    └── config.rs         # トップレベルへ
```

**影響範囲**: 大（runtime全体）
**優先度**: 高（Phase 15の目標と一致）

---

### 3.2 フラットすぎる

#### 🟡 `src/` トップレベル (32ディレクトリ + 20ファイル)

**現状の問題**:
```
src/
├── ast.rs              # 単一ファイル
├── box_arithmetic.rs   # 単一ファイル
├── box_operators.rs    # 単一ファイル
├── box_trait.rs        # 単一ファイル
├── channel_box.rs      # 単一ファイル
├── environment.rs      # 単一ファイル
├── exception_box.rs    # 単一ファイル
├── ...                 # 20個の単一ファイル
└── (32ディレクトリ)
```

**問題点**:
- トップレベルに20個の単一ファイルが散在
- 責務別のグルーピングがされていない

**改善提案**:
```
src/
├── core/              # コア型・トレイト
│   ├── ast.rs
│   ├── value.rs
│   ├── types.rs
│   └── environment.rs
├── boxes/             # Box実装（既存）
│   ├── primitives/
│   ├── operators/     # box_operators.rs → ここ
│   └── traits/        # box_trait.rs → ここ
├── frontend/          # フロントエンド
│   ├── tokenizer/
│   ├── parser/
│   └── syntax/
├── middle/            # 中間表現
│   ├── mir/
│   └── semantics/
├── backend/           # バックエンド（既存）
├── runtime/           # ランタイム（既存）
└── runner/            # 実行コーディネーター（既存）
```

**影響範囲**: 大（全体構造変更）
**優先度**: 中（長期的改善）

---

### 3.3 論理的なグルーピングができていない

#### 🟡 `src/mir/` の構造

**現状**:
```
mir/
├── basic_block.rs
├── builder.rs
├── definitions.rs
├── effect.rs
├── function.rs
├── instruction.rs
├── instruction_kinds/
├── instruction_introspection.rs
├── types.rs
├── loop_api.rs
├── loop_builder.rs
├── ssot/
├── optimizer.rs
├── optimizer_passes/
├── optimizer_stats.rs
├── passes/
├── printer.rs
├── printer_helpers.rs
├── function_emission.rs
├── hints.rs
├── slot_registry.rs
├── value_id.rs
├── verification.rs
├── verification_types.rs
├── utils/
└── phi_core/
```

**問題点**:
- 112ファイルがフラット寄りに配置
- builder/optimizer/verification等の関連ファイルが散在

**改善提案**:
```
mir/
├── core/              # コア定義
│   ├── instruction.rs
│   ├── basic_block.rs
│   ├── function.rs
│   ├── value_id.rs
│   └── types.rs
├── builder/           # MIRビルダー
│   ├── mod.rs
│   ├── builder_calls.rs
│   ├── loop_builder.rs
│   └── phi_core/
├── optimizer/         # 最適化
│   ├── mod.rs
│   ├── passes/
│   └── stats.rs
├── verification/      # 検証
│   ├── mod.rs
│   └── types.rs
├── emission/          # コード生成
│   ├── printer.rs
│   └── function_emission.rs
└── utils/             # ユーティリティ
    └── control_flow.rs
```

**影響範囲**: 中（mir/内部）
**優先度**: 中（整理済みの部分もある）

---

## 4. 命名・配置の改善

### 4.1 わかりにくい名前

#### 🔴 `src/llvm_py/` (Pythonコード)

**問題点**:
- Rustプロジェクトに`llvm_py/`という名前は混乱を招く
- Pythonコードが`src/`直下にあるのは非標準

**改善提案**:
```
# Option 1: scriptsへ移動
scripts/
└── llvm/
    ├── llvm_builder.py
    └── (その他Pythonファイル)

# Option 2: 別リポジトリ/サブプロジェクト化
nyash-llvm-backend/
└── (Pythonプロジェクト)
```

**影響範囲**: 中（ビルドスクリプト・実行経路）
**優先度**: 中（Phase 15でLLVMハーネス整理時に対応）

---

#### 🟡 `src/runner_plugin_init.rs`

**問題点**:
- トップレベルに配置されているが、runner/専用のコード
- 命名が汎用的すぎる（何のinitか不明）

**改善提案**:
```
src/runner/
└── plugins.rs  # または plugin_init.rs
```

**影響範囲**: 小（runner内部のみ）
**優先度**: 低（機能的には問題なし）

---

### 4.2 配置が不適切なモジュール

#### 🟡 `src/scope_tracker.rs`

**現状**: トップレベルに配置

**問題点**:
- VMバックエンド専用のコード（コメント: "Box lifecycle tracking for VM"）
- backend/mir_interpreter/と密結合

**改善提案**:
```
src/backend/mir_interpreter/
└── scope_tracker.rs
```

**影響範囲**: 小（backend内部のみ）
**優先度**: 低（機能的には問題なし）

---

#### 🟡 `src/abi/nyrt_shim.rs`

**現状**: abi/配下だが、実際にはC-ABI PoC用のシム

**改善提案**:
```
src/backend/
└── abi/
    └── nyrt_shim.rs
```

または

```
src/runtime/
└── abi/
    └── nyrt_shim.rs
```

**影響範囲**: 小（ABI関連のみ）
**優先度**: 低（現状でも明確）

---

### 4.3 utils/commonの肥大化

#### 🔴 utils/common/helpersの散在

**現状**:
```
src/
├── box_operators/helpers.rs
├── backend/mir_interpreter/helpers.rs
├── parser/common.rs
├── parser/statements/helpers.rs
├── parser/declarations/box_def/members/common.rs
├── mir/utils/
├── mir/phi_core/common.rs
└── runner/modes/common.rs (14,000行!)
```

**問題点**:
- helpers/common/utilsが各モジュールに散在
- 命名が汎用的で役割不明

**改善提案**:

**原則**:
- `helpers.rs` → 具体的な名前（例: `arithmetic_helpers.rs`）
- `common.rs` → 責務別に分割（例: `shared_types.rs`, `constants.rs`）
- `utils/` → 明確な責務（例: `control_flow.rs`, `string_utils.rs`）

**具体例**:
```
# Before
parser/common.rs

# After
parser/
├── shared_types.rs    # 共有型定義
├── constants.rs       # パーサー定数
└── cursor_helpers.rs  # TokenCursor操作
```

**影響範囲**: 中（各モジュール内部）
**優先度**: 中（可読性向上）

---

## 5. 改善提案の優先度付け

### 🔥 優先度: 緊急（Phase 15目標と直結）

1. **`runner/modes/common.rs`分割** (14,000行)
   - Phase 15のセルフホスティング整理に必須
   - 影響範囲: 大
   - 見積もり: 3-5日

2. **`backend/mir_interpreter/handlers/calls.rs`分割** (49,750行)
   - 最大ファイルの分割は可読性に直結
   - 影響範囲: 中
   - 見積もり: 2-3日

3. **`runtime/plugin_loader_v2/`階層整理**
   - ネストが深すぎる問題の解消
   - 影響範囲: 大
   - 見積もり: 2-3日

---

### ⚡ 優先度: 高（可読性・保守性向上）

4. **BID関連モジュール命名統一**
   - `bid-*` → `bid/`配下へ統合
   - 影響範囲: 中
   - 見積もり: 1-2日

5. **`boxes/`カテゴリ再編**
   - 61ファイルを責務別にグルーピング
   - 影響範囲: 中
   - 見積もり: 2-3日

6. **`runner/modes/common_util/`再構成**
   - using解決・セルフホスト実行を明確に分離
   - 影響範囲: 中
   - 見積もり: 1-2日

---

### 🔵 優先度: 中（長期的改善）

7. **`mir/`モジュールのグルーピング強化**
   - builder/optimizer/verification等を明確化
   - 影響範囲: 中
   - 見積もり: 2-3日

8. **helpers/common/utils命名統一**
   - 汎用名から具体的な名前へ
   - 影響範囲: 小〜中
   - 見積もり: 1-2日

9. **トップレベルファイルの整理**
   - 20個の単一ファイルをcore/等に移動
   - 影響範囲: 大（全体構造）
   - 見積もり: 3-5日

---

### 🟢 優先度: 低（Nice to have）

10. **`llvm_py/`の配置再検討**
    - scripts/またはサブプロジェクト化
    - 影響範囲: 中
    - 見積もり: 1日

11. **pub/pub(crate)の最適化**
    - 公開APIの明確化
    - 影響範囲: 小
    - 見積もり: 1日

12. **`scope_tracker.rs`等の配置最適化**
    - 専用モジュールへ移動
    - 影響範囲: 小
    - 見積もり: 0.5日

---

## 6. リファクタリングの影響範囲マトリックス

| 改善項目 | コンパイル影響 | テスト影響 | ドキュメント影響 | 外部API影響 |
|---------|------------|----------|--------------|-----------|
| handlers分割 | 小 | 小 | 小 | なし |
| runner/modes分割 | 中 | 中 | 中 | なし |
| plugin_loader階層 | 大 | 中 | 大 | 小 |
| BID命名統一 | 中 | 小 | 小 | なし |
| boxes再編 | 中 | 小 | 中 | 小 |
| mir/グルーピング | 中 | 小 | 中 | なし |
| トップレベル整理 | 大 | 中 | 大 | 中 |

---

## 7. 理想的なモジュール構造案

### 7.1 最終目標構造

```
src/
├── core/              # コア型・トレイト・環境
│   ├── ast.rs
│   ├── value.rs
│   ├── types.rs
│   └── environment.rs
│
├── frontend/          # フロントエンド
│   ├── tokenizer/
│   ├── parser/
│   │   ├── expressions/
│   │   ├── statements/
│   │   └── declarations/
│   ├── syntax/        # 構文糖
│   └── macro/         # マクロシステム
│
├── middle/            # 中間表現
│   ├── mir/
│   │   ├── core/      # 命令・関数・ブロック
│   │   ├── builder/   # MIRビルダー
│   │   ├── optimizer/ # 最適化
│   │   ├── verification/ # 検証
│   │   └── emission/  # コード生成
│   └── semantics/     # 意味解析
│
├── backend/           # バックエンド
│   ├── vm/            # Rust VM
│   │   ├── core/
│   │   ├── handlers/  # 命令ハンドラ
│   │   └── scope_tracker.rs
│   ├── llvm/          # LLVM（Rust側）
│   ├── wasm/          # WASM
│   └── abi/           # C-ABI
│
├── runtime/           # ランタイムシステム
│   ├── plugins/       # プラグインシステム
│   │   ├── core/
│   │   ├── loader/
│   │   └── config.rs
│   ├── gc/            # GCシステム
│   ├── scheduler/     # スケジューラ
│   └── registry/      # Box/型レジストリ
│
├── boxes/             # Box実装
│   ├── primitives/    # String/Integer/Bool
│   ├── collections/   # Array/Map/Buffer
│   ├── io/            # File/HTTP/Stream
│   ├── system/        # Console/Debug/Time
│   ├── advanced/      # JSON/Regex/Future
│   └── platform/      # Web/Audio/GUI
│
├── runner/            # 実行コーディネーター
│   ├── execution/     # 実行モード
│   ├── pipeline/      # パイプライン処理
│   ├── resolution/    # using/namespace解決
│   ├── selfhost/      # セルフホスト実行
│   └── io/            # 入出力
│
├── cli/               # CLIシステム
├── config/            # 設定システム
├── debug/             # デバッグ支援
├── using/             # using resolver
├── host_providers/    # ホストプロバイダ
│
├── lib.rs             # ライブラリエントリーポイント
└── main.rs            # 実行可能ファイルエントリーポイント

scripts/               # ビルドスクリプト（Rustの外）
└── llvm/
    └── (Pythonコード)
```

---

## 8. 段階的移行戦略

### Phase 1: 緊急対応（1-2週間）
1. `handlers/calls.rs`分割（49,750行）
2. `runner/modes/common.rs`分割（14,000行）
3. `runtime/plugin_loader_v2/`階層整理

### Phase 2: 高優先度（2-3週間）
4. BID関連モジュール統一
5. `boxes/`カテゴリ再編
6. `runner/modes/common_util/`再構成

### Phase 3: 中優先度（3-4週間）
7. `mir/`グルーピング強化
8. helpers/common/utils命名統一
9. `llvm_py/`配置再検討

### Phase 4: 長期改善（時期未定）
10. トップレベルファイル整理（全体構造変更）
11. pub/pub(crate)最適化
12. 細かい配置最適化

---

## 9. 成功メトリクス

### 定量的指標
- [ ] 1,000行超のファイル数: **現在20+** → **目標: 5以下**
- [ ] 平均ファイル行数: **現在500-1000** → **目標: 300以下**
- [ ] モジュール階層深度: **最大5階層** → **目標: 3階層以下**
- [ ] utils/common/helpersファイル数: **現在15+** → **目標: 5以下**

### 定性的指標
- [ ] 新規開発者が30分以内にモジュール構造を理解できる
- [ ] 単一ファイル内で完結する変更が80%以上
- [ ] テストの局所性が向上（モジュール単位でテスト可能）
- [ ] ドキュメントの保守コスト削減

---

## 10. リスク管理

### 高リスク項目
- **トップレベル構造変更**: 全体に影響、慎重な移行が必要
- **plugin_loader階層変更**: 多くのインポートパスが変更

### リスク軽減策
- 段階的移行（Phase 1→4）
- 各Phaseでコンパイル・テスト確認
- deprecation警告期間の設定（pub use経由で旧パス維持）
- ドキュメント同時更新

---

## 11. 参考資料

### Rustのモジュール設計ベストプラクティス
- [The Rust Programming Language - Modules](https://doc.rust-lang.org/book/ch07-00-managing-growing-projects-with-packages-crates-and-modules.html)
- [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)

### 類似プロジェクトの構造
- **rustc**: frontend/middle/backend分離
- **cargo**: src/cargo/{core,ops,sources,util}
- **tokio**: tokio/{runtime,task,sync,io}

---

## 12. まとめ

Hakoruneコードベースは概ね良好に整理されていますが、以下の3点が主要な改善機会です：

1. **巨大ファイルの分割** - calls.rs (49,750行)、common.rs (14,000行)
2. **階層の最適化** - plugin_loader_v2の深すぎるネスト
3. **命名・配置の統一** - BID関連、helpers/common/utils

Phase 15のセルフホスティング目標と連携し、段階的に改善を進めることで、
**可読性・保守性・拡張性**の大幅な向上が期待できます。

---

**次のアクション**:
1. このレポートをチームでレビュー
2. Phase 1の3項目を優先実施
3. 各Phase完了後に成功メトリクスを測定
