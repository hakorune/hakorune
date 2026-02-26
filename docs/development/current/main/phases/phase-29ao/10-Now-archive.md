# Self Current Task — Now (archive)

## Current Focus: Phase 29ao（CorePlan composition）

Next: Phase 29ao P37（TBD）
指示書: TBD
運用ルール: integration filter で phase143_* は回さない（JoinIR 回帰は phase29ae pack のみ）
運用ルール: phase286_pattern9_* は legacy pack (SKIP) を使う
移行道筋 SSOT: `docs/development/current/main/design/coreplan-migration-roadmap-ssot.md`

**2025-12-30: Phase 29ao P36 完了** ✅
- 目的: Pattern1 subset を release 既定でも Facts→CorePlan(skeleton) で採用する Stage-2 パイロットを開始（仕様不変）
- 変更: `src/mir/builder/control_flow/plan/composer/shadow_adopt.rs` / `src/mir/builder/control_flow/plan/composer/mod.rs` / `src/mir/builder/control_flow/joinir/patterns/router.rs` / `docs/development/current/main/phases/phase-29ao/README.md` / `docs/development/current/main/10-Now.md` / `docs/development/current/main/30-Backlog.md` / `docs/development/current/main/design/coreplan-migration-roadmap-ssot.md`
- 検証: `cargo build --release` / `./tools/smokes/v2/run.sh --profile quick` / `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh`

**2025-12-30: Phase 29ao P35 完了** ✅
- 目的: shadow adopt タグの必須/禁止を SSOT 化し、Pattern1 subset reject の negative gate を回帰で固定（仕様不変）
- 変更: `docs/development/current/main/design/coreplan-shadow-adopt-tag-coverage-ssot.md` / `docs/development/current/main/phases/phase-29ao/README.md` / `docs/development/current/main/10-Now.md` / `docs/development/current/main/30-Backlog.md` / `docs/development/current/main/design/coreplan-migration-roadmap-ssot.md`
- 検証: `cargo build --release` / `./tools/smokes/v2/run.sh --profile quick` / `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh`

**2025-12-30: Phase 29ao P34 完了** ✅
- 目的: Pattern2 negative ケース（freeze/notapplicable）で shadow adopt タグが出ないことを回帰で固定（仕様不変）
- 変更: `tools/smokes/v2/profiles/integration/apps/archive/phase29ab_pattern2_seg_notapplicable_min_vm.sh` / `tools/smokes/v2/profiles/integration/apps/archive/phase29ab_pattern2_seg_freeze_min_vm.sh` / `docs/development/current/main/phases/phase-29ao/README.md` / `docs/development/current/main/10-Now.md` / `docs/development/current/main/30-Backlog.md` / `docs/development/current/main/design/coreplan-migration-roadmap-ssot.md`
- 検証: `cargo build --release` / `./tools/smokes/v2/run.sh --profile quick` / `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh`

**2025-12-30: Phase 29ao P33 完了** ✅
- 目的: Pattern2 LoopBodyLocal を planner 由来 Pattern2Break に引き上げ、strict/dev の shadow adopt タグを回帰で固定（仕様不変）
- 変更: `src/mir/builder/control_flow/plan/facts/pattern2_break_facts.rs` / `src/mir/builder/control_flow/plan/normalizer/pattern2_break.rs` / `tools/smokes/v2/profiles/integration/apps/archive/phase29ab_pattern2_loopbodylocal_min_vm.sh` / `tools/smokes/v2/profiles/integration/apps/archive/phase29ab_pattern2_loopbodylocal_seg_min_vm.sh` / `docs/development/current/main/phases/phase-29ao/README.md` / `docs/development/current/main/10-Now.md` / `docs/development/current/main/30-Backlog.md` / `docs/development/current/main/design/coreplan-migration-roadmap-ssot.md`
- 検証: `cargo build --release` / `./tools/smokes/v2/run.sh --profile quick` / `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh`

**2025-12-30: Phase 29ao P32 完了** ✅
- 目的: Pattern2 real-world を planner subset に引き上げ、strict/dev で Facts→CorePlan shadow adopt を踏ませる（仕様不変）
- 変更: `src/mir/builder/control_flow/plan/facts/pattern2_break_facts.rs` / `tools/smokes/v2/profiles/integration/apps/archive/phase263_pattern2_seg_realworld_min_vm.sh` / `docs/development/current/main/phases/phase-29ao/README.md` / `docs/development/current/main/10-Now.md` / `docs/development/current/main/30-Backlog.md` / `CURRENT_TASK.md` / `docs/development/current/main/design/coreplan-migration-roadmap-ssot.md`
- 検証: `cargo build --release` / `./tools/smokes/v2/run.sh --profile quick` / `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh`

**2025-12-30: Phase 29ao P31 完了** ✅
- 目的: shadow adopt の判定/Fail-Fast/タグを composer 側に集約し、router を薄くする（仕様不変）
- 変更: `src/mir/builder/control_flow/plan/composer/shadow_adopt.rs` / `src/mir/builder/control_flow/plan/composer/mod.rs` / `src/mir/builder/control_flow/joinir/patterns/router.rs` / `docs/development/current/main/phases/phase-29ao/README.md` / `docs/development/current/main/10-Now.md` / `docs/development/current/main/30-Backlog.md` / `CURRENT_TASK.md` / `docs/development/current/main/design/coreplan-migration-roadmap-ssot.md`
- 検証: `cargo build --release` / `./tools/smokes/v2/run.sh --profile quick` / `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh`

**2025-12-30: Phase 29ao P30 完了** ✅
- 目的: Facts→CorePlan の入口を `plan/composer` に集約し、Normalizer の責務を DomainPlan→CorePlan に縮退（挙動不変）
- 変更: `src/mir/builder/control_flow/plan/composer/shadow_adopt.rs` / `src/mir/builder/control_flow/plan/composer/mod.rs` / `src/mir/builder/control_flow/joinir/patterns/router.rs` / `src/mir/builder/control_flow/plan/normalizer/pattern2_break.rs` / `src/mir/builder/control_flow/plan/normalizer/pattern3_if_phi.rs` / `src/mir/builder/control_flow/plan/normalizer/pattern5_infinite_early_exit.rs` / `src/mir/builder/control_flow/plan/normalizer/pattern_scan_with_init.rs` / `src/mir/builder/control_flow/plan/normalizer/pattern_split_scan.rs` / `docs/development/current/main/phases/phase-29ao/README.md` / `docs/development/current/main/10-Now.md` / `docs/development/current/main/30-Backlog.md` / `CURRENT_TASK.md` / `docs/development/current/main/design/coreplan-migration-roadmap-ssot.md`
- 検証: `cargo build --release` / `./tools/smokes/v2/run.sh --profile quick` / `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh`

**2025-12-30: Phase 29ao P29 完了** ✅
- 目的: regression gate 全パターンで shadow adopt タグを必須化し、strict/dev 実踏みを SSOT 化（仕様不変）
- 変更: `src/mir/builder/control_flow/joinir/patterns/router.rs` / `tools/smokes/v2/profiles/integration/joinir/phase29ao_pattern1_strict_shadow_vm.sh` / `tools/smokes/v2/profiles/integration/joinir/phase29ao_pattern5_strict_shadow_vm.sh` / `tools/smokes/v2/profiles/integration/apps/archive/phase29ai_pattern2_break_plan_subset_ok_min_vm.sh` / `tools/smokes/v2/profiles/integration/apps/phase118_pattern3_if_sum_vm.sh` / `tools/smokes/v2/lib/test_runner.sh` / `docs/development/current/main/phases/phase-29ae/README.md` / `docs/development/current/main/phases/phase-29ao/README.md` / `docs/development/current/main/10-Now.md` / `docs/development/current/main/30-Backlog.md` / `CURRENT_TASK.md` / `docs/development/current/main/design/coreplan-migration-roadmap-ssot.md`
- 検証: `cargo build --release` / `./tools/smokes/v2/run.sh --profile quick` / `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh`

**2025-12-30: Phase 29ao P28 完了** ✅
- 目的: strict/dev shadow adopt の実踏みを安定タグと回帰スモークで検証可能にする（仕様不変）
- 変更: `src/mir/builder/control_flow/joinir/patterns/router.rs` / `tools/smokes/v2/profiles/integration/joinir/phase29ao_pattern6_strict_shadow_vm.sh` / `tools/smokes/v2/profiles/integration/joinir/phase29ao_pattern7_strict_shadow_vm.sh` / `tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh` / `docs/development/current/main/phases/phase-29ae/README.md` / `docs/development/current/main/phases/phase-29ao/README.md` / `docs/development/current/main/10-Now.md` / `docs/development/current/main/30-Backlog.md` / `CURRENT_TASK.md` / `docs/development/current/main/design/coreplan-migration-roadmap-ssot.md`
- 検証: `cargo build --release` / `./tools/smokes/v2/run.sh --profile quick` / `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh`

**2025-12-30: Phase 29ao P27 完了** ✅
- 目的: Pattern6(ScanWithInit) の subset を strict/dev で Facts→CorePlan に寄せ、planner subset のズレを早期検知
- 変更: `src/mir/builder/control_flow/plan/normalizer/pattern_scan_with_init.rs` / `src/mir/builder/control_flow/joinir/patterns/router.rs` / `docs/development/current/main/phases/phase-29ao/README.md` / `docs/development/current/main/10-Now.md` / `docs/development/current/main/30-Backlog.md` / `CURRENT_TASK.md` / `docs/development/current/main/design/coreplan-migration-roadmap-ssot.md`
- 検証: `cargo build --release` / `./tools/smokes/v2/run.sh --profile quick` / `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh`

**2025-12-30: Phase 29ao P26 完了** ✅
- 目的: Pattern2(Break) の subset を strict/dev で Facts→CorePlan に寄せ、planner subset のズレを早期検知
- 変更: `src/mir/builder/control_flow/plan/normalizer/pattern2_break.rs` / `src/mir/builder/control_flow/joinir/patterns/router.rs` / `tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh` / `docs/development/current/main/phases/phase-29ae/README.md` / `docs/development/current/main/phases/phase-29ao/README.md` / `docs/development/current/main/10-Now.md` / `docs/development/current/main/30-Backlog.md` / `CURRENT_TASK.md` / `docs/development/current/main/design/coreplan-migration-roadmap-ssot.md`
- 検証: `cargo build --release` / `./tools/smokes/v2/run.sh --profile quick` / `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh`

**2025-12-30: Phase 29ao P25 完了** ✅
- 目的: Pattern5 を strict/dev で Facts→CorePlan に寄せ、DomainPlan 経路との差分を早期検知
- 変更: `src/mir/builder/control_flow/plan/normalizer/pattern5_infinite_early_exit.rs` / `src/mir/builder/control_flow/joinir/patterns/router.rs` / `tools/smokes/v2/profiles/integration/joinir/phase29ao_pattern5_strict_shadow_vm.sh` / `tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh` / `docs/development/current/main/phases/phase-29ae/README.md` / `docs/development/current/main/phases/phase-29ao/README.md` / `docs/development/current/main/10-Now.md` / `docs/development/current/main/30-Backlog.md` / `CURRENT_TASK.md` / `docs/development/current/main/design/coreplan-migration-roadmap-ssot.md`
- 検証: `cargo build --release` / `./tools/smokes/v2/run.sh --profile quick` / `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh`

**2025-12-30: Phase 29ao P24 完了** ✅
- 目的: Pattern7 を strict/dev で Facts→CorePlan に寄せ、DomainPlan とのズレを早期検知
- 変更: `src/mir/builder/control_flow/plan/normalizer/pattern_split_scan.rs` / `src/mir/builder/control_flow/joinir/patterns/router.rs` / `docs/development/current/main/phases/phase-29ao/README.md` / `docs/development/current/main/10-Now.md` / `docs/development/current/main/30-Backlog.md` / `CURRENT_TASK.md`
- 検証: `cargo build --release` / `./tools/smokes/v2/run.sh --profile quick` / `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh`

**2025-12-30: Phase 29ao P23 完了** ✅
- 目的: Pattern3 を strict/dev で Facts→CorePlan に寄せ、DomainPlan とのズレを早期検知
- 変更: `src/mir/builder/control_flow/plan/normalizer/pattern3_if_phi.rs` / `src/mir/builder/control_flow/joinir/patterns/router.rs` / `docs/development/current/main/phases/phase-29ao/README.md` / `docs/development/current/main/10-Now.md` / `docs/development/current/main/30-Backlog.md` / `CURRENT_TASK.md`
- 検証: `cargo build --release` / `./tools/smokes/v2/run.sh --profile quick` / `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh`

**2025-12-30: Phase 29ao P22 完了** ✅
- 目的: Pattern1 の CoreLoop 構築を SSOT 化し、DomainPlan/Facts 経路の二重実装を排除
- 変更: `src/mir/builder/control_flow/plan/normalizer/pattern1_coreloop_builder.rs` / `src/mir/builder/control_flow/plan/normalizer/pattern1_simple_while.rs` / `src/mir/builder/control_flow/plan/normalizer/skeleton_loop.rs` / `docs/development/current/main/phases/phase-29ao/README.md` / `docs/development/current/main/10-Now.md` / `docs/development/current/main/30-Backlog.md` / `CURRENT_TASK.md`
- 検証: `cargo build --release` / `./tools/smokes/v2/run.sh --profile quick` / `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh`

**2025-12-30: Phase 29ao P21 完了** ✅
- 目的: Pattern1 subset を body=step のみに引き締め、strict/dev shadow adopt の誤マッチを遮断
- 変更: `src/mir/builder/control_flow/plan/policies/pattern1_subset_policy.rs` / `src/mir/builder/control_flow/plan/facts/pattern1_simplewhile_facts.rs` / `src/mir/builder/control_flow/plan/extractors/pattern1.rs` / `apps/tests/phase29ao_pattern1_subset_reject_extra_stmt.hako` / `tools/smokes/v2/profiles/integration/joinir/phase29ao_pattern1_subset_reject_extra_stmt_vm.sh` / `tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh` / `docs/development/current/main/phases/phase-29ae/README.md` / `docs/development/current/main/phases/phase-29ao/README.md` / `docs/development/current/main/10-Now.md` / `docs/development/current/main/30-Backlog.md` / `CURRENT_TASK.md`
- 検証: `cargo build --release` / `./tools/smokes/v2/run.sh --profile quick` / `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh`

**2025-12-30: Phase 29ao P20 完了** ✅
- 目的: CoreLoop の ExitMap/Cleanup/ValueJoin 合成規約を SSOT 化（docs-only）
- 変更: `docs/development/current/main/design/coreloop-exitmap-composition-ssot.md` / `docs/development/current/main/design/planfrag-ssot-registry.md` / `docs/development/current/main/phases/phase-29ao/README.md` / `docs/development/current/main/10-Now.md` / `docs/development/current/main/30-Backlog.md` / `CURRENT_TASK.md`

**2025-12-30: Phase 29ao P19 完了** ✅
- 目的: regression gate に Pattern1 strict/dev shadow adopt を含め、回帰で必ず踏む（SSOT化）
- 変更: `tools/smokes/v2/profiles/integration/joinir/phase29ao_pattern1_strict_shadow_vm.sh` / `tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh` / `docs/development/current/main/phases/phase-29ae/README.md`
- 検証: `./tools/smokes/v2/run.sh --profile quick` / `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh`

**2025-12-30: Phase 29ao P0 完了** ✅
- 目的: `CanonicalLoopFacts → CorePlan` 合成の入口（未接続）を 1 箇所に作る
- 変更: `src/mir/builder/control_flow/plan/composer/mod.rs` / `src/mir/builder/control_flow/plan/mod.rs` / `docs/development/current/main/phases/phase-29ao/README.md`
- 検証: `cargo build --release` / `./tools/smokes/v2/run.sh --profile quick` / `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh`

**2025-12-30: Phase 29ao P1 完了** ✅
- 目的: composer bridge で `CanonicalLoopFacts` から `DomainPlan` を組み立てる入口を追加（未接続）
- 変更: `src/mir/builder/control_flow/plan/composer/mod.rs` / `docs/development/current/main/phases/phase-29ao/README.md` / `docs/development/current/main/10-Now.md` / `docs/development/current/main/30-Backlog.md` / `CURRENT_TASK.md`
- 検証: `cargo build --release` / `./tools/smokes/v2/run.sh --profile quick` / `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh`

**2025-12-30: Phase 29ao P2 完了** ✅
- 目的: `CanonicalLoopFacts → DomainPlan → PlanNormalizer → CorePlan` の bridge を未接続で固定
- 変更: `src/mir/builder/control_flow/plan/composer/mod.rs` / `docs/development/current/main/phases/phase-29ao/README.md` / `docs/development/current/main/10-Now.md` / `docs/development/current/main/30-Backlog.md` / `CURRENT_TASK.md`
- 検証: `cargo build --release` / `./tools/smokes/v2/run.sh --profile quick` / `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh`

**2025-12-30: Phase 29ao P3 完了** ✅
- 目的: `CanonicalLoopFacts` から `CorePlan::Loop`（skeleton）を direct 生成（Pattern1 subset のみ）
- 変更: `src/mir/builder/control_flow/plan/normalizer/skeleton_loop.rs` / `src/mir/builder/control_flow/plan/normalizer/mod.rs` / `src/mir/builder/control_flow/plan/composer/mod.rs` / `docs/development/current/main/phases/phase-29ao/README.md` / `docs/development/current/main/10-Now.md` / `docs/development/current/main/30-Backlog.md` / `CURRENT_TASK.md`
- 検証: `cargo build --release` / `./tools/smokes/v2/run.sh --profile quick` / `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh`

**2025-12-30: Phase 29ao P4 完了** ✅
- 目的: `exit_kinds_present` を `Frag.exits` に投影（未配線のまま語彙のみ固定）
- 変更: `src/mir/builder/control_flow/plan/normalizer/skeleton_loop.rs` / `src/mir/builder/control_flow/plan/composer/mod.rs` / `docs/development/current/main/phases/phase-29ao/README.md` / `docs/development/current/main/10-Now.md` / `docs/development/current/main/30-Backlog.md` / `CURRENT_TASK.md`
- 検証: `cargo build --release` / `./tools/smokes/v2/run.sh --profile quick` / `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh`

**2025-12-30: Phase 29ao P5 完了** ✅
- 目的: `cleanup_kinds_present` を ExitKind 語彙として `Frag.exits` に投影（未配線のまま語彙のみ固定）
- 変更: `src/mir/builder/control_flow/plan/normalizer/skeleton_loop.rs` / `src/mir/builder/control_flow/plan/composer/mod.rs` / `docs/development/current/main/phases/phase-29ao/README.md` / `docs/development/current/main/10-Now.md` / `docs/development/current/main/30-Backlog.md` / `CURRENT_TASK.md`
- 検証: `cargo build --release` / `./tools/smokes/v2/run.sh --profile quick` / `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh`

**2025-12-30: Phase 29ao P6 完了** ✅
- 目的: `value_join_needed` が立つケースは direct skeleton を採用しない（安全ゲート）
- 変更: `src/mir/builder/control_flow/plan/normalizer/skeleton_loop.rs` / `src/mir/builder/control_flow/plan/composer/mod.rs` / `docs/development/current/main/phases/phase-29ao/README.md` / `docs/development/current/main/10-Now.md` / `docs/development/current/main/30-Backlog.md` / `CURRENT_TASK.md`
- 検証: `cargo build --release` / `./tools/smokes/v2/run.sh --profile quick` / `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh`

**2025-12-30: Phase 29ao P7 完了** ✅
- 目的: `ExprResultPlusCarriers` の語彙固定と最小 verify を追加（未接続）
- 変更: `src/mir/builder/control_flow/plan/normalizer/value_join_args.rs` / `src/mir/builder/control_flow/plan/normalizer/mod.rs` / `src/mir/builder/control_flow/plan/verifier.rs` / `docs/development/current/main/phases/phase-29ao/README.md` / `docs/development/current/main/10-Now.md` / `docs/development/current/main/30-Backlog.md` / `CURRENT_TASK.md`
- 検証: `cargo build --release` / `./tools/smokes/v2/run.sh --profile quick` / `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh`

**2025-12-30: Phase 29ao P8 完了** ✅
- 目的: compose::seq/if_/cleanup が EdgeArgs(layout+values) を保持することをテストで固定
- 変更: `src/mir/builder/control_flow/edgecfg/api/compose/seq.rs` / `src/mir/builder/control_flow/edgecfg/api/compose/if_.rs` / `src/mir/builder/control_flow/edgecfg/api/compose/cleanup.rs` / `docs/development/current/main/phases/phase-29ao/README.md` / `docs/development/current/main/10-Now.md` / `docs/development/current/main/30-Backlog.md` / `CURRENT_TASK.md`
- 検証: `cargo test --release -p nyash-rust` / `cargo build --release` / `./tools/smokes/v2/run.sh --profile quick` / `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh`

**2025-12-30: Phase 29ao P9 完了** ✅
- 目的: EdgeCFG の block params 足場を追加し、strict/dev で join 受け口の整合を fail-fast で固定
- 変更: `src/mir/builder/control_flow/edgecfg/api/block_params.rs` / `src/mir/builder/control_flow/edgecfg/api/frag.rs` / `src/mir/builder/control_flow/edgecfg/api/compose/seq.rs` / `src/mir/builder/control_flow/edgecfg/api/compose/if_.rs` / `src/mir/builder/control_flow/edgecfg/api/compose/cleanup.rs` / `src/mir/builder/control_flow/edgecfg/api/compose/loop_.rs` / `src/mir/builder/control_flow/edgecfg/api/verify.rs` / `src/mir/builder/control_flow/edgecfg/api/emit.rs` / `docs/development/current/main/phases/phase-29ao/README.md` / `docs/development/current/main/10-Now.md` / `docs/development/current/main/30-Backlog.md` / `CURRENT_TASK.md`
- 検証: `cargo test --release -p nyash-rust --lib` / `cargo build --release` / `./tools/smokes/v2/run.sh --profile quick` / `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh`

**2025-12-30: Phase 29ao P10 完了** ✅
- 目的: `Frag.block_params` を `emit_frag()` で PHI に落とす唯一の接続点を追加（未接続のまま）
- 変更: `src/mir/builder/control_flow/edgecfg/api/emit.rs` / `src/mir/builder/control_flow/edgecfg/api/verify.rs` / `docs/development/current/main/phases/phase-29ao/README.md` / `docs/development/current/main/10-Now.md` / `docs/development/current/main/30-Backlog.md` / `CURRENT_TASK.md`
- 検証: `cargo test --release -p nyash-rust --lib` / `cargo build --release` / `./tools/smokes/v2/run.sh --profile quick` / `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh`

**2025-12-30: Phase 29ao P11 完了** ✅
- 目的: Normalizer が `Frag.block_params` を生成する最小ケース（If2 demo）を追加し、PHI挿入まで unit test で固定
- 変更: `src/mir/builder/control_flow/plan/normalizer/value_join_demo_if2.rs` / `src/mir/builder/control_flow/plan/normalizer/mod.rs` / `src/mir/builder/control_flow/edgecfg/api/emit.rs` / `src/mir/builder/control_flow/edgecfg/api/verify.rs` / `docs/development/current/main/phases/phase-29ao/README.md` / `docs/development/current/main/10-Now.md` / `docs/development/current/main/30-Backlog.md` / `CURRENT_TASK.md`
- 検証: `cargo test --release -p nyash-rust --lib` / `cargo build --release` / `./tools/smokes/v2/run.sh --profile quick` / `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh`

**2025-12-30: Phase 29ao P12 完了** ✅
- 目的: Pattern7 SplitScan の step join を `Frag.block_params + EdgeArgs` に移行
- 変更: `src/mir/builder/control_flow/plan/normalizer/pattern_split_scan.rs` / `src/mir/builder/control_flow/plan/normalizer/common.rs`
- 検証: `cargo test --release -p nyash-rust --lib` / `cargo build --release` / `./tools/smokes/v2/run.sh --profile quick` / `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh`

**2025-12-30: Phase 29ao P13 完了** ✅
- 目的: Pattern3 If‑Phi の merge join を `Frag.block_params + EdgeArgs` に移行（expr_result 的な join 値の実使用）
- 変更: `src/mir/builder/control_flow/plan/normalizer/pattern3_if_phi.rs` / `docs/development/current/main/phases/phase-29ao/P13-VALUEJOIN-REAL-USAGE-PATTERN3-IFPHI-MERGE-INSTRUCTIONS.md`
- 検証: `cargo test --release -p nyash-rust --lib` / `cargo build --release` / `./tools/smokes/v2/run.sh --profile quick` / `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh`

**2025-12-30: Phase 29ao P14 完了** ✅
- 目的: Pattern2 Break の after join を `Frag.block_params + EdgeArgs` に移行
- 変更: `src/mir/builder/control_flow/plan/normalizer/pattern2_break.rs` / `docs/development/current/main/phases/phase-29ao/P14-VALUEJOIN-REAL-USAGE-PATTERN2-BREAK-EXITJOIN-INSTRUCTIONS.md`
- 検証: `cargo test --release -p nyash-rust --lib` / `cargo build --release` / `./tools/smokes/v2/run.sh --profile quick` / `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh`

**2025-12-30: Phase 29ao P15 完了** ✅
- 目的: JoinIR 回帰パック（phase29ae pack）に Pattern3(If‑Phi, VM) を追加して、P13 の実経路をゲートで固定
- 変更: `tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh` / `docs/development/current/main/phases/phase-29ae/README.md` / `docs/development/current/main/phases/phase-29ao/P15-REGRESSION-PACK-INCLUDE-PATTERN3-INSTRUCTIONS.md`
- 検証: `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh` / `./tools/smokes/v2/run.sh --profile quick`

**2025-12-30: Phase 29ao P16 完了** ✅
- 目的: Pattern5 Infinite Early-Exit の after join を `Frag.block_params + EdgeArgs` に移行
- 変更: `src/mir/builder/control_flow/plan/normalizer/pattern5_infinite_early_exit.rs` / `tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh` / `docs/development/current/main/phases/phase-29ae/README.md`
- 検証: `cargo test --release -p nyash-rust --lib` / `cargo build --release` / `./tools/smokes/v2/run.sh --profile quick` / `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh`

**2025-12-30: Phase 29ao P17 完了** ✅
- 目的: strict/dev のみ Pattern1 を Facts→CorePlan(skeleton) で shadow adopt
- 変更: `src/mir/builder/control_flow/joinir/patterns/router.rs` / `docs/development/current/main/phases/phase-29ao/P17-COMPOSER-PATTERN1-STRICT-SHADOW-INSTRUCTIONS.md`
- 検証: `cargo test --release -p nyash-rust --lib` / `cargo build --release` / `./tools/smokes/v2/run.sh --profile quick` / `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh`

**2025-12-30: Phase 29ao P18 完了** ✅
- 目的: single_planner から planner outcome を受け取り、router の二重 planner 実行を撤去
- 変更: `src/mir/builder/control_flow/plan/single_planner/rules.rs` / `src/mir/builder/control_flow/plan/single_planner/mod.rs` / `src/mir/builder/control_flow/joinir/patterns/router.rs`
- 検証: `cargo test --release -p nyash-rust --lib` / `cargo build --release` / `./tools/smokes/v2/run.sh --profile quick` / `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh`

**2025-12-29: Phase 29an P15 完了** ✅
- 目的: P0–P14 の成果を closeout 形式でまとめ、次フェーズ（Phase 29ao）入口を固定
- 変更: `docs/development/current/main/phases/phase-29an/README.md` / `docs/development/current/main/10-Now.md` / `docs/development/current/main/30-Backlog.md` / `CURRENT_TASK.md`

**2025-12-29: Phase 29an P8 完了** ✅
- 目的: exit_usage と DomainPlan（Pattern1/2/4/5）の整合を debug-only で固定（release は仕様不変）
- 変更: `src/mir/builder/control_flow/plan/planner/build.rs`
- 検証: `cargo build --release` / `./tools/smokes/v2/run.sh --profile quick` / `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh`

**2025-12-29: Phase 29an P9 完了** ✅
- 目的: SkeletonFacts の一意化推論 API を追加し、0/1/2+ 境界をテストで固定（未接続・仕様不変）
- 変更: `src/mir/builder/control_flow/plan/facts/skeleton_facts.rs`
- 検証: `cargo build --release` / `./tools/smokes/v2/run.sh --profile quick` / `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh`

**2025-12-29: Phase 29an P10 完了** ✅
- 目的: ExitMapFacts の語彙足場（型）を追加（未接続・仕様不変）
- 変更: `src/mir/builder/control_flow/plan/facts/feature_facts.rs`
- 検証: `cargo build --release` / `./tools/smokes/v2/run.sh --profile quick` / `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh`

**2025-12-29: Phase 29an P11 完了** ✅
- 目的: ExitMapFacts を “存在集合” として最小で埋める（対応付け/CFGなし、仕様不変）
- 変更: `src/mir/builder/control_flow/plan/facts/feature_facts.rs`
- 検証: `cargo build --release` / `./tools/smokes/v2/run.sh --profile quick` / `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh`

**2025-12-29: Phase 29an P12 完了** ✅
- 目的: CanonicalLoopFacts に exitmap presence（exit_kinds_present）を投影（挙動不変）
- 変更: `src/mir/builder/control_flow/plan/normalize/canonicalize.rs` / `src/mir/builder/control_flow/plan/planner/build.rs`
- 検証: `cargo build --release` / `./tools/smokes/v2/run.sh --profile quick` / `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh`

**2025-12-29: Phase 29an P13 完了** ✅
- 目的: CleanupFacts の語彙足場 + canonical projection を追加（未接続・仕様不変）
- 変更: `src/mir/builder/control_flow/plan/facts/feature_facts.rs` / `src/mir/builder/control_flow/plan/normalize/canonicalize.rs`
- 検証: `cargo build --release` / `./tools/smokes/v2/run.sh --profile quick` / `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh`

**2025-12-29: Phase 29an P14 完了** ✅
- 目的: ValueJoinFacts の語彙足場 + canonical projection を追加（未接続・仕様不変）
- 変更: `src/mir/builder/control_flow/plan/facts/feature_facts.rs` / `src/mir/builder/control_flow/plan/normalize/canonicalize.rs` / `src/mir/builder/control_flow/plan/verifier.rs`
- 検証: `cargo build --release` / `./tools/smokes/v2/run.sh --profile quick` / `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh`

**2025-12-29: Phase 29an P7 完了** ✅
- 目的: CanonicalLoopFacts に skeleton/exit_usage の projection を追加（挙動不変）
- 変更: `src/mir/builder/control_flow/plan/normalize/canonicalize.rs` / `src/mir/builder/control_flow/plan/planner/build.rs`
- 検証: `cargo build --release` / `./tools/smokes/v2/run.sh --profile quick` / `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh`

**2025-12-29: Phase 29an P6 完了** ✅
- 目的: planner入口に skeleton gate を追加（Loop 以外は Ok(None) で fallback 維持、仕様不変）
- 変更: `src/mir/builder/control_flow/plan/planner/build.rs`
- 検証: `cargo build --release` / `./tools/smokes/v2/run.sh --profile quick` / `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh`

**2025-12-29: Phase 29an P4 完了** ✅
- 目的: LoopFacts が Some のとき skeleton/features が必ず揃うように型を引き締め（SSOT引き締め、仕様不変）
- 変更: `src/mir/builder/control_flow/plan/facts/loop_facts.rs` / `src/mir/builder/control_flow/plan/planner/build.rs`
- 検証: `cargo build --release` / `./tools/smokes/v2/run.sh --profile quick` / `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh`

**2025-12-29: Phase 29an P5 完了** ✅
- 目的: SkeletonFacts の If2 判定を else 有無に依存しない形へ修正（未接続・仕様不変）
- 変更: `src/mir/builder/control_flow/plan/facts/skeleton_facts.rs`
- 検証: `cargo build --release` / `./tools/smokes/v2/run.sh --profile quick` / `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh`

**2025-12-29: Phase 29an P3 完了** ✅
- 目的: Freeze taxonomy の `unstructured` タグをコード語彙へ追加（未使用のまま、仕様不変）
- 変更: `src/mir/builder/control_flow/plan/planner/freeze.rs`
- 検証: `cargo build --release` / `./tools/smokes/v2/run.sh --profile quick` / `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh`

**2025-12-29: Phase 29an P2 完了** ✅
- 目的: planner を Skeleton→Feature→CandidateSet の段取りへ整理（候補/順序/挙動は不変）
- 変更: `src/mir/builder/control_flow/plan/planner/build.rs`
- 検証: `cargo build --release` / `./tools/smokes/v2/run.sh --profile quick` / `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh`

**2025-12-29: Phase 29an P1 完了** ✅
- 目的: FeatureFacts（まず ExitUsage）を Facts SSOT として追加（仕様不変・未接続）
- 変更: `src/mir/builder/control_flow/plan/facts/feature_facts.rs` / `src/mir/builder/control_flow/plan/facts/loop_facts.rs`
- 検証: `cargo build --release` / `./tools/smokes/v2/run.sh --profile quick` / `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh`

**2025-12-29: Phase 29an P0 完了** ✅
- 目的: SkeletonFacts（Loop/If/BranchN/StraightLine）を Facts SSOT として追加（仕様不変・未接続）
- 変更: `src/mir/builder/control_flow/plan/facts/skeleton_facts.rs` / `src/mir/builder/control_flow/plan/facts/loop_facts.rs`
- 検証: `cargo build --release` / `./tools/smokes/v2/run.sh --profile quick` / `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh`

**2025-12-29: Phase 29am P3 完了** ✅
- 目的: Exit を “独立ノード増殖” にせず、Frag/ExitMap と整合する表現へ寄せる（仕様不変）
- 変更: `src/mir/builder/control_flow/plan/verifier.rs`（[V11]）
- 検証: `cargo build --release` / `./tools/smokes/v2/run.sh --profile quick` / `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh`

**2025-12-29: Phase 29am P2 完了** ✅
- 目的: Loop.body の語彙制約（Effect-only + Seq許可）を verifier に前倒しして fail-fast を局所化
- 変更: `src/mir/builder/control_flow/plan/verifier.rs`（[V12]）
- 検証: `cargo build --release` / `./tools/smokes/v2/run.sh --profile quick` / `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh`

**2025-12-29: Phase 29am P1 完了** ✅
- 目的: CoreLoopPlan.body で `Seq([Effect...])` を flatten して emit（Effect-only制約は維持）
- 変更: `src/mir/builder/control_flow/plan/lowerer.rs`
- 検証: `cargo build --release` / `./tools/smokes/v2/run.sh --profile quick` / `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh`

**2025-12-29: Phase 29am P0 完了** ✅
- 目的: CorePlan の If/Exit を lowerer/verifier で扱えるようにして、CorePlan 移行の土台を作る（仕様不変）
- 変更: `src/mir/builder/control_flow/plan/lowerer.rs` / `src/mir/builder/control_flow/plan/verifier.rs`
- 検証: `cargo build --release` / `./tools/smokes/v2/run.sh --profile quick` / `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh`

**2025-12-29: Phase 29al P3 完了** ✅
- 目的: cleanup を ExitKind と effect の契約として固定（仕様不変）
- SSOT: `docs/development/current/main/design/exitkind-cleanup-effect-contract-ssot.md`

**2025-12-29: Phase 29al P2 完了** ✅
- 目的: effect 分類と “許される変形” の最小法典を SSOT 化（仕様不変）
- SSOT: `docs/development/current/main/design/effect-classification-ssot.md`

**2025-12-29: Phase 29al P1 完了** ✅
- 目的: join 値（PHI相当）の最終表現と局所 verify を SSOT 化（仕様不変）
- SSOT: `docs/development/current/main/design/post-phi-final-form-ssot.md`

**2025-12-29: Phase 29al P0 完了** ✅
- 目的: Skeleton/Feature model を SSOT 化し、「通らない/危険」形を Freeze taxonomy に落とす（仕様不変）
- SSOT: `docs/development/current/main/design/coreplan-skeleton-feature-model.md`

**2025-12-29: Phase 29ak P5 完了** ✅
- 目的: planner の candidate gate を SSOT 化し、single_planner の Pattern1 抑制を撤去（仕様不変）
- 実装: `src/mir/builder/control_flow/plan/planner/build.rs` / `src/mir/builder/control_flow/plan/planner/outcome.rs` / `src/mir/builder/control_flow/plan/single_planner/rules.rs`
- 検証: `cargo build --release` / `./tools/smokes/v2/run.sh --profile quick` / `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh` PASS

**2025-12-29: Phase 29ak P4 完了** ✅
- 目的: Pattern1 guard を single_planner から撤去（仕様不変）
- 実装: `src/mir/builder/control_flow/plan/single_planner/rules.rs`
- 検証: `cargo build --release` / `./tools/smokes/v2/run.sh --profile quick` / `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh` PASS

**2025-12-29: Phase 29ak P3 完了** ✅
- 目的: Pattern8 static box filter を single_planner から撤去（仕様不変）
- 実装: `src/mir/builder/control_flow/plan/single_planner/rules.rs`
- 検証: `cargo build --release` / `./tools/smokes/v2/run.sh --profile quick` / `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh` PASS

**2025-12-29: Phase 29ak P2 完了** ✅
- 目的: Pattern8 static box filter を planner 側へ移し、facts 抽出を抑制（仕様不変）
- 実装: `src/mir/builder/control_flow/plan/facts/loop_facts.rs`
- 検証: `cargo build --release` / `./tools/smokes/v2/run.sh --profile quick` / `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh` PASS

**2025-12-29: Phase 29ak P1 完了** ✅
- 目的: Pattern1 guard を planner 側へ移して facts 抽出を抑制（仕様不変）
- 実装: `src/mir/builder/control_flow/plan/facts/loop_facts.rs` / `src/mir/builder/control_flow/plan/planner/outcome.rs`
- 検証: `cargo build --release` / `./tools/smokes/v2/run.sh --profile quick` / `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh` PASS

**2025-12-29: Phase 29ak P0 完了** ✅
- 目的: PlanRuleOrder SSOT を新設し、PlannerContext の配線だけ先に導入（仕様不変）
- 実装: `src/mir/builder/control_flow/plan/single_planner/rule_order.rs` / `src/mir/builder/control_flow/plan/planner/context.rs` / `src/mir/builder/control_flow/plan/single_planner/rules.rs`
- 検証: `cargo build --release` / `./tools/smokes/v2/run.sh --profile quick` / `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh` PASS

**2025-12-29: Phase 29aj P10 完了** ✅
- 目的: single_planner を全パターン planner-first 形に統一（挙動不変）
- 実装: `src/mir/builder/control_flow/plan/single_planner/rules.rs`
- 検証: `cargo build --release` / `./tools/smokes/v2/run.sh --profile quick` / `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh` PASS

**2025-12-29: Phase 29aj P9 完了** ✅
- 目的: phase286_pattern9_frag_poc を legacy pack (SKIP) に隔離して SSOT を固定
- 実装: `tools/smokes/v2/profiles/integration/joinir/phase286_pattern9_legacy_pack.sh` / `docs/development/current/main/phases/phase-29aj/README.md` / `docs/development/current/main/phases/phase-29ae/README.md`
- 検証: `./tools/smokes/v2/run.sh --profile quick` / `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh` / `./tools/smokes/v2/profiles/integration/joinir/phase286_pattern9_legacy_pack.sh` (SKIP)

**2025-12-29: Phase 29aj P8 完了** ✅
- 目的: Pattern9（AccumConstLoop）を Facts→Planner-first に移行（仕様不変）
- 実装: `src/mir/builder/control_flow/plan/facts/pattern9_accum_const_loop_facts.rs` / `src/mir/builder/control_flow/plan/planner/build.rs` / `src/mir/builder/control_flow/plan/single_planner/rules.rs`
- 検証: `cargo build --release` / `./tools/smokes/v2/run.sh --profile quick` / `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh` PASS

**2025-12-29: Phase 29aj P7 完了** ✅
- 目的: Pattern8（BoolPredicateScan）を Facts→Planner-first に移行（仕様不変）
- 実装: `src/mir/builder/control_flow/plan/facts/pattern8_bool_predicate_scan_facts.rs` / `src/mir/builder/control_flow/plan/planner/build.rs` / `src/mir/builder/control_flow/plan/single_planner/rules.rs`
- 検証: `cargo build --release` / `./tools/smokes/v2/run.sh --profile quick` / `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh` PASS

**2025-12-29: Phase 29aj P6 完了** ✅
- 目的: JoinIR 回帰の integration gate を phase29ae pack に固定し、phase143_* を隔離
- 実装: `tools/smokes/v2/profiles/integration/joinir/phase143_legacy_pack.sh` / `docs/development/current/main/phases/phase-29aj/README.md` / `docs/development/current/main/phases/phase-29ae/README.md`
- 検証: `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh` / `./tools/smokes/v2/profiles/integration/joinir/phase143_legacy_pack.sh` (SKIP)

**2025-12-29: Phase 29aj P5 完了** ✅
- 目的: Pattern5（Infinite Early Exit）を Facts→Planner-first に移行（仕様不変）
- 実装: `src/mir/builder/control_flow/plan/facts/pattern5_infinite_early_exit_facts.rs` / `src/mir/builder/control_flow/plan/planner/build.rs` / `src/mir/builder/control_flow/plan/single_planner/rules.rs`
- 検証: `cargo build --release` / `./tools/smokes/v2/run.sh --profile quick` / `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh` / `./tools/smokes/v2/run.sh --profile integration --filter "phase143_"` PASS

**2025-12-29: Phase 29aj P4 完了** ✅
- 目的: Pattern4（Continue）を Facts→Planner-first に移行（仕様不変）
- 実装: `src/mir/builder/control_flow/plan/facts/pattern4_continue_facts.rs` / `src/mir/builder/control_flow/plan/planner/build.rs` / `src/mir/builder/control_flow/plan/single_planner/rules.rs`
- 検証: `cargo build --release` / `./tools/smokes/v2/run.sh --profile quick` / `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh` / `./tools/smokes/v2/run.sh --profile integration --filter "phase286_pattern4_frag_poc"` PASS

**2025-12-29: Phase 29aj P3 完了** ✅
- 目的: Pattern3（If-Phi）を Facts→Planner-first に移行（仕様不変）
- 実装: `src/mir/builder/control_flow/plan/facts/pattern3_ifphi_facts.rs` / `src/mir/builder/control_flow/plan/planner/build.rs` / `src/mir/builder/control_flow/plan/single_planner/rules.rs`
- 検証: `cargo build --release` / `./tools/smokes/v2/run.sh --profile quick` / `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh` PASS

**2025-12-29: Phase 29aj P2 完了** ✅
- 目的: chosen_rule を撤去し、Pattern1 を Facts→Planner-first に移行（仕様不変）
- 実装: `src/mir/builder/control_flow/plan/planner/outcome.rs` / `src/mir/builder/control_flow/plan/facts/pattern1_simplewhile_facts.rs` / `src/mir/builder/control_flow/plan/planner/build.rs` / `src/mir/builder/control_flow/plan/single_planner/rules.rs`
- 検証: `cargo build --release` / `./tools/smokes/v2/run.sh --profile quick` / `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh` PASS

**2025-12-29: Phase 29aj P1 完了** ✅
- 目的: single_planner の legacy_rules を撤去し、plan extractor を SSOT に集約（仕様不変）
- 実装: `src/mir/builder/control_flow/plan/extractors/pattern{1,3,4,5,8,9}.rs` / `src/mir/builder/control_flow/plan/single_planner/rules.rs`
- 検証: `cargo build --release` / `./tools/smokes/v2/run.sh --profile quick` / `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh` PASS

**2025-12-29: Phase 29aj P0 完了** ✅
- 目的: planner outcome（facts+plan）を SSOT 化して strict 観測の再スキャンを撤去（仕様不変）
- 実装: `src/mir/builder/control_flow/plan/planner/outcome.rs` / `src/mir/builder/control_flow/plan/planner/build.rs` / `src/mir/builder/control_flow/plan/single_planner/rules.rs`
- 検証: `cargo build --release` / `./tools/smokes/v2/run.sh --profile quick` / `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh` PASS

**2025-12-29: Phase 29ai P15 完了** ✅
- 目的: strict/dev のときだけ LoopBodyLocal facts を安定タグで観測できるようにする（仕様不変）
- 実装: `src/mir/builder/control_flow/plan/single_planner/rules.rs` / `tools/smokes/v2/profiles/integration/apps/archive/phase29ab_pattern2_loopbodylocal_seg_min_vm.sh` / `tools/smokes/v2/profiles/integration/apps/archive/phase29ab_pattern2_loopbodylocal_min_vm.sh`
- 検証: `cargo build --release` / `./tools/smokes/v2/run.sh --profile quick` / `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh` PASS

**2025-12-29: Phase 29ai P14 完了** ✅
- 目的: Pattern2 LoopBodyLocal promotion の要求を Plan に載せる（仕様不変）
- 実装: `src/mir/builder/control_flow/plan/mod.rs` / `src/mir/builder/control_flow/plan/planner/build.rs` / `src/mir/builder/control_flow/plan/extractors/pattern2_break.rs`
- 検証: `cargo build --release` / `./tools/smokes/v2/run.sh --profile quick` / `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh` PASS

**2025-12-29: Phase 29ai P13 完了** ✅
- 目的: single_planner の planner 呼び出しを 1 回に memoize して二重スキャンを解消（仕様不変）
- 実装: `src/mir/builder/control_flow/plan/single_planner/rules.rs`
- 検証: `cargo build --release` / `./tools/smokes/v2/run.sh --profile quick` / `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh` PASS

**2025-12-29: Phase 29ai P12 完了** ✅
- 目的: Pattern2 LoopBodyLocal promotion の facts 抽出を SSOT 化（仕様不変）
- 実装: `src/mir/builder/control_flow/plan/facts/pattern2_loopbodylocal_facts.rs` / `src/mir/builder/control_flow/plan/facts/loop_facts.rs` / `src/mir/builder/control_flow/plan/facts/mod.rs`
- 検証: `cargo build --release` / `./tools/smokes/v2/run.sh --profile quick` / `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh` PASS

**2025-12-29: Phase 29ai P11 完了** ✅
- 目的: Pattern2 break subset を Facts→Planner に吸収し、single_planner で planner-first を開始（仕様不変）
- 実装: `src/mir/builder/control_flow/plan/facts/pattern2_break_facts.rs` / `src/mir/builder/control_flow/plan/planner/build.rs` / `src/mir/builder/control_flow/plan/single_planner/rules.rs`
- 検証: `cargo build --release` / `./tools/smokes/v2/run.sh --profile quick` / `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh` / `./tools/smokes/v2/run.sh --profile integration --filter "phase29ai_pattern2_break_plan_subset_ok_min"` PASS

**2025-12-29: Phase 29ai P10 完了** ✅
- 目的: Pattern2 extractor を plan 層へ移設して依存方向を固定（仕様不変）
- 実装: `src/mir/builder/control_flow/plan/extractors/pattern2_break.rs` / `src/mir/builder/control_flow/joinir/patterns/extractors/pattern2.rs` / `src/mir/builder/control_flow/plan/single_planner/rules.rs`
- 検証: `cargo build --release` / `./tools/smokes/v2/run.sh --profile quick` / `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh` PASS

**2025-12-29: Phase 29ai P9 完了** ✅
- 目的: Pattern7 split-scan subset を Facts→Planner→DomainPlan まで到達させ、single_planner の Pattern7 で planner-first を開始（仕様不変）
- 実装: `src/mir/builder/control_flow/plan/facts/loop_facts.rs` / `src/mir/builder/control_flow/plan/planner/build.rs` / `src/mir/builder/control_flow/plan/single_planner/rules.rs`
- 検証: `cargo build --release` / `./tools/smokes/v2/run.sh --profile quick` / `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh` PASS

**2025-12-29: Phase 29ai P8 完了** ✅
- 目的: Facts→Planner を実行経路へ 1 歩だけ接続し、Pattern6（scan-with-init）の subset から吸収を開始（仕様不変）
- 実装: `src/mir/builder/control_flow/plan/single_planner/rules.rs`（Pattern6 rule の先頭で planner を試す）
- 検証: `cargo build --release` / `./tools/smokes/v2/run.sh --profile quick` / `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh` PASS

**2025-12-29: Phase 29ag P1 完了** ✅
- 目的: coordinator の ValueId(idx) 前提を撤去し、boundary.join_inputs を SSOT 化（仕様不変）
- 入口: `docs/development/current/main/phases/phase-29ag/README.md`
- 検証: `cargo build --release` / `./tools/smokes/v2/run.sh --profile quick` / `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh` PASS
- 回帰パック: `phase263_pattern2_*` を追加済み

**2025-12-29: Phase 29ah P0 完了** ✅
- 目的: JoinIR 回帰パックに real-world Pattern2（Phase 263）を追加（仕様不変）
- 入口: `docs/development/current/main/phases/phase-29ah/README.md`
- 回帰パック: `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh`

**2025-12-29: Phase 29af P5 完了** ✅
- 目的: 29af を closeout して JoinIR 回帰確認を 1 本に収束（仕様不変）
- 回帰パック: `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh`
- 検証: `cargo build --release` / `./tools/smokes/v2/run.sh --profile quick` / `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh` PASS

**2025-12-29: Phase 29af P4 完了** ✅
- 目的: BoundaryCarrierLayout と header PHI の同順一致を strict/dev で Fail-Fast 固定（仕様不変）
- 入口: `src/mir/builder/control_flow/joinir/merge/contract_checks/header_phi_layout.rs`
- 検証: `cargo build --release` / `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh` / `./tools/smokes/v2/run.sh --profile quick` PASS

**2025-12-29: Phase 29af P3 完了** ✅
- 目的: carrier の順序（loop_var + carriers）を merge 側 SSOT に統合（仕様不変）
- 入口: `src/mir/builder/control_flow/joinir/merge/boundary_carrier_layout.rs`
- 検証: `cargo build --release` / `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh` / `./tools/smokes/v2/run.sh --profile quick` PASS

**2025-12-29: Phase 29af P1 完了** ✅
- 目的: boundary hygiene を merge 入口（`contract_checks`）へ集約して再発検知を SSOT 化（仕様不変）
- 実装: `src/mir/builder/control_flow/joinir/merge/contract_checks/boundary_hygiene.rs`（strict/dev のみ）
- 配線: `src/mir/builder/control_flow/joinir/merge/contract_checks/boundary_creation.rs`
- 検証: `cargo build --release` / `./tools/smokes/v2/run.sh --profile quick` / `./tools/smokes/v2/run.sh --profile integration --filter "phase29ab_pattern2_"` / `./tools/smokes/v2/run.sh --profile integration --filter "phase1883_"` PASS
- JoinIR 回帰確認: `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh`

**2025-12-29: Phase 29af P0 完了** ✅
- 目的: Pattern2 の boundary 情報の歪みを SSOT 化し、exit/header/latch の責務境界を固定（仕様不変）
- 入口: `docs/development/current/main/phases/phase-29af/README.md`
- 変更: exit_bindings は LoopState のみ（ConditionOnly/LoopLocalZero は carrier_info→header PHI）
- 検証: `cargo build --release` / `./tools/smokes/v2/run.sh --profile quick` / `./tools/smokes/v2/run.sh --profile integration --filter "phase29ab_pattern2_"` / `./tools/smokes/v2/run.sh --profile integration --filter "phase1883_"` PASS

**2025-12-28: Phase 29ae P1 完了** ✅
- 目的: Merge/Phi Contract SSOT + 回帰パック完全固定
- 入口: `docs/development/current/main/phases/phase-29ae/README.md`
- 追加: Header PHI Entry/Latch Contract の要点を SSOT 化
- 検証: `./tools/smokes/v2/run.sh --profile integration --filter "phase1883_"` / `phase29ab_pattern2_` / `phase29ab_pattern6_` / `phase29ab_pattern7_` PASS

**2025-12-28: Phase 29ae P0 完了** ✅
- 目的: JoinIR の最小回帰セットを SSOT で固定
- 入口: `docs/development/current/main/phases/phase-29ae/README.md`

**2025-12-28: Phase 29ad 完了** ✅
- 目的: Pattern6/7 fixture/smoke の命名規約を SSOT 化し、variant 名を明示して迷いを消す
- 入口: `docs/development/current/main/phases/phase-29ad/README.md`

**2025-12-28: Phase 29ac 完了** ✅
- 目的: Pattern6/7 の near-miss（freeze固定）を、契約維持のまま PASS へ倒す（silent fallback禁止）
- 入口: `docs/development/current/main/phases/phase-29ac/README.md`
- 結果: reverse/matchscan は `phase29ab_pattern6_*` を OK 化、split-scan は near-miss OK fixture（`phase29ab_pattern7_splitscan_nearmiss_ok_min`）を追加

**2025-12-28: Phase 29ab P1 完了** ✅
- 目的: Pattern2 の LoopBodyLocal promotion の最小ケースを fixture+integration smoke で固定
- 実装: carrier binding policy を導入し、ConditionOnly / LoopLocalZero を host binding しない
- 追加: `apps/tests/phase29ab_pattern2_loopbodylocal_min.hako`
- Smoke: `tools/smokes/v2/profiles/integration/apps/archive/phase29ab_pattern2_loopbodylocal_min_vm.sh`
- 検証: `./tools/smokes/v2/run.sh --profile integration --filter "phase29ab_pattern2_*"` PASS

**2025-12-28: Phase 29ab P2 完了** ✅
- 目的: Pattern2 Trim（A-3）相当の `LoopBodyLocal(seg)` 最小ケースを fixture+integration smoke で固定
- 追加: `apps/tests/phase29ab_pattern2_loopbodylocal_seg_min.hako`
- Smoke: `tools/smokes/v2/profiles/integration/apps/archive/phase29ab_pattern2_loopbodylocal_seg_min_vm.sh`
- 検証: `./tools/smokes/v2/run.sh --profile integration --filter "phase29ab_pattern2_*"` PASS（2/2）

**2025-12-28: Phase 29ab P3 完了** ✅
- 目的: Pattern2 promotion の NotApplicable/Freeze 境界を contract+fixture+smoke で固定（JoinIR-only前提）
- 契約: `src/mir/builder/control_flow/joinir/patterns/pattern2/api/README.md`
- fixtures: `apps/tests/phase29ab_pattern2_seg_{notapplicable,freeze}_min.hako`
- smokes: `tools/smokes/v2/profiles/integration/apps/phase29ab_pattern2_seg_{notapplicable,freeze}_min_vm.sh`
- 検証: `./tools/smokes/v2/run.sh --profile integration --filter "phase29ab_pattern2_*"` PASS（4/4）

**2025-12-28: Phase 29ab P4 完了** ✅
- 目的: Phase 263 の実ログ seg を Derived slot 方針で通し、fixture+integration smoke で PASS 固定
- Smoke: `tools/smokes/v2/profiles/integration/apps/archive/phase263_pattern2_seg_realworld_min_vm.sh`
- 追加/調整: `String.indexOf(search, fromIndex)` の2引数対応（arity解決を含む）を VM/JoinIR lowering 側で整備

**2025-12-28: Phase 29ab P5 完了** ✅
- 目的: Pattern7 SplitScan の「形は近いが契約違反」ケースを extractor 段で freeze に固定（SSOT化）
- 実装: `src/mir/builder/control_flow/joinir/patterns/pattern7_split_scan.rs`（`freeze_with_hint("phase29ab/pattern7/contract", ...)`）
- fixture: `apps/tests/phase29ab_pattern7_splitscan_contract_min.hako`
- smoke: `tools/smokes/v2/profiles/integration/apps/archive/phase29ab_pattern7_splitscan_contract_min_vm.sh`
- 検証: `./tools/smokes/v2/run.sh --profile integration --filter "phase29ab_pattern7_*"` PASS

**2025-12-28: Phase 29ab P6 完了** ✅
- 目的: Pattern6 ScanWithInit の near-miss（契約違反）を extractor 段で freeze に固定（SSOT化）
- 実装: `src/mir/builder/control_flow/joinir/patterns/pattern6_scan_with_init.rs`（`[joinir/phase29ab/pattern6/contract]`）
- fixture: `apps/tests/phase29ab_pattern6_scan_with_init_contract_min.hako`
- smoke: `tools/smokes/v2/profiles/integration/apps/archive/phase29ab_pattern6_scan_with_init_contract_min_vm.sh`
- 検証: `./tools/smokes/v2/run.sh --profile integration --filter "phase29ab_pattern6_*"` PASS

**2025-12-28: Phase 29ab P7 完了** ✅
- 目的: Pattern6 reverse scan / matchscan の near-miss を追加で freeze 固定し、契約境界を安定化
- fixtures: `apps/tests/phase29ab_pattern6_{reverse,matchscan}_contract_min.hako`
- smokes: `tools/smokes/v2/profiles/integration/apps/phase29ab_pattern6_{reverse,matchscan}_contract_min_vm.sh`
- 検証: `./tools/smokes/v2/run.sh --profile integration --filter "phase29ab_pattern6_*"` PASS（3/3）

**2025-12-28: Phase 29ab P8 完了** ✅
- 目的: Pattern6/7 の正常系 fixture を Plan/Frag/compose 経路で PASS 固定（freeze を減らす）
- Pattern6 OK: `apps/tests/phase29ab_pattern6_scan_with_init_ok_min.hako`
- Pattern7 OK: `apps/tests/phase29ab_pattern7_splitscan_ok_min.hako`
- Smokes: `phase29ab_pattern6_scan_with_init_ok_min_vm`, `phase29ab_pattern7_splitscan_ok_min_vm`

**2025-12-28: Phase 29ab P9 完了** ✅
- 目的: Phase 29ab closeout（入口SSOTの集約）
- 入口: `docs/development/current/main/phases/phase-29ab/README.md`

**2025-12-28: Phase 29aa P5 完了** ✅
- 目的: Return block が複数 predecessor のとき、incoming state が完全一致する場合のみ ReturnCleanup を成立させる
- 入口: `docs/development/current/main/phases/phase-29aa/README.md`
- Safety Contract: state 不一致なら join state を作らない（誤 release 防止）
- Selfcheck: Case 3.7（state一致→cleanup）/ Case 3.8（state不一致→no cleanup）PASS
- 検証: quick 154/154 PASS / selfcheck PASS

**2025-12-28: Phase 29aa P6 完了** ✅
- 目的: multi-predecessor Return で state 不一致でも、共通部分（intersection）だけ ReturnCleanup を成立
- 入口: `docs/development/current/main/phases/phase-29aa/README.md`
- Safety Contract: intersection（全経路で ptr=val が一致するもののみ）で cleanup
- Conservative: value 不一致の ptr は release しない＝リーク方向（安全だが保守的）
- Selfcheck: Case 3.9（部分一致→cleanup）/ Case 3.10（intersection空→no cleanup）PASS
- 検証: quick 154/154 PASS / selfcheck PASS

**2025-12-28: Phase 29aa P7 完了** ✅
- 目的: ReleaseStrong の `values` 順序を決定的にする（HashSet/HashMap 由来の非決定性排除）
- 入口: `docs/development/current/main/phases/phase-29aa/README.md`
- Contract: `sort_unstable()` + `dedup()` で ValueId 昇順に固定
- ヘルパー関数 `sorted_release_values` で全 ReleaseStrong 生成箇所（2箇所）を統一
- Selfcheck: Case 3.11（values が昇順であることを検証）PASS
- 検証: quick 154/154 PASS / selfcheck PASS

**2025-12-28: Phase 29aa P8 完了** ✅
- 目的: CFG を跨いだ null 伝播（Copy-only）で explicit drop（Store null）の精度を上げる
- 入口: `docs/development/current/main/phases/phase-29aa/README.md`
- Contract: single-predecessor Jump-chain のみ null_values 伝播（multi-pred Return は合流しない）
- Non-goals: edge_args 経由の null 伝播（ValueId 同一性だけでは追えない）
- Selfcheck: Case 3.12（null 伝播で ReturnCleanup なし）PASS
- 検証: quick 154/154 PASS / selfcheck PASS

**2025-12-27: Phase 29aa P4 完了** ✅
- 目的: Jump の直列チェーン（単一 predecessor）を通して ReturnCleanup を成立させる（cleanup は Return block のみ）
- 入口: `docs/development/current/main/phases/phase-29aa/README.md`

**2025-12-27: Phase 29aa P3 完了** ✅
- 目的: Jump→Return（単一 predecessor）で state 伝播し ReturnCleanup を成立
- 入口: `docs/development/current/main/phases/phase-29aa/README.md`

**2025-12-27: Phase 29z P2 closeout** ✅
- `src/mir/passes/rc_insertion.rs`: `Store` 上書き + `Store null`（explicit drop）+ Return終端cleanup の最小 release 挿入（単一block・安全ガード）
- 既定OFF: Cargo feature `rc-insertion-minimal`（env var 新設なし）
- 検証: quick 154/154 PASS 維持 + `cargo run --bin rc_insertion_selfcheck --features rc-insertion-minimal`
- 入口: `docs/development/current/main/phases/phase-29z/README.md`
- 次: Phase 29aa P2（CFG-aware の最小ターゲット選定）

**2025-12-27: Phase 29y P0 完了** ✅
- docs-first SSOT finalized（ABI/RC insertion/Observability）
- 3つのSSOT文書（10/20/30）を Ready に確定
- pilot 実装（Phase 29y.1）の入口を固定
- Next steps（実装タスク3つ）を明文化
- 入口: `docs/development/current/main/phases/phase-29y/README.md`
- **Next**: Phase 29aa（CFG-aware RC insertion）または Phase 29x（De-Rust runtime）候補

**2025-12-27: Phase 287 完了** ✅
- P0-P9完了（big files modularization / facade pattern / SSOT establishment / closeout）
- 検証: quick 154/154 PASS維持、意味論不変
- 入口: `docs/development/current/main/phases/phase-287/README.md`

**2025-12-27: Phase 188.3 完了** ✅
- Pattern6（NestedLoopMinimal）: `apps/tests/phase1883_nested_minimal.hako` が RC=9
- Merge SSOT（latch/entry-like/double latch）を固定（BackEdgeのみlatch記録、main entry blockのみentry-like、二重latchはdebug_assert）
- `./tools/smokes/v2/run.sh --profile quick` 154/154 PASS 維持
- 入口: `docs/development/current/main/phases/phase-188.3/README.md`
- 次の指示書（refactor挟み込み）: `docs/development/current/main/phases/phase-188.3/P2-REFACTORING-INSTRUCTIONS.md`

**2025-12-27: Phase 287 P0 完了** ✅
- `merge/mod.rs` を modularize（1,555 → 1,053 lines）
- SSOT: `boundary.loop_header_func_name` 優先、`boundary.continuation_func_ids` で除外、`MAIN` 明示除外
- 検証: Pattern6 RC=9 / quick 154 PASS / 恒常ログ増加なし
- 入口: `docs/development/current/main/phases/phase-287/P0-BIGFILES-REFACTORING-INSTRUCTIONS.md`

**2025-12-27: Phase 287 P1 完了** ✅
- `ast_feature_extractor.rs` を facade 化（1,148 → 135 lines）
- `pattern_recognizers/`（8 modules）へ分割（1 module = 1 質問）
- 検証: Build 0 errors / Pattern6 RC=9 / quick 154 PASS / 恒常ログ増加なし
- 入口: `docs/development/current/main/phases/phase-287/P1-AST_FEATURE_EXTRACTOR-INSTRUCTIONS.md`
- 次の指示書（P2）: `docs/development/current/main/phases/phase-287/P2-CONTRACT_CHECKS-MODULARIZATION-INSTRUCTIONS.md`（完了）

**2025-12-27: Phase 287 P2 完了** ✅
- `contract_checks.rs` を facade 化し、`contract_checks/` 配下へ契約単位で分割（1 module = 1 contract）
- 検証: Build 0 errors / Pattern6 RC=9 / quick 154 PASS / 恒常ログ増加なし
- 入口: `docs/development/current/main/phases/phase-287/P2-CONTRACT_CHECKS-MODULARIZATION-INSTRUCTIONS.md`
- 次の指示書（P3）: `docs/development/current/main/phases/phase-287/P3-INSTRUCTION_REWRITER-MODULARIZATION-INSTRUCTIONS.md`（完了）

**2025-12-27: Phase 287 P3 完了** ✅
- `instruction_rewriter.rs` を stage 単位へ物理分割（Scan/Plan/Apply）し、facade（orchestrator）へ縮退（意味論不変）
- 検証: Build 0 errors / quick 154 PASS / 恒常ログ増加なし
- 入口: `docs/development/current/main/phases/phase-287/P3-INSTRUCTION_REWRITER-MODULARIZATION-INSTRUCTIONS.md`
- 次の指示書（P4）: `docs/development/current/main/phases/phase-287/P4-PLAN_STAGE-MODULARIZATION-INSTRUCTIONS.md`（完了）

**2025-12-27: Phase 287 P4 完了** ✅
- `rewriter/stages/plan.rs` を facade 化し、`rewriter/stages/plan/` 配下へ責務単位で分割（意味論不変）
- 検証: Build 0 errors / quick 154 PASS / 恒常ログ増加なし
- 入口: `docs/development/current/main/phases/phase-287/P4-PLAN_STAGE-MODULARIZATION-INSTRUCTIONS.md`
- 次の指示書（P5）: `docs/development/current/main/phases/phase-287/P5-STAGES-VISIBILITY-FACADE-INSTRUCTIONS.md`（完了）

**2025-12-27: Phase 287 P5 完了** ✅
- `stages/mod.rs` を facade にして stage 関数を re-export（単一入口化）
- 検証: Build 0 errors / quick 154 PASS / 恒常ログ増加なし
- 入口: `docs/development/current/main/phases/phase-287/P5-STAGES-VISIBILITY-FACADE-INSTRUCTIONS.md`
- 次の指示書（P6）: `docs/development/current/main/phases/phase-287/P6-SCAN_PLAN-INTEGRATION-INSTRUCTIONS.md`（完了）

**2025-12-27: Phase 287 P6 完了** ✅
- Scan stage を削除し、pipeline を 2-stage（Plan→Apply）へ単純化（意味論不変）
- 検証: quick 154 PASS
- 入口: `docs/development/current/main/phases/phase-287/P6-SCAN_PLAN-INTEGRATION-INSTRUCTIONS.md`
- 次の指示書（P7）: `docs/development/current/main/phases/phase-287/P7-REWRITER-BOX-SCAFFOLDING-CLEANUP-INSTRUCTIONS.md`（完了）

**2025-12-27: Phase 287 P7 完了** ✅
- `rewriter/` の未使用 Box 雛形を削除し、SSOT を `rewriter/stages/*` に寄せた（意味論不変）
- 検証: Build 0 errors / quick 154 PASS / 恒常ログ増加なし
- 入口: `docs/development/current/main/phases/phase-287/P7-REWRITER-BOX-SCAFFOLDING-CLEANUP-INSTRUCTIONS.md`
- 次の指示書（P8）: `docs/development/current/main/phases/phase-287/P8-REWRITER-README-GUARD-INSTRUCTIONS.md`

**2025-12-27: Phase 287 P8 完了** ✅
- `rewriter/README.md` を追加し、責務境界と SSOT（Plan→Apply）を明文化（docs-only）
- 検証: cargo check / quick PASS
- 入口: `src/mir/builder/control_flow/joinir/merge/rewriter/README.md`
- 次の指示書（P9）: `docs/development/current/main/phases/phase-287/P9-PHASE-CLOSEOUT-INSTRUCTIONS.md`

**2025-12-27: Phase 188.2 完了** ✅
- StepTreeの `max_loop_depth` を SSOT に採用（Option A）
- strict mode で depth > 2 を明示エラー化（Fail-Fast）
- quick 154/154 PASS、integration selfhost FAIL=0 維持
- 次: `docs/development/current/main/phases/phase-188.3/P2-REFACTORING-INSTRUCTIONS.md`（意味論不変での整理を優先）

**2025-12-27: Phase S0.1 完了** ✅
- integration selfhost を「落ちない状態」に収束（FAIL=0）
- canary テスト opt-in 化（SMOKES_ENABLE_SELFHOST=1、9本）
- baseline テスト条件付き SKIP（該当ログの時だけ、unknown は FAIL 維持）
- quick 154/154 PASS 維持、Fail-Fast 原則遵守
- SSOT: [selfhost-integration-limitations.md](investigations/selfhost-integration-limitations.md)

**2025-12-27: Phase S0 Selfhost Integration 安定化（実ログベース）完了** ✅
- **実ログ採取完了**: 5本のスクリプト直叩き + 全体ログで事実確定（推定排除）
- **確認済みエラーパターン**（実ログベース）:
  - Pattern 1: Loop lowering failed / StepTree lowering returned None（JoinIR loop pattern gap）
  - Pattern 2: cap_missing/NestedLoop（JoinIR caps gap）
  - Pattern 3: strict mode panic（NYASH_JOINIR_STRICT=1 削除で対応）
  - Pattern 4: Argument list too long（OS limitation）
- **条件付き SKIP 実装**: selfhost_minimal.sh, phase150, mir_min_vm, canary 4本（該当ログの時だけ、それ以外はFAIL）
- **進捗**: 12 FAIL → 10 FAIL（selfhost_minimal, mir_min_vm が PASS に）
- **quick smoke**: 154/154 PASS 維持 ✅
- **SSOT**: [selfhost-integration-limitations.md](investigations/selfhost-integration-limitations.md)
- **未確認**: String+Integer 型エラー（実ログで未検出、追加しない）

**2025-12-27: Phase 29y.1（pilot plumbing）完了** ✅
- docs SSOT（ABI/RC insertion/observability）を Phase 29y に集約
- "後続実装へ迷わず切るための最小導線" を追加（意味論は変更しない）
  - NyRT handle ABI shim（`crates/nyash_kernel/src/ffi/lifecycle.rs`）
  - RC insertion pass 入口（no-op skeleton）
  - leak report に root categories（handlesのみ）を追加（Phase 1 limitation 明記）
- integration smokes 追加（Phase 29y.1）: `phase29y_handle_abi_{vm,llvm}.sh`
- quick smoke 154/154 PASS 維持

**2025-12-26: Phase 285 P2 完了** ✅
- weak の意味論（`weak <expr>` + `weak_to_strong()` 成功）を integration smoke で固定
- Fixture A（成功パターン）: exit 2 で明確化、VM/LLVM PASS
- Fixture B（失敗パターン）: 明示 drop (`x = null`) 方式
- Block scope drop conformance は別タスク（P2 では扱わない）
- quick smoke 154/154 PASS 維持

**2025-12-26: Phase 285 P2.1 完了** ✅
- hidden root を根治し、weak-fail fixture を PASS に復帰（`x = null` 後に `weak_to_strong()` が `null`）
- `KeepAlive` / `ReleaseStrong` により「スコープ維持 / 上書きdrop」を語彙として分離（後続の命令分離で整理）

**2025-12-26: Phase 285 P2.2（hygiene）完了** ✅
- `KeepAlive { values, drop_after }` を廃止し、`KeepAlive` / `ReleaseStrong` に命令分離（意図を MIR 語彙で明確化）
- quick smoke 154/154 PASS 維持

**2025-12-27: Phase 285 P4 Post-Completion** ✅
- **Phase 284 P2 Integration Fix** (commit `225600b5f`)
  - `phase284_p2_return_in_loop_llvm` FAIL 修正（unreachable instruction 使用）
  - Integration LLVM tests: 3/3 PASS（FAIL 残りなし）
- **ret.py Box-First Refactoring** (commits `32aa0ddf6`, `5a88c4eb2`)
  - Phase 1-2: UnreachableReturnHandlerBox + ReturnTypeAdjusterBox
  - Phase 3: StringBoxerBox + ReturnPhiSynthesizerBox
  - lower_return() 削減: 166→117 lines (-29%)
- **Code Quality Improvements** (commits `d7c6df367`, `798c193cb`, `1869396fd`, `095213c58`)
  - LLVM exit code SSOT化、nyash_kernel FFI分割、LLVM検出集約、auto_detect.conf明確化
- quick smoke 154/154 PASS 維持

**2025-12-26: Phase 285 P0 完了** ✅
- 言語 SSOT との境界明文化（lifecycle.md, types.md）
- 用語・禁止事項・VM/LLVM差分分類を固定
- P1/P2 への導線を箇条書きで追加

**2025-12-26: Phase 284 P2 完了** ✅
- return を含む loop（Pattern5）を VM で smoke 固定
- LLVM harness integration でも PASS（unreachable return の型不整合を修正）
- quick smoke 154/154 PASS 維持

設計相談（将来の正規化）:
- `docs/development/current/main/investigations/phase-286-plan-normalization-consult.md`

**2025-12-26: Phase 286 P2 完了** ✅
- Pattern4 (Loop with Continue) → Plan/Frag SSOT 化 PoC 成功
- phi_bindings 導入で PHI dst 参照を正しく解決
- Integration test PASS (output: 6), quick smoke 154/154 PASS

**2025-12-26: Phase 286 P2.1 完了** ✅
- Pattern1 (SimpleWhile) → Plan/Frag SSOT 化 PoC 成功
- Integration test PASS (return: 3), quick smoke 154/154 PASS
- Plan line routing: `route=plan strategy=extract pattern=Pattern1_SimpleWhile`

**2025-12-26: Phase 286 P2.2 完了** ✅
- extractor helper化: `extract_loop_increment_plan` → `common_helpers.rs` に統一（重複排除 ~25行）
- router helper化: `lower_via_plan()` 追加で Pattern6/7/4/1 の boilerplate 削減（~40行）
- quick smoke 154/154 PASS 維持、Pattern1/4 PoC 両方 PASS

**次のステップ**:
1. **Phase 287（P9）**: Phase closeout（docs-only）
   - 指示書: `docs/development/current/main/phases/phase-287/P9-PHASE-CLOSEOUT-INSTRUCTIONS.md`
2. （post self-host / docs-first）**Phase 29y**: MIR lifecycle vocab freeze（RC/weak/ABI）
   - 相談パケット: `docs/development/current/main/investigations/phase-29y-mir-lifecycle-vocab-consult.md`
3. （future design, separate phase）Plan 生成の正規化（相談パケット）
   - `docs/development/current/main/investigations/phase-286-plan-normalization-consult.md`

## Recently Completed

### 2025-12-25: Phase 288.1（REPL Session Persistence + Auto-Display）

- ✅ **Phase 288.1完了**: REPL variable persistence + expression auto-display
  - 詳細: `docs/development/current/main/phases/phase-288/README.md`
  - 実装方式: AST Rewrite + ExternCall Bridge
  - 達成内容:
    - ✅ AST Rewriter 実装（~430行、`src/runner/repl/ast_rewriter.rs`）
    - ✅ __repl.get/set ExternCall handlers 実装
    - ✅ Rc<RefCell<ReplSessionBox>> session sharing
    - ✅ Expression auto-display + `_` variable
    - ✅ Fail-fast undefined variable errors
  - 検証結果:
    - ✅ Variable persistence: `x = 42` then `print(x)` → `42`
    - ✅ Expression display: `1 + 1` → `2`
    - ✅ `_` variable: `10 * 2` → `20`, then `_` → `20`
    - ✅ Session reset: `.reset` clears all variables
    - ✅ 154/154 smoke tests PASS (no regressions)
  - Files: 8 files, +592 lines (REPL-isolated)

### 2025-12-24: Phase 285LLVM-1.3（LLVM InstanceBox Field Access）

- ✅ **Phase 285LLVM-1.3完了**: InstanceBox field access (getField/setField) implementation
  - 詳細: `docs/development/current/main/phases/phase-285/phase-285llvm-1.3-verification-report.md`
  - 実装: `crates/nyash_kernel/src/plugin/invoke.rs` (~170 lines)
  - 達成内容:
    - ✅ getField/setField handlers 実装完了（SSOT `fields_ng` 直接アクセス）
    - ✅ Fail-Fast error logging 実装（`[llvm/invoke/{get,set}Field]` tags）
    - ✅ Raw i64 fallback 対応（LLVM backend 特有の挙動）
    - ✅ Handle resolution 動作確認（handle 4 → IntegerBox(42)）
  - 検証結果:
    - ✅ setField: Integer(42) を正しく保存
    - ✅ getField: Integer(42) を正しく取得、handle 返却
    - ✅ print issue: Phase 285LLVM-1.4（型タグ伝播）で解決済み

### 2025-12-23

- Phase 283（bugfix）: JoinIR if-condition remap fix: `docs/development/current/main/phases/phase-283/README.md`
- Phase 282（Router shrinkage + extraction-based migration + extractor refactor P0–P9a）: `docs/development/current/main/phases/phase-282/README.md`
- Phase 280（Frag composition SSOT positioning）: `docs/development/current/main/phases/phase-280/README.md`
- Phase 281（Pattern6/7 compose adoption, P0–P3）: `docs/development/current/main/phases/phase-281/README.md`
- Phase 273（Plan line SSOT）: `docs/development/current/main/phases/phase-273/README.md`
- Phase 275 P0（A1/B2/C2 coercion SSOT）: `docs/development/current/main/phases/phase-275/README.md`
- Phase 276 P0（quick wins / type_helper SSOT）: `docs/development/current/main/phases/phase-276/README.md`
- Phase 277 P1（PHI strict fail-fast）: `docs/development/current/main/phases/phase-277/README.md`
- Phase 277 P2（PHI env var 統合）: `docs/development/current/main/phases/phase-277/README.md`
- Phase 278 P0（deprecated PHI env vars removal）: `docs/development/current/main/phases/phase-278/README.md`
- Phase 279 P0（type propagation pipeline SSOT unification）: `docs/development/current/main/phases/phase-279/README.md`

---

## 2025-12-22: Follow-ups（post Phase 275/276/277）

- 目的: Phase 275/276 で積み残した改善タスクを完全実装（デッドコード削除・SSOT使用推進・Void検出）
- 達成内容:
  - ✅ **P0: box_from_f64 削除**（デッドコード）
    - `crates/nyash_kernel/src/lib.rs` から2関数削除
    - Phase 275 P0 の Float 型 SSOT 方針により boxing helper 不要
    - ビルド成功・テスト成功確認
  - ✅ **P1: dst_type_to_llvm_type 使用推進**
    - `phi_wiring/wiring.py` を type_helper.py SSOT に統一
    - 型変換ロジックが完全に1箇所に集約（拡張性向上）
  - ✅ **LLVM A1: Void検出修正（Phase 275 P0 残タスク）**
    - `branch.py` に Void/VoidBox 検出ロジック実装済みを確認
    - エラーメッセージ追加（fail-fast 原則）
    - テストケース作成（/tmp/test_p275_a1_void.hako）
    - VM側で正常にエラー検出確認
  - ✅ **LLVM Smoke Tests 完全実施**:
    - Test 1 (simple Int+Float): ✅ PASS (exit=3, VM/LLVM parity)
    - Test 2 (two Int+Float ops): ✅ PASS (exit=3, VM/LLVM parity)
    - Test 3 (Float + String): C2 では **TypeError** が期待（文字列混在 `+` は禁止）。ここが通るならバグとして扱う
- 効果:
  - Float PHI 完全動作（VM/LLVM parity 達成）
  - SSOT 原則完全適用（型変換・環境変数）
  - Fail-Fast 原則適用（Void in boolean context）
  - デッドコード削減（保守性向上）

## 2025-12-22: Phase 277（P2）— PHI関連環境変数の統合・整理 ✅

- 目的: PHI関連環境変数を **8個 → 3個** に統合してユーザビリティ向上・保守性向上
- 完了日: 2025-12-22
- 達成内容:
  - ✅ `debug_helper.py` 作成（環境変数チェックのSSOT）
  - ✅ 3つの統合関数実装:
    - `is_phi_debug_enabled()`: 一般デバッグ（LLVM_PHI_DEBUG + PHI_TYPE_DEBUG + PHI_ORDERING_DEBUG 統合）
    - `is_phi_trace_enabled()`: 詳細トレース（LLVM_TRACE_PHI + LLVM_VMAP_TRACE 統合）
    - `is_phi_strict_enabled()`: 厳格モード（既存維持）
  - ✅ 全PHI関連ファイル修正完了（9ファイル）:
    - `phi_wiring/wiring.py`, `phi_wiring/tagging.py`, `phi_wiring/common.py`
    - `phi_placement.py`, `trace.py`, `instructions/phi.py`
    - `resolver.py`, `utils/values.py`, `builders/block_lower.py` 他
  - ✅ 後方互換性対応（非推奨警告付き、Phase 278で削除予定）
  - ✅ ドキュメント更新:
    - `docs/reference/environment-variables.md` に詳細セクション追加
    - 使用例・出力例・移行ガイド記載
- 効果:
  - ユーザビリティ向上（覚える変数 8個→3個）
  - ドキュメント簡潔化（環境変数セクションが短く）
  - 保守性向上（関連設定が1つに）
  - SSOT原則適用（環境変数チェックロジック統一）

## 2025-12-22: Phase 276（P0）— Quick Win 改善（型取得SSOT化） ✅

- 目的: Phase 275 P0 完了後の堅牢性改善（デバッグコード削減・型取得ロジックSSOT化・警告強化）
- 完了ドキュメント: `docs/development/current/main/phases/phase-276/P0-COMPLETION.md`
- 達成内容:
  - ✅ デバッグスタックトレース削除（wiring.py）
  - ✅ box_from_f64 使用確認（削除可能と判断）
  - ✅ 型取得ロジックSSOT化（type_helper.py 作成、3ファイル統一）
  - ✅ 型不一致CRITICAL警告強化（PhiManager連携）
- 効果:
  - 型取得ロジックの重複削除（3箇所 → 1箇所）
  - SSOT原則適用（バグ防止・拡張性向上）
  - デバッグ性向上（CRITICAL警告で早期発見）

## 2025-12-22：Phase 275（P0）— coercion SSOT rollout（truthiness / `==` / `+`） ✅

- 完了: Phase 274 P3 で確定した coercion ルール（A1/B2/C2）を VM/LLVM に実装
- Decision (SSOT): `docs/development/current/main/phases/phase-274/P3-DECISIONS.md`
- 実装ガイド: `docs/development/current/main/phases/phase-275/P0-INSTRUCTIONS.md`
- 重要修正:
  - Float型PHI完全対応（MIR型伝播 → LLVM IR double生成）
  - 型取得ロジック3箇所統一（type_helper.py）
  - 型不一致警告強化（CRITICAL表示）
- fixture/smoke:
  - `apps/tests/phase275_p0_float_phi_min.hako` (想定)
  - LLVM harness で exit=3 動作確認済み

### 過去の Blocker: 型伝播パイプラインの二重化（lifecycle vs JoinIR）

- 現状、型伝播/PHI 型解決の順序が経路により異なり、同一 fixture が別ルートで壊れ得る（実質 "2本のコンパイラ"）。
- 対処（SSOT, short-term）: Phase 276 P0 で型取得ロジックをSSOT化（部分対応）
- 根治（SSOT, long-term）: Phase 279 で type propagation pipeline の入口/順序を完全統一
- 予定: `docs/development/current/main/phases/phase-279/README.md`

## 2025-12-22：Phase 274（P1）— TypeOp（is/as）を Rust VM で実行可能にする ✅

- 完了: Rust VM が `MirInstruction::TypeOp`（Check/Cast）を実行可能
- fixture/smoke:
  - `apps/tests/phase274_p1_typeop_is_as_min.hako`
  - `tools/smokes/v2/profiles/integration/apps/archive/phase274_p1_typeop_is_as_vm.sh`

## 2025-12-22：Phase 274（P2）— LLVM TypeOp alignment ✅

- 完了: LLVM harness でも `TypeOp(Check/Cast)` が SSOT（Rust VM）と一致
- 重要修正: MIR JSON emitter（bin）で `typeop` が欠落していたため、JSON に `op:"typeop"` を出力するよう修正
- fixture/smoke（LLVM）:
  - `apps/tests/phase274_p2_typeop_primitives_only.hako`
  - `tools/smokes/v2/profiles/integration/apps/archive/phase274_p2_typeop_is_as_llvm.sh`

## 2025-12-22：Phase 272（P0.2）— Pattern7 Frag+emit_frag 移行 ✅

- 目的: Pattern7（split scan）を `Frag + emit_frag()` 経路へ移行し、terminator emission を SSOT に集約する（副作用 `result.push` を含む）
- 状況: Phase 272 P0.1（Pattern6）+ P0.2（Pattern7）ともに ✅ 完了
- 入口 fixture/smoke:
  - Pattern7: `apps/tests/phase256_p0_split_min.hako` + `tools/smokes/v2/profiles/integration/apps/archive/phase256_p0_split_vm.sh`
- SSOT: `docs/development/current/main/design/edgecfg-fragments.md`（合成則/bridge撤去条件）
- 詳細: `docs/development/current/main/phases/phase-272/README.md`

## 2025-12-22：Phase 271（docs-only）— Bridge pattern 撤去条件SSOT ✅

- 変更:
  - `docs/development/current/main/design/edgecfg-fragments.md` に bridge contract（テンプレ）+ `Pattern9_AccumConstLoop` 撤去条件を追記
  - `docs/development/current/main/30-Backlog.md` の Phase 271 成果物を明文化

## 2025-12-22：Phase 269 P1.2（this/me in loop）— Static Call 正規化SSOT ✅

- 目的: static box 内の `this.method(...)` / `me.method(...)` を runtime receiver にせず、compile-time に static call へ正規化する（NewBox 禁止 / by-name ハードコード禁止）
- SSOT: `comp_ctx.current_static_box` と `BoxName.method/arity`（canonical key）
- 実装: MethodCall 共通入口で `This/Me` receiver を最優先で検出し、static call へ正規化（ハードコード無し）
- fixture/smoke:
  - `apps/tests/phase269_p1_2_this_method_in_loop_min.hako`
  - `tools/smokes/v2/profiles/integration/apps/archive/phase269_p1_2_this_method_in_loop_vm.sh`
- 受け入れ: MIR dump に receiver `const "StringUtils"` が出ない / `call_method StringUtils.is_digit/1`（同等の static call）になる

## 2025-12-22：Phase 269 P1（Pattern8 EdgeCFG lowering）— SSA を閉じる ✅

- 完了: header に `i_current = phi [i_init, preheader], [i_next, step_bb]` を入れて SSA を閉じ、header/body/step の参照を `i_current` に統一
- 検証: `tools/smokes/v2/profiles/integration/apps/archive/phase269_p0_pattern8_frag_vm.sh` PASS（+ 回帰 `phase259_p0_is_integer_vm` PASS）
- 詳細: `docs/development/current/main/phases/phase-269/README.md`

## 2025-12-21：Phase 270（P0+P1）— JoinIR-only minimal loop SSOT ✅

- 目的: `loop(i < 3)` + `sum=sum+i` + `i=i+1` を JoinIR 経路で通すことを fixture/smoke で固定
- 結果: Pattern1 は test-only stub のため不適合 → Pattern9（AccumConstLoop）を橋渡しとして追加し、fixture は exit=3 で PASS
- 制約: `cf_loop` は JoinIR-only（非JoinIR loop 経路や env-var 分岐は追加しない）
- 詳細: `docs/development/current/main/phases/phase-270/README.md`

## 2025-12-21：Phase 269 P1（Pattern8 EdgeCFG lowering）✅

**目的**: Pattern8（BoolPredicateScan）を JoinIR ではなく **EdgeCFG Frag + emit_frag()** で “本当に”動かす（層境界は維持）
**スコープ**: Pattern8 内だけ差し替え（merge/EdgeCFG plumbing/Pattern6/7/9 は触らない、cf_loop hard-freeze維持）

**完了内容（P1）**:
- ✅ emission 入口 `loop_predicate_scan` を追加し、Frag 構築 + `emit_frag()` を配線
- ✅ 5ブロック構成（header/body/step/after/ret_false）で terminator を生成
- ✅ header に PHI を挿入して `i` の SSA を閉じた（`i_current`）
- ✅ early-exit `return false` は Return wire、`return true` は loop 後 AST に任せる
- ✅ Pattern8 lower は当面 `emit_void(builder)`（loop-statement 扱い）

**詳細**: `docs/development/current/main/phases/phase-269/README.md`

## 2025-12-21：Phase 269 P1.1（call_method return type）— 署名SSOTで型注釈 ✅

- 問題: `call_method BoxName.method/N(...)` の戻り値型が既定 `String` になり、Bool を返すメソッドが誤動作する
- 修正方針: `emit_unified_call_impl` の直後で、`BoxName.method/arity` の canonical key を構築し、
  `MirFunction.signature.return_type`（SSOT）から dst の型を注釈する（ハードコード禁止）

## 2025-12-21：Phase 267 P0（BranchStub + emit_frag）✅

**目的**: Frag に Branch を第一級で追加し、wires（Jump/Return）と同様に MIR terminator へ落とせる入口（SSOT）を作る  
**結果**: `emit_frag()` により、`wires + branches` を BasicBlockId 層で PoC 証明（unit tests）まで完了

- ✅ `BranchStub` 追加 + `Frag.branches` 追加
- ✅ `compose::if_` が header→then/else の `BranchStub` を生成
- ✅ `emit_frag(function, frag)` 追加（`verify_frag_invariants_strict` を先頭で実行、1 block = 1 terminator を Fail-Fast）
- ✅ `cargo test -p nyash-rust --lib` PASS

**注意（P1）**: NormalizedShadow/JoinIR への実適用は層ミスマッチがあるため Phase 268 に繰り越し
**詳細**: `docs/development/current/main/phases/phase-267/README.md`

## 2025-12-21：Phase 268 P1（compose::if_ entry edge-args SSOT化）✅

**目的**: compose::if_() の then/else entry edge-args を呼び出し側 SSOT にし、TODO 削除（Phase 267 P2+ からの継続）

**実装完了内容**:
- ✅ compose::if_() シグネチャ変更（then_entry_args, else_entry_args パラメータ追加）
- ✅ emission/branch.rs::emit_conditional_edgecfg() から空 EdgeArgs を then/else 両方に渡す
- ✅ EdgeCFG テスト更新（compose.rs 2箇所、emit.rs 1箇所）
- ✅ TODO コメント削除完了（Phase 267 P2+ TODO 解消）

**テスト結果**:
- ✅ cargo build --release: **成功**（0エラー）
- ✅ cargo test --lib --release: **1444/1444 PASS**
- ✅ quick smoke: **45/46 PASS**（既存状態維持）

**核心的な設計判断**:
1. **SSOT 原則**: compose::if_() 内部で then/else entry edge-args を "勝手に空 Vec で生成" しない → 呼び出し側が明示的に渡す
2. **P0 との整合性**: P0 で emission/branch.rs に薄いラッパーを作ったので、edge-args も同じ層で SSOT として渡す

**次フェーズへの橋渡し**:
- Phase 269: Pattern6/7/8 への Frag 適用 + fixture/smoke test

**詳細**: `docs/development/current/main/phases/phase-268/README.md`

## 2025-12-21：Phase 266（wires → MIR terminator 生成 - 最小 PoC）✅

**目的**: wires を MIR terminator に変換する最小 PoC を実装し、Phase 267 での本格適用に備える

**実装完了内容**:
- ✅ emit.rs 作成（emit_wires 実装 + unit test 4個）
  - from ごとにグループ化して1本だけ許可（1 block = 1 terminator 制約）
  - Return は target=None を許可（意味を持たない）
  - Jump/Return 対応（Branch は Phase 267）
- ✅ verify_frag_invariants_strict() 追加（段階導入を壊さない）
  - 既存の verify_frag_invariants() は変更なし（警告のまま）
  - wires/exits 分離契約を Err 化（Return の target=None は許可）
- ✅ mod.rs 更新（emit module エクスポート）

**テスト結果**:
- ✅ edgecfg::api::emit テスト: **4/4 PASS**
  - test_emit_wires_jump_basic
  - test_emit_wires_return_basic
  - test_emit_wires_unwired_stub_fails
  - test_emit_wires_multiple_from_same_block_fails
- ✅ 全 lib テスト: **1392/1392 PASS**（既存 1388 + 新規 4個、退行なし）

**核心的な設計判断**:
1. **from グループ化**: 同じ block に複数 terminator を設定すると上書き → BTreeMap で from ごとにグループ化し、1本だけ許可
2. **Return の target=None 許可**: Return は呼び出し元に戻るので target が意味を持たない → Normal/Break/Continue/Unwind のみ target 必須
3. **verify strict 版**: 既存の verify_frag_invariants() を Err 化すると既存コードが壊れる → 新規に strict 版を追加し、段階導入
4. **Phase 260 terminator 語彙ルール厳守**: Jump は set_jump_with_edge_args()、Return は set_terminator() + set_return_env()

**重要**: JoinIR/NormalizedShadow には触らない（Phase 267）。Branch terminator 生成も Phase 267 に繰り越し。

**次フェーズへの橋渡し**:
- Phase 267: NormalizedShadow への Frag 適用 + Pattern6/7/8 を Frag 化 + Branch 生成

**詳細**: `docs/development/current/main/design/edgecfg-fragments.md`

## 2025-12-21：Phase 265 P2（seq/if_ 実装 - wires/exits 分離）✅

**目的**: 「解決済み配線（wires）」と「未解決 exit（exits）」を分離し、Frag 合成の基本パターンを完成させる

**実装完了内容**:
- ✅ Frag に `wires: Vec<EdgeStub>` フィールド追加
- ✅ wires/exits 分離設計確立
  - **exits**: target = None のみ（未配線、外へ出る exit）
  - **wires**: target = Some(...) のみ（配線済み、内部配線）
- ✅ loop_() を wires 対応に更新（Break/Continue → wires）
- ✅ seq(a, b) 実装完了（a.Normal → b.entry が wires、seq の Normal exit は b の Normal）
- ✅ if_(header, cond, t, e, join_frag) 実装完了（t/e.Normal → join が wires、if の exit は join_frag.exits）
- ✅ verify_frag_invariants() に wires/exits 分離契約追加（警告のみ、Err化は Phase 266）

**テスト結果**:
- ✅ edgecfg::api テスト: **13/13 PASS**（frag 3個 + compose 9個 + verify 1個）
- ✅ 全 lib テスト: **1388/1388 PASS**（退行なし）

**核心的な設計判断**:
1. **wires/exits 分離**: 解決済み配線と未解決 exit を混ぜると再配線バグが起きる → 分離して不変条件強化
2. **if_ は join_frag 受け取り**: join: BasicBlockId では「join block」か「join 以降」か曖昧 → join_frag: Frag で明確化
3. **verify は警告のみ**: P2 は wires/exits 分離の証明に集中、Err 化は Phase 266 で MIR 生成時に実施

**重要**:
- MIR 命令生成の **PoC（emit_wires）**は Phase 266 で完了。
- Pattern6/7/8 や NormalizedShadow への **実適用**は Phase 267 以降（層境界維持）。

**次フェーズへの橋渡し**:
- Phase 266: wires を MIR terminator に落とす（PoC: emit_wires）✅
- Phase 267: NormalizedShadow/JoinIR への適用 + Pattern6/7/8 を Frag 化 + Branch 生成

**詳細**: `docs/development/current/main/phases/phase-265/` + `docs/development/current/main/design/edgecfg-fragments.md`

## 2025-12-21：Phase 263 P0.2（Pattern2 promotion API SSOT）✅

- **Goal**: Pattern2 の “Reject/continue/fallback の揺れ” を **型 + 入口SSOT**で封じ、部分続行（後段で落ちる）を構造で不可能にする
- **実装**:
  - `PromoteDecision::{Promoted, NotApplicable, Freeze}`（Option 多重を撤去）
  - `pattern2/api/` を入口SSOTとして新設し、`try_promote(...)` を **単一参照点**に固定
- **効果**:
  - `NotApplicable` は **必ず** `Ok(None)` で Pattern2 全体を抜ける（後続経路へ）
  - `Freeze` は **必ず** Fail-Fast（close-but-unsupported のみ即死）
- **検証結果**:
  - cargo test --lib: **1368/1368 PASS** ✅
  - quick smoke: **45/46 PASS** ✅（既知 1 件は別論点）
- **Commits**:
  - `abdb860e7`（P0.1）: PromoteDecision 導入（Option 揺れの撤去）
  - `e17902a44`（P0.2）: `pattern2/api/` で入口SSOT物理固定
- **詳細**: `docs/development/current/main/phases/phase-263/README.md`

## 2025-12-21：Phase 264 P0（EdgeCFG Fragment 入口作成）✅

**目的**: Frag/ExitKind を一次概念にする入口APIを用意（実装置換は次フェーズ）

**完了内容**:
- 入口フォルダ作成: `src/mir/builder/control_flow/edgecfg/api/`
- コア型定義: `ExitKind`, `EdgeStub`, `Frag`
- 合成関数シグネチャ: `seq`, `if_`, `loop_`, `cleanup`（中身TODO）
- 最小テスト: 3個のユニットテスト追加
- ドキュメント連動: `edgecfg-fragments.md` に入口情報追記

**重要**: 既存実装（pattern6/7/8, merge/EdgeCFG）は未改変。
入口だけ固定し、適用は quick 復旧後の次フェーズで実施。

**次のステップ**:
- Phase 265: Pattern8 を Frag 合成に移行（最小適用）
- Phase 266: Pattern6/7 への展開（再利用確認）

**詳細**: `docs/development/current/main/phases/phase-264/README.md` + `docs/development/current/main/design/edgecfg-fragments.md`

## 2025-12-21：Phase 265 P0（compose/verify 最小実装）✅

**目的**: 入口SSOTを触って迷子防止（compose/verify の形を固める）

**完了内容**:
- `compose::loop_()` 最小実装（exit集合の分類のみ、配線はP1以降）
- `verify_frag_invariants()` 最小実装（デバッグガード付き）
- compose::loop_() のユニットテスト 2個追加

**重要**: Pattern8への適用はP0ではやらない（偽Fragを避ける）。
配線ロジックはP1で実装、Pattern8適用もP1から。

**次のステップ**:
- Phase 265 P1: 配線ロジック実装 + Pattern8適用
- Phase 265 P2: seq/if_ 実装（pattern番号分岐削減の見通し）

**詳細**: `docs/development/current/main/phases/phase-265/README.md`

## 2025-12-21：Phase 265 P1（compose 配線ロジック実装）✅

**目的**: Frag/ExitKind が BasicBlockId 層で配線できることを証明

**完了内容**:
- EdgeStub に `target: Option<BasicBlockId>` 追加
- compose::loop_() 配線ロジック実装（Continue → header, Break → after）
- verify_frag_invariants() 配線契約検証追加
- test-only PoC で実証完了（5個のテスト: 既存2個更新 + 新規3個追加）

**配線契約**:
- Continue(loop_id) の EdgeStub.target = Some(header)
- Break(loop_id) の EdgeStub.target = Some(after)
- Normal/Return/Unwind の EdgeStub.target = None（上位へ伝搬）

**重要**:
- MIR 命令生成はまだしない（Frag 層の配線能力証明に集中）
- NormalizedShadow/JoinIR層への適用は Phase 266 に繰り越し（層境界を守る）

**次のステップ**:
- Phase 265 P2: seq/if_ 実装（順次合成・条件分岐合成）
- Phase 266: JoinIR-VM Bridge 改修後、NormalizedShadow への適用

**詳細**: `docs/development/current/main/phases/phase-265/README.md`

## Next (planned)

- Phase 265（planned）: Pattern8 を Frag 合成に移行し、ExitKind+Frag の実装適用を開始（`compose::loop_` 実装）
- Phase 266（planned）: catch/cleanup / cleanup/defer / async を "exit-edge 正規化" で追加できる形へ（設計: `docs/development/current/main/design/exception-cleanup-async.md`）
- Phase 141 P2+: Call/MethodCall 対応（effects + typing を分離して段階投入、ANF を前提に順序固定）
- Phase 143-loopvocab P3+: 条件スコープ拡張（impure conditions 対応）
- 詳細: `docs/development/current/main/30-Backlog.md`

## Phase 260（大工事）ロードマップ（要約）

- P0: edge-args を MIR terminator operand として **併存導入**（Branch を含むので参照点は `out_edges()` 系に一本化）
- P1: terminator更新APIを一本化し、successors/preds 同期漏れを構造で潰す
- P2: `BasicBlock.jump_args` を削除（terminator operand を SSOT 化）
- P3: spans を `Vec<Spanned<_>>` に収束（段階導入）

## Phase 261 P0 チェックリスト（legacy-only 経路の棚卸し）

- `src/mir/basic_block.rs:119` — reason: legacy layout の初期値が None（空メタ）; expected: OK（legacy 未設定時の既定）
- `src/mir/basic_block.rs:239` — reason: out_edges が legacy fallback を持つ; expected: Jump/Branch で edge-args を書く経路を増やし、verify で legacy-only を検出
- `src/mir/basic_block.rs:254` — reason: legacy setter API; expected: writer 側は edge-args を併記する経路に統一
- `src/mir/basic_block.rs:281` — reason: legacy edge-args 生成 API; expected: read-side の移行完了後に削除（P2）
- `src/mir/basic_block.rs:337` — reason: legacy edge-args fallback; expected: verify で legacy-only を検出し、P2で撤去

## Current First FAIL (SSOT)

- **After Phase 260 P0.3**: `core_direct_array_oob_set_rc_vm / Stage-B compile / JoinIR Pattern2 LoopBodyLocal(seg)`

### FAIL Details (既知・Phase 260 scope外)

- **Test**: `core_direct_array_oob_set_rc_vm`
- **Status**: **既知 / Phase 260 scope外**（JoinIR Pattern2 LoopBodyLocal promotion 未実装）
- **Phase**: Stage‑B compile (`stageb_compile_to_json`)
- **Error**: `[cf_loop/pattern2] Cannot promote LoopBodyLocal variables ["seg"]: No promotable pattern detected (tried A-3 Trim, A-4 DigitPos); read-only-slot rejected: [joinir/freeze] [pattern2/body_local_slot/contract/not_readonly] 'seg' must be read-only (assignment detected in loop body)`
- **Expected**: Stage-B compile succeeds (bundle_resolver loop compiles)
- **Actual**: MIR compilation error (LoopBodyLocal "seg" cannot be promoted - A-3/A-4 patterns not detected)
- **Reproduce**:
  ```bash
  ./tools/smokes/v2/run.sh --profile quick
  # または
  bash tools/smokes/v2/profiles/quick/core/core_direct_array_oob_set_rc_vm.sh
  ```
- **Root Cause**: Pattern2 の LoopBodyLocal promotion（A-3 Trim / A-4 DigitPos）に依存しており、Stage‑B の bundle_resolver 系のループで露出している。
- **Next Action**: **Phase 260 完了**（P0.3まで完了）→ **今後は機能側のPhaseへ**（Pattern2 LoopBodyLocal promotion 機能実装）
- **分類**: JoinIR Pattern2 機能拡張（compiler機能側、CFG基盤整備は完了）

### 次の次（構造で迷子を潰す）

Phase 263（Pattern2 LoopBodyLocal “seg”）が片付いたら、Pattern2 の「Reject/continue/fallback の揺れ」を構造で潰す。

- ねらい: “Reject でも続行して後段で落ちる” を型/APIで不可能にする
- 方針（最小）:
  - `PromoteStepBox::try_promote(...) -> Result<PromoteDecision, String>`
  - `PromoteDecision::{Promoted, NotApplicable, Freeze}`
  - orchestrator が `NotApplicable` を受け取ったら **Pattern2 全体を `Ok(None)` で抜けて fallback**（SSOT）
  - “部分続行” を禁止（Fail-Fast/SSOTを維持）

## 2025-12-21：Phase 260 P2（BasicBlock.jump_args 完全削除）✅

- **EdgeCFG SSOT確立完了**: `BasicBlock.jump_args` フィールド削除、edge-args SSOTをterminator operand側に一本化
- **フィールド変更**: `jump_args` → `return_env`（Return専用metadata、terminator operandなし）
- **API簡略化**: legacy helper 8個削除、terminator更新API保持（successors同期漏れ防止）
- **Verification簡素化**: dual-source検証削除（cfg.rs 62行削除）
- **テスト結果**:
  - cargo test --lib: 1368 PASS
  - quick smoke: 45/46 PASS
  - phase258 tail call: PASS（Return env保持確認）
- **検証**: `rg 'jump_args' src/` = 0件（コメント除く）
- **修正ファイル**: 9ファイル（basic_block.rs、handlers 4件、verification、test）
- **詳細**: `docs/development/current/main/phases/phase-260/README.md`

## 2025-12-21：Phase 260 P0.2/P0.3（モジュール化大工事）✅

- **P0.2**: instruction_rewriter モジュール化
  - Commits: `cbed040a7`, `aa3fdf3c1`, `c2e8099ff`, `84cd653ae`, `875bfee1b`, `666de9d3e`
  - 抽出: exit_collection, block_allocator, merge_variable_handler, call_generator, handlers/（8ファイル）
- **P0.3**: joinir_block_converter モジュール化
  - Commits: `e7f9adcfe`, `2c01a7335`
  - Phase 1: terminator_builder, block_finalizer
  - Phase 2: handlers/（call, jump, conditional_method_call, if_merge, nested_if_merge）
  - **成果**: 15モジュール、約3941行、53単体テスト全てpass、45/46統合テストpass
- **SSOT維持確認**: jump_args直参照ゼロ、out_edges()/edge_args_to() SSOT維持
- **詳細**: `docs/development/current/main/phases/phase-260/README.md`

## 2025-12-20：Phase 260 P0/P0.1（edge-args Strangler）✅

- P0: MIR terminator の edge-args 併存導入（読む側 SSOT を `out_edges()`/`edge_args_to()` へ寄せた）
- P0.1: legacy layout の未設定を禁止（verify fail-fast）＋ terminator 直代入を `set_terminator*()` へ寄せた
- Commits:
  - P0: `4dfe3349b`
  - P0.1: `1fe5be347`

## 2025-12-21：Phase 259 P0（Pattern8 BoolPredicateScan）✅

- Phase 259 README: `docs/development/current/main/phases/phase-259/README.md`
- Result: `StringUtils.is_integer/1` を Pattern8（新規）で受理
- Fixtures:
  - `apps/tests/phase259_p0_is_integer_min.hako`（expected exit 7）
- Smokes:
  - `tools/smokes/v2/profiles/integration/apps/archive/phase259_p0_is_integer_vm.sh` ✅ PASS
  - `tools/smokes/v2/profiles/integration/apps/archive/phase259_p0_is_integer_llvm_exe.sh`（LLVM harness 要設定）
- Key Implementation:
  - `src/mir/builder/control_flow/joinir/patterns/pattern8_scan_bool_predicate.rs`（新規）
  - `src/mir/join_ir/lowering/scan_bool_predicate_minimal.rs`（新規）
- Design Decision: Pattern8 を新設（Pattern6 拡張ではなく分離）
  - Pattern6: "見つける" scan（返り値: Integer）
  - Pattern8: "全部検証する" predicate scan（返り値: Boolean）
- Note: json_lint_vm はまだ FAIL だが、is_integer 自体は解決済み。残りは nested-loop with break パターン（Pattern2 の別問題）

## 2025-12-20：Phase 258（index_of_string/2 dynamic window scan）✅

- Phase 258 README: `docs/development/current/main/phases/phase-258/README.md`
- Result: `index_of_string/2` を JoinIR で受理し、quick の first FAIL を `is_integer/1` へ進めた

## 2025-12-20：Phase 257（Pattern6 reverse scan + PHI/CFG stabilization）✅

- Phase 257 README: `docs/development/current/main/phases/phase-257/README.md`
- Result: `last_index_of/2` を Pattern6（reverse scan）で受理し、PHI predecessor mismatch を fail-fast + 自動補正で根治

## 2025-12-19：Phase 146/147 完了 ✅

- Phase 146 README: `docs/development/current/main/phases/phase-146/README.md`
- Fixtures:
  - `apps/tests/phase146_p0_if_cond_unified_min.hako`（P0: pure cond, expected exit 7）
  - `apps/tests/phase146_p1_if_cond_intrinsic_min.hako`（P1: `s.length() == 3`, expected exit 7）
- Smokes:
  - `tools/smokes/v2/profiles/integration/apps/archive/phase146_p0_if_cond_unified_vm.sh`
  - `tools/smokes/v2/profiles/integration/apps/archive/phase146_p0_if_cond_unified_llvm_exe.sh`
  - `tools/smokes/v2/profiles/integration/apps/archive/phase146_p1_if_cond_intrinsic_vm.sh`
  - `tools/smokes/v2/profiles/integration/apps/archive/phase146_p1_if_cond_intrinsic_llvm_exe.sh`
- Flags:
  - `HAKO_ANF_DEV=1`（dev-only: ANF routing 有効化）
  - `HAKO_ANF_ALLOW_PURE=1`（dev-only: PureOnly scope で ANF 有効化）

## 2025-12-19：Phase 251 Fix（JoinIR 条件変数抽出 / デバッグ出力整理）✅

- Phase 251 README: `docs/development/current/main/phases/phase-251/README.md`
- Fix:
  - `collect_variables_recursive()` を拡張し、`MethodCall/FieldAccess/Index/Call` の基底変数を ConditionEnv に登録できるようにした
  - `loop_with_if_phi_if_sum.rs` の無条件 `eprintln!` を `is_joinir_debug()` ガードに移した（デフォルトは clean output）
- Status:
  - 元の回帰（`arr.length()` の `arr` が ConditionEnv に入らない）: 解決
  - `--profile quick` の `json_lint_vm` は別件で失敗が残る（JoinIR Pattern2 の break 条件で `MethodCall` が未対応）

## 2025-12-19：Phase 252 P0/P1（Pattern2 break 条件: `this.methodcall`）✅

- Phase 252 README: `docs/development/current/main/phases/phase-252/README.md`
- Status:
  - `cargo check` は通過（0 errors）
  - `--profile quick` は次の FAIL が残る → Phase 253（`[joinir/mutable-acc-spec]`）

## 2025-12-19：Phase 255（Multi-param loop wiring）✅

- Phase 255 README: `docs/development/current/main/phases/phase-255/README.md`
- Status:
  - Pattern6/index_of が VM/LLVM で PASS
  - `loop_invariants` を導入して ConditionOnly 誤用を根治

## 2025-12-19：Phase 256（StringUtils.split/2 可変 step ループ）✅

- Phase 256 README: `docs/development/current/main/phases/phase-256/README.md`
- Status:
  - `StringUtils.split/2` は VM `--verify` / integration smoke まで PASS
  - `--profile quick` の最初の FAIL は Phase 257（`StringUtils.last_index_of/2`）へ移動
  - 設計SSOT: `docs/development/current/main/design/join-explicit-cfg-construction.md`
- 直近の主要fix:
  - `ValueId(57)` undefined は根治（原因は `const_1` 未初期化）。
  - SSA undef（`%49/%67`）は P1.7 で根治（continuation 関数名の SSOT 不一致）。
  - P1.8で ExitLine/jump_args の余剰許容と関数名マッピングを整流。
  - P1.9で `JoinInst::Jump` を tail call として bridge に落とし、`jump_args` を SSOT として保持。
  - P1.10で DCE が `jump_args` を used 扱いし、`instruction_spans` を同期（SPAN MISMATCH 根治）。
  - P1.11で ExitArgsCollector の expr_result slot 判定を明確化し、split が `--verify` / integration smoke まで PASS。
  - P1.5-DBG: boundary entry params の契約チェックを追加（VM実行前 fail-fast）。
  - P1.6: 契約チェックの薄い集約 `run_all_pipeline_checks()` を導入（pipeline の責務を縮退）。
  - P1.13: Pattern2 boundary entry params を `join_module.entry.params` SSOT へ寄せた（ValueId 推測生成の撤去）。
  - P1.13.5（= Phase 256.8.5）: Boundary SSOT 統一（Pattern4/6/7 横展開 + hardcoded ValueId/PARAM_MIN 撤去）。
  - Known issue（非ブロッカー）: Pattern7 integration smoke の `phi predecessor mismatch` は残存（今回の修正とは独立）。

## 2025-12-20：Phase 257（last_index_of early return loop）🔜

- Phase 257 README: `docs/development/current/main/phases/phase-257/README.md`
- Goal: `StringUtils.last_index_of/2` を JoinIR で受理し、`--profile quick` を緑に戻す
  - Investigation（最小再現/論点）: `docs/development/current/main/investigations/phase-257-last-index-of-loop-shape.md`
  - Status: Pattern6 reverse scan + PHI/CFG 安定化は完了（最初の FAIL は次へ移動）
  - Current first FAIL: `json_lint_vm / StringUtils.index_of_string/2`（dynamic window scan, unsupported）

## 2025-12-20：Phase 258（is_integer nested-if + loop）🔜

- Phase 258 README: `docs/development/current/main/phases/phase-258/README.md`
  - Status: `index_of_string/2` を対象（dynamic window scan）

## 2025-12-20：Phase 259（is_integer nested-if + loop）🔜

- Phase 259 README: `docs/development/current/main/phases/phase-259/README.md`

## 2025-12-19：Phase 254（index_of loop pattern）✅ 完了（Blocked by Phase 255）

- Phase 254 README: `docs/development/current/main/phases/phase-254/README.md`
- Status: **Pattern 6 実装完了、ただし実行失敗（Phase 255 で unblock）**
- 完了項目:
  - ✅ Pattern 6 DetectorBox 実装（`Pattern6_ScanWithInit MATCHED`）
  - ✅ extract_scan_with_init_parts() - 構造抽出
  - ✅ scan_with_init_minimal.rs - JoinIR lowerer（main/loop_step/k_exit 生成）
  - ✅ MirBuilder 統合 - boundary 構築と merge 実行
  - ✅ substring を BoxCall として init-time に emit
- ブロッカー:
  - JoinIR→MIR merge/boundary が複数ループ変数（s, ch, i）に未対応
  - PHI ノードが 1つしか作られず、undefined value エラー
- **Phase 254 の受け入れ境界**: Pattern 6 検出＋JoinIR 生成まで ✅
- **実行 PASS は Phase 255 の範囲**

## 2025-12-19：Phase 253（mutable-acc-spec）✅

- Phase 253 README: `docs/development/current/main/phases/phase-253/README.md`
- Goal: `--profile quick` を緑に戻す（対処療法なし、analyzer 契約の整理で直す）

## 2025-12-19：Phase 145-anf P0/P1/P2 完了 ✅

- SSOT docs:
  - `docs/development/current/main/phases/phase-145-anf/README.md`
  - `docs/development/current/main/phases/phase-144-anf/INSTRUCTIONS.md`
- 実装 SSOT:
  - `src/mir/control_tree/normalized_shadow/anf/`
  - 入口（接続箇所 SSOT）: `src/mir/control_tree/normalized_shadow/common/expr_lowerer_box.rs`
- 環境変数:
  - `HAKO_ANF_DEV=1`（dev-only: ANF 有効化）
  - `HAKO_ANF_STRICT=1`（dev-only: ANF fail-fast）
- Fixtures & smokes（VM/LLVM EXE parity）:
  - `apps/tests/phase145_p1_anf_length_min.hako` → exit 12
  - `apps/tests/phase145_p2_compound_expr_binop_min.hako` → exit 18
  - `apps/tests/phase145_p2_compound_expr_double_intrinsic_min.hako` → exit 5

## 2025-12-19：Phase 143-loopvocab P2 完了 ✅

- 対象: else 対称化（B-C / C-B）
- Fixtures & smokes（VM/LLVM EXE parity）:
  - `apps/tests/phase143_p2_loop_true_if_bc_min.hako` → exit 8
  - `apps/tests/phase143_p2_loop_true_if_cb_min.hako` → exit 9

## 2025-12-19：Phase 143 実行系契約修正 ✅

**問題**: normalized_helpers が env params を `ValueId(1,2...)` で割り当てていた（PHI Reserved 領域 0-99）。

**根本原因**: JoinValueSpace 契約では Param 領域は 100-999。4つの normalized shadow モジュールが全て間違った領域に params を割り当てていた。

**修正**:
- `NormalizedHelperBox::alloc_env_params_param_region()` 新規追加（100+ 開始）
- 4 ファイルの normalized shadow 生成を更新:
  - `loop_true_if_break_continue.rs`
  - `loop_true_break_once.rs`
  - `if_as_last_join_k.rs`
  - `post_if_post_k.rs`
- `instruction_rewriter.rs` 型ミスマッチ修正（`func.signature.params` → `func.params`）

**検証**:
- VM exit=7 ✅（phase143_loop_true_if_break_vm.sh PASS）
- LLVM EXE exit=7 ✅（phase143_loop_true_if_break_llvm_exe.sh PASS、timeout 解消）
- Unit tests: 69/69 PASS

**ValueId Space Contract (Phase 201)**:
| Region | Range | Purpose |
|--------|-------|---------|
| PHI Reserved | 0-99 | Loop header PHI dst |
| **Param** | **100-999** | **env params (flag, counter, etc.)** |
| Local | 1000+ | Const, BinOp, condition results |

## 2025-12-19：Phase 143.5 + P1 完了 ✅

**Phase 143.5: NormalizedHelperBox 箱化（リファクタリング）**
- 目的: 120+ 行のヘルパー関数重複を消去（4 ファイル共通化）
- 実装: `src/mir/control_tree/normalized_shadow/common/normalized_helpers.rs` (151行+6テスト)
- 効果: 136行追加 - 149行削除 = **-13行**（保守性大幅向上）
- 統計: 67/67 tests PASS（新規テスト 6個含む）
- 検証: cargo check 0 errors, cargo test 100% green

**Phase 143 P1: Continue Support（条件付きループ継続）**
- 目的: `loop(true) { if(cond_pure) continue }` パターン追加
- 実装: Loop-If-Exit contract enum 駆動（break/continue 弁別）
- 変更:
  - `extract_pattern_shape()`: Err-based pattern matching to LoopIfExitShape
  - `extract_exit_action()`: Break/Continue を enum variant として識別
  - `loop_cond_check`: match shape.then で Jump target を動的決定
- Fixtures & Tests:
  - `apps/tests/phase143_loop_true_if_continue_min.hako`
  - `tools/smokes/v2/profiles/integration/apps/archive/phase143_loop_true_if_continue_vm.sh`
  - `tools/smokes/v2/profiles/integration/apps/archive/phase143_loop_true_if_continue_llvm_exe.sh`
- 検証: 契約ベース（shape.validate_for_p1() で P1 制約チェック）
- 注記: Phase 131 Pattern matching Issue 既知（ルーティング層の pre-existing failure）

## 2025-12-19：Phase 143-loopvocab P0 完了 ✅

**Phase 143-loopvocab P0: Conditional Break Vocabulary Extension**
- 目的: `loop(true) { if(cond_pure) break }` パターンを Normalized shadow で実装（Phase 131 の条件付き拡張）
- 仕様:
  - Loop条件: `true` リテラルのみ
  - ループ本体: 単一 if statement（else なし）
  - If then: `break` のみ（no continue, no nested if）
  - 条件: pure expression のみ（変数/リテラル/算術/比較、Method call なし）
  - Out-of-scope は `Ok(None)` で graceful fallback
- 実装 SSOT:
  - `src/mir/control_tree/normalized_shadow/loop_true_if_break_continue.rs`
  - 6-function JoinModule（main → loop_step → loop_cond_check → Jump/Call → k_exit → Ret）
  - Jump: if true → k_exit, if false → fall through to Call(loop_step)
- Fixtures:
  - `apps/tests/phase143_loop_true_if_break_min.hako`（expected exit code 7）
- Smoke tests:
  - VM: `tools/smokes/v2/profiles/integration/apps/archive/phase143_loop_true_if_break_vm.sh` ✅ PASS
  - LLVM EXE: `tools/smokes/v2/profiles/integration/apps/archive/phase143_loop_true_if_break_llvm_exe.sh` ✅ PASS
- Regression: Phase 131-142 green（no regressions）
- 統計: +400 lines（loop_true_if_break_continue.rs）, 0 change to existing code
- 入口: `docs/development/current/main/phases/phase-143-loopvocab/README.md`

## 2025-12-19：Phase 142-loopstmt P0 完了 ✅

**Phase 142-loopstmt P0: Statement-Level Loop Normalization**
- 目的: 正規化単位を "block suffix" から "statement (loop 1個)" へ寄せてパターン爆発を防ぐ
- 変更:
  - PlanBox: loop(true) に対して常に loop_only() を返す（consumed=1）
  - SuffixRouter: LoopOnly を受け入れて実行
  - build_block: consumed 後も後続文を処理（break 削除）
- 実装 SSOT:
  - `src/mir/builder/control_flow/normalization/plan_box.rs`
  - `src/mir/builder/control_flow/joinir/patterns/policies/normalized_shadow_suffix_router_box.rs`
  - `src/mir/builder/stmts.rs`
- Refactoring:
  - LoopWithPost variant を deprecated（4 commits）
  - suffix_router コメント更新
  - README 更新
- Tests:
  - Fixture: `apps/tests/phase142_loop_stmt_only_then_return_length_min.hako`（exit code 3）
  - VM smoke: ✅ PASS
  - Unit tests: ✅ 10/10 passed
  - Regression: ✅ Phase 131, 141 green
- 統計: -38 lines net (code reduction success!)
- 入口: `docs/development/current/main/phases/phase-142-loopstmt/README.md`

⚠️ **Note**: Phase 142 (Canonicalizer Pattern Extension) とは別物。SSOT 衝突回避のため phase-142-loopstmt として独立管理。

## 2025-12-19：Phase 141 P1.5 完了 ✅

**Phase 141 P1.5: KnownIntrinsic registry + available_inputs 3-source merge + diagnostics**
- 目的: “既知 intrinsic だけ” を SSOT 化しつつ、suffix 正規化が prefix の変数を見失わないようにする（既定挙動不変）。
- Task B（バグ修正）: `AvailableInputsCollectorBox::collect(.., prefix_variables)` を追加し、Function params > Prefix variables > CapturedEnv の 3-source merge に変更
- Task A（SSOT化）: `KnownIntrinsicRegistryBox` を追加し、intrinsic の metadata（name/arity/type_hint）を `known_intrinsics.rs` に集約
- Task C（診断）: `OutOfScopeReason::IntrinsicNotWhitelisted` を追加し、Call/MethodCall の out-of-scope 理由を精密化
- 実装 SSOT:
  - `src/mir/control_tree/normalized_shadow/available_inputs_collector.rs`
  - `src/mir/control_tree/normalized_shadow/common/known_intrinsics.rs`
  - `src/mir/control_tree/normalized_shadow/common/expr_lowerer_box.rs`
  - `src/mir/control_tree/normalized_shadow/common/expr_lowering_contract.rs`
  - `src/mir/builder/control_flow/normalization/execute_box.rs`
- 設計 SSOT:
  - `docs/development/current/main/design/normalized-expr-lowering.md`

## 2025-12-19：Phase 141 P1 完了 ✅

**Phase 141 P1: KnownIntrinsicOnly (length0)**
- 目的: impure 導入の安全なオンランプとして、既知 intrinsic（小さな allowlist）のみを ExprLowerer に追加
- 仕様:
  - `ExprLoweringScope::WithImpure(ImpurePolicy::KnownIntrinsicOnly)` でのみ `receiver.length()` を lowering
  - それ以外の Call/MethodCall は引き続き `Ok(None)`（既定挙動不変）
- Fixture:
  - `apps/tests/phase141_p1_if_only_post_k_return_length_min.hako`（expected exit code 3）
- Smoke tests:
  - VM: `tools/smokes/v2/profiles/integration/apps/archive/phase141_p1_if_only_post_k_return_length_vm.sh`
  - LLVM EXE: `tools/smokes/v2/profiles/integration/apps/archive/phase141_p1_if_only_post_k_return_length_llvm_exe.sh`
- 入口: `docs/development/current/main/phases/phase-141/README.md`

## 2025-12-19：Phase 141 P0 完了 ✅

**Phase 141 P0: Impure Extension Contract (Call/MethodCall stays out-of-scope)**
- 目的: 次フェーズの Call/MethodCall 導入に向けて、pure/impure 境界の contract（SSOT）を型で固定
- SSOT:
  - `src/mir/control_tree/normalized_shadow/common/expr_lowering_contract.rs`
  - `src/mir/control_tree/normalized_shadow/common/expr_lowerer_box.rs`
- 仕様: Call/MethodCall は引き続き `Ok(None)`（既定挙動不変）
- 入口: `docs/development/current/main/phases/phase-141/README.md`

## 2025-12-19：Phase 140 完了 ✅

**Phase 140: NormalizedExprLowererBox (pure expressions)**
- 目的: “return の形追加” をやめて、pure expression を AST walker で一般化して収束させる
- SSOT:
  - `src/mir/control_tree/normalized_shadow/common/expr_lowerer_box.rs`
  - `src/mir/control_tree/normalized_shadow/common/return_value_lowerer_box.rs`
- 仕様:
  - pure のみ（Variable / Integer&Bool literal / unary(not,-) / arith(+,-,*,/) / compare(==,!=,<,<=,>,>=)）
  - Call/MethodCall など impure は `Ok(None)`（Phase 141+）
- 入口: `docs/development/current/main/phases/phase-140/README.md`

## 2025-12-19：Phase 139 完了 ✅

**Phase 139: post-if `post_k` Return Lowering Unification**
- 目的: if-only `post_k` 側の return lowering を `ReturnValueLowererBox` に統一し、loop/if の出口を一本化
- 実装:
  - `src/mir/control_tree/normalized_shadow/post_if_post_k.rs` の return lowering を `ReturnValueLowererBox::lower_to_value_id()` に委譲
  - out-of-scope は `Ok(None)` でフォールバック（既定挙動不変・dev-only）
- Fixture:
  - `apps/tests/phase139_if_only_post_k_return_add_min.hako`（expected exit code 4）
- Smoke tests:
  - VM: `tools/smokes/v2/profiles/integration/apps/archive/phase139_if_only_post_k_return_add_vm.sh`
  - LLVM EXE: `tools/smokes/v2/profiles/integration/apps/archive/phase139_if_only_post_k_return_add_llvm_exe.sh`
- 入口: `docs/development/current/main/phases/phase-139/README.md`

## 2025-12-18：Phase 138 完了 ✅

**Phase 138: ReturnValueLowererBox - Return Lowering SSOT**
- 目的: Return lowering logic を共有 Box として抽出し SSOT を確立
- 実装:
  - Created `common/return_value_lowerer_box.rs` (~300 lines)
  - Migrated `loop_true_break_once.rs` to use Box (2 call sites)
  - 5 comprehensive unit tests for all patterns
  - Code reduction: ~115 lines removed from loop_true_break_once.rs
- SSOT:
  - `ReturnValueLowererBox::lower_to_value_id()`
  - Supported: Variable, Integer literal, Add expression (x + 2, 5 + 3)
  - Fallback: Out-of-scope patterns return `Ok(None)`
- Tests:
  - 5 new unit tests: variable, integer literal, add (var+int), add (int+int), fallback (subtract)
  - All regressions PASS: Phase 137/97/131/135/136
- Design Decision:
  - Phase 138 P0 migrated loop paths only
  - Phase 139 P0 will unify post_if_post_k.rs for complete SSOT
- Architecture Impact:
  - Single location for return lowering improvements
  - Isolated unit tests for return lowering logic
  - Easy to add new return patterns in one location
- 入口: `docs/development/current/main/phases/phase-138/README.md`

---

## 2025-12-18：Phase 137 完了 ✅

**Phase 137: loop(true) break-once with return add expression**
- 目的: Phase 136 を拡張し、return 時に最小 add 式をサポート
- 仕様:
  - `return x + 2`（variable + integer literal）→ exit code 3
  - `return 5 + 3`（integer literal + integer literal）→ exit code 8
  - `loop(true) { x = 1; break }; x = x + 10; return x + 2` → exit code 13
- 実装:
  - `src/mir/control_tree/normalized_shadow/loop_true_break_once.rs`（`lower_return_value_to_vid()` 行 638-743, BinaryOp Add at 行 673）
  - BinaryOp Add パターン追加（LHS: Variable or Integer literal, RHS: Integer literal only）
  - Ok(None) fallback for out-of-scope patterns（`return x + y`, `return x - 2` 等）
- Return Value Lowering SSOT:
  - Documentation: `loop_true_break_once.rs`（行 29-46, module-level comment）
  - Boxification trigger: 2+ files で同一 return lowering logic が必要になった時
- Fixtures:
  - `phase137_loop_true_break_once_return_add_min.hako`（期待: exit code 3）
  - `phase137_loop_true_break_once_return_add_const_min.hako`（期待: exit code 8）
  - `phase137_loop_true_break_once_post_return_add_min.hako`（期待: exit code 13）
- Smoke tests:
  - VM: 3/3 PASS
  - LLVM EXE: 3/3 PASS
- Regression:
  - Phase 97: 2/2 PASS（next_non_ws, json_loader_escape）
  - Phase 131/135/136: 3/3 PASS
- 設計判断（Approach A 採用）:
  - 直接拡張（boxification なし）、変更スコープ小
  - post_if_post_k.rs は未変更（責任分離）
- 入口: `docs/development/current/main/phases/phase-137/README.md`

---

## 2025-12-18：Phase 136 完了 ✅

**Phase 136: loop(true) break-once with return literal**
- 目的: Phase 131-135 を拡張し、return integer literal をサポート
- 仕様:
  - `loop(true) { x = 1; break }; return 7` → exit code 7
  - `loop(true) { x = 1; break }; x = x + 2; return 7` → exit code 7
  - PHI禁止維持、dev-only、既定挙動不変
- 実装:
  - `src/mir/control_tree/normalized_shadow/loop_true_break_once.rs`（`lower_return_value_to_vid()` 行 638-743, Integer literal at 行 661）
  - Integer literal パターン: Const generation（Phase 123 パターン再利用）
  - Ok(None) fallback for out-of-scope patterns（`return "hello"`, `return 3.14` 等）
- Fixtures:
  - `phase136_loop_true_break_once_return_literal_min.hako`（期待: exit code 7）
  - `phase136_loop_true_break_once_post_return_literal_min.hako`（期待: exit code 7）
- Smoke tests:
  - VM: 2/2 PASS
  - LLVM EXE: 2/2 PASS
- Regression:
  - Phase 131: 2/2 PASS
  - Phase 135: 2/2 PASS
- 入口: `docs/development/current/main/phases/phase-136/README.md`

---

## 2025-12-18：Phase 135 P0 完了 ✅

**Phase 135 P0: Normalization Plan Suffix Detection Generalization**
- 目的: NormalizationPlanBox の suffix 検出を一般化し、post-loop assign が 0 回でも OK に
- 背景:
  - Phase 133 までは `loop + assign+ + return`（assign 1回以上必須）
  - Phase 131 は `loop` のみ（return なし）
  - ギャップ: `loop + return`（0 assign）が別経路
- 改善:
  - `loop + assign* + return`（N >= 0 assignments）を統一パターンとして検出
  - Phase 131 と Phase 132-133 を `LoopWithPost { post_assign_count }` enum で統一
  - Phase 131 (loop-only, no return) は `PlanKind::LoopOnly` として独立維持
- 実装:
  - `src/mir/builder/control_flow/normalization/plan_box.rs`（検出ロジック修正のみ）
  - `post_assign_count >= 1` チェックを削除 → `>= 0` を許容
  - `execute_box.rs` は変更なし（既存ロジックが 0 assigns を自然にサポート）
- Fixture: `apps/tests/phase135_loop_true_break_once_post_empty_return_min.hako`（期待: exit code 1）
- Smoke tests:
  - `phase135_loop_true_break_once_post_empty_return_vm.sh` PASS
  - `phase135_loop_true_break_once_post_empty_return_llvm_exe.sh` PASS
- Regression:
  - Phase 131: 2/2 PASS（VM + LLVM EXE）
  - Phase 133: 2/2 PASS（VM + LLVM EXE）
- Unit tests: 9/9 PASS（plan_box module）
- 設計原則:
  - **最小変更**: plan_box.rs の検出条件のみ変更
  - **SSOT 維持**: 検出ロジックは plan_box.rs に集約
  - **Box-First**: PlanBox（what）と ExecuteBox（how）の責任分離維持
- 入口: `src/mir/builder/control_flow/normalization/README.md`（Phase 135 セクション追加）

## 2025-12-18：Phase 133 完了 ✅

**Phase 133: loop(true) break-once + multiple post-loop assigns（dev-only）**
- 目的: Phase 132 を拡張し、post-loop で複数の assign を受理（PHI-free 維持）
- 仕様:
  - `loop(true) { x = 1; break }; x = x + 2; x = x + 3; return x` は exit code `6`（VM/LLVM EXE parity）
  - post_nodes 検出を `len() == 2` から `len() >= 2` に拡張
  - 複数 assign を iterative に lower（LegacyLowerer::lower_assign_stmt 再利用）
- 実装:
  - `src/mir/control_tree/normalized_shadow/loop_true_break_once.rs`（3箇所編集、~30行追加）
  - Pattern detection: all_assigns + ends_with_return
  - Post-loop lowering: for loop で複数 assign 処理
  - SSOT: env_post_k が最終値を保持（ExitMeta に反映）
- Fixture: `apps/tests/phase133_loop_true_break_once_post_multi_add_min.hako`（期待: 6）
- Smoke:
  - `phase133_loop_true_break_once_post_multi_add_vm.sh` PASS
  - `phase133_loop_true_break_once_post_multi_add_llvm_exe.sh` PASS
- Regression: Phase 132/131/97 維持確認（全 PASS）
- Unit tests: 1176/1176 PASS
- 入口: `docs/development/current/main/phases/phase-133/README.md`

## 2025-12-18：Phase 130 完了 ✅

**Phase 130: if-only Normalized "Small Expr/Assign" Expansion（dev-only）**
- 目的: post_k 内の最小 post-if 計算（`x = x + 3; return x`）を Normalized で通す（PHI禁止）
- 実装:
  - P1: Assign(Variable) - `x = y` サポート（env map 直接更新）
  - P2: Assign(Add) - `x = x + <int literal>` サポート（Const + BinOp Add）
  - P3: Verifier - env map が env layout（writes + inputs）外の変数を導入しないことを検証
  - `src/mir/control_tree/normalized_shadow/legacy/mod.rs` (lower_assign_stmt 拡張)
  - `src/mir/control_tree/normalized_shadow/normalized_verifier.rs` (verify_env_writes_discipline 追加)
- Fixture: `apps/tests/phase130_if_only_post_if_add_min.hako`（期待出力: 5\n4）
- Smoke: `phase130_if_only_post_if_add_vm.sh` PASS
- LLVM EXE smoke: `phase130_if_only_post_if_add_llvm_exe.sh`（LLVM 前提が無い環境では SKIP）
- Regression: Phase 129/128 維持確認（全 PASS）
- Unit tests: 1155/1155 PASS
- 入口: `docs/development/current/main/phases/phase-130/README.md`

## 2025-12-18：Phase 129-C 完了 ✅

**Phase 129-C: post-if / post_k continuation（dev-only）**
- post-if（`if { x=2 }; return x`）を post_k continuation で表現
- join_k が env merge → TailCall(post_k, merged_env)
- post_k が post-if statements 実行 → Ret
- PHI禁止: Normalized IR 内に PHI 相当を入れず env 引数で合流
- 実装:
  - `src/mir/control_tree/normalized_shadow/post_if_post_k.rs`（392行、新規）
  - `builder.rs` に PostIfPostKBuilderBox 統合
  - `normalized_verifier.rs` に post_k 構造検証追加
  - `parity_contract.rs` に StructureMismatch 追加
- Fixture: `apps/tests/phase129_if_only_post_if_return_var_min.hako`
- Smoke: `phase129_if_only_post_if_return_var_vm.sh` PASS
- Regression: Phase 129-B, 128 維持確認（全 PASS）
- Unit tests: 1155/1155 PASS
- 入口: `docs/development/current/main/phases/phase-129/README.md`

## 2025-12-18：Phase 132 完了 ✅

**Phase 132: loop(true) break-once + post-loop minimal（dev-only）**
- 目的: Phase 131 を拡張し、ループ後の最小 post 計算まで Normalized shadow で固定（PHI-free 維持）
- 仕様:
  - `loop(true) { x = 1; break }; x = x + 2; return x` は exit code `3`（VM/LLVM EXE parity）
  - post_k continuation で post-loop statements を処理
- 実装:
  - **P0**: post_k 生成（loop_true_break_once.rs 拡張）
  - **P0.5**: StepTree が post-loop statements を保持（suffix router box 追加）
  - **P1**: k_exit continuation 分類修正（構造ベースの判定）
  - **R0**: Continuation SSOT 一本化 + legacy 隔離 + docs 整備
- SSOT:
  - `JoinInlineBoundary::default_continuations()` - continuation ID の集約
  - `src/mir/builder/control_flow/joinir/merge/README.md` - merge 契約明文化
  - `src/mir/builder/control_flow/joinir/legacy/README.md` - legacy 撤去条件
- テスト配置: `src/mir/builder/control_flow/joinir/merge/tests/continuation_contract.rs`
- 検証:
  - `bash tools/smokes/v2/profiles/integration/apps/archive/phase132_loop_true_break_once_post_add_vm.sh`
  - `bash tools/smokes/v2/profiles/integration/apps/archive/phase132_loop_true_break_once_post_add_llvm_exe.sh`
- 回帰: Phase 131/97 維持確認（全 PASS）
- 入口: `docs/development/current/main/phases/phase-132/README.md`

## 2025-12-18：Phase 131 P2 完了 ✅

**Phase 131: loop(true) break-once Normalized（dev-only）**
- 目的: Normalized shadow の最小ループを VM/LLVM EXE 両方で動かし、更新値が外に見えることまで固定
- 仕様:
  - `loop(true) { x = 1; break }; return x` は exit code `1`
  - DirectValue mode（PHI-free）で exit 値を `variable_map` に再接続
- 検証:
  - `bash tools/smokes/v2/profiles/integration/apps/archive/phase131_loop_true_break_once_vm.sh`
  - `bash tools/smokes/v2/profiles/integration/apps/archive/phase131_loop_true_break_once_llvm_exe.sh`
- 環境: WSL の一時的な EXDEV（`Invalid cross-device link`）は `wsl --shutdown` 再起動で解消、上記 + `phase97_next_non_ws_llvm_exe.sh` まで PASS
- 入口: `docs/development/current/main/phases/phase-131/README.md`

## 2025-12-18：Phase 127 完了 ✅

**Phase 127: unknown-read strict Fail-Fast（dev-only）**
- unknown-read = reads - (writes ∪ inputs) を検出
- strict: `freeze_with_hint("phase127/unknown_read/<name>", ...)` で即停止（hint必須）
- dev-only non-strict: 理由ログ（tag + count + 先頭数件）
- 入口: `docs/development/current/main/phases/phase-127/README.md`

## 2025-12-18：Phase 128 完了 ✅

**Phase 128: if-only partial assign keep/merge in Normalized (dev-only)**
- StepStmtKind::Assign に value_ast 追加（Phase 128）
- Normalized builder に Assign(int literal) 対応（env 更新）
- Fixture: phase128_if_only_partial_assign_normalized_min.hako（簡易版）
- Smoke: phase128_if_only_partial_assign_normalized_vm.sh（PASS）
- Regression: Phase 121-126, 118 維持確認（全 PASS）
- Unit tests: 1165/1165 PASS
- 入口: `docs/development/current/main/phases/phase-128/README.md`
- 実装:
  - `src/mir/control_tree/step_tree.rs` (value_ast 追加、14行追加)
  - `src/mir/control_tree/normalized_shadow/builder.rs` (Assign lowering、87行追加)
- Note: 完全な join_k continuation は future work（Phase 128 は基本構造確立）

## 2025-12-18：Phase 126 完了 ✅

**Phase 126: available_inputs SSOT wiring (dev-only)**
- AvailableInputsCollectorBox 実装（function params + CapturedEnv 収集）
- try_lower_if_only() に available_inputs 配線（dev-only）
- EnvLayout.inputs が実際に使用されるようになった
- Fixture 強化: reads-only 変数の return 解決を確認
- Regression: Phase 121-125, 118 維持確認（全 PASS）
- Unit tests: 28/28 PASS (including 5 new AvailableInputsCollectorBox tests)
- Integration smoke: PASS (phase125_if_only_return_input_vm.sh, exit code 7)
- 入口: `docs/development/current/main/phases/phase-126/README.md`
- 実装:
  - `src/mir/control_tree/normalized_shadow/available_inputs_collector.rs` (143行、新規)
  - `src/mir/control_tree/normalized_shadow/builder.rs` (available_inputs 配線)
  - `src/mir/builder/calls/lowering.rs` (AvailableInputsCollectorBox::collect() 呼び出し)

## 2025-12-18：Phase 125 P2-P5 完了 ✅

**Phase 125 P2-P5: Reads-Only Env Inputs (dev-only, structure)**
- EnvLayout 導入（writes + inputs の 2 レーン化）
- from_contract: reads ∩ available_inputs で inputs を決定（決定的順序）
- Return(Variable) 解決拡張: writes or inputs から解決
- Fail-Fast with hint: env に無い変数は構造化エラー
- Unit tests: 18/18 PASS (including new test_return_variable_from_inputs_stub)
- Integration smoke: PASS (phase125_if_only_return_input_vm.sh, exit code 7)
- Regression: Phase 121-124, 118 維持確認（全 PASS）
- 入口: `docs/development/current/main/phases/phase-125/README.md`
- 実装:
  - `src/mir/control_tree/normalized_shadow/builder.rs` (EnvLayout, 186行追加)
- Note: P3 (available_inputs wiring) 未実装、inputs は空（structure-only）

## 2025-12-18：Phase 124 完了 ✅

**Phase 124: Normalized reads facts + Return(Variable from env)（dev-only）**
- StepTreeFacts に reads 追加（Variable 参照を AST から抽出）
- StepTreeContract signature に reads 反映（決定性維持）
- env マッピング（変数名 → ValueId）を writes から生成
- Return(Variable) サポート: env にある変数のみ（writes 由来）
- env に無い Variable は Fail-Fast エラー（phase124 error → Ok(None) fallback）
- Box-first modularization: extract_variables_from_ast() で SSOT 化
- Unit tests: 1159 tests PASS (including test_return_variable_from_env)
- Integration smoke: PASS (`phase124_if_only_return_var_vm.sh`, exit code 7 許容）
- 回帰: Phase 121/123/118 維持確認
- 入口: `docs/development/current/main/phases/phase-124/README.md`
- 実装:
  - `src/mir/control_tree/step_tree_facts.rs` (reads 追加、76行)
  - `src/mir/control_tree/step_tree_contract_box.rs` (reads 反映、101行)
  - `src/mir/control_tree/step_tree.rs` (extract_variables_from_ast 追加、612行)
  - `src/mir/control_tree/normalized_shadow/builder.rs` (env マッピング追加、837行)

## 2025-12-18：Phase 123 完了 ✅

**Phase 123: if-only Normalized semantics（dev-only）**
- Return(Integer literal) → `Const + Ret(Some(vid))` 生成
- Return(Variable) → `Ok(None)` (graceful degradation, Phase 124 で対応)
- If(minimal compare) → Variable vs Integer literal のみ対応
- Graceful degradation: Phase 123 制限は `Ok(None)` で legacy に fallback
- Structured error codes: `[phase123/...]` prefix で明示的エラー
- Box-first modularization: parse/verify/lower 責務分離
- Unit tests: 8 tests PASS (including graceful degradation test)
- Integration smoke: PASS (`phase123_if_only_normalized_semantics_vm.sh`)
- 入口: `docs/development/current/main/phases/phase-123/README.md`

## 2025-12-18：Phase 121 完了 ✅

**Phase 121: StepTree→Normalized Shadow Lowering (if-only, dev-only)**
- 箱化モジュール化: normalized_shadow/{contracts,builder,parity}.rs (508行、新規)
- Shadow lowering: StepTree → JoinModule (Normalized方言、if-only限定)
- Capability guard: Loop/Break/Continue を明示的拒否（SSOT）
- Parity 検証: exit contracts + writes 比較（dev ログ / strict fail-fast）
- Dev-only wiring: `joinir_dev_enabled()` のときのみ shadow 生成
- Strict fail-fast: `freeze_with_hint` で mismatch を即座に検出（hint必須）
- Smoke tests: VM 3/3 PASS、LLVM スタブ（ハーネス設定必要）
- 回帰: Phase 120 維持確認、全テスト PASS
- 入口: `docs/development/current/main/phases/phase-121/README.md`
- 設計: `docs/development/current/main/design/control-tree.md` (Phase 121章)

## 2025-12-18：Phase 120 完了 ✅

**Phase 120: StepTree "Facts→Decision→Emit" 箱化モジュール化**
- 箱化モジュール化: StepTreeFacts / StepTreeContractBox の責務分離
- StepTreeBuilderBox: 構造 + facts 抽出まで（意思決定・整形・署名生成はしない）
- StepTreeContractBox: facts → contract の整形のみ（idempotent, deterministic）
- BTreeSet で順序決定性保証（facts は順序に依存しない）
- signature_basis_string() の決定性維持（既定挙動不変）
- 回帰: Phase 103/118 維持確認、全 1142 テスト PASS
- 入口: `docs/development/current/main/design/control-tree.md`
- 実装:
  - `src/mir/control_tree/step_tree_facts.rs` (63行、新規)
  - `src/mir/control_tree/step_tree_contract_box.rs` (168行、新規)
  - `src/mir/control_tree/step_tree.rs` (262→411行、リファクタリング)

## 2025-12-18：Phase 119 完了 ✅

**Phase 119: StepTree cond SSOT（AST handle）**
- StepNode::If / StepNode::Loop に cond_ast (AstNodeHandle) を追加
- SSOT: cond は AST 参照を保持、cond_sig は派生（署名/ログ/差分検知）
- 不変条件: cond_ast は signature_basis_string() に混ぜない（決定性維持）
- 実装: Box<ASTNode> clone（dev-only なので許容）
- 回帰: Phase 103/104/118 維持確認
- 入口: `docs/development/current/main/design/control-tree.md`

## 2025-12-18：Phase 118 完了 ✅

**Phase 118: loop + if-only merge parity**
- loop + if-only で条件付き変数更新 merge を VM/LLVM で固定
- Fixture: phase118_loop_nested_if_merge_min.hako (expected: 2)
- Pattern3 (if-sum) 活用
- Smoke: VM + LLVM EXE parity 検証済み
- Follow-up: Pattern3 carrier PHI contract（expected: 12）
  - Fixture: phase118_pattern3_if_sum_min.hako
  - Smoke: phase118_pattern3_if_sum_{vm,llvm_exe}.sh
- 回帰: Phase 117 維持確認
- 入口: `docs/development/current/main/phases/phase-118/README.md`

## 2025-12-18：Phase 117 完了 ✅

**Phase 117: if-only nested-if + call merge parity**
- ネストしたif-only（内側if + 外側else）で call 結果 merge を VM/LLVM で固定
- Fixture: phase117_if_only_nested_if_call_merge_min.hako (expected: 2, 3, 4)
- Smoke: VM + LLVM EXE parity 検証済み
- 回帰: Phase 116 維持確認
- 入口: `docs/development/current/main/phases/phase-117/README.md`

## 2025-12-18：Phase 116 完了 ✅

**Phase 116: if-only keep+call merge parity**
- if-only で片側が元値保持、片側が call 結果のパターンを VM/LLVM で固定
- Fixture: phase116_if_only_keep_plus_call_min.hako (expected: 10, 2)
- Smoke: VM + LLVM EXE parity 検証済み
- 回帰: Phase 115 維持確認
- 入口: `docs/development/current/main/phases/phase-116/README.md`

## 2025-12-18：Phase 115 完了 ✅

**Phase 115: if-only call result merge parity**
- if-only で関数呼び出し結果を merge するパターンを VM/LLVM で固定
- Fixture: phase115_if_only_call_merge_min.hako (expected: 2, 3)
- Smoke: VM + LLVM EXE parity 検証済み
- 回帰: Phase 103/113/114 維持確認
- 入口: `docs/development/current/main/phases/phase-115/README.md`

## 2025-12-18：Phase 114 完了 ✅

**Phase 114: if-only return+post parity**
- if-only で early return + post-if statements パターンを VM/LLVM で固定
- Fixture: phase114_if_only_return_then_post_min.hako (expected: 7, 2)
- Smoke: VM + LLVM EXE parity 検証済み
- 回帰: Phase 103/113 維持確認
- 入口: `docs/development/current/main/phases/phase-114/README.md`

## 2025-12-18：Phase 113 完了 ✅

**Phase 113: if-only partial assign parity**
- if-only（else なし）の片側代入で「保持 merge」パターンを VM/LLVM で固定
- Fixture: phase113_if_only_partial_assign_min.hako
- Smoke: VM + LLVM EXE parity 検証済み

## 2025-12-18：Phase 112 完了 ✅

**Phase 112: StepTree Capability Guard (strict-only)**
- StepTree の required_caps を strict mode でチェック
- Allowlist: If, NestedIf, Loop, Return, Break, Continue
- Deny (strict): NestedLoop, TryCatch, Throw, Lambda, While, ForRange, Match, Arrow
- Error format: `[joinir/control_tree/cap_missing/<Cap>] <msg>  Hint: <hint>`
- Default behavior unchanged (strict=false always Ok)
- 入口: `docs/development/current/main/design/control-tree.md`
- 実装:
  - `src/mir/builder/control_flow/joinir/control_tree_capability_guard.rs`
  - `src/mir/builder/calls/lowering.rs` (wired into lower_function_body)

## 2025-12-18：Phase 111 完了 ✅

**Phase 111: StepTreeContract + StepTreeSignature（dev-only）**
- StepTreeContract を追加（`exits` / `writes` / `required_caps` / `cond_sig`）
- StepTreeSignature を追加（`signature_basis_string()` を安定 hash、Span は含めない）
- dev-only parity を break/continue に加えて return まで拡張
- 入口: `docs/development/current/main/design/control-tree.md`
- 実装: `src/mir/control_tree/step_tree.rs`

## 2025-12-18：Phase 110 完了 ✅

**Phase 110: ControlTree / StepTree（構造SSOT・dev-only観測）**
- StepTree の語彙（Block/If/Loop/Stmt）と Feature を pure AST で実装（ValueId/PHI/CFG を混ぜない）
- joinir routing 側で extractor との “分類矛盾” を dev-only で検出（strict のみ Fail-Fast）
- 関数本体の StepTree を dev-only でダンプ（既定挙動不変）
- 入口: `docs/development/current/main/design/control-tree.md`
- 実装:
  - `src/mir/control_tree/step_tree.rs`
  - `src/mir/builder/calls/lowering.rs`
  - `src/mir/builder/control_flow/joinir/routing.rs`

## 2025-12-17：Phase 109 完了 ✅

**Phase 109: error_tags hints SSOT**
- policy/validator エラーを "tag + message + hint" 形式に統一
- freeze_with_hint() API 追加（hint 必須、空なら panic）
- Phase 107/104/100 の代表3箱を hint 対応に移行
- 入口: `docs/development/current/main/phases/phase-109/README.md`

## 2025-12-17：Phase 104 完了 ✅

**Phase 104: loop(true) + break-only digits（read_digits 系）**
- read_digits_from 形の `loop(true)` を Pattern2 で受理（loop var 抽出 + break cond 正規化）
- fixture: `apps/tests/phase104_read_digits_loop_true_min.hako`（expected: `2`, `1`）
- smoke: `tools/smokes/v2/profiles/integration/apps/phase104_read_digits_vm.sh` / `tools/smokes/v2/profiles/integration/apps/phase104_read_digits_llvm_exe.sh`
- P2: json_cur 由来の回帰面を追加（fixture+VM/LLVM EXE smoke）

## 2025-12-17：Phase 107 完了 ✅

**Phase 107: json_cur find_balanced_* depth scan（VM + LLVM EXE parity）**
- depth scan + nested if + return-in-loop を Pattern2 policy で受理（hardcode なし）
- fixture:
  - `apps/tests/phase107_find_balanced_array_end_min.hako`（expected: `1`, `3`）
  - `apps/tests/phase107_find_balanced_object_end_min.hako`（expected: `1`, `3`）
- smoke（integration）:
  - VM: `tools/smokes/v2/profiles/integration/apps/phase107_find_balanced_array_end_vm.sh` / `tools/smokes/v2/profiles/integration/apps/phase107_find_balanced_object_end_vm.sh`
  - LLVM EXE: `tools/smokes/v2/profiles/integration/apps/phase107_find_balanced_array_end_llvm_exe.sh` / `tools/smokes/v2/profiles/integration/apps/phase107_find_balanced_object_end_llvm_exe.sh`
- 入口: `docs/development/current/main/phases/phase-107/README.md`

## 2025-12-17：Phase 108 完了 ✅

**Phase 108: Pattern2 policy router SSOT（薄い入口の固定）**
- post-loop early return を一般 plan（`cond`/`ret_expr`）として独立し、Pattern2Inputs の依存を解消
- ApplyPolicyStepBox から “直叩き” を撤去し、policy router 1本に集約（入口SSOT）
- 入口: `docs/development/current/main/phases/phase-108/README.md`

## 2025-12-17：Phase 103 P0 完了 ✅

**Phase 103: if-only regression baseline**
- if-only（loop無し）の nested if + merge を fixture 化し、VM/LLVM EXE parity を integration smoke で固定

## 2025-12-17：Phase 100 P3 完了 ✅

**Phase 100 P3: String Accumulator Captures**
- String accumulator（`out = out + ch`）を最小形で固定し、VM/LLVM EXE parity を smoke で検証
- LLVM EXE 側の stringish 伝播（PHI/copy/binop）を修正し、concat の意味論を安定化

## 2025-12-17：Phase 100 P2 完了 ✅

**Phase 100 P2: Mutable Accumulator Captures**
- Mutable accumulator pattern (`out = out + ch`) を Pattern2 carrier で対応
- ScopeManager 委譲により read-only check を回避
- Numeric output validation (count=3) で fixture 固定
- VM/LLVM EXE parity 完了（smoke 追加）

## 2025-12-17：Phase 100 P1 完了 ✅

**Phase 100 P1: Pinned Local Captures**
- CapturedEnv に CapturedKind (Explicit/Pinned) 拡張
- PinnedLocalAnalyzer で loop-outer read-only locals を識別
- Pinned receiver (例: s.substring()) が loop 内で使用可能に
- Fixture/smoke で動作確認済み

## 2025‑12‑17：Phase 99 完了 ✅

**Phase 99: Trim/escape 実コード寄り強化（VM+LLVM EXE）**
- next_non_ws を3ケース固定（`2`, `-1`, `3`）— 改行/CR/Tab混在パターン追加
- escape 末尾バックスラッシュを best-effort として固定（`hello\` そのまま出力）
- VM+LLVM EXE parity 完全対応、integration smoke で検証済み

関連（設計/実装の入口）:
- Phase 100: `docs/development/current/main/phases/phase-100/README.md`

## 2025‑12‑15：Phase 132 完了 ✅

**Phase 132: LLVM Exit PHI=0 根治修正 完了！**
- ループ exit PHI が 0 を返す問題を根治解決
- 原因（2層）: (1) JoinIR/Boundary が exit 値を境界に渡していない、(2) LLVM Python が PHI を落とす/上書きする
- 修正: Pattern 1 で exit 値を明示的に渡す + `predeclared_ret_phis` 使用 + `builder.vmap` の PHI 保護
- 結果: `/tmp/p1_return_i.hako` が 3 を返す（VM 一致）
- 詳細: `investigations/phase132-llvm-exit-phi-wrong-result.md`

**追加（Phase 132-P2）: Case C の Exit PHI ValueId 衝突を修正**
- 原因: `exit_phi_builder.rs` が module-level allocator を使い、同一関数内で ValueId が衝突し得た
- 修正: `func.next_value_id()`（function-level）へ統一（`bd07b7f4`）
- 結果: `apps/tests/llvm_stage3_loop_only.hako` が LLVM EXE でも `Result: 3`（VM 一致）
- 詳細: `investigations/phase132-case-c-llvm-exe.md`

**追加（Phase 132-P3）: Exit PHI collision の早期検出（debug-only）**
- `verify_exit_phi_no_collision()` を `contract_checks.rs` に追加し、ValueId 衝突を JoinIR merge の段階で Fail-Fast する

## 2025‑12‑15：Phase 133–136（短報）

- Phase 133: promoted carrier（Trim）の `join_id` 解決を SSOT に寄せて修正（smoke は compile-only）。
- Phase 134: plugin loader best-effort loading を導入（決定的順序 + failure 集約 + 継続）。
- Phase 135: ConditionLoweringBox が allocator SSOT を無視して ValueId 衝突を起こす問題を根治。
  - 詳細: `docs/development/current/main/phases/phase-135/README.md`
- Phase 136: MirBuilder の Context 分割を完了し、状態の SSOT を Context に一本化。
  - 詳細: `docs/development/current/main/phases/phase-136/README.md`

## 2025‑12‑16：Phase 137‑141（短報）

- Loop Canonicalizer（前処理 SSOT）は Phase 141 まで完了（既定挙動は不変、dev-only 観測/strict parity あり）。
  - 設計 SSOT: `docs/development/current/main/design/loop-canonicalizer.md`
  - 実装: `src/mir/loop_canonicalizer/mod.rs`（+ 観測: `src/mir/builder/control_flow/joinir/routing.rs`）
  - Phase 記録: `docs/development/current/main/phases/phase-137/README.md`

## 2025‑12‑16：Phase 92（短報）

- Phase 92（ConditionalStep / P5b escape の lowering 基盤）は完了。
  - 条件付き更新（`ConditionalStep`）のJoinIR表現 + Pattern2 側の委譲（emitter 箱化）
  - 条件式での body-local 参照（`ConditionEnv → LoopBodyLocalEnv`）+ 最小E2E smoke 固定
  - Phase 記録（SSOT入口）: `docs/development/current/main/phases/phase-92/README.md`
  - 残: P5b の “完全E2E” は body-local promotion 拡張後（スコープ外で保留）

## 2025‑12‑16：Phase 93（短報）

- Trim の `is_ch_match` など「ConditionOnly（PHIで運ばない派生値）」を Derived Slot として毎イテレーション再計算できるようにした。
  - SSOT: `ConditionOnlyRecipe`（運搬禁止）+ `ConditionOnlyEmitter`
  - schedule: `body-init → derived → break` を評価順SSOTとして強制
  - Phase 記録（入口）: `docs/development/current/main/phases/phase-93/README.md`

## 2025‑12‑16：Phase 94（短報）

- P5b escape（`ch` 再代入 + `i` の +1/+2）を “derived（Select）” として扱い、VM E2E を固定。
  - 新箱: `BodyLocalDerivedEmitter` + 明示ポリシー（strict で理由付き Fail-Fast）
  - integration smoke: `tools/smokes/v2/profiles/integration/apps/archive/phase94_p5b_escape_e2e.sh`
  - Phase 記録（入口）: `docs/development/current/main/phases/phase-94/README.md`

## 2025‑12‑16：Phase 95（短報）

- MiniJsonLoader の escape ループを Phase 94 基盤で固定（派生 body-local + 条件付き skip）。
  - フィクスチャ: `apps/tests/phase95_json_loader_escape_min.hako`
  - smoke: `tools/smokes/v2/profiles/integration/apps/archive/phase95_json_loader_escape_vm.sh`
  - Phase 記録（入口）: `docs/development/current/main/phases/phase-95/README.md`

## 2025‑12‑16：Phase 96（短報）

- Trim系 policy 化を開始し、MiniJsonLoader の next_non_ws ループを fixtures/smoke に追加。
  - フィクスチャ: `apps/tests/phase96_json_loader_next_non_ws_min.hako`
  - smoke: `tools/smokes/v2/profiles/integration/apps/archive/phase96_json_loader_next_non_ws_vm.sh`
  - Phase 記録（入口）: `docs/development/current/main/phases/phase-96/README.md`

## 2025‑12‑16：Phase 97（短報）

- Phase 95/96 の MiniJsonLoader fixture を LLVM EXE ラインでも固定し、JoinIR/Trim の退行検出を強化。
  - smoke: `tools/smokes/v2/profiles/integration/apps/archive/phase97_next_non_ws_llvm_exe.sh`（next_non_ws）
  - smoke: `tools/smokes/v2/profiles/integration/apps/archive/phase97_json_loader_escape_llvm_exe.sh`（escape）
  - Phase 記録（入口）: `docs/development/current/main/phases/phase-97/README.md`

## 2025‑12‑17：Phase 98（短報）

- plugin loader に strict fail-fast を導入し（HAKO_JOINIR_STRICT=1）、FileBox/MapBox の LLVM EXE parity を持続可能に。
  - smoke: `tools/smokes/v2/profiles/integration/apps/archive/phase97_next_non_ws_llvm_exe.sh`
  - smoke: `tools/smokes/v2/profiles/integration/apps/archive/phase97_json_loader_escape_llvm_exe.sh`
  - Phase 記録（入口）: `docs/development/current/main/phases/phase-98/README.md`

## 2025‑12‑17：Phase 102（短報）

- real-app（MiniJsonLoader.read_quoted_from）の loop を最小抽出し、VM + LLVM EXE で regression を固定（期待: length=4）。
  - フィクスチャ: `apps/tests/phase102_realapp_read_quoted_min.hako`
  - smoke: `tools/smokes/v2/profiles/integration/apps/archive/phase102_realapp_read_quoted_vm.sh`
  - smoke: `tools/smokes/v2/profiles/integration/apps/archive/phase102_realapp_read_quoted_llvm_exe.sh`
  - Phase 記録（入口）: `docs/development/current/main/phases/phase-102/README.md`

## 2025‑12‑14：現状サマリ

（補足）docs が増えて迷子になったときの「置き場所ルール（SSOT）」:

- `docs/development/current/main/DOCS_LAYOUT.md`

### JoinIR / Loop / If ライン

- LoopBuilder は Phase 186‑187 で完全削除済み。**JoinIR が唯一の loop lowering 経路**。
- LoopPattern 系は Pattern1–4 まで実装・本線化済み：
  - Pattern1: Simple While
  - Pattern2: Loop with Break
  - Pattern3: Loop with If‑Else PHI（break/continue なし）
  - Pattern4: Loop with Continue（multi‑carrier 対応）
- Exit/Carrier/Boundary ラインは次の箱で SSOT 化：
  - `CarrierInfo` / `ExitMeta` / `ExitBindingBuilder`
  - `JoinInlineBoundary` + `LoopExitBinding`
- If lowering は IfSelectLowerer/IfMergeLowerer を中心に整理済み。Select/PHI の扱いも Phase 189 系で橋渡し済み。
- 残課題（JoinIR ライン）：
  - JoinIR→MIR merge の一般化（複雑な Select/PHI パターンの統合）
  - JsonParserBox など実アプリ側での長期運用テスト
- Phase 86–90（Loop frontends）まとめ（1枚）:
  - `docs/development/current/main/phase86-90-loop-frontends-summary.md`

### Phase 86–90: Loop frontends（要約）

- Phase 86–90 は “dev-only fixtures + shape guard + fail-fast” で段階的に固定済み。
- 具体の fixture / shape / 未検証は `docs/development/current/main/phase86-90-loop-frontends-summary.md` を SSOT とする。

### Scope / BindingId（dev-only の段階移行ライン）

- MIR builder 側で lexical scope / shadowing を実在化し、言語仕様の “local はブロック境界で分離” を SSOT に揃えた。
  - JoinIR lowering 側は name-based から BindingId-based へ段階移行中：
  - `ScopeManager.lookup_with_binding()` / `ConditionEnv.binding_id_map` を導入し、shadowing を壊さずに解決できる足場を作った。
  - promoted carriers（DigitPos/Trim）については `BindingId(original) → BindingId(promoted) → ValueId(join)` の鎖を dev-only で整備中。
  - Phase 86 で `carrier_init_builder` / `error_tags` を SSOT 化し、段階移行ラインの基盤（ValueId 生成とエラー語彙）が確立した。
  - これにより、BindingId dual-path の拡張・統合時に「生成」と「表示」の責務が混ざらない構造が固定された。
  - Phase 81 で Pattern2（DigitPos/Trim）の ExitLine 契約（ConditionOnly を exit PHI から除外、LoopState のみを reconnect）を E2E で固定した。
  - 参照:
    - `docs/development/current/main/phase73-scope-manager-design.md`
    - `docs/development/current/main/phase78-bindingid-promoted-carriers.md`
    - `docs/development/current/main/phase80-bindingid-p3p4-plan.md`
    - `docs/development/current/main/phase81-pattern2-exitline-contract.md`
    - `docs/development/current/main/phase78-85-boxification-feedback.md`

### JsonParser / Selfhost depth‑2 ライン

- `selfhost_build.sh --json` で Program JSON v0 emit は安定。  
  `.hako` 側から env 経由で JSON を読む最小ループ（`program_read_min.hako`）は動作確認済み。
- JsonParserBox / BundleResolver のループ 21 本のうち：
  - 18 本は Pattern1–4 で JoinIR 対応済み（Phase 162–165）。
  - `_trim` を含む一部の複合ループは、ValueId 境界や Box 登録など残課題あり。
- BoolExprLowerer / condition_to_joinir で OR/AND/NOT 付き条件式の lowering は実装完了（Phase 168–169）。
- 残課題（JsonParser/selfhost depth‑2）：
  - JoinIR→MIR ValueId boundary の完全一般化（条件用 ValueId を含める）
  - JsonParserBox の using / Box 登録（Rust VM 側での認識）
  - Program JSON v0 を JsonParserBox 経由でフル解析する line の仕上げ

### Ring0 / Runtime / CoreServices ライン

- Ring0Context + Ring0Registry で OS API 抽象化レイヤ完成：
  - MemApi / IoApi / TimeApi / LogApi / FsApi / ThreadApi
  - RuntimeProfile(Default / NoFs) で条件付き必須を制御。
- CoreServices（ring1‑core）として次を実装済み：
  - StringService / IntegerService / BoolService
  - ArrayService / MapService / ConsoleService
  - PluginHost 統合 + UnifiedBoxRegistry からの自動初期化
- FileBox / FileHandleBox ライン：
  - Ring0FsFileIo 経由で read / write / append / metadata 完全対応
  - Default プロファイルでは必須、NoFs プロファイルでは disabled。
- Logging ライン：
  - ConsoleService（user‑facing）
  - Ring0.log（internal/dev）
  - println!（test 専用）
  の 3 層が `logging_policy.md` で整理済み。JoinIR/Loop trace も同ドキュメントに集約。
- VM backend の Box 解決（UnifiedBoxRegistry / BoxFactoryRegistry）の経路図:
  - `docs/development/current/main/phase131-2-box-resolution-map.md`
- LLVM（Python llvmlite）lowering の棚卸し（Phase 131-3..10）:
  - `docs/development/current/main/phase131-3-llvm-lowering-inventory.md`
  - 状態: Case B（Pattern1/loop_min_while）は EMIT/LINK/RUN まで復旧済み。Phase 132 で `return i` の VM/LLVM parity を固定。
  - Case C は別途 “Case C docs” を SSOT にして追跡する（状況は更新されるので、この箇所では断定しない）
  - Case C の調査と実装計画:
    - `docs/development/current/main/phase131-11-case-c-summary.md`
    - `docs/development/current/main/case-c-infinite-loop-analysis.md`

---

## 2025‑09‑08：旧スナップショット（参考）

- LLVM 側 P0 完了（BitOps/Array/Echo/Map 緑）。VInvoke(by‑name/by‑id vector) は戻り値マッピングの暫定課題を確認中（Decisions 参照）。
- selfhosting-dev の作業を main に順次取り込み。VM/MIR 基盤は main で先に整える方針。

直近タスク（当時）
1) continue/break の lowering（Builder 修正のみで表現）
   - ループ文脈スタック {head, exit} を導入。
   - continue に遭遇 → head（または latch）へ br を emit し終端。
   - break に遭遇 → exit へ br を emit し終端。
   - post‑terminated 後に emit しない制御を徹底。
2) ループ CFG の厳密化
   - 単一 exit ブロックの徹底。
   - Phi はヘッダでキャリー変数を合流（SSA/支配関係が崩れない形）。
3) 検証とスモーク
   - Verifier 緑（dominance/SSA）。
   - VM のループ代表（単純/ネスト/早期継続・脱出）。
   - LLVM/Cranelift EXE に綺麗に降りる（br/phi ベースで問題なし）。

代表コマンド（例）
- ビルド: `cargo build --release`
- LLVM smoke: `LLVM_SYS_180_PREFIX=$(llvm-config-18 --prefix) NYASH_LLVM_BITOPS_SMOKE=1 ./tools/llvm_smoke.sh release`
- VInvoke 調査: `NYASH_LLVM_VINVOKE_TRACE=1 NYASH_LLVM_VINVOKE_SMOKE=1 ./tools/llvm_smoke.sh release`
