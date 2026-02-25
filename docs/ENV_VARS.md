ENV Variables Index (Core, Builder, Smokes)

Purpose: quick reference for toggles used frequently during development and in smokes. Defaults aim to be safe and fail‑fast.

Runtime/VM
- NYASH_FAIL_FAST=0
  - Global relaxation switch; when set to 0/false, paired legacy guards may allow otherwise fail‑fast paths.
  - Use only for diagnosis; keep OFF in CI.

- NYASH_LEGACY_FIELDS_ENABLE=1
  - Materialize legacy InstanceBox shared fields map for compatibility with older code paths.
  - Default: OFF. Dev/compat only; safe to keep disabled in CI and normal runs.

- NYASH_JSON_ONLY=1
  - When tools/wrappers set this, Stage‑B and related emitters print only JSON to stdout. Auxiliary logs should be routed to stderr.
  - Smokes expect single‑line JSON on stdout for Program(JSON) and MIR(JSON) producers.

- NYASH_ENABLE_USING=1, HAKO_ENABLE_USING=1
  - Enable using/alias resolution in dev/VM runs. Smokes set both for symmetry.

Extern/Providers
- Arity suffix normalization
  - The VM accepts extern names with arity suffixes and normalizes them before dispatch:
    - Examples: `env.get/1` → `env.get`, `hostbridge.extern_invoke/3` → `hostbridge.extern_invoke`.
  - Implemented in `src/backend/mir_interpreter/handlers/calls/externs.rs` and `handlers/externals.rs`.

Plugins / Autoload
- NYASH_USING_DYLIB_AUTOLOAD=1
  - Enable autoload of `[using.<name>]` entries with `kind="dylib"` from `nyash.toml`.
  - The runner loads the specified shared libraries at startup and registers providers for the boxes declared in each plugin’s `nyash_box.toml`.
  - Default: OFF. Guarded to avoid surprises; respects `NYASH_DISABLE_PLUGINS=1`.

Parser/Stage‑B
- HAKO_PARSER_STAGE3=1, NYASH_PARSER_STAGE3=1
  - Accept Stage‑3 syntax (Break/Continue/Try/Catch, etc.). Enabled in smokes for Stage‑B runs.

- HAKO_STAGEB_FUNC_SCAN=1
  - Dev-only: inject a `defs` array into Program(JSON) with scanned method definitions for `box Main`.

- HAKO_STAGEB_MODULES_LIST
  - Stage‑B/Hakorune 向けの using 解決に使う `name=path` 連結文字列（`nyash.toml` の `[modules]` をシェル側で変換）。
  - 例: `lang.mir.builder.MirBuilderBox=lang/src/mir/builder/MirBuilderBox.hako|||lang.compiler.build.build_box=lang/src/compiler/build/build_box.hako`

- HAKO_STAGEB_APPLY_USINGS=0|1
  - `1` のとき、Stage‑B は `lang.compiler.entry.using_resolver_box` を通じて Stage1 用の using を text merge する。`0` で旧挙動。

- HAKO_STAGEB_BODY_EXTRACT=0|1
  - Toggle Stage‑B body extractor. When `0`, skip method‑body extraction and pass the full `--source` to `parse_program2`. Useful to avoid environment‑specific drift in extractors; default is `1` (enabled).

Selfhost builders and wrappers
- HAKO_SELFHOST_BUILDER_FIRST=1
  - Prefer the Hako MirBuilder path first; wrappers fall back to Rust CLI builder on failure to keep runs green.

- NYASH_LLVM_USE_HARNESS=1
  - Enable LLVM harness mode (ny‑llvmc crate backend). Used by builder scripts for EXE/OBJ emission.

Smokes
- SMOKES_DEFAULT_TIMEOUT
  - Per‑test timeout (seconds) used by `tools/smokes/v2/run.sh --timeout` or auto profile defaults. Quick profile defaults to ~15s.
  - Some tests wrap heavy steps (e.g., running a built EXE) with a shorter internal `timeout` to convert hangs into SKIP.
- HAKO_BUILD_TIMEOUT, HAKO_EXE_TIMEOUT
  - Internal timeouts (seconds) used by several phase2100 crate‑backend tests to bound ny‑llvmc build/link and EXE execution steps under quick.
  - Defaults: `HAKO_BUILD_TIMEOUT=10`, `HAKO_EXE_TIMEOUT=5`.

Notes
- Keep default behavior unchanged for users. Use these toggles in development and CI wrappers only.
- Avoid enabling legacy paths except for targeted diagnosis. The unified call system is the default in both builder and VM.

Using/Resolver
- HAKO_USING_RESOLVER_FIRST=1
  - Try the SSOT using resolver (`using::resolver::resolve_using_target_common`) first in the runner pipeline.
  - On failure, the pipeline falls back to the existing runner logic (aliases/builtins/plugins/relative search).
  - Default: OFF. Use to validate resolver‑first migration without changing defaults.

Builder/Emit (Selfhost)
- HAKO_SELFHOST_BUILDER_FIRST=1
  - Prefer Hako MirBuilder path first; delegates to provider/legacy on failure. Used by `tools/hakorune_emit_mir.sh` and bench scripts.
- HAKO_MIR_BUILDER_BOX=hako.mir.builder|min
  - Choose selfhost builder box (full or minimal runner).
- HAKO_SELFHOST_TRACE=1
  - Print additional traces during MIR emit bench/wrappers.

- HAKO_MIR_BUILDER_LOOP_FORCE_JSONFRAG=1（dev‑only）
  - 最小 MIR(JSON)（const/phi/compare/branch/jump/ret のみ）を強制生成する緊急回避。
  - emit が壊れているときの診断用途に限定。ベンチ/本番経路では使用しない。

- HAKO_MIR_BUILDER_JSONFRAG_NORMALIZE=1, HAKO_MIR_BUILDER_JSONFRAG_PURIFY=1（dev‑only）
  - JsonFrag の整形/純化ユーティリティ。比較/可視化の安定化が目的で、意味論や性能を変える“最適化”ではない。

Provider path (delegate)
- HAKO_MIR_NORMALIZE_PROVIDER=1
  - Provider（Rust）出力の MIR(JSON) に対して、Hako の JsonFrag 正規化パスを適用する（tools/hakorune_emit_mir.sh 内部）。
  - 互換維持のため既定はOFF。Box 系で ret ブロックに副作用命令が残るようなケースの暫定純化に利用できる。

- NYASH_LLVM_FAST_INT=1
  - Opt-in toggle for integer hot paths in LLVM MIR/IR. When `1`, `binop` や `compare` は同一ブロックの `vmap` 定義を最優先し、resolver/PHI を経由しない i64 経路を選びます。ループや分岐で i64 が綺麗に残るときだけ ON にして、CI 等では unset のまま。
 
 - NYASH_MIR_DEV_IDEMP=1
  - Dev-only: Enable idempotence markers inside MIR normalize passes. Each pass stamps `pass:function` in the module metadata after rewriting a function once, and subsequent optimizer runs skip reprocessing that function for the same pass. 仕様不変（既定OFF）。診断時に複数回最適化を呼んでも差分が出ないことを保証するための保険だよ。
- NYASH_LLVM_FAST=1
  - Enables NyRT-based FAST helpers (string length via `nyrt_string_length`, pointer re-use, etc.) and caches literal-backed `length/len` results so loops reuse the same constant value instead of re-calling the helper.
  - Also enables const-string intern helper (`nyash.box.from_i8_string_const`) and reuses literal-backed `new StringBox("...")` handles in LLVM fast lowering (AOT microbench向け)。const cache is capped (固定上限) and over-cap entries fall back to passthrough allocation. Default OFF.

- NYASH_MIR_LOOP_HOIST=1
  - AOT 前準備（AotPrepBox）での軽ホイスティングを有効化。固定文字列の `length/len` を即値に置換（JSON 書換え）する。制御フローは変更しない。既定 OFF。

- NYASH_AOT_COLLECTIONS_HOT=1
  - AOT 前準備（AotPrepBox）で Array/Map の `boxcall` を `externcall`（`nyash.array.*` / `nyash.map.*`）のホットパスに張り替える。AOT 専用の最短経路で、診断を省いてオーバーヘッドを抑える。既定 OFF。

- HAKO_MIR_NORMALIZE_PRINT=1
  - AotPrep の正規化パス（.hako）を有効化して、`print` 命令を `externcall env.console.log(value)` に書き換える（CFG 不変）。既定 OFF。

- HAKO_MIR_NORMALIZE_REF=1
  - AotPrep の正規化パス（.hako）を有効化して、`ref_get/ref_set` を `boxcall getField/setField` に書き換える（CFG 不変、best-effort）。既定 OFF。

- HAKO_MIR_NORMALIZE_ARRAY=1
  - AotPrep の正規化パス（.hako）を有効化して、`array_get/array_set`（および一部の `map_get/map_set`）を `boxcall get/set` に書き換える（CFG 不変、best-effort）。CollectionsHot の前処理として有効。既定 OFF。

- NYASH_AOT_INDEX_WINDOW=1
  - AotPrep(CollectionsHot 内部) の index 共有をブロック境界をまたいだ短窓で行う実験的トグル（デフォルト OFF）。
  - 窓はバイト数で固定（2048）。誤共有のリスクがあるため初期はベンチ限定で使用。

- NYASH_VERIFY_RET_PURITY=1
  - Ret ブロック純化の Fail-Fast トグル。Return の直前に `Const`・`Copy`・`Phi`・`Nop` 以外があると MIR/VM 実行が失敗するようにする開発ガード。既定 OFF。

- tools/perf/dump_mir.sh
  - MIR(JSON) を provider-first → jsonfrag フォールバックで吐く小さな wrapper。`--mode provider` で通常 builder 主導、`--mode jsonfrag` で最小化ループを強制。内部でブロックごとの op カウンターを出力するので、branch/map/kilo などのホットパスをすばやく可視化できるよ。

AOT/LLVM (ny-llvmc)
- HAKO_LLVM_OPT_LEVEL=0|1
  - ny-llvmc optimization level (default 0/O0). Bench scripts keep O0 unless overridden.
 - NYASH_LLVM_DUMP_MIR_IN=/path/to/out.json
   - ny-llvmc が受け取る MIR(JSON) をそのまま保存する開発用トグル。AotPrep 適用後の実入力を観測するのに使う。既定 OFF。

Bench helpers
- PERF_USE_PROVIDER=1
  - `tools/perf/microbench.sh` で provider/selfhost-first の MIR emit を優先（jsonfrag 強制を解除）。環境により provider 失敗時は自動で最小ループにフォールバック。
