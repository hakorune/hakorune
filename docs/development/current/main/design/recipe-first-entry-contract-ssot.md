Status: SSOT
Scope: Recipe-first entry contract (Facts → Recipe → Verify → Lower)
Related:
- `docs/development/current/main/design/recipe-tree-and-parts-ssot.md`
- `docs/development/current/main/design/planfrag-freeze-taxonomy.md`
- `docs/development/current/main/design/domainplan-thinning-ssot.md`

# Recipe-first Entry Contract (SSOT)

This document fixes the entry flow so that “pattern selection” does not define semantics.

## Pipeline (SSOT)

AST
  → Facts (observation only)
  → Recipe (structural contract)
  → Verifier (fail-fast: `[freeze:contract]`)
  → Lower (VerifiedRecipe only)
  → MIR

## Entry priority (SSOT)

入口の優先順位を固定し、統合後に揺れないようにする。

1) Router の recipe-first 入口（Facts → RecipeComposer）
2) strict/dev の pre-plan shadow_adopt（最小の構造 adopt のみ）
3) `none`（router は no-route を返し、外側の既存 caller fallback 判定へ戻す）
- Phase-3: release で recipe-first を優先するのは VerifiedRecipe が成立する場合。
  - nested loop は既定で保守側に置くが、nested-safe 例外（`nested_loop_minimal` / `generic_loop_v{1,0}` / `loop_cond_break_continue` の accept-kind allowlist）では recipe-first を先行許可する。
  - 上記例外で compose/verify/lower が reject のときは `Ok(None)` を返し、router の no-route 判定へ戻す（挙動互換）。

### Current baseline (2026-03-05)

- `PlanBuildOutcome` は `facts + recipe_contract` のみ（`outcome.plan` は撤去済み）。
- runtime router entry は `recipe_first | shadow_adopt | none` の3種に固定（`[plan/trace:entry_route]`）。
- `DomainPlan` / planner payload は runtime 経路から撤去済み。
- 本文中の Phase C* 章にある `DomainPlan` 記述は移行履歴として残る場合がある。現在契約は本節と上位 SSOT を優先する。

### Entry coherence（重なりの扱い）

- 候補が複数成立したら strict/dev(+planner_required) で **freeze**（入口の曖昧さは禁止）
- release は挙動を変えないが、設計目標は guard を disjoint にして一意化すること
- 「特定パターン優先」を避け、優先させたい場合は **guard を狭めて重なりを消す**（支配関係で一意化）
- loop_scan_methods_block_v0 が成立する場合は loop_scan_methods_v0 を候補にしない（entry disjoint）
- loop_scan_methods_block_v0 は nested loop では候補にしない（entry disjoint）

#### Entry guard matrix (summary)

| Candidate | Display label | Required facts (summary) | Excludes (summary) |
| --- | --- | --- | --- |
| `pattern2_break` | `LoopBreakRecipe` | facts present | excludes `loop_cond_break_continue`, `loop_true_break_continue`, `generic_loop_v1` |
| `pattern1_array_join` | `LoopArrayJoin` | facts present | excludes `loop_cond_break_continue` |
| `pattern1_char_map` | `LoopCharMap` | facts present | excludes `generic_loop_v1` |
| `pattern1_simplewhile` | `LoopSimpleWhile` | facts present | excludes `generic_loop_v1` |
| `pattern3_ifphi` | `IfPhiJoin` | facts present | excludes `loop_cond_break_continue` |
| `pattern4_continue` | `LoopContinueOnly` | facts present | excludes `loop_cond_break_continue`, `loop_cond_continue_only` |
| `pattern5_infinite_early_exit` | `LoopTrueEarlyExit` | facts present | excludes `loop_true_break_continue` |
| `loop_scan_methods_block_v0` | `LoopScanMethodsBlockV0` | non-nested scan methods | excludes `loop_scan_methods_v0`, `loop_cond_break_continue` |
| `loop_scan_methods_v0` | `LoopScanMethodsV0` | nested (or no block_v0) scan methods | excludes `loop_cond_break_continue` |
| `loop_scan_*` (v0/phi_vars/collect/bundle) | `LoopScan*` | facts present | excludes `loop_cond_break_continue` |
| `loop_bundle_resolver_v0` | `LoopBundleResolverV0` | facts present | excludes `generic_loop_v1` |
| `loop_cond_continue_only` | `LoopContinueOnly` | facts present | excludes `pattern4_continue` |
| `loop_cond_continue_with_return` | `LoopContinueWithReturn` | facts present | see bullet list |
| `loop_cond_return_in_body` | `LoopReturnInBody` | facts present | excludes `loop_cond_break_continue` |
| `loop_true_break_continue` | `LoopTrueBreakContinue` | facts present | excludes `pattern5_infinite_early_exit` |
| `loop_cond_break_continue` | `LoopExitIfBreakContinue` | exit-signal / conditional_update / nested-only | excluded by scan_methods candidates and pattern-specific disjoints |
| `scan_with_init` / `split_scan` / `pattern8_bool_predicate_scan` / `pattern9_accum_const_loop` | `ScanWithInit` / `SplitScan` / `BoolPredicateScan` / `AccumConstLoop` | facts present | excludes `loop_cond_break_continue` |
| `generic_loop_v0` | `LoopGenericFallbackV0` | fallback facts | release-only routing |
| `generic_loop_v1` | `LoopGenericRecipeV1` | general loop facts | excluded by pattern2_break / char_map / simplewhile / loop_cond_break_continue / loop_bundle_resolver |

Note: This matrix is a summary; the bullet list below is the authoritative SSOT. Display labels follow `entry-name-map-ssot.md`.
- Pattern2Break が成立する場合は LoopCondBreak を候補にしない（entry disjoint）
- Pattern2Break が成立する場合は LoopTrueBreak を候補にしない（entry disjoint）
- Pattern2Break が成立する場合は generic_loop_v1 を候補にしない（entry disjoint）
- pattern1_char_map が成立する場合、generic_loop_v1 を候補にしない
- pattern1_simplewhile が成立する場合は generic_loop_v1 を候補にしない
- loop_scan_methods_block_v0 は non-nested のみ候補（segments に nested がある場合は loop_scan_methods_v0 に寄せる）
- loop_cond_break_continue が成立する場合、generic_loop_v1 は候補にしない（conditional_update を含む）
- loop_bundle_resolver_v0 が成立する場合、generic_loop_v1 は候補にしない
- pattern3_ifphi が成立する場合、loop_cond_break_continue は候補にしない
- pattern4_continue が成立する場合、loop_cond_break_continue は候補にしない
- pattern4_continue が成立する場合、loop_cond_continue_only は候補にしない
- pattern5_infinite_early_exit が成立する場合、loop_true_break_continue は候補にしない
- pattern1_array_join が成立する場合、loop_cond_break_continue は候補にしない
- scan_methods_* が候補に入った場合のみ、loop_cond_break_continue を候補にしない
- loop_cond_return_in_body が成立する場合、loop_cond_break_continue は候補にしない
- loop_scan_*（v0/phi_vars/collect_using_entries/bundle_resolver）が成立する場合、loop_cond_break_continue は候補にしない
- scan_with_init / split_scan / pattern8_bool_predicate_scan / pattern9_accum_const_loop が成立する場合、loop_cond_break_continue は候補にしない
- 必要に応じて debug 時のみ `[plan/trace:entry_candidates]` で候補を可視化（任意・SSOTはこのポリシー）
- 観測: debug 時に `[plan/trace:entry_route]` で entry が recipe_first / shadow_adopt / none のどれに落ちたかを 1 行で確認できる
- BoxShape: `generic_loop_v1` は strict/dev(+planner_required) で registry 側に寄せ、shadow_adopt の残経路を縮退する（挙動不変）。
- BoxShape: `shadow_adopt` の generic 採用は候補一意（`generic_loop_v1` 優先、`generic_loop_v0` は v1 facts 不在時のみ）。v1→v0 の段階 fallback は行わない。

## Freeze responsibility (SSOT)

どこで freeze するかを固定する（入口の責務分担）。

- planner_required + recipe-first 対象: `recipe_contract` が無い場合は `[freeze:contract]`（fail-fast）。
- 例外入口（shadow_adopt / pre-plan / direct lower）は strict/dev **+ planner_required** で契約違反があれば freeze する（silent fallback 禁止）。
- 入口整理 Phase-1: shadow_adopt は recipe_contract がある場合のみ通す（strict/dev+planner_required では欠落を freeze、release は従来通り）。
- 入口整理 Phase-1: router で recipe-first が成立した場合は legacy へ落とさない（shadow_adopt のみ許容）。
- 入口整理 Phase-2: recipe-first が成立した場合は shadow_adopt もスキップ（legacy には絶対落とさない）。
- 入口整理 Phase-3: release でも recipe-first 成立後は shadow_adopt/legacy を通さない。
- 期待された plan が返らない場合の freeze は router で出す（taxonomy: `planfrag-freeze-taxonomy.md`）。
- Pattern6/7 の contract は専用タグで freeze（`[joinir/phase29ab/pattern6/contract]` / `[joinir/phase29ab/pattern7/contract]`）。

## FlowBox tag emission (SSOT)

- FlowBox adopt tag は **Verified CorePlan の lowering 時に 1 回だけ**出す。
- pre-plan / legacy で直接 tag を出さない（emit するなら router 側の lowering 経由に集約）。
- NotApplicable など「タグを出さない」ケースは router の責務で明示的に分岐する。

## Exceptions (entry bypass) — strict/dev

- Exception routes (for tracking only): `shadow_adopt`（pre-plan） / `direct lower`.
- Policy: VerifiedRecipeBlock 以外の entry は strict/dev で freeze する（fallback 禁止）。
- Non-planner_required + strict/dev: recipe-first 対象（Pattern5/6/7/Pattern1ArrayJoin）では
  `RecipeMatcher::try_match_loop()` を実行し、契約違反は freeze で止める（release は skip のみ）。

## Exception entry inventory (SSOT)

| Entry | Purpose | Allowed scope | FlowBox tag |
| --- | --- | --- | --- |
| `shadow_adopt` | strict/dev の最小 adopt（pre-plan） | allowlist のみ（Facts による構造が明確なもの） | router の Verified CorePlan lowering 経由のみ |
| `direct lower` | 既存の release 互換 | release のみ（strict/dev では禁止） | 出さない |

## Entry integration scope (SSOT)

- 統合対象の入口: `router` / `shadow_adopt` / `legacy`
- 統合後の責務: FlowBox emit は Verified CorePlan の lowering のみ、freeze は router と例外入口で fail-fast
- 非目標: BoxCount 追加なし、AST rewrite なし
- DomainPlan core_plan adopt は現在無効（入口分散を避けるため）

## Entry integration minimal gates (SSOT)

入口統合の回帰確認は **最小セット**を固定する。

- `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh`
- `./tools/smokes/v2/profiles/integration/joinir/phase29bp_planner_required_dev_gate_v4_vm.sh`
- FlowBox tags: `docs/development/current/main/design/flowbox-tag-coverage-map-ssot.md` の gate set

## Historical note: DomainPlan wording

- `DomainPlan` は移行期の語彙で、現在の runtime 経路では使用しない。
- 現在の entry 判定は Facts/Recipe/Verifier 契約のみで定義する。
- strict/dev の observability（`planner_first` / FlowBox tags）は Facts/Recipe/Verifier から機械的に出るようにする。

## Pilot: Pattern2Break (planner_required only)

- Pattern2Break now builds and verifies a RecipeBlock in planner_required mode.
- Lowering behavior is unchanged (DomainPlan → Normalizer path), so this is a proof-only step.
- Debug tag: `[recipe:verify] route=loop_break status=<ok|fail>` (guarded by `joinir_dev::debug_enabled()`).

## Phase C: Pattern2Break entry enforced (planner_required only)

- Pattern2Break now **requires** `recipe_contract` when `planner_required` is enabled.
- If planner hits Pattern2Break but `outcome.recipe_contract` is None, it's a `[freeze:contract]`.
- Debug tag: `[recipe:entry] loop_break: recipe_contract enforced`.
- Lowering behavior unchanged (DomainPlan → Normalizer path).

## Phase C2: Pattern2Break verification centralized (planner_required only)

- Pattern2Break recipe verification moved from `classic.rs` to `matcher.rs`.
- `try_match_loop` now returns `Result<Option<RecipeContract>, Freeze>`.
- Verification runs inside matcher (SSOT for entry verification).
- **planner_required mode**: verify/build failure = Freeze (fail-fast).
- Lowering behavior unchanged (DomainPlan → Normalizer path).

## Phase C3: Pattern2Break composed via RecipeComposer (planner_required only)

- Pattern2Break is now composed directly into CorePlan in planner_required mode.
- Route: `route_loop` → `RecipeComposer::compose_loop_break_recipe` → `PlanNormalizer::normalize_pattern2_break`
- Debug tag: `[recipe:compose] route=loop_break path=recipe_block`
- Lowering behavior unchanged (same normalizer, different entry path).
- DomainPlan still exists (for non-planner_required mode).

## Phase C4: Pattern2Break is recipe-only (planner_required only)

- Pattern2Break no longer returns DomainPlan in planner_required mode.
- `rules.rs` returns `(None, outcome)` for Pattern2Break when planner_required.
- Router detects Pattern2Break via `facts.pattern2_break` instead of `domain_plan`.
- Recipe compose runs **before** pre_plan to prevent generic/shadow absorption.
- Debug tag: `[recipe:entry] loop_break: recipe-only`
- (historical) DomainPlan path statement from migration phase.

## Phase C4b: Pattern2Break strict/dev observability shim (non-planner_required)

- strict/dev かつ **planner_required ではない**場合でも、回帰スモークが FlowBox adopt tag を要求するケースがある。
- そのため Router は DomainPlan ではなく Facts を見て分岐し、観測だけを安定化する（意味論は normalizer に委譲）。
  - `facts.pattern2_break` がある:
    - Plan subset / promotion など “タグ対象” の場合: `lower_verified_core_plan(... via=shadow)` で FlowBox adopt tag を出す
    - NotApplicable（promotion対象外）の場合: `PlanLowerer::lower(...)` で下ろし、FlowBox adopt tag は出さない（negative smoke）
  - `facts.pattern2_break` がない: 何もしない（plan へ入らず、FlowBox adopt tag も出さない）
- これは “入口/タグの整流” であり、受理形（BoxCount）を増やすものではない。
- BoxCount 進行中は入口整理を保留し、受理形の追加と混線させない（迷走防止）。
- Phase‑2 完了後、legacy は **release-only fallback**（strict/dev では使わない）。

### Recipe-only constraint (enforced by gate)

- planner_required/dev では Pattern2Break は **recipe-only**
- DomainPlan/Normalizer 経由は禁止
- recipe_contract が無い場合は Freeze
- Gate fixture: `apps/tests/phase29bq_pattern2_break_recipe_only_min.hako`

## Phase C6: Pattern3IfPhi recipe verification (planner_required only)

- Pattern3IfPhi now verifies recipe structure in `RecipeMatcher::try_match_loop()`.
- Recipe structure: `LoopV0 { body: [IfV2 { Join }, Stmt(increment)] }`
- Contract: `body_contract = NoExit` (no break/continue/return in body)
- Debug tag: `[recipe:verify] route=if_phi_join status=ok`
- Lowering behavior unchanged (DomainPlan → Normalizer path).

## Phase C7: Pattern3IfPhi composed via RecipeComposer (planner_required only)

- Pattern3IfPhi now composes CorePlan via `RecipeComposer::compose_if_phi_join_recipe()`.
- Route: `route_loop` → `RecipeComposer::compose_if_phi_join_recipe` → `PlanNormalizer::normalize_pattern3_if_phi`
- Debug tag: `[recipe:compose] route=if_phi_join path=recipe_block`
- Lowering behavior unchanged (same normalizer, different entry path).
- DomainPlan still exists (for non-planner_required mode).

## Phase C8: Pattern3IfPhi is recipe-only (planner_required only)

- Pattern3IfPhi no longer returns DomainPlan in planner_required mode.
- `rules.rs` returns `(None, outcome)` for Pattern3 when planner_required.
- Router detects Pattern3 via `facts.pattern3_ifphi` instead of `domain_plan`.
- Debug tag: `[recipe:entry] if_phi_join: recipe-only entry`
- (historical) DomainPlan path statement from migration phase.

## Phase C9: Pattern4Continue Recipe-first migration (planner_required only)

- Pattern4Continue now verifies recipe structure in `RecipeMatcher::try_match_loop()`.
- Recipe structure: `LoopV0 { body: [IfV2 { ExitOnly(Continue) }, Stmt(carrier_updates), Stmt(increment)] }`
- Contract: `body_contract = ExitAllowed` (continue exits the iteration)
- Debug tag: `[recipe:verify] route=loop_continue_only status=ok`
- Phase C9-2: Pattern4Continue composed via `RecipeComposer::compose_loop_continue_only_recipe()`.
- Debug tag: `[recipe:compose] route=loop_continue_only path=recipe_block`
- Phase C9-3: Pattern4Continue is recipe-only in planner_required mode.
- `rules.rs` returns `(None, outcome)` for Pattern4 when planner_required.
- Router detects Pattern4 via `facts.pattern4_continue` instead of `domain_plan`.
- Debug tag: `[recipe:entry] loop_continue_only: recipe-only entry`
- (historical) DomainPlan path statement from migration phase.

## Phase C10: Pattern5InfiniteEarlyExit Recipe-first migration (planner_required only)

- Pattern5InfiniteEarlyExit now verifies recipe structure in `RecipeMatcher::try_match_loop()`.
- Recipe structure: `LoopV0 { kind: Infinite, body: [IfV2 { ExitOnly }, Stmt?, Stmt(increment)] }`
- Contract: `body_contract = ExitAllowed` (return/break exits)
- Debug tag: `[recipe:verify] route=loop_true_early_exit status=ok`
- Phase C10-2: Pattern5 composed via `RecipeComposer::compose_loop_true_early_exit_recipe()`.
- Debug tag: `[recipe:compose] route=loop_true_early_exit path=recipe_block`
- Phase C10-3: Pattern5 is recipe-only in planner_required mode.
- `rules.rs` returns `(None, outcome)` for Pattern5 when planner_required.
- Router detects Pattern5 via `facts.pattern5_infinite_early_exit` instead of `domain_plan`.
- Non-planner_required: if `recipe_contract` is available, router may enter recipe-first to avoid legacy LoopBuilder.
- Debug tag: `[recipe:entry] loop_true_early_exit: recipe-only entry`
- (historical) DomainPlan path statement from migration phase.

## Phase C11: Pattern1SimpleWhile Recipe-first migration (planner_required only)

- Pattern1SimpleWhile now verifies recipe structure in `RecipeMatcher::try_match_loop()`.
- Recipe structure: `LoopV0 { kind: WhileLike, body: [Stmt(increment)] }`
- Contract: `body_contract = StmtOnly`, root verified with `NoExit`
- Debug tag: `[recipe:verify] route=loop_simple_while status=ok`
- Phase C11-2: Pattern1SimpleWhile composed via `RecipeComposer::compose_loop_simple_while_recipe()`.
- Debug tag: `[recipe:compose] route=loop_simple_while path=recipe_block`
- Phase C11-3: Pattern1SimpleWhile is recipe-only in planner_required mode.
- `rules.rs` returns `(None, outcome)` for Pattern1SimpleWhile when planner_required.
- Router detects Pattern1SimpleWhile via `facts.pattern1_simplewhile` instead of `domain_plan`.
- Debug tag: `[recipe:entry] loop_simple_while: recipe-only entry`
- (historical) DomainPlan path statement from migration phase.

## Phase C12: Pattern1CharMap Recipe-first migration (planner_required only)

- Pattern1CharMap now verifies recipe structure in `RecipeMatcher::try_match_loop()`.
- Recipe structure: `LoopV0 { kind: WhileLike, body: [Stmt(substring), Stmt(result_update), Stmt(increment)] }`
- Contract: `body_contract = StmtOnly`, root verified with `NoExit`
- NOTE: Body AST is reconstructed from Facts fields (Pattern1CharMapFacts does not store original body).
- Debug tag: `[recipe:verify] route=loop_char_map status=ok`
- Phase C12-2: Pattern1CharMap composed via `RecipeComposer::compose_loop_char_map_recipe()`.
- Debug tag: `[recipe:compose] route=loop_char_map path=recipe_block`
- Phase C12-3: Pattern1CharMap is recipe-only in planner_required mode.
- `rules.rs` returns `(None, outcome)` for Pattern1CharMap when planner_required.
- Router detects Pattern1CharMap via `facts.pattern1_char_map` instead of `domain_plan`.
- Debug tag: `[recipe:entry] loop_char_map: recipe-only entry`
- (historical) DomainPlan path statement from migration phase.

## Phase C13: Pattern1ArrayJoin Recipe-first migration (planner_required only)

- Pattern1ArrayJoin now verifies recipe structure in `RecipeMatcher::try_match_loop()`.
- Recipe structure: `LoopV0 { kind: WhileLike, body: [IfV2 { Join }, Stmt(element_append), Stmt(increment)] }`
- Contract: `body_contract = NoExit` (body contains IfV2, not StmtOnly)
- NOTE: Body AST is reconstructed from Facts fields (Pattern1ArrayJoinFacts does not store original body).
- Debug tag: `[recipe:verify] route=loop_array_join status=ok`
- Phase C13-2: Pattern1ArrayJoin composed via `RecipeComposer::compose_loop_array_join_recipe()`.
- Debug tag: `[recipe:compose] route=loop_array_join path=recipe_block`
- Phase C13-3: Pattern1ArrayJoin is recipe-only in planner_required mode.
- `rules.rs` returns `(None, outcome)` for Pattern1ArrayJoin when planner_required.
- Router detects Pattern1ArrayJoin via `facts.pattern1_array_join` instead of `domain_plan`.
- Debug tag: `[recipe:entry] loop_array_join: recipe-only entry`
- Non-planner_required: if `recipe_contract` is available, router may enter recipe-first to avoid legacy LoopBuilder.
- (historical) DomainPlan path statement from migration phase.

## Phase C14: Pattern6–9 Recipe-first migration (planner_required only)

### Pattern6 ScanWithInit

- Pattern6 ScanWithInit now verifies recipe structure in `RecipeMatcher::try_match_loop()`.
- Recipe structure: `LoopV0 { body: [IfV2 { ExitOnly(Return) }, Stmt(step)] }`
- Contract: `body_contract = ExitAllowed`
- Contract violation freeze tag (strict/dev): `[joinir/phase29ab/pattern6/contract]`
- Debug tag: `[recipe:verify] route=scan_with_init status=ok`
- Phase C14-2: Pattern6 composed via `RecipeComposer::compose_scan_with_init_recipe()`.
- Debug tag: `[recipe:compose] route=scan_with_init path=recipe_block`
- Phase C14-3: Pattern6 is recipe-only in planner_required mode.
- `rules.rs` returns `(None, outcome)` for Pattern6 when planner_required.
- Router detects Pattern6 via `facts.scan_with_init` instead of `domain_plan`.
- Debug tag: `[recipe:entry] scan_with_init: recipe-only entry`
- Non-planner_required: if `recipe_contract` is available, router may enter recipe-first to avoid legacy LoopBuilder.
- Gate fixtures: `apps/tests/phase29aq_string_index_of_min.hako`, `apps/tests/phase29aq_string_last_index_of_min.hako`

### Pattern7 SplitScan

- Pattern7 SplitScan now verifies recipe structure in `RecipeMatcher::try_match_loop()`.
- Recipe structure: `LoopV0 { body: [IfV2 { Join(then/else) }] }`
- Contract: `body_contract = NoExit`
- Contract violation freeze tag (strict/dev): `[joinir/phase29ab/pattern7/contract]`
- Debug tag: `[recipe:verify] route=split_scan status=ok`
- Phase C14-2: Pattern7 composed via `RecipeComposer::compose_split_scan_recipe()`.
- Debug tag: `[recipe:compose] route=split_scan path=recipe_block`
- Phase C14-3: Pattern7 is recipe-only in planner_required mode.
- `rules.rs` returns `(None, outcome)` for Pattern7 when planner_required.
- Router detects Pattern7 via `facts.split_scan` instead of `domain_plan`.
- Debug tag: `[recipe:entry] split_scan: recipe-only entry`
- Non-planner_required: if `recipe_contract` is available, router may enter recipe-first to avoid legacy LoopBuilder.
- Gate fixture: `apps/tests/phase29aq_string_split_min.hako`

### Pattern8 BoolPredicateScan

- Pattern8 now verifies recipe structure in `RecipeMatcher::try_match_loop()`.
- Recipe structure: `LoopV0 { body: [IfV2 { ExitOnly(Return false) }, Stmt(step)] }`
- Contract: `body_contract = ExitAllowed`
- Debug tag: `[recipe:verify] route=bool_predicate_scan status=ok`
- Phase C14-2: Pattern8 composed via `RecipeComposer::compose_bool_predicate_scan_recipe()`.
- Debug tag: `[recipe:compose] route=bool_predicate_scan path=recipe_block`
- Phase C14-3: Pattern8 is recipe-only in planner_required mode.
- `rules.rs` returns `(None, outcome)` for Pattern8 when planner_required.
- Router detects Pattern8 via `facts.pattern8_bool_predicate_scan` instead of `domain_plan`.
- Debug tag: `[recipe:entry] bool_predicate_scan: recipe-only entry`
- Gate fixture: `apps/tests/phase269_p0_pattern8_frag_min.hako`

### Pattern9 AccumConstLoop

- Pattern9 now verifies recipe structure in `RecipeMatcher::try_match_loop()`.
- Recipe structure: `LoopV0 { body: [Stmt(acc_update), Stmt(step)] }`
- Contract: `body_contract = StmtOnly`, root verified with `NoExit`
- Debug tag: `[recipe:verify] route=accum_const_loop status=ok`
- Phase C14-2: Pattern9 composed via `RecipeComposer::compose_accum_const_loop_recipe()`.
- Debug tag: `[recipe:compose] route=accum_const_loop path=recipe_block`
- Phase C14-3: Pattern9 is recipe-only in planner_required mode.
- `rules.rs` returns `(None, outcome)` for Pattern9 when planner_required.
- Router detects Pattern9 via `facts.pattern9_accum_const_loop` instead of `domain_plan`.
- Debug tag: `[recipe:entry] accum_const_loop: recipe-only entry`
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
- Debug tag: `[recipe:entry] scan_methods_v0: recipe-only entry`
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
- Debug tag: `[recipe:entry] scan_methods_block_v0: recipe-only entry`
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
- Debug tag: `[recipe:entry] scan_phi_vars_v0: recipe-only entry`

### loop_scan_v0

- Verifies existing recipe segments in `RecipeMatcher::try_match_loop()`.
- Segments:
  - Linear segments: `ExitAllowedBlockRecipe`
  - Nested segments: `NestedLoopRecipe` → verify `body_stmt_only` if present
- Debug tag: `[recipe:scan_v0] verified OK`
- Compose via `RecipeComposer::compose_loop_scan_v0()`
- Debug tag: `[recipe:compose] route=scan_v0 path=recipe_first`
- Recipe-only in planner_required mode (legacy plan lane retired)
- Debug tag: `[recipe:entry] scan_v0: recipe-only entry`

## Phase C16: Collection loops recipe-first migration (planner_required only)

### loop_collect_using_entries_v0

- Verifies `NoExitBlockRecipe` payload in `RecipeMatcher::try_match_loop()`.
- Debug tag: `[recipe:collect_using_entries] verified OK`
- Compose via `RecipeComposer::compose_loop_collect_using_entries_v0()`
- Debug tag: `[recipe:compose] route=collect_using_entries_v0 path=recipe_first`
- Recipe-only in planner_required mode (legacy plan lane retired)
- Debug tag: `[recipe:entry] collect_using_entries_v0: recipe-only entry`
- Gate fixture: `apps/tests/phase29bq_selfhost_blocker_collect_using_entries_loop_min.hako`

### loop_bundle_resolver_v0

- Verifies `ExitAllowedBlockRecipe` payload in `RecipeMatcher::try_match_loop()`.
- Debug tag: `[recipe:bundle_resolver] verified OK`
- Compose via `RecipeComposer::compose_loop_bundle_resolver_v0()`
- Debug tag: `[recipe:compose] route=bundle_resolver_v0 path=recipe_first`
- Recipe-only in planner_required mode (legacy plan lane retired)
- Debug tag: `[recipe:entry] bundle_resolver_v0: recipe-only entry`
- Gate fixture: `apps/tests/phase29bq_selfhost_blocker_bundle_resolver_min.hako`

### loop_true_break_continue

- Verifies `body_exit_allowed` when present; otherwise validates recipe indices in `LoopCondRecipe`.
- Debug tag: `[recipe:loop_true] verified OK`
- Compose via `RecipeComposer::compose_loop_true_break_continue()`
- Debug tag: `[recipe:compose] route=loop_true_break_continue path=direct_pipeline`
- Recipe-only in planner_required mode (legacy plan lane retired)
- Debug tag: `[recipe:entry] loop_true_break_continue: recipe-only entry`

## Phase C17: LoopCond* recipe-first migration (planner_required only)

### loop_cond_break_continue

- Verifies `body_exit_allowed` when policy is `ExitAllowed`; always validates recipe indices.
- Debug tag: `[recipe:loop_cond_break_continue] verified OK`
- Compose via `RecipeComposer::compose_loop_cond_break_continue()`
- Debug tag: `[recipe:compose] route=loop_cond_break_continue path=direct_pipeline`
- Recipe-only in planner_required mode (legacy plan lane retired)
- Debug tag: `[recipe:entry] loop_cond_break_continue: recipe-only entry`

### loop_cond_continue_only

- Verifies `ContinueOnlyRecipe` indices + span bounds.
- Debug tag: `[recipe:loop_cond_continue_only] verified OK`
- Compose via `RecipeComposer::compose_loop_cond_continue_only()`
- Debug tag: `[recipe:compose] route=loop_cond_continue_only path=direct_pipeline`
- Recipe-only in planner_required mode (legacy plan lane retired)
- Debug tag: `[recipe:entry] loop_cond_continue_only: recipe-only entry`
- Gate fixture: `apps/tests/phase29bq_loop_cond_continue_only_no_else_min.hako`

### loop_cond_continue_with_return

- Verifies `ContinueWithReturnRecipe` indices + span bounds.
- Debug tag: `[recipe:loop_cond_continue_with_return] verified OK`
- Compose via `RecipeComposer::compose_loop_cond_continue_with_return()`
- Debug tag: `[recipe:compose] route=loop_cond_continue_with_return path=direct_pipeline`
- Recipe-only in all modes (legacy plan lane retired)
- Debug tag: `[recipe:entry] loop_cond_continue_with_return: recipe-only entry`
- Gate fixture: `apps/tests/phase29bq_shortcircuit_loop_cond_continue_with_return_min.hako`

### loop_cond_return_in_body

- Verifies `LoopCondReturnInBodyRecipe` indices.
- Debug tag: `[recipe:loop_cond_return_in_body] verified OK`
- Compose via `RecipeComposer::compose_loop_cond_return_in_body()`
- Debug tag: `[recipe:compose] route=loop_cond_return_in_body path=direct_pipeline`
- Recipe-only in planner_required mode (legacy plan lane retired)
- Debug tag: `[recipe:entry] loop_cond_return_in_body: recipe-only entry`

## Rules for new shapes

- Define a RecipeBlock/RecipeTree contract first.
- Add fixture + fast gate + Acceptance Map update.
- DomainPlan label is optional and must not change semantics.

## Required updates (SSOT)

When adding or modifying entry behavior:

- SSOT: `docs/development/current/main/design/recipe-first-entry-contract-ssot.md`
- SSOT: `docs/development/current/main/design/recipe-tree-and-parts-ssot.md`
- `CURRENT_TASK.md`: record whether Recipe-first was preserved or violated

## Diagnostics

- Recipe contract failures use `[freeze:contract][<area>]`.
- Planner/Facts rejections use `[plan/freeze:*]` / `[plan/reject:*]`.
- JoinIR lowering failures use `[joinir/freeze]` or pattern contract tags.
