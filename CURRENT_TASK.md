# CURRENT_TASK (root pointer)

Status: SSOT
Date: 2026-03-05
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
- execution rule:
  - 1 blocker = 1受理形 = fixture+gate = 1 commit
  - BoxCount と BoxShape を同コミットで混在させない
- compiler fixed order:
  1. Pattern1..9 名を `router/planner` の主語（runtime label / message / surface）から段階的に外す。
  2. `normalizer/pattern*.rs` 依存を主経路から外し、recipe/composer 側へ責務集約する。
  3. `DomainPlan` は label-only（最終的には撤去）へ縮退する。
  4. `shadow_adopt` など暫定 fallback 経路を縮退し、strict/release 差分を最小化する。
  5. 経路を `Facts -> Recipe -> Composer -> Verifier -> Parts` に一本化する（router は recipe-first のみ）。

## Compiler Cleanup Order (2026-03-04, SSOT)

- D5-A: facts/planner dead staging 削除（挙動不変）
- D5-B: runtime 未参照の legacy module を isolate -> delete
- D5-C: diagnostic-only vocabulary を semantic key に揃える
- D5-D: test-only module wire を runtime build から分離
- D5-E: planner/build test 群を DomainPlan-only の事実に合わせて縮退

## Latest Probe Snapshot (direct route)

- command:
  - `tools/dev/direct_loop_progression_sweep.sh --profile phase29x-probe --allow-emit-fail`
- latest result (2026-03-05):
  - `emit_fail=0`
  - `run_nonzero=18`
  - `run_ok=101`
  - `route_blocker=0`
  - total=`119`
- class_counts:
  - `run:nonzero-empty=6`（intentional contract exit）
  - `run:nonzero=9`（direct runner bridge limitation cluster）
  - `run:vm-error=3`（provider-dependent cluster）
  - `run:ok=101`
- note:
  - monitor-known 扱い。compiler blocker は `none` 維持。

## Restart Handoff (2026-03-05)

- this round commits:
  - `734310f41` refactor D5 remove router domain-plan branch and keep recipe-first fallback lanes
    - `joinir/patterns/router.rs` から DomainPlan 専用分岐を削除し、entry flow を recipe-first + fallback lanes に整理
    - shadow/release adopt 呼び出しは `domain_plan` 依存なしの固定条件へ簡約
    - router 主経路の DomainPlan 依存を一段縮退（挙動維持）
  - `0e23eef27` refactor D5 deprecate domain-plan fallback branch and drop dead shadow-adopt expectation
    - `joinir/patterns/router.rs` の `DomainPlan` fallback 分岐を deprecated 扱いへ縮退（strictは fail-fast / releaseは明示 no-route）
    - 常に false だった `expectations::should_expect_shadow_adopt` を削除
    - DomainPlan 撤退に向けた死に枝を整理し、router 責務を recipe-first 側へ寄せる
  - `d7842eafb` refactor D5 rename single-planner route logs from pattern to rule vocabulary
    - `single_planner/rules.rs` の route log 主語を `pattern` から `rule` へ統一
    - planner 抽出経路の可観測語彙を semantic 命名に揃える
    - 動作は不変（ログ語彙のみ整理）
  - `0cff7a360` refactor D5 gate strict shadow-adopt fallback to planner-none routes
    - `joinir/patterns/router.rs` の strict/dev 側 shadow_adopt fallback も `domain_plan=None` 経路のみに限定
    - strict/release で fallback 適用条件を揃え、暫定経路の適用面積をさらに縮小
    - recipe-first / planner-first 優先を維持しつつ fallback を局所化
  - `061a18494` refactor D5 scope release shadow-adopt fallback to planner-none routes
    - `joinir/patterns/router.rs` の release adopt fallback を `domain_plan=None` 経路のみに限定
    - release fallback の適用面積を縮小し、暫定経路の責務を明示
    - recipe-first / planner 経路に優先順位を寄せる縮退（挙動維持）
  - `be2dc84cf` refactor D5 gate shadow-adopt fallback to recipe-contract-missing routes
    - `joinir/patterns/router.rs` で `allow_shadow_fallback = outcome.recipe_contract.is_none()` を導入
    - strict/release の shadow/release adopt fallback 両方を `allow_shadow_fallback` で統一ガード
    - recipe contract が確定した経路で legacy fallback を実行しない契約を明示化
  - `ddd3821d1` refactor D5 align router route diagnostics to loop vocabulary
    - `joinir/patterns/router.rs` の route exhausted 診断語彙を `pattern_kind` から `loop_kind` に変更
    - `no pattern matched` 文言を `no route matched` に統一
    - runtime surface の Pattern主語を縮退（挙動不変）
  - `8022c367d` refactor D5 decouple router from single-planner domain-plan tuple
    - `single_planner/mod.rs` に `try_build_outcome()` を追加（互換 tuple API は維持）
    - `joinir/patterns/router.rs` は `try_build_outcome()` を使い、`(domain_plan, outcome)` 依存を撤去
    - router 側の DomainPlan payload 依存を1段縮退し、outcome-first 契約へ寄せた
  - `26c5942ad` refactor D5 use single-planner outcome API in nested-loop plan path
    - `plan/nested_loop_plan.rs` で `try_build_domain_plan_with_outcome` 依存を撤去
    - `single_planner::try_build_outcome()` + `outcome.plan.take()` で既存挙動を維持
    - nested-loop 側も tuple 依存を減らし、DomainPlan payload 参照面積を縮小
  - `2c5e56f27` refactor D5 make single-planner rules outcome-first and keep tuple API as wrapper
    - `single_planner/rules.rs` の正本 entrypoint を `try_build_outcome()` に変更
    - `single_planner/mod.rs` の `try_build_domain_plan_with_outcome()` は互換 wrapper 化（`outcome.plan.take()`）
    - recipe-only rule で plan suppression（`outcome.plan=None`）契約を維持しつつ outcome-first へ移行
  - `2f60b34fe` refactor D5 migrate nested_loop_depth1 planner call to outcome API
    - `plan/features/nested_loop_depth1.rs` の `lower_nested_loop_single_planner` を `try_build_outcome()` へ移行
    - `outcome.plan.take()` で DomainPlan payload を取得し、既存エラー契約を維持
    - 残 tuple 呼び出しを `generic_loop_body/helpers.rs` の1箇所に縮小
  - `95a12aaef` refactor D5 shift planner-router runtime labels from pattern names to semantic names
    - `single_planner/rule_order.rs` の rule 定義から runtime 不要な Pattern文字列 payload を撤去
    - `joinir/patterns/registry/handlers.rs` の planner_required contract 文言を semantic rule 名へ統一
    - runtime surface の語彙を Pattern番号依存から段階的に切り離し（挙動不変）
  - `7e422e1e8` refactor D5 drop unused pattern2 api facade reexports
    - `plan/pattern2/api/mod.rs` から未使用の facade 再公開 (`PromoteDecision` / `try_promote`) を削除
    - `1b1e79a45` 後に発生した `unused_imports` warning 2件を解消
    - module API surface を実使用に同期（挙動不変）
  - `1b1e79a45` refactor D5 remove unused pattern2 lowering orchestrator module
    - `plan/mod.rs` から `pattern2_lowering_orchestrator` module wire を削除
    - 未参照の `plan/pattern2_lowering_orchestrator.rs` を削除
    - Pattern2 の未使用 orchestrator 残骸を除去して domain 縮退を前進
  - `4c8c8bc05` refactor D5 remove unused pattern1_simple_while legacy entrypoints
    - `plan/mod.rs` から `pattern1_simple_while` module wire を削除
    - 未参照の `plan/pattern1_simple_while.rs` を削除（legacy entrypoint 群を撤去）
    - Pattern domain 撤退の未使用1本を物理削除で前進（挙動不変）
  - `619fbd64b` refactor D5 remove unused pattern3_if_phi legacy entrypoints
    - `plan/mod.rs` から `pattern3_if_phi` module wire を削除
    - 未参照の `plan/pattern3_if_phi.rs` を削除（legacy entrypoint 群を撤去）
    - pattern domain 撤退の低リスク箇所を1段縮退（挙動不変）
  - `e53a7561d` refactor D5 remove no-op legacy domain-plan fallback in joinir router
    - `joinir/patterns/router.rs` から no-op な `legacy::lower_via_plan` 呼び出しを削除（`Ok(None)` 直返しへ）
    - `joinir/patterns/mod.rs` から `mod legacy;` を削除
    - 未使用になった `joinir/patterns/legacy.rs` を削除し、legacy no-op fallback の残骸を整理
  - `3bdb42d99` refactor D5 drop unused edgecfg api emit_frag facade reexport
    - `edgecfg/api/mod.rs` から未参照の `emit_frag` 再公開を削除
    - `FragEmitSession` を唯一の公開 emission 手順入口として維持（構造不変）
    - edgecfg facade surface を runtime 実使用に同期
  - `378257aa7` refactor D5 trim composer facade to runtime-used shadow adopt exports
    - `plan/composer/mod.rs` の shadow_adopt facade 再公開を runtime 実使用シンボルのみに縮退
    - 未使用だった release/shadow adopt 個別 helper の再公開群を削除
    - `#[allow(unused_imports)]` に頼らない facade へ整理（挙動不変）
  - `355dee3a6` refactor D5 remove dead generic-loop and recipe-tree facade reexports
    - `plan/generic_loop/mod.rs` から未使用 facade re-export 3群（extract/facts_types/shape_resolution）を削除
    - `plan/recipe_tree/mod.rs` から未使用 `VerifiedRecipeBlock` 再公開を削除
    - `allow(unused_imports)` 依存を増やさず、runtime 実使用面に再公開面積を同期
  - `6d85176d2` refactor D5 trim unused reexports in verifier policy and joinir merge facade
    - `plan/verifier/mod.rs` から未使用の `CanonicalLoopFacts` 再公開を削除
    - `plan/policies/.../balanced_depth_scan_policy.rs` を `BalancedDepthScanPolicyResult` のみ再公開へ縮退
    - `joinir/merge/mod.rs` の未使用 `MergeConfig` 再公開を削除し、`MergeContracts` 再公開の不要 suppression を撤去
    - re-export surface を実使用に寄せ、`allow(unused_imports)` 依存を削減
  - `0aac809b8` refactor D5 gate legacy normalization loop_with_post path to tests
    - `normalization/plan.rs` の `PlanKind::LoopWithPost` と `NormalizationPlan::loop_with_post()` を `#[cfg(test)]` 化
    - `normalization/execute_box.rs` の `execute_loop_with_post` を `#[cfg(test)]` 化
    - `joinir/routing.rs` / `normalized_shadow_suffix_router_box.rs` の legacy fallback arm を `#[cfg(test)]` 同期
    - release warning baseline を維持したまま `normalization` 側の `allow(dead_code)` を撤去
  - `805ba4731` refactor D5 trim control_flow dead_code allows in debug and loop-var helper path
    - `control_flow/debug.rs` の `trace_varmap` を `#[cfg(test)]` 化
    - `control_flow/utils.rs` の `extract_loop_variable_from_condition` は実使用経路（`plan/common_init.rs`）に合わせて suppression なしへ復帰
    - `control_flow/mod.rs` の wrapper method から不要 `allow(dead_code)` を撤去
  - `b08cefb24` refactor D5 gate joinir loop context module behind cfg(test)
    - `joinir/mod.rs` の `loop_context` module wire を `#[cfg(test)]` 化
    - `joinir/loop_context.rs` の module-level suppression (`allow(dead_code)`) を削除し、test専用補助箱として隔離
    - runtime build から未配線 context を除外して dead-noise を縮退
  - `c7d59f24a` refactor D5 trim joinir router context dead fields and test-only trace helpers
    - `joinir/patterns/router.rs` の未参照互換field（`has_continue` / `has_break` / `features`）と未使用 `with_skeleton` を削除
    - `LoopPatternContext.skeleton` は実使用経路（single_planner）に合わせて attribute なしへ復帰
    - `joinir/trace.rs` の未配線 helper（`is_varmap_enabled` / `phi` / `merge` / `exit_phi`）を `#[cfg(test)]` 化して runtime suppressions を撤去
  - `b14958530` refactor D5 reduce joinir merge dead-code surface and wire config knobs
    - `joinir/merge/config.rs` の `exit_reconnect_mode` / `allow_missing_exit_block` を runtime 側で実参照し、field-level `allow(dead_code)` を撤去
    - `coordinator/phase_5_6.rs` で `effective_mode = config override > boundary mode` を明示（既定挙動は不変）
    - `coordinator/phase_4_5.rs` で `allow_missing_exit_block` を contract 生成に反映（既定値 `true` 維持）
    - `merge_result.rs` の未使用 helper API (`new` / `add_*`) を削除
    - `exit_args_collector.rs` / `terminator_rewrite.rs` / `loop_header_phi_info.rs` で test-only 化と実参照化により `allow(dead_code)` を縮退
  - `9206a9052` refactor D5 remove exit_kind dead_code allows via strict sanity checks
    - `edgecfg/api/exit_kind.rs` の `Continue/Unwind/Cancel` と helper method の `allow(dead_code)` を撤去
    - `edgecfg/api/verify.rs` strict 入口に ExitKind semantic sanity check（debug_assert）を追加
    - suppression ではなく不変条件チェックで enum 語彙を保持
  - `f236a9736` refactor D5 drop redundant unused_imports allows in edgecfg api reexports
    - `edgecfg/api/mod.rs` の `BlockParams` 再公開から不要 `allow(unused_imports)` を削除
    - `edgecfg/api/compose/mod.rs` の `if_` 再公開から不要 `allow(unused_imports)` を削除
    - `emit_frag` 再公開は現状使用実態に合わせて抑制維持（warning baseline `0`）
  - `3c55a3d2f` refactor D5 gate edgecfg compose legacy helpers to test-only
    - `edgecfg/api/compose/{seq,loop_,cleanup}.rs` の legacy helper を `#[cfg(test)]` 化
    - `compose/mod.rs` の再公開を実使用に合わせて test-only 化（`if_` は runtime 維持）
    - `allow(dead_code)` 依存を増やさず warning baseline `0` を維持
  - `5ea8ef625` refactor D5 trim dead helper suppressions in remapper and edgecfg verify
    - `joinir_id_remapper.rs` の未使用 `remap_block` helper を削除
    - `edgecfg/api/verify.rs` の legacy `verify_frag_invariants` を `#[cfg(test)]` へ移行
    - `observe/resolve.rs` / `ast_lowerer/stmt_handlers.rs` の不要 `#[allow(dead_code)]` を撤去
    - release warning baseline `0` を維持
  - `e83ecc379` refactor D5 gate edgecfg test-only helpers behind cfg(test)
    - `edgecfg/api/frag.rs` の test-only helper（`add_exit` / `get_exits` / `exit_kinds`）を `#[cfg(test)]` 化
    - `edgecfg/api/edge_stub.rs` の `with_target` を `#[cfg(test)]` 化
    - suppression 依存を減らしつつ release warning `0` を維持
  - `4d73274b1` refactor D5 prune dead emission helpers and split scan module
    - `emission/loop_split_scan.rs` を削除（参照ゼロの未使用モジュール撤去）
    - `emission/compare.rs` の未使用 helper `emit_ne_to` を削除
    - `emission/copy_emitter.rs` の未使用 `CopyEmitReason` 4種を削除
    - `emission/mod.rs` の module wire/comment を実体に同期
  - `1bc5aa91f` refactor D5 drop local dead_code allows from clean plan modules
    - `plan/*` のクリーン13ファイルで局所 `#[allow(dead_code)]` を撤去（dirty file は非対象）
    - `pattern_pipeline.rs` / `pattern2_break_helpers.rs` / `exit_binding.rs` などの legacy attribute を縮退
    - `plan/mod.rs` umbrella 抑制を崩す前段として、局所 suppressions を先行削減
  - `495dfd032` refactor D5 remove unused predicate scan emitter and phi wrapper dead helper
    - `emission/loop_predicate_scan.rs` を削除（参照ゼロの未使用モジュール撤去）
    - `emission/phi.rs` の未使用 helper `insert_loop_phi` を削除し、module comment を現行責務に同期
    - `emission/mod.rs` の module wire/comment を実体に同期
  - `49c97d94f` fix D5 restore pattern2 promotion hint tag emission
  - `c2ab89104` refactor D5 prune dead single planner recipe-only guards
  - `936b7766a` docs D5 sync planner SSOT comments with domain-plan-only flow
  - `4d4effb81` docs phase29x classify run_nonzero monitor-known clusters
  - `17431bbae` refactor D5 collapse dead loop facts staging toggles
  - `22532f0bf` refactor D5 delete dead pattern8 plan-side module
  - `6d5b1ab7c` refactor D5 align planner shadow with semantic rule keys
  - `32f735d9c` refactor D5 gate facts loop_tests as test-only module
  - `23d5d2080` refactor D5 dedupe planner build tests with shared LoopFacts fixtures
    - `planner/build_tests.rs` を helper ベースへ再編（`LoopFacts` 重複初期化を共通化）
    - file size: `903 -> 425` lines（挙動不変）
  - `09da9205b` refactor D5 make planner legacy labels test-only in rule_order
    - `single_planner/rule_order.rs` の `planner_rule_legacy_name` を test-only 化
    - runtime 経路から Pattern番号ラベル map を分離（互換テストは維持）
  - `72826cb53` refactor D5 simplify registry handlers standard entry wiring
    - `handlers.rs` の標準ルート配線で `ENTRY_BASE + compose` 重複を削減（`const ENTRY` へ統一）
  - `150728b8c` refactor D5 dedupe registry candidate suppression logic
    - `registry/mod.rs` の `collect_candidates` で `entry.name` 判定重複を `should_skip_candidate` へ集約
    - `generic_loop_v1` 後段除外判定を単一ブールへ集約（挙動不変）
  - `2a90073f7` refactor D5 trim planner facts dead-noise helpers
    - `planner/build_tests.rs` の dead import (`BTreeMap`) を削除
    - `planner/build.rs` / `candidates.rs` / `validators.rs` の no-op 分岐を縮約
    - `facts/feature_facts.rs` の nested-loop 判定定型を簡約（挙動不変）
  - `fbcedc2bb` refactor D5 delete unused planner entrypoints and validator stubs
    - `planner/build.rs` の未参照 entrypoint（`build_plan` / `build_plan_from_facts`）を削除
    - `planner/validators.rs` の未参照 stub（strict/dev helper と exit-usage debug assert）を削除
    - `plan/mod.rs` / `planner/mod.rs` の再公開口とコメントを実体に同期
  - `88c9ade9a` refactor D5 drop stale dead_code allows in freeze and loop tests
    - `planner/freeze.rs` の `#![allow(dead_code)]` を削除
    - `facts/loop_tests.rs` の `#![allow(dead_code)]` と未使用 helper を削除
  - `0759144b3` refactor D5 remove stale dead_code allows from facts modules
    - `facts/mod.rs` と `facts/scan_shapes.rs` の `#![allow(dead_code)]` を削除
    - 既存 test/build/gate/probe が追加 warning なしで通ることを確認
  - `9e4a82e03` refactor D5 drop planner mod unused_imports file-level allow
    - `planner/mod.rs` の `#![allow(unused_imports)]` を削除
    - `planner` 再公開口の実使用に合わせ、file-level suppression を撤去
  - `da2e9aacc` refactor D5 dedupe registry predicate scan-family gating
    - `registry/predicates.rs` に `ScanFamilyPresence` を追加し、scan-family 除外判定を SSOT 化
    - `pred_loop_simple_while` / `pred_loop_cond_break_continue` / `pred_loop_cond_return_in_body` / `pred_generic_loop_v1`
      の重複判定を helper 経由へ統一（挙動維持）
  - `a0e9c80d4` refactor D5 drop stale dead_code allows in plan helper leaves
    - `features/exit_branch.rs` / `features/exit_map.rs` / `scan_loop_segments.rs` /
      `steps/loop_wiring_standard5.rs` / `plan_build_session.rs` の file-level `dead_code` allow を撤去
    - 未参照 wrapper `build_return_exit_plan_opt` と `build_standard5_loop_frag` を削除
  - `2daf90430` refactor D5 remove stale dead_code allows from scan recipe leaves
    - `loop_scan_methods_v0/recipe.rs` / `loop_scan_methods_block_v0/recipe.rs` /
      `loop_scan_phi_vars_v0/recipe.rs` / `loop_scan_v0/recipe.rs` / `loop_bundle_resolver_v0/recipe.rs`
      の file-level `dead_code` allow を撤去
  - `56b32f014` refactor D5 remove dead_code allow from join key and cond view
    - `join_key.rs` / `canon/cond_block_view.rs` の file-level `dead_code` allow を撤去
    - `PlanBuildSession` / 条件view 呼び出し経路で参照される最小共有箱の suppression を縮退
  - `68ae7bb56` refactor D5 drop stale dead_code allows in recipe core modules
    - `recipes/mod.rs` / `recipe_tree/block.rs` / `recipe_tree/verified.rs` /
      `loop_cond/break_continue_recipe.rs` の file-level `dead_code` allow を撤去
  - `5b2e3f70f` refactor D5 remove dead_code allow from normalize canonicalize
    - `normalize/mod.rs` / `normalize/canonicalize.rs` の file-level `dead_code` allow を撤去
    - canonical facts 変換の SSOT 経路を suppression なしで固定
  - `a4f53764e` refactor D5 remove dead_code allow from recipe contracts
    - `recipe_tree/contracts.rs` の file-level `dead_code` allow を撤去
    - contract 型定義（`RecipeContract*`）を suppression なしで維持
  - `4f147d760` refactor D5 remove dead_code allow from loop cond bc else patterns
    - `features/loop_cond_bc_else_patterns.rs` の file-level `dead_code` allow を撤去
    - else-only-return / else-guard-break recipe handler を suppression なしで維持
  - `a8485b441` refactor D5 remove dead_code allows from parts loop modules
    - `parts/conditional_update.rs` / `parts/loop_.rs` の file-level `dead_code` allow を撤去
    - loop body contract lowering と conditional update parts を suppression なしで維持
  - `1fc72c5a2` refactor D5 prune dead call helper utilities
    - `calls/function_lowering.rs` / `calls/special_handlers.rs` の未参照 helper 10件を削除
    - file-level `dead_code` allow を撤去し、call helper 層の dead-noise を縮退
  - `0f2812fc4` refactor D5 drop dead_code allow from joinir entry params check
    - `joinir/merge/contract_checks/entry_params.rs` の file-level `dead_code` allow を撤去
    - boundary entry-params contract check を suppression なしで維持
  - `017536db0` refactor D5 drop dead_code allow from joinir exit meta collector
    - `joinir/merge/exit_line/meta_collector.rs` の file-level `dead_code` allow を撤去
    - ExitMetaCollector の exit_bindings 収集箱を suppression なしで維持
  - `10c3235b0` refactor D5 drop dead_code allow from joinir patterns facade
    - `joinir/patterns/mod.rs` の file-level `dead_code` allow を撤去
    - plan layer への thin facade ルータを suppression なしで維持
  - `115cd97bb` refactor D5 remove unused joinir tail call lowering policy box
    - `joinir/merge/tail_call_lowering_policy.rs` を削除（未使用 policy box 撤去）
    - `merge/mod.rs` / `instruction_rewriter.rs` から旧 policy 配線を削除
    - `rewriter/exit_collection.rs` の comment を現行 k_exit 経路に同期
  - `0db58ae75` refactor D5 drop rewriter dead modules and remove mod-level dead_code allow
    - `joinir/merge/rewriter/mod.rs` の file-level `dead_code` allow を撤去
    - 未使用モジュール `rewriter/{exit_collection,logging,type_propagation}.rs` を削除
    - `RewrittenBlocks` / `RewriteContext` / terminator helper の未使用要素を縮退
    - `rewriter/README.md` を実体に同期
  - `2b285ed24` refactor D5 remove unused plan common contract_error module
    - `plan/common/contract_error.rs` を削除（参照ゼロの未使用モジュール撤去）
    - `plan/common/mod.rs` の module wire を実体に同期
  - `0982d80ee` refactor D5 remove unused plan common ast_helpers module
    - `plan/common/ast_helpers.rs` を削除（参照ゼロの未使用 helper モジュール撤去）
    - `plan/common/mod.rs` の module wire を実体に同期

- verification (latest cleanup round):
  - `cargo build --release --bin hakorune`（post-2f60b34fe, `PASS`）
  - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq`（`PASS`, post-2f60b34fe）
  - `tools/dev/direct_loop_progression_sweep.sh --profile phase29x-probe --allow-emit-fail`
    - `emit_fail=0`, `run_nonzero=18`, `run_ok=101`, `route_blocker=0`（total=119, post-2f60b34fe, elapsed=`0:03.72`）
  - `cargo build --release --bin hakorune`（post-2c5e56f27, `PASS`）
  - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq`（`PASS`, post-2c5e56f27）
  - `tools/dev/direct_loop_progression_sweep.sh --profile phase29x-probe --allow-emit-fail`
    - `emit_fail=0`, `run_nonzero=18`, `run_ok=101`, `route_blocker=0`（total=119, post-2c5e56f27, elapsed=`0:03.90`）
  - `cargo build --release --bin hakorune`（post-26c5942ad, `PASS`）
  - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq`（`PASS`, post-26c5942ad）
  - `tools/dev/direct_loop_progression_sweep.sh --profile phase29x-probe --allow-emit-fail`
    - `emit_fail=0`, `run_nonzero=18`, `run_ok=101`, `route_blocker=0`（total=119, post-26c5942ad, elapsed=`0:03.69`）
  - `cargo build --release --bin hakorune`（post-8022c367d, `PASS`）
  - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq`（`PASS`, post-8022c367d）
  - `tools/dev/direct_loop_progression_sweep.sh --profile phase29x-probe --allow-emit-fail`
    - `emit_fail=0`, `run_nonzero=18`, `run_ok=101`, `route_blocker=0`（total=119, post-8022c367d, elapsed=`0:04.28`）
  - `cargo build --release --bin hakorune`（post-ddd3821d1, `PASS`）
  - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq`（`PASS`, post-ddd3821d1）
  - `tools/dev/direct_loop_progression_sweep.sh --profile phase29x-probe --allow-emit-fail`
    - `emit_fail=0`, `run_nonzero=18`, `run_ok=101`, `route_blocker=0`（total=119, post-ddd3821d1, elapsed=`0:03.73`）
  - `cargo build --release --bin hakorune`（post-be2dc84cf, `PASS`）
  - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq`（`PASS`, post-be2dc84cf）
  - `tools/dev/direct_loop_progression_sweep.sh --profile phase29x-probe --allow-emit-fail`
    - `emit_fail=0`, `run_nonzero=18`, `run_ok=101`, `route_blocker=0`（total=119, post-be2dc84cf, elapsed=`0:03.66`）
  - `cargo build --release --bin hakorune`（post-734310f41, `PASS`）
  - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq`（`PASS`, post-734310f41）
  - `tools/dev/direct_loop_progression_sweep.sh --profile phase29x-probe --allow-emit-fail`
    - `emit_fail=0`, `run_nonzero=18`, `run_ok=101`, `route_blocker=0`（total=119, post-734310f41, elapsed=`0:04.79`）
  - `cargo build --release --bin hakorune`（post-0e23eef27, `PASS`）
  - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq`（`PASS`, post-0e23eef27）
  - `tools/dev/direct_loop_progression_sweep.sh --profile phase29x-probe --allow-emit-fail`
    - `emit_fail=0`, `run_nonzero=18`, `run_ok=101`, `route_blocker=0`（total=119, post-0e23eef27, elapsed=`0:04.95`）
  - `cargo build --release --bin hakorune`（post-d7842eafb, `PASS`）
  - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq`（`PASS`, post-d7842eafb）
  - `tools/dev/direct_loop_progression_sweep.sh --profile phase29x-probe --allow-emit-fail`
    - `emit_fail=0`, `run_nonzero=18`, `run_ok=101`, `route_blocker=0`（total=119, post-d7842eafb, elapsed=`0:05.08`）
  - `cargo build --release --bin hakorune`（post-0cff7a360, `PASS`）
  - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq`（`PASS`, post-0cff7a360）
  - `tools/dev/direct_loop_progression_sweep.sh --profile phase29x-probe --allow-emit-fail`
    - `emit_fail=0`, `run_nonzero=18`, `run_ok=101`, `route_blocker=0`（total=119, post-0cff7a360, elapsed=`0:04.67`）
  - `cargo build --release --bin hakorune`（post-061a18494, `PASS`）
  - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq`（`PASS`, post-061a18494）
  - `tools/dev/direct_loop_progression_sweep.sh --profile phase29x-probe --allow-emit-fail`
    - `emit_fail=0`, `run_nonzero=18`, `run_ok=101`, `route_blocker=0`（total=119, post-061a18494, elapsed=`0:04.98`）
  - `cargo build --release --bin hakorune`（post-95a12aaef, `PASS`）
  - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq`（`PASS`, post-95a12aaef）
  - `tools/dev/direct_loop_progression_sweep.sh --profile phase29x-probe --allow-emit-fail`
    - `emit_fail=0`, `run_nonzero=18`, `run_ok=101`, `route_blocker=0`（total=119, post-95a12aaef, elapsed=`0:05.00`）
  - `cargo build --release --bin hakorune`（post-7e422e1e8, `PASS`, warning `0`）
  - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq`（`PASS`, post-7e422e1e8）
  - `tools/dev/direct_loop_progression_sweep.sh --profile phase29x-probe --allow-emit-fail`
    - `emit_fail=0`, `run_nonzero=18`, `run_ok=101`, `route_blocker=0`（total=119, post-7e422e1e8, elapsed=`0:04.02`）
  - `cargo build --release --bin hakorune`（post-4c8c8bc05, `PASS`）
  - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq`（`PASS`, post-4c8c8bc05）
  - `tools/dev/direct_loop_progression_sweep.sh --profile phase29x-probe --allow-emit-fail`
    - `emit_fail=0`, `run_nonzero=18`, `run_ok=101`, `route_blocker=0`（total=119, post-4c8c8bc05）
  - `cargo build --release --bin hakorune`（post-619fbd64b, `PASS`）
  - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq`（`PASS`, post-619fbd64b）
  - `tools/dev/direct_loop_progression_sweep.sh --profile phase29x-probe --allow-emit-fail`
    - `emit_fail=0`, `run_nonzero=18`, `run_ok=101`, `route_blocker=0`（total=119, post-619fbd64b）
  - `cargo build --release --bin hakorune`（post-e53a7561d, `PASS`）
  - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq`（`PASS`, post-e53a7561d）
  - `tools/dev/direct_loop_progression_sweep.sh --profile phase29x-probe --allow-emit-fail`
    - `emit_fail=0`, `run_nonzero=18`, `run_ok=101`, `route_blocker=0`（total=119, post-e53a7561d）
  - `cargo build --release --bin hakorune`（post-3bdb42d99, `warning: 0`）
  - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq`（`PASS`, post-3bdb42d99）
  - `tools/dev/direct_loop_progression_sweep.sh --profile phase29x-probe --allow-emit-fail`
    - `emit_fail=0`, `run_nonzero=18`, `run_ok=101`, `route_blocker=0`（total=119, post-3bdb42d99）
  - `cargo build --release --bin hakorune`（post-378257aa7, `warning: 0`）
  - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq`（`PASS`, post-378257aa7）
  - `tools/dev/direct_loop_progression_sweep.sh --profile phase29x-probe --allow-emit-fail`
    - `emit_fail=0`, `run_nonzero=18`, `run_ok=101`, `route_blocker=0`（total=119, post-378257aa7）
  - `cargo build --release --bin hakorune`（post-355dee3a6, `warning: 0`）
  - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq`（`PASS`, post-355dee3a6）
  - `tools/dev/direct_loop_progression_sweep.sh --profile phase29x-probe --allow-emit-fail`
    - `emit_fail=0`, `run_nonzero=18`, `run_ok=101`, `route_blocker=0`（total=119, post-355dee3a6）
  - `cargo build --release --bin hakorune`（post-6d85176d2, `warning: 0`）
  - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq`（`PASS`, post-6d85176d2）
  - `tools/dev/direct_loop_progression_sweep.sh --profile phase29x-probe --allow-emit-fail`
    - `emit_fail=0`, `run_nonzero=18`, `run_ok=101`, `route_blocker=0`（total=119, post-6d85176d2）
  - `cargo build --release --bin hakorune`（post-0aac809b8, `warning: 0`）
  - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq`（`PASS`, post-0aac809b8）
  - `tools/dev/direct_loop_progression_sweep.sh --profile phase29x-probe --allow-emit-fail`
    - `emit_fail=0`, `run_nonzero=18`, `run_ok=101`, `route_blocker=0`（total=119, post-0aac809b8）
  - `cargo build --release --bin hakorune`（post-805ba4731, `warning: 0`）
  - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq`（`PASS`, post-805ba4731）
  - `tools/dev/direct_loop_progression_sweep.sh --profile phase29x-probe --allow-emit-fail`
    - `emit_fail=0`, `run_nonzero=18`, `run_ok=101`, `route_blocker=0`（total=119, post-805ba4731）
  - `cargo build --release --bin hakorune`（post-b08cefb24, `warning: 0`）
  - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq`（`PASS`, post-b08cefb24）
  - `tools/dev/direct_loop_progression_sweep.sh --profile phase29x-probe --allow-emit-fail`
    - `emit_fail=0`, `run_nonzero=18`, `run_ok=101`, `route_blocker=0`（total=119, post-b08cefb24）
  - `cargo build --release --bin hakorune`（post-c7d59f24a, `warning: 0`）
  - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq`（`PASS`, post-c7d59f24a）
  - `tools/dev/direct_loop_progression_sweep.sh --profile phase29x-probe --allow-emit-fail`
    - `emit_fail=0`, `run_nonzero=18`, `run_ok=101`, `route_blocker=0`（total=119, post-c7d59f24a）
  - `cargo build --release --bin hakorune`（post-b14958530, `warning: 0`）
  - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq`（`PASS`, post-b14958530）
  - `tools/dev/direct_loop_progression_sweep.sh --profile phase29x-probe --allow-emit-fail`
    - `emit_fail=0`, `run_nonzero=18`, `run_ok=101`, `route_blocker=0`（total=119, post-b14958530）
  - `cargo build --release --bin hakorune`（post-9206a9052）
  - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq`（`PASS`, post-9206a9052）
  - `tools/dev/direct_loop_progression_sweep.sh --profile phase29x-probe --allow-emit-fail`
    - `emit_fail=0`, `run_nonzero=18`, `run_ok=101`, `route_blocker=0`（total=119, post-9206a9052）
  - `cargo build --release --bin hakorune`（warning baseline re-check: `warning: 0`, post-9206a9052）
  - `cargo build --release --bin hakorune`（post-f236a9736）
  - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq`（`PASS`, post-f236a9736）
  - `tools/dev/direct_loop_progression_sweep.sh --profile phase29x-probe --allow-emit-fail`
    - `emit_fail=0`, `run_nonzero=18`, `run_ok=101`, `route_blocker=0`（total=119, post-f236a9736）
  - `cargo build --release --bin hakorune`（warning baseline re-check: `warning: 0`, post-f236a9736）
  - `cargo build --release --bin hakorune`（post-3c55a3d2f）
  - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq`（`PASS`, post-3c55a3d2f）
  - `tools/dev/direct_loop_progression_sweep.sh --profile phase29x-probe --allow-emit-fail`
    - `emit_fail=0`, `run_nonzero=18`, `run_ok=101`, `route_blocker=0`（total=119, post-3c55a3d2f）
  - `cargo build --release --bin hakorune`（warning baseline re-check: `warning: 0`, post-3c55a3d2f）
  - `cargo build --release --bin hakorune`（post-5ea8ef625）
  - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq`（`PASS`, post-5ea8ef625）
  - `tools/dev/direct_loop_progression_sweep.sh --profile phase29x-probe --allow-emit-fail`
    - `emit_fail=0`, `run_nonzero=18`, `run_ok=101`, `route_blocker=0`（total=119, post-5ea8ef625）
  - `cargo build --release --bin hakorune`（warning baseline re-check: `warning: 0`, post-5ea8ef625）
  - `cargo build --release --bin hakorune`（post-e83ecc379）
  - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq`（`PASS`, post-e83ecc379）
  - `tools/dev/direct_loop_progression_sweep.sh --profile phase29x-probe --allow-emit-fail`
    - `emit_fail=0`, `run_nonzero=18`, `run_ok=101`, `route_blocker=0`（total=119, post-e83ecc379）
  - `cargo build --release --bin hakorune`（warning baseline re-check: `warning: 0`, post-e83ecc379）
  - `cargo build --release --bin hakorune`（post-4d73274b1）
  - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq`（`PASS`, post-4d73274b1）
  - `tools/dev/direct_loop_progression_sweep.sh --profile phase29x-probe --allow-emit-fail`
    - `emit_fail=0`, `run_nonzero=18`, `run_ok=101`, `route_blocker=0`（total=119, post-4d73274b1）
  - `cargo build --release --bin hakorune`（warning baseline re-check: `warning: 0`, post-4d73274b1）
  - `cargo build --release --bin hakorune`（post-1bc5aa91f）
  - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq`（`PASS`, post-1bc5aa91f）
  - `tools/dev/direct_loop_progression_sweep.sh --profile phase29x-probe --allow-emit-fail`
    - `emit_fail=0`, `run_nonzero=18`, `run_ok=101`, `route_blocker=0`（total=119, post-1bc5aa91f）
  - `cargo build --release --bin hakorune`（warning baseline re-check: `warning: 0`, post-1bc5aa91f）
  - `cargo build --release --bin hakorune`（post-495dfd032）
  - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq`（`PASS`, post-495dfd032）
  - `tools/dev/direct_loop_progression_sweep.sh --profile phase29x-probe --allow-emit-fail`
    - `emit_fail=0`, `run_nonzero=18`, `run_ok=101`, `route_blocker=0`（total=119, post-495dfd032）
  - `cargo build --release --bin hakorune`（warning baseline re-check: `warning: 0`）
  - `cargo test -q --lib planner_skips_split_scan_domain_plan`
  - `cargo test -q --lib planner_prefers_none_when_no_candidates`
  - `cargo test -q --lib planner_skips_scan_with_init_domain_plan`
  - `cargo test -q --lib planner_ignores_scan_with_init_negative_step`
  - `cargo test -q --lib planner_ignores_scan_with_init_feature_staging`
  - `cargo test -q --lib planner_gates_non_loop_skeletons`
  - `cargo test -q --lib planner_does_not_build_pattern1_simplewhile_plan_from_facts`
  - `cargo test -q --lib planner_does_not_build_pattern1_char_map_plan_from_facts`
  - `cargo test -q --lib planner_does_not_build_pattern1_array_join_plan_from_facts`
  - `cargo test -q --lib planner_does_not_build_pattern8_bool_predicate_scan_plan_from_facts`
  - `cargo test -q --lib planner_does_not_build_pattern9_accum_const_loop_plan_from_facts`
  - `cargo build --release --bin hakorune`
  - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq` (`PASS`)
  - `cargo test -q --lib rule_name_uses_semantic_label`
  - `cargo test -q --lib legacy_rule_name_alias_is_preserved`
  - `cargo test -q --lib planner_rule_order_is_domain_plan_only`
  - `cargo test -q --lib planner_first_tag_keeps_scan_split_compat_labels`
  - `tools/dev/direct_loop_progression_sweep.sh --profile phase29x-probe --allow-emit-fail`
    - `emit_fail=0`, `run_nonzero=18`, `run_ok=101`, `route_blocker=0`（total=119）
  - `bash tools/smokes/v2/profiles/integration/joinir/phase29bj_planner_required_pattern6_7_pack_vm.sh` (`PASS`)
  - `bash tools/smokes/v2/profiles/integration/joinir/phase29bo_planner_required_pattern8_9_pack_vm.sh` (`PASS`)
  - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq` (`PASS`, post-registry-cleanup)
  - `cargo test -q --lib planner_prefers_none_when_no_candidates`
  - `cargo test -q --lib planner_does_not_build_pattern9_accum_const_loop_plan_from_facts`
  - `cargo test -q --lib nested_loop_detects_if_branch_loop`
  - `cargo build --release --bin hakorune`
  - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq` (`PASS`, post-dead-noise-trim)
  - `tools/dev/direct_loop_progression_sweep.sh --profile phase29x-probe --allow-emit-fail`
    - `emit_fail=0`, `run_nonzero=18`, `run_ok=101`, `route_blocker=0`（total=119, post-dead-noise-trim）
  - `cargo test -q --lib planner_prefers_none_when_no_candidates`
  - `cargo test -q --lib planner_gates_non_loop_skeletons`
  - `cargo test -q --lib planner_does_not_build_pattern9_accum_const_loop_plan_from_facts`
  - `cargo build --release --bin hakorune`
  - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq` (`PASS`, post-unused-entry-delete)
  - `tools/dev/direct_loop_progression_sweep.sh --profile phase29x-probe --allow-emit-fail`
    - `emit_fail=0`, `run_nonzero=18`, `run_ok=101`, `route_blocker=0`（total=119, post-unused-entry-delete）
  - `cargo test -q --lib loop_facts_require_skeleton_and_features_when_present`
  - `cargo test -q --lib planner_prefers_none_when_no_candidates`
  - `cargo build --release --bin hakorune`
  - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq` (`PASS`, post-dead_code-allow-drop)
  - `tools/dev/direct_loop_progression_sweep.sh --profile phase29x-probe --allow-emit-fail`
    - `emit_fail=0`, `run_nonzero=18`, `run_ok=101`, `route_blocker=0`（total=119, post-dead_code-allow-drop）
  - `cargo test -q --lib planner_prefers_none_when_no_candidates`
  - `cargo test -q --lib loopfacts_ok_some_for_canonical_scan_with_init_minimal`
  - `cargo build --release --bin hakorune`
  - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq` (`PASS`, post-facts-allow-drop)
  - `tools/dev/direct_loop_progression_sweep.sh --profile phase29x-probe --allow-emit-fail`
    - `emit_fail=0`, `run_nonzero=18`, `run_ok=101`, `route_blocker=0`（total=119, post-facts-allow-drop）
  - `cargo test -q --lib planner_prefers_none_when_no_candidates`
  - `cargo build --release --bin hakorune`
  - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq` (`PASS`, post-planner-mod-allow-drop)
  - `tools/dev/direct_loop_progression_sweep.sh --profile phase29x-probe --allow-emit-fail`
    - `emit_fail=0`, `run_nonzero=18`, `run_ok=101`, `route_blocker=0`（total=119, post-planner-mod-allow-drop）
  - `cargo build --release --bin hakorune`
  - `bash tools/smokes/v2/profiles/integration/joinir/phase29bo_planner_required_pattern8_9_pack_vm.sh` (`PASS`)
  - `bash tools/smokes/v2/profiles/integration/joinir/phase29bj_planner_required_pattern6_7_pack_vm.sh` (`PASS`)
  - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq` (`PASS`, post-registry-predicate-dedupe)
  - `tools/dev/direct_loop_progression_sweep.sh --profile phase29x-probe --allow-emit-fail`
    - `emit_fail=0`, `run_nonzero=18`, `run_ok=101`, `route_blocker=0`（total=119, post-registry-predicate-dedupe）
  - `cargo test -q --lib planner_prefers_none_when_no_candidates`
  - `cargo build --release --bin hakorune`
  - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq` (`PASS`, post-plan-helper-leaf-cleanup)
  - `tools/dev/direct_loop_progression_sweep.sh --profile phase29x-probe --allow-emit-fail`
    - `emit_fail=0`, `run_nonzero=18`, `run_ok=101`, `route_blocker=0`（total=119, post-plan-helper-leaf-cleanup）
  - `cargo test -q --lib planner_prefers_none_when_no_candidates`
  - `cargo build --release --bin hakorune`
  - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq` (`PASS`, post-scan-recipe-allow-drop)
  - `tools/dev/direct_loop_progression_sweep.sh --profile phase29x-probe --allow-emit-fail`
    - `emit_fail=0`, `run_nonzero=18`, `run_ok=101`, `route_blocker=0`（total=119, post-scan-recipe-allow-drop）
  - `cargo test -q --lib planner_prefers_none_when_no_candidates`
  - `cargo build --release --bin hakorune`
  - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq` (`PASS`, post-joinkey-condview-cleanup)
  - `tools/dev/direct_loop_progression_sweep.sh --profile phase29x-probe --allow-emit-fail`
    - `emit_fail=0`, `run_nonzero=18`, `run_ok=101`, `route_blocker=0`（total=119, post-joinkey-condview-cleanup）
  - `cargo test -q --lib planner_prefers_none_when_no_candidates`
  - `cargo build --release --bin hakorune`
  - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq` (`PASS`, post-recipe-core-allow-drop)
  - `tools/dev/direct_loop_progression_sweep.sh --profile phase29x-probe --allow-emit-fail`
    - `emit_fail=0`, `run_nonzero=18`, `run_ok=101`, `route_blocker=0`（total=119, post-recipe-core-allow-drop）
  - `cargo test -q --lib planner_prefers_none_when_no_candidates`
  - `cargo test -q --lib canonical_projects_skeleton_and_exit_usage`
  - `cargo build --release --bin hakorune`
  - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq` (`PASS`, post-normalize-allow-drop)
  - `tools/dev/direct_loop_progression_sweep.sh --profile phase29x-probe --allow-emit-fail`
    - `emit_fail=0`, `run_nonzero=18`, `run_ok=101`, `route_blocker=0`（total=119, post-normalize-allow-drop）
  - `cargo build --release --bin hakorune`
  - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq` (`PASS`, post-recipe-contract-allow-drop)
  - `tools/dev/direct_loop_progression_sweep.sh --profile phase29x-probe --allow-emit-fail`
    - `emit_fail=0`, `run_nonzero=18`, `run_ok=101`, `route_blocker=0`（total=119, post-recipe-contract-allow-drop）
  - `cargo test -q --lib planner_prefers_none_when_no_candidates`
  - `cargo build --release --bin hakorune`
  - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq` (`PASS`, post-loop-cond-bc-else-pattern-allow-drop)
  - `tools/dev/direct_loop_progression_sweep.sh --profile phase29x-probe --allow-emit-fail`
    - `emit_fail=0`, `run_nonzero=18`, `run_ok=101`, `route_blocker=0`（total=119, post-loop-cond-bc-else-pattern-allow-drop）
  - `cargo test -q --lib planner_prefers_none_when_no_candidates`
  - `cargo build --release --bin hakorune`
  - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq` (`PASS`, post-parts-loop-allow-drop)
  - `tools/dev/direct_loop_progression_sweep.sh --profile phase29x-probe --allow-emit-fail`
    - `emit_fail=0`, `run_nonzero=18`, `run_ok=101`, `route_blocker=0`（total=119, post-parts-loop-allow-drop）
  - `cargo test -q --lib planner_prefers_none_when_no_candidates`
  - `cargo build --release --bin hakorune`
  - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq` (`PASS`, post-calls-dead-helper-prune)
  - `tools/dev/direct_loop_progression_sweep.sh --profile phase29x-probe --allow-emit-fail`
    - `emit_fail=0`, `run_nonzero=18`, `run_ok=101`, `route_blocker=0`（total=119, post-calls-dead-helper-prune）
  - `cargo test -q --lib planner_prefers_none_when_no_candidates`
  - `cargo build --release --bin hakorune`
  - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq` (`PASS`, post-joinir-entry-params-allow-drop)
  - `tools/dev/direct_loop_progression_sweep.sh --profile phase29x-probe --allow-emit-fail`
    - `emit_fail=0`, `run_nonzero=18`, `run_ok=101`, `route_blocker=0`（total=119, post-joinir-entry-params-allow-drop）
  - `cargo test -q --lib planner_prefers_none_when_no_candidates`
  - `cargo build --release --bin hakorune`
  - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq` (`PASS`, post-joinir-exit-meta-collector-allow-drop)
  - `tools/dev/direct_loop_progression_sweep.sh --profile phase29x-probe --allow-emit-fail`
    - `emit_fail=0`, `run_nonzero=18`, `run_ok=101`, `route_blocker=0`（total=119, post-joinir-exit-meta-collector-allow-drop）
  - `cargo test -q --lib planner_prefers_none_when_no_candidates`
  - `cargo build --release --bin hakorune`
  - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq` (`PASS`, post-joinir-patterns-facade-allow-drop)
  - `tools/dev/direct_loop_progression_sweep.sh --profile phase29x-probe --allow-emit-fail`
    - `emit_fail=0`, `run_nonzero=18`, `run_ok=101`, `route_blocker=0`（total=119, post-joinir-patterns-facade-allow-drop）
  - `cargo test -q --lib planner_prefers_none_when_no_candidates`
  - `cargo build --release --bin hakorune`
  - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq` (`PASS`, post-tail-call-policy-box-removal)
  - `tools/dev/direct_loop_progression_sweep.sh --profile phase29x-probe --allow-emit-fail`
    - `emit_fail=0`, `run_nonzero=18`, `run_ok=101`, `route_blocker=0`（total=119, post-tail-call-policy-box-removal）
  - `cargo test -q --lib planner_prefers_none_when_no_candidates`
  - `cargo build --release --bin hakorune`
  - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq` (`PASS`, post-rewriter-dead-module-prune)
  - `tools/dev/direct_loop_progression_sweep.sh --profile phase29x-probe --allow-emit-fail`
    - `emit_fail=0`, `run_nonzero=18`, `run_ok=101`, `route_blocker=0`（total=119, post-rewriter-dead-module-prune）
  - `cargo test -q --lib planner_prefers_none_when_no_candidates`
  - `cargo build --release --bin hakorune`
  - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq` (`PASS`, post-plan-common-contract-error-removal)
  - `tools/dev/direct_loop_progression_sweep.sh --profile phase29x-probe --allow-emit-fail`
    - `emit_fail=0`, `run_nonzero=18`, `run_ok=101`, `route_blocker=0`（total=119, post-plan-common-contract-error-removal）
  - `cargo test -q --lib planner_prefers_none_when_no_candidates`
  - `cargo build --release --bin hakorune`
  - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq` (`PASS`, post-plan-common-ast-helpers-removal)
  - `tools/dev/direct_loop_progression_sweep.sh --profile phase29x-probe --allow-emit-fail`
    - `emit_fail=0`, `run_nonzero=18`, `run_ok=101`, `route_blocker=0`（total=119, post-plan-common-ast-helpers-removal）

- key behavior lock (kept green):
  - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq`
  - `bash tools/smokes/v2/profiles/integration/joinir/phase29bj_planner_required_pattern6_7_pack_vm.sh`
  - `bash tools/smokes/v2/profiles/integration/joinir/phase29bo_planner_required_pattern8_9_pack_vm.sh`
  - `cargo build --release --bin hakorune`

- known note:
  - `cargo test -q --lib facts_extracts_pattern9_const_accum_success` は現作業ツリーで既存 mismatch（本ラウンド差分では未変更）
  - `plan/mod.rs` の file-level `dead_code` allow は現時点で撤去不可（撤去試行時に `cargo build` で `233 warnings` 顕在化）。
  - 残 suppressions（2026-03-04 時点）:
    - `plan/mod.rs`（umbrella / remove時 233 warnings）
    - `plan/extractors/common_helpers.rs`（他差分が同居する dirty file のため未着手）
    - `plan/loop_cond_unified_helpers.rs`（他差分が同居する dirty file のため未着手）

## next fixed order (resume point)

1. Pattern語彙の主語外し（step-1）: `single_planner/rule_order` / `joinir/patterns/registry` の runtime 文言を semantic 名へ統一。
2. `phase29bq_fast_gate_vm --only bq` と `phase29x-probe` を各 cleanup で継続し、`emit_fail=0` / `route_blocker=0` を維持。
3. `shadow_adopt` 縮退（step-2）: `recipe_contract.is_some()` 経路で strict/release fallback 禁止は適用済み。次は fallback 本体の撤去条件を固定する。
4. `DomainPlan` 縮退（step-3）: 1-variant 現状を label-only 化し、normalizer 直通依存を段階撤去。
   - `single_planner` 内部は outcome-first 化済み（`2c5e56f27`）。外側 tuple 呼び出しを段階撤去する。
   - 残 tuple 呼び出し（移行待ち）: `plan/features/generic_loop_body/helpers.rs`（dirty 同居差分を分離して次ラウンドで実施）
5. 進捗ログの時系列は archive 側へ寄せ、root pointer は fixed order と blocker だけを更新。

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

## Archive

- full historical log (2111 lines, archived 2026-03-04):
  - `docs/development/current/main/investigations/current_task_archive_2026-03-04.md`
- policy:
  - 長文の時系列ログは以後 archive 側へ追記し、`CURRENT_TASK.md` は再起動用の薄い入口を維持する。
