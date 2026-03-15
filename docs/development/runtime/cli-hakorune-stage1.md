# Stage1 Hakorune CLI Design（Proposal）

Status: design-only + Stage0 stub 実装済み（Phase 25.1 時点では仕様策定と導線の整理まで。selfhost EXE 本体は未実装）。  
Phase 25.1 A-3: `.hako` 側 Stage1Cli skeleton に env-only 実処理を実装（emit program-json / emit mir-json / run stub）。  
ブリッジ（Stage0 → `.hako` stub）は `NYASH_USE_STAGE1_CLI=1` / `STAGE1_EMIT_PROGRAM_JSON=1` 等の ENV で制御する。

## ゴール

- 「ユーザー／開発者が日常叩く CLI」としての `hakorune`（Stage1 selfhost バイナリ）のインターフェースを定義する。
- 既存の Rust CLI（Stage0: `nyash`）が提供している機能を、「パイプライン志向」の小さなサブコマンドに整理し直す。
- Stage1 は **パイプラインのオーケストレーションのみ** 担当し、MIR 実行・LLVM コード生成などの実行コアは Stage0/Rust に委譲する。

## バイナリとプロセスモデル

- Stage0（Rust CLI / ランタイムサービス）
  - 実体: `target/release/nyash`（将来: `hakorune-bootstrap`）
  - 役割: プロセス起動・VM/LLVM コア・ny-llvmc 呼び出しなどの「Ring0」。
  - Ny 側からは `env.mirbuilder.emit` / `env.codegen.emit_object` / `env.codegen.link_object` などの extern 名で見える。
  - Stage1 実行時は「CLI として」ではなく、これらのランタイムサービス層（C-ABI/extern）として利用することを前提とする。
  - Phase 25.1 現在は、Rust Stage0 から `.hako` 側 stub CLI（`lang/src/runner/stage1_cli.hako`）を子プロセスとして起動する
    ブリッジ（`src/runner/stage1_bridge.rs`）のみ実装済みで、自己ホスト EXE（`target/selfhost/hakorune`）はまだ設計段階。
    - ブリッジは env-only 仕様で Stage1 stub を呼び出し、`STAGE1_EMIT_PROGRAM_JSON` / `STAGE1_EMIT_MIR_JSON` / `STAGE1_BACKEND`
      / `STAGE1_SOURCE` などの環境変数をセットする。
    - Stage‑1 UsingResolver は `HAKO_STAGEB_APPLY_USINGS=0` を既定とし、CLI 経路では prefix 連結を行わない（using 解決の検証は
      専用テストで行い、CLI 本線は Program(JSON) 生成に集中させる）。
- Stage1（Hakorune selfhost CLI）
  - 実体: `target/selfhost/hakorune`（Phase 25.1 では Ny Executor プロトタイプ）。
  - 役割: `.hako → Program(JSON) → MIR(JSON) → 実行/EXE` というパイプラインの制御。
  - 将来的には、ユーザーが直接叩く標準 CLI として昇格（`PATH` に入れる対象）。

プロセスモデルの原則:
- Stage1 は **自分で VM/LLVM を実装しない**。常に Ring0 のサービス（env.codegen/env.mirbuilder, NyRT/ny-llvmc 等）を経由して実行・AOT する。
- Stage1 CLI は「どのステージまで進めるか」と「どのバックエンドで実行/ビルドするか」を宣言的に指定するだけに留める。
- Stage1 バイナリ自体は Stage0 の CLI からは独立しており、Stage0 はあくまで「ブートストラップおよびランタイムサービス提供者」として扱う。
- JSON v0 境界の扱い:
- Stage0 直通: Rust AST → MirCompiler で MIR を生成し、`--dump-mir` は MirPrinter の stdout 出力のみ（Program(JSON v0) は介さない）。
- Stage1/selfhost: BuildBox/ParserBox などが `Program(JSON v0)` を返し、Stage0 は `json_v0_bridge::parse_json_v0_to_module` → `maybe_dump_mir`（`RUST_MIR_DUMP_PATH`/`--dump-mir` 両対応） → VM/LLVM という共通導線で処理する。
- Stage‑1 専用モード: `STAGE1_EMIT_MIR_JSON=1` で Program(JSON v0) を生成して Rust 側が即座に MIR 化し dump/emit までを行う（実行はしない。`RUST_MIR_DUMP_PATH` / `--dump-mir` / `--emit-mir-json` が JSON v0→MIR 共通パスで効く）。
- CLI フラグ整理: `.hako` / Stage‑1 を経由する入口は `--hako-emit-program-json` / `--hako-emit-mir-json` / `--hako-run` を前置きで区別する（内部で Stage‑1 stub を呼び出し、JSON v0 境界から先は共通パスへ流す）。

## トップレベル構文

```text
hakorune <command> [<subcommand>] [options] [-- script_args...]
```

- `command`/`subcommand` は **パイプラインの到達点** を表す。
- `options` は主に:
  - 入出力ファイル
  - backend 選択（vm/llvm）
  - profile（dev/ci/lite 等のプリセット）
を指定する。
- `--` 以降はユーザープログラムに渡す引数（既存の `NYASH_SCRIPT_ARGS_JSON` 経路と整合させる）。

## コマンド一覧（MVP 案）

| コマンド                          | 役割                                      | Phase 25.1a 実装状況 |
|-----------------------------------|-------------------------------------------|----------------------|
| `run`                             | .hako をコンパイルして実行（既定 VM）     | プレースホルダ（`[hakorune] run: not implemented yet`） |
| `build exe`                       | .hako からネイティブ EXE を AOT ビルド    | 実装済み（current launcher lane は `env.codegen.compile_json_path/link_object` で `.o` → EXE を生成） |
| `emit program-json`               | Stage‑B で Program(JSON v0) を出力        | 実装済み（Stage0 ブリッジ + `.hako` Stage1Cli 完了） |
| `emit mir-json`                   | Program(JSON) → MIR(JSON) を出力          | 実装済み（Stage0 ブリッジ + `.hako` Stage1Cli 完了） |
| `check`                           | 将来の構文/型/using チェック（予約）      | プレースホルダ（`[hakorune] check: not implemented yet`） |

Phase 25.1a では、**`emit program-json` / `emit mir-json` / `build exe` の 3 系列のみが実働コード** であり、`run` / `check` はメッセージを返して終了するプレースホルダのまま運用する。CLI の出口コード（90〜93）やログ形式は docs と実装を同期済み。  
Phase 25.1 A-3 時点の stub 実装（`.hako` 側 Stage1Cli）は env-only 仕様で、成功時 0 / 入力不足 96 / 無効モード 97 / 実行失敗 98 を返す。

### 実装ステータス（Phase 25.1a+）

- `.hako → Program(JSON v0)`:
  - Stage‑B (`lang/src/compiler/entry/compiler_stageb.hako`) で `BuildBox.emit_program_json_v0` を呼び出し、`"version":0,"kind":"Program"` を持つ JSON を生成。
  - `Stage1UsingResolverBox` と `HAKO_STAGEB_MODULES_LIST` により、`using lang.mir.builder.MirBuilderBox` などの module alias を解決してから parse する。
- `Program(JSON v0) → MIR(JSON)`:
  - 既定は provider 経路（`env.mirbuilder.emit`）を利用。`HAKO_MIRBUILDER_IMPORTS` に `using ns.Type [as Alias]` から得た alias を JSON で渡し、Rust 側ブリッジが static box 参照（`MirBuilderBox`, `BuildBox` など）を `Const(String(alias))` で生成できるようにする。
  - `hostbridge` 参照は JSON ブリッジ側で well-known グローバルとして扱い、`hostbridge.extern_invoke(...)` を含む CLI コードでも `undefined variable: hostbridge` にならないようにした。
- `.hako → EXE` (`build exe`):
  - current launcher lane は `env.codegen.compile_json_path`（MIR JSON path→.o）と `env.codegen.link_object`（.o→EXE）を canonical extern として呼び出す。
  - `--quiet` でログ抑制、`-o/--out` で出力 EXE パスを指定可能。C-API トグル（`NYASH_LLVM_USE_CAPI`, `HAKO_V1_EXTERN_PROVIDER_C_ABI`）が無効な場合は fail-fast。
- Stage1 バイナリ（`target/selfhost/hakorune`）を直接叩く際は `NYASH_NYRT_SILENT_RESULT=1` を付与し、stdout に JSON だけを流す運用を徹底する（`tools/selfhost/run_stage1_cli.sh` が環境セットとバイナリ検出を担当）。

#### デバッグ Tips（Phase 25.1a）
- `STAGE1_CLI_DEBUG=1` を付けると `.hako` 側 `stage1_main` の ENTRY ログが出る。Rust ブリッジが正しく Stage1 stub を呼んでいるか確認する際に使う。
- `NYASH_CLI_VERBOSE=1` か `2` を付けると Rust 側 bridge (`stage1_bridge.rs`) が子プロセス起動ログを出力する。

## `run` コマンド

```text
hakorune run [options] <entry.hako> [-- script_args...]
```

### 意味論

- `.hako` ソースを Stage‑B → MirBuilder → AotPrep まで通し、選択された backend で実行する。
- 実行経路:
  - backend=`vm`   : Stage0 Rust VM（現行 `--backend vm` 相当）
  - backend=`llvm` : ny-llvmc＋NyRT を通した EXE 実行（実装は後続フェーズ）
- プログラムの戻り値をプロセスの exit code にマッピング（現行 Rust CLI と同じ）。

### 主なオプション案

- `--backend {vm|llvm}`（既定: `vm`）
- `--profile {dev|ci|lite|strict}`
  - `dev`   : 詳細ログ・トレースを有効（Phase 15/25 の既存 ENV を束ねる）
  - `ci`    : 安定志向・プラグイン無効化など（現行 quick profile 相当）
  - `lite`  : macro/using など重い機能をオフ
  - `strict`: 各種 STRICT トグルを有効（AotPrep/Verifier 等）
- `--using-path <paths>`: `tools/dev_selfhost_loop.sh` の `--using-path` と一致させる。
- `--json-only`（将来）: Stage‑B までで止め、Program(JSON v0) を stdout に出力。

### Stage0 との関係

- Stage1 `run` は **直接 MIR を実行しない**。
- 代わりに:
  1. Stage1 内で Program(JSON)/MIR(JSON) を構築。
  2. `NYASH_VERIFY_JSON` / `NYASH_JSON_ONLY` / `NYASH_SCRIPT_ARGS_JSON` など既存の ENV プロトコルを用いて Stage0 プロセスを呼び出す。
- これにより「Rust 側の VM/LLVM コアはそのまま」「CLI 表面だけ selfhost 化」という段階移行が可能になる。

## `build exe` コマンド

```text
hakorune build exe [options] <entry.hako>
```

### 意味論

- `.hako` からネイティブ EXE を生成する高レベル API。
- Phase 25.1 実装では、`.hako → Program(JSON v0) → MIR(JSON) → env.codegen.emit_object/link_object → EXE` までを 1 コマンドで行う。
- 具体的な呼び出し:

```text
hakorune build exe [-o <out>] [--quiet] <source.hako>
```

### 意味論（Phase 25.1 実装範囲）

- `.hako` から EXE までのパイプライン:
  1. `.hako` → Program(JSON v0):
     - `BuildBox.emit_program_json_v0(src, null)` を呼び出し。
     - `"version":0` / `"kind":"Program"` を検査。
  2. Program(JSON v0) → MIR(JSON):
     - `MirBuilderBox.emit_from_program_json_v0(program_json, null)` を呼び出し。
  3. MIR(JSON) → object:
     - `env.codegen.compile_json_path(mir_json_path)` を呼び出し、`.o` パスを取得。
  4. object → EXE:
     - `env.codegen.link_object(obj_path, out?)` を呼び出し、EXE パスを取得。
  5. `launcher.hako` は MIR(JSON) を temp file に materialize してから backend boundary へ渡す。
- 実行には C-API ルートの有効化が前提:
  - 例: `NYASH_LLVM_USE_CAPI=1`, `HAKO_V1_EXTERN_PROVIDER_C_ABI=1`, `NYASH_EMIT_EXE_NYRT` など。

### オプション（Phase 25.1 実装済み）

- `-o, --out <path>`:
  - 生成する EXE のパスを指定（省略時は env.codegen 側の既定パスを使用）。
- `--quiet`:
  - 成功時のステータス出力（`[hakorune] build exe: <exe_path>`）を抑制。

※ `--target` / `--nyrt` / `--skip-build` などは、現時点では未実装の設計（将来の AOT プロファイル用）。***

## `emit program-json` コマンド

```text
hakorune emit program-json [options] <entry.hako>
```

### 意味論（Phase 25.1 実装範囲）

- `.hako` ソースファイル（`<entry.hako>`）を読み込み、`BuildBox.emit_program_json_v0(src, null)` を呼び出して Program(JSON v0) を生成する。
- Phase 25.1 の実装では:
  - 入力: `<entry.hako>` パスのみ（標準入力や複数ファイルは未対応）。
  - 出力:
    - 既定: Program(JSON v0) を stdout にそのまま出力。
    - `-o/--out` 指定時: JSON はファイルに書き込み、stdout には短いステータス行のみを出力。
  - バリデーション: `"version":0` と `"kind":"Program"` を含まない場合はエラー終了（exit code 92）。

### オプション（Phase 25.1 実装済み）

- `-o, --out <file>`:
  - Program(JSON v0) を `<file>` に書き出す。
  - スクリプト互換性のため、stdout には短いメッセージ（タグ）だけを出す（JSON 本文は出さない）。
- `--quiet`:
  - `-o/--out` と組み合わせた場合に、ステータス行も抑制し「完全に無音」のファイル出力にする。
  - `--quiet` 単独では意味を持たず、現状は無視される（将来のログ制御用に予約）。

## `emit mir-json` コマンド

```text
hakorune emit mir-json [options] <entry.hako>
```

### 意味論（Phase 25.1 実装範囲）

- Program(JSON v0) から MIR(JSON) を出す経路に加えて、`.hako` から Program(JSON v0)→MIR(JSON) まで進める経路も実装済み。
- 実際の CLI 呼び出しは主に次の2通り:

```text
# 1) Program(JSON v0) から MIR(JSON)
hakorune emit mir-json --from-program-json <program.json>

# 2) .hako から直接 MIR(JSON) まで
hakorune emit mir-json [-o <out>] [--quiet] <source.hako>
```

- 処理内容:
  - `--from-program-json` 指定時:
    - FileBox で `<program.json>` を読み込み、文字列として取得。
    - `MirBuilderBox.emit_from_program_json_v0(program_json, null)` を呼び出して MIR(JSON) を生成。
  - `.hako` 直接指定時:
    - FileBox で `<source.hako>` を読み込み、`BuildBox.emit_program_json_v0(src, null)` で Program(JSON v0) を生成。
    - `"version":0` / `"kind":"Program"` を検査した上で、その Program(JSON v0) を `MirBuilderBox.emit_from_program_json_v0` に渡す。
  - 成功時は MIR(JSON) を stdout に出力（`-o/--out` 指定時はファイル出力）し、exit code 0。
  - 失敗時（ファイルエラー / builder null など）はエラーメッセージ＋exit code 92。

### オプション（Phase 25.1 実装済み）

- `--from-program-json <file>`:
  - Program(JSON v0) を含むファイルパスを指定。
  - `.hako` との併用は禁止（両方指定した場合はエラー）。
- `-o, --out <file>`:
  - MIR(JSON) を `<file>` に書き出す。
  - stdout に MIR(JSON) を直接出さなくなる（短いステータス行のみ）。
- `--quiet`:
  - `-o/--out` と併用時にステータス行も抑制し、MIR(JSON) をファイルにだけ書き込む。

※ `--force-jsonfrag` / `--normalize-provider` などは、引き続き設計のみで未実装。

## I/O と実行補助スクリプト

- Stage1 EXE（`target/selfhost/hakorune`）は NyRT（nyash_kernel）上で動作するため、既定ではプログラム終了時に `Result: <code>` が stdout に追記される。
- llvmlite ハーネスとの互換性を保つため、Stage1 CLI をスクリプトから呼び出す際は `NYASH_NYRT_SILENT_RESULT=1` を常に有効化し、JSON 出力を純粋に保つ。
- 補助スクリプト: `tools/selfhost/run_stage1_cli.sh`
  - 役割: Stage1 EXE の場所を解決し（既定 `target/selfhost/hakorune`）、`NYASH_NYRT_SILENT_RESULT=1` / `NYASH_DISABLE_PLUGINS=1` / `NYASH_FILEBOX_MODE=core-ro` を既定ONにしたうえで CLI 引数をそのまま渡す。
  - 使い方:
    ```bash
    tools/selfhost/run_stage1_cli.sh emit program-json apps/tests/minimal.hako
    tools/selfhost/run_stage1_cli.sh --bin /tmp/hakorune-dev emit mir-json apps/tests/minimal.hako
    ```
  - 直接 EXE を叩く場合も同じ環境変数を手動で設定すること（`NYASH_NYRT_SILENT_RESULT=1 ./target/selfhost/hakorune ...`）。  
    これにより、stdout は JSON のみを返し、終了コードで成否を判別できる（llvmlite ハーネスと同一の契約）。
- 現状の制約（2025-11-15 時点）:
  - `launcher.hako` の Stage‑B Program(JSON) と `--program-json-to-mir` route は、現在は `HakoCli.*` defs と root `user_box_decls` を保持する。`Unknown Box type: HakoCli` は current blocker ではない。
  - `launcher.hako` の `build exe` source lane は `env.codegen.compile_json_path/link_object` へ直接 lower されるので、`selfhost.shared.backend.llvm_backend` module-string receiver は current blocker ではない。
  - selfhost `launcher-exe` の残 blocker は defs 欠落ではなく entry argv handoff 側で、artifact 実行時に CLI args が `HakoCli.run(args)` まで届かない。したがって `emit program-json` / `emit mir-json` の daily proof は引き続き `tools/selfhost/run_stage1_cli.sh` と mainline emit helper で確認する。
  - using 解決は Stage0（Rust Runner）と Stage1（Hakorune）の二系統に分離する方針。Stage1 側は `lang.compiler.entry.using_resolver_box` で `nyash.toml` の `[modules]` を参照し、`HAKO_STAGEB_MODULES_LIST`（shell 側で生成した `name=path` リスト）をキーに依存 Box を text merge する。Rust 側は既存の Runner using 実装を維持し、Stage1 経路はこの Box で独立した自己ホスト導線を持つ。

### Stage‑1 CLI デバッグメモ（Stage1Cli + BuildBox + ParserBox）

- 中間スモーク: `apps/tests/stage1_cli_emit_program_min.hako`
  - 役割: `Stage1Cli.emit_program_json` → `BuildBox.emit_program_json_v0` → `ParserBox.parse_program2` までを、Rust ブリッジや Stage0 runner を経由せずに直接 VM 上で実行する最小ケース。
  - 実行例:
    ```bash
    NYASH_PARSER_STAGE3=1 HAKO_PARSER_STAGE3=1 \
    NYASH_ENABLE_USING=1 HAKO_ENABLE_USING=1 \
    ./target/release/hakorune apps/tests/stage1_cli_emit_program_min.hako
    ```
- Rust テスト側のハーネス: `src/tests/mir_stage1_cli_emit_program_min.rs`
  - `include_str!("../../lang/src/runner/stage1_cli.hako")` で Stage1Cli 本体をバンドルし、`static box Main` を末尾に付けて 1 ソースとしてパースする形に統一。
  - `mir_stage1_cli_emit_program_min_compiles_and_verifies`:
    - Stage1Cli + UsingResolver + BuildBox を含んだモジュールが MIR 生成・verify まで通ることを確認（SSA/PHI 崩壊なし）。
  - `mir_stage1_cli_emit_program_min_exec_hits_type_error`:
    - VM 実行まで進め、Stage‑1 CLI 経路の型エラーや未解決呼び出しが発生しないことを確認するための箱（現在は LoopForm/ParserBox 修正により安定化済み）。

### Stage‑1 CLI 環境変数（env-only 仕様）

- Stage0 の `stage1_bridge.rs` から `.hako` 側 `stage1_cli.hako` を呼び出す際の最低限の ENV:
  - `STAGE1_EMIT_PROGRAM_JSON` / `STAGE1_EMIT_MIR_JSON` / `NYASH_USE_STAGE1_CLI`:
    - モード選択（emit_program_json / emit_mir_json / run）。  
      - `STAGE1_EMIT_PROGRAM_JSON=1`: Program(JSON v0) を stdout に出して終了（VM 実行なし）。
      - `STAGE1_EMIT_MIR_JSON=1`: Program(JSON v0) を JSON v0 ブリッジで MIR(JSON) に変換し、`--dump-mir` / `RUST_MIR_DUMP_PATH` / `--emit-mir-json` を通す emit 専用モード（VM 実行なし）。
  - `STAGE1_SOURCE`:
    - .hako ソースパス（FileBox 経由で読み込むときに使用）。
  - `STAGE1_SOURCE_TEXT`:
    - ソース文字列を直接渡す開発用ショートカット（FileBox 不要、Stage‑B/Stage‑1 の最小スモーク用）。
  - `STAGE1_PROGRAM_JSON`:
    - 事前に生成した Program(JSON v0) のパス。
    - `emit_mir_json` / `run` モードでは、これが設定されていれば file→JSON を優先し、無ければ `.hako → Program(JSON)` を呼び出す。
  - `STAGE1_BACKEND`:
    - `run` 時の backend 選択（`vm` / `llvm`。既定は `vm`）。
  - `STAGE1_CLI_DEBUG`:
    - `1` のとき Stage‑1 CLI 側の debug ログ（`[stage1-cli/debug] ...`）と `__mir__.log` を有効化。

- 新形式のエイリアス（Phase 25.1 で導入済み）:
  - `NYASH_STAGE1_MODE` / `NYASH_STAGE1_INPUT` / `NYASH_STAGE1_BACKEND`
    - Stage1 stub が最初に参照する統一 env。未設定なら上記 legacy `STAGE1_*` からブリッジ（`stage1_bridge/env.rs`）が補完する。
    - 許容値: `emit-program` / `emit-program-json` / `emit-mir` / `emit-mir-json` / `run`。

| 目的            | 新 env                    | 既存/フォールバック          |
|-----------------|---------------------------|------------------------------|
| モード選択      | `NYASH_STAGE1_MODE`       | `STAGE1_EMIT_*` / `NYASH_USE_STAGE1_CLI` |
| 入力パス        | `NYASH_STAGE1_INPUT`      | `STAGE1_SOURCE`              |
| backend 指定    | `NYASH_STAGE1_BACKEND`    | `STAGE1_BACKEND`             |
| Program(JSON) パス | `NYASH_STAGE1_PROGRAM_JSON` | `STAGE1_PROGRAM_JSON`        |

env-only 仕様の原則:
- 入口 `Stage1Cli.stage1_main(args)` は `cli_args_raw` を一切参照せず、上記 ENV だけを見てモード/入力ソース/backend を決定する。
- `.hako` 側で Program(JSON v0) / MIR(JSON) を emit したうえで、実行や AOT は常に Stage0/Rust に委譲する（Stage1 は CLI オーケストレーション専任）。

## `check` コマンド（予約）

```text
hakorune check [options] <entry.hako>
```

### 意味論（将来）

- Stage‑B / MirBuilder / AotPrep を **実行せずに**:
  - 構文
  - using 解決
  - System Hakorune subset 制約
などを検証するためのエントリポイント。

  - 実装は `.hako` 側で `tools/hako-check` 相当のロジックを呼び出す想定。
- Phase 25.1 では「名前予約」と「インターフェース定義」のみを行い、実装は Phase 26 以降。

## Stage0 / Stage1 の責務分離（CLI 視点）

- Stage1（hakorune）
  - ユーザー向け CLI surface。
  - パイプライン選択とオプション解釈。
  - JSON v0/v1 の配線・一時ファイル管理。
- Stage0（nyash / hakorune-bootstrap）
  - VM 実行（vm/backend=vm）。
  - LLVM ハーネス／ny-llvmc 経由の AOT（backend=llvm）。
  - env.codegen / env.mirbuilder などのホストブリッジ提供。

原則:
- Stage1 は **新しい意味論・最適化ロジックを持たない**。
- Stage1 CLI は「どの Stage0/ny-llvmc サービスをどう呼ぶか」を決める **構造レイヤ**。

## フェーズ別導入計画（CLI 観点）

- Phase 25.1
  - `build_stage1.sh` による Ny Executor EXE（`target/selfhost/hakorune`）を用意。
  - 本ドキュメントで CLI サブコマンドと引数の仕様を固定。
  - `hakorune emit mir-json` / `hakorune build exe` に対応する内部 API を `.hako` 側で設計（実装は最小限 or 後続）。
- Phase 26 以降
  - `run` サブコマンドを実装し、日常的な `hakorune run apps/APP/main.hako` 導線を整備。
  - 既存の selfhost スクリプト（`selfhost_build.sh` / `hakorune_emit_mir.sh` / `selfhost_exe_stageb.sh`）を段階的に CLI 経由に移行。
  - `check` を `.hako` 側の hako-check ライブラリに接続。
  - Stage1 → Stage1' の自己ホストサイクルを検証する:
    - Stage1 が自分自身（launcher/runner/CLI を含む）を AOT して Stage1' を生成できること。
    - Stage1 / Stage1' の CLI インターフェース・代表挙動が一致することをゴールデン/スモークで確認する。

このファイルは「Stage1 CLI の仕様 SSOT」として扱い、実装時は本仕様を先に更新→テスト→コードの順で進める。***

- Stage‑B using 未解決メモ:
  - `lang.mir.builder.MirBuilderBox` などの `using` 依存を Stage‑B emit が連結できておらず、`tools/selfhost/build_stage1.sh` では `undefined variable: MirBuilderBox` で停止する。BundleResolver / using resolver を Stage‑B 経路に統合し、依存 Box 定義を Program(JSON) に含めるのが次タスク。
