---
Status: Active
Scope: planner-required master list を selfhost 入口へ接続（.hako 側の導線準備）
Related:
- docs/development/current/main/10-Now.md
- docs/development/current/main/30-Backlog.md
- docs/development/current/main/design/coreplan-migration-roadmap-ssot.md
- docs/development/current/main/design/joinir-planner-required-gates-ssot.md
- docs/development/current/main/phases/phase-29bp/README.md
- docs/development/current/main/phases/phase-29bq/29bq-90-selfhost-checklist.md
- docs/development/current/main/phases/phase-29bq/29bq-91-mirbuilder-migration-progress-checklist.md
- docs/development/current/main/phases/phase-29bq/29bq-92-parser-handoff-checklist.md
- docs/development/current/main/phases/phase-29bq/29bq-113-hako-recipe-first-migration-lane.md
- docs/development/current/main/phases/phase-29x/29x-62-derust-done-sync-ssot.md
---

# Phase 29bq: planner-required → selfhost entry wiring

## Goal

- planner-required（strict/dev）を「selfhost 側からも 1 コマンドで回せる入口」に接続する。
- 既定挙動は不変（release default unchanged）。
- 既存 gate（29ae regression / planner-required v4）は緑維持。

## Non-goals

- 新しい language feature の追加
- by-name 分岐
- silent fallback の復活

## Priority (SSOT)

この phase は selfhost canary を扱うが、**最優先は compiler 側（Facts/Normalizer/CorePlan）の表現力と構造の収束**。
“selfhost を通す” は目的ではなく、ブロッカー形の抽出・契約固定（fixture+fast gate）の入口として使う。

- 方針 SSOT: `docs/development/current/main/design/compiler-expressivity-first-policy.md`
- レゴ化（完成品キット増殖を防ぐ）SSOT: `docs/development/current/main/design/lego-composability-policy.md`
- Task order SSOT: `docs/development/current/main/design/compiler-task-map-ssot.md`

### Task Order Snapshot (SSOT)

Phase 29bq の実装順序は `compiler-task-map-ssot.md` に従う（selfhost に引きずられない）。

1) CondProfile stabilization → 2) VerifiedRecipe boundary tightening → 3) GenericLoopV1 acceptance closure → 4) LoopCond recipe-only tag pinning → 5) historical DomainPlan wording cleanup → 6) idx_var contract → 7) Selfhost/OOM

## Plan (P0-P3)

- P0: docs-first（入口/SSOT の 1 枚化）
- P1: selfhost 入口（スクリプト/runner）設計と SSOT 固定
- P2: 実装（入口追加）→ 既存 gate green 維持
- P3: closeout（post-change green + SSOT 更新）
- Note: ExitBranch moved to `features/exit_branch.rs` (behavior unchanged)

## Active checklists (SSOT)

- Selfhost / gate operations: `29bq-90-selfhost-checklist.md`
- MirBuilder migration progress ledger: `29bq-91-mirbuilder-migration-progress-checklist.md`
- Parser handoff operations: `29bq-92-parser-handoff-checklist.md`
- `.hako` Recipe-first migration lane: `29bq-113-hako-recipe-first-migration-lane.md`
- `.hako` Cleanup integration prep lane: `29bq-114-hako-cleanup-integration-prep-lane.md`
- Selfhost closeout checklist: `29bq-115-selfhost-to-go-checklist.md`

## Planned next (after 29bq)

- CorePlan cleanup after “Loop as structural box”（優先: CleanupWrap / Select / `loop(true|cond)` の小箱で混線を減らす）
  - SSOT: `docs/development/current/main/design/cleanupwrap-cleanup-region-boundary-ssot.md`
- Design consult: keep surface `if` syntax unchanged; unify internally via CondBlockView (canon-layer, no rewrite).
  - SSOT: `docs/development/current/main/design/condblockview-desugar-consult.md`
- Design SSOT (must-do early, avoid debt): block expressions / condition blocks semantics & staging.
  - SSOT: `docs/development/current/main/design/block-expressions-and-condition-blocks-ssot.md`
- Follow-up (planned): reuse CondBlockView as the condition-entry interface for match guard conditions, so canon/cond
  growth benefits `if/loop/while/match-guard` uniformly (no new surface syntax required).

## Planned next (inside 29bq, expressivity-first)

selfhost は opt-in canary として維持しつつ、`.hako` 側の workaround ではなく compiler 側の “小箱” で unblock する。

- StepPlacement を Facts で保持し、step の扱いを `StepMode` として分離する（no rewrite）
  - `ExtractToStepBB`（既存の安全経路）
  - `InlineInBody`（追加予定: step を body の位置で lower）
- Verifier 契約（最初は conservative）:
  - `InlineInBody` は continue 禁止 + step_bb 空
- 付随の土台（語彙を増やさないスロット）:
  - LoopFrame の `ContinueTarget` を SSOT 化（将来 continue 解禁時に使う）

SSOT (design):
- `docs/development/current/main/design/coreloop-stepmode-inline-in-body-ssot.md`

### Special-rule triggers (avoid “exception pileup”)

selfhost canary を “最小fixture→fast gate→最小拡張” で前に進める際、次のカテゴリに入った場合は
「その場しのぎの受理拡張」で通さず、Freeze taxonomy + 専用 SSOT を先に作る（= special rule 扱い）。
入口SSOTは `docs/development/current/main/design/coreplan-skeleton-feature-model.md` の Section 3。

- irreducible / multi-entry loop（skeleton が一意に決まらない）
- unwind / finally / cleanup 境界（ExitMap+Cleanup が必須になる）
- coroutine/yield（関数内制御が閉じない）
- 評価順/回数/数値規約に触れる必要がある “見かけ等価変形” を要求するケース（no rewrite に抵触）

## BoxCount rule (phase-29bq)

ブロッカー対応は BoxCount で進めるが、**完成品キット（大箱の複製）を増やさない**ことを明文化する。

- 1ブロッカー=1fixture=1fast gate pin は維持
- ただし、次に当てはまる場合は BoxCount を先にやらず BoxShape を優先する:
  - “数だけ増える軸”（clusterN / nested loop 個数など）で、箱が増殖しそう
  - pipeline が skeleton/PHI/walker を複製しており、既存の共通部品SSOTに寄せられる
  - allowlist/composer の例外（`shadow_adopt` 等）により、変更点が散って見落としやすい

## Planned BoxShape work (compiler-first)

Phase 29bq の fast iteration を維持しつつ、完成品キット化を止めるために BoxShape の優先キューを固定する。

- reject_reason → handoff を 1 箇所のテーブルSSOTへ（順序依存で偶然通る状態を減らす）
- nested loop “数だけ増える” 問題（clusterN）を profile/table 化して追加点を 1 箇所へ（箱増殖を抑制）
- pipeline の共通工程（carrier init / join payload / wires / frag build）を step 化して SSOT に押し込む
- composer allowlist（`shadow_adopt` 等）の罠を減らす（legacy route 名を含むエラー/チェックリスト/自動検出）

## BoxCount restart plan (one-pager)

目的: BoxShape 完了後の受理拡張を **1ブロッカー=1受理形=1fixture=1コミット** で再開する。

1) 再開判断: loop → if → loop の fixture pin を **1件だけ**進める（generic_loop_v1 recipe-first）。
2) 入口: selfhostの実作業に引きずられない。必要最小の受理形を先に固定（fixture + fast gate）。
3) 手順: fixture追加 → fast gate 1件 pin → 単体gate → コミット。
4) 禁止: BoxShape/入口整理と混ぜない。AST rewrite / silent fallback 禁止。

## Gate (SSOT)

- Loopless subset: `./tools/hako_check_loopless_gate.sh`
- Planner-required dev gate v4: `./tools/smokes/v2/profiles/integration/joinir/phase29bp_planner_required_dev_gate_v4_vm.sh`
- JoinIR regression pack: `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh`
- Fast iteration (29bq): `./tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh`
- Fast gate list (SSOT): `tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_cases.tsv`（case_id + reason 1行）
- Selfhost planner-required entry: `SMOKES_ENABLE_SELFHOST=1 ./tools/smokes/v2/profiles/integration/selfhost/phase29bq_selfhost_planner_required_dev_gate_vm.sh`
- Baseline (implementation safety): `./tools/smokes/v2/profiles/integration/joinir/phase29bs_fast_gate_vm.sh --full`
- Loop(true) multi-break gate: `./tools/smokes/v2/profiles/integration/joinir/phase29bq_loop_true_multi_break_planner_required_vm.sh`
- Conditional update/join gate: `./tools/smokes/v2/profiles/integration/joinir/phase29bq_conditional_update_join_planner_required_vm.sh`
- Loop(cond) multi-exit gate: `./tools/smokes/v2/profiles/integration/joinir/phase29bq_loop_cond_multi_exit_planner_required_vm.sh`
- Step-before-break gate: `./tools/smokes/v2/profiles/integration/joinir/phase29bq_step_then_tail_break_planner_required_vm.sh`
- General-if-in-body gate: `./tools/smokes/v2/profiles/integration/joinir/phase29bq_general_if_in_loop_body_planner_required_vm.sh`
- LoopContinueOnly multidelta gate（legacy script name keeps `pattern4continue`）: `./tools/smokes/v2/profiles/integration/joinir/phase29bq_pattern4continue_multidelta_planner_required_vm.sh`

## Acceptance criteria (RC)

- selfhost 入口（P2 で追加）を 1 コマンドで実行でき、RC=0 で通る（ログ導線あり）。
- subset TSV を 1 → 3 本まで段階的に増やして PASS（freeze が出たら増加停止し、README に LOG を固定）。
- master TSV は opt-in で少数だけ流せる（全件強制はしない）。
- `phase29bp_planner_required_dev_gate_v4_vm.sh` / `phase29ae_regression_pack_vm.sh` / `hako_check_loopless_gate.sh` が緑維持。

## De-rust done handshake (boundary)

- `phase29bq` の fast/planner-required/selfhost gate は lane A/B の運用証跡であり、de-rust transfer lane done 判定とは別契約。
- done 判定は `docs/development/current/main/phases/phase-29x/29x-62-derust-done-sync-ssot.md`
  の X32/X33/X34/X35 matrix replay を正本とする。

## Selfhost entry (SSOT)

- Entry command: `SMOKES_ENABLE_SELFHOST=1 ./tools/smokes/v2/profiles/integration/selfhost/phase29bq_selfhost_planner_required_dev_gate_vm.sh`
- Baseline check: `./tools/smokes/v2/profiles/integration/joinir/phase29bs_fast_gate_vm.sh --full`
- Opt-in: `SMOKES_ENABLE_SELFHOST=1` がない場合は SKIP（RC=0）。
- Log contract: 失敗時は最後に `LOG: /tmp/phase29bq_selfhost_<case>.log` を出力する。
- Execution path: `compiler.hako --stage-b --stage3` で Program(JSON v0) を生成し、`--json-file` で実行する（strict/dev + planner-required）。
- Compiler entry wiring SSOT: `src/mir/builder/control_flow/plan/REGISTRY.md`（Entry SSOT: router→planner→composer→lower）
- Default list: `tools/smokes/v2/profiles/integration/selfhost/planner_required_selfhost_subset.tsv`（selfhost 実行可能な subset）
- Master list: `SMOKES_SELFHOST_LIST=tools/smokes/v2/profiles/integration/joinir/planner_required_cases.tsv` で指定する。

## LoopTrueBreak gate (P2)

- Fixture: `apps/tests/phase29bq_loop_true_multi_break_parserish_min.hako`
- Tag: `[joinir/planner_first rule=LoopTrueBreak]`
- Note: `loop(true)` の break/continue 受理は **fallback-only**（他候補がある場合は既存パターンを優先し、競合で freeze しない）

## Conditional update/join gate (P2)

- Fixture: `apps/tests/phase29bq_conditional_update_join_min.hako`
- Tag: `[joinir/planner_first rule=LoopSimpleWhile]`

## Loop(cond) multi-exit gate (P2)

- Fixture: `apps/tests/phase29bq_loop_cond_multi_exit_min.hako`
- Tag: `[joinir/planner_first rule=LoopCondBreak]`
- Note: break-only を既存SSOTとして維持しつつ、break+continue 混在も保守的に受理（continue-only は拒否）

## Parser-ish handled loop (contract)

- Shape: `loop(cond){ handled=0; ...; if handled==0 { break } ; <update>; }`
- Allowed: local/assign/pure cond/exit-if(break)/conditional update(select)
- Forbidden: return / nested loop / non-pure conditional update / rewrite
- Tag: `[joinir/planner_first rule=LoopCondBreak]` (temporary)
- Gate: `phase29bq_fast_gate_cases.tsv` case_id=parserish_handled

## Selfhost fixtures (SSOT)

- Selfhost-derived fixtures are tracked only in `tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_cases.tsv`.
- README keeps just the current blocker and the canary log path.
- AI handoff/debug workflow SSOT: `docs/development/current/main/design/ai-handoff-and-debug-contract.md`
- Reject→Handoff / Gap taxonomy SSOT: `docs/development/current/main/design/plan-reject-handoff-gap-taxonomy-ssot.md`

## Selfhost Progress Board (frontier)

Frontier-only summary (full list lives in the TSVs above).

| Category | Latest pinned boundary | Fixture | Status |
| --- | --- | --- | --- |
| if (non-loop) | if-fallthrough join | `apps/tests/phase29bq_selfhost_blocker_parse_program2_if_fallthrough_join_min.hako` | pinned |
| loop if-return | if-else-if-else return | `apps/tests/phase29bq_selfhost_blocker_parse_program2_loop_if_else_if_else_return_min.hako` | pinned |
| nested loop | inner if-else fallthrough join else-return local | `apps/tests/phase29bq_selfhost_blocker_parse_program2_nested_loop_if_else_fallthrough_join_else_return_local_min.hako` | pinned |
| module_roots/using | priority ([modules] wins) | `apps/tests/phase29bq_using_module_roots_priority_min.hako` | pinned |

## Selfhost Acceptance Map (draft)

Minimal mapping from fixture → accept predicate → recipe → lower entry → gate case_id.
Update when a new accept shape is added.

Note: Gate case_id is `selfhost-only` if not found in fast gate TSV.
Body lowering policy: `body_exit_allowed` is used only when `BodyLoweringPolicy::ExitAllowed` (allow_extended=true and no ThenOnlyBreakIf); `allow_join_if=false` is fixed.

| Fixture | Accept predicate (Facts) | Recipe builder | Lower entry | Gate case_id |
| --- | --- | --- | --- | --- |
| `apps/tests/phase118_pattern3_if_sum_min.hako` | `if_phi_join/facts.rs`（legacy key: `pattern3_ifphi`） | `(legacy)` | `if_phi_join route`（legacy: `pattern3_pipeline.rs`） | `if_sum min` |
| `apps/tests/phase29bq_selfhost_blocker_rewriteknown_try_apply_loop_true_else_exit_min.hako` | `loop_true_break_continue/facts.rs` | `loop_true_break_continue/recipe.rs` | `loop_true_break_continue_pipeline/` | `selfhost_rewriteknown_try_apply_loop_true_else_exit_min` |
| `apps/tests/phase29bq_selfhost_subset_scan_funcs_import_min.hako` | `break_continue/facts.rs` | `break_continue/recipe.rs` | `loop_cond_break_continue_pipeline/` | `selfhost-only` |
| `apps/tests/phase29bq_map_literal_percent_min.hako` | `(no-loop)` | `step_tree` | `if_lowering/` | `map_literal_percent_min` |
| `apps/tests/phase29bq_strict_nested_loop_guard_min.hako` | `composer/shadow_adopt.rs (strict_nested_loop_guard)` | `(freeze)` | `shadow_pre_plan_guard` | `strict_nested_loop_guard_min` |
| `apps/tests/phase29bq_strict_nested_loop_guard_accept_min.hako` | `composer/shadow_adopt.rs (strict_nested_loop_guard, accept-min1)` | `(n/a)` | `recipe_first loop_continue_only accept-min1` | `strict_nested_loop_guard_accept_min` |
| `apps/tests/phase29bq_selfhost_blocker_trim_generic_loop_min.hako` | `generic_loop/facts/` | `generic_loop/recipe.rs` | `generic_loop_pipeline.rs` | `selfhost_trim_generic_loop_min` |
| `apps/tests/phase29bq_selfhost_blocker_rewriteknown_trim_loop_cond_and_methodcall_min.hako` | `generic_loop/facts/` | `generic_loop/recipe.rs` | `generic_loop_pipeline.rs` | `selfhost_rewriteknown_trim_loop_cond_and_methodcall_min` |
| `apps/tests/phase29bq_selfhost_blocker_parse_if_min.hako` | `(no-loop)` | `step_tree` | `if_lowering/` | `selfhost_parse_if_min` |
| `apps/tests/phase29bq_selfhost_blocker_parse_program2_if_return_min.hako` | `(no-loop)` | `step_tree` | `if_lowering/` | `selfhost_parse_program2_if_return_min` |
| `apps/tests/phase29bq_selfhost_blocker_parse_program2_if_return_var_min.hako` | `(no-loop)` | `step_tree` | `if_lowering/` | `selfhost_parse_program2_if_return_var_min` |
| `apps/tests/phase29bq_selfhost_blocker_parse_program2_if_return_local_min.hako` | `(no-loop)` | `step_tree` | `if_lowering/` | `selfhost_parse_program2_if_return_local_min` |
| `apps/tests/phase29bq_selfhost_blocker_parse_program2_if_fallthrough_join_min.hako` | `(no-loop)` | `step_tree` | `if_lowering/` | `selfhost_parse_program2_if_fallthrough_join_min` |
| `apps/tests/phase29bq_selfhost_blocker_parse_program2_if_else_return_min.hako` | `(no-loop)` | `step_tree` | `if_lowering/` | `selfhost_parse_program2_if_else_return_min` |
| `apps/tests/phase29bq_selfhost_blocker_parse_program2_if_else_return_var_min.hako` | `(no-loop)` | `step_tree` | `if_lowering/` | `selfhost_parse_program2_if_else_return_var_min` |
| `apps/tests/phase29bq_selfhost_blocker_parse_program2_if_else_return_local_min.hako` | `(no-loop)` | `step_tree` | `if_lowering/` | `selfhost_parse_program2_if_else_return_local_min` |
| `apps/tests/phase29bq_selfhost_blocker_parse_program2_if_else_if_return_min.hako` | `(no-loop)` | `step_tree` | `if_lowering/` | `selfhost_parse_program2_if_else_if_return_min` |
| `apps/tests/phase29bq_selfhost_blocker_rewriteknown_itoa_complex_step_min.hako` | `generic_loop/facts/` | `generic_loop/recipe.rs` | `generic_loop_pipeline.rs` | `selfhost_rewriteknown_itoa_complex_step_min` |
| `apps/tests/phase29bq_selfhost_blocker_while_cap_min.hako` | `generic_loop/facts/` | `generic_loop/recipe.rs` | `generic_loop_pipeline.rs` | `while_cap` |
| `apps/tests/phase29bq_selfhost_blocker_scan_methods_loop_min.hako` | `generic_loop/facts/` | `generic_loop/recipe.rs` | `generic_loop_pipeline.rs` | `selfhost_blocker_scan_methods_loop_min` |
| `apps/tests/phase29bq_selfhost_blocker_collect_using_entries_loop_min.hako` | `loop_collect_using_entries_v0/facts.rs` | `exit_only_block.rs (ExitAllowed)` | `loop_collect_using_entries_v0/pipeline.rs` | `selfhost_collect_using_entries_loop_min` |
| `apps/tests/phase29bq_selfhost_blocker_decode_escapes_loop_min.hako` | `generic_loop/facts/` | `generic_loop/recipe.rs` | `generic_loop_pipeline.rs` | `selfhost_decode_escapes_loop_min` |
| `apps/tests/phase29bq_selfhost_blocker_phi_injector_collect_phi_vars_len_loop_min.hako` | `generic_loop/facts/` | `generic_loop/recipe.rs` | `generic_loop_pipeline.rs` | `phi_injector_len_loop` |
| `apps/tests/phase29bq_selfhost_blocker_phi_injector_collect_phi_vars_k_loop_no_exit_min.hako` | `generic_loop/facts/` | `generic_loop/recipe.rs` | `generic_loop_pipeline.rs` | `selfhost_phi_injector_k_loop_no_exit_min` |
| `apps/tests/phase29bq_selfhost_blocker_parse_map_min.hako` | `generic_loop/facts/` | `generic_loop/recipe.rs` | `generic_loop_pipeline.rs` | `selfhost_parse_map_min` |
| `apps/tests/phase29bq_selfhost_blocker_parse_block_expr_min.hako` | `generic_loop/facts/` | `generic_loop/recipe.rs` | `generic_loop_pipeline.rs` | `selfhost_parse_block_expr_min` |
| `apps/tests/phase29bq_selfhost_blocker_parse_term2_min.hako` | `loop_true_break_continue/facts.rs` | `loop_true_break_continue/recipe.rs` | `loop_true_break_continue_pipeline/` | `selfhost_parse_term2_min` |
| `apps/tests/phase29bq_selfhost_blocker_parse_string2_min.hako` | `break_continue/facts.rs` | `break_continue/recipe.rs` | `loop_cond_break_continue_pipeline/` | `selfhost_parse_string2_min` |
| `apps/tests/phase29bq_selfhost_blocker_parse_program2_ws_min.hako` | `break_continue/facts.rs` | `break_continue/recipe.rs` | `loop_cond_break_continue_pipeline/` | `selfhost_parse_program2_ws_min` |
| `apps/tests/phase29bq_selfhost_blocker_parse_program2_ws_or_min.hako` | `break_continue/facts.rs` | `break_continue/recipe.rs` | `loop_cond_break_continue_pipeline/` | `selfhost_parse_program2_ws_or_min` |
| `apps/tests/phase29bq_selfhost_blocker_parse_program2_cond_and_min.hako` | `loop_true_break_continue/facts.rs` | `loop_true_break_continue/recipe.rs` | `loop_true_break_continue_pipeline/` | `selfhost_parse_program2_cond_and_min` |
| `apps/tests/phase29bq_selfhost_blocker_parse_program2_effect_if_min.hako` | `break_continue/facts.rs` | `break_continue/recipe.rs` | `loop_cond_break_continue_pipeline/` | `selfhost_parse_program2_effect_if_min` |
| `apps/tests/phase29bq_selfhost_blocker_parse_program2_bool_or_mod_min.hako` | `break_continue/facts.rs` | `break_continue/recipe.rs` | `loop_cond_break_continue_pipeline/` | `selfhost_parse_program2_bool_or_mod_min` |
| `apps/tests/phase29bq_selfhost_blocker_parse_program2_loop_min.hako` | `break_continue/facts.rs` | `break_continue/recipe.rs` | `loop_cond_break_continue_pipeline/` | `selfhost_parse_program2_loop_min` |
| `apps/tests/phase29bq_selfhost_blocker_parse_program2_loop_if_fallthrough_join_min.hako` | `break_continue/facts.rs` | `break_continue/recipe.rs` | `loop_cond_break_continue_pipeline/` | `selfhost_parse_program2_loop_if_fallthrough_join_min` |
| `apps/tests/phase29bq_selfhost_blocker_parse_program2_loop_if_return_min.hako` | `break_continue/facts.rs` | `break_continue/recipe.rs` | `loop_cond_break_continue_pipeline/` | `selfhost_parse_program2_loop_if_return_min` |
| `apps/tests/phase29bq_selfhost_blocker_parse_program2_loop_if_return_var_min.hako` | `break_continue/facts.rs` | `break_continue/recipe.rs` | `loop_cond_break_continue_pipeline/` | `selfhost_parse_program2_loop_if_return_var_min` |
| `apps/tests/phase29bq_selfhost_blocker_parse_program2_loop_if_return_local_min.hako` | `break_continue/facts.rs` | `break_continue/recipe.rs` | `loop_cond_break_continue_pipeline/` | `selfhost_parse_program2_loop_if_return_local_min` |
| `apps/tests/phase29bq_selfhost_blocker_parse_program2_loop_continue_if_min.hako` | `break_continue/facts.rs` | `break_continue/recipe.rs` | `loop_cond_break_continue_pipeline/` | `selfhost_parse_program2_loop_continue_if_min` |
| `apps/tests/phase29bq_selfhost_blocker_parse_program2_loop_if_else_return_min.hako` | `break_continue/facts.rs` | `break_continue/recipe.rs` | `loop_cond_break_continue_pipeline/` | `selfhost_parse_program2_loop_if_else_return_min` |
| `apps/tests/phase29bq_selfhost_blocker_parse_program2_loop_if_else_return_var_min.hako` | `break_continue/facts.rs` | `break_continue/recipe.rs` | `loop_cond_break_continue_pipeline/` | `selfhost_parse_program2_loop_if_else_return_var_min` |
| `apps/tests/phase29bq_selfhost_blocker_parse_program2_loop_if_else_return_local_min.hako` | `break_continue/facts.rs` | `break_continue/recipe.rs` | `loop_cond_break_continue_pipeline/` | `selfhost_parse_program2_loop_if_else_return_local_min` |
| `apps/tests/phase29bq_selfhost_blocker_parse_program2_loop_if_else_if_return_min.hako` | `break_continue/facts.rs` | `break_continue/recipe.rs` | `loop_cond_break_continue_pipeline/` | `selfhost_parse_program2_loop_if_else_if_return_min` |
| `apps/tests/phase29bq_selfhost_blocker_parse_program2_loop_if_else_if_else_return_min.hako` | `break_continue/facts.rs` | `break_continue/recipe.rs` | `loop_cond_break_continue_pipeline/` | `selfhost_parse_program2_loop_if_else_if_else_return_min` |
| `apps/tests/phase29bq_selfhost_blocker_parse_program2_nested_loop_if_return_min.hako` | `generic_loop/facts/` | `generic_loop/recipe.rs` | `generic_loop_pipeline.rs` | `selfhost_parse_program2_nested_loop_if_return_min` |
| `apps/tests/phase29bq_selfhost_blocker_parse_program2_nested_loop_if_else_return_min.hako` | `generic_loop/facts/` | `generic_loop/recipe.rs` | `generic_loop_pipeline.rs` | `selfhost_parse_program2_nested_loop_if_else_return_min` |
| `apps/tests/phase29bq_selfhost_blocker_parse_program2_nested_loop_if_return_var_min.hako` | `generic_loop/facts/` | `generic_loop/recipe.rs` | `generic_loop_pipeline.rs` | `selfhost_parse_program2_nested_loop_if_return_var_min` |
| `apps/tests/phase29bq_selfhost_blocker_parse_program2_nested_loop_if_return_local_min.hako` | `generic_loop/facts/` | `generic_loop/recipe.rs` | `generic_loop_pipeline.rs` | `selfhost_parse_program2_nested_loop_if_return_local_min` |
| `apps/tests/phase29bq_selfhost_blocker_parse_program2_nested_loop_if_else_return_var_min.hako` | `generic_loop/facts/` | `generic_loop/recipe.rs` | `generic_loop_pipeline.rs` | `selfhost_parse_program2_nested_loop_if_else_return_var_min` |
| `apps/tests/phase29bq_selfhost_blocker_parse_program2_nested_loop_if_else_return_local_min.hako` | `generic_loop/facts/` | `generic_loop/recipe.rs` | `generic_loop_pipeline.rs` | `selfhost_parse_program2_nested_loop_if_else_return_local_min` |
| `apps/tests/phase29bq_selfhost_blocker_parse_program2_nested_loop_if_else_if_return_min.hako` | `generic_loop/facts/` | `generic_loop/recipe.rs` | `generic_loop_pipeline.rs` | `selfhost_parse_program2_nested_loop_if_else_if_return_min` |
| `apps/tests/phase29bq_selfhost_blocker_parse_program2_nested_loop_if_else_if_else_return_min.hako` | `break_continue/facts.rs` | `break_continue/recipe.rs` | `loop_cond_break_continue_pipeline/` | `selfhost_parse_program2_nested_loop_if_else_if_else_return_min` |
| `apps/tests/phase29bq_selfhost_blocker_parse_program2_nested_loop_if_fallthrough_join_min.hako` | `break_continue/facts.rs` | `break_continue/recipe.rs` | `loop_cond_break_continue_pipeline/` | `selfhost_parse_program2_nested_loop_if_fallthrough_join_min` |
| `apps/tests/phase29bq_selfhost_blocker_parse_program2_nested_loop_if_else_fallthrough_join_min.hako` | `break_continue/facts.rs` | `break_continue/recipe.rs` | `loop_cond_break_continue_pipeline/` | `selfhost_parse_program2_nested_loop_if_else_fallthrough_join_min` |
| `apps/tests/phase29bq_selfhost_blocker_parse_program2_nested_loop_if_else_fallthrough_join_return_var_min.hako` | `break_continue/facts.rs` | `break_continue/recipe.rs` | `loop_cond_break_continue_pipeline/` | `selfhost_parse_program2_nested_loop_if_else_fallthrough_join_return_var_min` |
| `apps/tests/phase29bq_selfhost_blocker_parse_program2_nested_loop_if_else_fallthrough_join_return_local_min.hako` | `break_continue/facts.rs` | `break_continue/recipe.rs` | `loop_cond_break_continue_pipeline/` | `selfhost_parse_program2_nested_loop_if_else_fallthrough_join_return_local_min` |
| `apps/tests/phase29bq_selfhost_blocker_parse_program2_nested_loop_if_else_fallthrough_join_return_blockexpr_min.hako` | `break_continue/facts.rs` | `break_continue/recipe.rs` | `loop_cond_break_continue_pipeline/` | `selfhost_parse_program2_nested_loop_if_else_fallthrough_join_return_blockexpr_min` |
| `apps/tests/phase29bq_selfhost_blocker_parse_program2_nested_loop_if_else_fallthrough_join_else_return_blockexpr_min.hako` | `break_continue/facts.rs` | `break_continue/recipe.rs` | `loop_cond_break_continue_pipeline/` | `selfhost_parse_program2_nested_loop_if_else_fallthrough_join_else_return_blockexpr_min` |
| `apps/tests/phase29bq_selfhost_blocker_parse_program2_nested_loop_if_else_fallthrough_join_else_return_blockexpr_var_min.hako` | `break_continue/facts.rs` | `break_continue/recipe.rs` | `loop_cond_break_continue_pipeline/` | `selfhost_parse_program2_nested_loop_if_else_fallthrough_join_else_return_blockexpr_var_min` |
| `apps/tests/phase29bq_selfhost_blocker_parse_program2_nested_loop_if_else_fallthrough_join_else_return_local_min.hako` | `ElseOnlyReturnIf` + else-local prelude | `ElseOnlyReturnIf` | `else_patterns.rs` | `selfhost_parse_program2_nested_loop_if_else_fallthrough_join_else_return_local_min` |
| `apps/tests/phase29bq_selfhost_blocker_parse_program2_nested_loop_if_else_fallthrough_join_else_return_local_min.hako` | `ElseOnlyReturnIf` + else-local prelude | `ElseOnlyReturnIf` | `else_patterns.rs` | `selfhost_parse_program2_nested_loop_if_else_fallthrough_join_else_return_local_min` |
| `apps/tests/phase29bq_loop_cond_else_only_return_print_min.hako` | `ElseOnlyReturnIf` + else-print prelude | `ElseOnlyReturnIf` | `else_patterns.rs` | `loop_cond_else_only_return_print_min` |
| `apps/tests/phase29bq_selfhost_blocker_parse_program2_nested_loop_if_else_fallthrough_join_return_blockexpr_local_min.hako` | `break_continue/facts.rs` | `break_continue/recipe.rs` | `loop_cond_break_continue_pipeline/` | `selfhost_parse_program2_nested_loop_if_else_fallthrough_join_return_blockexpr_local_min` |
| `apps/tests/phase29bq_selfhost_blocker_parse_program2_nested_loop_if_else_fallthrough_join_return_blockexpr_local2_min.hako` | `break_continue/facts.rs` | `break_continue/recipe.rs` | `loop_cond_break_continue_pipeline/` | `selfhost_parse_program2_nested_loop_if_else_fallthrough_join_return_blockexpr_local2_min` |
| `apps/tests/phase29bq_selfhost_blocker_parse_program2_nested_loop_if_else_fallthrough_join_return_var_blockexpr_min.hako` | `break_continue/facts.rs` | `break_continue/recipe.rs` | `loop_cond_break_continue_pipeline/` | `selfhost_parse_program2_nested_loop_if_else_fallthrough_join_return_var_blockexpr_min` |
| `apps/tests/phase29bq_selfhost_blocker_parse_program2_nested_loop_if_else_fallthrough_join_return_local_blockexpr_min.hako` | `ThenOnlyReturnIf` + single local-init pure (BlockExpr allowed) | `ThenOnlyReturnIf` | `else_patterns.rs` | `selfhost_parse_program2_nested_loop_if_else_fallthrough_join_return_local_blockexpr_min` |
| `apps/tests/phase29bq_selfhost_blocker_parse_program2_nested_loop_if_else_fallthrough_join_return_blockexpr_blockexpr_min.hako` | `break_continue/facts.rs` | `break_continue/recipe.rs` | `loop_cond_break_continue_pipeline/` | `selfhost_parse_program2_nested_loop_if_else_fallthrough_join_return_blockexpr_blockexpr_min` |
| `apps/tests/phase29bq_selfhost_blocker_parse_program2_nested_loop_if_else_fallthrough_join_return_blockexpr_var_min.hako` | `break_continue/facts.rs` | `break_continue/recipe.rs` | `loop_cond_break_continue_pipeline/` | `selfhost_parse_program2_nested_loop_if_else_fallthrough_join_return_blockexpr_var_min` |
| `apps/tests/phase29bq_selfhost_blocker_scan_methods_nested_loop_depth1_no_break_or_continue_min.hako` | `break_continue/facts.rs` | `break_continue/recipe.rs` | `loop_cond_break_continue_pipeline/` | `selfhost_scan_methods_nested_loop_depth1_no_break_or_continue_min` |
| `apps/tests/phase29bq_selfhost_blocker_scan_methods_nested_loop_depth1_no_break_or_continue_pure_min.hako` | `break_continue/facts.rs` | `break_continue/recipe.rs` | `loop_cond_break_continue_pipeline/` | `selfhost_scan_methods_nested_loop_depth1_no_break_or_continue_pure_min` |
| `apps/tests/phase29bq_selfhost_blocker_scan_methods_loop_min.hako` | `generic_loop/facts/` | `generic_loop/recipe.rs` | `generic_loop_pipeline.rs` | `selfhost_blocker_scan_methods_loop_min` |
| `apps/tests/phase29bq_selfhost_blocker_scan_with_quote_loop_min.hako` | `return_in_body/facts.rs` | `return_in_body/recipe.rs` | `loop_cond_return_in_body_pipeline.rs` | `selfhost_scan_with_quote_loop_min` |
| `apps/tests/phase29bq_selfhost_blocker_scan_with_quote_loop_full_min.hako` | `return_in_body/facts.rs` | `return_in_body/recipe.rs` | `loop_cond_return_in_body_pipeline.rs` | `selfhost_scan_with_quote_loop_full_min` |
| `apps/tests/phase29bq_selfhost_blocker_usingcollector_loop_full_min.hako` | `generic_loop/facts/` | `generic_loop/recipe.rs` | `generic_loop_pipeline.rs` | `selfhost_usingcollector_loop_full_min` |
| `apps/tests/phase29bq_selfhost_blocker_peek_parse_min.hako` | `generic_loop/facts/` | `generic_loop/recipe.rs` | `generic_loop_pipeline.rs` | `selfhost_peek_parse_min` |
| `apps/tests/phase29bq_selfhost_blocker_loop_cond_if_assign_min.hako` | `break_continue/facts.rs` | `break_continue/recipe.rs` | `loop_cond_break_continue_pipeline/` | `selfhost_loop_cond_if_assign_min` |
| `apps/tests/phase29bq_using_module_roots_min.hako` | `(no-loop)` | `step_tree` | `if_lowering/` | `using_module_roots_min` |
| `apps/tests/phase29bq_using_module_roots_multi_min.hako` | `(no-loop)` | `step_tree` | `if_lowering/` | `using_module_roots_multi_min` |
| `apps/tests/phase29bq_using_module_roots_priority_min.hako` | `(no-loop)` | `step_tree` | `if_lowering/` | `using_module_roots_priority_min` |
| `apps/tests/phase29bq_pattern2_break_recipe_only_min.hako` | `loop_break/facts.rs`（legacy key: `pattern2_break`） | `recipe_tree/composer.rs`（loop_break recipe） | `loop_break route`（semantic helper: `normalizer::normalize_loop_break`, legacy file: `pattern2_break.rs`） | `pattern2 recipe-only` |

## Selfhost Goal Checklist (frontier)

High-level progress only. Keep this in sync with the board above and the TSVs.

- [x] Fast gate green: `phase29bq_fast_gate_vm.sh`
- [x] Selfhost canary green (debug-fuel unlimited, retry-once rule for `scan_methods_loop_min`)
- [x] if (non-loop) boundaries pinned (return + join)
- [x] loop if-return boundaries pinned (if/else/else-if/else-if-else)
- [x] nested loop boundaries pinned (inner if-return + inner if-return var + inner if-return local + inner if-else return + inner if-else return var + inner if-else return local + inner if-else-if return + inner if-else-if-else return + inner if-fallthrough join + inner if-else fallthrough join + inner if-else fallthrough join return var + inner if-else fallthrough join return local + inner if-else fallthrough join return blockexpr + inner if-else fallthrough join return blockexpr-local + inner if-else fallthrough join return var blockexpr + inner if-else fallthrough join return local blockexpr + inner if-else fallthrough join return blockexpr-blockexpr + inner if-else fallthrough join return blockexpr-var + inner if-else fallthrough join else-return blockexpr + inner if-else fallthrough join else-return blockexpr-var + inner if-else fallthrough join else-return local)
- [x] module_roots/using pinned (multi using + [modules] override)
- [x] BlockExpr return pinned (fast gate)
- [x] Stage-B Program JSON v0 BlockExpr return pinned
- [x] BlockExpr exit-forbidden fail-fast in fast gate (optional)

Next actions (post-common):

- Run fast gate routinely; selfhost canary only at milestone cadence
- Add new fixture only when real selfhost run hits a new freeze/reject
- Update Acceptance Map when a new fixture is pinned

## Selfhost Expansion Policy (post-common)

After the common patterns are pinned, stop expanding the subset.
Add new fixtures only when a real selfhost run hits a new freeze/reject.

- New fixture rule: add 1 fixture only after `/tmp/*summary` + first_freeze_or_reject is recorded
- Health checks only: keep fast gate green; selfhost canary is milestone-only (not every change)
- Selfhost canary cadence: run on new blockers or after N fixture pins; otherwise skip
- Single-case runs: use `SMOKES_SELFHOST_LIST=...` to validate just one case
- Logging: record the summary path in `CURRENT_TASK.md` (1 line) and stop

## Selfhost Inventory (reported)

Inventory summary for remaining selfhost work (draft, reconcile with SSOT as needed).

### De-Rust roadmap (moved to SSOT)

Use the dedicated roadmap SSOT:
- `docs/development/current/main/design/de-rust-compiler-thin-rust-roadmap-ssot.md`
- Daily identity default (stage1-first): `bash tools/selfhost_identity_check.sh --mode smoke --skip-build`
- Default artifact paths (`cli-mode=stage1`): `target/selfhost/hakorune.stage1_cli` / `target/selfhost/hakorune.stage1_cli.stage2`
- `--cli-mode auto` is compatibility-only; fallback to stage0 is reported via `[identity/compat-fallback]` and is not accepted as full-mode evidence

This section is kept only as historical snapshot/context. Operational order is defined in the SSOT above.

| Snapshot phase | Rough scope | Where it maps in this repo |
| --- | --- | --- |
| Phase 0 | MIR JSON v0 + linear | Stage0→Stage1→Stage2 identity + entry contract pins (Progress Board + TSVs) |
| Phase 1 | if patterns | `if (non-loop)` pins (return/join) |
| Phase 2 | basic loops | `loop(cond)` / `generic_loop_v0` pins (fast gate + planner_required packs) |
| Phase 3 | loop exits | `loop(true)`, break/continue, ExitIfTree / exit-allowed recipes |
| Phase 4 | nested | nested loop pins + carrier/phi fixes (Progress Board: `nested loop`) |
| Phase 5 | JoinIR integration | PHI/CFG merge tracked by CorePlan/JoinIR SSOT (no new SSOT here) |
| Phase 6 | selfhost complete | Stage identity verification + canary/E2E expansion |

#### Phase 5/6 breakdown (draft)

- Phase 5 (JoinIR integration):
  - [x] Add a PHI/CFG merge fixture and pin in fast gate
  - [x] Update Acceptance Map entry (Facts/Recipe/Lower)
  - [x] JoinIR gates green
  - Evidence gate: `tools/smokes/v2/profiles/integration/joinir/phase29bq_phase5_joinir_integration_gate_vm.sh`
- Phase 6 (selfhost complete):
  - [x] Stage0→Stage1→Stage2 identity tests green
    - Evidence: `tools/selfhost_identity_check.sh --mode full --skip-build --bin-stage1 target/selfhost/hakorune.stage1_cli --bin-stage2 target/selfhost/hakorune.stage1_cli.stage2` (2026-02-25 PASS)
  - [x] `.hako mirbuilder` pin smokes green
    - Evidence: `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_hako_mirbuilder_quick_suite_vm.sh` (2026-02-25 PASS)
  - [x] Selfhost canary green at milestone cadence
    - Evidence: `RUN_TIMEOUT_SECS=120 SMOKES_ENABLE_SELFHOST=1 HAKO_JOINIR_PLANNER_REQUIRED=1 bash tools/smokes/v2/profiles/integration/selfhost/phase29bq_selfhost_planner_required_dev_gate_vm.sh` (`198/198`, `total_secs=649`, `avg_case_secs=3.28`, 2026-02-25 PASS)

### MIRBuilder Phase-4+ (draft)

| Phase | Status | Major tasks | Done when |
| --- | --- | --- | --- |
| Phase 10 | in_progress | LoopInfo expansion; NestedLoopFacts type | fixture pinned + fast gate green + Acceptance Map row |
| Phase 11 | in_progress | StepFacts type; body step extraction | fixture pinned + fast gate green + StepTree extractor updated |
| Phase 12 | in_progress | LoopPlanBuilder trait; Plan generation | planner_required gate green + Acceptance Map row |
| Phase 13 | in_progress | CorePlan emission; 5-block standardization | core plan fixture pinned + fast gate green |
| Phase 14 | in_progress | PHI node unification; exit merge | PHI fixture pinned + JoinIR gate green |
| Phase 15 | in_progress | Nested loop lowering | nested loop fixture pinned + selfhost canary green |
| Phase 16 | in_progress | Canary expansion; E2E tests | selfhost canary green (milestone) + E2E list updated |
| Phase 17+ | deferred | While/match/complex patterns | decision in docs + fixtures pinned |

### Parser migration (draft)

| Priority | Task | Status |
| --- | --- | --- |
| P0 | BlockExpr validation (value contexts beyond basic return) | done (pinned) |
| P0 | cond_prelude support | done |
| P1 | not operator support | done |
| P1 | E2E tests expansion | done |
| P2 | error message improvements | later |
| P3 | performance optimization | later |

#### Parser migration detail (draft)

- BlockExpr validation:
  - [x] enumerate contexts (normalizer / JSON v0 / condition)
    - normalizer: `blockexpr_basic_min`
    - JSON v0: `stageb_blockexpr_return_min`
    - condition: `cond_prelude_planner_required_min`
  - [x] add fixture per context and pin（all three are listed in `phase29bq_fast_gate_cases.tsv` and PASS on 2026-02-25）
  - [x] update Acceptance Map row（BlockExpr-related rows and fast-gate case IDs are synchronized）
- cond_prelude support:
  - [x] fast gate pinned (cond_prelude planner_required)
- not operator support:
  - [x] grammar + lowering + fixture pin (`phase29bq_selfhost_local_expr_unary_not_cleanup_min.hako`; probe: `./tools/selfhost/run.sh --gate --planner-required 1 --filter phase29bq_selfhost_local_expr_unary_not_cleanup_min --max-cases 1`, 2026-02-25 PASS)
- E2E tests expansion:
  - [x] add 2–3 selfhost cases
    - evidence probes (2026-02-28):
      - `./tools/selfhost/run.sh --gate --planner-required 1 --filter mirror_sync_tail_cleanup_min --max-cases 1 --timeout-secs 120`
      - `./tools/selfhost/run.sh --gate --planner-required 1 --filter local_fini_multi_lifo_cleanup_min --max-cases 1 --timeout-secs 120`
      - `./tools/selfhost/run.sh --gate --planner-required 1 --filter local_expr_blockexpr_fini_cleanup_min --max-cases 1 --timeout-secs 120`
  - [x] milestone canary run
    - evidence (2026-02-28): `RUN_TIMEOUT_SECS=120 SMOKES_ENABLE_SELFHOST=1 HAKO_JOINIR_PLANNER_REQUIRED=1 bash tools/smokes/v2/profiles/integration/selfhost/phase29bq_selfhost_planner_required_dev_gate_vm.sh` (`198/198`, `total_secs=682`, `avg_case_secs=3.44`, `jobs=4`)

Selfhost canary reported: 198/198 tests PASS (milestone run on 2026-02-28, `total_secs=682`, `avg_case_secs=3.44`).

## Latest updates (log)
- Lane A daily monitor evidence (2026-02-25): `bash ./tools/selfhost/run_lane_a_daily.sh` PASS.
- `.hako mirbuilder` contract pin: minimal multi-Local accept shape (`Local>Local>Print(Var(last_local))>Return(Int)`) is now pinned with fixture `phase29bq_hako_mirbuilder_phase10_multi_local_last_var_min.hako`.
- `.hako mirbuilder` contract pin: unsupported multi-Local variant (print uses non-last local) remains reject-pinned with fixture `phase29bq_hako_mirbuilder_phase10_multi_local_reject_min.hako`.
- BoxCount: ScanWhilePredicate shape added to generic_loop_v1 for _extract_ident selfhost blocker (fixture: `phase29bq_funcscanner_extract_ident_min.hako`).
- BoxCount: DivCountdownBy10 shape added to generic_loop_v1 for int_to_str selfhost blocker (fixture: `phase29bq_div_countdown_by10_min.hako`).
- Recipe-first: Phase C10 LoopTrueEarlyExit recipe-first migration completed (builder, matcher verification, composer, recipe-only gate).
- Recipe-first: Phase C9 LoopContinueOnly recipe-first migration completed (builder, matcher verification, composer, recipe-only gate).
- Recipe-first: Phase C13 LoopArrayJoin recipe-first migration completed (builder with IfV2+Stmt×2, matcher verification, composer, recipe-only gate).
- Recipe-first: Phase C14 ScanWithInit / SplitScan / BoolPredicateScan / AccumConstLoop recipe-first migration completed (builder, matcher verification, composer, recipe-only gate).
- Recipe-first: Phase C15 Scan loop v0 family (loop_scan_methods_v0 / loop_scan_methods_block_v0 / loop_scan_phi_vars_v0 / loop_scan_v0) recipe-first migration completed (segment verification, composer, recipe-only gate).
- Recipe-first: Phase C16 Collection loops (loop_collect_using_entries_v0 / loop_bundle_resolver_v0 / loop_true_break_continue) recipe-first migration completed (segment verification, composer, recipe-only gate).
- Recipe-first: Phase C17 LoopCond* (break_continue / continue_only / continue_with_return / return_in_body) recipe-first migration completed (matcher verification, composer, recipe-only gate).
- Recipe-first: Phase C12 LoopCharMap recipe-first migration completed (builder, matcher verification, composer, recipe-only gate).
- Recipe-first: Phase C11 LoopSimpleWhile recipe-first migration completed.
- Recipe-first: Fixed LoopV0-containing root blocks to use NoExit contract instead of StmtOnly (StmtOnly applies to nested body_block only).
- Fixture pin: nested loop inner if-else fallthrough join else-return local (`apps/tests/phase29bq_selfhost_blocker_parse_program2_nested_loop_if_else_fallthrough_join_else_return_local_min.hako`)
- BoxCount: ElseOnlyReturnIf で else 側 `local` + `return` を受理
- BoxCount: ElseOnlyReturnIf で else 側 `print` + `return`（effect prelude）を allow_extended のときのみ受理
- Fixture pin: nested loop inner if-else fallthrough join else-return blockexpr-var (`apps/tests/phase29bq_selfhost_blocker_parse_program2_nested_loop_if_else_fallthrough_join_else_return_blockexpr_var_min.hako`)

- Fixture pin: nested loop inner if-else fallthrough join else-return blockexpr (`apps/tests/phase29bq_selfhost_blocker_parse_program2_nested_loop_if_else_fallthrough_join_else_return_blockexpr_min.hako`)
- Fixture pin: nested loop inner if-else fallthrough join return blockexpr-var (`apps/tests/phase29bq_selfhost_blocker_parse_program2_nested_loop_if_else_fallthrough_join_return_blockexpr_var_min.hako`)
- Fixture pin: nested loop inner if-else fallthrough join return blockexpr-blockexpr (`apps/tests/phase29bq_selfhost_blocker_parse_program2_nested_loop_if_else_fallthrough_join_return_blockexpr_blockexpr_min.hako`)
- Fixture pin: nested loop inner if-else fallthrough join return local blockexpr (`apps/tests/phase29bq_selfhost_blocker_parse_program2_nested_loop_if_else_fallthrough_join_return_local_blockexpr_min.hako`)
- BoxCount: single local-init is pure に BlockExpr を許可（prelude no-exit + tail value-lowerable）
- Fixture pin: nested loop inner if-else fallthrough join return var blockexpr (`apps/tests/phase29bq_selfhost_blocker_parse_program2_nested_loop_if_else_fallthrough_join_return_var_blockexpr_min.hako`)
- Fixture pin: nested loop inner if-else fallthrough join return blockexpr-local (`apps/tests/phase29bq_selfhost_blocker_parse_program2_nested_loop_if_else_fallthrough_join_return_blockexpr_local_min.hako`)
- BoxCount: ThenOnlyReturnIf で then 側 `local` + `return <value>`（BlockExpr含む）を受理
- Fixture pin: nested loop inner if-else fallthrough join return blockexpr (`apps/tests/phase29bq_selfhost_blocker_parse_program2_nested_loop_if_else_fallthrough_join_return_blockexpr_min.hako`)
- Normalizer: BlockExpr value を planner-required で受理（JSON v0 bridge も対応）
- Fixture pin: nested loop inner if-else fallthrough join return local (`apps/tests/phase29bq_selfhost_blocker_parse_program2_nested_loop_if_else_fallthrough_join_return_local_min.hako`)
- BoxCount: ThenOnlyReturnIf の return-local 受理（then 側 local-init + return を許可）
- Fixture pin: nested loop inner if-else fallthrough join return var (`apps/tests/phase29bq_selfhost_blocker_parse_program2_nested_loop_if_else_fallthrough_join_return_var_min.hako`)
- Fixture pin: nested loop inner if-else fallthrough join (`apps/tests/phase29bq_selfhost_blocker_parse_program2_nested_loop_if_else_fallthrough_join_min.hako`)
- BoxCount: ThenOnlyReturnIf を追加（`if { return } else { fallthrough }` を受理）
- BoxCount: then-only 判定の no-exit を deep 判定に強化、`is_else_nested_exit_if_return_shape` は内側 else 無しも許可
- Fixture pin: nested loop inner if-fallthrough join (`apps/tests/phase29bq_selfhost_blocker_parse_program2_nested_loop_if_fallthrough_join_min.hako`)
- Selfhost canary flake (reproduced twice): `/tmp/phase29bq_selfhost_phase29bq_selfhost_blocker_scan_methods_loop_min.hako.log` (summary: `[diag/selfhost] steptree_root=1010:[phase132/debug] StepTree root for 'StageBBodyExtractorBox.build_body_src/2': ...`, first_freeze_or_reject: `not found`)
- Follow-up: `HAKO_JOINIR_DEBUG=0` の単独実行は rc=0（PASS）。DEBUG=1 は trace 出力が混ざって期待出力と不一致になるため、再現時のみ記録→停止で扱う。
- Blocker capture (planner_required, BoxCount seed): `/tmp/phase29bq_fast_gate_blocker_nested_loop_if_else_fallthrough_join_return_local_1768864049.summary` (first line: `fixture=apps/tests/phase29bq_selfhost_blocker_parse_program2_nested_loop_if_else_fallthrough_join_return_local_min.hako`, first_freeze_or_reject: `[ERROR] ❌ MIR compilation error: [plan/freeze:contract] planner required, but planner returned None (legacy fallback forbidden)`)
- Blocker capture (normalizer): `/tmp/phase29bq_fast_gate_blocker_nested_loop_if_else_fallthrough_join_return_blockexpr_1768866056.summary` (first line: `fixture=apps/tests/phase29bq_selfhost_blocker_parse_program2_nested_loop_if_else_fallthrough_join_return_blockexpr_min.hako`, first_freeze_or_reject: `[normalizer] Unsupported value AST: BlockExpr ...`)
- Blocker capture (planner_required, BoxCount seed): `/tmp/phase29bq_fast_gate_blocker_nested_loop_if_else_fallthrough_join_return_blockexpr_local_1768871507.summary` (first line: `fixture=apps/tests/phase29bq_selfhost_blocker_parse_program2_nested_loop_if_else_fallthrough_join_return_blockexpr_local_min.hako`, first_freeze_or_reject: `[ERROR] ❌ MIR compilation error: [plan/freeze:contract] planner required, but planner returned None (legacy fallback forbidden)`)
- Blocker capture (planner_required, BoxCount seed): `/tmp/phase29bq_fast_gate_blocker_else_return_local_20260121_013114.summary` (first line: `fixture=apps/tests/phase29bq_selfhost_blocker_parse_program2_nested_loop_if_else_fallthrough_join_else_return_local_min.hako`, first_freeze_or_reject: `[ERROR] ❌ MIR compilation error: [plan/freeze:contract] planner required, but planner returned None (legacy fallback forbidden)`)
- Blocker capture (planner_required, BoxCount seed): `/tmp/phase29bq_fast_gate_blocker_nested_loop_if_else_fallthrough_join_return_local_blockexpr_1768910831.summary` (first line: `fixture=apps/tests/phase29bq_selfhost_blocker_parse_program2_nested_loop_if_else_fallthrough_join_return_local_blockexpr_min.hako`, first_freeze_or_reject: `[ERROR] ❌ MIR compilation error: [plan/freeze:contract] planner required, but planner returned None (legacy fallback forbidden)`)
- Fixture pin: nested loop inner if-else-if-else return (`apps/tests/phase29bq_selfhost_blocker_parse_program2_nested_loop_if_else_if_else_return_min.hako`)
- Fixture pin: nested loop inner if-else-if return (`apps/tests/phase29bq_selfhost_blocker_parse_program2_nested_loop_if_else_if_return_min.hako`)
- Fixture pin: nested loop inner if-else return local (`apps/tests/phase29bq_selfhost_blocker_parse_program2_nested_loop_if_else_return_local_min.hako`)
- Fixture pin: nested loop inner if-else return var (`apps/tests/phase29bq_selfhost_blocker_parse_program2_nested_loop_if_else_return_var_min.hako`)
- Fixture pin: nested loop inner if-return local (`apps/tests/phase29bq_selfhost_blocker_parse_program2_nested_loop_if_return_local_min.hako`)
- Fixture pin: nested loop inner if-return var (`apps/tests/phase29bq_selfhost_blocker_parse_program2_nested_loop_if_return_var_min.hako`)
- BoxCount: `loop_scan_phi_vars_v0`（selfhost `PhiInjectorBox._collect_phi_vars/2` outer loop, nested break-loop + found-if）を追加、fixture: `apps/tests/phase29bq_selfhost_blocker_phi_collect_outer_loop_min.hako`
- Blocker capture (planner_required, BoxCount seed): `/tmp/phase29bq_joinir_blocker_loop_if_return_local_1042332.summary` (first line: `fixture=apps/tests/phase29bq_selfhost_blocker_parse_program2_loop_if_return_local_min.hako`)
- Blocker capture (planner_required, BoxCount seed): `/tmp/phase29bq_joinir_blocker_loop_if_else_return_local_1386020.summary` (first line: `fixture=apps/tests/phase29bq_selfhost_blocker_parse_program2_loop_if_else_return_local_min.hako`, first_freeze_or_reject: `47:[ERROR] ❌ MIR compilation error: [plan/freeze:contract] planner required, but planner returned None (legacy fallback forbidden)`)
- Blocker capture (planner_required, BoxCount seed): `/tmp/phase29bq_joinir_blocker_loop_if_else_if_return_1457605.summary` (first line: `fixture=apps/tests/phase29bq_selfhost_blocker_parse_program2_loop_if_else_if_return_min.hako`, first_freeze_or_reject: `47:[ERROR] ❌ MIR compilation error: [plan/freeze:contract] planner required, but planner returned None (legacy fallback forbidden)`)
- Blocker capture (planner_required, BoxCount seed): `/tmp/phase29bq_joinir_blocker_nested_loop_if_else_return_1613831.summary` (first line: `fixture=apps/tests/phase29bq_selfhost_blocker_parse_program2_nested_loop_if_else_return_min.hako`, first_freeze_or_reject: `not found`, note: `MIR compilation error: [normalizer] generic loop v0: nested loop has no plan`)
- Blocker capture (planner_required, BoxCount seed): `/tmp/phase29bq_fast_gate_2101751_bq_list.log` (first line: `fixture=apps/tests/phase29bq_selfhost_blocker_parse_program2_nested_loop_if_else_fallthrough_join_min.hako`, first_freeze_or_reject: `[ERROR] ❌ MIR compilation error: [plan/freeze:contract] planner required, but planner returned None (legacy fallback forbidden)`)
- BoxCount: `loop_cond_break_continue` は carrier=0 の場合に condition expr から補完し、V7(loop_phi_empty) を回避（planner_required）
- BoxCount: `ExitIfTree` (nested if-in-loop with all branches ending in exit)
- BoxCount: nested loop + inner if-else return を受理（inner else return を許可）
  - Fixture: `apps/tests/phase29bq_loop_cond_break_continue_nested_if_break_min.hako`
  - Recipe: `ExitKind`/`ExitLeaf`/`ExitIfTree` variants in `loop_cond_break_continue/recipe.rs`
  - Facts: `build_exit_if_tree_recipe` / `build_exit_only_recipe` recursive builders
  - Lower: `item_lowering.rs` match arms + `else_patterns.rs::lower_exit_if_tree`
  - Contract: All branches must end with exit (break/continue/return). Fallthrough prohibited.
- BoxShape: RecipeTree+Parts 収束（selfhost “復帰作業” は一時停止）
  - SSOT: `docs/development/current/main/design/recipe-tree-and-parts-ssot.md`
  - 現状: M1–M4 完了、現在は M5（Parts への横展開で features 側の “箱固有 lowering” を減らす）
  - Milestone (M18): features 側の if/branch assembly が 0 件になり、SSOT が `parts::dispatch` に収束
  - Milestone (M19): if 語彙を `RecipeItem::IfV2 { contract: IfContractKind, .. }` に統一（旧 `RecipeItem::{If,IfJoin}` は撤去）
  - Milestone (M20/M21): `parts::dispatch` 入口を `lower_block_internal(kind, ...)` に集約、accept_kind は log/contract-only に限定
  - Milestone (M22): `RecipeBlock` の手組みを `recipe_tree/builders.rs` へ集約（Parts/Features は builder 呼び出しのみ）
  - Milestone (M23/M24): `RecipeBodies::new()` の手組みを `recipe_tree/builders.rs` へ集約（plan 配下から `RecipeBodies::new()` が消滅）
  - M5a: return lowering を `parts::exit::lower_return_stmt_with_effects` に統一（commit `92866678c`, `70d3f10e5`）
  - M5b: break/continue(phi_args) を `parts::exit::{build_break_with_phi_args, build_continue_with_phi_args}` に統一（features 直呼び 0件）
- M5c: RecipeTree vocabulary 収束（4件完了）:
  - M5c-1: ExitLeafKind→ExitKind 統一
  - M5c-2: Recipe→RecipeNode adapter を recipe 層へ移動
  - M5c-3: item_lowering 依存を parts/if_.rs から除去
  - M5c-4: HAKO_EXIT_TREE_DEBUG 撤去（環境変数スパロー解消）
- M5d: 依存方向の収束（parts SSOT / features は委譲のみ）:
  - `parts::{stmt,exit}` が SSOT、`features::exit_branch` は委譲のみ（parts→features 依存を解消）
- Syntax staging: Phase B1 `%{...}` map literal (compat) completed; roadmap SSOT: `docs/development/current/main/design/map-literal-eviction-and-blockexpr-roadmap-ssot.md`.
- Syntax staging: Phase B2 `{...}` BlockExpr completed (incl. recursive exit prohibition hardening, B2-6); selfhost restart condition is documented in `docs/development/current/main/10-Now.md` (selfhost “復帰作業” is still paused).
- Syntax staging: Phase B4 condition-prelude in planner-required completed (pinned fixture: `apps/tests/phase29bq_cond_prelude_planner_required_min.hako`, vocab SSOT: `src/mir/builder/control_flow/plan/policies/cond_prelude_vocab.rs`).
- BoxShape roadmap (final form): `docs/development/current/main/design/recipe-tree-and-parts-ssot.md`
- Condition entry SSOT: `CondBlockView` + `lower_cond` introduced for if/if-in-loop paths (view-only, no rewrite).
- Fast gate fixtures added: `cond_truthiness_value`, `cond_truthiness_null`.
- LoopCondBreak: nested loop depth=1 is now accepted (pinned by `phase29bq_fast_gate_cases.tsv`).
- LoopCondBreak cluster4: nested loop count=4 を受理する箱を追加（fast gate に idx28 fixture を固定）。
- LoopCondBreak cluster5: nested loop count=5 を受理する箱を追加（idx35 fixture を pin）。
- Loop header condition (`cond_loop`) enters via `CondBlockView`.
  - ✅ Phase 2b-1: `LoopCondContinueOnly` / `LoopCondContinueWithReturn` now use `cond_lowering::lower_loop_header_cond()` (short-circuit, RHS can be skipped).
  - ✅ Phase 2b-2: `generic_loop_v0` header short-circuit (commit `f39e4233d`, fixture: `phase29bq_shortcircuit_generic_loop_v0_min`).
  - ✅ Phase 2b-3: `loop_cond_break_continue` (commit `9ab75d846`) / `loop_cond_return_in_body` (commit `ca3f8bc92`) header short-circuit.
  - **SSOT (loop header entry)**: `lower_loop_header_cond()` is the sole entry point for loop header conditions in all `*_pipeline.rs`. Direct `lower_cond_value()` for loop headers is prohibited.
  - Verification: `rg "lower_cond_value\(" src/mir/builder/control_flow/plan/features/*pipeline.rs` should return only body if-condition usages (not loop header).
  - Note: `lower_cond_value()` is still valid for body if-conditions (e.g., `loop_cond_return_in_body_pipeline.rs:387`).
- M6: CorePlan shrink criteria defined (SSOT: recipe-tree-and-parts-ssot.md)
- M25–M30: compiler-shape tightening (BoxShape)
  - M25: ExitKind depth != 1 is fail-fast (`[freeze:contract][exit_depth]`) across verifier/exit/conditional_update.
  - M27/M28: no-exit if lowering in `generic_loop_body` / `loop_true_break_continue` converged to `RecipeBlock -> parts::dispatch` (planner_required only).
  - M30: next milestone for CorePlan shrink/rename recorded (SSOT: `docs/development/current/main/design/coreplan-migration-roadmap-ssot.md`).

## Selfhost canary (current)

- Status: PASS（subsetに現ブロッカーなし / ただし復帰作業は一時停止）
- Entry: `SMOKES_ENABLE_SELFHOST=1 ./tools/smokes/v2/profiles/integration/selfhost/phase29bq_selfhost_planner_required_dev_gate_vm.sh`
- Stability: `scan_methods_loop_min` が debug-fuel 上限で落ちるため、canary の `--json-file` 実行は `--debug-fuel unlimited` を固定（timeout ガード維持）。
- LOG: `/tmp/phase29bq_selfhost_*.log`

## Selfhost blockers (recently resolved)

- ✅ Stage‑B `Program.defs` 注入が空になる問題（FuncScannerBox の ident 抽出を MapBox ステートで安定化）
  - Result: `RewriteKnownMini` の static 呼び出しが JSON v0 bridge で復帰（selfhost canary PASS）
- ✅ `RewriteKnown.try_apply/1` loop(true) with tail return inside loop body
  - Fixture: `apps/tests/phase29bq_selfhost_blocker_rewriteknown_try_apply_loop_true_tail_return_min.hako`
  - Fast gate: `tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_cases.tsv` case_id=`selfhost_rewriteknown_try_apply_loop_true_tail_return_min`
  - Note: per-continue carrier merge uses `ContinueWithPhiArgs` + `step_bb` join PHIs (no single "next_val" assumption).
- ✅ `RewriteKnown.try_apply/1` loop(true) with else-side exit-if
  - Fixture: `apps/tests/phase29bq_selfhost_blocker_rewriteknown_try_apply_loop_true_else_exit_min.hako`
  - Fast gate: `tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_cases.tsv` case_id=`selfhost_rewriteknown_try_apply_loop_true_else_exit_min`
  - Note: general-if then-body + else-side exit-if handled via `GeneralIfElseExit` recipe.
- ✅ `RewriteKnown._trim/1` loop(cond) with `And(..., MethodCall)` in condition
  - Fixture: `apps/tests/phase29bq_selfhost_blocker_rewriteknown_trim_loop_cond_and_methodcall_min.hako`
  - Fast gate: `tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_cases.tsv` case_id=`selfhost_rewriteknown_trim_loop_cond_and_methodcall_min`
  - Note: generic_loop_v0/v1 now allow methodcall terms inside `And/Or` condition chains (planner_required only).
- ✅ `RewriteKnown._itoa/1` loop(cond) with complex step + value-if
  - Fixture: `apps/tests/phase29bq_selfhost_blocker_rewriteknown_itoa_complex_step_min.hako`
  - Fast gate: `tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_cases.tsv` case_id=`selfhost_rewriteknown_itoa_complex_step_min`
  - Note: generic_loop step lowering now allows body-local refs in planner_required (cond pre-body, step post-body).
- ✅ `PhiInjectorBox._collect_phi_vars/2` nested `k` loop (local init patterns)
  - Note: generic_loop の value/local allowlist を selfhost 実態に合わせて拡張し、`no_valid_loop_var_candidates` の freeze を解消。
- ✅ `BreakFinderBox._parse_int/1` else-only return in loop body
  - Fixture: `apps/tests/phase29bq_selfhost_blocker_breakfinder_parse_int_min.hako`
  - Fast gate: `tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_cases.tsv` case_id=`selfhost_breakfinder_parse_int_min`
  - Design: avoid `IfAny` drift by adding a box-local recipe vocab (`ElseOnlyReturnIf`) in `loop_cond_break_continue`
- ✅ `BuildBox.emit_program_json_v0` loop(cond) with then-only break + else update
  - Fixture: `apps/tests/phase29bq_loop_cond_then_only_break_assign_min.hako`
  - Fast gate: `tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_cases.tsv` case_id=`loop_cond_then_only_break_assign_min`
  - Design: add box-local recipe vocab (`ThenOnlyBreakIf`) in `loop_cond_break_continue`

## P2 blocker (archived)

- (stale) `JsonFragBox._seek_array_end/2` の旧ログはアーカイブ扱い（詳細は履歴参照）。

### Temporary BoxShape detour (debugging infrastructure)

BoxCount を続ける前に、複数AIでの枝読みミスを止めるために `plan/trace`（candidates→finalize→try_take_planner）を "小さく完成" させる。
SSOT: `docs/development/current/main/design/ai-handoff-and-debug-contract.md`

### P2 blocker (resolved): phase107/balanced_depth_scan

- **Original Error**: `[phase107/balanced_depth_scan/contract/missing_depth_update] missing 'depth = depth + 1' in branch`
- **Decision**:
  - Analysis-only view を導入し、`balanced_depth_scan` の「深さ更新検出」を SSOT 化（AST rewrite 禁止、`+` は可換、`-` は非可換、BlockExpr は保守観測）。
  - 受理は strict/dev のみ（`NYASH_JOINIR_DEV=1` or `HAKO_JOINIR_STRICT=1`）。`HAKO_JOINIR_PLANNER_REQUIRED=1` には依存しない（release default unchanged）。
- **Acceptance**:
  - `./tools/smokes/v2/profiles/integration/apps/phase107_find_balanced_array_end_vm.sh` PASS
  - `./tools/smokes/v2/profiles/integration/apps/phase107_find_balanced_object_end_vm.sh` PASS
- **Fixtures**:
  - `apps/tests/phase107_find_balanced_array_end_min.hako`
  - `apps/tests/phase107_find_balanced_object_end_min.hako`

### P2 blocker (resolved): UsingResolveSSOTBox.resolve/2 (While cap)

- **Function**: `UsingResolveSSOTBox.resolve/2`
- **Error**: `[joinir/control_tree/cap_missing/While] While detected`
- **Decision**: Rust: While cap を allowlist に追加
- **Fixture**: `apps/tests/phase29bq_selfhost_blocker_while_cap_min.hako`

### P2 blocker (resolved): JsonFragBox._decode_escapes/1 (group-if with nested loop)

- **Function**: `JsonFragBox._decode_escapes/1`
- **Error**: `group-if fallthrough mutation is out-of-scope`
- **Decision**: `group_if_fallthrough_is_allowed` に Loop/While を追加し、pipeline に `lower_nested_loop_depth1_any` 呼び出しを追加
- **Fixture**: `apps/tests/phase29bq_selfhost_blocker_decode_escapes_group_if_fallthrough_mutation_min.hako`
- **Contract**: group-if 内の nested loop は、全パスが continue で終わる場合のみ許可（join 不要パターン）

### P2 blocker (resolved): JsonFragBox._decode_escapes/1 (nested loop in if)

- **Function**: `JsonFragBox._decode_escapes/1`
- **Error**: `unsupported_stmt idx=12 kind=If`（If body 内に nested loop があるため）
- **Decision**: `ContinueIfNestedLoop` レシピで解決済み（fixture: `decode_escapes_if_idx12_min`）

## TODO (ordered cleanup)

1. [x] Loop(cond) multi-continue + multi-delta update を strict/dev + planner_required 限定で受理（no rewrite / per-edge carrier merge を `step_bb` join PHI で実装）
2. [x] ContinueTarget slot を CoreLoop の土台として実装（既定は `step_bb`、strict/dev-only で切替。SSOT: `docs/development/current/main/design/coreloop-continue-target-slot-ssot.md`）
3. [x] ExitBranch feature（moved）: If/BranchN/Loop 内の “exit 付きブランチ” を共通化（prelude + ExitKind 抽出/正規化）して重複を削減（SSOT: `docs/development/current/main/design/exit-branch-feature-ssot.md`）
4. [x] helper boundary SSOT に従い、join/exit/phi/carrier を helper へ集約して ops 直書きを禁止（SSOT: `docs/development/current/main/design/feature-helper-boundary-ssot.md`）
5. [x] helper boundary SSOT の planned 分解（loop_carriers / edgecfg_stubs / carriers / canon-lower boundary）を順に実施（挙動不変の移設で gate green 維持）
6. [x] Condition entry SSOT（if + loop header）: `CondBlockView`（view-only, no rewrite）+ `lower_cond` 入口集約で “条件＝値” 契約を固定。
   - ✅ Phase 2a: `cond_lowering.rs` に `lower_cond_branch()` / `lower_cond_value()` を導入し、呼び出し側を入口へ集約。
   - ✅ Phase 2b-1: `LoopCondContinueOnly` / `LoopCondContinueWithReturn` の loop header を `lower_loop_header_cond()` に移行。
   - ✅ Phase 2b-2: `generic_loop_v0` loop header を `lower_loop_header_cond()` に移行。
   - ✅ Phase 2b-3: `loop_cond_break_continue` / `loop_cond_return_in_body` loop header を `lower_loop_header_cond()` に移行。
7. [x] StepMode: `InlineInBody` の “一般化” と verifier 契約の強化（S1+S2+S3 完了。LoopSimpleWhile source + runtime fixture pin を固定）
8. [x] CleanupWrap + cleanup region boundary（境界）を SSOT 化して、nested exit で意味論が混線しない土台を先に固める（SSOT: `docs/development/current/main/design/cleanupwrap-cleanup-region-boundary-ssot.md`）
9. [x] pattern2_break の subset を箱化して SSOT を一箇所に集約する（`src/mir/builder/control_flow/plan/facts/pattern2_break_facts/` 配下を README+登録表で統一）
10. [x] Phase 29bq の軽量 gate は list 駆動（`tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_cases.tsv`）
11. [x] `LoopBreakPlan.step_placement` を `Pattern2StepPlacement` enum に置換して Facts→Planner→Normalizer の分岐を明文化する（legacy file: `pattern2_break.rs`, dev-only subset は維持）

---

## Deferred / Known Unsupported

### LoopCondContinueWithReturnNested3（v2箱）

- **パターン**: 深さ3のネストしたreturn in else chain
- **fixture**: `phase29bq_selfhost_blocker_return_continue_hetero_nested_return_depth3_min.hako`
- **現状**: 未実装（v2箱の作成が必要）
- **手off**: v1（`LoopCondContinueWithReturn`）が完成したら、別Planでv2箱を実装
- **理由**: 深いネストreturnは「支配/mergeの境界」が変わって、v1の契約（浅いhetero-return-if）と別物。v1に「depth3だけ」足すと、pipelineが分岐だらけになり蜜結合の芽が出る。v2に分ければ1形固定→fixture+fast gateで契約を閉じられる（BoxCountの流儀）
