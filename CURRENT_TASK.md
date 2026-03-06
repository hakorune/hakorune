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
  - extractor cluster（`extractors/{mod,pattern1,pattern3,common_helpers}` + `pattern_recognizers/if_else_phi`）の補助コメントを route 主語へ同期（legacy label は注記で保持）
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
  - compiler 主経路の route enum を semantic 名へ同期済み（`LoopPatternKind::LoopBreak`, `RouteVariant::{LoopSimpleWhile,LoopBreak,IfPhiJoin,LoopContinueOnly}`）
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
  - `LoopPatternKind::{LoopSimpleWhile,NestedLoopMinimal}` へ active enum variant を semantic 化済み。classifier/router/loop_context/loop_canonicalizer/tests と active docs（`domainplan-thinning-ssot.md`, `coreplan-skeleton-feature-model.md`）も同期した
  - verification: `cargo build --release --bin hakorune` PASS / `phase29bq_fast_gate_vm.sh --only bq` PASS / `rg -n "Pattern1SimpleWhile|Pattern6NestedLoopMinimal" src CURRENT_TASK.md docs/development/current/main/design/domainplan-thinning-ssot.md docs/development/current/main/design/coreplan-skeleton-feature-model.md` = 0 hit
  - `src/mir/join_ir/lowering/**` の live `joinir/patternN` debug/error tags を route 主語へ同期済み（`simple_while_minimal` / `scan_with_init_{minimal,reverse}` / `split_scan_minimal` / `scan_bool_predicate_minimal` / `loop_with_if_phi_if_sum` / `error_tags`）。history/traceability docs と legacy smoke 名はこの slice では不変
  - `LoopPatternKind::LoopTrueEarlyExit` へ enum surface を semantic 化し、`loop_pattern_detection::{kind,classify}` と `join_ir/lowering::{loop_pattern_router,mod}` の current-facing prose も route 主語へ同期中
  - `ConditionCapability::IfPhiJoinComparable` / `LoopViewBuilder::try_loop_simple_while` へ current helper surface を semantic 化し、`features` / `condition_pattern` / `loop_view_builder` / `route_prep_pipeline` / `loop_with_if_phi_if_sum` の近傍 prose も route 主語へ同期中
  - `join_ir/lowering/loop_patterns/**` と `carrier_info` / `loop_update_summary` の stub/docs wording も route-first に同期し、残る `Pattern N` は `traceability-only` 注記へ後退
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
  - verification: `rg -n "join_ir::normalized::|pub mod normalized;|ParseStringComposite|if_sum_break_pattern|scope_manager_bindingid_poc" src tests` = 0 hit
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
    - synced files: `src/mir/loop_pattern_detection/kind.rs` / `classify.rs` / `src/mir/join_ir/lowering/loop_pattern_router.rs` / `mod.rs`
    - intent: current-facing `InfiniteEarlyExit` / `Pattern N` wording を runtime/public surface から外し、historical numbering は traceability note と `pattern_id()` に閉じる
    - verification: `cargo build --release --bin hakorune` PASS / `cargo test --release --lib loop_pattern_detection --no-run` PASS / `phase29bq_fast_gate_vm.sh --only bq` PASS
    - verification: `rg -n "LoopPatternKind::InfiniteEarlyExit|InfiniteEarlyExit\\b|Pattern 5 \\(InfiniteEarlyExit\\)|Pattern 5: Infinite|Pattern 1 / LoopSimpleWhile|Pattern 3: Loop with If-Else PHI|Pattern 4: Loop with Continue|Pattern 6 minimal lowerer|Pattern 7 minimal lowerer|Pattern 8 minimal lowerer" src/mir/loop_pattern_detection/{kind.rs,classify.rs} src/mir/join_ir/lowering/{loop_pattern_router.rs,mod.rs}` = 0 hit
  - naming cleanup (2026-03-06, slice 7): condition/route helper surface を semantic 化した
    - synced files: `src/mir/join_ir/lowering/condition_pattern.rs` / `loop_with_if_phi_if_sum.rs` / `loop_view_builder.rs` / `src/mir/builder/control_flow/plan/route_prep_pipeline.rs` / `src/mir/loop_pattern_detection/features.rs`
    - intent: current-facing `IfSumComparable` / `try_pattern1` などの helper surface を route 主語へ寄せ、`Pattern3/4` 由来の近傍 prose を current runtime surface から外す
    - verification: `cargo test --lib test_capability_if_phi_join_comparable_simple` PASS / `cargo check --tests` PASS / `cargo build --release --bin hakorune` PASS / `phase29bq_fast_gate_vm.sh --only bq` PASS
    - verification: `rg -n "IfSumComparable|try_pattern1\\b|pattern3_cond_i_mod_2_eq_1_is_recognized|Pattern 3 if-sum|Pattern3/4|Pattern3 heuristic" src/mir/loop_pattern_detection/features.rs src/mir/join_ir/lowering/condition_pattern.rs src/mir/join_ir/lowering/loop_update_summary.rs src/mir/join_ir/lowering/loop_view_builder.rs src/mir/builder/control_flow/plan/route_prep_pipeline.rs src/mir/join_ir/lowering/loop_with_if_phi_if_sum.rs` = 0 hit
  - naming cleanup (2026-03-06, slice 8): `join_ir/lowering/loop_patterns/**` の stub/docs/test wording と carrier/update helper docs を route-first に同期した
    - synced files: `src/mir/join_ir/lowering/loop_patterns/{mod,simple_while,with_break,with_continue,with_if_phi,nested_minimal}.rs` / `src/mir/join_ir/lowering/carrier_info/types.rs` / `src/mir/join_ir/lowering/loop_update_summary.rs`
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

## next fixed order (resume point)

1. `phase29bq_fast_gate_vm.sh --only bq` と `phase29x-probe` を各 cleanup で継続し、`unexpected_emit_fail=0` / `route_blocker=0` を維持する。
2. `truth` cleanup を継続し、active docs / runtime comments の旧主語を appendix / archive / README へさらに寄せる。
3. `naming` cleanup: legacy file/test/comment 名を semantic 名へ同期し、test-only wording と historical docs の境界をさらに薄くする。
4. `dust` cleanup: warnings / orphan helper / dead code を刈る。
5. docs / CURRENT_TASK / phase README は archive-first 運用を維持し、長文の時系列ログを root pointer に戻さない。

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
  - `bash tools/smokes/v2/profiles/integration/joinir/phase29bj_planner_required_pattern6_7_pack_vm.sh`
  - `bash tools/smokes/v2/profiles/integration/joinir/phase29bo_planner_required_pattern8_9_pack_vm.sh`
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
