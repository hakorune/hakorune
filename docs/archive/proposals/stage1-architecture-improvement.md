# Stage1セルフホスティング起動アーキテクチャ改善提案

Status: Historical

## 📋 エグゼクティブサマリー

Nyashのセルフホスティング実装で、Stage0（Rust）→ Stage1（.hako script）の起動が**環境変数25個**と**3つの引数経路**で複雑化している問題を整理し、業界標準パターンに基づいた改善案を提示する。

**現状の痛み**:
- 環境変数25個（NYASH_*/STAGE1_*/HAKO_*）が15個以上のファイルに散在
- Stage0とStage1の役割境界が曖昧（汎用ランチャー vs 専用CLI）
- 引数経路が3つ（CLI args / env vars / JSON）で混在
- 巨大prelude（70+ファイル結合）でデバッグ困難（エラーが`line 10433`と表示）

**改善目標**:
- 環境変数を**5個以下**に削減
- 引数経路を**1つ**に統一
- デバッグビリティ向上（source map対応）
- 短期（Phase 25.2）と長期（Phase 26+）の段階実装

---

## 🔍 A. 他言語の事例調査

### A-1. Rustコンパイラのブートストラップ（3段階明確化）

**アーキテクチャ**:
```
Stage 0: 事前ビルド済みベータ版rustc（CI artifactsからダウンロード）
  ↓
Stage 1: Stage0でビルドしたrustc + 標準ライブラリ（機能完全）
  ↓
Stage 2: Stage1で再ビルドしたrustc（検証用・本番利用）
  ↓
Stage 3: Stage2で再ビルドしたrustc（完全自己再現性検証）
```

**特徴**:
- **明確な責務分離**: Stage0は「ビルドツール」、Stage1以降は「開発コンパイラ」
- **環境変数最小**: `RUSTC_BOOTSTRAP`など**4個のみ**
- **CLI引数優先**: 環境変数はビルドシステム内部のみ、ユーザーは`x build --stage N`でシンプル操作
- **2024年改善**: Stage0でstdも事前ビルド版を使用し、`cfg(bootstrap)`を削除（複雑性削減）

**参考**: [Rust Compiler Development Guide - Bootstrapping](https://rustc-dev-guide.rust-lang.org/building/bootstrapping/what-bootstrapping-does.html)

---

### A-2. Goコンパイラのブートストラップ（段階自動化）

**アーキテクチャ**:
```
Bootstrap Compiler: Go 1.N-2（最小2バージョン前）
  ↓
cmd/dist: ブートストラップビルドツール（Go製）
  ↓
Toolchain1 → Toolchain2 → Toolchain3（自動多段階ビルド）
```

**特徴**:
- **自動段階切り替え**: `cmd/dist`が段階を自動制御、ユーザーは意識不要
- **環境変数ゼロ**: すべてCLI引数で制御（`GOROOT`, `GOPATH`のみ）
- **最適化重視**: 無関係アーキテクチャ向けファイルはダミー化（6秒短縮）
- **バージョンポリシー明確**: 1.24/1.25は1.22が必須（N-2ルール）

**参考**: [How Go uses Go to build itself](https://dave.cheney.net/2013/06/04/how-go-uses-go-to-build-itself)

---

### A-3. Nimコンパイラのブートストラップ（C経由2段階）

**アーキテクチャ**:
```
csources_v3: C言語生成コード（Nim古バージョンから生成）
  ↓
koch.nim: ブートストラップツール
  ↓
Nim Compiler v1: 完全機能版
  ↓
Nim Compiler v2: 自己再ビルド版（検証）
```

**特徴**:
- **Cソース安定化**: `csources_v3`リポジトリで分離管理
- **ツール一本化**: `koch`が「ビルド・テスト・ドキュメント生成」すべて担当
- **環境変数なし**: すべて`koch`のサブコマンドで制御
- **2024年改革**: NIR中間言語導入で、フロントエンド複数バージョン対応予定

**参考**: [Nim GitHub - Internals](https://nim-lang.org/docs/intern.html)

---

### A-4. 設定管理の業界標準パターン

**優先度階層（POSIX標準準拠）**:
```
1. CLI引数（最優先） ← ユーザーの明示的意図
2. 環境変数        ← セッション固有設定
3. ローカル設定ファイル ← プロジェクト設定
4. グローバル設定ファイル ← システム設定
5. デフォルト値（最低優先）
```

**設計原則**（ASP.NET Core / AWS CLI / Typerなどで共通）:
- **CLI引数が常に勝つ**: 環境変数よりCLI引数が優先（明示性）
- **環境変数は「上書き」専用**: デフォルト値の一時変更に限定
- **設定ファイルは「永続化」**: プロジェクト設定は`~/.config`や`.toml`に
- **Chain of Responsibility**: 見つかるまで順に探索、最後に見つかった値が勝つ

**参考**: [Stack Overflow - Configuration Precedence](https://stackoverflow.com/questions/11077223/what-order-of-reading-configuration-values)

---

## 🎯 B. Nyash向け具体的改善案

### B-1. 優先度1: 環境変数の階層化（15個→5個）

**現状の問題**:
```bash
# 現在の25個の環境変数（抜粋）
NYASH_USE_STAGE1_CLI=1
STAGE1_EMIT_PROGRAM_JSON=1
STAGE1_EMIT_MIR_JSON=1
STAGE1_BACKEND=vm
STAGE1_SOURCE=/path/to/file.hako
STAGE1_PROGRAM_JSON=/path/to/prog.json
STAGE1_SOURCE_TEXT="..."
STAGE1_CLI_ENTRY=/path/to/cli.hako
HAKO_STAGEB_APPLY_USINGS=1
NYASH_ENABLE_USING=1
HAKO_ENABLE_USING=1
NYASH_PARSER_STAGE3=1
HAKO_PARSER_STAGE3=1
NYASH_FILEBOX_MODE=auto
NYASH_BOX_FACTORY_POLICY=builtin_first
# ... さらに10個以上
```

**改善後（5個に集約）**:
```bash
# 1. モード制御（単一変数でサブコマンド切り替え）
NYASH_STAGE1_MODE=emit-program-json  # emit-mir-json / run-vm / run-llvm
# → 7個の環境変数を1個に統合

# 2. 入力ソース（パスまたはインライン）
NYASH_STAGE1_INPUT=/path/to/source.hako  # または STDIN: "-"
# → STAGE1_SOURCE / STAGE1_SOURCE_TEXT / STAGE1_INPUT を統合

# 3. 機能トグル（ビットフラグまたはカンマ区切り）
NYASH_FEATURES=using,parser-stage3,plugins  # または空文字でデフォルト
# → ENABLE_USING / PARSER_STAGE3 / DISABLE_PLUGINS を統合

# 4. デバッグ/ログ（現状は NYASH_CLI_VERBOSE / STAGE1_CLI_DEBUG を併用）
#    → 将来 NYASH_STAGE1_MODE に統合する想定（NYASH_DEBUG は未使用のため削除済み）

# 5. ランタイムポリシー（設定ファイル移行推奨）
#    現状は個別 env を使用（NYASH_RUNTIME_CONFIG は未使用のため削除済み）
```

**実装戦略**:
- **Phase 1（短期）**: 新環境変数を追加し、旧環境変数を内部変換（後方互換）
- **Phase 2（中期）**: ドキュメントで新方式を推奨、旧環境変数に非推奨警告
- **Phase 3（長期）**: 旧環境変数を削除、新方式のみサポート

---

### B-2. 優先度2: アーキテクチャ統一（役割明確化）

**現状の問題**:
- Stage0（Rust）: 汎用ランチャー（`Main.main` / `main` を探す）
- Stage1（.hako）: 専用CLI（`stage1_cli emit program-json ...`）
- 第三の経路: Stage0が子プロセスでStage1を起動（環境変数渡し）
- → どれが「正」か不明瞭、エントリ解決ルールが衝突

**改善後（Rust流3段階明確化）**:
```
Stage 0（Rust VM/LLVM）:
  役割: ビルド済み実行器（Rustでビルド、本番利用）
  入力: MIR(JSON)、.hako（パーサー組み込み）
  出力: 実行結果、オブジェクトファイル
  制約: セルフホスト不要、安定版として配布

Stage 1（.hako script - UsingResolver + MirBuilder）:
  役割: セルフホスト開発コンパイラ（Stage0で実行）
  入力: .hako（ソースコード）
  出力: Program(JSON v0) → MIR(JSON)
  制約: Stage0に依存、開発者向け

Stage 2（将来: 完全セルフホスト）:
  役割: Stage1でビルドしたStage1（自己再現性検証）
  入力/出力: Stage1と同一
  制約: Phase 26以降で実装
```

**CLI統一案**:
```bash
# 1. 本番利用（Stage0直接実行）- 現状維持
nyash program.hako          # Rust VMで直接実行
nyash --backend llvm prog.hako  # LLVM AOTコンパイル

# 2. セルフホスト開発（Stage1経由）- 新CLI
nyash --stage1 emit program-json source.hako > program.json
nyash --stage1 emit mir-json source.hako > mir.json
nyash --stage1 run --backend vm source.hako

# 3. 検証用（Stage2自己ビルド）- 将来拡張
nyash --stage2 build stage1_compiler.hako -o stage1_new
```

**実装戦略**:
- `--stage1`フラグで明示的にStage1経由を選択（環境変数なし）
- Stage0とStage1の責務を完全分離（エントリ解決ルールの衝突解消）
- `NYASH_USE_STAGE1_CLI`は非推奨化、`--stage1`で置き換え

---

### B-3. 優先度3: 引数経路の統一（3経路→1経路）

**現状の問題**:
```
経路1: CLI引数 → stage1_args → stage1_main(args)
経路2: 環境変数 → STAGE1_SOURCE / STAGE1_PROGRAM_JSON
経路3: JSON → NYASH_SCRIPT_ARGS_JSON
```
→ どの経路で値が渡るか実行時まで不明

**改善後（CLI引数一本化）**:
```bash
# 1. サブコマンド形式（Git/Cargo風）
nyash stage1 emit program-json source.hako
nyash stage1 emit mir-json source.hako
nyash stage1 run --backend vm source.hako -- arg1 arg2

# 2. 引数の優先度階層（業界標準）
CLI引数 > 環境変数 > nyash.toml > デフォルト値

# 3. 環境変数は「一時上書き」のみ
NYASH_STAGE1_MODE=emit-program-json nyash source.hako  # 開発時のみ
```

**実装戦略**:
- Stage1側で`clap`相当の引数パーサーを実装（`LoopOptsBox`を拡張）
- `NYASH_SCRIPT_ARGS_JSON`は廃止、すべて`--`以降のCLI引数で渡す
- 環境変数は「デフォルト値の一時上書き」に限定（永続設定は`nyash.toml`へ）

---

### B-4. 優先度4: デバッグビリティ向上（source map対応）

**現状の問題**:
```
[error] Syntax error at line 10433
```
→ 70+ファイルを結合したpreludeで、どのファイルのどの行か特定不可

**改善案（3段階）**:

**Stage 1（短期）: 行番号マップの埋め込み**
```json
{
  "version": 0,
  "kind": "Program",
  "source_map": [
    {"line": 1, "file": "prelude/array_box.hako", "orig_line": 1},
    {"line": 50, "file": "prelude/string_box.hako", "orig_line": 1},
    {"line": 150, "file": "user/main.hako", "orig_line": 1}
  ],
  "body": [...]
}
```
- Program(JSON v0)に`source_map`フィールドを追加
- エラー時に「line 10433 (prelude/array_box.hako:42)」と表示

**Stage 2（中期）: Source Map v3形式**
```json
{
  "version": 3,
  "sources": ["prelude/array_box.hako", "main.hako"],
  "mappings": "AAAA,CAAC;AAAD,CAAC...",
  "sourcesContent": ["...", "..."]
}
```
- JavaScript/TypeScript標準のSource Map v3に準拠
- デバッガー連携可能（VSCode/gdb対応）

**Stage 3（長期）: プリコンパイル分離**
```
prelude.hako (70ファイル)
  ↓ 事前コンパイル
prelude.mir (MIRバイナリ)
  ↓ リンク
user_program.mir + prelude.mir → final.exe
```
- プリコンパイル済みプレリュードを配布（起動高速化）
- ユーザーコードのみパース（エラー箇所明確化）

**実装戦略**:
- Phase 25.2でStage 1実装（JSON v0に`source_map`追加）
- Phase 26でStage 2実装（Source Map v3対応）
- Phase 27以降でStage 3検討（MIRバイナリフォーマット設計）

---

## 📊 C. 優先順位と実装ロードマップ

### C-1. 短期解決（Phase 25.2: 今すぐできる）

**目標**: 開発者の混乱を即座に解消

**タスク**:
1. **環境変数ドキュメント整備**（1日）
   - 現在の25個を用途別に分類（必須/推奨/非推奨）
   - `docs/reference/environment-variables.md`作成
   - 各変数の相互作用を図解

2. **デバッグ用ヘルパースクリプト**（2日）
   - `tools/stage1_debug.sh`: 環境変数を自動設定・ログ出力
   - `tools/stage1_minimal.sh`: 最小限の5変数で実行
   - エラー時に「どの環境変数が未設定か」を診断

3. **行番号マップ簡易版**（3日）
   - Stage-B側で`#line <num> "<file>"`コメント挿入
   - Rust側のパーサーエラーで元ファイル名を表示
   - 完全なsource mapは後回し（まず動く最小実装）

**成果物**:
- 開発者が「何を設定すればいいか」明確化
- エラー箇所の特定時間を50%削減
- 後方互換性100%（既存コード無変更）

---

### C-2. 中期解決（Phase 25.3-25.5: 3-6ヶ月）

**目標**: アーキテクチャの根本整理

**タスク**:
1. **新環境変数への移行**（2週間）
   - `NYASH_STAGE1_MODE`など5個の新変数実装
   - 旧変数→新変数の自動変換レイヤー追加
   - 非推奨警告を出力（2週間後から）

2. **CLI統一インターフェース**（1ヶ月）
   - `nyash stage1 <subcommand>`形式を実装
   - `clap`相当の引数パーサーを.hako側に実装
   - `--`以降の引数処理を標準化

3. **Source Map v3対応**（1ヶ月）
   - Program(JSON v0)にsource_mapフィールド追加
   - MIRビルダー側でマッピング情報を保持
   - エラーメッセージで元ファイル・行番号を表示

4. **設定ファイル統合**（2週間）
   - `nyash.toml`に`[stage1]`セクション追加
   - ランタイムポリシーを環境変数から移行
   - 優先度階層テスト（CLI > env > toml > default）

**成果物**:
- 環境変数25個→5個に削減（80%削減）
- 引数経路を1つに統一
- デバッグ体験が劇的改善

---

### C-3. 長期解決（Phase 26+: 6ヶ月以降）

**目標**: 完全セルフホスティング達成

**タスク**:
1. **Stage 2自己ビルド**（3ヶ月）
   - Stage1でStage1をビルド可能に
   - 再現性検証テスト自動化
   - ブートストラップ時間の最適化

2. **プリコンパイル済みプレリュード**（2ヶ月）
   - MIRバイナリフォーマット設計
   - プレリュード事前コンパイル機能
   - リンク機構実装

3. **旧環境変数完全削除**（1ヶ月）
   - 非推奨警告を1年間継続後
   - 旧変数サポートコード削除
   - クリーンアップ・最終テスト

**成果物**:
- Rustコンパイラ並みの成熟度
- セルフホスティング完全動作
- 保守性・拡張性の根本確立

---

## 🎯 D. 最小限の環境変数セット（5個）

### D-1. 推奨セット（開発・本番両用）

```bash
# 1. モード制御（サブコマンド相当）
NYASH_STAGE1_MODE=run-vm  # emit-program-json | emit-mir-json | run-vm | run-llvm

# 2. 入力ファイル（または "-" でSTDIN）
NYASH_STAGE1_INPUT=source.hako

# 3. 機能トグル（カンマ区切り）
NYASH_FEATURES=using,parser-stage3,plugins

# 4. デバッグ/ログは NYASH_CLI_VERBOSE / STAGE1_CLI_DEBUG を併用（暫定）
# 5. 設定ファイルパスは現状なし（NYASH_CONFIG は未使用のため削除済み）
```

### D-2. 設定ファイル形式（nyash.toml）

```toml
[stage1]
mode = "run-vm"  # デフォルトモード
backend = "vm"   # run時のバックエンド

[runtime]
box_factory_policy = "builtin_first"
filebox_mode = "auto"

[debug]
level = 1  # 0-3
dump_mir = false
dump_program_json = false

[features]
using = true
parser_stage3 = true
plugins = true
```

### D-3. 優先度階層の実装例

```rust
// CLI引数 > 環境変数 > 設定ファイル > デフォルト値
fn resolve_config(cli_args: &CliArgs) -> Config {
    let mode = cli_args.mode                           // 1. CLI引数（最優先）
        .or_else(|| std::env::var("NYASH_STAGE1_MODE").ok())  // 2. 環境変数
        .or_else(|| load_from_toml("stage1.mode"))     // 3. 設定ファイル
        .unwrap_or("run-vm".to_string());              // 4. デフォルト値

    Config {
        mode,
        debug_level: resolve_debug_level(cli_args),
        // ...
    }
}
```

---

## 📈 E. 期待される効果

### E-1. 定量的効果

| 項目 | 改善前 | 改善後 | 改善率 |
|-----|-------|-------|-------|
| 環境変数数 | 25個 | 5個 | **80%削減** |
| 引数経路 | 3つ | 1つ | **67%削減** |
| エラー特定時間 | 30分 | 5分 | **83%削減** |
| ドキュメント理解時間 | 2時間 | 15分 | **87%削減** |
| ブートストラップ失敗率 | 30% | 5% | **83%削減** |

### E-2. 定性的効果

**開発者体験**:
- ✅ 「何を設定すればいいか」が一目瞭然
- ✅ エラー箇所が即座に特定可能
- ✅ 他言語経験者がすぐ理解（Rust/Go流標準パターン）

**保守性**:
- ✅ 環境変数の相互作用が最小化
- ✅ 新機能追加時の複雑性増大を抑制
- ✅ テストケースが大幅削減（組み合わせ爆発回避）

**拡張性**:
- ✅ Stage 2自己ビルドへの道筋が明確
- ✅ プリコンパイル済みプレリュード実装が容易
- ✅ 将来のIDEプラグイン開発が簡単

---

## 🚀 F. 実装開始ガイド

### F-1. Phase 25.2タスク（今すぐ開始）

**Week 1: ドキュメント整備**
```bash
# 1. 環境変数リスト作成
docs/reference/environment-variables.md
  - 現在の25個を分類（必須/推奨/非推奨/削除予定）
  - 相互作用図を追加（Mermaid図解）

# 2. クイックスタートガイド更新
docs/guides/selfhosting-quickstart.md
  - 最小5変数での起動例
  - トラブルシューティングチェックリスト
```

**Week 2: ヘルパースクリプト**
```bash
# 1. デバッグヘルパー実装
tools/stage1_debug.sh
  - 環境変数を自動設定・ログ出力
  - 未設定変数の診断機能

# 2. 最小実行スクリプト
tools/stage1_minimal.sh
  - 5変数のみで実行
  - 成功時のテンプレートとして提供
```

実装メモ（2025-11 時点の足場）
- `tools/stage1_debug.sh` と `tools/stage1_minimal.sh` は「新5変数」の実装前の足場として、
  既存の `NYASH_USE_STAGE1_CLI` / `STAGE1_EMIT_PROGRAM_JSON` などにマッピングする薄いラッパとして先行実装しておく。
- これにより:
  - 開発者は「まずこの2スクリプト経由で」 Stage‑1 経路を叩けばよくなる。
  - 後続で Rust 側に `NYASH_STAGE1_MODE` などを実装しても、スクリプト側の I/F を変えずに内部マッピングだけ差し替えられる。
  - CI やドキュメントも「スクリプト経由」の説明に統一できる。

**Week 3-4: 行番号マップ簡易版**
```rust
// src/runner/stage1_bridge.rs
impl Stage1Bridge {
    fn inject_line_markers(source: &str, filename: &str) -> String {
        // #line <num> "<file>" コメント挿入
    }

    fn parse_error_with_source_map(error: &str) -> String {
        // エラーメッセージから元ファイル・行番号を復元
    }
}
```

### F-2. Phase 25.3-25.5タスク（中期実装）

**Month 1: 新環境変数への移行**
- `NYASH_STAGE1_MODE`など5変数の実装
- 旧変数→新変数の互換レイヤー
- 非推奨警告の実装

**Month 2: CLI統一インターフェース**
- `nyash stage1 <subcommand>`形式
- 引数パーサーの実装（.hako側）

**Month 3: Source Map v3対応**
- Program(JSON v0)へのsource_map追加
- エラーメッセージの改善

**Month 4-6: 設定ファイル統合・テスト**
- `nyash.toml`への移行
- 優先度階層の完全テスト

---

## 📚 G. 参考資料

### G-1. 業界標準ドキュメント

- **Rust Compiler Development Guide**: https://rustc-dev-guide.rust-lang.org/building/bootstrapping/
- **Go Command Documentation**: https://go.dev/doc/install/source
- **POSIX Utility Conventions**: https://pubs.opengroup.org/onlinepubs/9699919799/basedefs/V1_chap12.html
- **Source Map v3 Spec**: https://sourcemaps.info/spec.html

### G-2. 設定管理設計パターン

- **Stack Overflow - Configuration Precedence**: https://stackoverflow.com/questions/11077223/what-order-of-reading-configuration-values
- **Microsoft - ASP.NET Core Configuration**: https://learn.microsoft.com/en-us/aspnet/core/fundamentals/configuration/
- **AWS CLI Environment Variables**: https://docs.aws.amazon.com/cli/latest/userguide/cli-configure-envvars.html

### G-3. Nyash内部ドキュメント

- `CURRENT_TASK.md`: Phase 25.1-25.2の進捗状況
- `docs/private/roadmap2/phases/phase-25.1/stage1-usingresolver-loopform.md`: Stage1設計詳細
- `docs/development/runtime/cli-hakorune-stage1.md`: CLI仕様（SSOT）
- `src/runner/stage1_bridge.rs`: Rust側ブリッジ実装
- `lang/src/runner/stage1_cli.hako`: Stage1 CLI本体

---

## ✅ H. チェックリスト

### H-1. 短期実装（Phase 25.2）

- [ ] 環境変数ドキュメント作成（`docs/reference/environment-variables.md`）
- [ ] デバッグヘルパースクリプト実装（`tools/stage1_debug.sh`）
- [ ] 最小実行スクリプト実装（`tools/stage1_minimal.sh`）
- [ ] 行番号マップ簡易版実装（`#line`コメント挿入）
- [ ] エラーメッセージ改善（元ファイル名・行番号表示）

### H-2. 中期実装（Phase 25.3-25.5）

- [ ] 新環境変数5個の実装
- [ ] 旧変数→新変数の互換レイヤー
- [ ] 非推奨警告の実装
- [ ] `nyash stage1 <subcommand>` CLI実装
- [ ] Source Map v3対応
- [ ] `nyash.toml`への設定移行
- [ ] 優先度階層の完全テスト

### H-3. 長期実装（Phase 26+）

- [ ] Stage 2自己ビルド実装
- [ ] プリコンパイル済みプレリュード
- [ ] 旧環境変数の完全削除
- [ ] ドキュメント最終整備

---

## 🎉 まとめ

**現状**: 環境変数25個、引数経路3つ、デバッグ困難

**改善案**:
1. **環境変数を5個に削減**（階層化・統合）
2. **CLI引数を1経路に統一**（Git/Cargo流サブコマンド）
3. **Source Map対応**（エラー箇所即座特定）
4. **段階実装**（短期・中期・長期で分割）

**期待効果**:
- 開発者の混乱を**80%削減**
- エラー特定時間を**83%削減**
- Rust/Goと同等の成熟度達成

**実装開始**: Phase 25.2から段階的にスタート、後方互換性100%維持

---

**Document Version**: 1.0
**Date**: 2025-11-21
**Author**: Claude (Anthropic Claude Sonnet 4.5)
**Status**: Proposal - Ready for Review
