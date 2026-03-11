# Selfhost Bootstrap Route (SSOT)

Status: SSOT  
Scope: selfhost を「.hako コンパイラ → JoinIR → JSON v0 → VM」まで戻す最小経路の契約。

Related:
- CURRENT_TASK.md
- docs/development/current/main/design/selfhost-compiler-structure-ssot.md
- docs/development/current/main/10-Now.md
- docs/development/current/main/phases/phase-29bq/README.md
- docs/development/current/main/design/compiler-expressivity-first-policy.md
- docs/development/current/main/design/ai-handoff-and-debug-contract.md
- docs/development/current/main/design/selfhost-language-v1-freeze-ssot.md
- docs/development/current/main/design/hako-mirbuilder-migration-phase0-entry-contract-ssot.md
- docs/development/current/main/design/selfhost-parser-mirbuilder-migration-order-ssot.md
- docs/development/current/main/phases/phase-29cf/README.md
- docs/development/current/main/phases/phase-29ch/README.md
- docs/development/current/main/design/concurrency-async-pre-selfhost-ssot.md
- docs/development/current/main/design/pyvm-retreat-ssot.md

## Goal

selfhost を目的化せず、compiler-first の方針を守りつつ、
「.hako コンパイラが自己コンパイル可能な状態」へ戻すための最小経路を定義する。

## Reading Order

Restart / handoff では次の順で読む。

1. `CURRENT_TASK.md`
   - current blocker / next owner / latest accepted branch point
2. `docs/development/current/main/design/selfhost-parser-mirbuilder-migration-order-ssot.md`
   - final goal and migration order
3. `docs/development/current/main/design/selfhost-bootstrap-route-ssot.md`
   - current route authority and compatibility boundaries
4. `docs/development/current/main/design/selfhost-compiler-structure-ssot.md`
   - `.hako` / Rust ownership map and mainline structure
5. `docs/development/current/main/phases/phase-29ch/README.md`
   - active reduction slice
6. `docs/development/current/main/phases/phase-29cg/README.md`
   - solved reduced slice that must stay closed

## Policy (Steady State)

`Current blocker: none` の間は failure-driven 運用を維持する。
新規fixture追加や受理拡張は、freeze/reject・回帰・Decision変更が出た時だけ行い、
日常は軽量 gate で健康診断を回す。

Current blocker (2026-03-11):
- reduced `stage1-cli` authority itself remains green
- the `launcher-exe` widening slice is now green as well: `stage1` surrogate `BuildBox.emit_program_json_v0` materializes `HakoCli` defs/imports for `launcher.hako`, and the `stage1` surrogate `MirBuilderBox.emit_from_program_json_v0` injects `user_box_decls=[HakoCli, Main]`
- G1 full is now raw-exact green again on the same authority contract for both `Program(JSON v0)` and `MIR JSON v0`
- therefore there is no active reduced-case blocker on the authority route itself; next work is to choose the first true bootstrap reduction slice without changing the current authority contract

SSOT:
- `docs/development/current/main/design/recipe-tree-and-parts-ssot.md`

## Current Active Contract Snapshot (2026-03-09)

- mainline selfhost route は `stage1` / semantic wrapper / no-fallback を正本とする
- `phase-29cf` が `VM fallback compat lane` と `bootstrap boundary reduction` を accepted monitor-only で独立管理する
- `phase-29cg` が `stage1-cli` 時の Stage2 default-bootstrap dependency reduction を docs-first で管理する
- `phase-29ch` は `phase-29cg` の solved reduced case を入力に、MIR-direct bootstrap unification を別 phase で扱う
- `phase-29cg` の current contract は `raw NYASH_BIN replacement` ではなく `stage1-bridge helper contract を Stage2 build に持ち込む reduction` である
- `compat-fallback` は explicit compat keep であり、current caller authority は `phase29x_vm_route_non_strict_compat_boundary_vm.sh` / `phase29x_vm_route_observability_vm.sh` / `phase29x_vm_route_strict_dev_priority_vm.sh` に限定する
- 上の3本はさらに `route observability keep` / `strict-dev priority keep` / `non-strict compat boundary keep` に分けて扱い、generic monitor-only probe と混同しない
- `phase29x_derust_strict_default_route_vm.sh` は de-rust done-sync keep、`route_env_probe.sh` は current diagnostics keep、plugin route-resolver test は plugin test keep として別 bucket で管理する
- binary-only `--hako-emit-mir-json` / `--hako-run` は ported contract として monitor-only 運用する
- G1 identity (`tools/selfhost_identity_check.sh --mode full`) は現行 bootstrap contract の正本として維持する
- `tools/selfhost_identity_check.sh` は reduced case として artifact-kind=`stage1-cli` smoke lane で `NYASH_BIN=<stage1-cli>` bridge-first bootstrap を使う。raw direct `stage1-cli` replacement ではなく、helper-driven Stage1 bridge contract を Stage2 build に昇格した narrow reduction として扱う
- exact probe では `target/selfhost/hakorune.stage1_cli` は raw direct contract (`emit ...` / `--emit-mir-json`) で `97` を返す一方、`stage1_contract_exec_mode` は current reduced artifact に single-step source→MIR env contract を提供する。`tools/selfhost/run_stage1_cli.sh` は raw `emit program-json` / `emit mir-json` surface をこの env contract に変換する compatibility wrapper であり、新しい authority route ではない。したがって reduction target は `stage1-cli` binary の raw replacement ではなく、stage1-bridge helper contract を MIR-direct authority として Stage2 build に昇格することである
- `build_stage1.sh` の `stage1-cli bridge-first` bootstrap path は current reduced source (`lang/src/runner/stage1_cli_env.hako`) を `stage1_contract_exec_mode ... emit-mir <entry> <source_text>` で single-step source→MIR へ通し、`tools/ny_mir_builder.sh` には MIR(JSON) だけを渡す。source-only authority case では `stage1_cli_env.hako` が `MirBuilderBox.emit_from_source_v0(...)` を直接使い、explicit supplied Program(JSON) text がある時だけ `MirBuilderBox.emit_from_program_json_v0(...)` を compatibility input shape として残す。evidence として、`stage1_contract_exec_mode target/selfhost/hakorune.stage1_cli emit-mir apps/tests/hello_simple_llvm.hako "$(cat apps/tests/hello_simple_llvm.hako)"` と `stage1_contract_exec_mode target/selfhost/hakorune.stage1_cli emit-mir lang/src/runner/stage1_cli_env.hako "$(cat lang/src/runner/stage1_cli_env.hako)"` はともに `rc=0`、`bash tools/selfhost/run_stage1_cli.sh --bin target/selfhost/hakorune.stage1_cli emit program-json apps/tests/hello_simple_llvm.hako` / `... emit mir-json ...` も `rc=0`。`bash tools/dev/phase29ch_transient_boundary_probe.sh` は raw-exact quiet のままで、promoted source route と explicit supplied Program(JSON) compat input の semantic parity を維持している。さらに `bash tools/dev/phase29ch_stage1_cli_env_file_context_probe.sh` は previously red だった env-wrapper/source-route shapes (`env_branch_literal_empty`, `env_branch_helper_empty`, `env_branch_helper_env_text`, `env_branch_select_then_call`, `env_branch_same_callee_two_calls`, `mini_env`, `full`, `thin`, `thin_imports`) を fresh Stage1/Stage2 で green 固定した。reduced smoke case では `NYASH_BIN=target/selfhost/hakorune.stage1_cli bash tools/selfhost/build_stage1.sh --artifact-kind stage1-cli --out target/selfhost/hakorune.stage1_cli.next --force-rebuild` が green で、`tools/selfhost_identity_check.sh --mode smoke` も green。`tools/selfhost_identity_check.sh --mode full --skip-build --bin-stage1 target/selfhost/hakorune.stage1_cli --bin-stage2 target/selfhost/hakorune.stage1_cli.stage2` も raw-exact green. The remaining work is no longer the first source-route promotion; it is the next reduction slice after this source-mainline step.
- `phase-29ch` compare policy is split on purpose: authority and route identity stay exact, but G1 MIR comparison may start with a narrow semantic canonical compare while raw MIR exact diff remains tightening evidence. SSOT: `docs/development/current/main/design/selfhost-g1-mir-compare-policy-ssot.md`
- exact env-mainline route ids and their fail-fast hints are centralized in `tools/selfhost/lib/identity_routes.sh`. `tools/selfhost/build_stage1.sh` and `tools/selfhost_identity_check.sh` must consume those wrappers instead of carrying local copies, so owner-1 BoxShape cleanup does not fork route truth.
- `launcher-exe` helper/user-box closure bucket is now green: `NYASH_BIN=target/selfhost/hakorune.stage1_cli bash tools/selfhost/build_stage1.sh --artifact-kind launcher-exe --out target/selfhost/hakorune.launcher_from_stage1_cli --force-rebuild` passes, `stage1_contract_exec_mode ... emit-program lang/src/runner/launcher.hako ...` returns `defs_boxes=[HakoCli]`, `... emit-mir ...` returns `user_box_decls=[HakoCli, Main]`, and the built launcher no longer fails on `Unknown Box type: HakoCli` at startup
- G1 full-mode route guard も current authority に同期しており、`tools/selfhost_identity_check.sh --mode full` は `program-json=stage1-env-program` と `mir-json=stage1-env-mir-source` の exact route 以外では pass しない。`stage1-env-mir-program` / `stage1-env-mir-legacy` / `stage1-subcmd-mir-program` は compatibility-only boundary として残すが、reduced-case authority evidence には使わない
- supplied Program(JSON) compat path の shell-side SSOT は `tools/selfhost/lib/stage1_contract.sh` (`stage1_contract_run_bin_with_env` / `stage1_contract_exec_program_json_text` / `stage1_contract_exec_legacy_emit_mir_text`) -> `tools/selfhost/run_stage1_cli.sh` (`--from-program-json`) -> `tools/selfhost/lib/identity_routes.sh` (`run_stage1_subcmd_mir_program_compat_route`) で固定する。fresh compiled Stage1/Stage2 artifact の diagnostics probe (`tools/dev/phase29ch_program_json_compat_route_probe.sh`) は現在どちらも `stage1-env-mir-program` を返すため、live env-mainline compat route はもう text transport に寄っている。raw `run_stage1_cli.sh ... --from-program-json` も file input を 1 回 read したあと `stage1_contract_exec_program_json_text()` に合流するので、path lane は user-facing sugar に縮んだ。`stage1-env-mir-legacy` と `stage1-subcmd-mir-program` は cold compat keep として扱う。さらに `tools/dev/phase29ch_program_json_text_only_probe.sh` は fresh Stage1/Stage2 の両方で `text_only_rc=0` になったので、remaining compat resolver 自体は `*_PROGRAM_JSON_TEXT` だけで成立している
- `tools/selfhost/build_stage1.sh --artifact-kind stage1-cli` の post-build capability probe も同じ shared helper で `stage1-env-program` + `stage1-env-mir-source` を要求する。したがって stale/compat-only stage1-cli artifact は G1 まで進む前に build 時点で fail-fast する
- `tools/selfhost/build_stage1.sh` の stage1-cli bridge-first bootstrap 本体も同じ shared helper (`probe_exact_stage1_env_authority`) で MIR(JSON) を materialize する。build 本体 / build probe / G1 preflight-gate が同じ authority SSOT を見るので、manual `stage1_contract_exec_mode ... emit-mir` + local marker-grep を別 truth source として戻さない
- この文書の後半にある `BINARY-ONLY-*` / debt pack は active contract の補助 evidence であり、current blocker を直接定義しない

## Non-goals

- 大規模な仕様拡張
- AST rewrite による短絡的な通過
- fallback での隠れた成功

## Route IDs (SSOT)

selfhost の入口は複数あっても、契約はこの表を SSOT として固定する。
（入口を減らす前に、まず route ごとの責務と入出力を固定して drift を防ぐ）

| Route ID | Purpose | Entry | Input | Output | Acceptance |
| --- | --- | --- | --- | --- | --- |
| `SH-GATE-STAGEB` | selfhost gate の Stage-B コンパイル | `tools/selfhost/run_stageb_compiler_vm.sh --source-file <fixture>` | fixture source (`HAKO_SRC`) | Program(JSON v0) line on stdout | `tools/selfhost/run.sh --gate --planner-required 1` |
| `SH-RUNTIME-SELFHOST` | ランナー経由 selfhost pipeline（opt-in） | `tools/selfhost/run.sh --runtime --runtime-mode <stage-a|exe>` | `.ny/.nyash` source file | Program(JSON v0) -> MIR/VM execution | runtime execution + fast gate |
| `SH-JSONRUN` | JSON payload 実行 | `<nyash> --json-file <json>` | Program(JSON v0) or MIR(JSON) | VM exit code / stdout | gate stage2 run step |

運用ルール:
- route は stderr に安定タグで観測する（stdout は比較対象を汚さない）。
- route 追加/変更時は parity smoke も同コミットで更新する。
- PyVM は historical / opt-in 扱い。runtime/pipe route の既定導線には使わない（詳細: `pyvm-retreat-ssot.md`）。

Unified entry (ops):
- 日常運用の入口は `tools/selfhost/run.sh` を優先する（`--steady-state|--gate|--runtime|--direct`）。
- `--steady-state --quiet` を日常既定とし、詳細ログは `/tmp/phase29bq_selfhost_steady_state_*` で確認する。
- `run.sh` は薄い dispatcher とし、各 route の本体ロジックは既存スクリプト/ランナー側に残す。
- route smoke でも `run.sh` を wrapper 入口として使い、運用入口と検証入口を一致させる。

Emit route wrappers（Program→MIR）:
- SSOT entry:
  - `tools/smokes/v2/lib/emit_mir_route.sh --route {direct|hako-mainline|hako-helper} --out <mir.json> --input <src.hako>`
- legacy thin wrappers（互換用途）:
  - `tools/hakorune_emit_mir_mainline.sh <input.hako> <out.json>`
  - `tools/hakorune_emit_mir_compat.sh <input.hako> <out.json>`
- helper 実装:
  - `tools/hakorune_emit_mir.sh`
  - `HAKO_EMIT_MIR_MAINLINE_ONLY=1` のとき、Stage-B 失敗/invalid payload で direct fallback せず fail-fast する。

route contract note（fixed）:
- helper 撤退は route/wrapper の集約を意味し、JSON v0 bridge contract の削除は含まない。
- Program(JSON v0) を生成して MIR/VM に橋渡しする契約は本SSOTで維持する。
- したがって `hako-helper` は縮退対象でも、`json_v0` は保持対象である。

## Concurrency / Async Policy (SSOT)

selfhost 復帰の議論で混線しやすい点を、ここで固定する。

1) selfhost compiler の目的は「同じ JSON を吐く」ことであり、**compiler 自身の並列化は不要**（単一スレッドでよい）。

2) `nowait` / `await` は **既存の言語構文**であり、selfhost の都合で “消す/無視する” はしない。
   - 「並行性不要」≠「構文削除してOK」。
   - ただし selfhost compiler（`lang/src/compiler/**`）が現時点で依存していないなら、Phase-0/1 の範囲で “使わない” のは許可（依存はしない）。

3) `nowait` / `await` の **意味論と VM+LLVM の整合**は selfhost の外側で pin する（pre-selfhost stabilization）。
   - SSOT: `docs/development/current/main/design/concurrency-async-pre-selfhost-ssot.md`

4) `.hako mirbuilder` 移植の Phase-0 で async 構文に遭遇した場合は、回避せず **原因側で fail-fast** する（silent fallback 禁止）。
   - 先頭タグは Phase-0 契約に従い `[freeze:contract][hako_mirbuilder] ...` を用いる。

## Route Overview (SSOT)

1) **Parser parity (Rust ↔ .hako)**
   - AST の形と意味論を一致させる
   - span/error の契約を崩さない

2) **Resolver/using/macro contract**
   - using 解決・静的 box の defs 生成を安定させる
   - Stage-B の入口で欠けた defs は fail-fast
   - **Phase 29bq+**: `module_roots` を SSOT にして、Stage‑B に `HAKO_STAGEB_MODULE_ROOTS_LIST` を渡す
     - `hako.toml` の `[module_roots]` で最長 prefix 解決
     - `[modules]` は exact override（完全一致優先）
     - 0件/多重は `[freeze:contract][module_roots]` で fail-fast
     - 形式（SSOT）: `prefix=path` を `|||` で連結（末尾 `|||` なし）
     - SSOT: `docs/reference/language/using.md` → `module_roots` セクション
     - Drift check:
       - `rg -n "HAKO_STAGEB_MODULE_ROOTS_LIST" src/runner/stage1_bridge`

3) **.hako MirBuilder + JoinIR generation**
   - Facts → Recipe → CorePlan の契約を維持
   - planner_required の入口を維持する

4) **JSON v0 bridge contract**
   - Program.defs を必ず出力する
   - static_methods の解決を橋渡しで保証する
   - Stage‑3 exceptions（try/throw/catch/cleanup）は Bridge の Result‑mode lowering に pin する
     - gate/selfhost では `NYASH_TRY_RESULT_MODE=1` を固定する（legacy MIR Throw/Catch 経路は使わない）
     - Rust VM `Catch`/`Throw` 命令の実行実装は post-selfhost deferred。現行 lane は throw-free surface + JSON v0 Result-mode canary で契約監視する。
     - SSOT: `docs/guides/exceptions-stage3.md` / `docs/reference/architecture/parser_mvp_stage3.md`
   - `imports` は **Program.body で参照された alias のみ** compile/merge する（未参照は読み込まない）
     - 依存 compile は **強制で strict + planner_required** で実行する（環境揺れ防止）

5) **Runner/VM execution**
   - JSON v0 を Rust VM で実行し、期待出力を満たす
   - JoinIR VM bridge は `NYASH_JOINIR_EXPERIMENT=1` かつ `NYASH_JOINIR_VM_BRIDGE=1` のときのみ有効
   - VM bridge は stdout を汚さない（ログは stderr のみ）

## Bootstrap Stages (SSOT)

最短で後戻りを減らすための段階固定（compiler-first の自己ホスト）。

- **Stage0**: Rust コンパイラ（既存の hakorune）で `.hako` compiler をビルド
- **Stage1**: Stage0 で生成された `.hako` compiler
- **Stage2**: Stage1 で同一ソースを再ビルドした `.hako` compiler
- **Stage3 (optional)**: Stage2 で再ビルドした出力の一致確認（必要なら）

**Acceptance (SSOT)**:
- Stage1 と Stage2 の出力が一致すること（Program JSON v0 / MIR JSON v0 の比較で固定）
- 差分が出たらその stage で停止（自動で次へ進まない）

**Gate (G1)**:
- `tools/selfhost_identity_check.sh --mode full` — G1 done criteria（compiler_stageb.hako）
- `tools/selfhost_identity_check.sh --mode smoke` — 動作確認用（hello_simple_llvm.hako）
- 入力 SSOT (full): `lang/src/compiler/entry/compiler_stageb.hako`
- 入力 SSOT (smoke): `apps/tests/hello_simple_llvm.hako`
- PASS = 両方一致、FAIL = どちらか不一致
- Binary contract (重要):
  - full mode は **Stage1 CLI emit capability** を持つ binary を前提とする
  - `launcher-exe`（`tools/selfhost/build_stage1.sh` default artifact）は run 用であり、G1 emit identity の前提を満たさない
  - artifact kind は `tools/selfhost/build_stage1.sh --artifact-kind ...` と `<out>.artifact_kind` で明示する
- Build options:
  - デフォルト: Stage1/Stage2 をビルド（~35GB+ RAM 必要）
  - `--skip-build`: prebuilt binaries を使用（比較のみ、メモリ制約環境向け）
  - `--bin-stage1 <path>`, `--bin-stage2 <path>`: prebuilt binary のパス指定
- Note: ビルドが OOM する場合は `--skip-build` + 外部マシンでビルドした binaries を使用
- Latest evidence (2026-03-11):
  - `tools/selfhost_identity_check.sh --mode smoke` PASS
  - `tools/selfhost_identity_check.sh --mode full --skip-build --bin-stage1 target/selfhost/hakorune.stage1_cli --bin-stage2 target/selfhost/hakorune.stage1_cli.stage2` PASS (`Program JSON v0` raw match; `MIR JSON v0` raw match)

## Binary-only `--hako-emit-mir-json` Contract (lane B)

目的:
- `hakorune` 単体バイナリで Stage1 emit-mir route を実行可能にし、repo checkout 依存を除去する。

定義（success）:
- repo外ディレクトリで次が成功すること。
  - `./hakorune --hako-emit-mir-json /tmp/out.mir ./input.hako`
- MIR(JSON) が `/tmp/out.mir` に生成されること。
- fail時は stage1 fail-fast 契約（例: `[stage1-cli] ...`）で終了し、silent fallback しないこと。
- timeout diagnostics（non-gating）:
  - pin: `tools/smokes/v2/profiles/integration/apps/phase29y_hako_emit_mir_continue_assignment_timeout_block_vm.sh`
  - timeout source: `src/config/env/selfhost_flags.rs` `ny_compiler_emit_timeout_ms()`（unset時は `ny_compiler_timeout_ms()`） / `src/runner/stage1_bridge/mod.rs` `spawn_with_timeout(...)`
  - debug repro: `NYASH_STAGE1_EMIT_TIMEOUT_MS=12000 bash tools/smokes/v2/profiles/integration/apps/phase29y_hako_emit_mir_continue_assignment_timeout_block_vm.sh`

許可する外部依存:
- 入力ソース `input.hako`
- 出力先 `/tmp/out.mir`

禁止する内部依存（binary-only 達成条件）:
- `lang/src/**` の `.hako` ファイル読込
- `hako.toml` / `hakorune.toml` / `nyash.toml` の読込
- `*_module.toml` の読込

現状フロー（2026-02-18）:
1. Rust parent が stage1 route を起動する。
2. repo内（`lang/src/**` あり）では Stage1 child が using 解決・Stage-B・MirBuilder を実行する。
3. repo外（`lang/src/**` なし）では `NYASH_STAGE1_BINARY_ONLY_DIRECT=1` 明示時のみ binary-only direct route が Rust 側で MIR を生成して出力する（unset は OFF）。
4. どちらの経路でも parent が MIR(JSON) を出力ファイルへ書き込む。

固定順序（1 blocker = 1 fixture = 1 smoke = 1 commit）:
1. `BINARY-ONLY-B01` [done, 2026-02-18]: blocked smoke を追加し、repo外実行での現状依存を固定する。
   - smoke: `tools/smokes/v2/profiles/integration/apps/phase29y_hako_emit_mir_binary_only_block_vm.sh`
   - note: B04 完了後は歴史互換ラッパー（legacy alias）として `*_ported_vm.sh` へ委譲（active contract は B04）。
2. `BINARY-ONLY-B02` [done, 2026-02-18]: `stage1_cli.hako` のファイル依存を埋め込みへ移す（default route の entry path 外部依存を撤去）。
3. `BINARY-ONLY-B03` [done, 2026-02-18]: modules map 依存（TOML / module manifests）を埋め込み snapshot へ移す（`NYASH_STAGE1_MODULES_SOURCE=toml` で従来収集に切替可能）。
4. `BINARY-ONLY-B04` [done, 2026-02-18]: binary-only smoke を ported へ昇格し、lane B monitor-only へ戻す。
   - smoke: `tools/smokes/v2/profiles/integration/apps/phase29y_hako_emit_mir_binary_only_ported_vm.sh`
   - note: smoke 実行時は `NYASH_STAGE1_BINARY_ONLY_DIRECT=1` を明示して binary-only direct route を起動する。

運用メモ（命名）:
- active contract 実行は `phase29y_hako_emit_mir_binary_only_ported_vm.sh` を正本とする。
- `phase29y_hako_emit_mir_binary_only_block_vm.sh` は B01 履歴互換の legacy alias としてのみ保持する。
- legacy alias の撤去条件: `CURRENT_TASK.md` / `phase-29y` docs / 呼び出しスクリプトから `*_block_vm.sh` 参照が 0 件になった時点。

観測ルール（計測）:
- `strace -ff -e openat` で `--hako-emit-mir-json` 実行時の read を観測する。
- 非ゲートの monitor smoke:
  - `tools/smokes/v2/profiles/integration/apps/phase29y_hako_emit_mir_preemit_io_monitor_vm.sh`
- drift 疑い時のみ strict triage:
  - `bash tools/smokes/v2/profiles/integration/apps/phase29y_hako_emit_mir_preemit_io_monitor_vm.sh --strict`

## Binary-only `--hako-run` Contract (lane B)

目的:
- `hakorune` 単体バイナリで Stage1 run route を実行可能にし、repo checkout 依存を除去する。

定義（success）:
- repo外ディレクトリで次が成功すること。
  - `./hakorune --backend vm --hako-run ./input.hako`
- fail時は stage1 fail-fast 契約で終了し、silent fallback しないこと。

禁止する内部依存（binary-only 達成条件）:
- `lang/src/**` の `.hako` ファイル読込
- `hako.toml` / `hakorune.toml` / `nyash.toml` の読込
- `*_module.toml` の読込

固定順序（1 blocker = 1 fixture = 1 smoke = 1 commit）:
1. `BINARY-ONLY-RUN-01` [done, 2026-02-19]: blocked smoke を追加し、repo外 `--hako-run` の現状 blocker（`lang/src/**` read fail-fast）を固定する。
   - smoke: `tools/smokes/v2/profiles/integration/apps/phase29y_hako_run_binary_only_block_vm.sh`
2. `BINARY-ONLY-RUN-02` [done, 2026-02-19]: stage1 run route に binary-only direct route を追加し、repo file 依存（`lang/src/**` read）を撤去する。
3. `BINARY-ONLY-RUN-03` [done, 2026-02-19]: repo外 `--hako-run` を ported 契約へ昇格し、monitor-only へ戻す。
   - smoke: `tools/smokes/v2/profiles/integration/apps/phase29y_hako_run_binary_only_ported_vm.sh`
   - note: smoke 実行時は `NYASH_STAGE1_BINARY_ONLY_RUN_DIRECT=1`（または `NYASH_STAGE1_BINARY_ONLY_DIRECT=1`）を明示して direct route を起動する。

運用メモ（命名）:
- active contract 実行は `phase29y_hako_run_binary_only_ported_vm.sh` を正本とする。
- `phase29y_hako_run_binary_only_block_vm.sh` は RUN-01 履歴互換の legacy alias としてのみ保持する。
- legacy alias の撤去条件: `CURRENT_TASK.md` / `phase-29y` docs / 呼び出しスクリプトから `*_block_vm.sh` 参照が 0 件になった時点。
- backend mismatch（例: `--backend llvm --hako-run`）は non-gating pin で fail-fast 契約を固定する:
  - `tools/smokes/v2/profiles/integration/apps/phase29y_hako_run_binary_only_backend_mismatch_block_vm.sh`
- selfhost readiness proxy（non-gating）:
  - `tools/smokes/v2/profiles/integration/apps/phase29y_hako_binary_only_selfhost_readiness_vm.sh`
  - contract:
    - repo外 `--hako-emit-mir-json` を 2 回連続実行し、canonical MIR が一致すること（N->N+1->N+2 の代理固定）。
    - 同一 workdir で `--backend vm --hako-run` が成功し、stale blocker が再発しないこと。
  - note:
    - これは binary-only 導線の安定性固定であり、`stage1 -> stage2` 実バイナリ生成の G1 identity 完了を置き換えるものではない。

## Lane-B Nested Ternary Debt Pack (B-TERNARY-01..03)

目的:
- Rust route 先行で観測された nested ternary parity debt を、fail-fast境界を崩さずに段階的に縮退する。

固定順序（1 blocker = 1 fixture = 1 smoke = 1 commit）:
1. `B-TERNARY-01` [done, 2026-02-25]:
   - 対象: probe形（int/int固定）以外の nested ternary（var/int 混在）を最小受理で追加する。
   - 受け入れ:
     - baseline parity lock: `STRICT=1 tools/smokes/v2/profiles/integration/apps/phase29y_hako_emit_mir_nested_ternary_debt_probe_vm.sh`
     - var-values acceptance lock: `tools/smokes/v2/profiles/integration/apps/phase29y_hako_emit_mir_nested_ternary_var_values_lock_vm.sh`
   - note:
     - `phase29y_hako_emit_mir_nested_ternary_var_values_min.hako` は `ternary_no_lower` ではなく MIR 出力まで到達する。
     - canonical signature mismatch は B-TERNARY-03 判定対象として保持する。
2. `B-TERNARY-02` [done, 2026-02-25]:
   - 対象: 未対応形は `unsupported:ternary_no_lower` を維持し、fail-fast境界を fixture+smoke で固定する。
   - 受け入れ:
     - fixture: `apps/tests/phase29y_hako_emit_mir_nested_ternary_unsupported_boundary_min.hako`
     - smoke: `tools/smokes/v2/profiles/integration/apps/phase29y_hako_emit_mir_nested_ternary_unsupported_boundary_vm.sh`
3. `B-TERNARY-03` [done, 2026-02-25]:
   - 対象: parity lock を lane-B fast gate へ昇格するか判定し、採否を docs に固定する（昇格/据え置きの二択）。
   - 判定入力:
     - `phase29y_hako_emit_mir_nested_ternary_debt_probe_vm.sh`（strict/non-strict）
     - `phase29y_hako_emit_mir_nested_ternary_unsupported_boundary_vm.sh`
   - 判定出力:
     - 昇格する場合: lane-B fast gate への組み込み差分を同コミットで固定。
     - 昇格しない場合: non-gating diagnostic pin 維持を明記して終了。
   - decision (2026-02-25):
     - `phase29y_hako_emit_mir_nested_ternary_var_values_min.hako` で canonical signature mismatch が残存するため、
       lane-B fast gate への昇格は **据え置き**（non-gating diagnostics 維持）とする。

禁止:
- B-TERNARY-01 と B-TERNARY-02/03 を同コミットで混在させない。
- fail-fast marker を silent fallback へ置き換えない。

## Boundary shrink order (SSOT)

移植順の最短ルート（後戻り防止）:

1) **Stage1/Stage2 同一性テストを先に固定**
2) **module_roots / using 解決を `.hako` 側へ移す**
3) **AST JSON v0 を境界として固定し、parser は最後に回す**

single-developer の運用順は次を参照する:
- `docs/development/current/main/design/selfhost-parser-mirbuilder-migration-order-ssot.md`

## Subset expansion order (SSOT)

fixture の拡張は “境界条件を1つだけ” で増やす（混ぜない）。

1) 線形（stmt-only）
2) If（joinなし → joinあり）
3) Loop（break/continueなし → continueのみ → breakあり）
4) nested loop + if（segmenter で割る前提）

## Entry Points (code)

- Stage-B entry: `lang/src/compiler/entry/compiler_stageb.hako`
- Func scanner: `lang/src/compiler/entry/func_scanner.hako`
- JSON v0 bridge: `src/runner/json_v0_bridge/ast.rs`, `src/runner/json_v0_bridge/lowering.rs`
- Selfhost gate: `tools/selfhost/run.sh --gate --planner-required 1`

## Acceptance / Gates

- Fast gate: `./tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh`
- Selfhost canary (opt-in):
  `./tools/selfhost/run.sh --gate --planner-required 1 --timeout-secs 120`
  - 実行時は `HAKO_JOINIR_STRICT=1` / `HAKO_JOINIR_PLANNER_REQUIRED=1` を **Stage‑B と --json-file の両方で同じ条件に固定**
  - PASS判定は `HAKO_JOINIR_DEBUG=0` を推奨（stdout 比較の揺れを避ける）
  - 並列実行は `SMOKES_SELFHOST_JOBS`（または `run.sh --gate --jobs <n>`）で制御し、既定は `4`（必要時は `1` で直列）
  - latest evidence (2026-02-08): `RUN_TIMEOUT_SECS=120 SMOKES_ENABLE_SELFHOST=1 HAKO_JOINIR_PLANNER_REQUIRED=1 bash tools/smokes/v2/profiles/integration/selfhost/phase29bq_selfhost_planner_required_dev_gate_vm.sh` PASS（`187/187`, `total_secs=798`, `avg_case_secs=4.27`）
- Route parity smoke (Stage-B entry drift check):
  `bash ./tools/smokes/v2/profiles/integration/selfhost/phase29bq_selfhost_stageb_route_parity_smoke_vm.sh`
  - `SH-GATE-STAGEB`（wrapper）と direct route の Program(JSON v0) が一致することを確認する
- Runtime route smoke (route-tag contract check):
  `bash ./tools/smokes/v2/profiles/integration/selfhost/phase29bq_selfhost_runtime_route_smoke_vm.sh`
  - `SH-RUNTIME-SELFHOST` の route tag（`pipeline-entry` + `stage-a`）が stderr に出ることを固定する
  - stage-a は `[contract][runtime-route][accepted=mir-json]` を 1 行出力することを固定する
  - EXE route 確認時は `bash ./tools/smokes/v2/profiles/integration/selfhost/phase29bq_selfhost_runtime_route_smoke_vm.sh <fixture> exe`
- Runtime route reject smoke (strict+planner_required contract check):
  `bash ./tools/smokes/v2/profiles/integration/selfhost/phase29bq_selfhost_runtime_route_program_reject_smoke_vm.sh`
  - stage-a が Program(JSON v0) を受けた場合、`[contract][runtime-route][expected=mir-json]` + `rc=1` で fail-fast することを固定する
- Runtime mode parity smoke (stage-a/exe semantic parity):
  `bash ./tools/smokes/v2/profiles/integration/selfhost/phase29bq_selfhost_runtime_mode_parity_smoke_vm.sh`
  - 同一 `.ny` 入力で `stage-a` と `exe` の semantic result が一致することを固定する
- 追加受理形は必ず fixture + fast gate で固定する

## Blocker Capture (planner_required)

BoxCount 作業で “落ちた瞬間に” 最短で状況を固定するための採取手順（SSOT）。

- 実行（ログは `/tmp` に固定）:
  - `./tools/smokes/v2/profiles/integration/joinir/phase29bq_collect_planner_required_blocker_vm.sh apps/tests/<fixture>.hako <label>`
- 生成物:
  - `/tmp/phase29bq_joinir_blocker_<label>_<pid>.log`（生ログ）
  - `/tmp/phase29bq_joinir_blocker_<label>_<pid>.summary`（最初の freeze/reject 1行 + 直近 StepTree root）

### Record format (SSOT)

採取した内容は「再開時に迷わない最小セット」だけを SSOT に記録する。

- 記録先: `CURRENT_TASK.md` と `docs/development/current/main/phases/<phase>/README.md`
- 記録する2行（どちらも “1行だけ”）:
  1) `/tmp/*summary` の先頭1行（例: `fixture=...`）
  2) `/tmp/*summary` の `first_freeze_or_reject` の1行
     - 見当たらない場合は `first_freeze_or_reject: not found` と明記する

## Resume conditions (minimal)

selfhost の “復帰作業” を再開する条件:

- Loop acceptance-by-composability（RecipeBlock contract）へ十分寄っている（Loop RecipeBlockization v0）:
  - SSOT: `docs/development/current/main/design/recipe-tree-and-parts-ssot.md`
- RecipeVerifier (dev/strict) が有効で、契約違反が `[freeze:contract][recipe]` で止まる
- `.hako mirbuilder` migration の pin smokes が green（Phase-0 以降の導線が固定されている）
  - SSOT: `docs/development/current/main/design/hako-mirbuilder-migration-phase0-entry-contract-ssot.md`
- selfhost language v1 freeze が固定され、selfhost compiler の surface が v1 に揃っている
  - SSOT: `docs/development/current/main/design/selfhost-language-v1-freeze-ssot.md`
- fast gate が継続して green（退行なし）

## OOM/Canary Log Path (SSOT)

- Selfhost gate:
  - `SMOKES_ENABLE_SELFHOST=1 ./tools/smokes/v2/profiles/integration/selfhost/phase29bq_selfhost_planner_required_dev_gate_vm.sh`
- Logs:
  - `/tmp/phase29bq_selfhost_*.log`（既存のログ規約）
- Note:
  - canary JSON 実行時は debug-fuel unlimited を固定（必要時）

## Invariants (Fail-Fast)

- AST rewrite 禁止（CondCanon/UpdateCanon などの analysis-only view を使う）
- fallback 禁止（[freeze:contract] で明示的に落とす）
- Facts は Recipe を返し、Lower は Recipe のみを見る

## Return-to-selfhost checklist (minimal)

- Stage-B が Program.defs を含む JSON v0 を出力できる
- planner_required のタグが欠落しない
- JSON v0 bridge が static box 呼び出しを解決できる
- fast gate と selfhost canary の両方が green
