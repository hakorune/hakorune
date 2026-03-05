---
Status: SSOT
Scope: Task ordering map (compiler cleanliness / recipe-first)
Related:
- docs/development/current/main/10-Now.md
- docs/development/current/main/30-Backlog.md
- docs/development/current/main/design/compiler-pipeline-ssot.md
- docs/development/current/main/design/condprofile-migration-plan-ssot.md
- docs/development/current/main/design/valueflow-blockparams-ssot.md
- docs/development/current/main/design/copy-emission-ssot.md
- docs/development/current/main/design/recipe-tree-and-parts-ssot.md
- docs/development/current/main/design/recipe-first-entry-contract-ssot.md
---

# Compiler Task Map (SSOT)

This is the ordered task map for compiler cleanliness. Use this order unless a new blocker appears.

## Execution Order (SSOT)

“コンパイラーを綺麗に作る” を最優先にするための、迷走しにくい実行順序を固定する。
（原則: 観測SSOT → 境界SSOT → 受理SSOT → ガード/タグ → 残骸掃除）

1) Observation SSOT stabilization (no rewrite, behavior-preserving)
   - Condition/loop_var/step の判断源を 1 箇所へ固定する（SSOT: `condition-observation-ssot.md`）。
   - 重複パース/再導出/rewrite を禁止し、観測ログ契約を揺らさない。

2) Verified-only boundary tightening (Parts/Lower entry)
   - Lower は `VerifiedRecipeBlock` のみ受け取る（SSOT: `recipe-tree-and-parts-ssot.md`）。
   - verify 入口を `parts::entry` に寄せ、例外経路の “直lower/再判定” を潰す。
   - VerifiedRecipe が “配線できること” を契約（PortSig）として保証し、silent wrong を fail-fast に変換する（SSOT: `verified-recipe-port-sig-ssot.md`）。

3) generic_loop_v1 acceptance closure (stop ShapeId explosion)
   - 受理の真実を “ShapeId 列挙” から外し、Recipe/Verifier で決める。
   - ShapeId は coverage/診断の hint-only（SSOT: `generic-loop-v1-acceptance-by-recipe-ssot.md`）。

4) strict/dev guards + exception routes alignment
   - strict nested loop guard / shadow adopt 等が “受理の真実” を持たないように SSOT 整合させる。
   - taxonomy は `planfrag-freeze-taxonomy.md` に揃える（silent fallback 禁止）。

5) Gate/tag pinning
   - 変更ごとに freeze/reject タグを taxonomy に合わせて固定し、揺れを局所で止める。

6) Residue cleanup (DomainPlan / legacy extractors)
   - DomainPlan/Normalizer 残骸・未参照抽出器などを “掃除” として最後にまとめて削る。
   - ここで新しい受理形を増やさない（BoxCount を混ぜない）。
   - `joinir/patterns/` は最終的に「薄い入口/ルーティング層」に縮退させる（巨大 router / pattern*_minimal などの残骸を整理）。
     - ルーティングの SSOT は Facts/Recipe/Verifier に寄せ、patterns 層は “どの入口を呼ぶか” だけにする。
     - ただし release 挙動は変えない（構造整理で fail-fast 位置を動かす場合は strict/dev(+planner_required) 限定）。

## Phase29x Direct Route Recovery Pack (2026-03-03)

Baseline probe (fixed):
- command: `tools/dev/direct_loop_progression_sweep.sh --profile phase29x-probe`
- result: `emit_fail=13`, `run_nonzero=9`, `run_ok=96`, `route_blocker=0`（total=118）
- class: `emit:direct-verify=9`, `emit:other=4`, `run:vm-error=3`

Fixed order (this round):
1) `emit:direct-verify` 9件（box_member 7 + `scan_methods_nested_loop_idx19/28`）を dominance 契約で切り分ける
2) single-exit 系 fail-fast（`loop_cond_break_continue`）2件を Recipe 契約で整理する
3) 単発 `unsupported stmt Call`（1）を isolate して fixture+gate で固定する
4) 単発 `Expected BinOp`（1）を isolate して fixture+gate で固定する

Round guardrails (AI mistake resistant):
- 1 blocker = 1受理形 = fixture+gate = 1 commit（BoxCount と BoxShape を混ぜない）
- `planner_required` で string-match fallback を増やさない（受理判断は Recipe/Verifier SSOT 側へ寄せる）
- `variable_map` の一時退避復元は `parts::var_map_scope::with_saved_variable_map` を使い、手動 restore を増やさない
- 受理形を増やしたコミットでは StepTree/Extractor/Parity 観測SSOTを同コミット更新する
- 各ステップ完了時に probe を再実行し、`emit_fail` と class 差分を `CURRENT_TASK.md` に反映する

## Priority Order (current)

NOTE:
- `CURRENT_TASK.md` の blocker が **none** の場合、`0)` はスキップして `1)` から進める。
- blocker が復帰した場合のみ、`0)` を “最優先のBoxShape” として扱う（BoxCountと混ぜない）。

0) Selfhost Stage-B SSA dominance stabilization (BoxShape; no acceptance expansion)
   - 現象: `[freeze:contract][mir/verify:dominator_violation]` が selfhost Stage-B で発生（CURRENT_TASK.md に log snapshot を固定）。
   - ねらい: VM 実行まで行かずに “原因側” の箱で fail-fast し、責務混線（PHI/merge/local_ssa）を縮退させる。
   - SSOT pins (docs-first):
     - ValueFlow SSOT: `valueflow-blockparams-ssot.md`（merge の意味論は BlockParams/edge_args に固定）
     - Copy emission SSOT: `copy-emission-ssot.md`（Copy は materialize/alias に限定、救済禁止）
   - Planned (docs-first → 1-file migrations):
     - CopyEmitter の “挿入点API” を揃える（`in_block` / `before_terminator` / `after_phis`）→ json_v0_bridge や pass 側の直Copyを段階移行できるようにする。
     - `CopyReason` を enum としてSSOT化し、reasonのドリフト（typo）をコンパイルで防ぐ（文字列理由の縮退）。
     - `tools/checks/no_direct_copy_emission.sh` が PASS する状態まで、残存直Copyを 1ファイル=1コミットで潰す（allowlist拡張は最小・理由SSOT必須）。
   - Top candidate (1-commit BoxShape):
     - `BlockScheduleBox::{ensure_after_phis_copy, emit_before_call_copy}` の `Copy(dst, src)` を “non-dominating救済” に使わない。
       - strict/dev: non-dominating `src` は fail-fast（または pure-def のみ再物質化に限定）。
       - schedule は “物理配置” のみ、SSA修復は PHI/merge/LocalSSA の責務に寄せる。
     - `local_ssa::ensure()` は pure-def 再物質化を許可（Const/BinOp/Compare/Copy）。それ以外は strict/dev で fail-fast を検討。
   - Decision (docs-first): JoinIR の “PHIだけ部分介入” を禁止し、PHI生成責務を単一路線へ寄せる（詳細は cleanliness campaign SSOT に pin）。

1) CondProfile stabilization (observation-first, no rewrite)
   - D7 is deferred until `[condprofile:step_mismatch]` is sufficiently low.
   - D8+ (ConditionShape shrink) is design-only; keep logs/SSOT stable first.

2) VerifiedRecipe boundary tightening (exception route cleanup)
   - JoinIR recipe-first exception routes (shadow adopt / pre-plan / direct lower) must not bypass verifier.
   - Ensure `VerifiedRecipeBlock` is the only entry to Parts/Lower.

3) GenericLoopV1 acceptance closure (stop ShapeId explosion)
   - Prereq: Recipe-first payloadization (Facts builds recursive RecipeBlock for body).
   - Switch planner_required acceptance from “ShapeId required” to “VerifiedRecipe required”.
   - ShapeId becomes hint-only; nesting must be accepted by Recipe composability.
   - SSOT: `docs/development/current/main/design/generic-loop-v1-acceptance-by-recipe-ssot.md`

4) Gate/tag pinning for LoopCond recipe-only (stabilize expectations)
   - Keep planner_first / flowbox/adopt / freeze tags consistent with recipe-only paths.
   - Update TSV expectations only for the changed acceptance path (no broad re-baselining).

5) DomainPlan residue cleanup (last, behavior-preserving)
   - Remove remaining DomainPlan variants only after acceptance + tags are stable.
   - Do not mix BoxCount with this sweep.

6) Verifier idx_var contract enforcement
   - Status: ✅ done (C20-D5). CondProfile を SSOT として idx_var を固定し、drift は strict/dev で fail-fast する。
   - Pointer: `docs/development/current/main/10-Now.md` (C20-D5) / `docs/development/current/main/design/condprofile-migration-plan-ssot.md`

7) Cleanliness Wave 3 (inventory -> SSOT fix; docs-first)
   - Legacy/fallback / routing / entry order / observation sources / boundary rules を SSOT で固定し、
     実装タスクを “1コミットで閉じる粒度” に分解する（キャンペーンSSOT参照）。

8) Selfhost/OOM mitigation (parallel, lower priority)
   - Stage-B JSON streaming / JSONL
   - module splitting / minimal incremental

## When Green (SSOT)

`CURRENT_TASK.md` の blocker が **none** のとき “進まない” を防ぐため、selfhost は次のループで進める（詳細は phase checklist）。

- PROBE（no-commit）: selfhost subset に “次の1ユニットだけ” をローカル追加して canary を回し、次ブロッカーを掘る
- FIX（1ブロッカー=1コミット）: FAIL が出たら fixture+gate で pin して原因側を潰す
- PROMOTE（subset-only commit）: PROBE が PASS なら subset 追加だけを 1コミットで反映（コード変更禁止）

## Workstreams (by area)

### A. CondProfile
- D7: StepShape shrink (deferred)
- D8: ConditionShape shrink to CFG skeleton only
- D9: Verifier acceptance pivot to CondProfile

### B. Pattern / Recipe-first
- DomainPlan label layer removal
- Normalizer cleanup (recipe-only path)
- RecipeComposer consolidation

### H. RecipeComposer “脱・Normalizer”

目的: `recipe_tree::composer` が `PlanNormalizer::normalize_*` に依存せず、
RecipeBlock/IfV2/LoopV0 を **直接組み立てる** 形へ収束する（Recipe-first の最終形）。

順序（1ブロッカー=1コミット）:
1) ✅ LoopBreakRecipe: `normalize_pattern2_break` 依存を外して RecipeBlock 構築へ置換
2) ✅ IfPhiJoin: IfV2 + join_payload 直構築へ置換
3) ✅ LoopContinueOnly: LoopV0 + Continue port 直構築へ置換
4) ✅ LoopCond* / LoopTrue*: normalize 依存を段階的に置換
5) ✅ GenericLoopV0/ V1: recipe_path を完全化（Normalizer 経由を撤去）

### C. VerifiedRecipe / Parts
- parts::entry single gate enforcement
- verifier always-on scope decision
- join/phi contract mechanical checks

### E. patterns/（routing layer）cleanup

目標: `joinir/patterns/` を “薄いディスパッチャ” に縮退させ、意味論/契約は Recipe/Verifier/Parts に集約する。

1) Router registryization（挙動不変）
   - `router.rs` の “候補列挙 + compose 呼び出し” をテーブル化し、見通しを改善する（分岐爆発の抑止）。
2) Pattern implementation relocation（挙動不変）
   - `pattern*_minimal.rs` / `pattern3_with_if_phi.rs` 等の “直接MIR構築” を plan/recipe-first（Composer/Normalizer/Parts）側へ移す。
   - patterns 層は “入口選択と呼び出し” のみ（Verify/Lower の真実は持たない）。
   - Phase‑2/Step1: `pattern1_minimal.rs` の実装本体を plan 側へ移設。
   - Phase‑2/Step2: `pattern3_with_if_phi.rs` の実装本体を plan 側へ移設。
   - Phase‑2/Step3: `pattern8_scan_bool_predicate.rs` の実装本体を plan 側へ移設。
   - Phase‑2/Step4: `route_prep_pipeline.rs`（旧: `pattern_pipeline.rs`）を plan 側へ移設。
   - Phase‑2/Step5: `pattern2_lowering_orchestrator.rs` を plan 側へ移設。
   - Phase‑2/Step6: `pattern2_inputs_facts_box.rs` を plan 側へ移設。
   - Phase‑2/Step7: `pattern2_policy_router.rs` / `pattern2_break_condition_policy_router.rs` を plan 側へ移設。
   - Phase‑2/Step8: `pattern2_steps/` を plan 側へ移設。
   - Phase‑2/Step9: `conversion_pipeline.rs` を plan 側へ移設。
  - Phase‑2/Step10: trim 系（`trim_loop_lowering.rs` / `trim_pattern_lowerer.rs` / `trim_pattern_validator.rs`）を plan 側へ移設。
  - Phase‑2/Step11: loop_true_counter_extractor を plan 側へ移設。
  - Phase‑2/Step12: loop_scope_shape_builder を plan 側へ移設。
  - Phase‑2/Step13: exit_binding 系（orchestrator/validator/constructor/applicator）を plan 側へ移設。
  - Phase‑2/Step14: condition_env_builder を plan 側へ移設。
  - Phase‑2/Step15: ast_feature_extractor / escape_pattern_recognizer を plan 側へ移設。
  - Phase‑2/Step16: pattern_recognizers/ を plan 側へ移設。
  - Phase‑2/Step17: policies/ を plan 側へ移設。
  - Phase‑2/Step18: common_init + common/ を plan 側へ移設。
  - Phase‑2/Step19: read_digits_break_condition_box を plan 側へ移設。
  - Phase‑2/Step20: body_local_policy を plan 側へ移設。
  - Phase‑2/Step21: expectations を plan 側へ移設。
  - Phase‑2/Step22: pattern1/pattern3 extractors を plan 側へ移設。
  - Phase‑2/Step23: legacy は patterns で保持（入口互換のみ）。
  - Phase‑2/Step24: pattern2 module を plan 側へ移設。
  - Phase‑2/Step25: wrapper 実態監査と残存実体の移設完了。
  - Phase‑2/Step26: patterns wrapper を mod.rs に集約。
3) Label rename（意味名へ置換）
   - `PatternN` / `LoopCond*` のログ/TSV ラベルを意味名へ置換（入口の重なり解消後に一括）。
   - 置換は docs/SSOT/TSV/ログを同コミットで揃える（観測が揺れないようにする）。

### F. planner_first 表示名 rollout（Phase split）

- Phase A（完了）
  - display name map を SSOT 化し、`planner_first` ログに `label=` を付与（rule は変更しない）。
  - SSOT: `docs/development/current/main/design/entry-name-map-ssot.md`
- Phase B（未着手）
  - TSV/SSOT/Guard Matrix に display name（label）を併記。
  - `rule=` は不変のまま、可読性だけを上げる。

### G. Loop vocabulary convergence（Loopを“1語彙”へ収束）

最終形の目標: `loop(cond){...}` は **Loop 1語彙**へ収束し、`LoopCond*` / `Pattern*` は “入口ラベル/診断” に縮退する。

1) SSOTの明文化（docs-only）
   - RecipeTreeの最小語彙（`Seq/Stmt/If/Loop/Exit`）と、Verifier/Parts/Entry の責務境界を1枚で参照できるようにする。
   - pointer: `lego-composability-policy.md` / `verified-recipe-port-sig-ssot.md` / `recipe-first-entry-contract-ssot.md`
2) “分割しない” を既定にする（BoxShape）
   - `LoopExitIfBreakContinue` を Single/Multi に箱分割する前に、`break_kind`（Single/Multi）を facts/trace に落として観測固定する。
   - 「lowerが別経路になる」ことが証明できた場合のみ BoxCount として箱分割する（1形=1fixture=1commit）。
3) specialized entry の縮退（BoxShape→BoxCountの順）
   - `LoopCond*` は canonical `Loop`（generic_loop_v1）へ写像する方針を SSOT 化し、重なりは coherence/freezes で先に潰す。
4) fixture pin（BoxCount）
   - canonical Loop で `loop→if→loop` 等の合成ケースを 1件ずつ pin（実測タグは label 併記で可読化、ruleは安定ID）。

### D. Selfhost / OOM
- Stage-B JSON to_writer / JSONL
- module-based compilation
- minimal incremental cache

## Ordering Rule (SSOT)

- Do not mix BoxCount and BoxShape in the same change.
- If a blocker appears, select BoxCount vs BoxShape explicitly and update `10-Now.md`.
