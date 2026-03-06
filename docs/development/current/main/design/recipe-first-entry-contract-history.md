Status: historical reference
Scope: Recipe-first entry migration notes moved out of the active contract SSOT
Related:
- `docs/development/current/main/design/recipe-first-entry-contract-ssot.md`
- `docs/development/current/main/design/recipe-tree-and-parts-ssot.md`
- `CURRENT_TASK.md`

# Recipe-first Entry Contract History

This file preserves the phase-by-phase migration notes that were previously embedded in
`recipe-first-entry-contract-ssot.md`.

Use this file for traceability only.
Current runtime contract is defined by the active SSOT, not by the historical phase notes below.

## Historical note: legacy planner payload wording

- legacy planner payload は移行期の語彙で、現在の runtime 経路では使用しない。
- 現在の entry 判定は Facts/Recipe/Verifier 契約のみで定義する。
- strict/dev の observability（`planner_first` / FlowBox tags）は Facts/Recipe/Verifier から機械的に出るようにする。

## Appendix A: Historical migration notes

## Pilot: LoopBreak Route (legacy label: Pattern2Break, planner_required only)

- LoopBreak route (legacy label: Pattern2Break) now builds and verifies a RecipeBlock in planner_required mode.
- Lowering behavior is unchanged (legacy normalizer path), so this is a proof-only step.
- Debug tag: `[recipe:verify] route=loop_break status=<ok|fail>` (guarded by `joinir_dev::debug_enabled()`).

## Phase C: LoopBreak Route entry enforced (legacy label: Pattern2Break, planner_required only)

- LoopBreak route (legacy label: Pattern2Break) now **requires** `recipe_contract` when `planner_required` is enabled.
- If planner hits Pattern2Break but `outcome.recipe_contract` is None, it's a `[freeze:contract]`.
- Debug tag: `[recipe:entry] loop_break: recipe_contract enforced`.
- Lowering behavior unchanged (legacy normalizer path).

## Phase C2: LoopBreak Route verification centralized (legacy label: Pattern2Break, planner_required only)

- LoopBreak route recipe verification (legacy label: Pattern2Break) moved from `classic.rs` to `matcher.rs`.
- `try_match_loop` now returns `Result<Option<RecipeContract>, Freeze>`.
- Verification runs inside matcher (SSOT for entry verification).
- **planner_required mode**: verify/build failure = Freeze (fail-fast).
- Lowering behavior unchanged (legacy normalizer path).

## Phase C3: LoopBreak Route composed via RecipeComposer (legacy label: Pattern2Break, planner_required only)

- LoopBreak route (legacy label: Pattern2Break) is now composed directly into CorePlan in planner_required mode.
- Route: `route_loop` → `RecipeComposer::compose_loop_break_recipe` → `PlanNormalizer::normalize_loop_break`（at the time, legacy file: `pattern2_break.rs`; current test harness: `loop_break.rs`）
- Debug tag: `[recipe:compose] route=loop_break path=recipe_block`
- Lowering behavior unchanged (same normalizer, different entry path).
- Historical migration note only; current runtime path does not reintroduce the legacy planner payload lane.

## Phase C4: LoopBreak Route is recipe-only (legacy label: Pattern2Break, planner_required only)

- LoopBreak route (legacy label: Pattern2Break) no longer returns the legacy planner payload in planner_required mode.
- `rules.rs` returns `(None, outcome)` for Pattern2Break when planner_required.
- Router detects Pattern2Break directly via `facts.pattern2_break`.
- Recipe compose runs **before** pre_plan to prevent generic/shadow absorption.
- Debug tag: `[recipe:entry] loop_break: recipe_contract enforced` (planner_required), `[recipe:entry] loop_break: recipe-only entry` (non-planner_required).
- (historical) payload-lane note from migration phase.

## Phase C4b: LoopBreak Route strict/dev observability shim (legacy label: Pattern2Break, non-planner_required)

- strict/dev かつ **planner_required ではない**場合でも、回帰スモークが FlowBox adopt tag を要求するケースがある。
- そのため Router は fact key を見て分岐し、観測だけを安定化する（意味論は normalizer に委譲）。
  - `facts.pattern2_break` がある:
    - Plan subset / promotion など “タグ対象” の場合: `lower_verified_core_plan(... via=shadow)` で FlowBox adopt tag を出す
    - NotApplicable（promotion対象外）の場合: `PlanLowerer::lower(...)` で下ろし、FlowBox adopt tag は出さない（negative smoke）
  - `facts.pattern2_break` がない: 何もしない（plan へ入らず、FlowBox adopt tag も出さない）
- これは “入口/タグの整流” であり、受理形（BoxCount）を増やすものではない。
- BoxCount 進行中は入口整理を保留し、受理形の追加と混線させない（迷走防止）。
- Phase‑2 完了後、legacy は **release-only fallback**（strict/dev では使わない）。

### Recipe-only constraint (enforced by gate)

- planner_required/dev では Pattern2Break は **recipe-only**
- historical payload-lane wording を current entry 契約へ持ち込むのは禁止
- recipe_contract が無い場合は Freeze
- Gate fixture: `apps/tests/phase29bq_pattern2_break_recipe_only_min.hako`

## Phase C6: IfPhiJoin Route recipe verification (legacy label: Pattern3IfPhi, planner_required only)

- IfPhiJoin route (legacy label: Pattern3IfPhi) now verifies recipe structure in `RecipeMatcher::try_match_loop()`.
- Recipe structure: `LoopV0 { body: [IfV2 { Join }, Stmt(increment)] }`
- Contract: `body_contract = NoExit` (no break/continue/return in body)
- Debug tag: `[recipe:verify] route=if_phi_join status=ok`
- Lowering behavior unchanged (legacy normalizer path).

## Phase C7: IfPhiJoin Route composed via RecipeComposer (legacy label: Pattern3IfPhi, planner_required only)

- IfPhiJoin route (legacy label: Pattern3IfPhi) now composes CorePlan via `RecipeComposer::compose_if_phi_join_recipe()`.
- Route: `route_loop` → `RecipeComposer::compose_if_phi_join_recipe` → `parts::entry::lower_loop_v0`
- Debug tag: `[recipe:compose] route=if_phi_join path=recipe_block`
- Lowering behavior unchanged (same normalizer, different entry path).
- Historical migration note only; current runtime path does not reintroduce the legacy planner payload lane.

## Phase C8: IfPhiJoin Route is recipe-only (legacy label: Pattern3IfPhi, planner_required only)

- IfPhiJoin route (legacy label: Pattern3IfPhi) no longer returns the legacy planner payload in planner_required mode.
- `rules.rs` returns `(None, outcome)` for Pattern3 when planner_required.
- Router detects the route directly via `facts.if_phi_join()`.
- Debug tag: `[recipe:entry] if_phi_join: recipe_contract enforced` (planner_required), `[recipe:entry] if_phi_join: recipe-only entry` (non-planner_required).
- (historical) payload-lane note from migration phase.

## Phase C9: LoopContinueOnly Route Recipe-first migration (legacy label: Pattern4Continue, planner_required only)

- LoopContinueOnly route (legacy label: Pattern4Continue) now verifies recipe structure in `RecipeMatcher::try_match_loop()`.
- Recipe structure: `LoopV0 { body: [IfV2 { ExitOnly(Continue) }, Stmt(carrier_updates), Stmt(increment)] }`
- Contract: `body_contract = ExitAllowed` (continue exits the iteration)
- Debug tag: `[recipe:verify] route=loop_continue_only status=ok`
- Phase C9-2: Pattern4Continue composed via `RecipeComposer::compose_loop_continue_only_recipe()`.
- Debug tag: `[recipe:compose] route=loop_continue_only path=recipe_block`
- Phase C9-3: Pattern4Continue is recipe-only in planner_required mode.
- `rules.rs` returns `(None, outcome)` for Pattern4 when planner_required.
- Router detects the route directly via `facts.loop_continue_only()`.
- Debug tag: `[recipe:entry] loop_continue_only: recipe_contract enforced` (planner_required), `[recipe:entry] loop_continue_only: recipe-only entry` (non-planner_required).
- (historical) payload-lane note from migration phase.

## Phase C10: LoopTrueEarlyExit Route Recipe-first migration (legacy label: Pattern5InfiniteEarlyExit, planner_required only)

- LoopTrueEarlyExit route (legacy label: Pattern5InfiniteEarlyExit) now verifies recipe structure in `RecipeMatcher::try_match_loop()`.
- Recipe structure: `LoopV0 { kind: Infinite, body: [IfV2 { ExitOnly }, Stmt?, Stmt(increment)] }`
- Contract: `body_contract = ExitAllowed` (return/break exits)
- Debug tag: `[recipe:verify] route=loop_true_early_exit status=ok`
- Phase C10-2: Pattern5 composed via `RecipeComposer::compose_loop_true_early_exit_recipe()`.
- Debug tag: `[recipe:compose] route=loop_true_early_exit path=recipe_block`
- Phase C10-3: Pattern5 is recipe-only in planner_required mode.
- `rules.rs` returns `(None, outcome)` for Pattern5 when planner_required.
- Router detects Pattern5 directly via `facts.pattern5_infinite_early_exit`.
- Non-planner_required: if `recipe_contract` is available, router may enter recipe-first to avoid legacy LoopBuilder.
- Debug tag: `[recipe:entry] loop_true_early_exit: recipe_contract enforced` (planner_required), `[recipe:entry] loop_true_early_exit: recipe-only entry` (non-planner_required).
- (historical) payload-lane note from migration phase.

## Phase C11: LoopSimpleWhile Route Recipe-first migration (legacy label: Pattern1SimpleWhile, planner_required only)

- LoopSimpleWhile route (legacy label: Pattern1SimpleWhile) now verifies recipe structure in `RecipeMatcher::try_match_loop()`.
- Recipe structure: `LoopV0 { kind: WhileLike, body: [Stmt(increment)] }`
- Contract: `body_contract = StmtOnly`, root verified with `NoExit`
- Debug tag: `[recipe:verify] route=loop_simple_while status=ok`
- Phase C11-2: Pattern1SimpleWhile composed via `RecipeComposer::compose_loop_simple_while_recipe()`.
- Debug tag: `[recipe:compose] route=loop_simple_while path=recipe_block`
- Phase C11-3: Pattern1SimpleWhile is recipe-only in planner_required mode.
- `rules.rs` returns `(None, outcome)` for Pattern1SimpleWhile when planner_required.
- Router detects Pattern1SimpleWhile directly via `facts.pattern1_simplewhile`.
- Debug tag: `[recipe:entry] loop_simple_while: recipe_contract enforced` (planner_required), `[recipe:entry] loop_simple_while: recipe-only entry` (non-planner_required).
- (historical) payload-lane note from migration phase.

## Phase C12: LoopCharMap Route Recipe-first migration (legacy label: Pattern1CharMap, planner_required only)

- LoopCharMap route (legacy label: Pattern1CharMap) now verifies recipe structure in `RecipeMatcher::try_match_loop()`.
- Recipe structure: `LoopV0 { kind: WhileLike, body: [Stmt(substring), Stmt(result_update), Stmt(increment)] }`
- Contract: `body_contract = StmtOnly`, root verified with `NoExit`
- NOTE: Body AST is reconstructed from Facts fields (Pattern1CharMapFacts does not store original body).
- Debug tag: `[recipe:verify] route=loop_char_map status=ok`
- Phase C12-2: Pattern1CharMap composed via `RecipeComposer::compose_loop_char_map_recipe()`.
- Debug tag: `[recipe:compose] route=loop_char_map path=recipe_block`
- Phase C12-3: Pattern1CharMap is recipe-only in planner_required mode.
- `rules.rs` returns `(None, outcome)` for Pattern1CharMap when planner_required.
- Router detects Pattern1CharMap directly via `facts.pattern1_char_map`.
- Debug tag: `[recipe:entry] loop_char_map: recipe_contract enforced` (planner_required), `[recipe:entry] loop_char_map: recipe-only entry` (non-planner_required).
- (historical) payload-lane note from migration phase.

## Phase C13: LoopArrayJoin Route Recipe-first migration (legacy label: Pattern1ArrayJoin, planner_required only)

- LoopArrayJoin route (legacy label: Pattern1ArrayJoin) now verifies recipe structure in `RecipeMatcher::try_match_loop()`.
- Recipe structure: `LoopV0 { kind: WhileLike, body: [IfV2 { Join }, Stmt(element_append), Stmt(increment)] }`
- Contract: `body_contract = NoExit` (body contains IfV2, not StmtOnly)
- NOTE: Body AST is reconstructed from Facts fields (Pattern1ArrayJoinFacts does not store original body).
- Debug tag: `[recipe:verify] route=loop_array_join status=ok`
- Phase C13-2: Pattern1ArrayJoin composed via `RecipeComposer::compose_loop_array_join_recipe()`.
- Debug tag: `[recipe:compose] route=loop_array_join path=recipe_block`
- Phase C13-3: Pattern1ArrayJoin is recipe-only in planner_required mode.
- `rules.rs` returns `(None, outcome)` for Pattern1ArrayJoin when planner_required.
- Router detects Pattern1ArrayJoin directly via `facts.pattern1_array_join`.
- Debug tag: `[recipe:entry] loop_array_join: recipe_contract enforced` (planner_required), `[recipe:entry] loop_array_join: recipe-only entry` (non-planner_required).
- Non-planner_required: if `recipe_contract` is available, router may enter recipe-first to avoid legacy LoopBuilder.
- (historical) payload-lane note from migration phase.

## Phase C14: Scan/Predicate/Accum Routes Recipe-first migration (legacy labels: Pattern6–9, planner_required only)

### ScanWithInit Route (legacy label: Pattern6)

- ScanWithInit route (legacy label: Pattern6) now verifies recipe structure in `RecipeMatcher::try_match_loop()`.
- Recipe structure: `LoopV0 { body: [IfV2 { ExitOnly(Return) }, Stmt(step)] }`
- Contract: `body_contract = ExitAllowed`
- Contract violation freeze tag (strict/dev): `[joinir/phase29ab/scan_with_init/contract]`
- Debug tag: `[recipe:verify] route=scan_with_init status=ok`
- Phase C14-2: Pattern6 composed via `RecipeComposer::compose_scan_with_init_recipe()`.
- Debug tag: `[recipe:compose] route=scan_with_init path=recipe_block`
- Phase C14-3: Pattern6 is recipe-only in planner_required mode.
- `rules.rs` returns `(None, outcome)` for Pattern6 when planner_required.
- Router detects Pattern6 directly via `facts.scan_with_init`.
- Debug tag: `[recipe:entry] scan_with_init: recipe_contract enforced` (planner_required), `[recipe:entry] scan_with_init: recipe-only entry` (non-planner_required).
- Non-planner_required: if `recipe_contract` is available, router may enter recipe-first to avoid legacy LoopBuilder.
- Gate fixtures: `apps/tests/phase29aq_string_index_of_min.hako`, `apps/tests/phase29aq_string_last_index_of_min.hako`

### SplitScan Route (legacy label: Pattern7)

- SplitScan route (legacy label: Pattern7) now verifies recipe structure in `RecipeMatcher::try_match_loop()`.
- Recipe structure: `LoopV0 { body: [IfV2 { Join(then/else) }] }`
- Contract: `body_contract = NoExit`
- Contract violation freeze tag (strict/dev): `[joinir/phase29ab/split_scan/contract]`
- Debug tag: `[recipe:verify] route=split_scan status=ok`
- Phase C14-2: Pattern7 composed via `RecipeComposer::compose_split_scan_recipe()`.
- Debug tag: `[recipe:compose] route=split_scan path=recipe_block`
- Phase C14-3: Pattern7 is recipe-only in planner_required mode.
- `rules.rs` returns `(None, outcome)` for Pattern7 when planner_required.
- Router detects Pattern7 directly via `facts.split_scan`.
- Debug tag: `[recipe:entry] split_scan: recipe_contract enforced` (planner_required), `[recipe:entry] split_scan: recipe-only entry` (non-planner_required).
- Non-planner_required: if `recipe_contract` is available, router may enter recipe-first to avoid legacy LoopBuilder.
- Gate fixture: `apps/tests/phase29aq_string_split_min.hako`

### BoolPredicateScan Route (legacy label: Pattern8)

- BoolPredicateScan route (legacy label: Pattern8) now verifies recipe structure in `RecipeMatcher::try_match_loop()`.
- Recipe structure: `LoopV0 { body: [IfV2 { ExitOnly(Return false) }, Stmt(step)] }`
- Contract: `body_contract = ExitAllowed`
- Debug tag: `[recipe:verify] route=bool_predicate_scan status=ok`
- Phase C14-2: Pattern8 composed via `RecipeComposer::compose_bool_predicate_scan_recipe()`.
- Debug tag: `[recipe:compose] route=bool_predicate_scan path=recipe_block`
- Phase C14-3: Pattern8 is recipe-only in planner_required mode.
- `rules.rs` returns `(None, outcome)` for Pattern8 when planner_required.
- Router detects Pattern8 directly via `facts.pattern8_bool_predicate_scan`.
- Debug tag: `[recipe:entry] bool_predicate_scan: recipe_contract enforced` (planner_required), `[recipe:entry] bool_predicate_scan: recipe-only entry` (non-planner_required).
- Gate fixture: `apps/tests/phase269_p0_pattern8_frag_min.hako`

### AccumConstLoop Route (legacy label: Pattern9)

- AccumConstLoop route (legacy label: Pattern9) now verifies recipe structure in `RecipeMatcher::try_match_loop()`.
- Recipe structure: `LoopV0 { body: [Stmt(acc_update), Stmt(step)] }`
- Contract: `body_contract = StmtOnly`, root verified with `NoExit`
- Debug tag: `[recipe:verify] route=accum_const_loop status=ok`
- Phase C14-2: Pattern9 composed via `RecipeComposer::compose_accum_const_loop_recipe()`.
- Debug tag: `[recipe:compose] route=accum_const_loop path=recipe_block`
- Phase C14-3: Pattern9 is recipe-only in planner_required mode.
- `rules.rs` returns `(None, outcome)` for Pattern9 when planner_required.
- Router detects Pattern9 directly via `facts.pattern9_accum_const_loop`.
- Debug tag: `[recipe:entry] accum_const_loop: recipe_contract enforced` (planner_required), `[recipe:entry] accum_const_loop: recipe-only entry` (non-planner_required).
- Gate fixture: `apps/tests/phase286_pattern9_frag_poc.hako`

## Phase C15: Scan loop v0 family recipe-first migration (planner_required only)

### loop_scan_methods_v0

- Verifies existing recipe segments in `RecipeMatcher::try_match_loop()`.
- Segments:
  - Linear segments: `NoExitBlockRecipe` → `BlockContractKind::NoExit`
  - Nested segments: `NestedLoopRecipe` → verify `body_stmt_only` if present
- Debug tag: `[recipe:scan_methods] verified OK`
- Compose via `RecipeComposer::compose_loop_scan_methods_v0()`
- Debug tag: `[recipe:compose] route=scan_methods_v0 path=recipe_first`
- Recipe-only in planner_required mode (legacy plan lane retired)
- Debug tag: `[recipe:entry] scan_methods_v0: recipe_contract enforced` (planner_required), `[recipe:entry] scan_methods_v0: recipe-only entry` (non-planner_required).
- Gate fixture: `apps/tests/phase29bq_selfhost_blocker_scan_methods_loop_min.hako`

### loop_scan_methods_block_v0

- Verifies existing recipe segments in `RecipeMatcher::try_match_loop()`.
- Segments:
  - Linear segments: `NoExitBlockRecipe` or `ExitAllowedBlockRecipe`
  - Nested segments: `NestedLoopRecipe` → verify `body_stmt_only` if present
- Debug tag: `[recipe:scan_methods_block] verified OK`
- Compose via `RecipeComposer::compose_loop_scan_methods_block_v0()`
- Debug tag: `[recipe:compose] route=scan_methods_block_v0 path=recipe_first`
- Recipe-only in planner_required mode (legacy plan lane retired)
- Debug tag: `[recipe:entry] scan_methods_block_v0: recipe_contract enforced` (planner_required), `[recipe:entry] scan_methods_block_v0: recipe-only entry` (non-planner_required).
- Gate fixture: `apps/tests/phase29bq_selfhost_blocker_scan_methods_loop_block_min.hako`

### loop_scan_phi_vars_v0

- Verifies existing recipe segments in `RecipeMatcher::try_match_loop()`.
- Segments:
  - Linear segments: `NoExitBlockRecipe`
  - Nested segments: `NestedLoopRecipe` → verify `body_stmt_only` if present
- Debug tag: `[recipe:scan_phi_vars] verified OK`
- Compose via `RecipeComposer::compose_loop_scan_phi_vars_v0()`
- Debug tag: `[recipe:compose] route=scan_phi_vars_v0 path=recipe_first`
- Recipe-only in planner_required mode (legacy plan lane retired)
- Debug tag: `[recipe:entry] scan_phi_vars_v0: recipe_contract enforced` (planner_required), `[recipe:entry] scan_phi_vars_v0: recipe-only entry` (non-planner_required).

### loop_scan_v0

- Verifies existing recipe segments in `RecipeMatcher::try_match_loop()`.
- Segments:
  - Linear segments: `ExitAllowedBlockRecipe`
  - Nested segments: `NestedLoopRecipe` → verify `body_stmt_only` if present
- Debug tag: `[recipe:scan_v0] verified OK`
- Compose via `RecipeComposer::compose_loop_scan_v0()`
- Debug tag: `[recipe:compose] route=scan_v0 path=recipe_first`
- Recipe-only in planner_required mode (legacy plan lane retired)
- Debug tag: `[recipe:entry] scan_v0: recipe_contract enforced` (planner_required), `[recipe:entry] scan_v0: recipe-only entry` (non-planner_required).

## Phase C16: Collection loops recipe-first migration (planner_required only)

### loop_collect_using_entries_v0

- Verifies `NoExitBlockRecipe` payload in `RecipeMatcher::try_match_loop()`.
- Debug tag: `[recipe:collect_using_entries] verified OK`
- Compose via `RecipeComposer::compose_loop_collect_using_entries_v0()`
- Debug tag: `[recipe:compose] route=collect_using_entries_v0 path=recipe_first`
- Recipe-only in planner_required mode (legacy plan lane retired)
- Debug tag: `[recipe:entry] collect_using_entries_v0: recipe_contract enforced` (planner_required), `[recipe:entry] collect_using_entries_v0: recipe-only entry` (non-planner_required).
- Gate fixture: `apps/tests/phase29bq_selfhost_blocker_collect_using_entries_loop_min.hako`

### loop_bundle_resolver_v0

- Verifies `ExitAllowedBlockRecipe` payload in `RecipeMatcher::try_match_loop()`.
- Debug tag: `[recipe:bundle_resolver] verified OK`
- Compose via `RecipeComposer::compose_loop_bundle_resolver_v0()`
- Debug tag: `[recipe:compose] route=bundle_resolver_v0 path=recipe_first`
- Recipe-only in planner_required mode (legacy plan lane retired)
- Debug tag: `[recipe:entry] bundle_resolver_v0: recipe_contract enforced` (planner_required), `[recipe:entry] bundle_resolver_v0: recipe-only entry` (non-planner_required).
- Gate fixture: `apps/tests/phase29bq_selfhost_blocker_bundle_resolver_min.hako`

### loop_true_break_continue

- Verifies `body_exit_allowed` when present; otherwise validates recipe indices in `LoopCondRecipe`.
- Debug tag: `[recipe:loop_true] verified OK`
- Compose via `RecipeComposer::compose_loop_true_break_continue()`
- Debug tag: `[recipe:compose] route=loop_true_break_continue path=direct_pipeline`
- Recipe-only in planner_required mode (legacy plan lane retired)
- Debug tag: `[recipe:entry] loop_true_break_continue: recipe_contract enforced` (planner_required), `[recipe:entry] loop_true_break_continue: recipe-only entry` (non-planner_required).

## Phase C17: LoopCond* recipe-first migration (planner_required only)

### loop_cond_break_continue

- Verifies `body_exit_allowed` when policy is `ExitAllowed`; always validates recipe indices.
- Debug tag: `[recipe:loop_cond_break_continue] verified OK`
- Compose via `RecipeComposer::compose_loop_cond_break_continue()`
- Debug tag: `[recipe:compose] route=loop_cond_break_continue path=direct_pipeline`
- Recipe-only in planner_required mode (legacy plan lane retired)
- Debug tag: `[recipe:entry] loop_cond_break_continue: recipe_contract enforced` (planner_required), `[recipe:entry] loop_cond_break_continue: recipe-only entry` (non-planner_required).

### loop_cond_continue_only

- Verifies `ContinueOnlyRecipe` indices + span bounds.
- Debug tag: `[recipe:loop_cond_continue_only] verified OK`
- Compose via `RecipeComposer::compose_loop_cond_continue_only()`
- Debug tag: `[recipe:compose] route=loop_cond_continue_only path=direct_pipeline`
- Recipe-only in planner_required mode (legacy plan lane retired)
- Debug tag: `[recipe:entry] loop_cond_continue_only: recipe_contract enforced` (planner_required), `[recipe:entry] loop_cond_continue_only: recipe-only entry` (non-planner_required).
- Gate fixture: `apps/tests/phase29bq_loop_cond_continue_only_no_else_min.hako`

### loop_cond_continue_with_return

- Verifies `ContinueWithReturnRecipe` indices + span bounds.
- Debug tag: `[recipe:loop_cond_continue_with_return] verified OK`
- Compose via `RecipeComposer::compose_loop_cond_continue_with_return()`
- Debug tag: `[recipe:compose] route=loop_cond_continue_with_return path=direct_pipeline`
- Recipe-only in all modes (legacy plan lane retired)
- Debug tag: `[recipe:entry] loop_cond_continue_with_return: recipe_contract enforced` (planner_required), `[recipe:entry] loop_cond_continue_with_return: recipe-only entry` (non-planner_required).
- Gate fixture: `apps/tests/phase29bq_shortcircuit_loop_cond_continue_with_return_min.hako`

### loop_cond_return_in_body

- Verifies `LoopCondReturnInBodyRecipe` indices.
- Debug tag: `[recipe:loop_cond_return_in_body] verified OK`
- Compose via `RecipeComposer::compose_loop_cond_return_in_body()`
- Debug tag: `[recipe:compose] route=loop_cond_return_in_body path=direct_pipeline`
- Recipe-only in planner_required mode (legacy plan lane retired)
- Debug tag: `[recipe:entry] loop_cond_return_in_body: recipe_contract enforced` (planner_required), `[recipe:entry] loop_cond_return_in_body: recipe-only entry` (non-planner_required).
