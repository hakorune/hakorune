---
Status: SSOT
Scope: Phase 29bq 以降の「コンパイラーきれいきれい大作戦」運用（selfhost を目的化しない）
Related:
- docs/development/current/main/design/compiler-expressivity-first-policy.md
- docs/development/current/main/design/coreplan-skeleton-feature-model.md
- docs/development/current/main/design/compiler-pipeline-ssot.md
- docs/development/current/main/design/type-system-policy-ssot.md
- docs/development/current/main/design/lego-composability-policy.md
- docs/development/current/main/design/joinir-design-map.md
- docs/development/current/main/design/joinir-planner-required-gates-ssot.md
- src/mir/builder/control_flow/plan/REGISTRY.md
---

# Compiler cleanliness campaign (SSOT)

目的: selfhost を “通す” ではなく、**compiler 側の表現力と構造**を「石橋を叩いて渡る」流儀で収束させる。

## 原則

- **compiler-first**: CorePlan/Facts/Normalizer の設計収束が最優先。selfhost canary は “ブロッカー抽出の入口”。
- **no workaround**: JoinIR strict/planner_required の回避を `.hako` でやらない（方針SSOT参照）。
- **no rewrite**: AST の見た目等価変形は禁止。観測は analysis-only view（`CondBlockView`, `CondCanon`, `UpdateCanon`）。
- **BoxShape → BoxCount**: まず共通部品SSOT化/入口集約/テーブル化。箱の追加は最後の手段。
- **完成品キット禁止（原則）**: skeleton/PHI/walker を複製して増やす箱は作らない（例外は SSOT に撤去計画がある場合のみ）。

## AI mistake-resistant rules (always-on)

- 受理判断の真実を 1箇所に寄せる（Recipe/Verifier SSOT）。`planner_required` 下で文字列判定 fallback を足さない。
- `variable_map` の退避復元は scope helper を必須化する（`parts::var_map_scope::with_saved_variable_map`）。手動 save/restore は禁止。
- “受理を増やしたのに観測SSOTを更新しない” 変更を禁止する（StepTree/Extractor/parity を同コミット更新）。
- 失敗の主語は `freeze:contract` で原因側へ寄せる。panic/silent no-op で距離を伸ばさない。
- 実行順序は `compiler-task-map-ssot.md` の round pack を優先し、`CURRENT_TASK.md` の blocker 順序と同期する。

## このキャンペーンの“追加点”の置き方

“どこに何を足すか” が分かりにくいと AI が誤るので、追加点を SSOT で固定する。

1. **入口（観測）**
   - Facts は “観測のみ” に寄せ、分岐の入口は view/canon を SSOT にする。
2. **骨格（Skeleton）**
   - Skeleton は一意に決める。決まらなければ Freeze（silent fallback をしない）。
3. **Feature slot**
   - 直交差分は slot に押し込む（ExitBranch/ExitMap/ValueJoin/ContinueEdges/StepMode/CleanupWrap）。
4. **pipeline steps**
   - “重複ロジック” は helper ではなく、固定順の工程（step）として共通化する。

## “直交軸”の扱い（非SSOT）

“6直交軸” は **分類レンズ（タグ）**として使うが、SSOTにしない。
SSOTの主語は `(Skeleton, FeatureSet)` のみ（詳細は Skeleton/Feature SSOT）。

## 受け入れ基準（キャンペーン用）

- 新しい機能追加ではなく、**構造の収束**（責務分離/入口集約/SSOT化）で差分を閉じる。
- 既存の gate は緑維持（少なくとも `phase29bq_fast_gate_vm.sh` と `phase29bp_planner_required_dev_gate_v4_vm.sh`）。
- エラーは “どこで落ちるか” が 1 行で辿れる（`[plan/reject]` / legacy route label 表示など）。

## Next（小粒度）

次の BoxShape 候補（順不同、どれも "完成品キット増殖" を抑制するための土台）:

- reject_reason → handoff のテーブルSSOT化（順序依存の偶然通過を減らす）
- Selfhost Stage-B SSA dominance stabilization（fail-fast を “原因側” へ寄せる）
  - 現象: `[freeze:contract][mir/verify:dominator_violation]` が selfhost Stage-B で発生（詳細ログは `CURRENT_TASK.md` に固定）。
  - 方針: 受理形は増やさず（BoxCount禁止）、責務混線（PHI/merge/local_ssa）を BoxShape で収束させる。
  - 最小タスク（1コミット）候補:
    - `BlockScheduleBox::{ensure_after_phis_copy, emit_before_call_copy}` が non-dominating `src` を `Copy(dst, src)` で運ぶのを禁止（strict/dev のみ fail-fast、または pure-def のみ再物質化）。
    - `local_ssa::ensure()` は pure-def（Const/BinOp/Compare/Copy）の再物質化を許可し、cross-block Copy を減らす。
  - Decision（混線禁止）:
    - PHI生成の責務は 1 箱に集約する（variable_map を確定する “最終PHI” を複数経路で emit しない）。
    - JoinIR の “PHIだけ生成” の部分介入は禁止（やるなら dry-run/観測専用、または end-to-end に閉じる）。
    - PHI input の pred ラベルは “merge ブロックの実 predecessor” 以外を禁止（入口ブロックを pred にしない）。
    - schedule 層（BlockScheduleBox）は “物理配置” のみ（SSA修復の真実を持たない）。non-dominating救済を Copy でやらない。
  - Ownership（PHIの「運び方」SSOT）:
    - JoinIR: PhiSpec/JoinPayload/snapshots を返す（計画だけ; PHI命令は emit しない）。
    - SSA/PHI Lower: CFG の実 predecessor を見て、PHI（または edge-copy）を生成して variable_map を確定する。
    - Verifier: 「PHI入力は merge の実 predecessor に一致」「def dominates use」などを fail-fast で保証する。

## Cleanliness Wave 3（inventory → SSOT fix）

目的: “深層負債” を **大改造にせず**、まず SSOT で境界と撤去条件を固定し、1タスク=1コミットで削れる形に分解する。

ルール:
- まず design-only（docs/README/SSOT）で “残す理由 / 消す条件 / 禁止事項” を固定する。
- 実装に入るのは **SSOT が書けたものから**（順番に）。release 挙動は変えない（strict/dev(+planner_required) なら可）。
- タスク粒度は 1コミット=1ブロッカー（混ぜない）。

Wave 3 の小タスク（ordered; docs-first）

1) Normalization fallback（shadow）の SSOT 明記（理由/撤去条件）
   - 対象: `src/mir/builder/control_flow/normalization/plan_box.rs`
   - 追記先: 本書 + `src/mir/builder/control_flow/normalization/README.md`
2) JoinIR legacy routing の “入口/退避” を SSOT で線引き
   - 対象: `src/mir/builder/control_flow/joinir/legacy/README.md`
   - 追記先: JoinIR map SSOT（`joinir-design-map.md`）か、legacy README を “退避専用” に明記
3) compatibility-lane normalizer residue と recipe composer の入口順序を SSOT 固定
   - 対象例: `src/mir/builder/control_flow/joinir/patterns/router.rs`, `src/mir/builder/control_flow/plan/recipe_tree/*_composer.rs`
   - 追記先: 本書 + `recipe-tree-and-parts-ssot.md`
4) CondBlockView 生成の “許可入口一覧” を SSOT に追加（Facts 直生成の棚卸し）
   - 対象例: `src/mir/builder/control_flow/plan/facts/exit_only_block.rs`
   - 追記先: `condition-observation-ssot.md`
5) ConditionShape/StepShape → CondProfile 観測統合の移行表（対象ファイル一覧）を SSOT に記載
   - 対象例: `src/mir/builder/control_flow/plan/facts/scan_shapes.rs`
   - 追記先: `condprofile-migration-plan-ssot.md`
6) VerifiedRecipeBlock 生成 API の使用ルールを SSOT/README に固定（parts::entry 以外禁止）
   - 対象: `src/mir/builder/control_flow/plan/parts/entry.rs`, `src/mir/builder/control_flow/plan/recipe_tree/verified.rs`
   - 追記先: `recipe-tree-and-parts-ssot.md`
7) plan ↔ edgecfg 境界の責務整理（直接依存の “許可/禁止” を SSOT で線引き）
   - 対象例: `src/mir/builder/control_flow/plan/mod.rs`, `src/mir/builder/control_flow/plan/features/edgecfg_stubs.rs`
   - 追記先: 本書 +（必要なら）`src/mir/builder/control_flow/plan/README.md`
8) CondProfile/Shape の String/Option 所有ポリシーを SSOT に追加（clone削減の方針）
   - 対象例: `src/mir/policies/cond_profile.rs`
   - 追記先: `condprofile-migration-plan-ssot.md`

### Legacy normalization fallback (shadow)

Scope
- `src/mir/builder/control_flow/normalization/plan_box.rs` の fallback 経路

Reason
- 互換維持のための一時的残置（移行中の安全弁）

Removal condition
- strict/dev で Freeze に収束できること
- entry 統一/recipe-first 経路が安定していること
- fallback 依存のケースが 0 件であること

Verification
- design-only（挙動不変）

## Task order (SSOT)

日々の実行順序は `Compiler Task Map (SSOT)` に従う（迷走防止）:
- `docs/development/current/main/design/compiler-task-map-ssot.md`

## Roadmap to “Recipe-first final Lego” (SSOT)

目的: 「コンパイラーきれいきれい」を完了させてから selfhost/受理拡張へ戻る。
最終形は **語彙を増やさず**、Verifier の契約（PortSig/Obligation/Carriers/JoinPayload）で表現力を閉じる。

前提SSOT（必読）:
- `docs/development/current/main/design/lego-composability-policy.md`（最終レゴ語彙）
- `docs/development/current/main/design/verified-recipe-port-sig-ssot.md`（配線可能性）
- `docs/development/current/main/design/recipe-first-entry-contract-ssot.md`（entry coherence）

### Milestones (ordered)

1) Naming / visibility (docs-first, behavior-preserving)
   - planner_first の `rule=` は安定IDとして固定し、可読性は `label=` と “display name map” で補う。
   - 次は Phase B: TSV/SSOT/Guard Matrix に label（表示名）を併記し、作業者の混乱を止める（テストは rule のみ参照）。

2) Entry coherence → overlap 0（BoxShape）
   - strict/dev(+planner_required) で `entry_ambiguous` を freeze する（順序依存を禁止）。
   - “優先順位表/スコア” を導入せず、guard を狭めて重なりを消す（支配関係で一意化）。

3) Loop vocabulary convergence（BoxShape）
   - 最終形: `loop(cond){...}` は **Loop 1語彙**（Recipe: `Seq/Stmt/If/Loop/Exit`）へ収束。
   - `LoopCond*` / `Pattern*` は “入口ラベル/診断” に縮退し、実体は canonical Loop（generic_loop_v1）へ写像する。

4) Break/Continue/Return（Exit port）を最小語彙で閉じる（BoxShape）
   - Multi-break は “複数の出口エッジ” の一般形として扱い、single break は退化ケース（箱を増やさない）。
   - `SingleBreak/MultiBreak` の箱分割は lowering/PortSig が本質的に別になった場合のみ（それ以外は `break_kind` 観測で十分）。

5) BoxCount（必要な分だけ、合成で閉じる形で pin）
   - “正しく動く（or 期待通り freeze）” が確定してから fixture を pin する。
   - 1ブロッカー=1受理形=1fixture=1コミット（Gate緑で固定）。

6) Selfhost / acceptance expansion（最後）
   - compiler-first の収束が完了した後に、selfhost canary と受理拡張へ戻る（workaround禁止）。

### Exit criteria (“cleanliness done”)

- Recipe語彙が `Seq/Stmt/If/Loop/Exit` に固定され、Verifier-only acceptance が崩れない。
- Entry overlap が実運用で 0（曖昧は strict/dev+planner_required で freeze し、追加時は guard で disjoint 化する）。
- 最低限の gates が緑維持:
  - `phase29bq_fast_gate_vm.sh`
  - `phase29ae_regression_pack_vm.sh`
  - `phase29bp_planner_required_dev_gate_v4_vm.sh`

### RecipeComposer normalizer residue（release準備メモ）

strict/dev(+planner_required) は recipe 直組みへ移行済み。
release も段階移行を開始し、以下は release で recipe 直組みを使用:
- loop_break / if_phi_join / loop_continue_only
- loop_true_break_continue
- loop_cond_{break_continue, continue_only, continue_with_return, return_in_body}
- generic_loop_{v0,v1}

一方で strict/dev の `planner_required=false` は旧ルート（Normalizer）を使う。
release 側の残存 Normalizer 依存は次の段階導入で縮退させる。

PlanNormalizer は非‑Composer用途に限定し、Legacy/Analysis として隔離する（Composer/entry では使わない）。

Naming rule (behavior-preserving):
- runtime/composer 側から legacy normalizer helper を参照する必要が残る場合でも、公開入口は semantic 名を優先する。
- 例: simple-while coreloop helper は `build_simple_while_coreloop` を正とし、legacy file 名 `normalizer/pattern1_coreloop_builder.rs` は互換注記つきで保持する。
- test-only legacy harness も同様に、file 名は据え置いても型名/関数名は route 主語（例: `LoopBreakPlan`, `normalize_loop_break`）へ寄せる。

残存呼び出し（`PlanNormalizer::normalize_*`）:
- loop_simple_while / loop_char_map / loop_array_join / loop_true_early_exit / bool_predicate_scan / accum_const_loop / scan_with_init / split_scan

## Roadmap（Phase 12+ / compiler-first）

目的: selfhost ではなく **コンパイラーの収束**を進める。常に fast gate 緑維持（FAILのまま commit しない）。

### Track B: JoinIR/CorePlan "完成品キット"解体（低リスク・即効）

- ✅ **Phase 12: `nested_loop_depth1*` の統合（Kind enum）** — 完了
  - 結果: 4モジュール → 1モジュール + Kind enum（~400行削除）
- ✅ **Phase 13: `exit_map` / `exit_if_map` 調査 → effects_to_plans 共通化** — 完了
  - 結果: exit_map ≠ exit_if_map（別目的）、effects_to_plans のみ steps/effects.rs に集約

### Track A: 型システム責務分離（影響大・石橋）

- ✅ **Phase 14-15: 完了（責務分離・挙動不変）**
  - SSOT: `docs/development/current/main/design/type-system-policy-ssot.md`
  - 言語意味論SSOT: `docs/reference/language/types.md`（Decision: accepted）
  - 実装: `RuntimeTypeTag`（入口分類）/ `RuntimeTypeSpec`（意味論SSOT）/ `TypeView`（観測専用）

### Track B（重い設計変更 / 後回し）

- **Phase ?: legacy route/rule labels → Feature-set 表現へ移行**
  - これは “設計変更が大きい” ため、当面は分類レンズ（タグ）として扱う（SSOT: Skeleton/FeatureSet）

## Done

- ✅ Phase 5-6: reject_reason/handoff SSOT（loop_cond_continue_only, loop_cond_break_continue）
- ✅ Phase 5-6a-e: planner entry logs SSOT化（`[plan/reject]` / `[plan/accept]` / Freeze message SSOT, 挙動不変）
- ✅ Phase 5-6f: match_return_facts reject SSOT化（`MatchReturn*` reject reasons, 挙動不変）
- ✅ ExitBranch: major direct-exit sites migrated（`branchn_return`, `exit_map`, `generic_loop_body` → `features/exit_branch.rs` helpers）
- ✅ ExitBranch: conditional_update_join migrated（exit helpers）
- ✅ ExitBranch: loop_cond_* ContinueWithPhiArgs migrated（exit helper）
- ✅ Phase 5-6g: body-lowering の同型重複は解消（`steps/stmt_block.rs` を導入、以後は同型のみ移設）
- ✅ Phase 14-15: 型システム責務分離（MirType 意味論リーク 0 件、Decision: accepted）
- ✅ Phase 7: `steps/carrier_collect` 追加済み（loop_cond_continue_only, loop_cond_continue_with_return で共用）
- ✅ Phase 8: `steps/join_payload` 追加済み（conditional_update_join, loop_cond_continue_with_return で共用）
- ✅ Phase 9: `steps/loop_wiring_standard5` 追加済み（loop_cond_continue_only, loop_cond_continue_with_return で共用）
- ✅ Phase 10: HeaderBb legacy path を warn-only で StepBb 強制（loop_cond_continue_only, loop_cond_continue_with_return）
- ✅ Phase 11: HeaderBb legacy path 物理削除（loop_cond_continue_only, loop_cond_continue_with_return）
- ✅ Phase 12: nested_loop_depth1* 統合（4モジュール → 1 + Kind enum, ~400行削除）
- ✅ Phase 13: exit_map/exit_if_map 調査 → effects_to_plans を steps/effects.rs に集約
- ✅ Phase H: LoopCond/LoopTrue/GenericLoop normalizer fallback 撤去（recipe 直組み固定）
- ✅ Normalizer fallback 撤去完了（全ルート recipe 直組み固定）
- ✅ PlanNormalizer 参照ゼロ確認（plan/recipe_tree + joinir/patterns）
- ✅ PlanNormalizer 撤去対象は composer/entry 経路に限定（他は別フェーズ）
- ✅ Composer 経路の PlanNormalizer 参照ゼロ（plan/recipe_tree）
