# Phase 25.1a — Stage1 Build Pipeline Hotfix (Program→MIR)

Status: hotfix-in-progress（緊急タスク／配線修正フェーズ）

## ゴール

- Phase 25.1 で導入した Stage1 ランチャー（`lang/src/runner/launcher.hako`）と selfhost AOT パイプライン（`build_stage1.sh` 経由）を「実際に動く状態」に戻す。
- `.hako → Program(JSON v0) → MIR(JSON) → EXE` のうち、**Program(JSON v0) → MIR(JSON)** の導線を再構成し、`tools/hakorune_emit_mir.sh` / `tools/selfhost_exe_stageb.sh` / `tools/selfhost/build_stage1.sh` が代表ケースで成功するようにする。
- selfhost builder / provider / legacy CLI の 3 経路が混在した現状を見直し、**信頼できる1本の Program→MIR 経路**を中心に据える。

## 進捗状況（2025-11-16 時点）

- Stage-B と provider 経路で `.hako → Program(JSON) → MIR(JSON)` は安定。`hakorune_emit_mir.sh lang/src/runner/launcher.hako …` も `[OK] MIR JSON written (delegate:provider)`。
- selfhost builder (`HAKO_SELFHOST_BUILDER_FIRST=1`) では、Stage1 CLI の Program(JSON) を完全に lowering できず 171 bytes の stub MIR を返すため、現状 selfhost-first を既定ONにできない。
- Stage1 EXE（`/tmp/hakorune-dev`）は provider 経由の MIR を使えばリンク成功。ただし CLI I/O は JSON を出さず `Result: 0` のみ。JSON 出力契約に揃える作業は selfhost builder の修復後に実施予定。

## 現状の問題点（2025-11-15 時点）

- `tools/selfhost/build_stage1.sh`:
  - 現在の entry: `lang/src/runner/launcher.hako`（Stage1 CLI ランチャー）。
  - provider 経由 (`env.mirbuilder.emit`) では `.hako → Program(JSON) → MIR(JSON)` を安定して通せるが、selfhost builder (`MirBuilderBox` on VM) はまだ Stage1 CLI の Program(JSON) をフルで扱えず、最小パターンの stub MIR（171 bytes）を出力してしまう。
  - このため selfhost builder を既定ONにすると EXE が空挙動になるため、Phase 25.1a では暫定的に provider-first (`HAKO_SELFHOST_BUILDER_FIRST=0`) を維持しつつ、selfhost builder 側の機能範囲拡張を翌フェーズへ送っている。

- `tools/hakorune_emit_mir.sh` — Program→MIR 部分:
  1. Stage‑B（`compiler_stageb.hako`）:
     - `Stage-B: SUCCESS - Generated Program(JSON)` まで成功（`"version":0,"kind":"Program"` を含む JSON が得られている）。
  2. selfhost builder 経路（`try_selfhost_builder`）:
     - `builder_box=hako.mir.builder` で Runner を生成し、VM 経由で実行。
     - 当初は tmp ハコファイルに対して `Parse error: Unexpected token FN` や `Unexpected token ASSIGN` が発生し rc=1 で失敗していたが、`lang/src/mir/builder/func_lowering.hako` の `local fn = func_jsons.length()` を `local func_len = ...` にリネームすることで `Unexpected token FN` 自体は解消済み。
     - その後、`lang/src/shared/mir/loop_form_box.hako` と `lang/src/mir/builder/internal/lower_loop_count_param_box.hako` における `init` 予約語衝突（`local init = ...` / 引数名 `init`）を `start_value` 系にリネームし、LoopFormBox まわりの `Invalid expression` も解消済み。
     - 現在は selfhost builder Runner の実行フェーズで `VM error: Invalid instruction: global function: Unknown: self._is_on/1` が発生しており、MirBuilder 内部のトグルヘルパー `BuilderConfigBox._is_on` の呼び出し（`norm_if` ラムダ経由）が VM 側にまだ実装されていないために落ちている状態（構文ではなく実行時エラー）。
  3. provider 経路（`try_provider_emit` → `env.mirbuilder.emit`）:
     - 当初は `env.mirbuilder.emit` 実行時に `[mirbuilder/parse/error] undefined variable: args` により失敗していたが、Rust 側の Program→MIR ルート修正によりこのエラーは解消済み。現在は provider 経路経由で `launcher.hako` から MIR(JSON) を安定して生成できている。
  4. legacy CLI 経路（`--program-json-to-mir`）:
     - Program(JSON) を一時ファイルに書いて `nyash --program-json-to-mir` を叩くフォールバックも rc!=0 で終了していたが、Phase 25.1a では provider 経路の安定化を優先するため、現在は原則退避路とし、日常の導線では利用しない。

- Stage1 CLI (`launcher.hako`) の VM 実行:
  - `nyash --backend vm lang/src/runner/launcher.hako -- emit ...` で、
    - `using` の解決（`lang.compiler.build.build_box`）は nyash.toml に追加済みだが、
    - まだパーサが Stage‑3 構文/関数宣言の一部を受理できていない箇所があり、`Unexpected token ...` 系のエラーが残っている。

## Update (2025-11-16 — Stage‑B using resolverを module alias 化)

- `lang/src/compiler/entry/compiler_stageb.hako` で使用していたファイルパス形式の `using "..." as ...` を、`nyash.toml` `[modules]` に登録済みの module alias（`hako.compiler.entry.bundle_resolver` / `lang.compiler.entry.using_resolver`）へ置き換えた。
- `tools/hakorune_emit_mir.sh` が export する `HAKO_STAGEB_MODULES_LIST` / `HAKO_STAGEB_APPLY_USINGS` を Stage‑B が参照できるようになり、Stage‑3 パーサが `using` 行で失敗しなくなった（`HAKO_SELFHOST_TRACE=1 ./tools/hakorune_emit_mir.sh basic_test.hako /tmp/out.json` が `[emit:trace] Stage-B: SUCCESS ...` で止まらず `delegate:provider` まで進むことを確認）。
- これにより Phase 25.1a の Program→MIR ホットフィックスでは「Stage‑B→provider delegate」が安定経路になり、selfhost builder の修復に専念できる状態になった（Stage1 CLI 側の `using` も module alias 化を継続）。
- 追加で、`tools/hakorune_emit_mir.sh` 側の `HAKO_MIRBUILDER_IMPORTS` 生成ロジックを拡張し、`using ns.path.Type`（alias なし）の形式からも末尾セグメント（例: `MirBuilderBox`）を alias として抽出するようにした。これにより、`using lang.mir.builder.MirBuilderBox` を含む Stage1 CLI/Builder コードでも `env.mirbuilder.emit` が `undefined variable: MirBuilderBox` を出さずに Program(JSON v0) を lowering できるようになった。
- Stage1 CLI（`lang/src/runner/launcher.hako`）の emit/build コマンドを helper (`_read_file` / `_write_file`) で整理し、Stage‑3 friendly な `local` 宣言・ログメッセージに統一。`hakorune_emit_mir.sh lang/src/runner/launcher.hako …` を provider delegate で再度通し、62KB 超の MIR(JSON) が得られることを quick smoke（`phase251/stage1_launcher_program_to_mir_canary_vm.sh`）でカバー済み。
- Rust 側の JSON v0 ブリッジ（`src/runner/json_v0_bridge/lowering/expr.rs`）には `hostbridge` を well-known グローバルとして扱う最小の分岐を追加し、`hostbridge.extern_invoke(...)` を含む Program(JSON v0) でも `undefined variable: hostbridge` エラーで止まらないようにした（値は `Const(String("hostbridge"))` を発行する placeholder とし、実際の extern dispatch は VM/ランタイム側に委譲する）。


## フェーズ内タスク（25.1a TODO）

### A. Stage1 CLI ソースの VM 実行復旧

- [ ] `lang/src/runner/launcher.hako` を Stage‑3 パーサが素直に通る形に調整する。
  - [ ] 関数/ブロック構造・ローカル宣言のスタイルを既存の selfhost コードに合わせる（`function` 定義や `local` の位置など）。
  - [ ] `using lang.compiler.build.build_box as BuildBox` 経路を nyash.toml / hako_module.toml に統一し、「ファイルパス using」を完全に排除する。
- [ ] VM 実行スモーク:
  - [ ] `NYASH_ALLOW_NYASH=1 ./target/release/nyash --backend vm lang/src/runner/launcher.hako -- emit program-json apps/selfhost-minimal/main.hako` が parse error なく通ること。
  - [ ] 同様に `emit mir-json` / `build exe` も、少なくともエラーメッセージ付きで Fail-Fast するところまで確認する（VM 側での構文エラーがないこと）。

### B. Program→MIR selfhost builder 経路の安定化

- [ ] `try_selfhost_builder` 内で生成される tmp ハコファイル（`__BUILDER_BOX__` 版）を最小ケースで切り出し、単体で parse/実行できるように修正。
  - [ ] `args` 未定義エラーや `Invalid expression` の原因となっている記述を特定し、Runner 側の `Main.main(args)` などを正しく宣言する。
  - [ ] Stage‑3 構文の使用を必要最小限に抑え、selfhost builder 用 Runner のコードをシンプルに保つ。
- [x] Stage‑3 パーサで予約語となった `fn` をローカル変数名として使っている箇所（例: `lang/src/mir/builder/func_lowering.hako` の `local fn = func_jsons.length()`）をリネームし、`Unexpected token FN, expected identifier` を根本的に解消する。
  - [x] selfhost builder/min で使っていた `fn`（`norm_if` クロージャ）を helper メソッドに置き換え、lambda 構文を排除。Stage‑3 VM が `NewClosure` を扱わなくても builder が進むようにした。
  - [x] VM 側で prelude/text-merge 後の Hako コードを落とすデバッグ用トグルを追加（`NYASH_VM_DUMP_MERGED_HAKO=1` / `NYASH_VM_DUMP_MERGED_HAKO_PATH=<path>`）。selfhost builder 実行時は `HAKO_SELFHOST_DUMP_MERGED_HAKO=1` / `HAKO_SELFHOST_DUMP_MERGED_HAKO_PATH=/tmp/hako_builder_merged.hako` を通じて `/tmp/hako_builder_merged.hako` にマージ後の一時ハコを保存し、`Invalid expression at line <N>` の行を直接観察できるようにする。
- 進捗メモ（2025-11-15）:
  - `HAKO_SELFHOST_BUILDER_FIRST=1 HAKO_SELFHOST_DUMP_MERGED_HAKO_PATH=/tmp/hako_builder_merged.hako tools/hakorune_emit_mir.sh lang/src/runner/launcher.hako …` で 275KB の merged Hako が採取済み。`BuilderConfigBox` 内 `_is_on` など Stage‑3 で落ちやすい箇所を直接確認できる。
  - 同条件で `apps/selfhost-runtime/runner.hako` も `[OK] MIR JSON written (selfhost-first)` まで到達。stage3 parse 落ちは再現していないため、次回失敗したケースが出たら `/tmp/hako_builder_merged.hako` を差し替えて行番号を追跡する。
- [ ] `try_selfhost_builder` を **第一候補** とし、代表ケース（launcher.hako 等）で常にここが成功することを確認。
  - [ ] `HAKO_SELFHOST_BUILDER_FIRST=1` で `tools/hakorune_emit_mir.sh` を叩いたときに `[OK] MIR JSON written (selfhost-first)` まで到達することをスモークで確認。

### C. Provider / legacy delegate の整理

- [x] provider 経路（`env.mirbuilder.emit`）での `undefined variable: args` 原因を修正し、Stage‑B が出力する Program(JSON v0) を正しく受理できるようにする。
  - [ ] `HAKO_V1_EXTERN_PROVIDER` / `HAKO_V1_EXTERN_PROVIDER_C_ABI` トグルのデフォルトを見直し、「selfhost builder が成功するなら provider には落ちない」構造に寄せる。
- [ ] legacy CLI 経路（`--program-json-to-mir`）は、「selfhost builder が失敗したときだけ最後に試す」退避路として残しつつ、代表ケースでは通さない方針にする。
- [ ] 必要であれば Phase 25.1a 中は `HAKO_SELFHOST_NO_DELEGATE=1` を既定 ON に近い扱いにし、「selfhost builder が通る範囲」に問題を絞る。

### D. build_stage1.sh / selfhost_exe_stageb.sh 復旧

- [x] `NYASH_LLVM_SKIP_BUILD=1 tools/selfhost/build_stage1.sh --out /tmp/hakorune-dev` が 0 exit すること（現状は selfhost builder を既定OFFにし、provider ルートで MIR を生成）。
- [ ] 生成された `/tmp/hakorune-dev` について:
  - [ ] `Stage1 CLI emit` 系は現在 Result:0 のみで JSON を出さない（MIR が stub のため）ので、selfhost builder が機能するまで provider MIR を直接使う（`tools/selfhost/run_stage1_cli.sh` に JSON を書かせるタスクは後続）。
  - [ ] `./hakorune-dev build exe -o /tmp/hako_min apps/selfhost-minimal/main.hako` で簡単な EXE が生成され、実行して 0 exit を返すこと。
- [x] Stage1 CLI を実行する補助スクリプト `tools/selfhost/run_stage1_cli.sh` を追加し、`NYASH_NYRT_SILENT_RESULT=1` / `NYASH_DISABLE_PLUGINS=1` / `NYASH_FILEBOX_MODE=core-ro` を既定ONにした状態で CLI を呼び出せるようにした（llvmlite ハーネスと同じ JSON stdout 契約を満たすため）。
- [ ] `tools/selfhost_exe_stageb.sh` についても同様に `.hako → EXE` のスモークを通しておく（少なくとも launcher.hako / apps/selfhost-minimal/main.hako の2ケース）。
- 進捗メモ（2025-11-15）:
  - `FuncScannerBox` を拡張し、`static box` メソッドを `defs` に追加（暗黙 `me` もパラメータに補完）。
  - Rust 側 JSON v0 ブリッジに `ExprV0::Null` variant を追加して Stage‑B からの Null リテラルを受理。
  - `launcher.hako` の `while` を `loop` 構文へ置換し、Stage‑B パーサ互換に揃えた。
  - 依然として using 依存（例: `lang.mir.builder.MirBuilderBox`）を Stage‑B emit が解決できず、`env.mirbuilder.emit` が `undefined variable: MirBuilderBox` で停止。BundleResolver / using resolver を Stage‑B 経路に統合し、依存 Box を Program(JSON) に連結するのが次タスク。

### E. 次フェーズ（25.1b）に送る selfhost builder 強化項目

- `Program.defs` を MirBuilder 側でも処理し、`HakoCli.run` / `cmd_emit_*` / `cmd_build_*` などのメソッドを MIR 関数として生成する（現状は main 1 本のみ）。
- `func_lowering` / `call_resolve` 相当の処理を Hako 側に移植し、`Call("cmd_emit_mir_json")` が `Global` resolved call になるようにする。
- Loop / branch / compare / Array・Map 操作など Stage1 CLI で出現するステートメントを包括的に lowering するため、`lang/src/mir/builder/internal/*` の helper を本番経路に組み込む。
- JSON 出力を `jsonfrag` ベースで構造的に生成し、functions 配列に複数関数を格納できるようにする（文字列連結のみの暫定実装を置き換える）。
- 上記を満たした段階で `HAKO_SELFHOST_BUILDER_FIRST=1` を既定に戻し、Stage1 CLI バイナリの I/O（stdout JSON + exit code）を Rust/llvmlite と同じ契約に揃える。

## 25.1 / 25.1a / 25.2 の関係

- Phase 25.1:
  - Stage0/Stage1 の責務とバイナリレイアウトを設計し、Stage1 CLI（launcher.hako）の顔と構文を固めるフェーズ。
- Phase 25.1a（本ファイル）:
  - 「設計した Stage1 CLI / selfhost パイプラインが実際に動くようにする」緊急ホットフィックスフェーズ。
  - Scope はあくまで **Program→MIR と selfhost AOT の復旧** に限定し、numeric_core などの最適化には踏み込まない。
- Phase 25.2:
  - numeric_core AOT / microbench 統合・性能チューニングにフォーカス（`matmul_core` など）。
  - 25.1a で安定化した selfhost パイプラインの上に乗せる形で進める。

## Related docs

- `docs/private/roadmap2/phases/phase-25.1/README.md` … Stage0/Stage1 Bootstrap & Binary Layout（設計＋初期実装）。
- `docs/private/roadmap2/phases/phase-25/README.md` … Ring0/Ring1 再編と numeric_core BoxCall→Call パス。
- `docs/development/runtime/cli-hakorune-stage1.md` … Stage1 hakorune CLI のサブコマンド設計と実装範囲。
- `tools/hakorune_emit_mir.sh` … Stage‑B → Program(JSON v0) → MIR(JSON) の selfhost＋delegate パイプライン。
- `tools/selfhost_exe_stageb.sh` / `tools/selfhost/build_stage1.sh` … `.hako → MIR(JSON) → EXE` selfhost AOT パス。***
- Notes:
  - selfhost builder (`HAKO_SELFHOST_BUILDER_FIRST=1`) は依然として parse error で落ちるため、Phase 25.1a では **既定を 0（無効）** に切り替え、provider ルートを安定化させた。
  - builder-first 経路の再有効化は Phase 25.1a 中の後続タスクとして扱う。
