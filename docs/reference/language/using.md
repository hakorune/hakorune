# using — Imports and Namespaces (Phase 15+)

**実装状況**: Phase 15.5後に本格実装予定 | 基本ドット記法は実装済み

Status: Accepted (Runner‑side resolution). Using is resolved by the Runner; prelude is merged as text (DFS) before parsing/execution.

Phase 20.36 更新
- 依存の唯一の真実（SSOT）: `hako.toml` の `[module_roots]` + `[using]`（aliases/packages/paths）
- 実体の合成は“テキスト統合（merge_prelude_text）”に一本化（AST マージは撤退）
- プロファイル運用: `NYASH_USING_PROFILE={dev|ci|prod}` で厳格度を段階的に切替
  - dev: toml + ファイル内 using を許可（実験/bring‑up）。
  - ci: toml 優先、ファイル using は警告または限定許可。
  - prod: toml のみ。ファイル using/path はエラー（追記ガイドを提示）。

## 🎯 設計思想：Everything has Namespace

### **核心コンセプト**
すべてのBox、関数、メンバーが明確な名前空間を持ち、衝突・曖昧性を根本解決。

```nyash
// ✅ 実装済み：ドット記法
network.HttpClient()         // プラグイン修飾名
plugin.network.HttpClient() // フルパス

// 🚧 Phase 15.5後：明示的スコープ演算子
::print("global")           // グローバルスコープ
builtin::StringBox("test")  // 内蔵版明示
plugin::StringBox("test")   // プラグイン版明示
```

### **MIR Callee革新との統合**
[MIR Callee革新設計](../../development/architecture/mir-callee-revolution.md)と完全統合：

```rust
// Phase 1: 型安全関数呼び出し（実装済み）
pub enum Callee {
    Global(String),          // ::print, global::func
    Method { box_name, method, receiver }, // obj.method()
    Extern(String),          // nyash.console.log
    Value(ValueId),          // 第一級関数
}

// Phase 3: 完全修飾名対応（Phase 15.5後）
pub enum QualifiedCallee {
    Qualified { namespace: Vec<String>, name: String },
    Scoped { scope: ScopeKind, name: String },
}
```

## 📊 実装状況

### ✅ **現在実装済み**
- **ドット記法**: `plugin.BoxName`、`namespace.member`
- **using基本構文**: ファイルトップでの宣言
- **エイリアス**: `using long.path as Alias`
- **プラグイン修飾**: `network.HttpClient`

### 🚧 **Phase 15.5後実装予定**
- **built-in namespace**: `builtin.StringBox` vs `plugin.StringBox`
- **完全修飾名**: `nyash.builtin.print`、`std.console.log`
- **スコープ演算子**: `::global_func`、`Type::static_method`
- **厳密解決**: コンパイル時名前空間検証

Policy（Runner前処理）
- Accept `using` lines at the top of the file to declare module namespaces or file imports.
- Resolution is performed by the Rust Runner when `NYASH_ENABLE_USING=1`.
- 実体の結合はテキスト統合（merge_prelude_text）。AST マージ経路は撤退。
- Runner は `hako.toml` の `[using]` を SSOT として参照（prod）。互換として `nyash.toml` も読めるが、編集点は `hako.toml` に集約する。
- Selfhost compiler (Ny→JSON v0) collects using lines and emits `meta.usings` when present. The bridge currently ignores this meta field.
 - Prelude の中にさらに `using` が含まれている場合は、Runner が再帰的に `using` をストリップしてから AST として取り込みます（入れ子の前処理をサポート）。
 - パス解決の順序（dev/ci）: 呼び出し元ファイルのディレクトリ → `$NYASH_ROOT` → 実行バイナリからのプロジェクトルート推定（target/release/nyash の 3 階層上）→ `hako.toml` の `[using.paths]`（互換: `nyash.toml`）。

Deprecated: `include`
- 言語仕様としてはサポートしない（VM/コンパイラともに受理しない）。
- 例外は開発支援用の前処理（preinclude）のみ。実行系や言語仕様の責務ではなく、テストハーネスからフラグで明示的に有効化する。
  - Flags: `NYASH_PREINCLUDE=1` / `HAKO_PREINCLUDE=1`（既定OFF）
  - quick プロファイルでは include 依存は既定で SKIP（`SMOKES_INCLUDE_POLICY=skip|warn|error`。順次 ERROR へ移行予定）。
  - 本番（prod）では using/alias のみを正道に固定。`using "path"` は開発限定（`NYASH_ALLOW_USING_FILE=1`）で運用する。

## Namespace Resolution (Runner‑side)
- Goal: keep IR/VM/JIT untouched. All resolution happens in Runner/Registry.
- Default search order (3 stages, deterministic):
  1) Local/Core Boxes (nyrt)
  2) Aliases (nyash.toml [imports] / `needs … as …`)
  3) Plugins (short name if unique, otherwise qualified `pluginName.BoxName`)
- On ambiguity: error with candidates and remediation (qualify or define alias).
- Modes:
  - Relaxed (default): short names allowed when unique。
  - Strict: plugin短名にprefix必須（env `NYASH_PLUGIN_REQUIRE_PREFIX=1` または nyash.toml `[plugins] require_prefix=true`）。
- Aliases:
  - nyash.toml `[imports] HttpClient = "network.HttpClient"`
  - needs sugar: `needs plugin.network.HttpClient as HttpClient` (file‑scoped alias)

## 📦 静的 Box の using（Phase 173+）

**実装状況**: Phase 173 で実装中（2025-12-04）

### 基本概念

静的 Box（`static box`）をライブラリとして `using` で import し、静的メソッドを呼び出す機能。

```hako
// 静的 Box の定義（ライブラリ側）
static box JsonParserBox {
  method parse(json_str) {
    // JSON文字列をパース
    return me._parse_value(json_str, 0).get("value")
  }

  _parse_value(s, pos) {
    // 内部メソッド（private的な扱い）
    // ...
  }
}
```

```hako
// 静的 Box の使用（呼び出し側）
using tools.hako_shared.json_parser as JsonParserBox

static box Main {
  main() {
    // 静的メソッド呼び出し（インスタンス化不要）
    local result = JsonParserBox.parse("{\"x\":1}")

    // 結果の利用
    if result != null {
      local x = result.get("x")
      print("x = " + x)
    }

    return 0
  }
}
```

### 許容される呼び出しパターン

#### ✅ Phase 173 で実装済み

1. **静的メソッド直接呼び出し**
   ```hako
   using tools.hako_shared.json_parser as JsonParserBox

   local value = JsonParserBox.parse("{\"x\":1}")
   ```
   - `JsonParserBox` は静的 Box のエイリアス
   - `parse()` は静的メソッド
   - インスタンス化不要

#### 🚧 Phase 174+ で実装予定

2. **名前空間的な Box アクセス**（未実装）
   ```hako
   using lib.containers as Containers

   local list = new Containers.ListBox()    // エラー（Phase 173 時点）
   local map = new Containers.MapBox()      // エラー（Phase 173 時点）
   ```
   - `new Alias.BoxName()` 構文は未サポート
   - パーサーエラー: `Unexpected token DOT, expected LPAREN`

### 静的 Box vs インスタンス Box

| 特徴 | 静的 Box | インスタンス Box |
|-----|---------|----------------|
| 定義 | `static box BoxName { }` | `box BoxName { }` |
| インスタンス化 | 不要（シングルトン） | 必要（`new BoxName()`） |
| メソッド呼び出し | `BoxName.method()` | `instance.method()` |
| 状態保持 | 共有状態（グローバル的） | インスタンスごと |
| using での扱い | 型として登録 | 型として登録 |
| 典型的用途 | ユーティリティ、パーサー | データ構造、オブジェクト |

### 実装の技術的詳細

#### 名前解決の流れ

1. **using statement 解決**（`.hako` 側）
   - `UsingResolverBox.resolve_path_alias()` でファイルパス解決
   - エイリアスを「型」として環境に登録（Phase 173 で実装）

2. **AST 生成**（`.hako` パーサー）
   - `Alias.method()` を静的Box呼び出しとして認識
   - フラグ `is_static_box_call: true` を付与（Phase 173 で実装）

3. **MIR lowering**（Rust 側）
   - `CalleeResolverBox.resolve()` で Callee に変換
   - 静的Box呼び出しは `Callee::Global("BoxName.method/arity")` に解決
   - VM 実行時に正しく静的メソッドとして呼び出される

#### Phase 171-2 で発見された問題

**症状**:
```
[ERROR] ❌ [rust-vm] VM error: Unknown method '_skip_whitespace' on InstanceBox
```

**原因**:
- using で import した静的 Box が「インスタンス」として扱われていた
- 内部メソッド（`_skip_whitespace` 等）が解決できず

**解決**: Phase 173 で using system を修正し、静的 Box を型として正しく登録

### 使用例

#### JsonParserBox の利用
```hako
using tools.hako_shared.json_parser as JsonParserBox

static box HakoAnalysisBuilderBox {
  _extract_cfg_from_mir_json(json_text) {
    // JsonParserBox を使った JSON パース
    local root = JsonParserBox.parse(json_text)
    if root == null { return me._empty_cfg() }

    // CFG オブジェクトの抽出
    local cfg = root.get("cfg")
    if cfg == null { return me._empty_cfg() }

    local functions = cfg.get("functions")
    // ... 以下、CFG 処理続行
  }
}
```

#### ProgramJSONBox の利用（Phase 172）
```hako
using tools.hako_shared.json_parser as JsonParserBox

static box Compiler {
  load_program(json_str) {
    // Program JSON v0 をパース
    local program = JsonParserBox.parse_program(json_str)
    if program == null { return null }

    // ProgramJSONBox インスタンスから情報取得
    local version = program.get_version()  // "0"
    local kind = program.get_kind()        // "program"
    local defs = program.get_defs()        // ArrayBox

    return program
  }
}
```

### 制限事項（Phase 173 時点）

1. **パーサー制限**
   - `new Alias.BoxName()` 構文は未サポート
   - 静的 Box 内の Box 定義をインスタンス化できない

2. **名前空間階層**
   - `Alias.Namespace.BoxName` のような多階層は未サポート
   - エイリアスは1階層のみ

3. **型推論**
   - 静的メソッドの戻り値型は動的（MapBox/ArrayBox 等）
   - コンパイル時型チェックなし

### Phase 174+ での拡張予定

1. **名前空間的 Box アクセス**
   - `new Alias.BoxName()` 構文のサポート
   - パーサー修正による対応

2. **型システム統合**
   - HIR 層の導入
   - 静的型推論の強化

3. **明示的スコープ**
   - `::` 演算子のサポート
   - `BoxName::static_method()` 構文

## Plugins
- Unified namespace with Boxes. Prefer short names when unique.
- Qualified form: `network.HttpClient`
- Per‑plugin control (hako.toml): `prefix`, `require_prefix`, `expose_short_names`
  - 現状は設定の読み取りのみ（導線）。挙動への反映は段階的に実施予定。

## `needs` sugar (optional)
- Treated as a synonym to `using` on the Runner side; registers aliases only.
- Examples: `needs utils.StringHelper`, `needs plugin.network.HttpClient as HttpClient`, `needs plugin.network.*`

## hako.toml / nyash.toml — Unified Using（唯一の真実 / SSOT）

**設定ファイルの優先順位（Phase 20.37+）**
- `hako.toml` を優先（開発・ドキュメントの正道）
- `nyash.toml` は互換名（legacy）。編集点は `hako.toml` に集約する
  - このリポジトリでは `nyash.toml` は `hako.toml` の互換エイリアスとして扱う

### `module_roots` (Phase 29bq+)
`using` の名前空間を **root だけで宣言**するための SSOT。
個別ファイルを列挙せず、`using a.b.c` を root から決定的に解決する。

```toml
# hako.toml (SSOT)
[module_roots]
"lang.compiler" = "lang/src/compiler"
"core"          = "src/core"

[modules]
# optional explicit override (exact match wins)
"lang.compiler.entry.func_scanner" = "lang/src/compiler/entry/func_scanner.hako"
```

#### 解決アルゴリズム（SSOT）

1. `[modules]` に完全一致があればそれを使う（override）
2. `[module_roots]` で最長 prefix 一致 → path を構築
3. 0件/多重一致は `[freeze:contract][module_roots]` で fail-fast

```
using lang.compiler.parser.box
  → [modules] に完全一致なし
  → [module_roots] で "lang.compiler" が最長一致
  → "lang/src/compiler" + "/parser/box.hako"
  → "lang/src/compiler/parser/box.hako"
```

#### エラータグ（fail-fast）

| タグ | 意味 |
|------|------|
| `[freeze:contract][module_roots] not_found` | 0件一致（どの root にもマッチしない） |
| `[freeze:contract][module_roots] ambiguous` | 多重一致（同じ長さの prefix が複数） |
| `[freeze:contract][module_roots] both_manifest` | 予約（未使用）。このリポジトリでは `nyash.toml` は `hako.toml` の互換エイリアスとして共存する |

#### Stage-B への伝達

Rust は `HAKO_STAGEB_MODULE_ROOTS_LIST` 環境変数で .hako コンパイラに渡す:
```
HAKO_STAGEB_MODULE_ROOTS_LIST="lang.compiler=lang/src/compiler|||core=src/core"
```
形式（SSOT）:
- エントリ区切りは `|||` 固定
- 各エントリは `prefix=path`（`=` は1回のみ）
- 末尾の `|||` は付けない（空エントリ禁止）

#### 互換性

- 旧 `[modules]` は **exact override** として扱う（同名があれば `[modules]` が優先）。
- 新規追加は `module_roots` を使う（個別登録は増やさない）。
- `nyash.toml` は deprecated（hako.toml があれば無視、両方あれば freeze）。

Using resolution is centralized under the `[using]` table. Three forms are supported:

- `[using.paths]` — additional search roots for path lookups
  - Example: `paths = ["apps", "lib", "."]`
- `[using.<name>]` — named packages (file or directory)
  - Keys: `path = "lib/math_utils/"`, optional `main = "math_utils.hako"`
  - Optional `kind = "dylib"` with `bid = "MathBox"` for plug‑ins (dev only)
- `[using.aliases]` — alias mapping from short name to a package name
  - Example: `aliases.json = "json_native"`

Notes
- Aliases are fully resolved: `using json` first rewrites to `json_native`, then resolves to a concrete path via `[using.json_native]`.
- `include` は廃止。代替は `using "./path/to/file.hako" as Name`。prod では `hako.toml` への登録が必須。

Development toggles
- Resolution is performed by the Runner when `NYASH_ENABLE_USING=1`（既定ON）。
- Prelude は常にテキスト統合（DFS/循環検出/キャッシュ）。`NYASH_USING_AST` は後方互換のために残るが AST マージは行わない。
- `NYASH_RESOLVE_TRACE=1` で解決ログ（cache‑hit/候補/未解決）を出力。
- 前処理は最小 normalize を適用（CRLF→LF、`}` 直前の冗長 `;` を除去、EOF 改行付加）。prod のコードスタイルに依存しないこと。

### Dylib autoload (dev guard)
- Enable autoload during using resolution: set env `NYASH_USING_DYLIB_AUTOLOAD=1`.
- Resolution returns a token `dylib:<path>`; when autoload is on, Runner calls the plugin host to `load_library_direct(lib_name, path, boxes)`.
- `boxes` is taken from `[using.<name>].bid` if present; otherwise the loader falls back to plugin‑embedded TypeBox metadata.
- Safety: keep OFF by default. Prefer configuring libraries under `nyash.toml` for production.

## Index and Cache (Runner)
- BoxIndex（グローバル）：プラグインBox一覧とaliasesを集約し、Runner起動時（plugins init後）に構築・更新。
  - `aliases: HashMap<String,String>`（nyash.toml `[aliases]` と env `NYASH_ALIASES`）
  - `plugin_boxes: HashSet<String>`（読み取り専用）
- 解決キャッシュ：グローバルの小さなキャッシュで同一キーの再解決を回避（キー: `tgt|base|strict|paths`）。
- トレース：`NYASH_RESOLVE_TRACE=1` で解決手順やキャッシュヒット、未解決候補を出力。

Syntax
- Namespace: `using core.std` or `using core.std as Std`
- File path: `using "apps/examples/string_p0.hako" as Strings`
- Relative path is allowed; absolute paths are discouraged.

Style
- Place all `using` lines at the top of the file, before any code.
- One using per line; avoid trailing semicolons. Newline separation is preferred.
- Order: sort alphabetically by target. Group namespaces before file paths.
- Prefer an explicit alias (`as ...`) when the target is long. Suggested alias style is `PascalCase` (e.g., `Std`, `Json`, `UI`).

Examples
```nyash
using core.std as Std
using "apps/examples/string_p0.hako" as Strings

static box Main {
  main(args) {
    local console = new ConsoleBox()
    console.println("hello")
    return 0
  }
}
```

nyash.toml examples
```toml
[using]
paths = ["apps", "lib", "."]

[using.json_native]
path = "apps/lib/json_native/"
main = "parser.hako"

[using.aliases]
json = "json_native"

# Dylib (dev)
[using.math_plugin]
kind = "dylib"
path = "plugins/math/libmath.so"
bid = "MathBox"
```

Qualified/Plugins/Aliases examples
```nyash
# nyash.toml
[plugins.network]
path = "plugins/network.so"
prefix = "network"
require_prefix = false

[imports]
HttpClient = "network.HttpClient"

# code
needs plugin.network.HttpClient as HttpClient

static box Main {
  main(args) {
    let a = new HttpClient()         # alias
    let b = new network.HttpClient() # qualified
  }
}
```

📌 **Note: `Type::static_method` と宣言構文について**

- 呼び出し側では `Type::static_method(...)` のように「型にぶら下がった関数」を使うことができるが、
  これはあくまで **呼び出し構文** だよ。
- 宣言側の構文としては **`static box` の中に通常のメソッドを定義する** のが Stage‑3 仕様の正しい形で、
  `static method foo() { ... }` のようなトップレベル／Box外の宣言構文は **legacy/非推奨** であり、今後の実装では使用しない。

Runner Configuration
- Enable using pre‑processing: `NYASH_ENABLE_USING=1`
- CLI from-the-top registration: `--using "ns as Alias"` or `--using '"apps/foo.hako" as Foo'` (repeatable)
- Using profiles (phase‑in): `NYASH_USING_PROFILE={dev|ci|prod}`
  - dev: AST マージ 既定ON、legacy前置きは既定で無効（必要時は `NYASH_LEGACY_USING_ALLOW=1` で一時許可）
  - ci: AST マージ 既定ON、legacy前置きは既定で無効（同上の一時許可）
  - prod: AST マージ 既定OFF、toml のみ（file using/path はエラー・追記ガイド）
- Strict mode (plugin prefix required): `NYASH_PLUGIN_REQUIRE_PREFIX=1` または `nyash.toml` の `[plugins] require_prefix=true`
- Aliases from env: `NYASH_ALIASES="Foo=apps/foo/main.hako,Bar=lib/bar.hako"`
- Additional search paths: `NYASH_USING_PATH="apps:lib:."`
- Selfhost pipeline keeps child stdout quiet and extracts JSON only: `NYASH_JSON_ONLY=1` (set by Runner automatically for child)
- Selfhost emits `meta.usings` automatically when present; no additional flags required.

Note: Provider/Type 分離（型名は不変で提供者のみを切替）については ADR を参照。  
docs/development/adr/adr-001-no-corebox-everything-is-plugin.md

## 🔬 Quick Smokes（AST + Profiles）

開発・CIで最小コストに確認できるスモークを用意しています。AST プレリュードとプロファイル（dev/prod）の基本動作をカバーします。

- dev: `using "file"` 許可 + AST マージ
- prod: `using "file"` 禁止（toml へ誘導） / alias・package は許可

実行例（quick プロファイル）

```
# 1) dev で file using が通る（AST マージ）
./tools/smokes/v2/run.sh --profile quick --filter "using_profiles_ast.sh$"

# 2) 相対パス using（サブディレクトリ）
./tools/smokes/v2/run.sh --profile quick --filter "using_relative_file_ast.sh$"

# 3) 複数プレリュード（toml packages）+ 依存（B→A）
./tools/smokes/v2/run.sh --profile quick --filter "using_multi_prelude_dep_ast.sh$"
```

テストソース
- `tools/smokes/v2/profiles/quick/core/using_profiles_ast.sh`
- `tools/smokes/v2/profiles/quick/core/using_relative_file_ast.sh`
- `tools/smokes/v2/profiles/quick/core/using_multi_prelude_dep_ast.sh`

注意
- ログに `[using] stripped line:` が出力されますが、これは AST マージ前の using 行の除去ログです（機能上問題ありません）。
- 実行バイナリは `target/release/nyash` を前提とします。未ビルド時は `cargo build --release` を実行してください。

## 🔗 関連ドキュメント

### **設計・アーキテクチャ**
- [MIR Callee革新設計](../../development/architecture/mir-callee-revolution.md) - 型安全関数呼び出し
- [Phase 15.5 Core Box統一](../../development/roadmap/phases/phase-15.5/README.md) - プラグイン統一計画
- [Box Factory設計](../../reference/architecture/box-factory-design.md) - builtin vs plugin優先順位

### **実装ガイド**
- [Callee実装ロードマップ](../../development/roadmap/phases/phase-15/mir-callee-implementation-roadmap.md)
- [プラグインシステム](../../reference/plugin-system/) - プラグイン開発ガイド
- [完全言語リファレンス](../LANGUAGE_REFERENCE_2025.md) - 全構文仕様

## 📝 実装ノート

Notes
- Phase 15 keeps resolution in the Runner to minimize parser complexity. Future phases may leverage `meta.usings` for compiler decisions.
- レガシー実装の扱い: テキスト前置き/括弧補正などのシムは段階的に削除（prod プロファイルから先に無効化）。
- AST マージは dev/ci/prod の全プロファイルで共通基盤とし、曖昧性（宣言≻式）問題の再発を原理的に回避する。
- Unknown fields in the top‑level JSON (like `meta`) are ignored by the current bridge.
- 未解決時（非strict）は実行を継続し、`NYASH_RESOLVE_TRACE=1` で候補を提示。strict時はエラーで候補を表示。
- **Phase 15.5完了により、現代的な名前空間システムを実現予定**

## Deprecated: Include/Export（廃止）

このセクションは移行期の参考情報です。`include` は設計上の一貫性と学習コスト低減のため廃止しました。今後はすべて `using` に一本化してください（ファイル・パッケージ・DLL すべてを `using` で扱えます）。既存コードの移行は以下の対応例を推奨します。

- `local M = include "./path/module.hako"` → `using "./path/module.hako" as M`
- `include` の探索ルートは `[using.paths]` に統合（`nyash.toml`）

注: `include` は完全に非推奨です。コードは `using` に書き換えてください（互換シムは提供しません）。

Overview
- One file exports one static box. `include(path)` evaluates the file and returns that Box instance.

Syntax
```
local Math = include "lib/math.hako"
local r = Math.add(1, 2)
```

Rules
- Single static box per file（0/複数はエラー）
- Expression form: `include(...)` は Box インスタンスを返す式
- Caching: 同一パスは一度だけ評価（2回目以降はキャッシュ返却）
- Path resolution（MVP）:
  - Relative allowed; absolute discouraged
  - nyash.toml `[include.roots]` で `std=/stdlib` 等のルート定義を許可
  - 省略拡張は `.hako`、ディレクトリなら `index.hako`

Backends
- Interpreter: 実行時に評価し Box を返す
- VM/AOT: MIR Builder が対象ファイルを読み取り、同一 MIR モジュールに static box を降ろす（専用 MIR 命令は追加しない）

Limitations
- 循環 include の検出/診断は未実装（後続で active-load 追跡と経路表示を追加）

Rationale
- MIR 仕様に変更を入れず、実用的なモジュール分割を提供
- Everything‑is‑Box に整合（モジュール=Box、メソッド/フィールド=API）
