# Self Current Task — Now (main)

## Current Focus

- Phase: Phase 29bq（planner-required → selfhost entry wiring）
- Next: CorePlan cleanup after “Loop as structural box”（優先: CleanupWrap / Select / `loop(true|cond)` の小箱で混線を減らす）
- New gates: `phase29bq_loop_true_multi_break_planner_required_vm.sh` / `phase29bq_conditional_update_join_planner_required_vm.sh` は green（strict/dev + planner_required）
- Evidence (fast): `phase29bs_fast_gate_vm.sh --full` PASS（LOG: `/tmp/phase29bs_fast_gate_1754928_29bp.log`）
- Phase 29br closeout ✅
- JoinIR gate is green. Compare adopt default OFF is recorded in `docs/development/current/main/20-Decisions.md`.
- P2 blocker resolved: `phase29aq_string_parse_integer_sign_min_vm` flake（`-42` → `0`）は function lowering の type_ctx リーク（ValueId 衝突）を修正して解消。
- Trace: `NYASH_DEV_PROVIDER_TRACE=1`（provider/box/method 選択の候補・採用ログ。必要時のみ）。
- Selfhost gate blocker（opt-in / canary）: stage-b compile が `ParserStringScanBox.scan_with_quote/3` の `loop(cond)` で planner-required freeze（`generic loop v0.2: control flow after in-body step`）→ compiler 側の LoopFrame/step-in-body 語彙で受理範囲を拡張して潰す（`.hako` 側回避はしない）。
- Progress: generic loop v0 の condition から loop_var を “左辺Variable限定” ではなく候補列挙→step一致で一意決定する方式に拡張（`src/mir/builder/control_flow/plan/generic_loop/canon.rs`）。
- Done: UpdateCanon を追加（analysis-only / no rewrite）。loop condition/update の形揺れは raw rewrite せず、Facts/Normalize の観測で受理範囲を増やす（SSOT: `docs/development/current/main/design/coreloop-generic-loop-v0-ssot.md`）。
- Fix: `phase29aq_stdlib_pack_vm.sh` を run.sh 非依存にして timeout 由来の失敗を解消、`phase29ae_regression_pack_vm.sh` は post-change green。
- Planned (design): 代入-only if を `.hako` のダミー leaf effect で回避しないため、CorePlan 側に Select/IfSelect 正規化（条件付き更新のデータフロー表現）を追加する（SSOT: `docs/development/current/main/design/coreloop-generic-loop-v0-ssot.md`）。

## Gate (SSOT)

- Loopless subset gate (P2): `./tools/hako_check_loopless_gate.sh` (SSOT: `docs/development/current/main/phases/phase-29bg/P2-MAKE-HAKO_CHECK-LOOPLESS-INSTRUCTIONS.md`)
- P3 default entry: `./tools/hako_check_loopless_gate.sh`
- Integration gate: `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh`
- Phase 29bs fast iteration gate: `./tools/smokes/v2/profiles/integration/joinir/phase29bs_fast_gate_vm.sh`
- Phase 29bq fast iteration gate: `./tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh`
- Planner-required gates SSOT: `docs/development/current/main/design/joinir-planner-required-gates-ssot.md`
- Planner-first cases gate: `./tools/smokes/v2/profiles/integration/joinir/phase29bh_planner_first_cases_vm.sh` (list: `tools/smokes/v2/profiles/integration/joinir/planner_first_cases.tsv`)
- Pattern2 required pack: `./tools/smokes/v2/profiles/integration/joinir/phase29bi_planner_required_pattern2_pack_vm.sh`
- Pattern6/7 required pack: `./tools/smokes/v2/profiles/integration/joinir/phase29bj_planner_required_pattern6_7_pack_vm.sh`
- Pattern1/4/5 required pack: `./tools/smokes/v2/profiles/integration/joinir/phase29bl_planner_required_pattern1_4_5_pack_vm.sh`
- Dev default gate: `./tools/smokes/v2/profiles/integration/joinir/phase29bp_planner_required_dev_gate_v4_vm.sh`
- Phase 29bq loop(true) gate: `./tools/smokes/v2/profiles/integration/joinir/phase29bq_loop_true_multi_break_planner_required_vm.sh`
- Phase 29bq conditional update/join gate: `./tools/smokes/v2/profiles/integration/joinir/phase29bq_conditional_update_join_planner_required_vm.sh`
- Selfhost entry gate (opt-in): `SMOKES_ENABLE_SELFHOST=1 ./tools/smokes/v2/profiles/integration/selfhost/phase29bq_selfhost_planner_required_dev_gate_vm.sh`
- Planner-required master list: `tools/smokes/v2/profiles/integration/joinir/planner_required_cases.tsv`
- Quick smoke: `./tools/smokes/v2/run.sh --profile quick`
- P0 gate: build/deadcode/deadblocks/run_tests/json-lsp OK
- Status: loopless gate / phase29bp dev gate v4 / phase29ae regression pack は緑（parse_integer_sign は 500/500 安定）

## Ops Rules

- integration filter で `phase143_*` は回さない（JoinIR 回帰は gate のみ）
- `phase286_pattern9_*` は legacy pack (SKIP) のみで扱う

## Pointers

- CorePlan 移行道筋 SSOT: `docs/development/current/main/design/coreplan-migration-roadmap-ssot.md`
- 旧ログ（肥大化防止のためアーカイブ）: `docs/development/current/main/phases/phase-29ao/10-Now-archive.md`
