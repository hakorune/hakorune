# Nyash Environment Variables (管理棟ガイド)

本ドキュメントは Nyash の環境変数を用途別に整理し、最小限の運用セットを提示します。`nyash.toml` の `[env]` で上書き可能（起動時に適用）。

- 例: `nyash.toml`
```
[env]
NYASH_JIT_THRESHOLD = "1"
NYASH_CLI_VERBOSE = "1"
NYASH_DISABLE_PLUGINS = "1"
```

起動時に `nyash` は `[env]` の値を `std::env` に適用します（src/config/env.rs）。

## コア運用セット（最小）
- NYASH_CLI_VERBOSE: CLI の詳細ログ（"1" で有効）
- NYASH_DISABLE_PLUGINS: 外部プラグインを無効化（CI/再現性向上）
- NYASH_BOX_FACTORY_POLICY: Box生成の優先順位制御（Phase 21.4推奨ENV）
  - `builtin_first`: Builtin優先（Analyzer推奨、デフォルト）
  - `strict_plugin_first`: Plugin最優先（開発・本番環境）
  - `compat_plugin_first`: Plugin > Builtin > User（互換モード）

## JIT（共通）
- NYASH_JIT_THRESHOLD: JIT 降下開始の閾値（整数）
- NYASH_JIT_EXEC: JIT 実行（"1" で有効）
- NYASH_JIT_HOSTCALL: ホストコール経路の有効化
- NYASH_JIT_PHI_MIN: PHI(min) 合流の最適化ヒント
- NYASH_JIT_NATIVE_F64: f64 のネイティブ ABI 利用（実験的）
- NYASH_JIT_NATIVE_BOOL: bool のネイティブ ABI 利用（実験的）
- NYASH_JIT_ABI_B1: B1 返り値 ABI を要求（実験的）
- NYASH_JIT_RET_B1: bool 返り値ヒント（実験的）

## JIT トレース/ダンプ
- NYASH_JIT_DUMP: JIT IR/CFG ダンプ（"1" で有効）
- NYASH_JIT_DOT: DOT 出力先ファイル指定でダンプ暗黙有効
- NYASH_JIT_TRACE_BLOCKS: ブロック入場ログ
- NYASH_JIT_TRACE_BR: 条件分岐ログ
- NYASH_JIT_TRACE_SEL: select のログ
- NYASH_JIT_TRACE_RET: return 経路のログ
- NYASH_JIT_EVENTS_COMPILE: コンパイルイベント JSONL を出力
- NYASH_JIT_EVENTS_PATH: イベント出力パス（既定: events.jsonl）

## Async/Runtime
- NYASH_AWAIT_MAX_MS: await の最大待機ミリ秒（既定 5000）
- （今後）タスク/スケジューラ関連の変数は `runtime.*` 名で集約予定

## CLI Script Args（改行・特殊文字の安全輸送）
- NYASH_SCRIPT_ARGS_JSON: `--` 以降のスクリプト引数（JSON配列）。標準経路。
- NYASH_SCRIPT_ARGS_HEX_JSON: 上記のHEX版（各要素をUTF‑8→16進文字列化）。VMは HEX→JSON→ARGV の順で復元を試みる。
- NYASH_ARGV: 互換目的のJSON配列（最終フォールバック）。

メモ: 改行・特殊文字を含む長文を `--source-file <path> <text>` で渡す場合も HEX 経路で安全に輸送される。

## Debug/Tracing（開発用の軽量トグル）
- HAKO_TRACE_EXECUTION: 実行経路の可視化（"1" で有効）
  - 例: `[trace] executor: hv1_inline (rust)` / `[trace] executor: hakovm (hako)` / `[trace] executor: core (rust)`
  - 出力先: 原則 stderr（Hakovm は stdout）。テストランナーは numeric rc 抽出時に非数値行を無視します。
- HAKO_VERIFY_SHOW_LOGS: verify_v1_inline_file() のログ透過（"1" で有効）
  - hv1 inline 実行の全出力を stderr に表示し、数値 rc は別途抽出します。
- HAKO_DEBUG: 開発向け一括トグル（"1" で `HAKO_TRACE_EXECUTION=1` と `HAKO_VERIFY_SHOW_LOGS=1` を自動有効化）
  - 実装箇所: tools/smokes/v2/lib/test_runner.sh のデバッグ便宜機能

## FileBox Provider（コア/プラグイン切替）
- NYASH_FILEBOX_MODE: `auto|core-ro|plugin-only`
  - auto（既定）: プラグインがあれば PluginFileIo、無ければ CoreRoFileIo
  - core-ro: 常にコアの read‑only 実装を使用（Analyzer/CI 向け）
  - plugin-only: プラグイン必須（無ければ Fail‑Fast）
- NYASH_DISABLE_PLUGINS / HAKO_DISABLE_PLUGINS: `1` でプラグイン無効（結果として core‑ro 相当）

## LLVM/AOT
- NYASH_LLVM_FEATURE: LLVM機能選択（"llvm"(default) または "llvm-inkwell-legacy"）
- LLVM_SYS_180_PREFIX: LLVM 18 のパス指定（llvm-inkwell-legacy使用時のみ必要）
- NYASH_LLVM_VINVOKE_RET_SMOKE, NYASH_LLVM_ARRAY_RET_SMOKE: CI 用スモークトグル
- NYASH_LLVM_OBJ_OUT: LLVM経路で生成する `.o` の出力パス（Runner/スクリプトが尊重）
- NYASH_AOT_OBJECT_OUT: AOT パイプラインで使用する `.o` 出力ディレクトリ/パス
- NYASH_LLVM_USE_HARNESS: "1" で llvmlite ハーネス経路を有効化（MIR(JSON)→Python→.ll→llc→.o）

## AotPrep / Numeric Core（Phase 25 実験用）
- NYASH_AOT_COLLECTIONS_HOT: Array/Map boxcall を externcall に書き換えるホットパス（CollectionsHot パスを有効化）
- NYASH_AOT_NUMERIC_CORE: 数値系 BoxCall（MatI64 / IntArrayCore 等）の診断パスを有効化（`AotPrepNumericCoreBox`）。現状はログ出力のみで MIR は変更しない（将来の BoxCall→Call 降ろし用の足場）。
- NYASH_AOT_NUMERIC_CORE_TRACE: 上記 numeric core パスの詳細トレース（"1" で `mul_naive/at/set` など候補 BoxCall を stderr にログ出力）

### LLVM Feature 詳細
- **llvm** (デフォルト): llvmlite Python ハーネス使用、LLVM_SYS_180_PREFIX不要
- **llvm-inkwell-legacy**: Rust inkwell bindings使用、LLVM_SYS_180_PREFIX必要

## 管理方針（提案）
- コード側: `src/config/env.rs` を単一の集約窓口に（JIT は `jit::config` に委譲）。
- ドキュメント側: 本ファイルを単一索引にし、用途別に追加。
- 設定ファイル: `nyash.toml` の `[env]` で標準化（ブランチ/CI での一括制御）。
- 将来: `nyash env print/set` の CLI サブコマンドを追加し、実行前に `.env`/toml 反映と検証を行う。

## MIR Cleanup (Phase 11.8) 用トグル（段階導入）
- NYASH_MIR_ARRAY_BOXCALL: ArrayGet/Set → BoxCall 変換を有効化
- NYASH_MIR_REF_BOXCALL: RefGet/Set → BoxCall 変換を有効化
- NYASH_MIR_CORE13: Core‑13 セットの一括有効（将来拡張）
- NYASH_MIR_CORE13_PURE: Core‑13 純化モード（"1" で有効）。最終MIRは13命令のみ許可され、Load/Store などは `env.local.get/set`、`new` は `env.box.new` 経由へ強制正規化。禁制命令が残存するとコンパイルエラーで早期失敗。

## 非推奨ENV変数（Phase 21.4で段階的削除）

以下の環境変数は **非推奨** です。新しい統一ENV変数を使用してください：

### ❌ NYASH_USE_PLUGIN_BUILTINS（削除予定）
- **理由**: 自動設定による混乱、新しいポリシーシステムで代替
- **代替**: `NYASH_BOX_FACTORY_POLICY=strict_plugin_first` または `compat_plugin_first`
- **状態**: Phase 21.4で自動設定を削除、警告メッセージを表示
- **完全削除**: Phase 22で参照も含めて完全削除予定

### ❌ NYASH_PLUGIN_OVERRIDE_TYPES（削除予定）
- **理由**: 自動設定による不整合、FactoryPolicy で統一管理
- **代替**: `NYASH_BOX_FACTORY_POLICY` で全体的な優先順位を制御
- **状態**: Phase 21.4で自動設定を削除、警告メッセージを表示
- **完全削除**: Phase 22で参照も含めて完全削除予定

### 移行ガイド

#### Before (非推奨)
```bash
# ❌ 古い方法（自動設定に依存）
NYASH_USE_PLUGIN_BUILTINS=1 ./target/release/nyash program.hako
NYASH_PLUGIN_OVERRIDE_TYPES="ArrayBox,MapBox" ./target/release/nyash program.hako
```

#### After (推奨)
```bash
# ✅ 新しい方法（統一ポリシー）
# Analyzer環境（Builtin優先）
NYASH_BOX_FACTORY_POLICY=builtin_first ./target/release/nyash program.hako

# 開発・本番環境（Plugin優先）
NYASH_BOX_FACTORY_POLICY=strict_plugin_first ./target/release/nyash program.hako

# プラグイン完全無効（CI/検証）
NYASH_DISABLE_PLUGINS=1 ./target/release/nyash program.hako
```

#### FileBox専用制御
```bash
# ✅ FileBox providerの明示的制御
NYASH_FILEBOX_MODE=auto ./target/release/nyash program.hako        # 自動選択（デフォルト）
NYASH_FILEBOX_MODE=core-ro ./target/release/nyash program.hako     # Builtin core-ro固定
NYASH_FILEBOX_MODE=plugin-only ./target/release/nyash program.hako # Plugin必須（Fail-Fast）
```
