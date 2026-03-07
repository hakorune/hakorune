# CURRENT_TASK (root pointer)

Status: SSOT
Date: 2026-03-06
Scope: repo root の再起動入口。詳細ログは `docs/development/current/main/` を正本とする。

## Purpose

- root から最短で `Current Blocker` と `next fixed order` に到達する。
- 本ファイルは薄い入口に保ち、長文履歴はアーカイブへ逃がす。
- runtime lane の Next は `docs/development/current/main/phases/phase-29y/60-NEXT-TASK-PLAN.md` を単一正本に固定する。

## Focus Lock (2026-03-02)

- primary target: `kernel-mainline`（`.hako` kernel）を日常既定経路に固定。
- no-fallback: `NYASH_VM_USE_FALLBACK=0`（silent fallback 禁止 / fail-fast）。
- compiler lane は `phase-29bq` を monitor-only 運用（failure-driven reopen のみ）。

## Current Blocker (SSOT)

- compiler lane: `phase-29bq / monitor-only`
  - current blocker: `none`
  - reopen condition: `emit_fail > 0` または `route_blocker > 0`
  - task SSOT:
    - `docs/development/current/main/design/joinir-port-task-pack-ssot.md`
    - `docs/development/current/main/design/joinir-extension-dual-route-contract-ssot.md`
- runtime lane: `phase-29y / none`
  - fixed order SSOT:
    - `docs/development/current/main/phases/phase-29y/60-NEXT-TASK-PLAN.md`
- compiler pipeline lane: `hako-using-resolver-parity / monitor-only`
  - SSOT:
    - `docs/development/current/main/design/selfhost-bootstrap-route-ssot.md`
- de-rust orchestration lane: `phase-29cc / monitor-only`
  - SSOT:
    - `docs/development/current/main/phases/phase-29cc/README.md`
    - `docs/development/current/main/design/de-rust-scope-decision-ssot.md`
- perf lane: `phase-21.5 / monitor-only`
  - SSOT:
    - `docs/private/roadmap/phases/phase-21.5/PLAN.md`

## Immediate Next (this round)

- docs-first / compiler lane SSOT:
  - `docs/development/current/main/design/compiler-task-map-ssot.md`
  - `docs/development/current/main/design/compiler-cleanliness-campaign-ssot.md`
  - `docs/development/current/main/design/normalized-dev-removal-ssot.md`
  - `docs/development/current/main/design/joinir-frontend-legacy-fixture-key-retirement-ssot.md`
- execution rule:
  - 1 blocker = 1受理形 = fixture+gate = 1 commit
  - BoxCount と BoxShape を同コミットで混在させない
- latest audit snapshot (2026-03-06):
  - top-level loop route は recipe-first が主経路（`route_loop -> registry::try_route_recipe_first -> PlanLowerer`）
  - `normalizer/pattern*.rs` は runtime main route 直下では未使用（test-only / 補助経路）
  - `DomainPlan` / `domain_plan` / `DomainRecipe` 識別子は `src/mir/builder/control_flow/{plan,joinir}/**` で 0件。planner payload lane（`LoopCondContinueWithReturnPlan`）は runtime 経路から撤去済み
  - active SSOT の現行runtime主語は `Facts -> Recipe -> Verifier -> Lower`。`DomainPlan` 文言は historical note に限定する
  - active docs の cleanup tracker は historical planner-payload ledger と明示し、runtime contract と residue ledger を分離する
  - `PlanBuildOutcome` は `facts + recipe_contract` のみ。`outcome.plan` / `outcome.plan.take()` 参照は `src/mir/builder/control_flow/{plan,joinir}/**` で 0件
  - release nested-safe recipe-first は `nested_loop_minimal` / `generic_loop_v{1,0}` / exit-driven `loop_cond_break_continue` まで拡張済み
  - 上記 release 例外で compose/verify/lower reject 時は `Ok(None)` を返し、router の no-route 判定へ戻す（互換維持）
  - release scan（`phase29bq_fast_gate_cases.tsv` を release config + `HAKO_JOINIR_DEBUG=1` で実行）で `entry_route` は `recipe_first|none` のみ（126 fixture）
  - nested fallback（`nested_loop_plan` / `generic_loop_body/helpers` / `nested_loop_depth1`）は `planner_required` 時に `loop_cond_continue_with_return` を recipe composer 優先で下ろす（legacy payload 分岐なし）
  - nested fallback の `loop_cond_continue_with_return` gate は共通 helper (`try_compose_loop_cond_continue_with_return_recipe`) に集約済み（重複3箇所を削除）
  - nested fallback の legacy planner payload（`planner_payload -> PlanNormalizer`）は撤去済み
  - `single_planner/rules.rs` は `loop_cond_continue_with_return facts` を recipe-only rule hit 判定のSSOTにし、`route=plan strategy=extract` 経路を撤去済み
  - router の release pre-plan fallback（`release_adopt`）は撤去済み。strict/dev pre-plan は guard-only（`shadow_pre_plan_guard_error`）で adopt しない
  - recipe 補助ログは route 主語へ統一中（`[recipe:verify] route=<...> status=ok`, `[recipe:compose] route=<...> path=<...>`）
  - planner context 表層語彙は `route_kind` へ統一済み（`pattern_kind` は code path から撤去）
  - domain 内部語彙を route 主語へ縮退（`Pattern2StepPlacement` → `LoopBreakStepPlacement`, `Pattern5ExitKind` → `LoopTrueEarlyExitKind`）
  - loop_break 系の診断タグを route 主語へ同期（`[cf_loop/pattern2]` → `[cf_loop/loop_break]`, `joinir/pattern2` → `joinir/loop_break`）
  - RecipeComposer の主要 entrypoint 名を de-number 化（`compose_pattern*` → semantic `compose_<route>_recipe`）
  - `route_prep_pipeline` / `body_local_policy` / `trim_loop_lowering` / `normalization::plan_box` の補助コメントを route 主語へ同期（挙動不変）
  - strict-nested guard SSOT と flowbox tag coverage map の scenario 注記を route 主語へ同期（legacy label は注記で保持）
  - active SSOT docs（`recipe-first-entry-contract-ssot.md`, `plan-mod-layout-ssot.md`, `compiler-task-map-ssot.md`, `coreplan-shadow-adopt-tag-coverage-ssot.md`）の Pattern 主語を route 主語へ同期（契約キー/タグは不変）
  - coreloop test 残語彙を route 主語へ同期（`coreloop_v0/v1_tests` の test name / test labels / assert wording）
  - `plan/mod.rs` / `route_prep_pipeline.rs` / `policies/*` / `condition_env_builder.rs` / `joinir/merge/*` の補助コメントを route 主語へ同期（型名・module名・facts key は不変）
  - active design docs（`coreplan-unknown-loop-strategy-ssot.md`, `coreloop-generic-loop-v0-ssot.md`, `joinir-plan-frag-ssot.md`, `coreplan-migration-roadmap-ssot.md`）の Pattern 主語注記を route 主語へ同期（`legacy label` 注記保持）
  - loop_break module cluster（`loop_break_prep_box` / `loop_break_steps/*` / `loop_break/{api,contracts}`）の補助コメントを route 主語へ同期
  - recognizer/normalizer 補助注記（`if_else_phi`, `normalizer/helpers`, `ast_feature_extractor`, `loop_true_read_digits_policy`）を route 主語へ同期（挙動不変）
  - active docs（`condprofile-ssot.md`, `coreloop-composer-v0-v1-boundary-ssot.md`）の Pattern 主語注記を route 主語へ同期（契約キー/識別子は不変）
  - extractor cluster（`extractors/{mod,pattern1,pattern3,common_helpers}` + `route_shape_recognizers/if_else_phi`）の補助コメントを route 主語へ同期（legacy label は注記で保持）
  - `facts/pattern*_facts.rs` 系ヘッダと loop_break facts 補助注記を route 主語へ同期し、`facts/loop_break_{core,helpers,types,tests,body_local_subset,...}.rs` / `LoopBreakFacts` / `LoopBreakBodyLocalFacts` へ physical path と型名も同期
  - scan/split contract docs（`pattern6-7-contracts.md`, `planfrag-freeze-taxonomy.md`, `planfrag-ssot-registry.md`）を route 主語へ同期し、契約タグは `legacy label` 注記で保持
  - `common_init` / `loop_scope_shape_builder` / `conversion_pipeline` / `ast_feature_extractor` の補助コメントを route 主語へ同期（legacy label 注記を保持、挙動不変）
  - `phase-29ae/README.md` と `coreplan-shadow-adopt-tag-coverage-ssot.md` の Pattern 箇条書きを route 主語へ同期（script名・tag key は不変）
  - string-helper facts cluster も semantic surface へ同期済み（`string_is_integer` / `starts_with` / `int_to_str` / `escape_map` / `split_lines` / `skip_whitespace`）
- compiler fixed order:
  1. active docs（archive除外）の Pattern 主語注記を route 主語へ同期し、必要箇所だけ `legacy label` 注記を残す（進行中: `coreplan-shadow` / `plan-mod-layout` / `compiler-task-map` / `recipe-first-entry` / `condition-observation` / `domainplan-thinning` / `edgecfg-fragments` / `plan-dir-shallowing` / `coreplan-unknown-loop-strategy` / `coreloop-generic-loop-v0` / `joinir-plan-frag` / `coreplan-migration-roadmap` / `condprofile` / `coreloop-composer-v0-v1-boundary` / `pattern6-7-contracts` / `planfrag-freeze-taxonomy` / `planfrag-ssot-registry` は同期済み）。
  2. `plan/**` 内の pattern1..9 残語彙を「挙動不変の comment/test 名」から先に縮退し、型名・module名は inventory化して段階移行する（進行中: `coreloop_v0/v1` tests + `facts/loop_builder.rs` comment + `facts/loop_tests.rs` 名称 + `plan/mod.rs` / `route_prep_pipeline.rs` / `policies/*` comment + loop_break module cluster comment + extractor cluster comment + `facts/pattern*_facts.rs` header comment を同期済み）。
  3. planner/normalizer の dead comments・test-only wiring（payload 前提）を段階撤去する。

## Compiler Cleanup Order (2026-03-04, SSOT)

- D5-A: facts/planner dead staging 削除（挙動不変）
- D5-B: runtime 未参照の legacy module を isolate -> delete
- D5-C: diagnostic-only vocabulary を semantic key に揃える
- D5-D: test-only module wire を runtime build から分離
- D5-E: planner/build test 群を historical DomainPlan-era wording から facts/recipe 契約へ縮退

## Latest Probe Snapshot (direct route)

- command:
  - `tools/dev/direct_loop_progression_sweep.sh --profile phase29x-probe --allow-emit-fail`
- latest result (2026-03-06):
  - `emit_fail_count=2`
  - `unexpected_emit_fail_count=0`
  - `run_nonzero_count=17`
  - `route_blocker_count=0`
  - total=`119`
- class_counts:
  - `run:nonzero-empty=6`（intentional contract exit）
  - `run:nonzero=8`（direct runner bridge limitation cluster）
  - `run:vm-error=3`（provider-dependent cluster）
  - `run:ok=100`
  - `emit:joinir-reject=2`（known / allow-emit-fail）
- note:
  - monitor-known 扱い。compiler blocker は `none` 維持。

## Restart Handoff (2026-03-06)

- detailed archive:
  - `docs/development/current/main/investigations/current_task_archive_2026-03-06_compiler_cleanup.md`
- summary:
  - active docs は `Facts -> Recipe -> Verifier -> Lower` を現行runtime契約として固定済み
  - historical planner-payload wording は active contract から切り離し、ledger / appendix / archive へ退避済み
  - `loop_continue_only` の semantic entry (`LoopContinueOnlyFacts` / `loop_continue_only()` / `PlanRuleId::LoopContinueOnly`) へ runtime 露出を同期済み
  - `if_phi_join` / `loop_continue_only` の facts file/module/type 名も semantic 名へ同期済み
  - compiler 主経路の route enum を semantic 名へ同期済み（`LoopRouteKind::LoopBreak`, `RouteVariant::{LoopSimpleWhile,LoopBreak,IfPhiJoin,LoopContinueOnly}`）
  - loop_break prep-layer の internal type 名も semantic 化済み（`LoopBreakPrepFacts`, `LoopBreakPrepInputs`, `LoopBreakPrepFactsBox`, `LoopBreakDebugLog`）
  - semantic module surface と physical path を同期済み（`plan::loop_break`, `plan::loop_break_prep_box`, `plan::loop_break_steps`）
  - loop_break facts cluster も physical path / type / field を semantic 名へ同期済み（`facts/loop_break_{core,helpers,types,tests,body_local_subset,...}.rs`, `LoopBreakFacts`, `LoopBreakBodyLocalFacts`, `LoopFacts::{loop_break,loop_break_body_local}`）
  - loop_break normalizer harness も semantic file 名へ同期済み（`normalizer/loop_break.rs`。旧 `pattern2_break.rs` は historical docs のみ）
  - carrier/body-local residue も active identifier から整理済み（`promoted_body_locals`, `has_body_local`, `find_first_body_local_dependency`）。旧 `*loopbodylocal*` は historical path / script 名の traceability に限定
  - recipe_tree builder path は既に semantic 名へ収束済み（`recipe_tree/loop_break_builder.rs`）。active docs も `loop_break` / `if_phi_join` / `loop_continue_only` 主語へ追従し、`Pattern2Break` / `pattern2_break.rs` は migration/history docs へ後退
  - `extractors/if_phi_join.rs` へ physical file 名も同期済み。旧 `extractors/pattern3.rs` は消え、`IfPhiJoinParts` / `extract_loop_with_if_phi_parts()` が current surface
  - `extractors/loop_simple_while.rs` と `policies/loop_simple_while_subset_policy.rs` へ physical file/module/function 名も同期済み。旧 `pattern1.rs` / `pattern1_subset_policy.rs` は history/phase docs のみ
  - `loop_simple_while` / `loop_char_map` / `loop_array_join` の facts cluster も physical file/type/function/field 名を semantic 化済み（`facts/{loop_simple_while,loop_char_map,loop_array_join}_facts.rs`, `Loop*Facts`, `try_extract_loop_*_facts`, `LoopFacts::{loop_simple_while,loop_char_map,loop_array_join}`）
  - `loop_true_early_exit` / `nested_loop_minimal` / `bool_predicate_scan` / `accum_const_loop` の facts cluster も physical file/type/function/field 名を semantic 化済み（`facts/{loop_true_early_exit,nested_loop_minimal,bool_predicate_scan,accum_const_loop}_facts.rs`, `LoopTrueEarlyExitFacts`, `NestedLoopMinimalFacts`, `BoolPredicateScanFacts`, `AccumConstLoopFacts`, `LoopFacts::{loop_true_early_exit,nested_loop_minimal,bool_predicate_scan,accum_const_loop}`）
  - string-helper facts cluster も physical file/type/function/field 名を semantic 化済み（`facts/{string_is_integer,starts_with,int_to_str,escape_map,split_lines,skip_whitespace}_facts.rs`, `StringIsIntegerFacts`, `StartsWithFacts`, `IntToStrFacts`, `EscapeMapFacts`, `SplitLinesFacts`, `SkipWhitespaceFacts`, `LoopFacts::{string_is_integer,starts_with,int_to_str,escape_map,split_lines,skip_whitespace}`）
  - verification: `cargo build --release --bin hakorune` PASS / `cargo test --release --lib starts_with_subset_matches_minimal_shape` PASS / `cargo test --release --lib int_to_str_subset_matches_minimal_shape` PASS / `phase29bq_fast_gate_vm.sh --only bq` PASS
  - verification: `rg -n "Pattern(IsInteger|StartsWith|IntToStr|EscapeMap|SplitLines|SkipWs)|pattern_(is_integer|starts_with|int_to_str|escape_map|split_lines|skip_ws)" src/mir/builder/control_flow/plan docs/development/current/main/design CURRENT_TASK.md -g '!**/*history*' -g '!**/*archive*'` = 0 hit
  - `LoopRouteKind::{LoopSimpleWhile,NestedLoopMinimal}` へ active enum variant を semantic 化済み。classifier/router/loop_context/loop_canonicalizer/tests と active docs（`domainplan-thinning-ssot.md`, `coreplan-skeleton-feature-model.md`）も同期した
  - verification: `cargo build --release --bin hakorune` PASS / `phase29bq_fast_gate_vm.sh --only bq` PASS / `rg -n "Pattern1SimpleWhile|Pattern6NestedLoopMinimal" src CURRENT_TASK.md docs/development/current/main/design/domainplan-thinning-ssot.md docs/development/current/main/design/coreplan-skeleton-feature-model.md` = 0 hit
  - `src/mir/join_ir/lowering/**` の live `joinir/patternN` debug/error tags を route 主語へ同期済み（`simple_while_minimal` / `scan_with_init_{minimal,reverse}` / `split_scan_minimal` / `scan_bool_predicate_minimal` / `loop_with_if_phi_if_sum` / `error_tags`）。history/traceability docs と legacy smoke 名はこの slice では不変
  - `LoopRouteKind::LoopTrueEarlyExit` へ enum surface を semantic 化し、`loop_pattern_detection::{kind,classify}` と `join_ir/lowering::{loop_route_router,mod}` の current-facing prose も route 主語へ同期中
  - `ConditionCapability::IfPhiJoinComparable` / `LoopViewBuilder::try_loop_simple_while` へ current helper surface を semantic 化し、`features` / `condition_pattern` / `loop_view_builder` / `route_prep_pipeline` / `loop_with_if_phi_if_sum` の近傍 prose も route 主語へ同期中
  - `join_ir/lowering/loop_routes/**` と `carrier_info` / `loop_update_summary` の stub/docs wording も route-first に同期し、残る `Pattern N` は `traceability-only` 注記へ後退
  - active docs の `Pattern3/4` current-looking wording をさらに薄くし、`strict-nested-loop-guard` は actual route_kind `LoopContinueOnly` へ同期済み
  - verification: `cargo build --release --bin hakorune` PASS / `cargo check --tests` PASS / `cargo test --lib extract_loop_break_parse_integer_subset` PASS / `cargo test --lib loop_break_body_local_facts_detect_trim_seg` PASS / `cargo test --lib coreloop_v1_composes_loop_break_with_value_join` PASS / `phase29bq_fast_gate_vm.sh --only bq` PASS
  - verification: `rg -n 'pattern2_break_types|Pattern2BreakFacts|pattern2_loopbodylocal_facts|Pattern2LoopBodyLocalFacts|pattern2_break:|pattern2_loopbodylocal:' src/mir/builder/control_flow/plan src/mir/builder/control_flow/joinir CURRENT_TASK.md docs/development/current/main/design/plan-dir-shallowing-ssot.md docs/development/current/main/design/pattern-naming-migration-ssot.md` = migration-doc example rows only
  - verification: `rg -n 'pattern2_inputs_facts_box|pattern2_steps|pattern2/api|pattern2/contracts' src/mir/builder/control_flow/plan CURRENT_TASK.md docs/development/current/main/design/compiler-task-map-ssot.md` = 0 hit
  - `loop_break` lowerer / scheduler / scope-manager / targeted tests の log/comment/test 名も semantic 主語へ同期済み
  - verification: `cargo build --release --bin hakorune` PASS / `cargo test --release --lib loop_pattern_detection --no-run` PASS / `phase29bq_fast_gate_vm.sh --only bq` PASS
  - `normalized_dev` は runtime 主経路ではなく removal lane として扱う。削除順序 SSOT は `normalized-dev-removal-ssot.md`
  - Phase R1: `join_ir_runner` / `join_ir_vm_bridge` の normalized route を quarantine し、Structured-only runtime に固定する
  - Phase R2: `JoinIrMode` / `src/config/env` の public normalized helpers / `join_ir_vm_bridge` export を削り、runtime-public 露出を 0 にした
  - verification: `cargo build --release --bin hakorune` PASS / `phase29bq_fast_gate_vm.sh --only bq` PASS / `rg -n "NormalizedDev|NormalizedCanonical|NYASH_JOINIR_NORMALIZED_DEV_RUN" src/config src/mir/join_ir_runner src/mir/join_ir_vm_bridge` = 0 hit
  - Phase R3: `src/mir/join_ir/normalized.rs` + `src/mir/join_ir/normalized/**` + `tests/normalized_joinir_min*` を削除し、`join_ir/mod.rs` / `ast_lowerer/route.rs` / loop-pattern dev-only residue を同期した
  - verification: `rg -n "join_ir::normalized::|pub mod normalized;|ParseStringComposite|if_sum_break_route|scope_manager_bindingid_poc" src tests` = 0 hit
  - Phase R4: `Cargo.toml` の `normalized_dev` feature、mixed `#[cfg(feature = "normalized_dev")]` residue、orphan env inventory を撤去した
  - verification: `rg -n '#\\[cfg\\((not\\()?feature = \"normalized_dev\"' src` = 0 hit
  - verification: `rg -n 'feature *= *\"normalized_dev\"|normalized_dev|NYASH_JOINIR_NORMALIZED_DEV_RUN' Cargo.toml src tests` = 0 hit
  - verification: `cargo build --release --bin hakorune` PASS / `phase29bq_fast_gate_vm.sh --only bq` PASS
  - verification repair: ungated `ownership/ast_analyzer` helpers were synced (`pub(super)` sibling contract, `BlockExpr` coverage, test imports) so lib-test build and fast gate remain green after R4
  - `join_ir_vm_bridge/normalized_bridge/**` と `src/mir/join_ir/normalized/**` は removal 完了。残るのは historical docs / removal ledger の整理だけ
  - next cleanup phase (`residue cleanup`) は次の4分類で扱う:
    - `truth`: active docs / public API / runtime comments に残る旧主語を current-runtime lane から外す
    - `boundary`: no-op shim / compat helper / wrapper-only entry を削り、入口を減らす
    - `naming`: legacy file/test/comment 名を semantic 名へ寄せる
    - `dust`: unused import / dead code / orphan helper を刈る
  - cleanup order は `truth -> boundary -> naming -> dust` 固定。runtime-public lane への影響が大きいものから先に片づける
  - boundary cleanup (2026-03-06): retired promoted-binding shim を削除した
    - removed: `loop_pattern_detection/legacy/binding_map_provider.rs`
    - removed: `loop_pattern_detection/legacy/promoted_binding_recorder.rs`
    - removed: `join_ir/lowering/carrier_binding_assigner.rs`
    - synced callers: `loop_body_carrier_promoter.rs` / `loop_body_digitpos_promoter.rs` / `legacy/mod.rs` / `join_ir/lowering/mod.rs`
    - verification: `rg -n "BindingMapProvider|PromotedBindingRecorder|CarrierBindingAssigner|record_promotion" src tests docs/development/current/main/design CURRENT_TASK.md` = 0 hit
    - verification: `cargo build --release --bin hakorune` PASS / `phase29bq_fast_gate_vm.sh --only bq` PASS
  - truth cleanup (2026-03-06): active contract docs から phase-by-phase migration log を退避した
    - new historical doc: `docs/development/current/main/design/recipe-first-entry-contract-history.md`
    - slimmed active SSOT: `recipe-first-entry-contract-ssot.md`
    - reworded compatibility lane note: `recipe-tree-and-parts-ssot.md`
    - design index synced: `docs/development/current/main/design/README.md`
    - verification: `rg -n "^## (Pilot|Phase C|Appendix A)" docs/development/current/main/design/recipe-first-entry-contract-ssot.md` = 0 hit
  - truth cleanup (2026-03-06, slice 2): active design entry docs の route-first wording を明確化した
    - synced docs: `compiler-cleanliness-campaign-ssot.md` / `coreplan-migration-roadmap-ssot.md` / `boxcount-new-box-addition-checklist-ssot.md` / `plan-mod-layout-ssot.md` / `joinir-plan-frag-ssot.md`
    - intent: `legacy normalizer` / `Pattern1-9` を current architecture の主語ではなく traceability-only / compatibility-lane として明記
    - verification: wording drift grep on the synced docs only = expected hits only（`compatibility-lane`, `traceability-only legacy labels`, `transition-only residue`）
  - truth cleanup (2026-03-06, slice 3): active docs の Pattern-era wording を route-first に寄せた
    - synced docs: `environment-variables-inventory-ssot.md` / `condition-observation-ssot.md` / `strict-nested-loop-guard-ssot.md` / `join-explicit-cfg-construction.md` / `flowbox-tag-coverage-map-ssot.md` / `coreplan-shadow-adopt-tag-coverage-ssot.md`
    - intent: semantic route 名を主語にし、`Pattern*` / `pattern*` は smoke 名・tag suffix・legacy facts key・legacy env name の traceability に限定
    - verification: residual grep hits are expected only for smoke names / tag suffixes / legacy facts keys / legacy env names
  - truth cleanup (2026-03-06, slice 4): active route/Frag docs と code comments を route-first に同期した
    - synced docs: `joinir-extension-dual-route-contract-ssot.md` / `pattern6-7-contracts.md` / `planfrag-freeze-taxonomy.md` / `planfrag-ssot-registry.md` / `coreloop-generic-loop-v0-ssot.md` / `edgecfg-fragments.md` / `loop-canonicalizer.md` / `joinir-plan-frag-ssot.md`
    - synced code comments: `src/mir/policies/post_loop_early_return_plan.rs` / `src/mir/policies/balanced_depth_scan.rs` / `src/mir/loop_pattern_detection/legacy/trim_loop_helper.rs` / `src/mir/join_ir/mod.rs`
    - intent: active prose では route/semantic 名を主語にし、legacy labels は traceability-only 説明へ後退
    - verification: targeted grep on synced docs shows expected traceability-only hits; `cargo build --release --bin hakorune` PASS
  - naming cleanup (2026-03-06, slice 5): active JoinIR lowering の live debug/error tags を route 主語へ同期した
    - synced files: `src/mir/join_ir/lowering/error_tags.rs` / `simple_while_minimal.rs` / `scan_with_init_minimal.rs` / `scan_with_init_reverse.rs` / `split_scan_minimal.rs` / `scan_bool_predicate_minimal.rs` / `loop_with_if_phi_if_sum.rs`
    - intent: active `joinir/patternN` tags を `joinir/<route>` / `joinir/route/<route>` へ寄せ、近傍の current-looking `Pattern N` wording も semantic route 名へ同期する
    - verification: `cargo test --lib test_route_detection_failed_tag` PASS / `cargo check --tests` PASS / `cargo build --release --bin hakorune` PASS / `phase29bq_fast_gate_vm.sh --only bq` PASS
    - verification: `rg -n "joinir/pattern[1-9]|\\[joinir/pattern/|Pattern [13678]|Pattern[13678]" src/mir/join_ir/lowering/error_tags.rs src/mir/join_ir/lowering/simple_while_minimal.rs src/mir/join_ir/lowering/scan_with_init_minimal.rs src/mir/join_ir/lowering/scan_with_init_reverse.rs src/mir/join_ir/lowering/split_scan_minimal.rs src/mir/join_ir/lowering/scan_bool_predicate_minimal.rs src/mir/join_ir/lowering/loop_with_if_phi_if_sum.rs` = 0 hit
  - naming cleanup (2026-03-06, slice 6): `LoopPatternKind::LoopTrueEarlyExit` へ current enum surface を semantic 化し、router/classifier/lowering header の public prose を route 主語へ同期した
    - synced files: `src/mir/loop_pattern_detection/kind.rs` / `classify.rs` / `src/mir/join_ir/lowering/loop_route_router.rs` / `mod.rs`
    - intent: current-facing `InfiniteEarlyExit` / `Pattern N` wording を runtime/public surface から外し、historical numbering は traceability note と `pattern_id()` に閉じる
    - verification: `cargo build --release --bin hakorune` PASS / `cargo test --release --lib loop_pattern_detection --no-run` PASS / `phase29bq_fast_gate_vm.sh --only bq` PASS
    - verification: `rg -n "LoopPatternKind::InfiniteEarlyExit|InfiniteEarlyExit\\b|Pattern 5 \\(InfiniteEarlyExit\\)|Pattern 5: Infinite|Pattern 1 / LoopSimpleWhile|Pattern 3: Loop with If-Else PHI|Pattern 4: Loop with Continue|Pattern 6 minimal lowerer|Pattern 7 minimal lowerer|Pattern 8 minimal lowerer" src/mir/loop_pattern_detection/{kind.rs,classify.rs} src/mir/join_ir/lowering/{loop_route_router.rs,mod.rs}` = 0 hit
  - naming cleanup (2026-03-06, slice 7): condition/route helper surface を semantic 化した
    - synced files: `src/mir/join_ir/lowering/condition_pattern.rs` / `loop_with_if_phi_if_sum.rs` / `loop_view_builder.rs` / `src/mir/builder/control_flow/plan/route_prep_pipeline.rs` / `src/mir/loop_pattern_detection/features.rs`
    - intent: current-facing `IfSumComparable` / `try_pattern1` などの helper surface を route 主語へ寄せ、`Pattern3/4` 由来の近傍 prose を current runtime surface から外す
    - verification: `cargo test --lib test_capability_if_phi_join_comparable_simple` PASS / `cargo check --tests` PASS / `cargo build --release --bin hakorune` PASS / `phase29bq_fast_gate_vm.sh --only bq` PASS
    - verification: `rg -n "IfSumComparable|try_pattern1\\b|pattern3_cond_i_mod_2_eq_1_is_recognized|Pattern 3 if-sum|Pattern3/4|Pattern3 heuristic" src/mir/loop_pattern_detection/features.rs src/mir/join_ir/lowering/condition_pattern.rs src/mir/join_ir/lowering/loop_update_summary.rs src/mir/join_ir/lowering/loop_view_builder.rs src/mir/builder/control_flow/plan/route_prep_pipeline.rs src/mir/join_ir/lowering/loop_with_if_phi_if_sum.rs` = 0 hit
  - naming cleanup (2026-03-06, slice 8): `join_ir/lowering/loop_routes/**` の stub/docs/test wording と carrier/update helper docs を route-first に同期した
    - synced files: `src/mir/join_ir/lowering/loop_routes/{mod,simple_while,with_break,with_continue,with_if_phi,nested_minimal}.rs` / `src/mir/join_ir/lowering/carrier_info/types.rs` / `src/mir/join_ir/lowering/loop_update_summary.rs`
    - intent: current-facing `Pattern N` wording を route 名へ寄せ、残る numbered label は `legacy ... (traceability-only)` 注記つきの参照だけに縮退する
    - verification: `cargo test --lib test_capability_if_phi_join_comparable_simple` PASS / `cargo check --tests` PASS / `cargo build --release --bin hakorune` PASS / `phase29bq_fast_gate_vm.sh --only bq` PASS
    - verification: `rg -n "Pattern [1-9]|Pattern[1-9]|pattern[1-9]" src/mir/join_ir/lowering/loop_patterns src/mir/join_ir/lowering/loop_view_builder.rs src/mir/join_ir/lowering/carrier_info/types.rs src/mir/join_ir/lowering/loop_update_summary.rs src/mir/loop_pattern_detection/features.rs src/mir/join_ir/lowering/condition_pattern.rs` = traceability-only notes only
  - naming cleanup (2026-03-06, slice 9): lowering/common と condition entry の current-facing Pattern-era prose を route-first に同期した
    - synced files: `src/mir/join_ir/lowering/loop_with_break_minimal.rs` / `src/mir/join_ir/lowering/condition_lowering_box.rs` / `src/mir/join_ir/lowering/common/{body_local_derived_emitter,string_accumulator_emitter,dual_value_rewriter,condition_only_emitter}.rs`
    - intent: `Pattern 1/2/3/4` の current-looking 説明を `LoopSimpleWhile` / `loop_break` / `if_phi_join` / `loop_continue_only` に置き換え、残る numbered label を active lowering surface から外す
    - verification: `rg -n "Pattern [1-9]|Pattern[1-9]|pattern[1-9]" src/mir/join_ir/lowering/loop_with_break_minimal.rs src/mir/join_ir/lowering/condition_lowering_box.rs src/mir/join_ir/lowering/common/body_local_derived_emitter.rs src/mir/join_ir/lowering/common/string_accumulator_emitter.rs src/mir/join_ir/lowering/common/dual_value_rewriter.rs src/mir/join_ir/lowering/common/condition_only_emitter.rs` = 0 hit
    - verification: `cargo check --tests` PASS / `cargo build --release --bin hakorune` PASS / `phase29bq_fast_gate_vm.sh --only bq` PASS
  - naming cleanup (2026-03-06, slice 10): `loop_pattern_detection/legacy/**` の current-facing Pattern-era prose を route/body-local 主語へ同期した
    - synced files: `src/mir/loop_pattern_detection/legacy/{mod,loop_body_carrier_promoter,loop_body_digitpos_promoter,loop_body_cond_promoter,loop_condition_scope,trim_loop_helper,trim_detector,mutable_accumulator_analyzer,condition_var_analyzer,break_condition_analyzer}.rs` / `src/mir/loop_pattern_detection/legacy/function_scope_capture/analyzers/v2.rs` / `docs/development/current/main/design/ai-handoff-and-debug-contract.md`
    - intent: legacy detector/promotion box の active comments/docstrings/debug tag を `LoopSimpleWhile` / `LoopBreak` / `IfPhiJoin` / `LoopContinueOnly` と `body-local` 主語へ寄せ、numbered label は `legacy ... (traceability-only)` 注記に閉じる
    - verification: `rg -n "Pattern [1-9]|Pattern[1-9]|pattern[1-9]|loopbodylocal" src/mir/loop_pattern_detection/legacy/mod.rs src/mir/loop_pattern_detection/legacy/loop_body_carrier_promoter.rs src/mir/loop_pattern_detection/legacy/loop_body_digitpos_promoter.rs src/mir/loop_pattern_detection/legacy/loop_body_cond_promoter.rs src/mir/loop_pattern_detection/legacy/loop_condition_scope.rs src/mir/loop_pattern_detection/legacy/trim_loop_helper.rs src/mir/loop_pattern_detection/legacy/trim_detector.rs src/mir/loop_pattern_detection/legacy/mutable_accumulator_analyzer.rs src/mir/loop_pattern_detection/legacy/condition_var_analyzer.rs src/mir/loop_pattern_detection/legacy/break_condition_analyzer.rs src/mir/loop_pattern_detection/legacy/function_scope_capture/analyzers/v2.rs` = traceability-only notes / enum identifiers / legacy doc path only
    - verification: `cargo check --tests` PASS / `cargo build --release --bin hakorune` PASS / `phase29bq_fast_gate_vm.sh --only bq` PASS
  - naming cleanup (2026-03-06, slice 11): current module docs/comment の `Pattern N` residue を route-first に寄せた
    - synced files: `src/mir/loop_pattern_detection/{mod,classify}.rs` / `src/mir/builder/variable_context.rs` / `src/mir/join_ir/lowering/{simple_while,continue_branch_normalizer,loop_update_analyzer,return_collector,bool_expr_lowerer,join_value_space,canonical_names,funcscanner_append_defs,stage1_using_resolver}.rs` / `src/mir/join_ir/lowering/expr_lowerer/lowerer.rs` / `src/mir/join_ir/lowering/carrier_update_emitter/legacy.rs` / `src/mir/join_ir/lowering/loop_with_break_minimal/boundary_builder.rs` / `src/mir/join_ir/lowering/inline_boundary/{types,constructors}.rs` / `src/mir/join_ir/lowering/loop_scope_shape/case_a_lowering_shape.rs` / `src/mir/join_ir_vm_bridge/joinir_block_converter/mod.rs`
    - intent: current-facing docs/comment で `Pattern N` を architecture の主語にせず、`LoopSimpleWhile` / `LoopBreak` / `IfPhiJoin` / `LoopContinueOnly` などの route 名へ寄せる。numbered label は traceability-only 注記が必要な箇所に限定する
    - verification: `rg -n "Pattern [1-9]|Pattern[1-9]\\b|pattern[1-9]\\b|pattern[1-9]_" src/mir/loop_pattern_detection/mod.rs src/mir/builder/variable_context.rs src/mir/join_ir/lowering/simple_while.rs src/mir/join_ir_vm_bridge/joinir_block_converter/mod.rs src/mir/loop_pattern_detection/classify.rs src/mir/join_ir/lowering/funcscanner_append_defs.rs src/mir/join_ir/lowering/stage1_using_resolver.rs src/mir/join_ir/lowering/continue_branch_normalizer.rs src/mir/join_ir/lowering/loop_update_analyzer.rs src/mir/join_ir/lowering/return_collector.rs src/mir/join_ir/lowering/bool_expr_lowerer.rs src/mir/join_ir/lowering/expr_lowerer/lowerer.rs src/mir/join_ir/lowering/join_value_space.rs src/mir/join_ir/lowering/carrier_update_emitter/legacy.rs src/mir/join_ir/lowering/loop_with_break_minimal/boundary_builder.rs src/mir/join_ir/lowering/inline_boundary/types.rs src/mir/join_ir/lowering/inline_boundary/constructors.rs src/mir/join_ir/lowering/loop_scope_shape/case_a_lowering_shape.rs src/mir/join_ir/lowering/canonical_names.rs` = traceability-only notes only
    - verification: `cargo check --tests` PASS / `cargo build --release --bin hakorune` PASS / `phase29bq_fast_gate_vm.sh --only bq` PASS
  - naming cleanup (2026-03-06, slice 12): `merge/**` と extractor helper の current-facing `Pattern N` prose を route-first に寄せた
    - synced files: `src/mir/builder/control_flow/joinir/merge/{loop_header_phi_info,entry_selector,exit_args_collector}.rs` / `src/mir/builder/control_flow/joinir/merge/exit_line/{meta_collector,mod}.rs` / `src/mir/builder/control_flow/joinir/merge/contract_checks/entry_params.rs` / `src/mir/builder/control_flow/joinir/merge/coordinator/{mod,phase_5_6}.rs` / `src/mir/builder/control_flow/plan/extractors/{common_helpers,loop_simple_while,if_phi_join}.rs` / `src/mir/policies/post_loop_early_return_plan.rs` / `src/mir/join_ir/ownership/plan_validator.rs`
    - intent: merge/extractor/policy/ownership の current prose では route semantics を主語にし、numbered pattern は `legacy ... (traceability-only)` が必要な箇所だけに残す
    - verification: `rg -n "Pattern [1-9]|Pattern[1-9]\\b|pattern[1-9]\\b|pattern[1-9]_" src/mir/builder/control_flow/joinir/merge src/mir/builder/control_flow/plan/extractors/common_helpers.rs src/mir/builder/control_flow/plan/extractors/loop_simple_while.rs src/mir/builder/control_flow/plan/extractors/if_phi_join.rs -g '!src/**/tests.rs' -g '!src/**/test*.rs'` = traceability-only notes only
    - verification: `cargo check --tests` PASS / `cargo build --release --bin hakorune` PASS / `phase29bq_fast_gate_vm.sh --only bq` PASS
  - naming cleanup (2026-03-06, slice 13): current route docs の historical note をさらに薄くし、loop_break/loop_patterns の legacy label comment を route-first に同期した
    - synced files: `src/mir/loop_pattern_detection/{mod,kind,classify}.rs` / `src/mir/builder/control_flow/plan/route_prep_pipeline.rs` / `src/mir/builder/control_flow/plan/{loop_break_prep_box,condition_env_builder}.rs` / `src/mir/builder/control_flow/plan/common/carrier_binding_policy.rs` / `src/mir/builder/control_flow/plan/loop_break/api/{promote_decision,promote_runner}.rs` / `src/mir/builder/control_flow/plan/loop_break/contracts/derived_slot.rs` / `src/mir/join_ir/lowering/loop_routes/{mod,simple_while,with_break,with_continue,with_if_phi,nested_minimal}.rs`
    - intent: current-facing enum/module docs では route 名だけを主語にし、pattern 番号の traceability note を必要最小限の場所へさらに押し戻す
    - verification: `rg -n "Pattern [1-9]|Pattern[1-9]\\b|pattern[1-9]\\b|pattern[1-9]_" src/mir/loop_pattern_detection/mod.rs src/mir/loop_pattern_detection/kind.rs src/mir/loop_pattern_detection/classify.rs src/mir/builder/control_flow/plan/route_prep_pipeline.rs src/mir/join_ir/lowering/loop_patterns src/mir/builder/control_flow/plan/loop_break/api src/mir/builder/control_flow/plan/loop_break/contracts src/mir/builder/control_flow/plan/loop_break_prep_box.rs src/mir/builder/control_flow/plan/condition_env_builder.rs src/mir/builder/control_flow/plan/common/carrier_binding_policy.rs -g '!src/**/tests.rs' -g '!src/**/test*.rs'` = 0 hit
    - verification: `cargo check --tests` PASS / `cargo build --release --bin hakorune` PASS / `phase29bq_fast_gate_vm.sh --only bq` PASS
  - naming cleanup (2026-03-06, slice 14): current-facing route prose の numbered traceability note をさらに削り、`simple_while.rs` の API 名も route-first に寄せた
    - synced files: `src/mir/join_ir_vm_bridge/joinir_block_converter/mod.rs` / `src/mir/join_ir/lowering/{carrier_info/types,simple_while,loop_view_builder}.rs` / `src/mir/builder/control_flow/plan/{mod,ast_feature_extractor,conversion_pipeline,loop_true_counter_extractor}.rs` / `src/mir/builder/control_flow/plan/policies/policies/{mod,loop_true_read_digits_policy}.rs` / `src/mir/builder/control_flow/joinir/merge/{loop_header_phi_info,entry_selector,exit_args_collector}.rs` / `src/mir/builder/control_flow/joinir/merge/exit_line/{meta_collector,mod}.rs` / `src/mir/builder/control_flow/joinir/merge/coordinator/{mod,phase_5_6}.rs` / `src/mir/builder/control_flow/joinir/merge/contract_checks/entry_params.rs`
    - intent: runtime 近傍の docs/comment では route 名だけを前面に出し、historical pattern 番号は `legacy/**` や traceability-only ledger にさらに押し戻す
    - verification: `rg -n "legacy Pattern|legacy label: Pattern|Pattern [1-9]|Pattern[1-9]\\b|pattern[1-9]_" src/mir/join_ir_vm_bridge/joinir_block_converter/mod.rs src/mir/join_ir/lowering/carrier_info/types.rs src/mir/join_ir/lowering/simple_while.rs src/mir/join_ir/lowering/loop_view_builder.rs src/mir/builder/control_flow/plan/mod.rs src/mir/builder/control_flow/plan/ast_feature_extractor.rs src/mir/builder/control_flow/plan/conversion_pipeline.rs src/mir/builder/control_flow/plan/policies/policies/mod.rs src/mir/builder/control_flow/plan/policies/policies/loop_true_read_digits_policy.rs src/mir/builder/control_flow/plan/loop_true_counter_extractor.rs src/mir/builder/control_flow/joinir/merge/loop_header_phi_info.rs src/mir/builder/control_flow/joinir/merge/exit_line/meta_collector.rs src/mir/builder/control_flow/joinir/merge/exit_line/mod.rs src/mir/builder/control_flow/joinir/merge/coordinator/mod.rs src/mir/builder/control_flow/joinir/merge/coordinator/phase_5_6.rs src/mir/builder/control_flow/joinir/merge/entry_selector.rs src/mir/builder/control_flow/joinir/merge/exit_args_collector.rs src/mir/builder/control_flow/joinir/merge/contract_checks/entry_params.rs` = 0 hit
    - verification: `cargo check --tests` PASS / `cargo build --release --bin hakorune` PASS / `phase29bq_fast_gate_vm.sh --only bq` PASS
  - naming cleanup (2026-03-06, slice 15): `normalizer` の current-facing residue をさらに薄くし、simple-while coreloop helper の physical file 名も semantic に揃えた
    - synced files: `src/mir/join_ir/lowering/mod.rs` / `src/mir/builder/control_flow/plan/normalizer/{mod,helpers,README,simple_while_coreloop_builder}.rs` / `docs/development/current/main/design/compiler-cleanliness-campaign-ssot.md`
    - intent: non-legacy current code から `pattern1_coreloop_builder.rs` / `pattern2_step_schedule` の traceability string を外し、helper layout comment も route semantics だけに戻す
    - verification: `rg -n "Pattern [1-9]|Pattern[1-9]\\b|pattern[1-9]\\b|pattern[1-9]_" src/mir/join_ir/lowering/mod.rs src/mir/builder/control_flow/plan/normalizer src/mir/builder/control_flow/plan/REGISTRY.md docs/development/current/main/design/compiler-cleanliness-campaign-ssot.md -g '!**/REGISTRY.md'` = legacy-only hits or 0 hit on touched current files
    - verification: `cargo check --tests` PASS / `cargo build --release --bin hakorune` PASS / `phase29bq_fast_gate_vm.sh --only bq` PASS
  - naming cleanup (2026-03-06, slice 16): current-facing README の route prose を整理し、`Pattern` 主語を active guidance からさらに後退させた
    - synced files: `src/mir/builder/control_flow/plan/policies/policies/README.md` / `src/mir/builder/control_flow/plan/features/README.md`
    - intent: current README では route/policy decision を主語にし、historical file 名は physical path 注記に限定する
    - verification: `rg -n "Pattern [1-9]|Pattern[1-9]\\b|pattern[1-9]\\b|pattern[1-9]_" src/mir/builder/control_flow/plan/policies/policies/README.md src/mir/builder/control_flow/plan/features/README.md` = physical path note only
  - naming cleanup (2026-03-06, slice 17): `legacy` route detector の public helper 名を semantic に寄せ、route examples も追従した
    - synced files: `src/mir/loop_pattern_detection/legacy/mod.rs` / `src/mir/join_ir/lowering/loop_routes/{simple_while,with_break,with_if_phi,with_continue}.rs`
    - intent: `legacy/` 配下でも production helper surface では `*_pattern` を使わず、route 名をそのまま helper 名にする
    - verification: `rg -n "is_simple_while_pattern|is_loop_with_break_route|is_loop_with_conditional_phi_pattern|is_loop_with_continue_route" src/mir/loop_pattern_detection/legacy/mod.rs src/mir/join_ir/lowering/loop_patterns` = 0 hit
    - verification: `cargo check --tests` PASS / `cargo build --release --bin hakorune` PASS / `phase29bq_fast_gate_vm.sh --only bq` PASS
  - naming cleanup (2026-03-06, slice 18): test-only `Pattern2/3/4` residue を route 名へ揃えた
    - synced files: `src/mir/join_ir/lowering/inline_boundary_builder.rs` / `src/mir/join_ir/lowering/expr_lowerer/tests.rs`
    - intent: test 名と test comment でも route semantics を主語にし、numbered label を repo 内部テスト surface から外す
    - verification: `rg -n "pattern3_style|pattern4_style|pattern2_break_digit_pos_less_zero" src/mir/join_ir/lowering` = 0 hit
    - verification: `cargo check --tests` PASS / `cargo build --release --bin hakorune` PASS / `phase29bq_fast_gate_vm.sh --only bq` PASS
  - dust cleanup (2026-03-07, slice 19): low-risk warning sourceの unused import / dead helper / unhooked module を削った
    - synced files: `src/mir/builder.rs` / `src/mir/builder/control_flow/plan/{normalizer,verifier}/mod.rs` / `src/mir/builder/control_flow/joinir/trace.rs` / `src/mir/builder/control_flow/debug.rs` / `src/mir/builder/control_flow/edgecfg/api/frag.rs`
    - deleted: `src/mir/builder/loop_frontend_binding.rs`
    - intent: current runtime で未接続の helper と未使用 re-export を消し、warning を挙動不変で減らす
    - verification: `cargo check --tests` PASS / `cargo build --release --bin hakorune` PASS / `phase29bq_fast_gate_vm.sh --only bq` PASS
  - dust cleanup (2026-03-07, slice 20): test-only loop context と deprecated normalization residue を current contract に揃えた
    - synced files: `src/mir/builder/control_flow/joinir/{loop_context,routing}.rs` / `src/mir/builder/control_flow/normalization/{plan,execute_box}.rs` / `src/mir/builder/control_flow/normalization/README.md` / `src/mir/builder/control_flow/plan/policies/policies/normalized_shadow_suffix_router_box.rs` / `docs/development/current/main/design/normalized-expr-lowering.md`
    - intent: `LoopProcessingContext` を実際に読まれる field だけへ細くし、statement-level normalization 後も残っていた `LoopWithPost` current code を撤去する
    - verification: `cargo check --tests` PASS（warning 0） / `cargo build --release --bin hakorune` PASS / `phase29bq_fast_gate_vm.sh --only bq` PASS
  - truth cleanup (2026-03-07, slice 21): current-facing README / SSOT の `Pattern` wording を route-first に寄せた
    - synced files: `src/mir/control_tree/normalized_shadow/anf/README.md` / `src/mir/builder/control_flow/plan/REGISTRY.md` / `src/mir/builder/control_flow/plan/features/README.md` / `docs/development/current/main/design/{loop-canonicalizer,joinir-design-map,coreplan-skeleton-feature-model}.md`
    - intent: fixture key と physical file 名は残したまま、今読む guidance では `Pattern` ではなく route / shape / feature を主語にする
    - verification: `rg -n "Pattern Router|Pattern Lowerer|RoutingDecision\\(Pattern2\\)|Pattern 1:|Pattern 2:|Pattern2 の policy" src/mir/control_tree/normalized_shadow/anf/README.md src/mir/builder/control_flow/plan/REGISTRY.md src/mir/builder/control_flow/plan/features/README.md docs/development/current/main/design/{loop-canonicalizer,joinir-design-map,coreplan-skeleton-feature-model}.md` = 0 hit
  - truth cleanup (2026-03-07, slice 22): tests / smoke scripts の current-facing Pattern prose を route-first に寄せた
    - synced files: `tests/{phase245_json_parse_number,phase246_json_atoi}.rs` / `tools/smokes/v2/profiles/integration/joinir/{phase29ao_pattern1_strict_shadow_vm,phase29ao_pattern5_strict_shadow_vm}.sh` / `tools/smokes/v2/profiles/integration/selfhost/{selfhost_mir_min_vm,selfhost_minimal}.sh`
    - intent: file 名や fixture 名は保持したまま、test 名・doc comment・skip reason comment では `loop_break` / `loop_simple_while` / `loop_true_early_exit` を主語にする
    - verification: `cargo check --tests` PASS / `rg -n "json_parser_min_runs_via_joinir_pattern2_path|Pattern 2 \\(Break\\)|DomainPlan Pattern1|Pattern1 strict shadow|Pattern5 strict shadow|selfhost_mir_min_vm: Pattern 1|selfhost_minimal: Pattern 1|selfhost_minimal: Pattern 4|Pattern 6 \\(NestedLoop Minimal\\)" tests/phase245_json_parse_number.rs tests/phase246_json_atoi.rs tools/smokes/v2/profiles/integration/joinir/phase29ao_pattern1_strict_shadow_vm.sh tools/smokes/v2/profiles/integration/joinir/phase29ao_pattern5_strict_shadow_vm.sh tools/smokes/v2/profiles/integration/selfhost/selfhost_mir_min_vm.sh tools/smokes/v2/profiles/integration/selfhost/selfhost_minimal.sh` = 0 hit
  - route allowlist note (2026-03-07): `join_ir/frontend/ast_lowerer/route.rs` の fixture key は by-name allowlist 契約として扱う
    - legacy key: `pattern3_if_sum_multi_min` / `jsonparser_if_sum_min` / `selfhost_if_sum_p3` / `selfhost_if_sum_p3_ext`
    - semantic alias: `if_phi_join_multi_min` / `jsonparser_if_phi_join_min` / `selfhost_if_phi_join` / `selfhost_if_phi_join_ext`
    - rule: direct rename/delete は避ける。Phase B/C で semantic alias と managed assets migration を先に済ませ、Phase D で old key を retire する
    - rationale: `lower_program_json()` が `defs[0].name` を `resolve_function_route()` へ直結するため、runtime 契約は alias-first でしか安全に畳めない
  - compat retirement setup (2026-03-07, phase A): legacy by-name fixture key を独立フェーズとして固定した
    - SSOT: `docs/development/current/main/design/joinir-frontend-legacy-fixture-key-retirement-ssot.md`
    - scope: `src/mir/join_ir/frontend/ast_lowerer/route.rs` の `pattern3_if_sum_multi_min` / `jsonparser_if_sum_min` / `selfhost_if_sum_p3` / `selfhost_if_sum_p3_ext`
    - rule: A=inventory/decision, B=alias追加, C=fixture/doc移行, D=旧key retire。いきなり rename/delete はしない
    - rationale: active tests は薄いが private/historical JSON fixtures が `name` を pin しているため、互換契約を段階的に畳む必要がある
  - compat retirement (2026-03-07, phase B): semantic alias を追加し、old/new 両 key を同じ route へ固定した
    - synced files: `src/mir/join_ir/frontend/ast_lowerer/route.rs` / `docs/development/current/main/design/joinir-frontend-legacy-fixture-key-retirement-ssot.md`
    - alias map:
      - `pattern3_if_sum_multi_min` -> `if_phi_join_multi_min`
      - `jsonparser_if_sum_min` -> `jsonparser_if_phi_join_min`
      - `selfhost_if_sum_p3` -> `selfhost_if_phi_join`
      - `selfhost_if_sum_p3_ext` -> `selfhost_if_phi_join_ext`
    - intent: old key を壊さずに semantic key を先行導入し、fixture/doc migration の受け皿を作る
    - verification: `cargo test --lib legacy_and_semantic_if_phi_join_fixture_keys_resolve_to_loop_frontend` PASS / `cargo check --tests` PASS / `cargo build --release --bin hakorune` PASS / `phase29bq_fast_gate_vm.sh --only bq` PASS
  - compat retirement (2026-03-07, phase C): managed private fixtures/docs を semantic alias 側へ移行した
    - renamed fixtures:
      - `docs/private/roadmap2/phases/normalized_dev/fixtures/if_phi_join_multi_min.program.json`
      - `docs/private/roadmap2/phases/normalized_dev/fixtures/jsonparser_if_phi_join_min.program.json`
      - `docs/private/roadmap2/phases/normalized_dev/fixtures/selfhost_if_phi_join.program.json`
      - `docs/private/roadmap2/phases/normalized_dev/fixtures/selfhost_if_phi_join_ext.program.json`
    - synced docs:
      - `docs/private/development/current/main/joinir-architecture-overview.md`
      - `docs/private/development/current/main/phase47-norm-p3-design.md`
      - `docs/private/development/current/main/phase49-selfhost-joinir-depth2-design.md`
      - `docs/private/development/current/main/PHASE_61_SUMMARY.md`
      - `docs/development/current/main/design/joinir-frontend-legacy-fixture-key-retirement-ssot.md`
    - intent: in-repo managed fixture/doc 参照を semantic key へ寄せ、old key の残りを `route.rs` / retirement ledger / archive-history に縮める
    - verification: `rg -n "pattern3_if_sum_multi_min|jsonparser_if_sum_min|selfhost_if_sum_p3|selfhost_if_sum_p3_ext" src tests tools docs/development/current/main docs/private CURRENT_TASK.md` = `route.rs` / retirement docs / archive only
    - verification: `rg -n "if_phi_join_multi_min|jsonparser_if_phi_join_min|selfhost_if_phi_join|selfhost_if_phi_join_ext" docs/private/development/current/main docs/private/roadmap2/phases/normalized_dev/fixtures docs/development/current/main/design/joinir-frontend-legacy-fixture-key-retirement-ssot.md` = semantic fixtures/docs only
  - compat retirement (2026-03-07, phase D): `route.rs` から old key を retire し、runtime contract を semantic key のみに収束させた
    - synced files: `src/mir/join_ir/frontend/ast_lowerer/route.rs` / `docs/development/current/main/design/joinir-frontend-legacy-fixture-key-retirement-ssot.md`
    - runtime contract:
      - accepted: `if_phi_join_multi_min` / `jsonparser_if_phi_join_min` / `selfhost_if_phi_join` / `selfhost_if_phi_join_ext`
      - retired: `pattern3_if_sum_multi_min` / `jsonparser_if_sum_min` / `selfhost_if_sum_p3` / `selfhost_if_sum_p3_ext`
    - intent: live by-name contract から pattern-era key を外し、repo 内 managed assets と runtime entry を semantic 名へ揃える
    - verification: `rg -n "pattern3_if_sum_multi_min|jsonparser_if_sum_min|selfhost_if_sum_p3|selfhost_if_sum_p3_ext" src tests tools docs/development/current/main docs/private CURRENT_TASK.md` = `CURRENT_TASK` / retirement SSOT / archive / `route.rs` rejection test only
    - verification: `cargo test --lib semantic_if_phi_join_fixture_keys_resolve_to_loop_frontend` PASS / `cargo test --lib retired_legacy_if_phi_join_fixture_keys_are_rejected` PASS / `cargo build --release --bin hakorune` PASS / `phase29bq_fast_gate_vm.sh --only bq` PASS
  - truth cleanup (2026-03-07, slice 23): low-risk active docs の stale path / Pattern wording を route-first に寄せた
    - synced files: `docs/development/current/main/design/{coreloop-stepmode-inline-in-body-ssot,pattern-p5b-escape-design,coreplan-skeleton-feature-model,joinir-pattern-selection-shadow-ssot}.md`
    - intent: 現在参照される設計文では semantic route / family を主語にし、stale source path も今の file 名へ合わせる
    - verification: `rg -n "normalizer/pattern1_simple_while\\.rs|Pattern 1-4|Pattern 1, Canonicalizer picks P5b|Reuse Pattern 1-2 lowering|plan 側の Pattern1|pattern1_\\* variants" docs/development/current/main/design/{coreloop-stepmode-inline-in-body-ssot,pattern-p5b-escape-design,coreplan-skeleton-feature-model,joinir-pattern-selection-shadow-ssot}.md` = 0 hit
  - truth cleanup (2026-03-07, slice 24): smoke script header/comment の current-facing Pattern prose を route-first に寄せた
    - synced files: `tools/smokes/v2/profiles/integration/joinir/{phase29ao_pattern{1,2,3,5,6,7}_*,phase29ae_pattern{6,7}_*,phase29ap_pattern4_continue_min_vm.sh,phase29ap_pattern6_nested_*,phase29bi_planner_required_pattern2_pack_vm.sh,phase29bl_planner_required_pattern1_4_5_pack_vm.sh,phase29bn_planner_required_pattern3_pack_vm.sh,phase29bo_planner_required_pattern8_9_pack_vm.sh,phase286_pattern9_legacy_pack.sh}.sh` / `tools/smokes/v2/profiles/integration/selfhost/{selfhost_mir_min_vm,selfhost_minimal}.sh`
    - intent: script 名と fixture 名はそのまま残し、header comment / skip reason comment では `loop_simple_while` / `loop_break` / `if_phi_join` / `loop_continue_only` / `loop_true_early_exit` / `scan_with_init` / `split_scan` / `nested_loop_minimal` / `accum_const_loop` を主語にする
    - verification: `rg -n -g '*.sh' -- "- Pattern[1-9]|Pattern 1 \\(JoinIR loop route gap|Pattern 4 \\(OS limitation|Pattern 6 \\(NestedLoop Minimal\\)|Ensure Pattern1 subset" tools/smokes/v2/profiles/integration/joinir tools/smokes/v2/profiles/integration/selfhost` = 0 hit
  - truth cleanup (2026-03-07, slice 25): stale path inventory を現行 tree に同期した
    - synced files: `src/mir/builder/control_flow/plan/{REGISTRY.md,features/README.md}` / `docs/development/current/main/design/recipe-tree-and-parts-ssot.md`
    - intent: 既に撤去済みの `features/pattern5_infinite_early_exit_{pipeline,ops}.rs` と `normalizer/pattern8_bool_predicate_scan.rs` / `normalizer/pattern9_accum_const_loop.rs` を active inventory から外し、`loop_true_early_exit` の current implementation を recipe-tree 側へ合わせる
    - verification: `rg -n "pattern5_infinite_early_exit_pipeline|pattern5_infinite_early_exit_ops|normalizer/pattern8_bool_predicate_scan|normalizer/pattern9_accum_const_loop" src/mir/builder/control_flow/plan docs/development/current/main/design/recipe-tree-and-parts-ssot.md` = archive / history を除いて 0 hit
  - truth cleanup (2026-03-07, slice 26): active design の stale heading / stale file path を current route tree に同期した
    - synced files: `docs/development/current/main/design/{entry-name-map-ssot,plan-dir-shallowing-ssot,condprofile-migration-plan-ssot,compiler-task-map-ssot}.md`
    - intent: stale `Pattern6/7/8/9*` RuleId 名、`recipe_tree/pattern*_builder.rs`、`pattern1_minimal.rs` / `pattern8_scan_bool_predicate.rs` / `pattern2_policy_router.rs` などの旧 path を current semantic name / current file path に寄せる
    - verification: `rg -n "Pattern6ScanWithInit|Pattern7SplitScan|Pattern8BoolPredicateScan|Pattern9AccumConstLoop|recipe_tree/pattern\\*_builder|pattern1_minimal\\.rs|pattern8_scan_bool_predicate\\.rs|pattern2_policy_router\\.rs|pattern2_break_condition_policy_router\\.rs" docs/development/current/main/design/{entry-name-map-ssot,plan-dir-shallowing-ssot,condprofile-migration-plan-ssot,compiler-task-map-ssot}.md` = 0 hit
  - truth cleanup (2026-03-07, slice 27): active design の route-first wording を追加で同期した
    - synced files: `docs/development/current/main/design/{domainplan-thinning-ssot,condition-observation-ssot,coreloop-composer-v0-v1-boundary-ssot,coreplan-unknown-loop-strategy-ssot,joinir-plan-frag-ssot,loop-canonicalizer,plan-mod-layout-ssot}.md`
    - intent: current guidance から `PatternX` 主語を外し、route/capability/skeleton を本文の主語に寄せる。番号付き label は traceability-only note へ後退させる
    - verification: `rg -n "legacy label: PatternX|対応Pattern|選択された Pattern|Pattern 検出|Pattern1–N|Pattern1-9 は traceability-only legacy labels|Pattern6\\)|Pattern2/3/5/7" docs/development/current/main/design/{domainplan-thinning-ssot,condition-observation-ssot,coreloop-composer-v0-v1-boundary-ssot,coreplan-unknown-loop-strategy-ssot,joinir-plan-frag-ssot,loop-canonicalizer,plan-mod-layout-ssot}.md` = 0 hit
  - truth cleanup (2026-03-07, slice 28): active design の stale migration prose / stale JoinIR map を current runtime に同期した
    - synced files: `docs/development/current/main/design/{coreplan-migration-roadmap-ssot,joinir-design-map,condprofile-migration-plan-ssot,plan-dir-shallowing-ssot,generic-loop-v1-acceptance-by-recipe-ssot}.md`
    - intent: `pattern5` などの stale route 名、removed `join_ir/normalized` lane、`pattern1/8/9 facts` の旧語彙を current route tree / recipe-first runtime に合わせる
    - verification: `rg -n "pattern5|JoinIR \\(Normalized\\)|join_ir/normalized/shape_guard|Pattern router|pattern1 / pattern8 / pattern9 facts|legacy label: Pattern1|legacy label: PatternX" docs/development/current/main/design/{coreplan-migration-roadmap-ssot,joinir-design-map,condprofile-migration-plan-ssot,plan-dir-shallowing-ssot,generic-loop-v1-acceptance-by-recipe-ssot}.md` = 0 hit
  - truth cleanup (2026-03-07, slice 29): active design の numbered-label note をさらに薄くした
    - synced files: `docs/development/current/main/design/{loop-canonicalizer,plan-dir-shallowing-ssot,coreplan-migration-roadmap-ssot}.md`
    - intent: current heading / section title から `Pattern1/2/3/4/5` の直接言及を外し、残る legacy note を “numbered label” へ一般化する
    - verification: `rg -n "LoopSimpleWhile family \\(legacy label: Pattern1\\)|LoopBreak family \\(legacy label: Pattern2\\)|IfPhiJoin route \\(legacy label: Pattern3\\)|LoopContinueOnly route \\(legacy label: Pattern4\\)|LoopTrueEarlyExit family \\(legacy label: Pattern5|Pattern1-9 は traceability-only|Pattern1/2/4/5" docs/development/current/main/design/{loop-canonicalizer,plan-dir-shallowing-ssot,coreplan-migration-roadmap-ssot}.md` = 0 hit
  - truth cleanup (2026-03-07, slice 30): active design の numbered-label phrasing を追加で一般化した
    - synced files: `docs/development/current/main/design/{edgecfg-fragments,compiler-task-map-ssot}.md`
    - intent: current guidance に見える `Pattern2/6/7/8/9` の直接言及を “old numbered label / legacy bridge label” へ後退させ、route-first の本文を維持する
    - verification: `rg -n "legacy label: Pattern2|Pattern9_AccumConstLoop|Pattern2,|Pattern8,|Pattern\\*|PatternN|Pattern6/7" docs/development/current/main/design/{edgecfg-fragments,compiler-task-map-ssot}.md` = 0 hit
  - truth cleanup (2026-03-07, slice 31): active design の traceability-only note と stale Normalized prose をさらに薄くした
    - synced files: `docs/development/current/main/design/{condition-observation-ssot,domainplan-thinning-ssot,planfrag-freeze-taxonomy,join-explicit-cfg-construction}.md`
    - intent: specific `Pattern1CharMap/Pattern8` などの traceability note を “legacy numbered label” へ一般化し、`Normalized SSOT` を current route-first / contract-checked wording に寄せる
    - verification: `rg -n "Pattern1CharMap|Pattern1ArrayJoin|Pattern8BoolPredicateScan|Pattern9AccumConstLoop|Pattern6/7|Normalized SSOT|Structured → Normalized" docs/development/current/main/design/{condition-observation-ssot,domainplan-thinning-ssot,planfrag-freeze-taxonomy,join-explicit-cfg-construction}.md` = 0 hit
  - truth cleanup (2026-03-07, slice 32): active design の general guidance から `Pattern*` 一般論を外した
    - synced files: `docs/development/current/main/design/{recipe-first-entry-contract-ssot,coreplan-skeleton-feature-model,compiler-cleanliness-campaign-ssot,strict-nested-loop-guard-ssot}.md`
    - intent: architecture 説明で `Pattern*` を一般概念として使わず、legacy numbered label / legacy facts key へ縮退させる
    - verification: `rg -n "Historical labels \\(Pattern\\*|“Pattern” は入口の分岐名|LoopCond\\*.*Pattern\\*|pattern4_continue" docs/development/current/main/design/{recipe-first-entry-contract-ssot,coreplan-skeleton-feature-model,compiler-cleanliness-campaign-ssot,strict-nested-loop-guard-ssot}.md` = 0 hit
  - truth cleanup (2026-03-07, slice 33): active design の `legacy pattern` 文言を `legacy numbered route label` へ一般化した
    - synced files: `docs/development/current/main/design/{joinir-design-map,edgecfg-fragments,condprofile-migration-plan-ssot}.md`
    - intent: active guidance で `pattern` を architecture 概念として使わず、旧番号ラベルは numbered route label / bridge label としてだけ扱う
    - verification: `rg -n "numbered pattern label|legacy pattern|Pattern1CharMap|Pattern1ArrayJoin|Pattern8/9" docs/development/current/main/design/{joinir-design-map,edgecfg-fragments,condprofile-migration-plan-ssot}.md` = 0 hit
  - truth cleanup (2026-03-07, slice 34): active SSOT の legacy-note wording を numbered route label に寄せた
    - synced files: `docs/development/current/main/design/{joinir-plan-frag-ssot,planfrag-ssot-registry,recipe-first-entry-contract-ssot,loop-canonicalizer}.md`
    - intent: route-first な現役 guidance では `Pattern*` を general term にせず、必要な traceability note だけ numbered route label として残す
    - verification: `rg -n "legacy pattern labels|Pattern\\* labels|legacy pattern space|legacy labels Pattern6/7" docs/development/current/main/design/{joinir-plan-frag-ssot,planfrag-ssot-registry,recipe-first-entry-contract-ssot,loop-canonicalizer}.md` = 0 hit
  - truth cleanup (2026-03-07, slice 35): retired/contract docs の `Pattern` 名詞を route/shape 主語へ薄くした
    - synced files: `docs/development/current/main/design/{pattern6-7-contracts,joinir-pattern-selection-shadow-ssot,json-v0-bridge-lowering-split-ssot}.md`
    - intent: current guidance では `Pattern` を retired title や legacy window の名詞にし続けず、必要な traceability は numbered route label / legacy shape window に寄せる
    - verification: `rg -n "Pattern Selection Shadow|legacy pattern windows|legacy label Pattern6|legacy label Pattern7" docs/development/current/main/design/{pattern6-7-contracts,joinir-pattern-selection-shadow-ssot,json-v0-bridge-lowering-split-ssot}.md` = 0 hit
  - truth cleanup (2026-03-07, slice 36): `pattern-p5b-escape-design` の現役本文を escape-route 主語へ寄せた
    - synced files: `docs/development/current/main/design/pattern-p5b-escape-design.md`
    - intent: P5b の traceability は残しつつ、active design 本文では `Pattern P5b` を architecture の主語にせず `escape route P5b` / route contract / route detection へ寄せる
    - verification: `rg -n "Pattern P5b|Pattern Definition|P5b Pattern|Pattern5b|Pattern P5c|Pattern P5d|No pattern matched" docs/development/current/main/design/pattern-p5b-escape-design.md` = 0 hit
  - naming cleanup (2026-03-07, slice 37): normalization 層の current-facing `Pattern detection` wording を `shape detection` へ寄せた
    - synced files: `src/mir/builder/control_flow/normalization/{plan_box.rs,plan.rs,README.md}` / `src/mir/builder/control_flow/plan/policies/policies/normalized_shadow_suffix_router_box.rs`
    - intent: Normalized shadow の現役 guide/comment では `Pattern` を detection の主語にせず、shape/contract を前面に出す
    - verification: `rg -n "Pattern detection|Pattern detected|Pattern not matched|Pattern: Single loop statement|## Pattern Detection|pattern explosion" src/mir/builder/control_flow/normalization src/mir/builder/control_flow/plan/policies/policies/normalized_shadow_suffix_router_box.rs` = 0 hit
  - naming cleanup (2026-03-07, slice 38): selfhost fixture/smoke comment の pattern-era wording を route-first に寄せた
    - synced files: `tools/selfhost/{test_jsonparser_match_literal,test_bundleresolver_merge,test_pattern1_simple,test_pattern2_parse_number,test_pattern2_search,test_pattern3_skip_whitespace,test_pattern3_trim_leading,test_pattern3_trim_trailing,test_pattern3_if_phi_no_break,test_pattern4_parse_string,test_pattern4_parse_array,test_pattern4_simple_continue,test_pattern4_continue_return_minimal,test_pattern5b_escape_minimal}.hako` / `tools/smokes/v2/profiles/integration/selfhost/{selfhost_mir_min_vm,selfhost_minimal}.sh`
    - intent: file 名はそのまま残しつつ、header/comment/skip message では `loop_simple_while` / `loop_break` / `if_phi_join-style` / `loop_continue_only-style` / `escape route P5b` を主語にする
    - verification: `rg -n "Pattern1|Pattern2|Pattern3|Pattern4|Pattern P5b|Pattern:" tools/selfhost/test_jsonparser_match_literal.hako tools/selfhost/test_bundleresolver_merge.hako tools/selfhost/test_pattern1_simple.hako tools/selfhost/test_pattern2_parse_number.hako tools/selfhost/test_pattern2_search.hako tools/selfhost/test_pattern3_skip_whitespace.hako tools/selfhost/test_pattern3_trim_leading.hako tools/selfhost/test_pattern3_trim_trailing.hako tools/selfhost/test_pattern3_if_phi_no_break.hako tools/selfhost/test_pattern4_parse_string.hako tools/selfhost/test_pattern4_parse_array.hako tools/selfhost/test_pattern4_simple_continue.hako tools/selfhost/test_pattern4_continue_return_minimal.hako tools/selfhost/test_pattern5b_escape_minimal.hako tools/smokes/v2/profiles/integration/selfhost/selfhost_mir_min_vm.sh tools/smokes/v2/profiles/integration/selfhost/selfhost_minimal.sh` = 0 hit
  - truth cleanup (2026-03-07, slice 39): tag/script inventory docs の semantic route と legacy file/tag token を列で分離した
    - synced files: `docs/development/current/main/design/{coreplan-shadow-adopt-tag-coverage-ssot,flowbox-tag-coverage-map-ssot,boxcount-new-box-addition-checklist-ssot}.md`
    - intent: active inventory では semantic scenario を主語にし、pattern-era token は smoke stem / tag suffix / fixture key の traceability 列へ隔離する
    - verification: `rg -n "Scenario \\(smoke\\)|legacy smoke label|phase118_pattern3_if_sum_min" docs/development/current/main/design/{coreplan-shadow-adopt-tag-coverage-ssot,flowbox-tag-coverage-map-ssot,boxcount-new-box-addition-checklist-ssot}.md` = expected legacy fixture/tag references only
  - naming cleanup (2026-03-07, slice 40): `loop_canonicalizer` と近傍 lower comment の current-facing `Pattern` wording を route-shape 主語へ寄せた
  - naming cleanup (2026-03-07, slice 41): `normalization` と canonicalizer/bridge tests の current-facing `Pattern` wording を route-shape 主語へ寄せた
  - naming cleanup (2026-03-07, slice 42): `pattern_recognizers` と generic-loop shape detector の current-facing `Pattern` prose を route-shape 主語へ寄せた
  - naming cleanup (2026-03-07, slice 43): recognizer module surface を `route_shape_recognizers` に切り替え、current code/docs の主語を semantic 名へ固定した
  - naming cleanup (2026-03-07, slice 44): non-escape detector/export/wrapper 名の `*_pattern` を `*_shape` に切り替え、canonicalizer helper surface を semantic 化した
  - naming cleanup (2026-03-07, slice 45): `escape` detector/export/wrapper 名も `*_shape` へ揃え、escape recognizer prose を route-shape 主語へ寄せた
    - synced files: `src/mir/{builder.rs,mod.rs}` / `src/mir/builder/control_flow/{mod.rs,joinir/mod.rs,joinir/patterns/mod.rs}` / `src/mir/builder/control_flow/plan/{ast_feature_extractor.rs,escape_shape_recognizer.rs,policies/policies/{p5b_escape_derived_policy.rs,README.md}}` / `src/mir/loop_canonicalizer/{route_shape_recognizer.rs,canonicalizer.rs}` / `docs/development/current/main/design/pattern-p5b-escape-design.md`
    - intent: current API surface では `detect_escape_skip_pattern` / `try_extract_escape_skip_pattern` / `EscapeSkipPatternInfo` を使わず、`*_shape` / `EscapeSkipShapeInfo` に揃える
    - verification: `rg -n "EscapeSkipPatternInfo|detect_escape_skip_pattern|try_extract_escape_skip_pattern" src/mir docs/development/current/main/design/pattern-p5b-escape-design.md -g '!**/*history*' -g '!**/*archive*'` = 0 hit
  - naming cleanup (2026-03-07, slice 46): legacy physical file residue のうち、semantic 名へ安全に rename できる 3 点を整理した
    - synced files: `src/mir/builder/control_flow/plan/{mod.rs,ast_feature_extractor.rs,escape_shape_recognizer.rs,policies/policies/{README.md,p5b_escape_derived_policy.rs}}` / `src/mir/builder/control_flow/joinir/patterns/mod.rs` / `src/mir/builder/control_flow/plan/facts/{mod.rs,match_return_facts.rs}` / `src/mir/builder/control_flow/plan/canon/generic_loop/{update.rs,update/literal_match.rs}` / `src/mir/builder/control_flow/plan/generic_loop/README.md` / `docs/development/current/main/design/{plan-mod-layout-ssot,compiler-task-map-ssot}.md`
    - intent: `escape_pattern_recognizer.rs` / `pattern_match_return_facts.rs` / `update/pattern_match.rs` をそれぞれ `escape_shape_recognizer.rs` / `match_return_facts.rs` / `update/literal_match.rs` へ rename し、current code/docs から legacy physical path を後退させる
    - verification: `rg -n "escape_pattern_recognizer|pattern_match_return_facts|mod pattern_match;|pattern_match::|update/pattern_match\\.rs" src/mir docs/development/current/main/design -g '!**/*history*' -g '!**/*archive*'` = 0 hit
    - synced files: `src/mir/{builder.rs,mod.rs}` / `src/mir/builder/control_flow/{mod.rs,joinir/mod.rs,joinir/patterns/mod.rs}` / `src/mir/builder/control_flow/plan/{ast_feature_extractor.rs,route_shape_recognizers/{parse_number,parse_string,skip_whitespace}.rs,policies/policies/loop_true_read_digits_policy.rs}` / `src/mir/loop_canonicalizer/{route_shape_recognizer.rs,canonicalizer.rs}`
    - intent: current API surface では `detect_*_pattern` / `try_extract_*_pattern` / `ContinuePatternInfo` を使わず、`*_shape` / `ContinueShapeInfo` に揃える
    - verification: `rg -n "detect_(skip_whitespace|read_digits_loop_true|parse_number|parse_string|continue)_pattern|try_extract_(skip_whitespace|read_digits_loop_true|parse_number|parse_string|continue)_pattern|ContinuePatternInfo" src/mir docs/development/current/main/design/pattern-p5b-escape-design.md CURRENT_TASK.md -g '!**/*history*' -g '!**/*archive*'` = 0 hit
    - synced files: `src/mir/builder/control_flow/plan/{mod.rs,ast_feature_extractor.rs}` / `docs/development/current/main/design/{plan-mod-layout-ssot,compiler-task-map-ssot}.md`
    - intent: current code/docs では semantic module surface を `route_shape_recognizers` に固定する
    - verification: `rg -n "mod pattern_recognizers|use .*pattern_recognizers|pattern_recognizers::|pattern_recognizers - route recognizers" src/mir/builder/control_flow/plan docs/development/current/main/design/{plan-mod-layout-ssot,compiler-task-map-ssot}.md` = physical path note only
  - naming cleanup (2026-03-07, slice 47): recognizer の remaining on-disk residue を semantic path へ揃えた
    - synced files: `src/mir/builder/control_flow/plan/{mod.rs,ast_feature_extractor.rs,route_shape_recognizers/**}` / `src/mir/loop_canonicalizer/{mod.rs,route_shape_recognizer.rs,canonicalizer.rs}` / `docs/development/current/main/design/{plan-mod-layout-ssot,compiler-task-map-ssot,loop-canonicalizer}.md`
    - intent: `plan/pattern_recognizers/` を `plan/route_shape_recognizers/` に、`loop_canonicalizer/pattern_recognizer.rs` を `loop_canonicalizer/route_shape_recognizer.rs` に rename し、current docs/comment の legacy on-disk note を除去する
    - verification: `rg -n "pattern_recognizers/|pattern_recognizer\\.rs|mod pattern_recognizer;|super::pattern_recognizer|Legacy on-disk directory name remains" src/mir docs/development/current/main/design -g '!**/*history*' -g '!**/*archive*'` = 0 hit
    - synced files: `src/mir/builder/control_flow/plan/route_shape_recognizers/{mod,parse_number,parse_string,skip_whitespace,if_else_phi}.rs` / `src/mir/builder/control_flow/plan/generic_loop/body_check_shape_detectors/{basic,accum,complex_parsers}.rs` / `src/mir/builder/control_flow/plan/generic_loop/README.md`
    - intent: active recognizer/detector prose では `Pattern` を architecture の主語にせず、route shape / shape detector / legacy file name 注記に縮退する
    - verification: `rg -n "Pattern Recognizers Module|Pattern Detection|pattern-specific semantics|^/// Pattern:|Recognized patterns|pattern analyzers|if-sum patterns|loop route\\.|int_to_str loop route" src/mir/builder/control_flow/plan/route_shape_recognizers src/mir/builder/control_flow/plan/generic_loop/body_check_shape_detectors src/mir/builder/control_flow/plan/generic_loop/README.md` = 0 hit
  - naming cleanup (2026-03-07, slice 48): trim helper の current plan path/type 名から `pattern` を外した
    - synced files: `src/mir/builder/control_flow/plan/{mod.rs,trim_loop_lowering.rs,trim_lowerer.rs,trim_validator.rs}` / `src/mir/builder/control_flow/joinir/patterns/mod.rs` / `src/mir/join_ir/lowering/common/condition_only_emitter.rs` / `docs/development/current/main/design/compiler-task-map-ssot.md`
    - intent: `trim_pattern_lowerer.rs` / `trim_pattern_validator.rs` と `TrimPatternLowerer` / `TrimPatternValidator` を `trim_lowerer.rs` / `trim_validator.rs` と `TrimLowerer` / `TrimValidator` へ rename し、current plan helper surface を semantic 名へ揃える
    - verification: `rg -n "trim_pattern_lowerer|trim_pattern_validator|TrimPatternLowerer|TrimPatternValidator" src docs/development/current/main/design -g '!**/*history*' -g '!**/*archive*'` = 0 hit
  - naming cleanup (2026-03-07, slice 49): JoinIR lowering の route cluster を `pattern` 主語から `route` 主語へ寄せた
    - synced files: `src/mir/join_ir/lowering/{mod.rs,loop_route_router.rs,loop_route_validator.rs,if_lowering_router.rs,loop_to_join/{mod.rs,core.rs},loop_routes/**}` / `src/mir/loop_pattern_detection/classify.rs`
    - intent: `loop_pattern_router.rs` / `loop_pattern_validator.rs` / `loop_routes/` を `loop_route_router.rs` / `loop_route_validator.rs` / `loop_routes/` へ rename し、current lowering path と helper 名を route-first に揃える
    - verification: `rg -n "loop_pattern_router|loop_pattern_validator|LoopPatternValidator|try_lower_loop_pattern_to_joinir|loop_routes/|loop_routes::|loop_routes/tests" src/mir docs/development/current/main/design -g '!src/mir/join_ir/frontend/**' -g '!**/*history*' -g '!**/*archive*'` = 0 hit
    - synced files: `src/mir/builder/control_flow/normalization/{README.md,mod.rs,plan.rs,plan_box.rs,execute_box.rs}` / `src/mir/builder/control_flow/plan/policies/policies/normalized_shadow_suffix_router_box.rs` / `src/mir/loop_canonicalizer/canonicalizer_tests/{parse_number,trim_trailing,trim_leading,parse_string,parse_array,parse_object,escape_skip,skip_whitespace,continue_route}.rs` / `src/tests/{joinir_vm_bridge_trim,joinir_vm_bridge_skip_ws}.rs`
    - intent: current-facing code/test comment では `Pattern` を architecture の主語にせず、normalized shape / route shape / route choice を主語にする
    - verification: `rg -n "Test Pattern:|skip_ws pattern|pattern detection|normalized pattern|Loop-only pattern|Pattern should|test_.*_pattern_|Escape pattern canonicalization|simple continue pattern" src/mir/builder/control_flow/normalization src/mir/builder/control_flow/plan/policies/policies/normalized_shadow_suffix_router_box.rs src/mir/loop_canonicalizer/canonicalizer_tests src/tests/joinir_vm_bridge_{trim,skip_ws}.rs` = 0 hit
    - synced files: `src/mir/loop_canonicalizer/{mod.rs,route_shape_recognizer.rs,canonicalizer.rs}` / `src/mir/join_ir/lowering/{mod.rs,loop_with_break_minimal.rs}`
    - intent: current code comment/docstring では `Pattern` を architecture の主語にせず、route shape / route detection / route lowerer を主語にする
    - verification: `rg -n "Supported patterns|Continue pattern|Parse Number Pattern|Parse String/Array Pattern|Escape Sequence Handling Pattern|Pattern not recognized|Pattern Lowerer|Pattern Recognition Helpers|Pattern not matched or lowering error|if-sum pattern|Pattern B" src/mir/loop_canonicalizer src/mir/join_ir/lowering/{mod.rs,loop_with_break_minimal.rs}` = 0 hit
  - naming cleanup (2026-03-07, slice 50): frontend `ast_lowerer` の loop route cluster を semantic path/type 名へ同期した
    - synced files: `src/mir/join_ir/frontend/ast_lowerer/{mod.rs,analysis.rs,loop_frontend_binding.rs,loop_routes/**}` / `CURRENT_TASK.md`
    - intent: `frontend/ast_lowerer/loop_patterns/` を `loop_routes/` に rename し、`LoopPattern` / `LoopPatternLowerer` / `detect_loop_pattern` / `lower_loop_with_pattern` / `UnimplementedPattern` を `LoopRoute` / `LoopRouteLowerer` / `detect_loop_route` / `lower_loop_with_route` / `UnimplementedRoute` に揃える。current-facing README / comment / error text でも `pattern` を architecture の主語にしない
    - verification: `rg -n "loop_patterns/|loop_patterns::|mod loop_patterns;|LoopPattern\\b|LoopPatternLowerer|UnimplementedPattern|detect_loop_pattern|lower_loop_with_pattern|break_pattern|continue_pattern|continue_return_pattern|UnimplementedRoute \\{\\s*pattern:" src/mir/join_ir/frontend/ast_lowerer -g '!src/mir/join_ir/frontend/ast_lowerer/if_in_loop/**'` = 0 hit
    - verification: `cargo build --release --bin hakorune` PASS / `phase29bq_fast_gate_vm.sh --only bq` PASS / `tools/dev/direct_loop_progression_sweep.sh --profile phase29x-probe --allow-emit-fail` PASS
  - naming cleanup (2026-03-07, slice 51): frontend `if_in_loop` の current path/type 名を `shape` 主語へ同期した
    - synced files: `src/mir/join_ir/frontend/ast_lowerer/if_in_loop/{mod.rs,shape.rs}` / `CURRENT_TASK.md`
    - intent: `if_in_loop/pattern.rs` を `shape.rs` に rename し、`IfInLoopPattern` と local `pattern` binding を `IfInLoopShape` / `shape` に揃える。current frontend path では `pattern` を route/shape 判定の主語にしない
    - verification: `rg -n "IfInLoopPattern|if_in_loop/pattern|pub mod pattern;|use pattern::" src/mir/join_ir/frontend/ast_lowerer docs/development/current/main/design CURRENT_TASK.md -g '!**/*history*' -g '!**/*archive*'` = 0 hit
    - verification: `cargo build --release --bin hakorune` PASS / `phase29bq_fast_gate_vm.sh --only bq` PASS / `tools/dev/direct_loop_progression_sweep.sh --profile phase29x-probe --allow-emit-fail` PASS
  - naming cleanup (2026-03-07, slice 52): trim promotion helper の live type 名を `route` 主語へ同期した
    - synced files: `src/mir/loop_pattern_detection/legacy/{loop_body_carrier_promoter.rs,trim_loop_helper.rs}` / `src/mir/join_ir/lowering/carrier_info/carrier_info_impl.rs` / `src/mir/builder/control_flow/plan/trim_loop_lowering.rs` / `CURRENT_TASK.md`
    - intent: `TrimPatternInfo` / `from_pattern_info` を `TrimRouteInfo` / `from_route_info` に rename し、trim promotion / helper / carrier-info / trim lowerer の current comment でも `Trim pattern` を route 主語へ後退させる
    - verification: `rg -n "TrimPatternInfo|from_pattern_info" src/mir CURRENT_TASK.md docs/development/current/main/design -g '!**/*history*' -g '!**/*archive*'` = 0 hit
    - verification: `cargo build --release --bin hakorune` PASS / `phase29bq_fast_gate_vm.sh --only bq` PASS / `tools/dev/direct_loop_progression_sweep.sh --profile phase29x-probe --allow-emit-fail` PASS
  - truth cleanup (2026-03-07, slice 53): `joinir/patterns` 表記を active architecture 主語から外し、legacy physical path 注記へ後退させた
    - synced files: `src/mir/join_ir/frontend/ast_lowerer/README.md` / `src/mir/builder/control_flow/mod.rs` / `docs/development/current/main/design/{plan-lowering-entry-ssot,joinir-design-map,compiler-task-map-ssot}.md` / `CURRENT_TASK.md`
    - intent: current README / SSOT では `joinir/patterns` を active module surface として語らず、route-entry layer の legacy on-disk path だと明示する
    - verification: targeted doc/comment grep on the synced files shows only intentional `legacy physical path` notes
    - verification: docs/comment-only slice のため build/gate 再実行なし
  - truth cleanup (2026-03-07, slice 54): `loop_pattern_detection/legacy/**` の current-facing prose を route-first に薄くした
    - synced files: `src/mir/loop_pattern_detection/{mod.rs,legacy/{mod.rs,trim_detector.rs,digitpos_detector.rs,loop_body_cond_promoter.rs,loop_body_digitpos_promoter.rs}}` / `CURRENT_TASK.md`
    - intent: `Loop Pattern Detection` / `Trim Pattern` / `DigitPos Pattern` / `Pattern detected` を current production prose から外し、route-shape / route promotion / route detected へ寄せる
    - verification: `rg -n "Loop Pattern Detection Module|Trim Pattern|DigitPos Pattern|Pattern detected|pattern detected|No promotable pattern detected" src/mir/loop_pattern_detection src/mir/loop_pattern_detection/legacy -g '!**/*history*' -g '!**/*archive*'` = generic `Accumulator pattern detected` only
    - verification: `cargo build --release --bin hakorune` PASS / `phase29bq_fast_gate_vm.sh --only bq` PASS / `tools/dev/direct_loop_progression_sweep.sh --profile phase29x-probe --allow-emit-fail` PASS
  - naming cleanup (2026-03-07, slice 55): `LoopPatternKind` を alias-first で `LoopRouteKind` へ移し、current runtime surface を semantic enum 名へ同期した
    - synced files: `src/mir/loop_pattern_detection/{kind,mod,classify,tests}.rs` / `src/mir/loop_canonicalizer/{mod,canonicalizer,capability_guard,canonicalizer_tests/**}.rs` / `src/mir/builder/control_flow/{joinir/{loop_context,patterns/router,routing}.rs,plan/{planner/context.rs,composer/shadow_adopt.rs,facts/loop_tests.rs}}` / `src/mir/join_ir/lowering/{loop_route_router.rs,loop_scope_shape/case_a_lowering_shape.rs,loop_routes/nested_minimal.rs}` / active docs（`joinir-design-map.md`, `loop-canonicalizer.md`）
    - intent: runtime/current docs と planner/joinir/canonicalizer surface では `LoopRouteKind` を主語にし、`LoopPatternKind` は `loop_pattern_detection::kind` の traceability alias に後退させる
    - verification: `rg -n "\\bLoopPatternKind\\b|LoopForm → extract_features\\(\\) → LoopFeatures → classify\\(\\) → LoopPatternKind|pattern selection and failure reasons|result of pattern selection" src/mir/loop_pattern_detection src/mir/loop_canonicalizer src/mir/builder/control_flow/joinir/loop_context.rs src/mir/join_ir/lowering/loop_route_router.rs docs/development/current/main/design/{joinir-design-map,loop-canonicalizer}.md CURRENT_TASK.md -g '!**/*history*' -g '!**/*archive*'` = alias export + `CURRENT_TASK` 履歴のみ
    - verification: `cargo check --tests` PASS / `cargo build --release --bin hakorune` PASS / `phase29bq_fast_gate_vm.sh --only bq` PASS / `tools/dev/direct_loop_progression_sweep.sh --profile phase29x-probe --allow-emit-fail` PASS（`unexpected_emit_fail_count=0`, `route_blocker_count=0`）
  - naming cleanup (2026-03-07, slice 56): `LoopPatternKind` alias を traceability-only に閉じ込め、remaining current-facing route/shape logs を薄くした
    - synced files: `src/mir/loop_pattern_detection/{mod,kind}.rs` / `src/mir/join_ir/lowering/{loop_route_router.rs,loop_routes/{with_break,with_continue,with_if_phi}.rs,if_lowering_router.rs,if_select.rs,if_dry_runner.rs,exit_meta_builder.rs}` / `src/mir/builder/{exprs.rs,if_form.rs}` / `src/mir/builder/control_flow/{mod.rs,joinir/merge/contract_checks/boundary_creation.rs,plan/{common/mod.rs,features/{loop_cond_co_continue_if.rs,loop_cond_co_group_if.rs,loop_cond_bc_else_patterns.rs}}}` / `src/mir/mod.rs`
    - intent: `LoopPatternKind` re-export を止めて alias を `kind.rs` 内の traceability-only residue に縮退し、`try_lower_loop_pattern` / `[loop_patterns]` / `pattern not matched` などの current-facing log/comment を route/shape 主語へ寄せる
    - verification: `rg -n "\\bLoopPatternKind\\b|try_lower_loop_pattern|\\[loop_patterns\\]|pattern not matched|no pattern matched|Loop pattern matched|JoinIR pattern not matched|Pattern selection:|pattern lowering code" src/mir docs/development/current/main/design CURRENT_TASK.md -g '!**/*history*' -g '!**/*archive*'` = `kind.rs` alias + `CURRENT_TASK` 履歴のみ
    - verification: `cargo build --release --bin hakorune` PASS / `phase29bq_fast_gate_vm.sh --only bq` PASS / `tools/dev/direct_loop_progression_sweep.sh --profile phase29x-probe --allow-emit-fail` PASS（`unexpected_emit_fail_count=0`, `route_blocker_count=0`）
  - naming cleanup (2026-03-07, slice 57): smoke gate labels / TSV reason prose の numbered token を semantic route 名へ寄せた
    - synced files: `tools/smokes/v2/profiles/integration/joinir/{phase29ae_regression_pack_vm.sh,phase29as_purity_gate_vm.sh,planner_required_cases.tsv,phase29bq_fast_gate_cases.tsv}` / `CURRENT_TASK.md`
    - intent: smoke script 内の display label / helper 名 / reason 列だけを `loop_break` / `if_phi_join` / `loop_continue_only` / `loop_simple_while` / `loop_true_early_exit` / `scan_with_init` / `nested_loop_minimal` / `split_scan` に同期し、`phase29ao_pattern*` の script stem・fixture filename・`case_id` は互換 token として不変に保つ
    - verification: `bash tools/smokes/v2/profiles/integration/joinir/phase29as_purity_gate_vm.sh` PASS / `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq` PASS / `tools/dev/direct_loop_progression_sweep.sh --profile phase29x-probe --allow-emit-fail` PASS（`unexpected_emit_fail_count=0`, `route_blocker_count=0`）
  - truth cleanup (2026-03-07, slice 58): active smoke helper 名と selfhost canary comment の current-looking numbered wording を route-first に薄くした
    - synced files: `tools/smokes/v2/profiles/integration/joinir/phase29av_flowbox_tags_gate_vm.sh` / `tools/smokes/v2/profiles/integration/core/phase2047/{selfhost_s1_s2_from_builder_canary_vm,selfhost_s1_s2_from_builder_compare_cfg_canary_vm,selfhost_s1_s2_from_builder_compare_ret_canary_vm}.sh` / `tools/smokes/v2/profiles/integration/core/phase2051/selfhost_v0_s1s2_repeat_canary_vm.sh` / `tools/smokes/v2/profiles/integration/{selfhost_phase150_depth1_smoke.sh,selfhost/selfhost_mir_min_vm.sh}` / `CURRENT_TASK.md`
    - intent: `phase29av` の live helper/label では `if_phi_join` を主語にし、selfhost canary comment は `known route-shape gap` / `legacy route error signature` へ寄せる。runtime grep signature（`loop pattern is not supported` など）と script stem は compat のため不変に保つ
    - verification: `bash tools/smokes/v2/profiles/integration/joinir/phase29av_flowbox_tags_gate_vm.sh` PASS / `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq` PASS / `tools/dev/direct_loop_progression_sweep.sh --profile phase29x-probe --allow-emit-fail` PASS（`unexpected_emit_fail_count=0`, `route_blocker_count=0`）
  - naming cleanup (2026-03-07, slice 59): planner-required wrapper の `/tmp` log 名と selfhost baseline comment を semantic route 名へ寄せた
    - synced files: `tools/smokes/v2/profiles/integration/joinir/{phase29bk_planner_required_dev_gate_vm,phase29bn_planner_required_dev_gate_v2_vm,phase29bo_planner_required_dev_gate_v3_vm}.sh` / `tools/smokes/v2/profiles/integration/selfhost/selfhost_minimal.sh` / `CURRENT_TASK.md`
    - intent: wrapper script stem と pack filename は compat のため不変に保ちつつ、`/tmp` log 名だけを `loop_break` / `loop_simple_while` / `loop_continue_only` / `loop_true_early_exit` / `if_phi_join` / `bool_predicate_scan` / `accum_const_loop` 主語へ同期する。`selfhost_minimal.sh` は current-facing comment の `known patterns` を `known route-shape gaps` に寄せる
    - verification: `bash tools/smokes/v2/profiles/integration/joinir/phase29bk_planner_required_dev_gate_vm.sh` PASS / `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq` PASS
  - naming cleanup (2026-03-07, slice 60): legacy smoke stem retirement を phase 化し、semantic alias wrapper を active docs / regression pack surface へ導入した
    - synced files: `docs/development/current/main/design/{joinir-smoke-legacy-stem-retirement-ssot,coreplan-shadow-adopt-tag-coverage-ssot,flowbox-tag-coverage-map-ssot}.md` / `tools/smokes/v2/profiles/integration/joinir/{loop_simple_while_strict_shadow_vm,loop_simple_while_subset_reject_extra_stmt_vm,loop_break_release_adopt_vm,loop_break_plan_subset_vm,loop_break_realworld_vm,loop_break_body_local_vm,loop_break_body_local_seg_vm,if_phi_join_vm,if_phi_join_release_adopt_vm,loop_continue_only_vm,loop_true_early_exit_vm,loop_true_early_exit_strict_shadow_vm,loop_true_early_exit_release_adopt_vm,scan_with_init_strict_shadow_vm,scan_with_init_release_adopt_vm,scan_with_init_regression_pack_vm,nested_loop_minimal_release_adopt_vm,nested_loop_minimal_strict_shadow_vm,split_scan_strict_shadow_vm,split_scan_release_adopt_vm,split_scan_regression_pack_vm}.sh` / `tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh` / `CURRENT_TASK.md`
    - intent: old script stem は互換 entry として残しつつ、semantic alias wrapper を新 surface として追加する。active docs と regression pack filter は alias wrapper を優先し、legacy stem は archive / traceability-only note に後退させる
    - verification: `bash tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh` PASS / `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq` PASS
  - naming cleanup (2026-03-07, slice 61): planner-required pack stem も alias-first で semantic wrapper を追加し、dev gate/current guidance を wrapper surface へ切り替えた
    - synced files: `docs/development/current/main/design/joinir-smoke-legacy-stem-retirement-ssot.md` / `tools/smokes/v2/profiles/integration/joinir/{loop_break_planner_required_pack_vm,scan_split_planner_required_pack_vm,core_loop_routes_planner_required_pack_vm,if_phi_join_planner_required_pack_vm,bool_predicate_accum_planner_required_pack_vm}.sh` / `tools/smokes/v2/profiles/integration/joinir/{phase29bk_planner_required_dev_gate_vm,phase29bn_planner_required_dev_gate_v2_vm,phase29bo_planner_required_dev_gate_v3_vm}.sh` / `CURRENT_TASK.md`
    - intent: `phase29bi/bl/bn/bo_*` pack stem は互換 entry として残しつつ、semantic pack alias wrapper を current gate surface と daily guidance に導入する。旧 stem retire は active caller が 0 になるまで行わない
    - verification: `bash tools/smokes/v2/profiles/integration/joinir/phase29bo_planner_required_dev_gate_v3_vm.sh` PASS / `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq` PASS / `tools/dev/direct_loop_progression_sweep.sh --profile phase29x-probe --allow-emit-fail` PASS
  - truth cleanup (2026-03-07, slice 62): selfhost gate filter contract と legacy pin note を current guidance に同期した
    - synced files: `docs/development/current/main/design/{boxcount-new-box-addition-checklist-ssot,coreloop-stepmode-inline-in-body-ssot}.md` / `tools/smokes/v2/profiles/integration/selfhost/{phase29bq_selfhost_planner_required_dev_gate_vm.sh,planner_required_selfhost_subset.tsv}` / `CURRENT_TASK.md`
    - intent: selfhost gate の絞り込み契約を `SMOKES_SELFHOST_FILTER` に固定し、`phase118_pattern3_if_sum_min` / `pattern1_inline_explicit_step_min` を legacy fixture/case token として明示する。route semantics は `if_phi_join` / `loop_simple_while explicit-step` として読む
    - verification: `SMOKES_ENABLE_SELFHOST=1 SMOKES_SELFHOST_FILTER=phase118_pattern3_if_sum_min RUN_TIMEOUT_SECS=120 bash tools/smokes/v2/profiles/integration/selfhost/phase29bq_selfhost_planner_required_dev_gate_vm.sh` PASS / `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq` PASS
  - truth cleanup (2026-03-07, slice 63): active design docs の legacy fixture pin を inventory 化し、本文の主語を route semantics へ寄せた
    - synced files: `docs/development/current/main/design/{joinir-legacy-fixture-pin-inventory-ssot,pattern6-7-contracts,pattern-p5b-escape-design,loop-canonicalizer}.md` / `CURRENT_TASK.md`
    - intent: `phase29ab_pattern6_*` / `phase29ab_pattern7_*` / `phase286_pattern5_break_min` / `phase269_p0_pattern8_frag_min` / `phase286_pattern9_frag_poc` / `test_pattern5b_escape_minimal.hako` のような active fixture filename を `legacy fixture pin token` として inventory に逃がし、各本文では `scan_with_init` / `split_scan` / `loop_true_early_exit` / `bool_predicate_scan` / `accum_const_loop` / `escape route P5b` を主語にする
    - verification: `rg -n "legacy fixture pin|legacy selfhost test stem|joinir-legacy-fixture-pin-inventory-ssot" docs/development/current/main/design/{pattern6-7-contracts,pattern-p5b-escape-design,loop-canonicalizer}.md docs/development/current/main/design/joinir-legacy-fixture-pin-inventory-ssot.md` = expected hits only
  - truth cleanup (2026-03-07, slice 64): selfhost test stem の active docs を legacy pin inventory に寄せた
    - synced files: `docs/development/current/main/design/{joinir-legacy-fixture-pin-inventory-ssot,pattern-p5b-escape-design,loop-canonicalizer}.md` / `CURRENT_TASK.md`
    - intent: `test_pattern3_skip_whitespace.hako` と `test_pattern5b_escape_*` を `legacy selfhost test stem` として inventory へ逃がし、本文の主語を `skip_whitespace` / `escape route P5b` に固定する
    - verification: `rg -n "legacy selfhost test stem|test_pattern3_skip_whitespace|test_pattern5b_escape_" docs/development/current/main/design/{joinir-legacy-fixture-pin-inventory-ssot,pattern-p5b-escape-design,loop-canonicalizer}.md` = expected hits only
  - truth cleanup (2026-03-07, slice 65): FlowBox/CorePlan coverage docs の stem/token 役割を分離した
    - synced files: `docs/development/current/main/design/{flowbox-tag-coverage-map-ssot,coreplan-shadow-adopt-tag-coverage-ssot}.md` / `CURRENT_TASK.md`
    - intent: FlowBox は semantic alias wrapper stem を主語にし、negative coverage に残る `phase29ab_pattern2_seg_*` は `archived legacy stem` として明示する。CorePlan shadow-adopt は `tag suffix = legacy token` / `smoke path = current wrapper or archived legacy stem` に切り分ける
    - verification: `rg -n "archived legacy stem|current semantic wrapper|joinir-smoke-legacy-stem-retirement-ssot" docs/development/current/main/design/{flowbox-tag-coverage-map-ssot,coreplan-shadow-adopt-tag-coverage-ssot}.md` = expected hits only

## next fixed order (resume point)

1. `phase29bq_fast_gate_vm.sh --only bq` と `phase29x-probe` を各 cleanup で継続し、`unexpected_emit_fail=0` / `route_blocker=0` を維持する。
2. legacy fixture key retirement は完了。old/new mapping は `CURRENT_TASK` / retirement SSOT / archive-history にだけ残し、runtime contract へ戻さない。
3. `truth` cleanup を継続し、active docs の remaining traceability-only note を `joinir-design-map.md` / `planfrag-freeze-taxonomy.md` / `edgecfg-fragments.md` などからさらに薄くする。
4. `docs/private` は nested git repo として別管理し、fixture rename / private doc drift は top-level commit と混ぜない。
5. `naming` cleanup: smoke/test/script の legacy token は display label / reason / helper 名から先に外した。smoke/planner pack alias wrapper は current gate surface へ導入済みで、active docs の fixture pin inventory も切り出し中。old stem retire は active caller が 0 になってから別 phase で扱う。
6. `dust` cleanup: warnings / orphan helper / dead code を刈る。
7. docs / CURRENT_TASK / phase README は archive-first 運用を維持し、長文の時系列ログを root pointer に戻さない。

## Quick Restart (After Reboot)

1. `git status -sb`
2. `sed -n '1,220p' CURRENT_TASK.md`
3. `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq`
4. `tools/dev/direct_loop_progression_sweep.sh --profile phase29x-probe --allow-emit-fail`

## Read-First Navigation

- root pointer: `CURRENT_TASK.md`（このファイル）
- compiler map SSOT: `docs/development/current/main/design/compiler-task-map-ssot.md`
- cleanliness SSOT: `docs/development/current/main/design/compiler-cleanliness-campaign-ssot.md`
- planner gate SSOT: `docs/development/current/main/design/joinir-planner-required-gates-ssot.md`
- ai/debug contract SSOT: `docs/development/current/main/design/ai-handoff-and-debug-contract.md`

## Quick Entry: Selfhost Migration

1. `docs/development/current/main/design/selfhost-parser-mirbuilder-migration-order-ssot.md`
2. `docs/development/current/main/phases/phase-29bq/29bq-90-selfhost-checklist.md`
3. `docs/development/current/main/phases/phase-29bq/29bq-91-mirbuilder-migration-progress-checklist.md`
4. `docs/development/current/main/phases/phase-29bq/29bq-92-parser-handoff-checklist.md`

## Daily Commands

- fast gate:
  - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq`
- planner-required packs:
  - `bash tools/smokes/v2/profiles/integration/joinir/scan_split_planner_required_pack_vm.sh`
  - `bash tools/smokes/v2/profiles/integration/joinir/bool_predicate_accum_planner_required_pack_vm.sh`
- probe:
  - `tools/dev/direct_loop_progression_sweep.sh --profile phase29x-probe --allow-emit-fail`
- allowlist guard:
  - `tools/dev/check_loop_pattern_context_allowlist.sh`

## Archive

- full historical log (2111 lines, archived 2026-03-04):
  - `docs/development/current/main/investigations/current_task_archive_2026-03-04.md`
- compiler cleanup handoff archive (2026-03-06):
  - `docs/development/current/main/investigations/current_task_archive_2026-03-06_compiler_cleanup.md`
- policy:
  - 長文の時系列ログは以後 archive 側へ追記し、`CURRENT_TASK.md` は再起動用の薄い入口を維持する。
