# 環境変数リファレンス

Nyash の主要な環境変数をカテゴリ別に整理するよ。`適用経路` はどのパスで効くかを示す:

- Rust AST: Rust パーサ直通（例: `--dump-mir`。compile-only の入口）
- JSON v0/Stage-1: selfhost/Stage-1/`--ny-parser-pipe` 経由（json_v0_bridge で処理）
- Any: どの経路でも有効

---

## ダンプ / 診断

| 変数 | デフォルト | 適用経路 | 説明 |
| --- | --- | --- | --- |
| `RUST_MIR_DUMP_PATH=/tmp/out.mir` | OFF | JSON v0/Stage-1 | MIR printer の出力をファイルに書く (`json_v0_bridge::maybe_dump_mir` 経由) |
| `NYASH_CLI_VERBOSE=1` | OFF | Any | 詳細ログ。`maybe_dump_mir` が stdout に MIR を出す |
| `NYASH_CLI_VERBOSE=2` | OFF | Any | さらに詳細なログ（Ny compiler 経路の診断ログ含む） |
| `NYASH_RING0_LOG_LEVEL=INFO` | `INFO` | Any | Ring0 logger の最小レベル（`DEBUG`/`INFO`/`WARN`/`ERROR`） |
| `NYASH_VM_DUMP_MIR=1` | OFF | Any | VM 実行前の MIR を出力 |
| `HAKO_VM_MAX_STEPS=1000000` | `1000000` | VM | VM の 1関数あたり最大ステップ数。`0` は上限なし（診断専用）。 |
| `NYASH_VM_MAX_STEPS=1000000` | unset | VM | `HAKO_VM_MAX_STEPS` の alias（互換）。 |
| `NYASH_VM_TRACE=1` | OFF | VM | VM のブロック進入/命令実行トレースを stderr に出す。 |
| `NYASH_VM_TRACE_EXEC=1` | OFF | VM | `NYASH_VM_TRACE` の alias（互換）。 |
| `NYASH_VM_TRACE_LOG=1` | OFF | VM | VM トレースをファイルへ追記。`1` の場合は `__mir__.log`、それ以外は指定パス。 |
| `HAKO_VM_ERROR_LOC=1` | OFF | VM | VM エラー時に簡易位置を 1行で出す（`[vm/error/loc] ...`）。 |
| `NYASH_DUMP_JSON_IR=1` | OFF | Any | JSON IR をダンプ |
| `NYASH_DEBUG_STACK_OVERFLOW=1` | OFF | Any | スタックオーバーフロー時に backtrace を有効化 |
| `NYASH_BINOP_REPROP_DEBUG=1` | OFF | Any | BinOp 型再伝播（MIR）をトレース |
| `NYASH_LEAK_LOG=1` | OFF | VM (full), LLVM (parent process roots only) | プログラム終了時に残存する強参照を報告（サマリー） |
| `NYASH_LEAK_LOG=2` | OFF | VM (full), LLVM (parent process roots only) | プログラム終了時に残存する強参照を報告（詳細、最初の10件まで） |

### ダンプの使い分け
- 実行経路SSOT（推奨）: `NYASH_VM_DUMP_MIR=1 ./target/release/hakorune --backend vm apps/tests/minimal.hako`
- Rust AST 直通（compile-only）: `./target/release/hakorune --dump-mir apps/tests/minimal.hako`（env は不要、stdout のみ）
- JSON v0 経路/Stage-1: `RUST_MIR_DUMP_PATH=/tmp/out.mir NYASH_USE_STAGE1_CLI=1 STAGE1_EMIT_MIR_JSON=1 ./target/release/hakorune --dump-mir`（stdout + ファイル）

### NYASH_LEAK_LOG（Phase 285LLVM-0）

**Backend Support**: VM (full), LLVM (parent process roots only)

プログラム終了時に残存する強参照を報告する診断機能（デフォルトOFF）。

**値**:
- `1`: サマリーのみ（modules, host_handles, plugin_boxes の数）
- `2`: 詳細（名前/エントリを含む、最初の10件まで）

**例**:
```bash
# VM: 完全なリークレポート
NYASH_LEAK_LOG=2 ./target/release/hakorune program.hako

# LLVM compat/probe keep lane: 親プロセスのroot snapshotのみ
NYASH_LEAK_LOG=2 NYASH_LLVM_USE_HARNESS=1 ./target/release/hakorune --backend llvm program.hako
```

**LLVM制限**: LLVM harness runnerは親プロセス（Rust VM側）のroot snapshotのみ報告。子プロセス（native executable）内部の到達可能性はプロセス境界の制約により見えない。

---

## Stage-1 / selfhost CLI

| 変数 | デフォルト | 適用経路 | 説明 |
| --- | --- | --- | --- |
| `NYASH_USE_STAGE1_CLI=1` | OFF | Stage-1 | Stage-1 stub 経由に切替 |
| `NYASH_STAGE1_MODE=emit-mir` | unset | Stage-1 | `emit-program` / `emit-mir` / `run` を明示（`emit-program` は compat-only） |
| `STAGE1_EMIT_PROGRAM_JSON=1` | OFF | Stage-1 | Program(JSON v0) を吐いて終了（compat-only legacy alias） |
| `STAGE1_EMIT_MIR_JSON=1` | OFF | Stage-1 | Program(JSON v0)→MIR(JSON) を Rust 側で降ろす（レガシー alias） |
| `HAKO_STAGE1_MODE={emit-program\|emit-mir\|run}` | unset | Stage-1 | .hako / Stage-1 ルート専用のモード指定（`--hako-*` が設定） |
| `HAKO_EMIT_PROGRAM_JSON=1` | OFF | Stage-1 | `.hako` stub に Program(JSON v0) emit を指示（compat-only） |
| `HAKO_EMIT_MIR_JSON=1` | OFF | Stage-1 | `.hako` stub に MIR(JSON) emit を指示（json_v0_bridge 経由） |
| `NYASH_STAGE1_INPUT=path` | unset | Stage-1 | 入力ソース（alias: `STAGE1_SOURCE`, `STAGE1_INPUT`） |
| `HAKO_STAGE1_INPUT=path` | unset | Stage-1 | `.hako` stub 用の入力ソース（`--hako-*` が設定） |
| `NYASH_STAGE1_PROGRAM_JSON=path` | unset | Stage-1 | Program(JSON v0) のパス（compat-only; alias: `STAGE1_PROGRAM_JSON`） |
| `HAKO_STAGE1_PROGRAM_JSON=path` | unset | Stage-1 | `.hako` stub 用 Program(JSON v0) パス（compat-only） |
| `NYASH_STAGE1_BACKEND=vm` | `vm` | Stage-1 | Stage-1 実行の backend ヒント（alias: `STAGE1_BACKEND`） |
| `NYASH_STAGE1_CLI_CHILD=1` | OFF | Stage-1 | 再帰呼び出しガード |
| `STAGE1_CLI_ENTRY=...` | `lang/src/runner/stage1_cli.hako` | Stage-1 | Stage-1 stub のエントリ差し替え。canonical compat owner は `lang/src/runner/compat/stage1_cli.hako`、この default は薄い keep wrapper を指す。 |
| `NYASH_STAGE1_BINARY_ONLY_DIRECT={0\|1}` | unset | Stage-1 | binary-only direct route を強制ON/OFF（unset は OFF。明示時のみ有効） |
| `NYASH_STAGE1_BINARY_ONLY_RUN_DIRECT={0\|1}` | unset | Stage-1 | `--hako-run` の binary-only direct route を強制ON/OFF（unset は `NYASH_STAGE1_BINARY_ONLY_DIRECT` を継承し、最終的に OFF） |
| `HAKO_SELFHOST_NO_DELEGATE={0\|1}` | unset | Stage-1 / selfhost | `env.mirbuilder.emit` の delegate route を禁止（`1` で fail-fast 固定） |
| `HAKO_MIR_BUILDER_DELEGATE={0\|1}` | unset | Stage-1 / selfhost | MirBuilder delegate route の互換トグル（mainline child では `0` に固定） |
| `STAGE1_*` alias | legacy | Stage-1 | `NYASH_STAGE1_*` の旧名。互換のため受理するが順次廃止予定 |

### Stage-1 経路の例
```bash
# Stage-1 で MIR(JSON) を受け取り、Rust 側で dump（preferred）
RUST_MIR_DUMP_PATH=/tmp/out.mir \
NYASH_USE_STAGE1_CLI=1 STAGE1_EMIT_MIR_JSON=1 \
  ./target/release/hakorune --dump-mir apps/tests/minimal.hako

# hako- 前置の Stage-1 MIR launcher（stage1-env-mir-source）
./target/release/hakorune --hako-emit-mir-json /tmp/out.mir apps/tests/minimal.hako --dump-mir
```

Compat-only raw Program(JSON) route:
```bash
./target/release/hakorune --emit-program-json-v0 /tmp/out.json apps/tests/minimal.hako
```

Note:
- `--hako-emit-mir-json` is the current Stage-1 MIR launcher entry.
- Program(JSON) is compat-only; hako-prefixed Program(JSON) public aliases and
  raw Program(JSON)->MIR CLI conversion are retired.
- explicit Program(JSON)->MIR helper work should use `env.mirbuilder.emit` /
  `tools/selfhost/lib/program_json_mir_bridge.sh` or dedicated compat probes.
- `--emit-program-json-v0` remains delete-last while explicit keeper/probe
  routes still exist (for example `tools/dev/program_json_v0/stageb_artifact_probe.sh`
  and `tools/dev/phase29ci_selfhost_build_exe_consumer_probe.sh`).

---

## Parser / using

| 変数 | デフォルト | 適用経路 | 説明 |
| --- | --- | --- | --- |
| `NYASH_FEATURES=stage3` | `stage3` (implicit) | Any | カンマ区切りの機能フラグ。`stage3` で Stage-3 構文を許可（既定ON）。 |
| `NYASH_PARSER_STAGE3=1` | legacy | Any | Stage-3 旧エイリアス。将来削除予定。OFF にしたい場合のみ指定。 |
| `HAKO_PARSER_STAGE3=1` | legacy | Any | `.hako` 向け Stage-3 legacy alias。将来削除予定。 |
| `NYASH_TRY_RESULT_MODE=1` | OFF | JSON v0 | try/catch/cleanup を Result-mode（structured blocks + jumps）で lower する（MIR Throw/Catch を使わない） |
| `NYASH_ENABLE_USING=1` | ON | Any | using 文を有効化 |
| `HAKO_ENABLE_USING=1` | ON | Any | using 文 alias (.hako) |
| `NYASH_RESOLVE_TRACE=1` | OFF | Any | using/prelude 解決のトレース |
| `NYASH_RESOLVE_SEAM_DEBUG=1` | OFF | Any | using/prelude の結合境界マーカーを挿入（診断用） |
| `NYASH_RESOLVE_DUMP_MERGED=/path/to/out.hako` | unset | Any | using/prelude の text-merge 後ソースを指定パスにダンプ（診断用） |
| `NYASH_VM_DUMP_MERGED_HAKO=1` | OFF | Rust AST | using/prelude マージ後の Hako ソースをダンプ |
| `NYASH_PARSER_TRACE_STATIC=1` | OFF | Rust AST | static box parser の1行診断ログを有効化 |
| `NYASH_PARSER_SEAM_BREAK_ON_STATIC=1` | OFF | Rust AST | static box 内で継ぎ目由来の `static` を見た時に箱を閉じる compat shim |
| `NYASH_PARSER_SEAM_TOLERANT=1` | OFF | Rust AST | static box member level の bare `=` を text-merge seam として扱う compat shim |
| `NYASH_PARSER_STATIC_INIT_STRICT=1` | OFF | Rust AST | static initializer の `static` 判定を strict gate に通す |
| `NYASH_PARSER_METHOD_PARAM_STRICT=1` | OFF | Rust AST | static box method parameter list の unexpected token を fail-fast する |
| `NYASH_PARSER_METHOD_BODY_STRICT=1` | OFF | Rust AST | static box method body を strict method-body parser で読む |
| `NYASH_PARSER_TOKEN_CURSOR=1` | OFF | Rust AST | experimental TokenCursor expression path and newline handling bridge |

Throw surface policy:
- parser は `throw` を常時拒否する（`[freeze:contract][parser/throw_reserved]`）。
- `throw-compat` フラグは撤去済み。
- throw/catch/cleanup 実行経路の検証は `NYASH_TRY_RESULT_MODE=1` + JSON v0 canary を使用する。

---

## Runner / backend 選択

| 変数 | デフォルト | 適用経路 | 説明 |
| --- | --- | --- | --- |
| `NYASH_ROOT=/path/to/repo` | unset | Any | リポジトリルートのヒント（パス解決・ツール用途）。未指定なら自動推定 |
| `NYASH_WASM_ROUTE_POLICY=default\|legacy\|legacy-wasm-rust` | `default` | WASM (`--backend wasm`) | WASM 出力 route policy。`default`/`legacy-wasm-rust` のみ受理し、無効値は fail-fast（`[freeze:contract][wasm/route-policy]`）。 |
| `NYASH_WASM_ROUTE_TRACE=1` | OFF | WASM (`--backend wasm`) | route 決定時に1行の観測ログを出す（`[wasm/route-trace] policy=<...> plan=<...> shape_id=<...>`）。 |
| `NYASH_JSON_V0_IMPORT_TRACE=1` | OFF | Runner / Program(JSON v0) compat loader | `--json-file` compat umbrella intake の import-bundle 合流時に安定トレースを出す。summary は info-level で default-visible（`phase=<enter|skip|merge.done|fail>`）、詳細は `NYASH_RING0_LOG_LEVEL=DEBUG` でのみ出る（`phase=merge.begin|guard.set|restore`）。mainline MIR route trace ではなく、compat loader の hot/cold 切り分け専用。 |
| `NYASH_VM_USE_PY=1` | Removed (no-op) | Historical only | runtime/selfhost route 分岐は撤去済み。必要なら `tools/historical/pyvm/pyvm_runner.py` を直接使う |
| `NYASH_PIPE_USE_PYVM=1` | Removed (no-op) | Historical only | `--ny-parser-pipe` 分岐は撤去済み。必要なら historical スクリプトの direct route を使う |
| `NYASH_VM_PLUGIN_STRICT=1` | OFF | Any | 必須プラグイン欠如で fail-fast |
| `NYASH_FAIL_FAST=0` | ON | Any | フォールバックを許容（既定は拒否） |
| `NYASH_LLVM_USE_CAPI=1` | OFF | LLVM / backend-zero | LLVM C-API provider を有効化。`phase-29ck` runtime proof では pinned keep env。 |
| `HAKO_V1_EXTERN_PROVIDER_C_ABI=1` | OFF | LLVM / backend-zero | extern provider の C-ABI bridge を有効化。`phase-29ck` runtime proof では pinned keep env。 |
| `NYASH_LLVM_ROUTE_TRACE=1` | OFF | LLVM / backend-zero | backend route trace を stderr に 1 行で出す。`[llvm-route/select] owner=boundary recipe=<...> compat_replay=<...> symbol=<...>` と `[llvm-route/replay] lane=<none\|harness> reason=<...>` に加え、dev/optimization wave では `[llvm-route/trace] stage=<...> result=<...> reason=<...> extra=<...>` を emit する。perf/mainline の owner proof と route/window debug bundle の両方に使う。 |
| `HAKO_BACKEND_COMPILE_RECIPE=pure-first` | OFF | LLVM / backend-zero | backend-zero の transport hint。`.hako` daily compile は explicit recipe payload を渡し、Rust/C transport が boundary handoff でこの値を mirror する。recipe-aware caller は explicit `pure-first` FFI export を選び、generic C export には route 意味論を増やさない。 |
| `HAKO_BACKEND_COMPAT_REPLAY={none\|harness}` | OFF | LLVM / backend-zero | pure-first route で unsupported shape をどの compat keep へ流すかを示す transport hint。mainline/perf judge は `none` を正本にし、`harness` は explicit Stage0 keep lane だけで使う。 |
| `HAKO_AOT_LDFLAGS` | OFF | LLVM / backend-zero | AOT link の compat append ldflags。daily caller の main route は `LlvmBackendBox.link_exe(..., libs)` の 3rd arg。 |
| `HAKO_CAPI_PURE=1` | OFF | LLVM / compat-only pure-lowering | legacy alias / retire-target。historical pure C-API/FFI lowering route の旧 spelling。daily backend-zero route は `HAKO_BACKEND_COMPILE_RECIPE=pure-first` を正本にする。P96 以降は caller inventory → active smoke 置換 → warning/fail-fast の順で撤退する。 |

PyVM position:
- 日常運用は Rust VM / LLVM を使う（PyVM は historical / direct-only）。
- 退避ポリシーSSOT: `docs/development/current/main/design/archive/pyvm-retreat-ssot.md`

direct-v0 bridge policy:
- `--parser ny` は mainline 入口から削除済み（CLI で reject）。
- `NYASH_USE_NY_PARSER=1` は legacy no-op（mainline では direct-v0 route を起動しない）。
- retired route の freeze tag（`runtime-route/direct-v0-bridge-disabled`）は historical contract として保持し、silent fallback は許可しない。

---

## Runtime / Scheduler

| 変数 | デフォルト | 適用経路 | 説明 |
| --- | --- | --- | --- |
| `NYASH_SCHED_TRACE=1` | OFF | Any | Scheduler poll の移動/実行数をトレース |
| `NYASH_SCHED_POLL_BUDGET=1` | `1` | Any | 1回の poll で実行するタスク数の上限 |
| `NYASH_SCHED_POLL_IN_SAFEPOINT=1` | `1` | Any | `safepoint_and_poll()` で `sched.poll()` を呼ぶかを制御（`0` で無効、許容値以外は fail-fast） |
| `NYASH_HOST_HANDLE_ALLOC_POLICY=lifo\|none\|off\|no-reuse` | `lifo` | Any | host handle 再利用 policy。`lifo` は現行挙動、`none/off/no-reuse` は drop 後の handle 再利用を無効化（fresh 発番のみ）。 |

補足:
- `NYASH_SCHED_POLL_IN_SAFEPOINT` は `NYASH_GC_MODE` とは独立。
- `NYASH_GC_MODE=off` でも既定では poll は有効のまま（実行進行を維持）。
- `NYASH_SCHED_POLL_IN_SAFEPOINT=0` は perf/診断向けの明示トグル。
- 許容値: `0|1|off|on|false|true`（それ以外は `[freeze:contract][sched/poll_in_safepoint]`）。

---

## String / Unicode

| 変数 | デフォルト | 適用経路 | 説明 |
| --- | --- | --- | --- |
| `NYASH_STR_CP=1` | OFF | Any | 文字列の `length` / `indexOf` / `lastIndexOf` / `substring` をコードポイント基準にする（既定はバイト基準） |
| `NYASH_STRING_SPAN_CACHE_POLICY=on\|off\|enabled\|disabled\|1\|0` | `on` | Any | TLS string span cache の policy。`on` は現行挙動、`off` は span cache を bypass（lookup/put しない）。 |
| `NYASH_PERF_COUNTERS=1` | OFF | Any | `perf-observe` build でだけ canonical perf front の opt-in counter summary を process exit 時に stderr へ出す。default build では compile-out。current observe backend は TLS exact counter + current-thread flush で、route tags は `store.array.str` と `const_suffix`。 |
| `NYASH_PERF_BYPASS_GC_ALLOC=1` | OFF | Any | `perf-observe` build でだけ `MaterializeOwned` hot path の `global_hooks::gc_alloc(...)` を診断用に bypass する narrow debug knob。daily/mainline の owner や baseline を切り替える用途には使わない。 |
| `NYASH_PERF_TRACE=1` | OFF | Any | `perf-trace` build でだけ heavy trace / sampled probe lane を有効化する。現状は trace lane の placeholder sink を stderr に出すだけで、exact counter lane とは分離されている。 |

---

## PHI デバッグ関連 (Phase 277 P2 統合版)

**Phase 277 P2** で PHI 関連環境変数を **8個 → 3個** に統合しました。

| 変数 | デフォルト | 適用経路 | 説明 |
| --- | --- | --- | --- |
| `NYASH_LLVM_DEBUG_PHI=1` | OFF | LLVM | PHI生成・型推論・順序チェックのデバッグ出力 |
| `NYASH_LLVM_DEBUG_PHI_TRACE=1` | OFF | LLVM | PHI wiring詳細トレース、vmap変化の追跡 |
| `NYASH_LLVM_PHI_STRICT=1` | OFF | LLVM | PHI値解決時のゼロフォールバックを禁止（厳格モード） |

### 旧環境変数（Phase 278で削除完了）

以下の環境変数は **Phase 277 P2** で統合され、**Phase 278 P0** で削除されました。

**エラーメッセージ**:
```bash
❌ ERROR: NYASH_LLVM_PHI_DEBUG was removed in Phase 278. Use NYASH_LLVM_DEBUG_PHI=1 instead.
```

**マイグレーション（移行方法）**:

| 旧変数 | 新変数（SSOT） |
| --- | --- |
| `NYASH_LLVM_PHI_DEBUG` | `NYASH_LLVM_DEBUG_PHI=1` |
| `NYASH_PHI_TYPE_DEBUG` | `NYASH_LLVM_DEBUG_PHI=1` |
| `NYASH_PHI_ORDERING_DEBUG` | `NYASH_LLVM_DEBUG_PHI=1` |
| `NYASH_LLVM_TRACE_PHI` | `NYASH_LLVM_DEBUG_PHI_TRACE=1` |
| `NYASH_LLVM_VMAP_TRACE` | `NYASH_LLVM_DEBUG_PHI_TRACE=1` |

**理由**: Phase 277 P2 で環境変数を 8個→3個 に統合。Phase 278 で後方互換性削除（スプロール防止）。

### 使用例

```bash
# PHI一般デバッグ（生成・型・順序）
NYASH_LLVM_DEBUG_PHI=1 ./target/release/hakorune --backend llvm program.hako

# PHI詳細トレース
NYASH_LLVM_DEBUG_PHI_TRACE=1 ./target/release/hakorune --backend llvm program.hako

# 厳格モード（fail-fast）
NYASH_LLVM_PHI_STRICT=1 ./target/release/hakorune --backend llvm program.hako

# 組み合わせ
NYASH_LLVM_DEBUG_PHI=1 NYASH_LLVM_DEBUG_PHI_TRACE=1 \
  ./target/release/hakorune --backend llvm program.hako
```

### 出力例

**`NYASH_LLVM_DEBUG_PHI=1`:**
```
[phi_wiring/create] v36 dst_type=f64 -> phi_type=double
[phi_wiring/reuse] v36 predeclared PHI type matches: double
⚠️  [phi_wiring/CRITICAL] PHI type mismatch! v12: predeclared=i64 expected=double
```

**`NYASH_LLVM_DEBUG_PHI_TRACE=1`:**
```
[trace:phi] wire_process: dst=36, decl_b=3, v_src=12
[trace:phi] wire_pred_match: decl_b=3, pred_match=0
[trace:phi] add_incoming: dst=36, pred=0
```

---

## LLVM Build Pipeline

`tools/build_llvm.sh` で使用される環境変数。詳細は [`phase87-selfhost-llvm-exe-line.md`](../development/current/main/phase87-selfhost-llvm-exe-line.md) を参照。
この節の `NYASH_LLVM_COMPILER` は build-helper の mode selector だけを指すよ。backend-zero daily boundary の ny-llvmc path truth は `NYASH_NY_LLVM_COMPILER` だよ。

### Control Variables

| 変数 | デフォルト | 説明 |
| --- | --- | --- |
| `NYASH_BIN` | `./target/release/hakorune` | hakorune バイナリのパス |
| `NYASH_LLVM_COMPILER` | `crate` | `tools/build_llvm.sh` のローカル mode selector。`harness` または `crate`。mainline backend boundary の ny-llvmc path truth には使わない |
| `NYASH_NY_LLVM_COMPILER` | `target/release/ny-llvmc` | ny-llvmc バイナリのパス。backend-zero thin boundary / selfhost / stage1 helper で使う path truth |
| `NYASH_LLVM_FEATURE` | `llvm` | LLVM feature flag (`llvm` または `llvm-inkwell-legacy`) |
| `NYASH_LLVM_OBJ_OUT` | `target/aot_objects/<stem>.o` | オブジェクトファイル出力パス |
| `NYASH_CLI_VERBOSE` | `0` | 詳細ビルド出力を有効化 |

### Advanced Control Variables

| 変数 | デフォルト | 説明 |
| --- | --- | --- |
| `NYASH_LLVM_SKIP_EMIT` | `0` | オブジェクト生成をスキップ（既存 .o 使用） |
| `NYASH_LLVM_ONLY_OBJ` | `0` | オブジェクト生成後に停止（リンクスキップ） |
| `NYASH_LLVM_SKIP_NYRT_BUILD` | `0` | Nyash Kernel runtime ビルドをスキップ |
| `NYASH_LLVM_FORCE_NYRT_BUILD` | `0` | Nyash Kernel runtime のキャッシュがあっても再ビルドする（`tools/build_llvm.sh`） |
| `NYASH_LLVM_MIR_JSON` | (auto) | 事前生成 MIR JSON パス (crate mode) |
| `NYASH_LLVM_VALIDATE_JSON` | `0` | MIR JSON スキーマ検証を有効化 (crate mode) |
| `NYASH_LLVM_EMIT` | `obj` | 出力タイプ: `obj` または `exe` (crate only) |
| `NYASH_LLVM_NYRT` | `crates/nyash_kernel/target/release` | Nyash Kernel runtime パス |
| `NYASH_LLVM_LIBS` | (empty) | 追加リンクライブラリ |
| `NYASH_LLVM_USE_HARNESS` | explicit keep only | Python llvmlite の explicit compat/probe keep lane を要求する legacy hint。daily object emit の current owner ではない |

### LLVM harness debug（Python llvmlite）

| 変数 | デフォルト | 説明 |
| --- | --- | --- |
| `NYASH_LLVM_DUMP_IR=1` | OFF | 生成した LLVM IR を `<output>.ll` に書き出す（harness 実装側の簡易ダンプ） |
| `NYASH_LLVM_DUMP_IR=/path/to/out.ll` | unset | 生成した LLVM IR を指定パスに書き出す（`tools/build_llvm.sh` の内部経由でも可） |
| `NYASH_LLVM_TRACE_PHI=1` | OFF | PHI 配線/スナップショット解決の詳細トレース（Python backend） |
| `NYASH_LLVM_TRACE_VALUES=1` | OFF | value 解決トレース（Python backend） |
| `NYASH_LLVM_TRACE_OUT=/tmp/llvm_trace.log` | unset | LLVM トレースの出力先（未指定なら stdout） |
| `NYASH_LLVM_STRICT=1` | OFF | Python LLVM backend を Fail-Fast モードにする（snapshot miss / use-before-def / PHI不整合 を即エラー化） |
| `NYASH_LLVM_PHI_STRICT=1` | OFF | PHI の default-zero フォールバックを禁止し、incoming miss を即エラー化 |
| `NYASH_LLVM_FAST_NATIVE=1` | ON (`NYASH_LLVM_FAST=1` 時) | FAST lane の target machine を host CPU/features 向けに調整する（perf 向け）。`0` で generic target を強制。 |
| `NYASH_LLVM_OPT_LEVEL=0..3` | `2` | LLVM | llvmlite codegen の最適化レベル。`HAKO_LLVM_OPT_LEVEL` は互換 alias（未指定時のみ参照）。 |

### 使用例

```bash
# 基本ビルド（全デフォルト）
tools/build_llvm.sh program.hako -o output

# 詳細デバッグ
NYASH_CLI_VERBOSE=1 tools/build_llvm.sh program.hako -o output

# Crate mode + JSON検証
NYASH_LLVM_COMPILER=crate NYASH_LLVM_VALIDATE_JSON=1 \
  tools/build_llvm.sh program.hako -o output
```

---

## Selfhost compiler / Ny compiler

| 変数 | デフォルト | 適用経路 | 説明 |
| --- | --- | --- | --- |
| `NYASH_USE_NY_COMPILER=1` | OFF | JSON v0 | Ny selfhost コンパイラを使用 |
| `NYASH_NY_COMPILER_STAGE3=1` | OFF | JSON v0 | Ny コンパイラ子プロセスで Stage-3 surface を許可 |
| `NYASH_NY_COMPILER_TIMEOUT_MS=2000` | `2000` | JSON v0 | selfhost 子プロセスのタイムアウト (ms) |
| `NYASH_STAGE1_EMIT_TIMEOUT_MS=<ms>` | unset | JSON v0 | Stage-1 emit系（`--hako-emit-mir-json` / emit-program）のタイムアウト上書き。未指定時は `NYASH_NY_COMPILER_TIMEOUT_MS` を使用 |
| `NYASH_NY_COMPILER_EMIT_ONLY=1` | ON | JSON v0 | selfhost コンパイラを emit-only で動かす |
| `NYASH_NY_COMPILER_CHILD_ARGS="-- --min-json"` | unset | JSON v0 | 子プロセスへ透過する追加引数 |

### Ny compiler 経路の観測テンプレート (Phase 29)

```bash
# Ny compiler 経路で Program(JSON v0)→MIR→dump を観測
NYASH_USE_NY_COMPILER=1 \
NYASH_FEATURES=stage3 \
RUST_MIR_DUMP_PATH=/tmp/ny_selfhost_minimal.mir \
NYASH_CLI_VERBOSE=2 \
./target/release/hakorune --dump-mir apps/tests/minimal_ssa_skip_ws.hako \
  2>/tmp/ny_selfhost_minimal.log
```

確認項目:
- `/tmp/ny_selfhost_minimal.log` に以下の診断ログが出ているか:
  - `[selfhost/ny] spawning Ny compiler child process: ...`
  - `[selfhost/ny] received Program(JSON v0), size=... bytes`
  - `[selfhost/ny] lowering Program(JSON v0) → MIR via json_v0_bridge`
  - `[selfhost/ny] calling maybe_dump_mir (RUST_MIR_DUMP_PATH=..., cli_verbose=...)`
  - `[selfhost/ny] ✅ MIR dump file created` または `⚠️ MIR dump file NOT created`

注意: Ny selfhost compiler のエントリは現在 `lang/src/compiler/entry/compiler.hako` に統一されているよ。このファイルが存在しない場合、preferred child process 経路は発火しない。

---

## GC / Runtime

| 変数 | デフォルト | 適用経路 | 説明 |
| --- | --- | --- | --- |
| `NYASH_GC_MODE={auto|rc+cycle|off}` | `rc+cycle` | Any | GC モード選択。運用SSOTで固定されている比較対象は `rc+cycle/off`（意味論不変ゲート）。その他の値は fail-fast。 |
| `NYASH_GC_TRACE=1` | OFF | Any | GC トレース出力 (0-3) |
| `NYASH_GC_METRICS=1` | OFF | Any | GC メトリクス (text)。`rc+cycle` 時は optional GC 診断タグ（`[gc/optional:mode]`）も出力。 |
| `NYASH_VM_TRACE=1` | OFF | Any | VM 実行トレース |
| `NYASH_LEAK_LOG={1\|2}` | OFF | Any | Exit-time leak report (Phase 285)。`1`=summary counts, `2`=verbose (names/entries) |

---

## Smokes / Test Runner（tools）

`tools/smokes/v2/run.sh` とその周辺スクリプトで使う環境変数（Nyash 本体の意味論には影響しない）。

| 変数 | デフォルト | 適用経路 | 説明 |
| --- | --- | --- | --- |
| `SMOKES_REPRO=N` | OFF | tools | 失敗したスモークを最大 N 回だけ同条件で再実行する（フレーク検知）。ログを `/tmp/hakorune_smoke_*.log` に保持する。 |
| `HAKO_SILENT_TAGS={0\|1}` | `1` | tools | `[provider/select:*]` などの noisy tag line をフィルタする（`0` で raw を表示）。 |
| `NYASH_WASM_TARGETED_CARGO_TEST={0\|1}` | `1` | tools (`phase29cc_wsm_*`) | wasm smoke contract の `cargo test` を `--lib` / `--test wasm_demo_min_fixture` に絞って実行する。`0` で従来の広域 `cargo test <filter>` に戻す（診断向け）。 |

---

## プラグイン / Box

| 変数 | デフォルト | 適用経路 | 説明 |
| --- | --- | --- | --- |
| `NYASH_DISABLE_PLUGINS=1` | OFF | Any | プラグイン無効化 |
| `NYASH_BOX_FACTORY_POLICY={builtin_first|compat_plugin_first|strict_plugin_first}` | `builtin_first` | Any | Box factory の優先順位 |
| `NYASH_PLUGIN_EXEC_MODE={module_first|dynamic_only|dynamic_first}` | `module_first` | Any | プラグイン実行経路。`module_first` は Core6（Array/String/Map/Console/File/Path）を dynamic route から除外。Math/Net は compat lane（dynamic）維持。 |
| `NYASH_DEV_PROVIDER_TRACE=1` | OFF | Any | provider/box/method 選択の候補・採用ログを出力（dev-only）。 |
| `NYASH_FILEBOX_MODE={auto|plugin|builtin}` | `auto` | Any | FileBox 実装選択 |

---

## Operator Boxes（adopt）

OperatorBox の `apply` を「観測（observe）」から「採用（adopt）」へ切り替えるためのフラグ。

| 変数 | デフォルト | 適用経路 | 説明 |
| --- | --- | --- | --- |
| `NYASH_OPERATOR_BOX_COMPARE_ADOPT={0\|1}` | `0` | Any | CompareOperator.apply の adopt を有効化（既定OFF）。現在は CompareOperator が observe-only（Void返り）なので、意味論はVM側が常にSSOT。 |

---

## VM behavior（dev-only）

| 変数 | デフォルト | 適用経路 | 説明 |
| --- | --- | --- | --- |
| `NYASH_VM_TOLERATE_VOID=1` | OFF | Any | VM の比較/二項演算で Void を許容する（dev/diagnostic 専用）。既定OFFのままにする。 |

---

## ANF / Normalized (dev-only)

| 変数 | デフォルト | 適用経路 | 説明 |
| --- | --- | --- | --- |
| `HAKO_ANF_DEV=1` | OFF | Any | ANF routing を dev-only で有効化。撤去条件: ANF が本線化した時に削除 |
| `HAKO_ANF_ALLOW_PURE=1` | OFF | Any | PureOnly scope の ANF を許可（dev-only、`HAKO_ANF_DEV=1` 前提）。撤去条件: PureOnly ANF が本線化した時に削除 |
| `HAKO_ANF_STRICT=1` | OFF | Any | ANF の fail-fast を有効化（dev-only）。撤去条件: fail-fast を常時化した時に削除 |

補足: 実装は `src/config/env` に集約し、直読みはしない。

---

## JoinIR トグル (Phase 72 整理版)

JoinIR は制御構造を関数呼び出し + 継続に正規化する IR 層。フラグは config/env のポリシーで集約するよ。

**ポリシー入口**
- `joinir_core_enabled()` … JoinIR は常に ON。`NYASH_JOINIR_CORE` は deprecated で無視（0 を指定すると警告だけ出す）。
- `joinir_dev_enabled()` … `NYASH_JOINIR_DEV=1` または JoinIR debug level > 0 で ON（開発者向け束ねスイッチ）。

LoopBuilder は物理削除済みで、JoinIR を OFF にするモードは存在しない。

### Core（本線化対象）

| 変数 | デフォルト | 説明 |
| --- | --- | --- |
| `NYASH_JOINIR_EXPERIMENT` | OFF | JoinIR 実験メイントグル（Core 判定に含まれる） |
| `HAKO_JOINIR_IF_SELECT` | OFF | IfSelect/IfMerge JoinIR 経路。エイリアス `NYASH_JOINIR_IF_SELECT` は Deprecated。 |
| `HAKO_JOINIR_IF_IN_LOOP_ENABLE` | OFF | if-in-loop JoinIR 本線切替（Core 候補）。 |
| `NYASH_JOINIR_VM_BRIDGE` | OFF | VM bridge Route B。Core 判定に含まれる。 |
| `NYASH_JOINIR_LLVM_EXPERIMENT` | OFF | LLVM 経路 JoinIR 実験（ハーネス専用）。Core 判定に含まれる。 |
| ~~`NYASH_HAKO_CHECK_JOINIR`~~ | 削除済み | **Phase 124 で削除**: hako_check は JoinIR 専用化。環境変数不要。 |

補足:
- VM bridge Route B は `NYASH_JOINIR_EXPERIMENT=1` と併用が前提。
- VM bridge 経路は stdout を汚さない（ログは stderr のみ）。

### DevOnly（開発/計測専用）

| 変数 | デフォルト | 説明 |
| --- | --- | --- |
| `NYASH_JOINIR_DEV` | OFF | DevOnly まとめて ON。 |
| `NYASH_JOINIR_LOWER_FROM_MIR` | OFF | MIR ベース lowering 切替。 |
| `NYASH_JOINIR_LOWER_GENERIC` | OFF | 関数名フィルタなし generic lowering。 |
| `NYASH_JOINIR_VM_BRIDGE_DEBUG` | OFF | VM bridge 追加ログ。 |
| `NYASH_JOINIR_MAINLINE_DEBUG` | OFF | Mainline 追加ログ。 |
| `HAKO_JOINIR_PLANNER_REQUIRED` | OFF | Planner が None を返したとき legacy fallback を禁止する（single-case gate 用、既定OFF）。 |
| `HAKO_JOINIR_IF_IN_LOOP_DRYRUN` | OFF | if-in-loop dry-run。 |
| `HAKO_JOINIR_IF_TOPLEVEL` / `_DRYRUN` | OFF | ループ外 if JoinIR 経路 / dry-run。 |
| `HAKO_JOINIR_STAGE1` | OFF | Stage‑1 JoinIR 経路。 |
| `HAKO_JOINIR_PRINT_TOKENS_MAIN` | OFF | print_tokens main A/B。 |
| `HAKO_JOINIR_ARRAY_FILTER_MAIN` | OFF | array.filter main A/B。 |
| `NYASH_JOINIR_DEBUG` / `HAKO_JOINIR_DEBUG` | OFF | JoinIR デバッグログ（推奨: `HAKO_JOINIR_DEBUG=1`、`NYASH_*` は legacy）。 |

補足:
- `HAKO_JOINIR_DEBUG=0` のように **0 を明示した場合は無効**（envが存在しても debug にはならない）。

### Dev/Release プロファイル（簡易）

JoinIR の **strict / planner_required / debug は別トグル**。`--dev`（または `NYASH_DEV=1`）は **JoinIR strict を自動で
有効化しない**ので、必要なら明示的に足す。

| プロファイル | 目的 | 推奨セット |
| --- | --- | --- |
| release（既定） | 本番挙動 | 何も設定しない |
| dev（CLI） | 開発用の安全既定 | `--dev` または `NYASH_DEV=1` |
| joinir‑strict | fail‑fast / contract検証 | `HAKO_JOINIR_STRICT=1` |
| planner‑required | legacy fallback 禁止（ゲート用） | `HAKO_JOINIR_PLANNER_REQUIRED=1` |
| debug | JoinIR トレース | `HAKO_JOINIR_DEBUG=1..3` |

### Deprecated / 廃止候補

| 変数 | 状態 | 説明 |
| --- | --- | --- |
| `NYASH_JOINIR_CORE` | Deprecated | JoinIR 本線の ON/OFF トグルだったが、LoopBuilder 削除後は無効化不可。設定しても警告のみにして無視する。 |
| `HAKO_JOINIR_NESTED_IF` | Deprecated候補 | Route B nested if。 |
| `HAKO_JOINIR_READ_QUOTED` / `_IFMERGE` | Deprecated候補 | read_quoted JoinIR 実験。 |

### 使用例

```bash
# JoinIR は常に ON。Stage-3（推奨）
env NYASH_FEATURES=stage3 ./target/release/hakorune program.hako

# VM bridge Route B（開発用）
env NYASH_FEATURES=stage3 NYASH_JOINIR_EXPERIMENT=1 NYASH_JOINIR_VM_BRIDGE=1 ./target/release/hakorune program.hako

# LLVM ハーネス JoinIR 実験（explicit keep lane）
env NYASH_FEATURES=stage3 NYASH_LLVM_USE_HARNESS=1 \
    NYASH_JOINIR_EXPERIMENT=1 NYASH_JOINIR_LLVM_EXPERIMENT=1 \
    ./target/release/hakorune --backend llvm apps/tests/minimal_ssa_skip_ws.hako
```

詳細: [ENV_INVENTORY.md](../private/roadmap2/phases/phase-29-longterm-joinir-full/ENV_INVENTORY.md) / [Phase 72 フラグ整理](../private/roadmap2/phases/phase-72-joinir-dev-flags/README.md)

---

## MIR 検証系（代表）

| 変数 | デフォルト | 適用経路 | 説明 |
| --- | --- | --- | --- |
| `NYASH_VERIFY_ALLOW_NO_PHI=1` | OFF | Any | PHI 検証をスキップ |
| `NYASH_VERIFY_EDGE_COPY_STRICT=1` | OFF | Any | Edge copy 検証を厳格化 |
| `NYASH_VERIFY_RET_PURITY=1` | OFF | Any | return ブロックの純粋性検証 |
| `NYASH_ME_CALL_ARITY_STRICT=1` | OFF | Any | me.method の arity 不一致でエラー |
| `NYASH_MIR_DISABLE_OPT=1` | OFF | Any | MIR Optimizer 全体を無効化（開発/診断用、`src/mir/optimizer.rs`） |
| `NYASH_MIR_CONCAT3_CANON=1` | OFF | Any | MIR Pass 6.6 の concat3 正規化を有効化（`(a+b)+c` / `a+(b+c)` → `nyash.string.concat3_hhh`）。実験的opt-in。 |
| `NYASH_TRACE_VARMAP=1` | OFF | Any | `MirBuilder.variable_map` の状態をトレース出力（`[varmap/<tag>] {name=ValueId(..),..}`）。JoinIR loop 統合のデバッグ用。 |
| `NYASH_DCE_TRACE=1` | OFF | Any | DCE パスが削除した純粋命令を stderr にログ出力（`src/mir/passes/dce.rs`）。 |

### MIR / PHI diagnostics（dev-only）

| 変数 | デフォルト | 適用経路 | 説明 |
| --- | --- | --- | --- |
| `NYASH_PHI_TYPE_DEBUG=1` | OFF | Any | `PhiTypeResolver` の詳細トレース（`[phi/type] ...`） |
| `NYASH_PHI_META_DEBUG=1` | OFF | Any | PHI metadata の伝播トレース（PHI dst / incoming の追跡） |
| `NYASH_PHI_GLOBAL_DEBUG=1` | OFF | Any | PHI 型再推論のグローバルトレース（`[lifecycle/phi-*] ...`） |

---

## Perf Lane (tools/perf)

`tools/perf/*` でのみ使う計測専用トグル。ランタイム全体既定とは分離して運用する。

| 変数 | デフォルト | 適用経路 | 説明 |
| --- | --- | --- | --- |
| `PERF_AOT={0\|1}` | `0` | tools/perf (`bench_compare_c_vs_hako.sh`) | C/VM比較で AOT 行を追加出力するかを切り替える。`0/1` 以外は fail-fast。 |
| `PERF_SUBTRACT_STARTUP={0\|1}` | `0` | tools/perf (`bench_compare_c_vs_hako.sh`) | VM/AOT の startup baseline を差し引く。`0/1` 以外は fail-fast。 |
| `PERF_AOT_SKIP_BUILD={auto\|0\|1}` | `auto` | tools/perf | AOT helper の再ビルド制御。`auto` は必要成果物が揃っているときだけ build skip。`0` は常に build、`1` は常に skip。`auto/0/1` 以外は fail-fast。 |
| `PERF_AOT_AUTO_SAFEPOINT={0\|1}` | `0` | tools/perf (`bench_compare_c_py_vs_hako.sh`) | AOT 比較レーンで `NYASH_LLVM_AUTO_SAFEPOINT` を上書き。未指定時は `NYASH_LLVM_AUTO_SAFEPOINT` を fallback 参照。`0/1` 以外は fail-fast。 |
| `PERF_AOT_DIRECT_ONLY={0\|1}` | `*_hk` は `1`、それ以外 `0` | tools/perf (`bench_compare_c_py_vs_hako.sh`) | direct の `--emit-mir-json` のみで MIR 生成を試行。`1` で helper 再試行禁止（直行路で失敗したら skip）。 |
| `PERF_SKIP_VM_PREFLIGHT={0\|1}` | `0` | tools/perf | VM preflight を省略（AOT契約だけを見たい場合）。`0/1` 以外は fail-fast。 |
| `PERF_VM_FORCE_NO_FALLBACK={0\|1}` | `0` | tools/perf (`bench_compare_c_py_vs_hako.sh`) | VM/AOT 実行時に `NYASH_VM_USE_FALLBACK=0` を強制。`*_hk` key はこの値が `1` でないと fail-fast。 |
| `PERF_ROUTE_PROBE={0\|1}` | `1` | tools/perf (`bench_compare_c_py_vs_hako.sh`) | one-shot route probe を実行して `[bench4-route]`（`vm_lane` / `derust_source`）を出力。`0/1` 以外は fail-fast。 |
| `PERF_REQUIRE_AOT_RESULT_PARITY={0\|1}` | `*_hk` は `1`、それ以外 `0` | tools/perf (`bench_compare_c_py_vs_hako.sh`) | `1` のとき VM の `RC:` と AOT の `Result:` 一致を必須化。不一致は fail-fast（route drift / hidden fallback 検知）。 |
| `PERF_BUNDLE_KILO_MODE={strict\|diagnostic}` | `strict` | tools/perf (`bench_crosslang_apps_bundle.sh`) | APP-PERF-03 bundle の kilo lane 動作。`strict` は parity 必須、`diagnostic` は parity check をスキップして timing-only 実行。 |

---

## Macro システム (Phase 286A 集約版)

**Phase 286A** で Macro 系環境変数を `src/config/env/macro_flags.rs` に集約しました。

### Core（本線化対象）

| 変数 | デフォルト | 適用経路 | 説明 |
| --- | --- | --- | --- |
| `NYASH_MACRO_PATHS=comma,separated,paths` | unset | Any | MacroBox パス（カンマ区切り）。推奨経路。 |
| `NYASH_MACRO_ENABLE={0\|1}` | `1` | Any | Macro システム全体の有効化。`0` で完全無効化（lite プロファイル用）。 |
| `NYASH_MACRO_STRICT={0\|1\|true\|false}` | `true` | Any | Strict モード。エラー時に即失敗（未設定時は ON）。 |
| `NYASH_MACRO_TRACE={0\|1}` | `0` | Any | Macro トレース出力。 |
| `NYASH_MACRO_BOX={0\|1}` | `0` | Any | MacroBox システムの有効化。 |
| `NYASH_MACRO_BOX_ENABLE=list` | unset | Any | 有効化する MacroBox 名（カンマ区切り）。 |

### Legacy / Backward compat

| 変数 | デフォルト | 適用経路 | 説明 |
| --- | --- | --- | --- |
| `NYASH_MACRO_BOX_NY={0\|1}` | `0` | Any | Legacy: Nyash MacroBox ロード有効化。非推奨：`NYASH_MACRO_PATHS` を使用してね。 |
| `NYASH_MACRO_BOX_NY_PATHS=comma,separated,paths` | unset | Any | Legacy: Nyash MacroBox パス（カンマ区切り）。非推奨：`NYASH_MACRO_PATHS` を使用してね。 |
| `NYASH_MACRO_TOPLEVEL_ALLOW={0\|1\|true\|false}` | `false` | Any | Legacy: トップレベル `static function MacroBoxSpec.expand(json)` を許可。非推奨：BoxDeclaration を使用してね。 |
| `NYASH_MACRO_BOX_CHILD={0\|1\|true\|false}` | `true` | Any | 子プロセス MacroBox を使用。 |
| `NYASH_MACRO_BOX_CHILD_RUNNER={0\|1\|true\|false}` | `false` | Any | Legacy: Runner モードを強制。非推奨：自動管理されるから手動設定不要だよ。 |
| `NYASH_MACRO_BOX_NY_IDENTITY_ROUNDTRIP={0\|1}` | `0` | Any | Identity MacroBox の JSON ラウンドトリップ検証。 |

### Deprecated 警告動作 (Phase 286A)

**Phase 286A** で deprecated 環境変数の警告動作を明確化しました：

- **警告タイミング**: deprecated 変数が**明示的に設定されている場合のみ**警告を出力します。未設定時は警告なし。
- **警告フォーマット**: `[macro][compat] <変数名> is deprecated; <推奨アクション>`
- **対象変数**:
  - `NYASH_MACRO_TOPLEVEL_ALLOW` - BoxDeclaration を使用推奨
  - `NYASH_MACRO_BOX_CHILD_RUNNER` - 自動管理されるため設定不要
  - `NYASH_MACRO_BOX_NY*` - `NYASH_MACRO_PATHS` を使用推奨

**例**:
```bash
# 未設定 - 警告なし
./target/release/hakorune program.hako

# 設定あり - 警告あり
NYASH_MACRO_TOPLEVEL_ALLOW=1 ./target/release/hakorune program.hako
# stderr: [macro][compat] NYASH_MACRO_TOPLEVEL_ALLOW is deprecated; ...
```

### Capabilities（Capability ガード）

| 変数 | デフォルト | 適用経路 | 説明 |
| --- | --- | --- | --- |
| `NYASH_MACRO_CAP_IO={0\|1\|true\|false}` | `false` | Any | Macro から IO (File/Path/Dir) を許可。 |
| `NYASH_MACRO_CAP_NET={0\|1\|true\|false}` | `false` | Any | Macro から NET (HTTP/Socket) を許可。 |
| `NYASH_MACRO_CAP_ENV={0\|1\|true\|false}` | `false` | Any | Macro から環境変数読み取りを許可。 |

### Advanced / Dev-only

| 変数 | デフォルト | 適用経路 | 説明 |
| --- | --- | --- | --- |
| `NYASH_MACRO_MAX_PASSES=N` | unset | Any | Macro パス最大回数。 |
| `NYASH_MACRO_CYCLE_WINDOW=N` | unset | Any | Macro 検出ウィンドウ。 |
| `NYASH_MACRO_DERIVE_ALL={0\|1}` | `0` | Any | すべての MacroBox で派生マクロを有効化。 |
| `NYASH_MACRO_DERIVE=target` | unset | Any | 派生マクロのターゲット。 |
| `NYASH_MACRO_TRACE_JSONL=path` | unset | Any | Macro トレースを JSONL 形式で出力。 |
| `NYASH_MACRO_DISABLE={0\|1\|true\|false}` | `false` | Any | Macro システム全体の無効化（`NYASH_MACRO_ENABLE=0` と同等）。 |
| `NYASH_MACRO_BOX_EXAMPLE={0\|1}` | `0` | Any | Example MacroBox を有効化。 |

### Syntax Sugar

| 変数 | デフォルト | 適用経路 | 説明 |
| --- | --- | --- | --- |
| `NYASH_SYNTAX_SUGAR_LEVEL={none\|basic\|full}` | `none` | Any | 構文糖衣レベル。Macro スキャン中は一時的に `basic` に設定される。 |

### 使用例

```bash
# 基本使用（推奨）
NYASH_MACRO_PATHS=macros/my_macro.ny \
./target/release/hakorune --backend vm program.hako

# Strict モード
NYASH_MACRO_STRICT=1 \
./target/release/hakorune --backend vm program.hako

# Trace 出力
NYASH_MACRO_TRACE=1 \
./target/release/hakorune --backend vm program.hako

# MacroBox 有効化
NYASH_MACRO_BOX=1 NYASH_MACRO_BOX_ENABLE=MyMacroBox,AnotherBox \
./target/release/hakorune --backend vm program.hako

# Capability ガード
NYASH_MACRO_CAP_IO=1 NYASH_MACRO_CAP_NET=0 \
./target/release/hakorune --backend vm program.hako

# Legacy（非推奨）
NYASH_MACRO_BOX_NY=1 NYASH_MACRO_BOX_NY_PATHS=legacy_macros.ny \
./target/release/hakorune --backend vm program.hako
```

参考: [docs/guides/macro-profiles.md](../guides/macro-profiles.md) / [docs/guides/macro-box.md](../guides/macro-box.md) / [docs/guides/macro-box-nyash.md](../guides/macro-box-nyash.md)

---

参考: [docs/development/architecture/mir-logs-observability.md](../development/architecture/mir-logs-observability.md) / [src/mir/verification/](../../src/mir/verification/)
