# RecipeTree + Parts SSOT

Status: SSOT  
Scope: Recipe の再帰構造と Parts/Verifier の責務境界を固定する。

Related:
- docs/development/current/main/design/lego-composability-policy.md
- docs/development/current/main/design/feature-helper-boundary-ssot.md
- docs/development/current/main/design/verified-recipe-port-sig-ssot.md
- docs/development/current/main/design/selfhost-bootstrap-route-ssot.md
- src/mir/builder/control_flow/plan/REGISTRY.md

## Goal

Facts → Recipe → Lower の境界を壊さず、再帰的な構造処理を Recipe 層に集約する。
Lower は Parts と RecipeTree だけを見る構造に寄せる。

## Recipe-first entry policy (SSOT)

- 現行 runtime 主経路は **`Facts -> Recipe -> Verifier -> Lower`** に固定する。
- legacy planner payload は移行期の語彙で、active docs では historical note としてのみ扱う。
- 新しい受理形は **RecipeBlock/RecipeTree を先に定義**する（Recipe-first）。
- Lower は VerifiedRecipe のみを受け取る（未検証 Recipe の直lowerは禁止）。

### Compatibility lane order (transition-only) (SSOT)

互換用の normalizer residue が残っていても、active runtime contract は recipe-first のまま固定する。
ここでは “現役の入口” と “移行中の互換 lane” の順序だけを SSOT として固定する。

Rule:
- **Primary**: RecipeComposer/RecipeTree 経路（Recipe-first）。
- **Compatibility-only**: PlanNormalizer 経路（transition-only residue）。新規に依存を増やさない。

Allowed Normalizer residue（現状の限定リスト）:
- `loop_simple_while` / `loop_char_map` / `loop_array_join`
- `loop_true_early_exit` / `bool_predicate_scan` / `accum_const_loop`
- `scan_with_init` / `split_scan`

Drift checks:
- `rg -n "PlanNormalizer::normalize_" src/mir/builder/control_flow/plan --glob '!*.md'`（依存が増えていないこと）
- planner-payload residue audit is tracked in `domainplan-residue-ssot.md` and must stay at 0 hit in runtime source（runtime path は domain-free を維持）

## Goal: acceptance by composability (Loop)

Loop 受理は「route-entry 箱の列挙」ではなく、**RecipeBlock の合成可能性**へ寄せる（最終ゴール）。

- 最終目標:
  - Facts は「受理判定」を抱えず、**RecipeBlock を構築できるか**に責務を寄せる。
  - Parts 側は「RecipeBlock を lower できるか（contract を満たすか）」だけを見る（箱固有の例外分岐を増やさない）。
  - route-entry 箱は「観測薄皮（view/facts化の入口）」に縮退し、受理の本体は Recipe/Parts の composability に寄せる。
  - Anti-goal: 非再帰の allowlist（例: ShapeId 列挙）を “受理の真実” にしない（ネスト合成が閉じないため）。
    - generic_loop_v1 の受理SSOT: `docs/development/current/main/design/generic-loop-v1-acceptance-by-recipe-ssot.md`
- Loop の入口部品（SSOT）:
  - `CondBlockView`（view-first / analysis-only）
  - `RecipeBlock(body)`（contract: stmt-only / no-exit / exit-only など。契約違反は `[freeze:contract]`）
  - loop skeleton（steps / blocks / edge wiring）
- LoopCondBreakContinue（planner-required）の補助分類:
  - `LoopCondBreakAcceptKind::ProgramBlockNoExit` は「body が Program/ScopeBox を含むが、ExitIf/ContinueIf/ConditionalUpdate 等のカテゴリが 0」のときの contract-only 分類。
  - 目的は “Facts→Lower の match を閉じて `[plan/freeze:bug] accept_kind missing` を防ぐ” だけ（受理拡張は Facts/Recipe の composability で決まる）。
- LoopCondBreakContinue（planner-required）の body lowering policy（Facts→Lower contract）:
  - `BodyLoweringPolicy::{RecipeOnly, ExitAllowed{allow_join_if}}` を Facts が明示する（allow_extended に暗黙依存しない）。
  - `allow_extended=false` のときは `RecipeOnly`。
  - `ThenOnlyBreakIf` を含むときも `RecipeOnly`。
  - `ExitAllowed` の `allow_join_if` は現状 `false` 固定（exit_allowed 内の join-if は原則禁止）。
- Drift check（受理判定の増殖防止）:
  - `rg -n "try_lower_.*loop_" src/mir/builder/control_flow/plan/features --glob '!*.md'`（features 側に “受理判定” が増殖しない）
  - `rg -n "accept_kind|reject_reason" src/mir/builder/control_flow/plan/features --glob '!*.md'`（accept/reject の判断を features に戻さない）
- Drift check（stmt-only 判定のSSOT化）:
  - `rg -n "try_build_stmt_only_block_recipe\\b" src/mir/builder/control_flow/plan/facts/stmt_view.rs`（Facts が RecipeBlock を作る入口SSOT）
  - `rg -n "classify_cond_prelude_stmt\\b" src/mir/builder/control_flow/plan/features --glob '!*.md'` → 0件（features 側で語彙判定を二重化しない）
  - `rg -n "contains_non_local_exit\\b" src/mir/builder/control_flow/plan/features/generic_loop_body.rs` → 0件（generic_loop_body の stmt-only 手走査を戻さない）
- Drift check（ExitOnly/NoExit 判定のSSOT化）:
  - `rg -n "ends_with_exit_on_all_paths\\b" src/mir/builder/control_flow/plan/features --glob '!*.md'` → 0件（features 側で exit-only の終端判定を持たない）
  - `rg -n "try_build_(stmt_only|exit_only|no_exit)_block" src/mir/builder/control_flow/plan/features --glob '!*.md'` → 想定箇所のみ（Facts SSOT の recipe builder を features 側へ増殖させない）
  - `rg -n "count_control_flow\\b|ControlFlowDetector\\b|then_has_exit\\b|else_has_exit\\b" src/mir/builder/control_flow/plan/parts/stmt.rs` → 0件（parts 側に no-exit 判定を戻さない）
  - `rg -n "try_build_no_exit_block_recipe\\b" src/mir/builder/control_flow/plan/parts/stmt.rs` → 1件以上（parts は Facts SSOT を呼ぶだけ）
  - `rg -n "GeneralIfNoElse" src/mir/builder/control_flow/plan --glob '!*.md'` → 0件（Facts→Recipe→Parts の escape hatch を戻さない）
  - `rg -n "RecipeItem::LoopV0\\b" src/mir/builder/control_flow/plan/facts/no_exit_block.rs --glob '!*.md'` → 1件以上（NoExit は nested loop を構造語彙で運ぶ）
- Drift check（ContinueIfWithElse の recipe-first 維持）:
  - `rg -n "block_lowering::lower_loop_cond_recipe_block" src/mir/builder/control_flow/plan/features/loop_cond_break_continue_pipeline/continue_if.rs --glob '!*.md'` → 0件（旧 block_lowering 依存を戻さない）
- Drift check（ProgramBlock の recipe-first 維持）:
  - `rg -n "ProgramBlock\\(" src/mir/builder/control_flow/plan/features/loop_cond_break_continue_pipeline --glob '!*.md'` → 0件（features 側に ProgramBlock 特例 lowering を戻さない）
- Drift check（NestedLoopDepth1 の payloadization 維持）:
  - `rg -n "NestedLoopDepth1\\(StmtRef\\)" src/mir/builder/control_flow/plan/loop_cond_break_continue/recipe.rs --glob '!*.md'` → 0件（StmtRef-only payload を戻さない）
  - `rg -n "if let Some\\(body_recipe\\) = payload_stmt_only" src/mir/builder/control_flow/plan/features/loop_cond_break_continue_pipeline/item_lowering.rs --glob '!*.md'` → 1件以上（payload優先経路を維持）
- Drift check（ExitIf/TailBreak の ExitAllowed payloadization 維持）:
  - `rg -n "ExitIf\\(StmtRef\\)" src/mir/builder/control_flow/plan/loop_cond_break_continue/recipe.rs --glob '!*.md'` → 0件（StmtRef-only variant を戻さない）
  - `rg -n "\\bTailBreak\\s*," src/mir/builder/control_flow/plan/loop_cond_break_continue/recipe.rs --glob '!*.md'` → 0件（unit variant を戻さない）
- Drift check（ElseOnlyReturnIf の payloadization 維持）:
  - `rg -n "ElseOnlyReturnIf\\(StmtRef\\)" src/mir/builder/control_flow/plan/loop_cond_break_continue/recipe.rs --glob '!*.md'` → 0件（StmtRef-only variant を戻さない）
  - `rg -n "then_no_exit\\.as_ref\\(\\)" src/mir/builder/control_flow/plan/features/loop_cond_break_continue_pipeline/item_lowering.rs --glob '!*.md'` → 1件以上（payload を渡す経路を維持）
- Drift check（recipe の “袋( Vec<ASTNode> ) field” 禁止）:
  - `rg -n "pub .*Vec<ASTNode>" src/mir/builder/control_flow/plan --glob '*recipe.rs' --glob '!*.md'` → 0件（recipe payload は RecipeBody/RecipeBlock に集約）
- Drift check（nested_loop_depth1 facts の payloadization 維持）:
  - `rg -n "Vec<ASTNode>" src/mir/builder/control_flow/plan/nested_loop_depth1/facts.rs --glob '!*.md'` → 0件（facts 側の “袋” を戻さない）
  - `rg -n "body_stmt_only" src/mir/builder/control_flow/plan/nested_loop_depth1/facts.rs --glob '!*.md'` → 1件以上（payload を保持する）
- Drift check（ElseGuardBreakIf else=ExitAllowed 経路の維持）:
  - `rg -n "lower_exit_allowed_block_verified\\(" src/mir/builder/control_flow/plan/features/loop_cond_break_continue_pipeline/else_patterns.rs --glob '!*.md'` → 1件以上
- Drift check（loop body 1パーツ（ExitAllowed）経路の維持）:
  - `rg -n "body_exit_allowed\\b" src/mir/builder/control_flow/plan/loop_cond_unified/variants/break_continue.rs --glob '!*.md'` → 1件以上
- Drift check（loop_true の body 1パーツ（ExitAllowed）経路の維持）:
  - `rg -n "lower_exit_allowed_block_verified\\(" src/mir/builder/control_flow/plan/features/loop_true_break_continue_pipeline.rs --glob '!*.md'` → 1件以上
- Drift check（generic_loop_body の body 1パーツ（ExitAllowed）経路の維持）:
  - `rg -n "lower_exit_allowed_block_verified\\(" src/mir/builder/control_flow/plan/features/generic_loop_body.rs --glob '!*.md'` → 1件以上
- Vocabulary SSOT（stmt-only の許可語彙）:
  - SSOT: `src/mir/builder/control_flow/plan/policies/cond_prelude_vocab.rs`
  - `try_build_stmt_only_block_recipe()` / `CondBlockView::prelude_stmts` / normalizer は **必ず** `classify_cond_prelude_stmt()` を参照する（語彙の二重化禁止）。
  - Drift check: `rg -n "enum CondPreludeStmtKind\\b" src/mir/builder/control_flow/plan --glob '!*.md'` → 1件（語彙定義を増やさない）

## Final milestone (Loop RecipeBlockization v0)

Rust 層の “中間ゴール” として、loop body の受理と lowering を **RecipeBlock 契約で統一**する。
この状態は `.hako mirbuilder` 移植の写経元SSOTとして扱える。

- Done criteria:
  - loop body を必ず次のいずれかに落とし、Parts で lower する（features 側に手判定を戻さない）:
    - `StmtOnlyBlockRecipe`（SSOT: `src/mir/builder/control_flow/plan/facts/stmt_view.rs`）
    - `ExitOnlyBlockRecipe`（SSOT: `src/mir/builder/control_flow/plan/facts/exit_only_block.rs`）
    - `NoExitBlockRecipe`（SSOT: `src/mir/builder/control_flow/plan/facts/no_exit_block.rs`）
  - NoExit の join branch は `NoExitBlockRecipe` の再帰で閉じる（then/else を stmt-only 裸リストにしない）。
  - Verifier は NoExit join branch の再帰に追従する（NoExit verifier が then/else を NoExit として検証する）。
  - 共通ポリシーは `facts/block_policies.rs` に集約する（同一判定の二重化禁止）。
  - loop_cond_break_continue の主要 item は recipe-first payload を運び、features 側は parts へ渡すだけ:
    - `ProgramBlock` / `ContinueIfWithElse` / `GeneralIf` / `ConditionalUpdateIf`

- Drift checks (closeout):
  - `rg -n "block_lowering::lower_loop_cond_(block|recipe_block)\\b" src/mir/builder/control_flow/plan/features --glob '!*.md'` → 0件（旧 lowering 依存を戻さない）
  - `rg -n "ControlFlowDetector\\b|count_control_flow\\b|then_has_exit\\b|else_has_exit\\b" src/mir/builder/control_flow/plan/features --glob '!*.md'` → 0件（features 側に手判定を戻さない）
  - `rg -n "ProgramBlock\\(" src/mir/builder/control_flow/plan/features/loop_cond_break_continue_pipeline --glob '!*.md'` → 0件（ProgramBlock の旧lowerを戻さない）
  - Status: ✅ achieved（closeout drift checks are green）

## Non-goals

- CorePlan の即時削除（段階的縮退のみ）
- AST rewrite による通過
- 箱ごとの lowering 実装の増殖

## Pipeline (SSOT)

AST
  → Canon/View (analysis-only, no rewrite)
  → Facts (観測のみ)
  → Router/Composer (Facts → Recipe)
  → RecipeVerifier (fail-fast: [freeze:contract])
  → Parts/Lower (VerifiedRecipe only)
  → emit_frag
  → verify / DCE

## Verified-only lowering boundary (type-level contract)

Lower is forbidden from touching unverified recipes. Enforce this at the type
boundary to prevent “silent re-interpretation” in Parts.

### Contract (SSOT)

- `RecipeBlock` → **must** pass Verifier to become `VerifiedRecipeBlock`.
- `Lower` accepts **only** `VerifiedRecipeBlock`.
- Unverified recipes are not a valid input to Parts/Lower (compile-time error).
- Exception (verification-only): `recipe_tree::matcher` may call
  `check_block_contract(...)` to validate structure but must **not** lower or
  construct `VerifiedRecipeBlock`.

### Minimal type shape (implemented boundary)

```
struct RecipeBlock { ... }            // raw, unverified
struct VerifiedRecipeBlock { ... }    // wrapper created only by Verifier

fn verify_recipe(block: &RecipeBlock) -> Result<VerifiedRecipeBlock, Freeze>;
fn lower_block(block: &VerifiedRecipeBlock, ...) -> Result<..., Freeze>;
```

Current code anchor:
- `recipe_tree/verified.rs` defines `VerifiedRecipeBlock`
- `parts::entry` is the only intended verify/lower gate

### Migration rule

- New/edited code must pass through `parts::entry` (Verifier gate).
- Direct `RecipeBlock` lowering is a bug (use the Verified wrapper).

## RecipeTree (Minimal vocabulary)

Legacy note (SSOT unification):
- SSOT は `RecipeBlock + RecipeItem` に固定する（Parts/Verifier の入口もこちら）。
- `recipe_tree::RecipeNode` は削除済み（旧legacyの二重SSOTを禁止）。

現行 SSOT（構造語彙）は `RecipeBlock` と `RecipeItem` のみ。

- **Structure**: `RecipeBlock { body_id, items: Vec<RecipeItem> }`
  - `RecipeItem::{Stmt, IfV2, LoopV0, Exit}` のみ（拡張は構造語彙に限定）
- **Storage**: `RecipeBodies/RecipeBody`（arena, out-of-band）
  - `StmtRef/BodyId` の参照先として保持するだけ（制御構造は持たない）
- **Views**: `CondBlockView`（analysis-only view）
  - Facts 側が `CondBlockView::from_expr` で作り、Parts は view をそのまま使う
- **Verified gate**: `VerifiedRecipeBlock`（Verifier を通過した wrapper）
  - `parts::entry` が唯一の verify 入口（release でも常時検証）
  - `parts::dispatch` は Verified を前提に lower する（受理の再判定はしない）
  - `lower_*_verified` が canonical lowering API。non-verified wrapper は legacy shim で、薄く保つ
  - **Parts entrypoints (SSOT)**: `parts::entry::{verify_exit_only_block, verify_stmt_only_block, verify_exit_allowed_block, verify_no_exit_block, lower_exit_only_block_verified, lower_exit_allowed_block_verified, lower_no_exit_block_verified, lower_no_exit_block_with_stmt_lowerer_verified, lower_exit_allowed_block, lower_no_exit_block, lower_no_exit_block_with_stmt_lowerer, lower_if_join_with_branch_lowerers, lower_value_cond_if_with_filtered_joins, lower_loop_with_body_block, lower_loop_v0, lower_nested_loop_depth1_stmt_only, lower_nested_loop_recipe_stmt_only}`

## Final Target (Vocabulary set)

最終形では「箱ごとの特化 lowering」をやめ、再帰構造は RecipeTree に集約する。
“ExitIf/ExitAll” のような性質は `IfMode` と `RecipeVerifier` で表現し、特別な `ExitIfTree` 語彙を増やさない。

### Final SSOT (structure + storage)

- **SSOT: RecipeTree（構造のみ）**
  - Verifier が “lower 可能” を保証できるかで受理を決める
  - Lower は VerifiedRecipe のみを見る（再判定しない）
- **Storage: RecipeBody（arena / out-of-band）**
  - StmtRef/BodyId の参照先としての保管庫
  - **制御構造は持たない**（木にしない）
- 方針: 「RecipeTree = red（外の真実）」「RecipeBody = green（内部実装）」で固定する。

### Public boundary (SSOT)

- **公開してよいもの（red / SSOT）**
  - `RecipeTree`（構造）
  - `RecipeItem::{Stmt,IfV2,LoopV0,Exit}`（構造語彙）
  - `CondBlockView`（analysis-only view）
- **公開してよい理由**
  - 受理/Verifier/Lower の“唯一の真実”を参照するため
- **公開してはいけないもの（green / internal）**
  - `RecipeBody` の実体（構造を持つ形で外に出さない）
  - `RecipeBodies::bodies` の直接アクセス
  - CFG/SSA/ValueId/BasicBlockId/Frag/PHI/Join wiring など
- **運用**
  - `RecipeBody` は `BodyId`/`StmtRef` を通した “参照専用” のみ許可
  - Verifier/Parts/Lower は RecipeTree に対してのみ契約判定を行う

Drift check:
- `rg -n "pub .*RecipeBodies\\b" src/mir/builder/control_flow/plan --glob '!*.md'` → 想定箇所のみ（public exposure を増やさない）
- `rg -n "verify_.*_contract" src/mir/builder/control_flow/plan/parts/dispatch.rs --glob '!*.md'` → 0件（dispatch に verifier 直呼びを戻さない）
- `rg -n "lower_.*_block_verified" src/mir/builder/control_flow/plan/parts/entry.rs --glob '!*.md'` → 1件以上（Verified entry が存在する）
- `rg -n "lower_nested_loop_depth1_any" src/mir/builder/control_flow/plan/parts/dispatch.rs --glob '!*.md'` → 0件（LoopV0 の AST 直下ろし禁止）
- `rg -n "lower_return_prelude_block\\(" src/mir/builder/control_flow/plan/parts/stmt.rs --glob '!*.md'` → 0件（return-prelude の AST 直再帰を戻さない）
- `rg -n "ASTNode::(Program|ScopeBox|Loop|While)\\b" src/mir/builder/control_flow/plan/parts/stmt.rs --glob '!*.md'` → 0件（return-prelude の container 分岐を Parts に戻さない）

- **Structure**: `Seq` / `If` / `Loop` / `Stmt`
- **Exit**: `ExitKind`（Return/Break/Continue + depth）
- **Join**: `JoinPayload`（PHI/join の散逸を止める SSOT）
- **Scope/Region**: cleanup・寿命境界（将来。RecipeTree の語彙として持つ）
- **Analysis-only views**: `CondBlockView` / `CondCanon` / `UpdateCanon` / `StepPlacement/StepMode`
- **Verification**: `RecipeVerifier`（機械検証のみ）

## IfMode (SSOT)

- `IfMode::ExitIf`
  - then は exit-only
  - else は fallthrough 可（Optional）
- `IfMode::ExitAll`
  - then/else 両方 exit-only
  - else 必須
- `IfMode::ElseOnlyExit`
  - then は no-exit（fallthrough）
  - else は exit-only（break/continue/return）
  - else 必須

ExitIfTree の曖昧さは IfMode で解消する。

## If Vocabulary Unification (SSOT)

方針: `RecipeItem` の “if” 語彙は 1 つに統一し、契約差は `IfContractKind` で表現する。

- SSOT: `RecipeItem::IfV2 { if_stmt, cond_view, contract: IfContractKind, then_block, else_block }`
  - `IfContractKind::ExitOnly { mode: IfMode }`（exit-only / 旧 `RecipeItem::If` 相当（撤去済み））
- `IfContractKind::ExitAllowed { mode: IfMode }`（else-only-exit pattern）
    - `mode: ElseOnlyExit` のみ（then=fallthrough, else=exit）
    - ElseOnlyExit は base 受理（planner_required のみに限定しない）
  - `IfContractKind::Join`（join-bearing / 旧 `RecipeItem::IfJoin` 相当）

Drift check（到達条件）:
- `rg -n "RecipeItem::IfJoin\\b" src/mir/builder/control_flow/plan --glob '!*.md'` → 0件
- `rg -n "RecipeItem::If\\b" src/mir/builder/control_flow/plan --glob '!*.md'` → 0件
- Drift check（ElseOnlyExit パターンの維持）:
  - `rg -n "ElseOnlyExit" src/mir/builder/control_flow/plan --glob '!*.md'` → 想定箇所のみ（recipe_tree, exit_only_block, dispatch, verified, verify）
  - `rg -n "IfContractKind::ExitAllowed" src/mir/builder/control_flow/plan --glob '!*.md'` → 想定箇所のみ
  - `rg -n "lower_else_only_exit_if\\b" src/mir/builder/control_flow/plan/parts/dispatch.rs` → 1件以上

## Loop Vocabulary (SSOT)

Loop は **構造語彙**として `RecipeItem::LoopV0` に固定する（CFG/SSA を Recipe に入れない）。

- 禁止: BB/PHI/Frag/ValueId/CFG などの “下ろしの都合” を recipe payload に持ち込まない。
- Contract:
  - `body_contract` に応じて verifier/parts が `RecipeBlock` を下ろす（再判定しない）。
- Lowering SSOT:
  - `RecipeItem::LoopV0` は `parts::loop_::lower_loop_v0` で lower する（RecipeTree→Parts のみ、AST loop lowering 禁止）。
  - body の語彙は既存の `StmtOnly` / `NoExit` / `ExitAllowed` / `ExitOnly` 契約に従う。

Drift check:
- `rg -n "LoopV0" src/mir/builder/control_flow/plan --glob '!*.md'` → 1件以上（LoopV0 導入が進んでいる）

### scan 系: linear は RecipeBlock、nested は planner

- `loop_scan_methods_v0` などの scan 系は、**線形部分を `NoExitBlockRecipe` として固定**し、nested loop は single_planner に委譲する（別レゴとして合成）。
- `loop_scan_methods_block_v0` も同じ方針で segment-first にする（BoxCountで追加された “ブロック内に nested loop” 形の薄皮）。
- pipeline 側での直 lower（手走査/手 lowering）を戻さない。

scan系 segment vocabulary SSOT:
- `src/mir/builder/control_flow/plan/scan_loop_segments.rs`

Drift check:
- `rg -n "lower_stmt_list\\b|lower_stmt\\b" src/mir/builder/control_flow/plan/loop_scan_methods_v0 --glob '!*.md'` → 0件
- `rg -n "NestedLoop\\(StmtRef\\)" src/mir/builder/control_flow/plan/loop_scan_methods_v0 --glob '!*.md'` → 0件
- `rg -n "LoopV0" src/mir/builder/control_flow/plan/loop_scan_methods_v0 --glob '!*.md'` → 1件以上
- `rg -n "NestedLoop\\(StmtRef\\)" src/mir/builder/control_flow/plan/loop_scan_phi_vars_v0 --glob '!*.md'` → 0件
- `rg -n "LoopV0" src/mir/builder/control_flow/plan/loop_scan_phi_vars_v0 --glob '!*.md'` → 1件以上
- `rg -n "NestedLoop\\(StmtRef\\)" src/mir/builder/control_flow/plan/loop_scan_v0 --glob '!*.md'` → 0件
- `rg -n "LoopV0" src/mir/builder/control_flow/plan/loop_scan_v0 --glob '!*.md'` → 1件以上
- `rg -n "struct NestedLoopRecipe\\b" src/mir/builder/control_flow/plan/loop_scan_* --glob '!*.md'` → 0件
- `rg -n "enum LoopScanSegment\\b" src/mir/builder/control_flow/plan/loop_scan_* --glob '!*.md'` → 0件
- `rg -n "lower_nested_loop_depth1_any\\b" src/mir/builder/control_flow/plan/loop_scan_* --glob '!*.md'` → 0件
- `rg -n "lower_nested_loop_recipe_stmt_only\\b" src/mir/builder/control_flow/plan/loop_scan_* --glob '!*.md'` → 1件以上
- `rg -n "lower_stmt_list_join_if_and_nested_loops\\b|lower_effect_stmt\\b" src/mir/builder/control_flow/plan/loop_scan_methods_block_v0 --glob '!*.md'` → 0件
- `rg -n "LoopScanSegment\\b" src/mir/builder/control_flow/plan/loop_scan_methods_block_v0 --glob '!*.md'` → 0件（ローカル袋禁止: `scan_loop_segments` の alias を使う）
- `rg -n "pub .*RecipeBody\\b" src/mir/builder/control_flow/plan --glob '!*.md'` → 想定箇所のみ（payload用の `pub body: RecipeBody` など）
- `rg -n "RecipeBodies::bodies" src --glob '!*.md'` → 0件

### Segmenter SSOT (linear contract only)

BlockContract は **線形区間のみに適用**し、混在は `Seq` に分割する。

- 境界で必ず割る: `Loop` / `Exit` / 「契約に収まらない If」。
- `MixedBlockRecipe` は追加しない（必要なら `Seq` へ分割）。
- ルール: `NoExit` は **線形のみ**を目標とする（`LoopV0` は構造ノード側へ退避）。

Next (BoxShape):
- `NoExitBlockRecipe` から `LoopV0` を除外して、線形専用へ戻す。
- segmenter の境界判定を SSOT 化（scan 系だけでなく一般パスにも適用）。

## Parts (Lowerer dependency)

Lowerer は Parts 以外の箱依存を持たない。

- `parts/seq.rs`: Seq の lowering と terminator 整合
- `parts/stmt.rs`: StmtRef の lowering
- `parts/if_.rs`: If lowering (mode で join/exit を固定)
- `parts/exit.rs`: Exit lowering (Return/Break/Continue)
- `parts/loop.rs`: Loop skeleton 組み立て
- `parts/join.rs`: JoinPayload 生成・検証
- `parts/verify.rs`: RecipeVerifier
- `parts/dispatch.rs`: if/join/branch/value-cond の assembly SSOT（filtered joins を含む）
- `parts/conditional_update.rs`: conditional_update lowering SSOT（features 側は facade のみ）

## Cond Prelude (status)

Phase B2 により `ASTNode::BlockExpr { prelude_stmts, tail_expr }` が導入され、`CondBlockView` は **構造的に** condition prelude を表現できるようになった。
ただし現状の JoinIR/plan の lowering は、互換性維持のため **prelude は reject-by-contract**（fail-fast）に固定している（暫定）。

- SSOT: `docs/development/current/main/design/cond-block-view-prelude-ssot.md`
- Roadmap SSOT: `docs/development/current/main/design/map-literal-eviction-and-blockexpr-roadmap-ssot.md`（Phase B4）

## RecipeVerifier (Fail-fast)

機械検証のみ（意味論チェックは禁止）。

- Seq の途中に Exit が出たら後続禁止
- ExitAll は else 必須、then/else が exit-only
- Break/Continue depth が LoopFrame で解決できる
- join が必要な If で join spec が欠けていない
- IfV2{contract: Join} は **then/else の2-edgeのみ** が join に到達することを保証する
  - pred と join inputs が一致しない場合は freeze（fail-fast）
  - 片側未定義（片枝のみ代入）は planner_required では freeze（暗黙の pre 値流入は禁止）

Verifier は 2層で運用する。

- SSOT: Verifier が常時 fail-fast（契約違反は `[freeze:contract][recipe] ...`）を担う。
- Dispatch/Parts の contract 再判定は debug-only（`cfg(debug_assertions)`）に縮退する。

- ContractVerifier: 契約違反は常時 fail-fast（SSOT）
- DebugVerifier: 冗長チェックのみ dev/strict

### Verifier SSOT (current)

現状の SSOT は `RecipeBlock`（body_id + items）に対する検証で、入口は `parts/entry.rs`。
旧 `RecipeNode` verifier は撤去済み（SSOT の二重化を禁止）。
契約チェックは常時ON、冗長チェックのみ dev/strict に限定する（Decision 参照）。
Drift check:
- `rg -n "RecipeNode::" src/mir/builder/control_flow/plan --glob '!*.md'` → 0件（RecipeNode の利用を戻さない）
- `rg -n "recipe_tree::node" src --glob '!*.md'` → 0件（legacy node モジュールを戻さない）
- `rg -n "verify_recipe_contract_if_enabled" src/mir/builder/control_flow/plan` → 0件
- `rg -n "ASTNode::If" src/mir/builder/control_flow/plan/parts/dispatch.rs` → 0件
- `rg -n "CondBlockView::from_expr\\(" src/mir/builder/control_flow/plan/parts/dispatch.rs` → 0件
- `rg -n "cond_view: _" src/mir/builder/control_flow/plan/loop_cond_break_continue/recipe_to_block.rs` → 0件
- `rg -n "CoreIfPlan\\b" src/mir/builder/control_flow/plan/features` → 0件（doc を除く）
- `rg -n "build_join_payload_filtered\\(" src/mir/builder/control_flow/plan/features` → 0件
- `rg -n "joinir_dev::debug_enabled\\(" src/mir/builder/control_flow/plan --glob '!*.md'` → 0件

## CorePlan Policy

当面は薄いアダプタとして残す。

Phase 1: RecipeTree → Parts → CorePlan (CFG/SSA の機械)
Phase 2: CorePlan を LoweredRecipe に縮退
Phase 3: emit_frag が汎用化できたら CorePlan を削除

## CorePlan shrink criteria (SSOT)

- CorePlan を "意味論ゼロ" とみなせる条件（＝縮退/削除の入口）
    - CorePlan が **受理形の判定（shape/accept_kind/reject_reason）**を持たない
    - CorePlan が **条件式の解釈・分類（ExitIf/ExitAll 等）**を持たない
    - CorePlan が Recipe の再判定をしない（Lower は Recipe/Parts のみ）
- CorePlan が保持してよいもの（機械のみ）
    - BasicBlock 割当・CFG wiring・Frag 構築・sealing（FragEmitSession 等）
    - PHI の wiring / port→payload の接続（JoinPayload SSOT の"機械"部分）
    - debug/dump（dev-only、タグ固定、無条件出力禁止）
- CorePlan が保持してはいけないもの
    - "箱ごとの例外" の分岐
    - AST 直読み / rewrite
    - 部品（Parts）を跨ぐ意味語彙の判断
- 削除の前提（CorePlan を消す前に必要な状態）
    - Parts が Frag を直接生成できる（または同等のLoweredRecipe表現がある）
    - RecipeVerifier が SSOT になり、契約違反は [freeze:contract][recipe] で落ちる
    - phase29bq_fast_gate_vm.sh と selfhost canary が継続して green

## Migration Steps (BoxShape, reversible)

1) RecipeTree + Parts の土台追加（挙動不変）
2) RecipeVerifier 追加（当初は dev/strict のみ。現行は契約チェック常時ON）
3) ExitIfTree を IfMode へ変換（曖昧さ排除）
4) ExitIfTree lowering を Parts に移設
5) item_lowering を Parts dispatch に置換
6) accept_kind をログ用途へ降格

## Roadmap to the final form (SSOT)

最終形へ向けた段階的な予定（小さく・可逆・1コミット単位）。

### M0 (now)

- BoxCount: `loop_cond_break_continue` に再帰 recipe（`ExitLeaf` / `ExitIfTree`）を導入し、nested if-in-loop を fixture+gate で固定済み。
- `loop_cond_break_continue`: then-only continue（else無し）は nested loop がある場合のみ ExitIf 扱いで受理（continue_if_seen 不在でも可）。
- `loop_cond_break_continue`: ProgramBlock を含む場合は exit signal ありとして受理（no-exit でも可）。
- `loop_cond_break_continue`: nested-only（exit_if/continue_if/cond_update/ProgramBlock なし）の no-exit 受理は cluster profile（require_nested_loops 指定）時のみ許可する。
- Cluster profiles は nested count 3+ のみを対象とし、nested=1 は base 抽出（exit signal 必須）に寄せる。
- `loop_cond_break_continue`: break/continue/return を含まない If は受理してよい（exit signal が無い If のみ許可）。
- `loop_cond_break_continue`: break_kind（Single/Multi）は観測のみで箱分割しない（trace で可視化）。
  - 観測例: `[plan/trace:loopcond_break_kind] kind=Multi exit_sites=2`
- LoopCond* は canonical Loop（generic_loop_v1）へ写像し、入口ラベルとしてのみ残す（レゴ語彙は増やさない）。
- `parts/loop_v0`: carrier は body 非localに加え、pre で定義済みの代入先（local 含む）も持ち上げる。
- `generic_loop_v1`: nested loop 後は pre 既知の bindings を variable_map で同期する（更新の取りこぼし防止）。

### M1 (scaffold: no behavior change)

- Historical scaffold note: `recipe_tree/`（型）と `parts/`（dispatch 入口）を追加するが、当時の既存 pipeline は触らない。
- 目的: “Parts only lowering” に移行できる置き場を先に作る。

Done:
- `cargo check` / fast gate green
- 新しい型が既存の lowering に影響しない（呼び出しゼロ）

### M2 (verifier: initial dev/strict only)

- `RecipeVerifier` を追加し、dev/strict でのみ有効化（fail-fast: `[freeze:contract][recipe]`）。
  - その後の決定で「契約チェック常時ON / 冗長チェックのみ dev/strict」に更新。
- まずは `Seq/If(ExitIf/ExitAll)/Exit` の機械検証だけを導入する（意味論検証は禁止）。

Done:
- verifier を入れても fast gate green
- verifier の失敗は 1行タグで止まる

### M3 (parts dispatch: move one lowering)

- `loop_cond_break_continue` の “ExitIfTree/ExitLeaf lowering” を `parts/if.rs` + `parts/exit.rs` に移設し、pipeline 側は parts を呼ぶだけにする。

Done:
- `loop_cond_break_continue_pipeline/*` に tree-lowering の本体が残らない（呼び出しのみ）
- fast gate green

### M4 (IfMode adoption: remove ambiguity)

- `ExitIfTree` の意味を `IfMode::{ExitIf, ExitAll}` に寄せる（変換アダプタで開始し、徐々に語彙を縮退）。
- 最終的に `ExitIfTree` は “移行用 alias” としてのみ残す（or 削除）。

Done:
- ExitAll は else 必須、ExitIf は else optional の契約が verifier で固定
- fast gate green

### M5 (expand to other features)

- 影響の小さい箱から順に、recipe→parts の経路を増やす（BoxShape）。
- “箱ごとの lowering” を禁止していく（構造で逆流を止める）。

Progress (M5a):
- ✅ `exit_branch::lower_return_stmt_with_effects` の features 直呼びを廃止し、`parts::exit::lower_return_stmt_with_effects` に統一した（return 経路の SSOT を Parts に寄せた）。
  - Drift check: `rg "exit_branch::lower_return_stmt_with_effects" src/mir/builder/control_flow/plan/features` → 0件

## Definition: loop_cond_break_continue の “完全Recipe化”

`loop_cond_break_continue` について「完全Recipe化」と呼ぶ条件（完了条件SSOT）をここで固定する。

- Recipe payload:
  - `LoopCondBreakContinueItem` の payload が **AST/StmtRef を直接運ばない**（CondViewRef / RecipeBlock / ExitOnly/NoExit/StmtOnly recipes など “recipe-first payload” のみを運ぶ）。
  - 例外: `RecipeBlock` 内部の `StmtRef`（参照としての stmt idx）は許容（body は arena 参照で保持される）。
  - 受理移行中の例外: Else/Then only return/break の `StmtRef` は **branch-local**（then/else 内の idx）。Verifier は
    loop body の長さではなく **branch length** で検証する（`if_stmt` 自体は body-local）。
- Lowering boundary:
  - `src/mir/builder/control_flow/plan/features/loop_cond_break_continue_pipeline/*` は **parts 呼び出しのみ**（手組み join / 手走査 / 手 lowering を持たない）。
  - 受理判定（shape/vocab/exit 判定）は Facts SSOT 側へ集約され、features は再判定しない。
- Drift checks:
  - “payload が StmtRef を運ぶ variant が残っていない” を `rg` で固定（例: `rg -n "NestedLoopDepth1\\(StmtRef\\)" ...` のようなチェックを追加する）。
  - “features が ASTNode を直読して判定していない” を `rg` で固定（例: `rg -n "match stmt\\b|node_type\\(" ...`）。

Next (M5b):
- ✅ `exit_branch::{build_break_with_phi_args, build_continue_with_phi_args}` の features 直呼びを廃止し、`parts::exit::{build_break_with_phi_args, build_continue_with_phi_args}` に統一した。
  - Drift check: `rg "exit_branch::build_(break|continue)_with_phi_args" src/mir/builder/control_flow/plan/features --glob '!*.md'` → 0件

Progress (M5c):
- ✅ M5c-1: `ExitLeafKind` を削除し、`recipe_tree::ExitKind` に統一した。
  - Drift check: `rg "ExitLeafKind" src/mir/builder/control_flow/plan` → 0件
- ✅ M5c-2: `RecipeNode` は legacy として凍結（export停止 + `RecipeNode::` 利用 0件を drift check で固定）。
  - Drift check: `rg -n "RecipeNode::" src/mir/builder/control_flow/plan --glob '!*.md'` → 0件
- ✅ M5c-3: `item_lowering` 依存を `parts/if_.rs` から除去した。
  - Drift check: `rg "loop_cond_break_continue_pipeline::item_lowering" src/mir/builder/control_flow/plan/parts/if_.rs` → 0件
- ✅ M5c-4: `HAKO_EXIT_TREE_DEBUG` 環境変数を撤去し、環境変数スパローとログ契約違反を解消した。
  - Drift check: `rg "HAKO_EXIT_TREE_DEBUG" src/mir/builder/control_flow/plan` → 0件

Progress (M5d):
- ✅ `parts::{stmt,exit}` を SSOT にし、`features::exit_branch` は委譲のみへ縮退した（依存方向を features→parts に統一）。

### M6 (RecipeBlock view-first)

- ✅ `RecipeItem::IfV2` が `CondBlockView` を保持し、`parts/dispatch.rs` は AST を直読せずに `cond_view` だけで if を lower する。
- 目的: “受理側が view を作って渡す → parts はそれを使うだけ” の線を SSOT として固定し、M7 以降の横展開を安全にする。

### M7 (non-exit stmt-only RecipeBlock dispatch)

- ✅ `parts/dispatch.rs` に “stmt-only（non-exit）ブロック” の lowering 入口 `lower_stmt_only_block(...)` を追加した。
- ✅ `parts/stmt.rs` の general-if lowering は、planner_required 経路でのみ then/else を stmt-only RecipeBlock に変換し、dispatch 経由で block を下ろす。

Drift checks:
- `rg -n "lower_stmt_only_block\\(" src/mir/builder/control_flow/plan/parts/stmt.rs` → 1件以上
- `rg -n "RecipeItem::Stmt\\b" src/mir/builder/control_flow/plan/parts/dispatch.rs` → 1件以上
- `rg -n "verify_stmt_only_block_contract_if_enabled" src/mir/builder/control_flow/plan` → `parts/verify.rs` + `parts/dispatch.rs` のみ

### M8 (join-bearing non-exit if via RecipeBlock)

- ✅ join を伴う general-if を RecipeBlock/dispatch で lower できる語彙を導入し、現在は `RecipeItem::IfV2 { contract: IfContractKind::Join, ... }` に統一している（`RecipeItem::IfJoin` は撤去済み）。
- ✅ `parts/dispatch.rs` に `lower_no_exit_block(...)` を追加し、join 生成（`build_join_payload`）と join の適用をここへ集約した。
- ✅ `parts/stmt.rs` は `try_lower_general_if_view` を使わず、planner_required 経路で `IfV2{Join}` を組んで dispatch に渡す（既定のコスト増を避ける）。
- ✅ `NoExitBlockRecipe` の join-if 枝は `NoExitBlockRecipe` を再帰で保持する（Option A: contract closed）。

#### SSOT: join 反映ポリシー注入（NoExitBlockRecipe / IfV2{Join}）

`NoExitBlockRecipe`（`RecipeItem::IfV2 { contract: Join, .. }` を含む）を lower する時、**join 後に `current_bindings` を更新する変数集合**は箱ごとに異なる。
これを features 側の手実装へ戻さないため、SSOT は `parts/dispatch.rs::lower_no_exit_block_with_stmt_lowerer(...)` の引数として **ポリシー注入**する。

- 注入点（SSOT）:
  - `make_lower_stmt`: stmt lowering（箱の文脈に応じて carrier_updates 等の局所状態を持てる）
  - `should_update_binding(name, bindings) -> bool`: join 反映を `current_bindings` に適用するかを決める
- Contract:
  - Facts が `NoExitBlockRecipe` を返す時点で「join 反映してよい変数」以外の更新は **reject** されている（silent fallback禁止）。
  - Lower は recipe を再判定せず、注入された `should_update_binding` のみで join 反映を行う（責務の逆流禁止）。
- ConditionalUpdateIf の着手前条件:
  - ConditionalUpdateIf の join/更新反映は、この注入ポリシーで表現できる形に限定し、SSOT を docs に固定してから実装する。

Drift checks:
- `rg -n "RecipeItem::IfJoin\\b" src/mir/builder/control_flow/plan --glob '!*.md'` → 0件
- `rg -n "build_join_payload\\(" src/mir/builder/control_flow/plan/parts/dispatch.rs` → 1件以上
- `rg -n "try_lower_general_if_view" src/mir/builder/control_flow/plan/parts/stmt.rs` → 0件
- `rg -n "build_stmt_only_block\\b" src/mir/builder/control_flow/plan/facts/no_exit_block.rs --glob '!*.md'` → 0件（NoExit join branch の裸リスト化を戻さない）
  - Drift check: `rg "features::exit_branch" src/mir/builder/control_flow/plan/parts` → 0件

### M9 (loop_cond_break_continue GeneralIf via RecipeBlock/dispatch)

- ✅ `loop_cond_break_continue` の `GeneralIf` lowering は `RecipeItem::IfV2{Join}` + `parts/dispatch` を SSOT とした（`conditional_update_join::lower_general_if_assume` 直呼びを廃止）。
- ✅ `parts/dispatch` は “stmt lowerer 注入” + “join 反映ポリシー注入” を受け取れる（箱ごとの binding 更新ポリシーを外出しできる）。

Drift checks:
- `rg -n "lower_general_if_assume" src/mir/builder/control_flow/plan/features/loop_cond_break_continue_pipeline/item_lowering.rs` → 0件
- `rg -n "lower_no_exit_block_with_stmt_lowerer_verified" src/mir/builder/control_flow/plan` → 想定箇所以外に増殖しない

### M10 (ElseGuardBreakIf join SSOT via parts/dispatch)

- ✅ `else_patterns.rs::lower_else_guard_break_if` の join 組み立て（`pre/then/else` map, `build_join_payload`, join 反映）を `parts/dispatch` の `lower_if_join_with_branch_lowerers(...)` に集約した。

Drift checks:
- `rg -n "build_join_payload\\(" src/mir/builder/control_flow/plan/features/loop_cond_break_continue_pipeline/else_patterns.rs` → 0件
- `rg -n "lower_if_join_with_branch_lowerers\\(" src/mir/builder/control_flow/plan` → 想定箇所以外に増殖しない

### M11 (ContinueIfWithElse join SSOT via parts/dispatch)

- ✅ `continue_if.rs::lower_continue_if_with_else` の if lowering を `parts::lower_if_join_with_branch_lowerers(...)` 経由に統一し、`lower_cond_branch` 直呼びと手動の state 保存/復元を削除した（挙動不変）。

Drift checks:
- `rg -n "lower_cond_branch\\(" src/mir/builder/control_flow/plan/features/loop_cond_break_continue_pipeline/continue_if.rs` → 0件

### M11b (ConditionalUpdateIf recipe-first via parts/conditional_update)

- ✅ `ConditionalUpdateIf` は Facts が cond_view + branch recipes（+ tail break/continue）を運び、features 側は parts に渡すだけにする（挙動不変、受理拡張なし）。

Drift checks:
- `rg -n "lower_conditional_update_if_assume_with_break_phi_args\\(" src/mir/builder/control_flow/plan/features/loop_cond_break_continue_pipeline --glob '!*.md'` → 0件（旧 entry を features に戻さない）
- `rg -n "lower_conditional_update_if_assume_with_break_phi_args_recipe_first\\b" src/mir/builder/control_flow/plan/parts/conditional_update.rs --glob '!*.md'` → 1件（recipe-first entry を parts SSOT に一箇所だけ置く）

### M12 (ElseOnlyReturnIf join SSOT via parts/dispatch)

- ✅ `else_patterns.rs::lower_else_only_return_if` の if lowering を `parts::lower_if_join_with_branch_lowerers(...)` 経由に統一し、`lower_cond_branch` 直呼びを削除した（挙動不変）。

Drift checks:
- `rg -n "lower_cond_branch\\(" src/mir/builder/control_flow/plan/features/loop_cond_break_continue_pipeline/else_patterns.rs` → 0件
- `rg -n "build_join_payload\\(" src/mir/builder/control_flow/plan/features/loop_cond_break_continue_pipeline/else_patterns.rs` → 0件
- `rg -n "lower_if_join_with_branch_lowerers\\(" src/mir/builder/control_flow/plan/features/loop_cond_break_continue_pipeline/else_patterns.rs` → 2件

### M13 (IfContractKind::Join SSOT rollout + dead helper removal)

- ✅ `loop_true_break_continue_pipeline.rs` の join 組み立て（`pre/then/else map` + `build_join_payload` + `lower_cond_branch` + join反映）を
  `parts::lower_if_join_with_branch_lowerers(...)` に統一した。
- ✅ call site 0 の `conditional_update_join::lower_general_if_assume` を削除し、dead な join 組み立て島を撤去した。
- ✅ 方針固定: `ConditionalUpdateIf` は join-bearing if 語彙（`IfContractKind::Join`）に寄せない（Select 語彙で完結）。

Drift checks:
- `rg -n "build_join_payload\\(" src/mir/builder/control_flow/plan/features/loop_true_break_continue_pipeline.rs` → 0件
- `rg -n "lower_cond_branch\\(" src/mir/builder/control_flow/plan/features/loop_true_break_continue_pipeline.rs` → 0件
- `rg -n "lower_general_if_assume\\b" src` → 0件
- `rg -n "build_join_payload\\(" src/mir/builder/control_flow/plan/features/conditional_update_join.rs` → 0件

### M14 (loop_cond_return_in_body join SSOT via parts/dispatch)

- ✅ `loop_cond_return_in_body_pipeline.rs` の join 組み立て（`pre/then/else map` + `build_join_payload` + join反映）を撤去し、
  `parts::lower_if_join_with_branch_lowerers(...)` に統一した（挙動不変）。

Drift checks:
- `rg -n "build_join_payload\\(" src/mir/builder/control_flow/plan/features/loop_cond_return_in_body_pipeline.rs` → 0件

### M15 (loop_cond_continue_only no-join if lowering SSOT via parts/dispatch)

- ✅ `loop_cond_continue_only_pipeline.rs` の `lower_cond_branch(...)` 直呼び（join無し if lowering 島）を撤去し、
  `parts::lower_if_join_with_branch_lowerers(...)` 経由に統一した（挙動不変、join反映は無効化）。

Drift checks:
- `rg -n "lower_cond_branch\\(" src/mir/builder/control_flow/plan/features/loop_cond_continue_only_pipeline.rs` → 0件

### M16 (loop_cond_continue_with_return no-join if lowering SSOT via parts/dispatch)

- ✅ `loop_cond_continue_with_return_pipeline.rs` の join無し if lowering（`lower_cond_branch(...)` 直呼び）を全撤去し、
  `parts::lower_if_join_with_branch_lowerers(...)` 経由に統一した（挙動不変、join反映は無効化）。

Drift checks:
- `rg -n "lower_cond_branch\\(" src/mir/builder/control_flow/plan/features/loop_cond_continue_with_return_pipeline.rs` → 0件

### M17 (generic_loop_body if-join SSOT via parts/dispatch)

- ✅ `generic_loop_body.rs` の if lowering に残っていた join 組み立て島（`then/else map` + join生成 + `lower_cond_branch` + join反映）を撤去し、
  `parts::lower_if_join_with_branch_lowerers(...)` に統一した（挙動不変）。

Drift checks:
- `rg -n "lower_cond_branch\\(" src/mir/builder/control_flow/plan/features/generic_loop_body.rs` → 0件
- `rg -n "\\bCoreIfJoin\\b" src/mir/builder/control_flow/plan/features/generic_loop_body.rs` → 0件

### M18 (exit_map if lowering SSOT via parts/dispatch)

- ✅ `exit_map.rs::lower_if_exit_stmt` の `lower_cond_branch(...)` 直呼びを撤去し、
  `parts::lower_if_join_with_branch_lowerers(...)` に統一した（挙動不変）。

Drift checks:
- `rg -n "lower_cond_branch\\(" src/mir/builder/control_flow/plan/features/exit_map.rs` → 0件
- `rg -n "\\blower_cond_branch\\(" src/mir/builder/control_flow/plan/features` → 0件

### Milestone (M18): features-side if/branch assembly eliminated

- ✅ features 配下から if/branch の「組み立て」が消滅し、SSOT が `parts::dispatch` に収束した。

Closeout drift checks (SSOT):
- `rg -n "\\blower_cond_branch\\(" src/mir/builder/control_flow/plan/features | wc -l` → 0
- `rg -n "build_join_payload\\(" src/mir/builder/control_flow/plan/features | wc -l` → 0

### M19 (If vocabulary unified via IfContractKind)

- ✅ `RecipeItem::IfJoin` を撤去し、`RecipeItem::IfV2 { contract: IfContractKind, ... }` に統一した。
- ✅ exit-only 側も `RecipeItem::IfV2 { contract: ExitOnly{..}, ... }` に統一し、`RecipeItem::If` を撤去した。

Drift checks:
- `rg -n "RecipeItem::IfJoin\\b" src/mir/builder/control_flow/plan --glob '!*.md'` → 0件
- `rg -n "RecipeItem::If\\b" src/mir/builder/control_flow/plan --glob '!*.md'` → 0件

### M20 (parts/dispatch entrypoint unified)

- ✅ `parts/dispatch.rs` の lowering 入口を `lower_block_internal(kind, ...)` に集約し、既存公開 API は薄い wrapper に統一した（挙動不変）。

Drift checks:
- `rg -n "fn lower_block_internal\\b" src/mir/builder/control_flow/plan/parts/dispatch.rs` → 1件以上

### M21 (accept_kind becomes log/contract-only)

- ✅ `LoopCondBreakAcceptKind` を “Facts→Lower 契約の exhaustive match + ログ用途” に限定し、挙動分岐の意味は Facts 側の boolean/field へ移した。

Drift checks:
- `rg -n "accept_kind == LoopCondBreakAcceptKind::" src/mir/builder/control_flow/plan/features` → 0件

### M22 (RecipeBlock assembly SSOT via recipe_tree::builders)

- ✅ `RecipeBlock` の「手組み」を `recipe_tree/builders.rs` に集約し、Parts/Features は builder を呼ぶだけに統一した（挙動不変）。

Drift checks:
- `rg -n "RecipeBlock::new\\(" src/mir/builder/control_flow/plan | cat` → `recipe_tree/builders.rs` と `recipe_to_block.rs` のみ

### M23 (RecipeBodies+if_body_id assembly SSOT via builders)

- ✅ `RecipeBodies::new()` と `if_body_id` 手組み（arena 初期化＋if-body登録）を `recipe_tree/builders.rs` に集約した（挙動不変）。

Drift checks:
- `rg -n "RecipeBodies::new\\(" src/mir/builder/control_flow/plan | cat` → `recipe_tree/builders.rs` のみ

### M24 (exit-only arena assembly SSOT via builders)

- ✅ exit-only tree の arena 手組み（`RecipeBodies::new()`）を `recipe_tree/builders.rs` に集約し、Parts 側から `RecipeBodies::new()` を完全排除した（挙動不変）。

Drift checks:
- `rg -n "RecipeBodies::new\\(" src/mir/builder/control_flow/plan | cat` → `recipe_tree/builders.rs` のみ

### M25 (ExitKind depth check in verifier)

- ✅ `parts/verify.rs` に ExitKind depth チェックを追加し、`depth != 1` を `[freeze:contract][recipe][exit_depth]` で reject した。
- ✅ depth 関連エラーのタグを `[freeze:contract][exit_depth]` に統一した（exit.rs, conditional_update.rs）。

Drift checks:
- `rg -n "\\[freeze:contract\\]\\[exit_depth\\]" src/mir/builder/control_flow/plan` → 5件（verify.rs, exit.rs, conditional_update.rs）

### M27 (generic_loop_body if(no-exit) dispatch via RecipeBlock)

- ✅ `generic_loop_body.rs::lower_if_stmt_v1` の if(no-exit) lowering を、planner_required 経路で RecipeBlock + `parts::lower_no_exit_block` 経由に統一した（挙動不変）。

Drift checks:
- `rg -n "build_join_payload\\(" src/mir/builder/control_flow/plan/features/generic_loop_body.rs` → 0件

### M28 (loop_true general-if no-exit dispatch via RecipeBlock)

- ✅ `loop_true_break_continue_pipeline.rs::lower_general_if_stmt` の if(no-exit) lowering を、planner_required 経路で RecipeBlock + `parts::entry::lower_no_exit_block_with_stmt_lowerer_verified` 経由に統一した（挙動不変）。

Drift checks:
- `rg -n "build_join_payload\\(" src/mir/builder/control_flow/plan/features/loop_true_break_continue_pipeline.rs` → 0件

### M29 (NoExit join dispatch consolidation)

- ✅ NoExit join dispatch は parts 側へ集約済み（features 直実装なし）。
- ✅ `parts::dispatch::block.rs` の `BlockKindInternal::NoExit` が `IfV2{Join}` を処理し、features 側での個別 join 組み立てを不要にした。
- ✅ `lower_no_exit_block_with_stmt_lowerer_verified` が SSOT entry として機能し、重複実装を排除。

Drift checks:
- `rg -n "build_join_payload\\(" src/mir/builder/control_flow/plan/features --glob '!*.md' | wc -l` → 徐減（0目標）

Progress (M5e):
- ✅ SSOT ドキュメントを更新し、parts::stmt/parts::exit が ExitBranch SSOT であることを記録した。
  - Drift check: `rg "parts/(stmt|exit)\\.rs" docs/development/current/main/design` → SSOT参照として存在

Progress (M5f):
- ✅ `plan::steps` を中立層として新設し、parts が features を直参照しない構造に統一した。
- ✅ `try_lower_general_if` を `parts/if_general.rs` に移設し、features は委譲のみにした。
- ✅ `lower_if_exit_stmt*` を `parts/if_exit.rs` に移設し、features は委譲のみにした。
  - Drift check: `rg "control_flow::plan::features::" src/mir/builder/control_flow/plan/parts` → 0件

Progress (M5g):
- ✅ `ExitIfTree` が `cond_view: CondBlockView` を保持し、Parts は AST から condition を抽出しない構造に統一した。
- ✅ `CondRef(StmtRef)` を `CondViewRef(CondBlockView)` に変更し、recipe_tree の条件参照を view ベースに統一した。
  - Drift check: `rg "cond_view: CondBlockView" src/mir/builder/control_flow/plan/loop_cond_break_continue/recipe.rs` → 存在
  - Drift check: `rg "CondViewRef" src/mir/builder/control_flow/plan/recipe_tree/node.rs` → 存在

Progress (M5h):
- ✅ `parts/if_general.rs` に `try_lower_general_if_view` を追加し、ASTNode 版は view 版に委譲する構造にした。
- ✅ `parts/if_exit.rs` に view-first 版を追加（`lower_if_exit_stmt_view` 等）し、ASTNode 版は view 版に委譲する構造にした。
- ✅ `parts/stmt.rs` で `CondBlockView::from_expr` を一度作成し、view 版を使う構造に統一した。
- ✅ `RecipeVerifier` に cond prelude empty チェックを追加した（当初 dev/strict、現行は契約常時ON）。
  - Drift check: `rg "CondBlockView::from_expr(condition)" src/mir/builder/control_flow/plan/parts/if_general.rs` → 1件（委譲 wrapper のみ）
  - Drift check: `rg "CondBlockView::from_expr(condition)" src/mir/builder/control_flow/plan/parts/if_exit.rs` → 2件（委譲 wrapper のみ）

Progress (M5i):
- ✅ `parts/exit_branch.rs` を新設し、exit-branch helper（`split_exit_branch`, `lower_exit_branch_with_prelude*`）を parts に移設した。
- ✅ `parts/if_exit.rs` を `parts::exit_branch` 直参照に変更し、`super::super::steps` 経由を廃止した。
- ✅ `steps/mod.rs` から `features::exit_branch` の re-export を削除した。
- ✅ `features/exit_branch.rs` を `parts::exit_branch` へ委譲する構造に変更した（互換維持）。
  - Drift check: `rg "features::exit_branch" src/mir/builder/control_flow/plan/steps/mod.rs` → 0件
  - Drift check: `rg "super::super::steps.*exit_branch" src/mir/builder/control_flow/plan/parts` → 0件

Progress (M5j):
- ✅ `effects_to_plans` を `plan/steps/effects.rs` に移設し、features/steps は委譲のみにした。
- ✅ `build_join_payload*` を `plan/steps/join_payload.rs` に移設し、features/steps は委譲のみにした。
  - Drift check: `rg "features::steps::effects" src/mir/builder/control_flow/plan` → 0件
  - Drift check: `rg "features::steps::join_payload" src/mir/builder/control_flow/plan` → 0件

Progress (M5k):
- ✅ `collect_carrier_inits` を `plan/steps/carrier_collect.rs` に移設。
- ✅ `lower_stmt_block` を `plan/steps/stmt_block.rs` に移設。
- ✅ `build_standard5_*` / `empty_carriers_args` を `plan/steps/loop_wiring_standard5.rs` に移設。
- ✅ `features/steps/` は facade（re-export のみ）に縮退。
  - Drift check: `rg "features::steps::(carrier_collect|stmt_block|loop_wiring_standard5)" src/mir/builder/control_flow/plan` → 0件
  - Drift check: `rg "^pub fn" src/mir/builder/control_flow/plan/features/steps` → 0件

Progress (M5l):
- ✅ `features/steps` を facade-only として固定し、参照は `plan::steps` に統一した。
  - Drift check: `rg "^pub fn" src/mir/builder/control_flow/plan/features/steps --glob '!mod.rs'` → 0件
  - Drift check: `rg "control_flow::plan::features::steps" src/mir/builder/control_flow/plan` → 0件

Progress (M5m):
- ✅ `RecipeBlock` / `dispatch` 経路を導入し、exit-only の再帰 lowering を Parts だけで完結できる入口を追加した。
- ✅ RecipeBlock verifier を導入し、壊れた recipe は `[freeze:contract][recipe]` で止まる（現行は常時ON）。
  - Drift check: `rg -n "verify_recipe_contract_if_enabled" src/mir/builder/control_flow/plan` → 0件

Next:
- Exit depth contract (current): `ExitKind::{Break,Continue}{depth!=1}` is fail-fast (`[freeze:contract][exit_depth]`). Supporting depth>1 requires a dedicated design SSOT + fixture pin (BoxCount) before widening acceptance.
- Expand `RecipeBlock -> parts::dispatch` usage beyond `loop_cond_break_continue` (keep it planner_required-only, behavior-preserving): prioritize additional "no-exit" if sites that currently assemble plans inline.

### Exception routes

Status: exception routes = 0 (all entry-gate migrations complete; the temporary tracking table was retired)

### Exception Removal Plan (SSOT)

- R1: return lowering 入口の統一
  - 対象: return系 7件
  - 方針: parts::entry に薄い入口を追加し、1件ずつ移行
  - 完了条件: exception table から return 系を削除
  - 進捗: 7件移行済み（features/exit_branch.rs, features/exit_map.rs, features/loop_cond_return_in_body_pipeline.rs, features/loop_cond_bc_else_patterns.rs, features/loop_true_break_continue_pipeline.rs, features/loop_cond_bc_item_stmt.rs, features/loop_cond_continue_with_return_pipeline.rs）

- R2: conditional_update 入口の統一
  - 対象: conditional_update 系 3件
  - 方針: parts::entry 経由に統一する（verify は維持）
  - 完了条件: exception table から conditional_update 系を削除
  - 進捗: 3件移行済み（features/loop_cond_co_stmt.rs, features/loop_cond_continue_with_return_pipeline.rs, features/loop_cond_bc_item_stmt.rs）
  - 状態: 完了（exception table から conditional_update 系を削除済み）

- R3: build_*_with_phi_args の扱い決定
  - 対象: build_*_with_phi_args 系 15件
  - 方針: Allowed Helper として扱うか、parts::entry に集約するかを Decision で固定
  - 完了条件: exception table のステータス更新（Allowed Helper / Removed）
  - **Decision**: Allowed Helper として扱う（entry 例外ではない）
  - **理由**: Parts内のexit構築ヘルパーで、entry gateをバイパスしているわけではない
  - 状態: 完了

- R4: loop_cond_exit_leaf 入口の統一
  - 対象: loop_cond_exit_leaf 1件
  - 方針: parts::entry に薄い入口を追加し、移行
  - 完了条件: exception table から削除
  - 状態: 完了（exception table は空（0件））

### M6 (CorePlan shrink)

- CorePlan が “意味論” を持たない状態になったら、名称/責務を縮退（`LoweredRecipe` など）。
- emit_frag が十分汎用化できたら CorePlan を削除（最終段）。
- ✅ M6-min1: `plan/core.rs` で `type LoweredRecipe = CorePlan` を導入し、Recipe/Parts 境界（`parts/entry.rs`）の戻り型を `Vec<LoweredRecipe>` に寄せた（挙動不変）。
  - Drift check: `rg -n "type LoweredRecipe = CorePlan" src/mir/builder/control_flow/plan/core.rs` → 1件
  - Drift check: `rg -n "Result<.*LoweredRecipe" src/mir/builder/control_flow/plan/parts/entry.rs` → 1件以上
- ✅ M6-min2: `parts/{dispatch,if_,loop_,stmt}` の機械境界（戻り型/スライス型）を `LoweredRecipe` へ段階移行した（挙動不変）。
  - Drift check: `rg -n "Vec<CorePlan>|\\[CorePlan\\]" src/mir/builder/control_flow/plan/parts/dispatch src/mir/builder/control_flow/plan/parts/if_.rs src/mir/builder/control_flow/plan/parts/loop_.rs src/mir/builder/control_flow/plan/parts/stmt.rs` → 0件
  - Note: enum match は `CorePlan::` を維持（名前置換は M6 後段）
- ✅ M6-min3: `lowerer/core`・`lowerer/plan_lowering`・`verifier/core` の入出力境界を `LoweredRecipe` へ段階移行した（挙動不変）。
  - Drift check: `rg -n "fn (lower|lower_with_stack|verify|verify_plan)|\\[LoweredRecipe\\]|Vec<LoweredRecipe>" src/mir/builder/control_flow/plan/lowerer/core.rs src/mir/builder/control_flow/plan/lowerer/plan_lowering.rs src/mir/builder/control_flow/plan/verifier/core.rs` → 1件以上
  - Note: 内部 validator/lower match は `CorePlan::` を維持（名前置換は M6 後段）
- ✅ M6-min4: `verifier/{plan_validators,position_validators,loop_body_validators,loop_validators}` の内部境界（slice/参照）を `LoweredRecipe` へ段階移行した（挙動不変）。
  - Drift check: `rg -n "\\[CorePlan\\]|&CorePlan|Vec<CorePlan>|Result<.*CorePlan" src/mir/builder/control_flow/plan/verifier --glob '!**/tests.rs'` → 0件
  - Note: validator の enum match は `CorePlan::` を維持（名前置換は M6 後段）
- ✅ M6-min5: `lowerer/{body_processing,body_processing/helpers,block_effect_emission,loop_preparation}` の内部境界（slice/参照）を `LoweredRecipe` へ段階移行した（挙動不変）。
  - Drift check: `rg -n "\\[CorePlan\\]|&CorePlan|Vec<CorePlan>|Result<.*CorePlan|Option<Vec<CorePlan>>" src/mir/builder/control_flow/plan/lowerer --glob '!**/tests.rs'` → 0件
  - Note: lowerer の enum match は `CorePlan::` を維持（名前置換は M6 後段）
- ✅ M6-min6: `parts/{conditional_update,if_general,if_exit,exit,exit_branch}` と `parts/dispatch/block` の残り型境界（戻り型/参照）を `LoweredRecipe` へ段階移行した（挙動不変）。
  - Drift check: `rg -n "\\[CorePlan\\]|&CorePlan|Vec<CorePlan>|Result<.*CorePlan|Option<Vec<CorePlan>>" src/mir/builder/control_flow/plan/parts --glob '!**/tests.rs'` → 0件
  - Note: parts の enum match は `CorePlan::` を維持（名前置換は M6 後段）
- ✅ M6-min7: `steps/{effects,stmt_block}` の型境界（戻り型/関数境界）を `LoweredRecipe` へ段階移行した（挙動不変）。
  - Drift check: `rg -n "\\[CorePlan\\]|&CorePlan|Vec<CorePlan>|Result<.*CorePlan|Option<Vec<CorePlan>>" src/mir/builder/control_flow/plan/steps --glob '!**/tests.rs'` → 0件
  - Note: `effects_to_plans` の `CorePlan::Effect` 生成は維持（名前置換は M6 後段）
- ✅ M6-min8: `features/{exit_branch,exit_map,exit_if_map,if_branch_lowering}` の型境界（戻り型/closure trait 境界）を `LoweredRecipe` へ段階移行した（挙動不変）。
  - Drift check: `rg -n "\\[CorePlan\\]|&CorePlan|Vec<CorePlan>|Result<.*CorePlan|Option<Vec<CorePlan>>" src/mir/builder/control_flow/plan/features/exit_branch.rs src/mir/builder/control_flow/plan/features/exit_map.rs src/mir/builder/control_flow/plan/features/exit_if_map.rs src/mir/builder/control_flow/plan/features/if_branch_lowering.rs` → 0件
  - Note: `build_*_only` などの `CorePlan::Exit(...)` helper は維持（名前置換は M6 後段）
- ✅ M6-min9: `recipe_tree/loop_true_early_exit_composer.rs` と `features/{scan_with_init_pipeline,split_scan_pipeline,split_emit,loop_cond_co_block,loop_cond_bc_nested_carriers}` の型境界（戻り型/参照）を `LoweredRecipe` へ段階移行した（挙動不変）。
  - Drift check: `rg -n "\\[CorePlan\\]|&CorePlan|Vec<CorePlan>|Result<.*CorePlan|Option<Vec<CorePlan>>" src/mir/builder/control_flow/plan/recipe_tree/loop_true_early_exit_composer.rs src/mir/builder/control_flow/plan/features/scan_with_init_pipeline.rs src/mir/builder/control_flow/plan/features/split_scan_pipeline.rs src/mir/builder/control_flow/plan/features/split_emit.rs src/mir/builder/control_flow/plan/features/loop_cond_co_block.rs src/mir/builder/control_flow/plan/features/loop_cond_bc_nested_carriers.rs` → 0件
  - Note: `CorePlan::Loop` / `CorePlan::Effect` の生成と `CorePlan::Loop` match は維持（名前置換は M6 後段）
- ✅ M6-min10: `features/{loop_cond_co_pipeline,loop_cond_co_stmt,loop_cond_co_continue_if,loop_cond_co_group_if}` の型境界（戻り型/closure trait 境界）を `LoweredRecipe` へ段階移行した（挙動不変）。
  - Drift check: `rg -n "\\[CorePlan\\]|&CorePlan|Vec<CorePlan>|Result<.*CorePlan|Option<Vec<CorePlan>>" src/mir/builder/control_flow/plan/features/loop_cond_co_pipeline.rs src/mir/builder/control_flow/plan/features/loop_cond_co_stmt.rs src/mir/builder/control_flow/plan/features/loop_cond_co_continue_if.rs src/mir/builder/control_flow/plan/features/loop_cond_co_group_if.rs` → 0件
  - Note: `CorePlan::Exit` / `CorePlan::Loop` の生成と match は維持（名前置換は M6 後段）
- ✅ M6-min11: `features/{loop_cond_bc,loop_cond_bc_item,loop_cond_bc_item_stmt,loop_cond_bc_util}` の型境界（戻り型/Option戻り型）を `LoweredRecipe` へ段階移行した（挙動不変）。
  - Drift check: `rg -n "\\[CorePlan\\]|&CorePlan|Vec<CorePlan>|Result<.*CorePlan|Option<Vec<CorePlan>>" src/mir/builder/control_flow/plan/features/loop_cond_bc.rs src/mir/builder/control_flow/plan/features/loop_cond_bc_item.rs src/mir/builder/control_flow/plan/features/loop_cond_bc_item_stmt.rs src/mir/builder/control_flow/plan/features/loop_cond_bc_util.rs` → 0件
  - Note: `CorePlan::Exit` / `CorePlan::Loop` / `CorePlan::Effect` の生成と match は維持（名前置換は M6 後段）
- ✅ M6-min12: `features/{nested_loop_depth1,generic_loop_body/helpers,generic_loop_body/v0}` の型境界（戻り型/参照/Vec境界）を `LoweredRecipe` へ段階移行した（挙動不変）。
  - Drift check: `rg -n "\\[CorePlan\\]|&CorePlan|Vec<CorePlan>|Result<.*CorePlan|Option<Vec<CorePlan>>" src/mir/builder/control_flow/plan/features/nested_loop_depth1.rs src/mir/builder/control_flow/plan/features/generic_loop_body/helpers.rs src/mir/builder/control_flow/plan/features/generic_loop_body/v0.rs` → 0件
  - Note: `CorePlan::Loop` / `CorePlan::If` / `CorePlan::Seq` / `CorePlan::Exit` の生成と match は維持（名前置換は M6 後段）
- ✅ M6-min13: `features/loop_cond_return_in_body_pipeline` の型境界（戻り型/Vec境界）を `LoweredRecipe` へ段階移行した（挙動不変）。
  - Drift check: `rg -n "\\[CorePlan\\]|&CorePlan|Vec<CorePlan>|Result<.*CorePlan|Option<Vec<CorePlan>>" src/mir/builder/control_flow/plan/features/loop_cond_return_in_body_pipeline.rs` → 0件
  - Note: `CorePlan::Loop` / `CorePlan::If` の生成と match は維持（名前置換は M6 後段）
- ✅ M6-min14: `features/loop_cond_bc_continue_if` の型境界（戻り型/closure trait 境界）を `LoweredRecipe` へ段階移行した（挙動不変）。
  - Drift check: `rg -n "\\[CorePlan\\]|&CorePlan|Vec<CorePlan>|Result<.*CorePlan|Option<Vec<CorePlan>>" src/mir/builder/control_flow/plan/features/loop_cond_bc_continue_if.rs` → 0件
  - Note: `CorePlan::Exit` の生成は維持（名前置換は M6 後段）
- ✅ M6-min15: `features/loop_true_break_continue_pipeline` の型境界（戻り型/Vec境界）を `LoweredRecipe` へ段階移行した（挙動不変）。
  - Drift check: `rg -n "\\[CorePlan\\]|&CorePlan|Vec<CorePlan>|Result<.*CorePlan|Option<Vec<CorePlan>>" src/mir/builder/control_flow/plan/features/loop_true_break_continue_pipeline.rs` → 0件
  - Note: `CorePlan::Loop` / `CorePlan::Exit` の生成と match は維持（名前置換は M6 後段）
- ✅ M6-min16: `features/{generic_loop_body/v1,loop_cond_bc_else_patterns,loop_cond_continue_with_return_pipeline}` の型境界（戻り型/Vec境界/closure trait 境界）を `LoweredRecipe` へ段階移行した（挙動不変）。
  - Drift check: `rg -n "\\[CorePlan\\]|&CorePlan|Vec<CorePlan>|Result<.*CorePlan|Option<Vec<CorePlan>>" src/mir/builder/control_flow/plan/features/generic_loop_body/v1.rs src/mir/builder/control_flow/plan/features/loop_cond_bc_else_patterns.rs src/mir/builder/control_flow/plan/features/loop_cond_continue_with_return_pipeline.rs` → 0件
  - Note: `CorePlan::Loop` / `CorePlan::Exit` / `CorePlan::Effect` の生成と match は維持（名前置換は M6 後段）
  - Milestone: `plan/features` 配下の `Vec<CorePlan>` / `Result<CorePlan>` 型境界は 0件（tests除く）
- ✅ M6-min17: `loop_{scan_v0,scan_methods_v0,scan_methods_block_v0,scan_phi_vars_v0,bundle_resolver_v0,collect_using_entries_v0}/pipeline` の型境界（戻り型/Vec境界/参照境界）を `LoweredRecipe` へ段階移行した（挙動不変）。
  - Drift check: `rg -n "\\[CorePlan\\]|&CorePlan|Vec<CorePlan>|Result<.*CorePlan|Option<Vec<CorePlan>>" src/mir/builder/control_flow/plan/loop_scan_v0/pipeline.rs src/mir/builder/control_flow/plan/loop_scan_methods_v0/pipeline.rs src/mir/builder/control_flow/plan/loop_scan_methods_block_v0/pipeline.rs src/mir/builder/control_flow/plan/loop_scan_phi_vars_v0/pipeline.rs src/mir/builder/control_flow/plan/loop_bundle_resolver_v0/pipeline.rs src/mir/builder/control_flow/plan/loop_collect_using_entries_v0/pipeline.rs` → 0件
  - Note: `CorePlan::Loop` / `CorePlan::Exit` / `CorePlan::Effect` の生成と match は維持（名前置換は M6 後段）
- ✅ M6-min18: `loop_true_break_continue/normalizer`・`generic_loop/normalizer`・`nested_loop_depth1/normalizer`・`nested_loop_plan`・`composer/coreloop_{v0,v1,v2_nested_minimal,single_entry}` の型境界（戻り型/Option戻り型）を `LoweredRecipe` へ段階移行した（挙動不変）。
  - Drift check: `rg -n "\\[CorePlan\\]|&CorePlan|Vec<CorePlan>|Result<.*CorePlan|Option<CorePlan>|Option<Vec<CorePlan>>" src/mir/builder/control_flow/plan/loop_true_break_continue/normalizer.rs src/mir/builder/control_flow/plan/generic_loop/normalizer.rs src/mir/builder/control_flow/plan/nested_loop_depth1/normalizer.rs src/mir/builder/control_flow/plan/nested_loop_plan.rs src/mir/builder/control_flow/plan/composer/coreloop_v0.rs src/mir/builder/control_flow/plan/composer/coreloop_v1.rs src/mir/builder/control_flow/plan/composer/coreloop_v2_nested_minimal.rs src/mir/builder/control_flow/plan/composer/coreloop_single_entry.rs` → 0件
  - Note: `CorePlan::Loop` の生成は維持（名前置換は M6 後段）
- ✅ M6-min19: `normalizer/mod.rs` と semantic normalizer helper 群の型境界（戻り型/Vec境界）を `LoweredRecipe` へ段階移行した（挙動不変）。
  - Drift check: `rg -n "\\[CorePlan\\]|&CorePlan|Vec<CorePlan>|Result<.*CorePlan|Option<CorePlan>|Option<Vec<CorePlan>>" src/mir/builder/control_flow/plan/normalizer/mod.rs src/mir/builder/control_flow/plan/normalizer/{loop_break.rs,simple_while_coreloop_builder.rs,helpers.rs,common.rs,loop_body_lowering.rs,value_join_args.rs,value_join_demo_if2.rs} --glob '!**/tests.rs' --glob '!*.md'` → 0件
  - Note: `CorePlan::Loop` / `CorePlan::Effect` の生成は維持（名前置換は M6 後段）
- ✅ M6-min20: `normalizer/{cond_lowering_entry,cond_lowering_if_plan,cond_lowering_freshen/*}` の型境界（戻り型/Vec境界/参照境界）を `LoweredRecipe` へ段階移行した（挙動不変）。
  - Drift check: `rg -n "\\[CorePlan\\]|&CorePlan|Vec<CorePlan>|Result<.*CorePlan|Option<CorePlan>|Option<Vec<CorePlan>>" src/mir/builder/control_flow/plan/normalizer/cond_lowering_entry.rs src/mir/builder/control_flow/plan/normalizer/cond_lowering_if_plan.rs src/mir/builder/control_flow/plan/normalizer/cond_lowering_freshen/mod.rs src/mir/builder/control_flow/plan/normalizer/cond_lowering_freshen/collector.rs src/mir/builder/control_flow/plan/normalizer/cond_lowering_freshen/remapper.rs src/mir/builder/control_flow/plan/normalizer/cond_lowering_freshen/verifier.rs` → 0件
  - Note: `CorePlan::If` / `CorePlan::Loop` / `CorePlan::Effect` の match・生成は維持（名前置換は M6 後段）
- ✅ M6-min21: `recipe_tree/loop_cond_composer` と `composer/shadow_adopt` の型境界（戻り型/Option戻り型/struct field）を `LoweredRecipe` へ段階移行した（挙動不変）。
  - Drift check: `rg -n "\\[CorePlan\\]|&CorePlan|Vec<CorePlan>|Result<.*CorePlan|Option<CorePlan>|Option<Vec<CorePlan>>" src/mir/builder/control_flow/plan/recipe_tree/loop_cond_composer.rs src/mir/builder/control_flow/plan/composer/shadow_adopt.rs` → 0件
  - Note: `CorePlan::` の match・生成は維持（名前置換は M6 後段）
- ✅ M6-min22: `recipe_tree/*_composer`（loop_simple/loop_break/if_phi/split_scan/scan_with_init/loop_true ほか）の型境界（戻り型）を `LoweredRecipe` へ段階移行した（挙動不変）。
  - Drift check: `rg -n "Result<CorePlan, Freeze>" src/mir/builder/control_flow/plan/recipe_tree --glob '*_composer.rs'` → 0件
  - Note: `generic_loop_composer` の `CorePlan::Loop` 生成は維持（名前置換は M6 後段）
- ✅ M6-min23: `composer/branchn_return` の型境界（`MatchReturnPlan.core_plan` / `Vec` 境界）を `LoweredRecipe` へ段階移行した（挙動不変）。
  - Drift check: `rg -n "Vec<CorePlan>|pub core_plan: CorePlan" src/mir/builder/control_flow/plan/composer/branchn_return.rs` → 0件
  - Note: `CorePlan::Effect` / `CorePlan::BranchN` / `CorePlan::Seq` の生成は維持（名前置換は M6 後段）
- ✅ M6-min24: `features/exit_branch`・`observability/flowbox_tags`・`branchn`・`normalizer/cond_lowering_freshen/remapper` の型境界（戻り型/参照/struct field）を `LoweredRecipe` へ段階移行した（挙動不変）。
  - Drift check: `rg -n \"->\\s*CorePlan\\b|&CorePlan\\b|Vec<CorePlan>|pub core_plan:\\s*CorePlan\" src/mir/builder/control_flow/plan/features/exit_branch.rs src/mir/builder/control_flow/plan/observability/flowbox_tags.rs src/mir/builder/control_flow/plan/branchn.rs src/mir/builder/control_flow/plan/normalizer/cond_lowering_freshen/remapper.rs` → 0件
  - Note: `CorePlan::` の match・生成は維持（名前置換は M6 後段）
- ✅ M6-min25: `core::{CoreLoopPlan, CoreIfPlan}` のフィールド境界（`body/then_plans/else_plans`）を `LoweredRecipe` へ段階移行した（挙動不変）。
  - Drift check: `rg -n "pub body: Vec<CorePlan>|pub then_plans: Vec<CorePlan>|pub else_plans: Option<Vec<CorePlan>>" src/mir/builder/control_flow/plan/core.rs` → 0件
  - Note: `CorePlan::Seq(Vec<CorePlan>)` は alias shrink 後段まで維持（型定義本体）
- ✅ M6-final-1: `CorePlan` 型境界の残差を whitelist 化し、再混入を防ぐ guard を導入した（挙動不変）。
  - Allowed residue (code): `src/mir/builder/control_flow/plan/core.rs` の `CorePlan::Seq(Vec<CorePlan>)` のみ（enum 型定義本体）
  - Allowed residue (comment/doc): `-> CorePlan` の doc/comment 文字列は検査対象外（comment-only line filter）
  - Drift check: `tools/checks/no_unapproved_coreplan_boundary.sh` → OK
  - Gate wiring: `tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh` の pre-check 群に接続済み

## Progress recount (scheduled checks)

各マイルストーンで必ず実施する再確認（逸脱ゼロの棚卸し）。

- Gate: `./tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh`
- Canary (opt-in): `SMOKES_ENABLE_SELFHOST=1 ./tools/smokes/v2/profiles/integration/selfhost/phase29bq_selfhost_planner_required_dev_gate_vm.sh`
- Drift check (examples):
  - `rg \"ExitIfTree\" src/mir/builder/control_flow/plan/features`（M3 以降は tree lowering 本体が features 側に残らない）
  - `rg \"eprintln!\\(\" src/mir/builder/control_flow/plan`（無条件ログの再流入を防ぐ）

## Risks

- else なし ExitAll の混在
- Exit 後の stmt が残る (double terminator)
- 深い再帰で variable_map clone が過剰

## Invariants

- AST rewrite 禁止
- Facts は Recipe を返すのみ
- Lower は Recipe 以外を再判定しない

## Idea: ExitKind depth normalization (SSOT)

- Rationale: Facts (depthなし) と RecipeTree/Parts (depthあり) の非対称を解消。
- Status: implemented (adapter only, no behavior change).
- Non-goal: accept depth>1; representation unification only.

### Scope (design-only)
- Facts: ExitKindFacts は depth を持たないまま維持
- RecipeTree/Parts: ExitKind は depth を持つ（現状維持）
- 目標: 表現の橋渡し（adapter）を SSOT 化する

### Adapter Plan
- 新設: ExitKindDepthView (analysis-only)
  - depth が 1 以外のときは freeze ではなく "UnsupportedDepth" として観測ログのみ
- Facts → RecipeTree 変換時に adapter を通す（lower は再判定しない）

### Invariants
- 受理条件は変えない
- depth>1 の受理は増やさない
- strict/dev の freeze 条件は既存のまま

### Verification (design-only)
- `rg -n "ExitKindFacts" src/` で depth を持たないことを確認
- `rg -n "ExitKind::" src/` で depth を持つ表現が RecipeTree/Parts に限定されていることを確認

## Idea: Verifier vs Lower duplication removal (SSOT)

- Rationale: VerifiedRecipeBlock を唯一ゲートにし、Lower の再検証を排除。
- Status: design-only (no behavior change).
- Non-goal: widen acceptance; only remove redundant checks.

## Idea: RecipeBody/StmtRef sharing to avoid AST reconstruction (SSOT)

- Rationale: 解析→抽出→AST再構築の二重経路を減らす。
- Status: design-only (no behavior change).
- Non-goal: change AST ownership; prefer sharing RecipeBody/StmtRef.

### Scope (design-only)
- RecipeBody/StmtRef を「共有参照の標準形」として優先
- AST の再構築は最小化（必要な場合のみ）

### Adapter Plan
- Facts/Extractor は RecipeBody/StmtRef を保持し、AST再構築しない
- RecipeTree builder が AST の所有を持つ唯一の場所に寄せる

### Invariants
- 挙動不変（acceptance に影響させない）
- AST rewrite 禁止
- Verifier が唯一の受理ゲート

### Verification (design-only)
- `rg -n "RecipeBody::new" src/mir/builder/control_flow/plan` で手組みの残存を棚卸し
- `rg -n "StmtRef::new" src/mir/builder/control_flow/plan` で直生成箇所を棚卸し

## CleanupWrap + cleanup region boundary (SSOT pointer)

- Cleanup 責務境界の正本は専用SSOTへ移動:
  - `docs/development/current/main/design/cleanupwrap-cleanup-region-boundary-ssot.md`
- 本書では pointer のみを維持し、受理/境界契約の重複定義をしない。
