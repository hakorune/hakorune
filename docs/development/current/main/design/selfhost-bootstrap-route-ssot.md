# Selfhost Bootstrap Route (SSOT)

Status: SSOT  
Scope: selfhost を「.hako コンパイラ → JoinIR → JSON v0 → VM」まで戻す最小経路の契約。

Related:
- CURRENT_TASK.md
- docs/development/current/main/design/selfhost-compiler-structure-ssot.md
- docs/development/current/main/design/selfhost-bootstrap-route-evidence-and-legacy-lanes.md
- docs/development/current/main/10-Now.md
- docs/development/current/main/phases/phase-29bq/README.md
- docs/development/current/main/design/compiler-expressivity-first-policy.md
- docs/development/current/main/design/ai-handoff-and-debug-contract.md
- docs/development/current/main/design/selfhost-language-v1-freeze-ssot.md
- docs/development/current/main/design/hako-mirbuilder-migration-phase0-entry-contract-ssot.md
- docs/development/current/main/design/selfhost-parser-mirbuilder-migration-order-ssot.md
- docs/development/current/main/phases/phase-29cf/README.md
- docs/development/current/main/phases/phase-29ch/README.md
- docs/development/current/main/phases/phase-29ch/29ch-20-route-evidence-and-probes.md
- docs/development/current/main/design/concurrency-async-pre-selfhost-ssot.md
- docs/development/current/main/design/pyvm-retreat-ssot.md

## Goal

selfhost を目的化せず、compiler-first の方針を守りつつ、
「.hako コンパイラが自己コンパイル可能な状態」へ戻すための最小経路を定義する。

End-state note:
- bootstrap route の最終目標は `.hako` compiler mainline が compiler meaning を持ち、
  plugin behavior も `.hako` 側へ寄り、Rust は host/runtime/backend の最小面だけを残すこと
- `Program(JSON v0)` / stage1 wrapper / surrogate provider はその途中にある bootstrap-only boundary で、authority ではなく最終 retire target だと扱う

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
- reduced `stage1-cli` authority の current accepted truth は次で固定する:
  - authority / compat boundaries / route ids:
    - `docs/development/current/main/design/selfhost-bootstrap-route-ssot.md`
  - `.hako` / Rust ownership map:
    - `docs/development/current/main/design/selfhost-compiler-structure-ssot.md`
  - active phase truth:
    - `docs/development/current/main/phases/phase-29ch/README.md`
  - detailed evidence / diagnostics probes:
    - `docs/development/current/main/phases/phase-29ch/29ch-20-route-evidence-and-probes.md`
  - legacy lane / binary-only / blocker capture supplement:
    - `docs/development/current/main/design/selfhost-bootstrap-route-evidence-and-legacy-lanes.md`
- `tools/selfhost_identity_check.sh` は reduced case として artifact-kind=`stage1-cli` smoke lane で `NYASH_BIN=<stage1-cli>` bridge-first bootstrap を使う。raw direct `stage1-cli` replacement ではなく、helper-driven Stage1 bridge contract を Stage2 build に昇格した narrow reduction として扱う
- exact probe では `target/selfhost/hakorune.stage1_cli` は raw direct contract (`emit ...` / `--emit-mir-json`) で `97` を返す一方、`stage1_contract_exec_mode` は current reduced artifact に single-step source→MIR env contract を提供する。`tools/selfhost/run_stage1_cli.sh` は raw `emit program-json` / `emit mir-json` surface をこの env contract に変換する compatibility wrapper であり、新しい authority route ではない
- `build_stage1.sh` の `stage1-cli bridge-first` bootstrap path は current reduced source (`lang/src/runner/stage1_cli_env.hako`) を single-step source→MIR へ通し、`tools/ny_mir_builder.sh` には MIR(JSON) だけを渡す
- `stage1_cli_env.hako::Stage1InputContractBox` isolates the shared env/source resolution contract so authority/compat boxes do not need to duplicate input shaping
- `stage1_cli_env.hako::Stage1ProgramAuthorityBox` isolates the current emit-program authority so `Main` can stay a thin dispatcher while defs synthesis/materialization remain same-file
- materialized Program(JSON) validation is isolated in `stage1_cli_env.hako::Stage1ProgramResultValidationBox`, keeping emit-program on the same thin-dispatch pattern
- source-only authority case では `stage1_cli_env.hako::Stage1SourceMirAuthorityBox` が `MirBuilderBox.emit_from_source_v0(...)` を担当し、explicit supplied Program(JSON) text-only input がある時だけ `MirBuilderBox.emit_from_program_json_v0(...)` を explicit compatibility input shape として残す
- materialized MIR(JSON) validation / debug surface is isolated in `stage1_cli_env.hako::Stage1MirResultValidationBox`, keeping `Main` as a thin dispatcher
- that explicit compatibility gate/call is now quarantined in `Stage1ProgramJsonCompatBox` inside `lang/src/runner/stage1_cli_env.hako`
- explicit compatibility input shape は exact-only `emit-mir-program` mode でのみ許可し、live text transport は `STAGE1_SOURCE_TEXT` を再利用する
- plain `emit-mir` は mixed-in `STAGE1_PROGRAM_JSON_TEXT` を fail-fast する
- legacy alias forms for that explicit mode are not part of the active contract
- raw `stage1-cli` artifact の helper execution lane はまだ `rc=97` のため、explicit Program(JSON) compat dispatch は Stage1-side keep のままにする
- `phase-29ch` compare policy is split on purpose: authority and route identity stay exact, but G1 MIR comparison may start with a narrow semantic canonical compare while raw MIR exact diff remains tightening evidence
- exact env-mainline route ids and their fail-fast hints are centralized in `tools/selfhost/lib/identity_routes.sh`
- G1 full-mode route guard も current authority に同期しており、`tools/selfhost_identity_check.sh --mode full` は `program-json=stage1-env-program` と `mir-json=stage1-env-mir-source` の exact route 以外では pass しない
- supplied Program(JSON) compat text transport の shell-side SSOT は `tools/selfhost/lib/stage1_contract.sh` -> `tools/selfhost/run_stage1_cli.sh` -> `tools/selfhost/lib/identity_routes.sh` で固定し、live transport は `STAGE1_SOURCE_TEXT` を再利用する
- exact-only compat helper / mode / sentinel entry (`stage1_contract_exec_program_json_compat()` / `emit-mir-program` / `__stage1_program_json__`) も `tools/selfhost/lib/stage1_contract.sh` を単一正本にする
- `STAGE1_PROGRAM_JSON_TEXT` is outside the live shell contract and exists only for fail-fast / diagnostics probes
- retired path transport is not part of the live shell contract anymore; live shell compat is exact-helper only
- `tools/selfhost/build_stage1.sh --artifact-kind stage1-cli` の post-build capability probe も同じ shared helper で `stage1-env-program` + `stage1-env-mir-source` を要求する
- `tools/selfhost/build_stage1.sh` の stage1-cli bridge-first bootstrap 本体も同じ shared helper (`probe_exact_stage1_env_authority`) で MIR(JSON) を materialize する

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

Detailed evidence / monitor-only lane packs:
- `docs/development/current/main/design/selfhost-bootstrap-route-evidence-and-legacy-lanes.md`

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

## Invariants (Fail-Fast)

- AST rewrite 禁止（CondCanon/UpdateCanon などの analysis-only view を使う）
- fallback 禁止（[freeze:contract] で明示的に落とす）
- Facts は Recipe を返し、Lower は Recipe のみを見る

## Return-to-selfhost checklist (minimal)

- Stage-B が Program.defs を含む JSON v0 を出力できる
- planner_required のタグが欠落しない
- JSON v0 bridge が static box 呼び出しを解決できる
- fast gate と selfhost canary の両方が green
