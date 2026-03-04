# CURRENT_TASK (root pointer)

Status: SSOT
Date: 2026-03-04
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
  1. `phase29x-probe` を定点観測し、`emit_fail=0` / `route_blocker=0` を維持確認。
  2. D5 cleanup 継続: facts/planner 側の dead entry を isolate -> delete。
  3. D5 cleanup 継続: test-only wire の `#[cfg(test)]` 化と不要 module wire 削除。
  4. D5 cleanup 継続: planner/build の legacy negative test 群を DomainPlan-only 契約へ整形。

## Compiler Cleanup Order (2026-03-04, SSOT)

- D5-A: facts/planner dead staging 削除（挙動不変）
- D5-B: runtime 未参照の legacy module を isolate -> delete
- D5-C: diagnostic-only vocabulary を semantic key に揃える
- D5-D: test-only module wire を runtime build から分離
- D5-E: planner/build test 群を DomainPlan-only の事実に合わせて縮退

## Latest Probe Snapshot (direct route)

- command:
  - `tools/dev/direct_loop_progression_sweep.sh --profile phase29x-probe --allow-emit-fail`
- latest result (2026-03-04):
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

## Restart Handoff (2026-03-04)

- this round commits:
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

- verification (latest cleanup round):
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

- key behavior lock (kept green):
  - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq`
  - `bash tools/smokes/v2/profiles/integration/joinir/phase29bj_planner_required_pattern6_7_pack_vm.sh`
  - `bash tools/smokes/v2/profiles/integration/joinir/phase29bo_planner_required_pattern8_9_pack_vm.sh`
  - `cargo build --release --bin hakorune`

- known note:
  - `cargo test -q --lib facts_extracts_pattern9_const_accum_success` は現作業ツリーで既存 mismatch（本ラウンド差分では未変更）

## next fixed order (resume point)

1. D5-E: `facts/planner` / `registry` の残り dead-noise を clean file 限定で継続縮退（挙動不変のみ）。
2. `phase29bq_fast_gate_vm --only bq` と `phase29x-probe` を各 cleanup で継続し、`emit_fail=0` / `route_blocker=0` を維持。
3. compiler cleanliness の次段（Pattern/domain 撤退に向けた isolate-first）へ接続する前に、`CURRENT_TASK.md` を再起動入口の薄さで維持。
4. 進捗ログの時系列は archive 側へ寄せ、root pointer は fixed order と blocker だけを更新。

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
