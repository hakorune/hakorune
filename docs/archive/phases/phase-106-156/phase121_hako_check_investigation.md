# Phase 121: hako_check 現状調査結果

## 実行日時

2025-12-04（Phase 120 完了直後）

## 調査項目 1: エントリーポイント

### 1.1 シェルスクリプトエントリー

**ファイル**: `tools/hako_check.sh`

**実装内容**:
```bash
#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
BIN="${NYASH_BIN:-$ROOT/target/release/hakorune}"

# ... 環境変数設定 ...

"$BIN" --backend vm "$ROOT/tools/hako_check/cli.hako" -- --source-file "$f" "$text"
```

**重要な発見**: hako_check は **Rust バイナリではなく .hako スクリプト**として実装されている。

**環境変数**:
- `NYASH_DISABLE_PLUGINS=1`: プラグイン無効化（安定性優先）
- `NYASH_BOX_FACTORY_POLICY=builtin_first`: ビルトイン Box 優先
- `NYASH_DISABLE_NY_COMPILER=1`: Ny コンパイラ無効化
- `HAKO_DISABLE_NY_COMPILER=1`: Hako コンパイラ無効化
- `NYASH_FEATURES=stage3`: Stage-3 パーサー使用
- `NYASH_PARSER_SEAM_TOLERANT=1`: パーサー seam 許容
- `HAKO_PARSER_SEAM_TOLERANT=1`: Hako パーサー seam 許容
- `NYASH_PARSER_ALLOW_SEMICOLON=1`: セミコロン許容
- `NYASH_ENABLE_USING=1`: using 文有効化
- `HAKO_ENABLE_USING=1`: Hako using 文有効化
- `NYASH_USING_AST=1`: AST ベース using 解決
- `NYASH_NY_COMPILER_TIMEOUT_MS`: コンパイラタイムアウト（デフォルト 8000ms）

**コマンドライン引数**:
- `--backend vm`: VM バックエンド強制
- `--source-file <path> <text>`: ファイルパスと内容をインライン渡し
- `--format <text|dot|json-lsp>`: 出力フォーマット指定

### 1.2 .hako エントリーポイント

**ファイル**: `tools/hako_check/cli.hako`

**実装内容**:
```hako
static box HakoAnalyzerBox {
  run(args) {
    // ... 引数解析 ...

    // IR 生成
    ir = HakoAnalysisBuilderBox.build_from_source_flags(text, p, no_ast_eff)

    // 診断ルール実行（HC001-HC031）
    // ...
  }
}

static box Main { method main(args) { return HakoAnalyzerBox.run(args) } }
```

**重要なポイント**:
- エントリーポイントは `Main.main()` メソッド
- `HakoAnalyzerBox.run()` で診断ロジックを実行
- IR 生成は `HakoAnalysisBuilderBox.build_from_source_flags()` を使用

## 調査項目 2: MIR 生成経路

### 2.1 IR 生成フロー

**ファイル**: `tools/hako_check/analysis_consumer.hako`

**使用している MirBuilder**: 間接的に使用（VM 内部）

**フロー**:
```
HakoAnalysisBuilderBox.build_from_source_flags()
  ↓
HakoParserCoreBox.parse(text)  // .hako パーサーで AST 生成
  ↓
[Rust VM 内部]
  ↓
nyash_rust::parser::NyashParser::parse_from_string_with_fuel()
  ↓
MirCompiler::compile_with_source()
  ↓
MirBuilder::build_module(ast)
  ↓
[If/Loop 文の処理]
  ↓
MirBuilder::cf_if() / MirBuilder::cf_loop()
```

**呼び出し箇所**:
- `tools/hako_check/analysis_consumer.hako:32`: `HakoParserCoreBox.parse(text)`
- `src/runner/modes/common.rs:112`: `NyashParser::parse_from_string_with_fuel()`
- `src/mir/mod.rs:120`: `MirBuilder::build_module(ast)`

### 2.2 JoinIR 統合状況

**✅ 部分的に統合**: Loop の一部関数のみ JoinIR 経由

**調査結果**:

| 構文 | JoinIR 統合状況 | 実装箇所 | 制御方法 |
|------|----------------|---------|---------|
| **Loop** | ⚠️ 部分統合 | `src/mir/builder/control_flow.rs` | `HAKO_JOINIR_*_MAIN=1` |
| **If** | ❌ 未統合 | `src/mir/builder/if_form.rs` | なし |

**Loop の JoinIR 統合詳細**:

```rust
// src/mir/builder/control_flow.rs:L156-L200
fn try_cf_loop_joinir(&mut self, condition: &ASTNode, body: &ASTNode) -> Result<Option<ValueId>, String> {
    let core_on = crate::config::env::joinir_core_enabled();

    // Mainline targets: print_tokens, filter
    if core_on && is_mainline_target(&func_name) {
        // JoinIR Frontend を試す
        return self.cf_loop_joinir_impl(condition, body, &func_name, debug);
    }

    // フォールバック: 旧 LoopBuilder
    Ok(None)
}
```

**If の現状**:

```rust
// src/mir/builder/if_form.rs
// JoinIR 統合なし、旧 PHI 生成器を使用中
```

## 調査項目 3: PHI 生成経路

### 3.1 If 文の PHI 生成

**ファイル**: `src/mir/builder/if_form.rs`

**使用経路**: ❌ **旧 If Builder**（JoinIR 未統合）

**実装**:
```rust
// src/mir/builder/if_form.rs
pub fn cf_if(
    &mut self,
    condition: &ASTNode,
    then_block: &ASTNode,
    else_block: Option<&ASTNode>,
) -> Result<ValueId, String> {
    // 旧 PHI 生成ロジック
    // Phase 33-10 で JoinIR If Lowering が実装されたが、
    // MirBuilder 経路ではまだ統合されていない
}
```

**問題点**:
- Phase 33-10 で JoinIR If Lowering が実装済み（`src/mir/join_ir/lowering/if_select.rs`）
- しかし MirBuilder の `cf_if()` はまだ旧経路を使用
- hako_check で If 文を含むコードを解析する際、旧 PHI 生成器が使われる

### 3.2 Loop の PHI 生成

**ファイル**: `src/mir/builder/control_flow.rs`

**使用経路**: ⚠️ **部分的に JoinIR Loop Lowering**（Phase 49 統合）

**実装**:
```rust
// src/mir/builder/control_flow.rs
pub fn cf_loop(&mut self, condition: &ASTNode, body: &ASTNode) -> Result<ValueId, String> {
    // Phase 49/80: Try JoinIR Frontend route for mainline targets
    if let Some(result) = self.try_cf_loop_joinir(&condition, &body)? {
        return Ok(result);
    }

    // Fallback: 旧 LoopBuilder
    self.cf_loop_legacy(condition, body)
}
```

**Mainline Targets** (JoinIR 経由):
- `JsonTokenizer.print_tokens/0`: `HAKO_JOINIR_PRINT_TOKENS_MAIN=1`
- `ArrayExtBox.filter/2`: `HAKO_JOINIR_ARRAY_FILTER_MAIN=1`

**その他の Loop**: 旧 LoopBuilder へフォールバック

**JoinIR Frontend の実装箇所**:
- `src/mir/join_ir/frontend/mod.rs`: `AstToJoinIrLowerer`
- `src/mir/join_ir/lowering/loop_*.rs`: Loop Lowering 実装

## 調査項目 4: 環境変数・フラグ

### 4.1 検索コマンド

```bash
rg "NYASH_HAKO_CHECK" --type rust
rg "NYASH_JOINIR" --type rust | grep -i "hako_check"
```

### 4.2 発見された環境変数

#### hako_check 専用環境変数

| 環境変数 | 用途 | 設定箇所 |
|---------|-----|---------|
| `NYASH_DISABLE_PLUGINS=1` | プラグイン無効化 | `tools/hako_check.sh` |
| `NYASH_BOX_FACTORY_POLICY=builtin_first` | ビルトイン Box 優先 | `tools/hako_check.sh` |
| `NYASH_DISABLE_NY_COMPILER=1` | Ny コンパイラ無効化 | `tools/hako_check.sh` |
| `HAKO_DISABLE_NY_COMPILER=1` | Hako コンパイラ無効化 | `tools/hako_check.sh` |
| `NYASH_FEATURES=stage3` | Stage-3 パーサー使用 | `tools/hako_check.sh` |
| `NYASH_JSON_ONLY=1` | JSON 出力のみ | `tools/hako_check.sh` |

#### JoinIR 関連環境変数

| 環境変数 | 用途 | 実装箇所 |
|---------|-----|---------|
| `NYASH_JOINIR_STRICT=1` | フォールバック禁止 | `src/config/env.rs` |
| `NYASH_JOINIR_CORE=1` | JoinIR Core 有効化 | `src/config/env.rs` |
| `HAKO_JOINIR_PRINT_TOKENS_MAIN=1` | print_tokens を JoinIR 経由 | `src/mir/builder/control_flow.rs` |
| `HAKO_JOINIR_ARRAY_FILTER_MAIN=1` | ArrayExt.filter を JoinIR 経由 | `src/mir/builder/control_flow.rs` |
| `NYASH_JOINIR_MAINLINE_DEBUG=1` | JoinIR Mainline デバッグログ | `src/mir/builder/control_flow.rs` |
| `NYASH_JOINIR_EXPERIMENT=1` | JoinIR 実験モード | `src/tests/helpers/joinir_env.rs` |

#### hako_check で使用されていない JoinIR 変数

**Phase 121 調査結果**: hako_check は現在 JoinIR 関連環境変数を使用していない。

**理由**:
- hako_check.sh で `NYASH_DISABLE_NY_COMPILER=1` を設定
- JoinIR 統合は VM の MirBuilder に実装されているが、環境変数での制御がない
- Phase 122 で `NYASH_HAKO_CHECK_JOINIR=1` を導入予定

## Phase 122+ への提言

### 優先度高（Phase 122 実装必須）

- [ ] **`NYASH_HAKO_CHECK_JOINIR=1` 環境変数追加**: hako_check で JoinIR 経路を有効化
- [ ] **If 文の JoinIR 統合**: `src/mir/builder/if_form.rs` の `cf_if()` を JoinIR 対応
- [ ] **Loop の JoinIR 統合拡張**: Mainline Targets 以外も JoinIR 経由に

### 優先度中（Phase 123 実装推奨）

- [ ] **デフォルト変更**: JoinIR 経路をデフォルトに
- [ ] **`NYASH_LEGACY_PHI=1` 環境変数追加**: 旧経路への明示的切り替え
- [ ] **警告メッセージ追加**: 旧経路使用時に非推奨警告

### 優先度低（Phase 124 クリーンアップ）

- [ ] **旧経路削除**: `if_form.rs` / `control_flow.rs` の旧 PHI 生成ロジック削除
- [ ] **環境変数削除**: `NYASH_LEGACY_PHI=1` サポート削除
- [ ] **ドキュメント更新**: 旧経路に関する記述を全削除

## 結論

hako_check 経路の現状は：

### ✅ **良好な点**

1. **安定した実装**: .hako スクリプトとして実装され、VM で安定動作
2. **環境変数制御**: 明確な環境変数で動作を制御
3. **部分統合開始**: Loop の Mainline Targets で JoinIR 統合実績あり

### ❌ **課題**

1. **If 文未統合**: If 文は旧 PHI 生成器を使用中
2. **Loop 部分統合**: Mainline Targets 以外は旧 LoopBuilder にフォールバック
3. **環境変数未整備**: JoinIR 経路を選択する統一的な環境変数がない

### ⚠️ **注意点**

1. **.hako スクリプト**: hako_check は Rust バイナリではないため、VM の MirBuilder に依存
2. **VM 経路のみ**: hako_check は常に `--backend vm` を使用
3. **プラグイン無効**: `NYASH_DISABLE_PLUGINS=1` で安定性優先

### 次のステップ

Phase 122+ で上記課題を段階的に解決する。特に **If 文の JoinIR 統合**が最優先課題。
Status: Historical
