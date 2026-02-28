---
Status: Active (tracking SSOT)
Scope: `.hako` mirbuilder migration の進捗を “fixture + gate” で計測するチェックリスト（主観の%は書かない）。
Related:
  - docs/development/current/main/phases/phase-29bq/29bq-90-selfhost-checklist.md
  - docs/development/current/main/design/hako-mirbuilder-migration-phase0-entry-contract-ssot.md
  - docs/development/current/main/design/selfhost-parser-mirbuilder-migration-order-ssot.md
  - docs/development/current/main/design/de-rust-compiler-thin-rust-roadmap-ssot.md
  - docs/development/current/main/phases/phase-29bq/29bq-109-hako-mirbuilder-handler-extraction-backlog.md
  - docs/development/current/main/phases/phase-29bq/29bq-113-hako-recipe-first-migration-lane.md
  - docs/development/current/main/phases/phase-29bq/29bq-115-selfhost-to-go-checklist.md
  - docs/development/current/main/design/selfhost-language-v1-freeze-ssot.md
  - tools/smokes/v2/profiles/integration/selfhost/planner_required_selfhost_subset.tsv
---

# Phase 29bq — MirBuilder Migration Progress Checklist (SSOT)

目的: “どこまで移植できたか” を **実データ（fixture + gate）**で追跡し、曖昧な進捗表現を避ける。

## 0) ルール（運用SSOT）

- 進捗は **gate の PASS**でのみ “✅” にする（「だいたいできた」は書かない）。
- 1項目を追加するなら、同時に **証拠（fixture + gate コマンド）**を必ず書く。
- 仕様の話は設計SSOTへ（このファイルは “進捗の台帳” に限定）。

## 1) 見方（最短）

- 毎日の運用は `29bq-90-selfhost-checklist.md`（コマンドSSOT）に従う。
- このファイルは「移植対象ごとのカバレッジ台帳」。

## 2) Route readiness（最低限）

### 2.1 Release build

- [x] `cargo build --release --bin hakorune` PASS

### 2.2 Daily gates（証拠）

- [x] JoinIR fast gate: `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq` PASS
- [x] Selfhost canary: `RUN_TIMEOUT_SECS=120 SMOKES_ENABLE_SELFHOST=1 HAKO_JOINIR_PLANNER_REQUIRED=1 bash tools/smokes/v2/profiles/integration/selfhost/phase29bq_selfhost_planner_required_dev_gate_vm.sh` PASS

### 2.3 Latest verification snapshot（2026-02-25）

- [x] de-rust boundary inventory（2026-02-11 snapshot, stage1-first）を確認済み
  - `docs/development/current/main/design/de-rust-compiler-thin-rust-roadmap-ssot.md` の `Current boundary inventory` 節
- [x] `tools/selfhost_identity_check.sh --mode full --skip-build --bin-stage1 target/selfhost/hakorune.stage1_cli --bin-stage2 target/selfhost/hakorune.stage1_cli.stage2` PASS
- [x] `./tools/selfhost/run.sh --gate --planner-required 1 --max-cases 5 --jobs 4` PASS（`5/5`, `stageb_total_secs=18`, `avg_case_secs=3.60`）
- [x] `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq` PASS
- [x] `RUN_TIMEOUT_SECS=120 SMOKES_ENABLE_SELFHOST=1 HAKO_JOINIR_PLANNER_REQUIRED=1 bash tools/smokes/v2/profiles/integration/selfhost/phase29bq_selfhost_planner_required_dev_gate_vm.sh` PASS (`198/198`, `total_secs=649`, `avg_case_secs=3.28`, `jobs=4`)
- [x] `bash tools/smokes/v2/profiles/integration/selfhost/phase29bq_selfhost_stageb_route_parity_smoke_vm.sh` PASS
- [x] `bash tools/smokes/v2/profiles/integration/selfhost/phase29bq_selfhost_runtime_route_smoke_vm.sh` PASS
- [x] `bash tools/smokes/v2/profiles/integration/selfhost/phase29bq_selfhost_runtime_mode_parity_smoke_vm.sh` PASS
- [x] quick probe: `SMOKES_ENABLE_SELFHOST=1 HAKO_JOINIR_PLANNER_REQUIRED=1 SMOKES_SELFHOST_FILTER=scan_methods_loop_min SMOKES_SELFHOST_MAX_CASES=1 bash tools/smokes/v2/profiles/integration/selfhost/phase29bq_selfhost_planner_required_dev_gate_vm.sh` PASS
- [x] quick probe: `SMOKES_ENABLE_SELFHOST=1 HAKO_JOINIR_PLANNER_REQUIRED=1 SMOKES_SELFHOST_FILTER=using_module_roots bash tools/smokes/v2/profiles/integration/selfhost/phase29bq_selfhost_planner_required_dev_gate_vm.sh` PASS (`3/3`)
- [x] quick probe: `SMOKES_ENABLE_SELFHOST=1 HAKO_JOINIR_PLANNER_REQUIRED=1 SMOKES_SELFHOST_FILTER=side_effect_tail_nested_join_tail bash tools/smokes/v2/profiles/integration/selfhost/phase29bq_selfhost_planner_required_dev_gate_vm.sh` PASS (`4/4`)
- [x] quick probe: `SMOKES_ENABLE_SELFHOST=1 HAKO_JOINIR_PLANNER_REQUIRED=1 SMOKES_SELFHOST_FILTER=try_loop_throw_catch_min SMOKES_SELFHOST_MAX_CASES=1 bash tools/smokes/v2/profiles/integration/selfhost/phase29bq_selfhost_planner_required_dev_gate_vm.sh` PASS (`Expected 15`)
- [x] quick probe: `SMOKES_ENABLE_SELFHOST=1 HAKO_JOINIR_PLANNER_REQUIRED=1 SMOKES_SELFHOST_FILTER=cleanup_only_min SMOKES_SELFHOST_MAX_CASES=1 bash tools/smokes/v2/profiles/integration/selfhost/phase29bq_selfhost_planner_required_dev_gate_vm.sh` PASS (`Expected 11`)
- [x] quick probe: `SMOKES_ENABLE_SELFHOST=1 HAKO_JOINIR_PLANNER_REQUIRED=1 SMOKES_SELFHOST_FILTER=parse_string2_return_prelude_call_min SMOKES_SELFHOST_MAX_CASES=1 bash tools/smokes/v2/profiles/integration/selfhost/phase29bq_selfhost_planner_required_dev_gate_vm.sh` PASS (`Expected MISS`)
- [x] closeout blockers status is synced in `29bq-115-selfhost-to-go-checklist.md`（G1/G2/G3 complete）

### 2.3.1 Quick evidence refresh（2026-02-28, Stage2 fixed）

- [x] `cargo check --bin hakorune` PASS
- [x] `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq` PASS
- [x] `./tools/selfhost/run.sh --gate --planner-required 1 --max-cases 5 --jobs 4` PASS（`5/5`, `stageb_total_secs=19`, `avg_case_secs=3.80`）
- [x] `bash tools/selfhost_identity_check.sh --mode smoke --skip-build` PASS（Program/MIR MATCH）
- [x] `bash tools/selfhost_identity_check.sh --mode full --skip-build --bin-stage1 target/selfhost/hakorune.stage1_cli --bin-stage2 target/selfhost/hakorune.stage1_cli.stage2` PASS（Program/MIR MATCH）

### 2.4 Post-migration default checks（.hako MirBuilder移植後）

この節は「移植した .hako 実装を Rust compiler 側で常に検証する」ための定型チェック。

- [x] quick internal-only（毎コミット）: `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_hako_mirbuilder_quick_suite_vm.sh`
- [x] milestone internal-only（PROMOTE前）: `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_hako_mirbuilder_quick_suite_vm.sh --with-stage1`
- [x] quick + `phase29bq_fast_gate_vm.sh --only bq` が green のときだけ、移植進捗のチェックを更新する。
- [x] mirbuilder hostbridge deny check: `bash tools/checks/hako_mirbuilder_no_hostbridge.sh`
- [x] Program JSON contract pin: `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_hako_program_json_contract_pin_vm.sh`（Print / Expr(Call) / If / Loop）
- [x] Loop pin: `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_hako_mirbuilder_phase11_min_vm.sh`
- [x] MIR instruction pin (`Return(NewBox)` minimal): `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_hako_mirbuilder_phase12_return_newbox_min_vm.sh`
- [x] MIR instruction pin (`Return(Call id())` minimal): `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_hako_mirbuilder_phase13_return_call_id0_min_vm.sh`
- [x] MIR instruction pin (`Return(BoxCall StringBox("abc").length())` minimal): `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_hako_mirbuilder_phase14_return_boxcall_stringbox_length_abc_min_vm.sh`
- [x] MIR instruction pin (`Return(Call id(9))` one-arg minimal): `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_hako_mirbuilder_phase15_return_call_id1_int9_min_vm.sh`
- [x] MIR instruction pin (`Return(New StringBox("abc"))` one-arg minimal): `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_hako_mirbuilder_phase16_return_newbox_stringbox_abc_min_vm.sh`
- [x] MIR instruction pin (`Return(BoxCall StringBox("abc").indexOf("b"))` one-arg minimal): `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_hako_mirbuilder_phase17_return_boxcall_stringbox_indexof_b_abc_min_vm.sh`
- [x] MIR instruction pin (`Return(Call id(7))` one-arg minimal): `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_hako_mirbuilder_phase18_return_call_id1_int7_min_vm.sh`
- [x] Load/Store lane (docs-first) SSOT fixed: `docs/development/current/main/design/hako-mirbuilder-load-store-minimal-contract-ssot.md`
- [x] LS0 pin (`mir_json_v0` loader readiness for `load/store`): `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_hako_mirbuilder_ls0_mir_json_v0_load_store_min_vm.sh`
- [x] MIR instruction pin (`Load` minimal): `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_hako_mirbuilder_phase19_load_local_var_min_vm.sh`
- [x] MIR instruction pin (`Store` minimal): `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_hako_mirbuilder_phase20_store_assignment_int_min_vm.sh`

### 2.5 Current extraction lane（post parser Tier-18）

- [x] Tier-18 完了後に `29bq-109` の M0（Print handler extraction）へ着手
- [x] M1（Local handler extraction）完了
- [x] M2（Assignment handler extraction）完了
- [x] M3（Return handler extraction）完了
- [x] M4（inline residue cleanup + docs sync）完了

### 2.6 Post-M4 lane（ordered; `29bq-111`）

- [x] P1（Print logic ownership move）完了
- [x] P2（Local logic ownership move）完了
- [x] P3（Assignment logic ownership move）完了
- [x] P4（Return logic ownership move）完了
- [x] P5（consumer residue cleanup + docs sync）完了

### 2.7 Recipe-first lane（ordered; `29bq-113`）

- [x] R0（Recipe core vocabulary）完了
- [x] R1（Facts extraction for stmt-4）完了
- [x] R2（Verifier always-on）完了
- [x] R3（Lower wiring by Recipe）完了
- [x] R4（If integration / M5）完了
- [x] R5（Loop integration / M6）完了
- [x] R6（residue cleanup + docs sync）完了

### 2.8 Cleanup integration prep（C2 boundary pin; docs-only）

- [x] C2 dispatch boundary fixed:
  - `ProgramJsonV0PhaseStateConsumerBox.consume_stmt(...)` / `_dispatch_or_unsupported(...)` を cleanup 受け口の単一入口として固定
- [x] C2 responsibility split fixed:
  - handlers = routing/recipe only
  - facts/verifier = acceptance + invariant checks
  - builder = verified recipe lowering only
- [x] C2 pointer sync:
  - lane SSOT: `29bq-114-hako-cleanup-integration-prep-lane.md`
  - code-side note: `lang/src/compiler/mirbuilder/README.md` の "Cleanup integration boundary (C2 prep)"
- [x] M7-min-1 contract fixed:
  - Program(JSON v0) `StmtV0::Try` の non-loop 最小形のみ `.hako` mirbuilder で受理
  - pin: `tools/smokes/v2/profiles/integration/joinir/phase29bq_hako_mirbuilder_cleanup_try_min_vm.sh`
  - non-minimal 形は fail-fast（`[freeze:contract][hako_mirbuilder][cap_missing/stmt:Try]`）
- [x] M7-min-2 reject boundary fixed:
  - non-minimal `Try(cleanup)`（multi stmt / catches 非空 / loop+cleanup）を reject pin 化
  - pin: `tools/smokes/v2/profiles/integration/joinir/phase29bq_hako_mirbuilder_cleanup_try_reject_nonminimal_vm.sh`
  - loop+cleanup は `[cap_missing/stmt:Loop]`、その他は `[cap_missing/stmt:Try]` で fail-fast
- [x] M7-min-3 accept boundary widened:
  - non-loop cleanup で `finally=[Local 1stmt]` を 1形だけ受理
  - pin: `tools/smokes/v2/profiles/integration/joinir/phase29bq_hako_mirbuilder_cleanup_try_finally_local_min_vm.sh`
  - contract: `Try` body と `finally Local` は同一変数更新のみ許可
- [x] M7-min-4 reject boundary fixed:
  - `Try` body と `finally Local` の更新変数不一致を reject pin 化
  - pin: `tools/smokes/v2/profiles/integration/joinir/phase29bq_hako_mirbuilder_cleanup_try_finally_local_var_mismatch_reject_vm.sh`
  - contract: mismatch は `[cap_missing/stmt:Try] Try finally Local must update the same var as Try body` で fail-fast

### 2.9 Cleanup integration prep（C3 go/no-go; docs-only）

- [x] C3 go/no-go fixed:
  - C0-C2 fixed + accept/reject pin reproducible + quick verify green を実装開始条件に固定
- [x] C3 implementation unit fixed:
  - 1 commit = 1 acceptance shape + fixture/pin + quick gate
- [x] C3 failure-driven compatibility fixed:
  - full canary は節目のみ、BoxShape/BoxCount 混在禁止を再確認
- [x] M7-min-1 implemented:
  - commit: `192173aca`
  - target fixture: `apps/tests/phase29bq_selfhost_cleanup_only_min.hako`
- [x] M7-min-2 implemented:
  - commit: this change-set（M7-min-2）
  - target pin: `tools/smokes/v2/profiles/integration/joinir/phase29bq_hako_mirbuilder_cleanup_try_reject_nonminimal_vm.sh`
- [x] M7-min-3 implemented:
  - commit: this change-set（M7-min-3）
  - target pin: `tools/smokes/v2/profiles/integration/joinir/phase29bq_hako_mirbuilder_cleanup_try_finally_local_min_vm.sh`
- [x] M7-min-4 implemented:
  - commit: this change-set（M7-min-4）
  - target pin: `tools/smokes/v2/profiles/integration/joinir/phase29bq_hako_mirbuilder_cleanup_try_finally_local_var_mismatch_reject_vm.sh`
- [x] next operation fixed:
  - M7 lane は failure-driven（新規 freeze/reject か契約変更が出るまで追加拡張なし）

## 3) Stage‑B JSON v0（AST）coverage

この節の “✅” は「Stage‑B が該当ノードを JSON v0 に出し、それを selfhost gate で VM 実行まで通した」ことを意味する。

- [x] `BlockExpr`（tail が expr JSON）:
  - Evidence: `apps/tests/phase29bq_blockexpr_basic_min.hako`（fast gate bq）
- [x] `BlockExpr`（tail が stmt JSON: `If` など）:
  - Evidence: `apps/tests/phase29bq_selfhost_blocker_parse_try_program_block_min.hako`（selfhost subset）
- [x] `Try` + `Throw`（stmt）+ `catch` + `cleanup`（Result-mode）:
  - Evidence: `apps/tests/phase29bq_selfhost_try_throw_catch_cleanup_min.hako`（selfhost subset）

## 4) JSON v0 bridge lowering coverage

この節の “✅” は「bridge が該当ノードを MIR に下ろし、VM 実行まで通した」ことを意味する。

- [x] `StmtV0::Try`（Result-mode / no MIR Catch/Throw）:
  - Evidence: `apps/tests/phase29bq_selfhost_try_throw_catch_cleanup_min.hako`
- [x] `StmtV0::Throw`（Result-mode の throw_ctx 経由）:
  - Evidence: `apps/tests/phase29bq_selfhost_try_throw_catch_cleanup_min.hako`

### 4.1 Known gap（次のBoxCount候補）

- [x] throw が loop の中で起きたとき、catch 側で周辺変数が throw 時点の値で見えること（var map snapshot）:
  - Evidence fixture: `apps/tests/phase29bq_selfhost_try_loop_throw_catch_min.hako`（selfhost subset PROMOTE）
  - Fix area: `src/runner/json_v0_bridge/lowering/{throw_ctx.rs,throw_lower.rs,try_catch.rs}`
