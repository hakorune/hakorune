# Self Current Task — Backlog (main)

Status: Active  
Scope: 「次にやる候補」を短く列挙するメモ。入口は `docs/development/current/main/10-Now.md`。  

## Active

- JoinIR regression gate SSOT: `docs/development/current/main/phases/phase-29ae/README.md`
- Selfhost canary (secondary/opt-in): `docs/development/current/main/phases/phase-29bq/README.md`

## Planned (next)

- **Async/Concurrency (pre-selfhost; VM+LLVM)**:
  - Plan SSOT: `docs/development/current/main/design/concurrency-async-pre-selfhost-ssot.md`
  - 目的: `nowait/await` を backend 一致で pin（真の並列性は後段）。まずは Future 命令の “動く/動かない” を解消する。

- **ValueFlow / Copy SSOT (docs-first; design-only)**:
  - ValueFlow SSOT: merge の意味論表現を `BlockParams + edge_args` に固定（PHI/Copy は materialize 詳細へ縮退）
    - SSOT: `docs/development/current/main/design/valueflow-blockparams-ssot.md`
  - Copy emission SSOT: Copy を materialize/alias に限定し、救済 Copy を禁止する方針を固定
    - SSOT: `docs/development/current/main/design/copy-emission-ssot.md`
  - ねらい: selfhost Stage-B の `[mir/verify:dominator_violation]` を “原因側” へ寄せるための境界SSOTを先に作る

- **CopyEmitter hardening (SSOT → enforcement → 1-file migrations)**:
  - `tools/checks/no_direct_copy_emission.sh` を PASS する状態まで、残存直Copyを 1ファイル=1コミットで移行する（例外追加は最小、SSOT理由必須）。
  - CopyEmitter の挿入点APIを拡張して、`before_terminator` / `after_phis` でも “直Copyなし” で書けるようにする。
  - `CopyReason` を enum としてSSOT化し、文字列の reason を段階的に縮退する（typo防止）。

- **Cleanliness Wave 2 (design-first tasks)**:
  - CondProfile を loop_var の唯一ソースにする（Facts からの二重抽出を廃止）※ design-only
  - Facts/Canon の二重観測を統合（CondBlockView / ConditionShape / StepShape の責務整理）※ design-only
  - ExitKind depth 情報の標準化（Facts ↔ RecipeTree の不一致解消）※ design-only
  - Verifier vs Lower の重複検証を整理（VerifiedRecipeBlock を唯一ゲートに固定）※ design-only
  - CanonicalLoopFacts 再構築の冗長性整理（Normalizer の再計算を減らす）※ design-only
  - RecipeTree/AST の再構築ループ削減（RecipeBody/StmtRef の共有を優先）※ design-only

- **Cleanliness Wave 3 (inventory → SSOT fix; docs-first)**:
  - Normalization fallback（shadow）の SSOT 明記（残す理由/撤去条件）
    - 対象: `src/mir/builder/control_flow/normalization/plan_box.rs`
    - SSOT: `docs/development/current/main/design/compiler-cleanliness-campaign-ssot.md`
  - JoinIR legacy routing の “入口/退避” 線引きを SSOT に固定
    - 対象: `src/mir/builder/control_flow/joinir/legacy/README.md`
    - SSOT: `docs/development/current/main/design/joinir-design-map.md`（または legacy README を退避専用に明記）
  - DomainPlan → CorePlan の入口順序を SSOT 固定（normalizer vs recipe composer）
    - 対象: `src/mir/builder/control_flow/joinir/patterns/router.rs`, `src/mir/builder/control_flow/plan/recipe_tree/*_composer.rs`
    - SSOT: `docs/development/current/main/design/recipe-tree-and-parts-ssot.md`
  - CondBlockView 生成の “許可入口一覧” を SSOT へ追加（Facts 直生成の棚卸し）
    - 対象: `src/mir/builder/control_flow/plan/facts/**`
    - SSOT: `docs/development/current/main/design/condition-observation-ssot.md`
  - ConditionShape/StepShape → CondProfile 観測統合の移行表（対象ファイル一覧）を SSOT 化
    - SSOT: `docs/development/current/main/design/condprofile-migration-plan-ssot.md`
  - VerifiedRecipeBlock 生成 API の使用ルールを SSOT/README に固定（parts::entry 以外禁止）
    - SSOT: `docs/development/current/main/design/recipe-tree-and-parts-ssot.md`
  - plan ↔ edgecfg 境界の責務整理（直接依存の “許可/禁止” を SSOT で線引き）
    - 対象: `src/mir/builder/control_flow/plan/mod.rs`
  - CondProfile/Shape の String/Option 所有ポリシーを SSOT に追加（clone削減方針）
    - SSOT: `docs/development/current/main/design/condprofile-migration-plan-ssot.md`

- **[BoxCount] loop → if → loop を fixture で pin（入口寄せが前提 / いったん保留）**:
  - 入口寄せ（recipe-first 経路の安定化）が必要
  - BoxCount は最小3形で固定済みのため、次段は回帰安定後に再開
  - 現象: `loop → if → loop` で内側ループが実行されない（期待 10、実 0）
  - `loop → loop → if` は `[joinir/freeze] Loop lowering failed`
  - 設計SSOT: `docs/development/current/main/design/verified-recipe-port-sig-ssot.md`
  - Gap SSOT: `docs/development/current/main/design/generic-loop-v1-acceptance-by-recipe-ssot.md#known-gaps-未対応構造`
  - 調査: `docs/development/current/main/investigations/loop-if-loop-gap.md`
- StepMode generalization (strict/dev only): `StepPlacement/StepMode::InlineInBody` を “例外パターン” ではなく SSOT + verifier で吸収（SSOT: `docs/development/current/main/design/coreloop-stepmode-inline-in-body-ssot.md`）
- Remaining legacy normalizers lego-ization (pipeline/skeleton/feature 化): `src/mir/builder/control_flow/plan/REGISTRY.md` の “Remaining legacy normalizers” table を SSOT として進める
- Cleanup foundation (planned): `CleanupWrap` + cleanup region boundary / `Seq(Block)` の SSOT 固定（selfhost 移植時の負債化を防ぐ）
- NewBox→birth() invariant warning cleanup (milestone): `[warn] dev verify: NewBox FileBox ... not followed by birth()` の整理（builtin/bridge の期待値と verifier を揃える）
  - Design-only note:
    - Scope: builtin/bridge の期待値と verifier を一致させる
    - Non-goal: 受理拡張なし
    - Fix order: verifier expectations → builtin/bridge 修正 → warning 消滅確認

## v2 backlog (deferred)
- B3 sugar: `if local x = f(); x > 0 { ... }` 等（parser sugar; emits BlockExpr directly）
  - 設計SSOT（design only）: `docs/development/current/main/design/block-expr-b3-sugar-decision.md`
  - Decision: `docs/development/current/main/20-Decisions.md`

## Complete

- Phase 29bw: `docs/development/current/main/phases/phase-29bw/README.md`
- Phase 29bv: `docs/development/current/main/phases/phase-29bv/README.md`
- Phase 29bu: `docs/development/current/main/phases/phase-29bu/README.md`
- Phase 29bs: `docs/development/current/main/phases/phase-29bs/README.md`
- Phase 29br: `docs/development/current/main/phases/phase-29br/README.md`
- Phase 29bt: `docs/development/current/main/phases/phase-29bt/README.md`
- Phase 29bp: `docs/development/current/main/phases/phase-29bp/README.md`
- Phase 29bo: `docs/development/current/main/phases/phase-29bo/README.md`
- Phase 29bn: `docs/development/current/main/phases/phase-29bn/README.md`
- Phase 29bm: `docs/development/current/main/phases/phase-29bm/README.md`
- Phase 29bl: `docs/development/current/main/phases/phase-29bl/README.md`
- Phase 29bk: `docs/development/current/main/phases/phase-29bk/README.md`
- Phase 29bj: `docs/development/current/main/phases/phase-29bj/README.md`
- Phase 29bi: `docs/development/current/main/phases/phase-29bi/README.md`
- Phase 29bh: `docs/development/current/main/phases/phase-29bh/README.md`
- Phase 29bg: `docs/development/current/main/phases/phase-29bg/README.md`
- CorePlan migration finalization: `docs/development/current/main/phases/phase-29bf/README.md`
- CorePlan purity Stage-3 (domain-plan-free gate): `docs/development/current/main/phases/phase-29be/README.md`
- CorePlan purity Stage-2 (fallback -> 0): `docs/development/current/main/phases/phase-29bd/README.md`
- Composer API consolidation + cleanup: `docs/development/current/main/phases/phase-29bc/README.md`
- CoreLoopComposer unification: `docs/development/current/main/phases/phase-29bb/README.md`
- CorePlan hardening (docs-first): `docs/development/current/main/phases/phase-29al/README.md`
- CorePlan purity Stage-2: `docs/development/current/main/phases/phase-29ax/README.md`
- Phase 29au: `docs/development/current/main/phases/phase-29au/README.md`
- Phase 29aw: `docs/development/current/main/phases/phase-29aw/README.md`
- Phase 29av: `docs/development/current/main/phases/phase-29av/README.md`
- Phase 29at: `docs/development/current/main/phases/phase-29at/README.md`
- Phase 29as: `docs/development/current/main/phases/phase-29as/README.md`
- Phase 29ar: `docs/development/current/main/phases/phase-29ar/README.md`
- Phase 29ap: `docs/development/current/main/phases/phase-29ap/README.md`
- Phase 29aq: `docs/development/current/main/phases/phase-29aq/README.md`
- Phase 29ca: `docs/development/current/main/phases/phase-29ca/README.md`
- Phase 29cb: `docs/development/current/main/phases/phase-29cb/README.md`
- Unwind integration design (docs-first): `docs/development/current/main/phases/phase-29ay/README.md`
- FlowBox adopt tag migration (strict/dev only): `docs/development/current/main/phases/phase-29az/README.md`

Archive: `docs/development/current/main/phases/phase-29ao/30-Backlog-archive.md`
