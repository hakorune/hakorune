---
Status: Active (closeout blockers completed; failure-driven backlog only)
Scope: Phase 29bq から「selfhosting closeout」へ進む最短チェックリスト（single-developer）
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/design/selfhost-bootstrap-route-ssot.md
  - docs/development/current/main/design/selfhost-parser-mirbuilder-migration-order-ssot.md
  - docs/development/current/main/design/de-rust-compiler-thin-rust-roadmap-ssot.md
  - docs/development/current/main/phases/phase-29bq/29bq-90-selfhost-checklist.md
  - docs/development/current/main/phases/phase-29bq/29bq-91-mirbuilder-migration-progress-checklist.md
---

# Phase 29bq — Selfhost To-Go Checklist (SSOT)

目的: 「今どこまで終わっていて、selfhosting closeout まで何が残っているか」を 1 枚で固定する。  
原則: failure-driven（新規 freeze/reject が出るまで受理拡張しない）。

## 0) Normalized snapshot (2026-02-11)

- [x] `Current blocker: none`
- [x] selfhost subset `remaining_unpromoted=0`
- [x] `.hako mirbuilder` quick suite green（`--with-stage1` 含む）
- [x] full selfhost canary green（`198/198`, `jobs=4`）
- [x] stage identity full check green（`tools/selfhost_identity_check.sh --mode full --skip-build --bin-stage1 target/selfhost/hakorune.stage1_cli --bin-stage2 target/selfhost/hakorune.stage1_cli.stage2`）
- [x] route parity 3点セット green（stageb/runtime/runtime-mode）

注意:
- ここからの closeout では、機能追加ではなく「証拠の固定」と「再現手順の安定化」を優先する。
- parser/mirbuilder の受理拡張は、failure-driven 条件を満たした時だけ行う。

## 1) Blocking tasks for selfhost closeout

### G1. Stage identity evidence refresh (must)

- [x] Stage1/Stage2 CLI binary を準備する（OOM 環境は prebuilt 利用）
  - `target/selfhost/hakorune.stage1_cli`
  - `target/selfhost/hakorune.stage1_cli.stage2`
- [x] de-rust boundary inventory（2026-02-11 snapshot, stage1-first）を確認する
  - `docs/development/current/main/design/de-rust-compiler-thin-rust-roadmap-ssot.md` の `Current boundary inventory` 節
- [x] identity full を再検証する
  - `tools/selfhost_identity_check.sh --mode full --skip-build --bin-stage1 target/selfhost/hakorune.stage1_cli --bin-stage2 target/selfhost/hakorune.stage1_cli.stage2`
- [x] PASS 証拠を `29bq-91` と `CURRENT_TASK.md` に同期する

運用注記:
- `--cli-mode` 省略時は stage1-first 既定（`stage1-cli(.stage2)`）を使う。
- `--cli-mode auto` は互換診断専用（`[identity/compat-fallback]` 観測用）で、full-mode 証拠には使わない。

受け入れ基準:
- Stage1/Stage2 の Program/MIR 出力が一致し、`--mode full` が `exit 0`。

### G2. Route parity and daily route contract refresh (must)

- [x] `bash tools/smokes/v2/profiles/integration/selfhost/phase29bq_selfhost_stageb_route_parity_smoke_vm.sh`
- [x] `bash tools/smokes/v2/profiles/integration/selfhost/phase29bq_selfhost_runtime_route_smoke_vm.sh`
- [x] `bash tools/smokes/v2/profiles/integration/selfhost/phase29bq_selfhost_runtime_mode_parity_smoke_vm.sh`
- [x] `SH-GATE-STAGEB` / `SH-RUNTIME-SELFHOST` の route tag 契約が崩れていないことを確認する

受け入れ基準:
- route parity 3点セットが green。

### G3. Milestone stability snapshot refresh (must)

- [x] `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq`
- [x] `RUN_TIMEOUT_SECS=120 SMOKES_ENABLE_SELFHOST=1 HAKO_JOINIR_PLANNER_REQUIRED=1 bash tools/smokes/v2/profiles/integration/selfhost/phase29bq_selfhost_planner_required_dev_gate_vm.sh`
- [x] 最新 PASS 値（cases/total_secs/avg_case_secs/jobs）を `29bq-91` に記録する

受け入れ基準:
- fast gate と full canary の両方が green。

## 2) Non-blocking backlog (post-closeout; keep deferred)

この節は「closeout を止めない」。新規 blocker が出た時だけ着手する。

- [ ] parser capability widening（box/static box decl, method decl, lambda など）
  - [x] PW1: delegated box header `box Child from Parent { ... }` を `FuncScannerBox.scan_all_boxes` で受理（fixture: `apps/tests/phase29bq_selfhost_funcscanner_box_from_min.hako`, pin: `tools/smokes/v2/profiles/integration/selfhost/phase29bq_selfhost_stageb_funcscanner_box_from_min_vm.sh`）
  - [x] PW2: method decl widening（`static`/modern 宣言の境界固定）
    - fixture: `apps/tests/phase29bq_selfhost_funcscanner_method_boundary_from_birth_min.hako`
    - pin: `tools/smokes/v2/profiles/integration/selfhost/phase29bq_selfhost_stageb_funcscanner_method_boundary_min_vm.sh`
  - [x] PW3: lambda literal widening（`fn(...) { ... }`）
    - fixture: `apps/tests/phase29bq_selfhost_funcscanner_lambda_literal_min.hako`
    - pin: `tools/smokes/v2/profiles/integration/selfhost/phase29bq_selfhost_stageb_lambda_literal_pair_min_vm.sh`
- [ ] MIR instruction widening（Call/BoxCall/NewBox/Load/Store など）
  - [x] MIW1: `Return(NewBox)` 最小受理（`args=[]` のみ）
    - fixture: `apps/tests/phase29bq_hako_mirbuilder_phase12_return_newbox_min.hako`
    - pin: `tools/smokes/v2/profiles/integration/joinir/phase29bq_hako_mirbuilder_phase12_return_newbox_min_vm.sh`
  - [x] MIW2: `Return(Call)` 最小受理（`Call name=id args=[]` のみ）
    - fixture: `apps/tests/phase29bq_hako_mirbuilder_phase13_return_call_id0_min.hako`
    - pin: `tools/smokes/v2/profiles/integration/joinir/phase29bq_hako_mirbuilder_phase13_return_call_id0_min_vm.sh`
  - [x] MIW3: `Return(BoxCall)` の最小受理を 1形追加（`Method recv=New(StringBox("abc")) method=length args=[]`）
    - fixture: `apps/tests/phase29bq_hako_mirbuilder_phase14_return_boxcall_stringbox_length_abc_min.hako`
    - pin: `tools/smokes/v2/profiles/integration/joinir/phase29bq_hako_mirbuilder_phase14_return_boxcall_stringbox_length_abc_min_vm.sh`
  - [x] MIW4: `Return(Call)` 1引数形（`Call name=id args=[Int(9)]`）の最小受理を 1形追加（failure-driven）
    - fixture: `apps/tests/phase29bq_hako_mirbuilder_phase15_return_call_id1_int9_min.hako`
    - pin: `tools/smokes/v2/profiles/integration/joinir/phase29bq_hako_mirbuilder_phase15_return_call_id1_int9_min_vm.sh`
  - [x] MIW5: `Return(NewBox)` 1引数形（`new StringBox("abc")`）の最小受理を 1形追加（failure-driven）
    - fixture: `apps/tests/phase29bq_hako_mirbuilder_phase16_return_newbox_stringbox_abc_min.hako`
    - pin: `tools/smokes/v2/profiles/integration/joinir/phase29bq_hako_mirbuilder_phase16_return_newbox_stringbox_abc_min_vm.sh`
  - [x] MIW6: `Return(BoxCall)` 1引数形（`Method recv=New(StringBox("abc")) method=indexOf args=[Str("b")]`）の最小受理を 1形追加（failure-driven）
    - fixture: `apps/tests/phase29bq_hako_mirbuilder_phase17_return_boxcall_stringbox_indexof_b_abc_min.hako`
    - pin: `tools/smokes/v2/profiles/integration/joinir/phase29bq_hako_mirbuilder_phase17_return_boxcall_stringbox_indexof_b_abc_min_vm.sh`
  - [x] MIW7: `Return(Call)` 1引数形（`Call name=id args=[Int(7)]`）の最小受理を 1形追加（failure-driven）
    - fixture: `apps/tests/phase29bq_hako_mirbuilder_phase18_return_call_id1_int7_min.hako`
    - pin: `tools/smokes/v2/profiles/integration/joinir/phase29bq_hako_mirbuilder_phase18_return_call_id1_int7_min_vm.sh`
  - [x] 直近順序（固定）: `MIW7` → quick suite 緑確認 → 次判断
  - [ ] 条件付き: `MIW8-10`（値バリエーション）は実ブロッカー発生時のみ着手
  - [x] 設計先行: Load/Store は実装前に設計SSOT（受理最小形とpin）を先に確定
    - SSOT: `docs/development/current/main/design/hako-mirbuilder-load-store-minimal-contract-ssot.md`
    - 実装順（固定）: `LS0(v0 loader readiness)` -> `LS1(Load minimal)` -> `LS2(Store minimal)`
  - [x] LS0: v0 loader readiness pin を固定（handwritten MIR JSON で `load/store` を直接検証）
    - pin: `tools/smokes/v2/profiles/integration/joinir/phase29bq_hako_mirbuilder_ls0_mir_json_v0_load_store_min_vm.sh`
    - quick suite 同期: `tools/smokes/v2/profiles/integration/joinir/phase29bq_hako_mirbuilder_quick_suite_vm.sh`
  - [x] LS1: Load minimal pin を固定（`Local(Int)>Local(Var)>Return(Var)`）
    - fixture: `apps/tests/phase29bq_hako_mirbuilder_phase19_load_local_var_min.hako`
    - pin: `tools/smokes/v2/profiles/integration/joinir/phase29bq_hako_mirbuilder_phase19_load_local_var_min_vm.sh`
  - [x] LS2: Store minimal pin を固定（`Local(Int)>Assignment(Int)>Return(Var)`）
    - fixture: `apps/tests/phase29bq_hako_mirbuilder_phase20_store_assignment_int_min.hako`
    - pin: `tools/smokes/v2/profiles/integration/joinir/phase29bq_hako_mirbuilder_phase20_store_assignment_int_min_vm.sh`
- [ ] 深い nested control widening（break/continue 複合形、複雑 cleanup）

運用ルール:
- BoxCount は 1形=1fixture=1commit。
- BoxShape と混ぜない。

## 3) Guardrails（always-on; completion task ではない）

- AST rewrite 禁止（analysis-only 観測）。
- fallback で通さない（strict/dev + planner_required で fail-fast）。
- 新規 fixture は `no-try/no-throw` 方針を維持（postfix `cleanup` / DropScope 優先）。

## 4) Done definition (selfhost closeout)

- [x] G1, G2, G3 がすべて完了
- [x] `Current blocker: none` を維持
- [x] closeout 日付つき snapshot を `CURRENT_TASK.md` と `29bq-91` に反映
