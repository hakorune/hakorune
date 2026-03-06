Status: SSOT
Scope: Recipe-first entry contract (Facts → Recipe → Verify → Lower)
Related:
- `docs/development/current/main/design/recipe-tree-and-parts-ssot.md`
- `docs/development/current/main/design/planfrag-freeze-taxonomy.md`
- `docs/development/current/main/design/domainplan-thinning-ssot.md`

# Recipe-first Entry Contract (SSOT)

This document fixes the entry flow so that entry-route selection does not define semantics; semantics come from Facts → Recipe → Verifier.

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
2) strict/dev の pre-plan guard（`shadow_pre_plan_guard_error`。adopt しない）
3) `none`（router は no-route を返し、外側の既存 caller fallback 判定へ戻す）
- Phase-3: release で recipe-first を優先するのは VerifiedRecipe が成立する場合。
  - nested loop は既定で保守側に置くが、nested-safe 例外（`nested_loop_minimal` / `generic_loop_v{1,0}` / `loop_cond_break_continue` の accept-kind allowlist）では recipe-first を先行許可する。
  - 上記例外で compose/verify/lower が reject のときは `Ok(None)` を返し、router の no-route 判定へ戻す（挙動互換）。

### Current baseline (2026-03-06)

- `PlanBuildOutcome` は `facts + recipe_contract` のみ（`outcome.plan` は撤去済み）。
- runtime router entry は `recipe_first | none` の2種（`[plan/trace:entry_route]`）。shadow pre-plan は guard-only で adopt しない。
- legacy planner payload lane は runtime 経路から撤去済み。
- Appendix A の migration notes は履歴参照専用。現在契約は本節と上位 SSOT を優先する。

### Entry coherence（重なりの扱い）

- 候補が複数成立したら strict/dev(+planner_required) で **freeze**（入口の曖昧さは禁止）
- release は挙動を変えないが、設計目標は guard を disjoint にして一意化すること
- 「特定パターン優先」を避け、優先させたい場合は **guard を狭めて重なりを消す**（支配関係で一意化）
- loop_scan_methods_block_v0 が成立する場合は loop_scan_methods_v0 を候補にしない（entry disjoint）
- loop_scan_methods_block_v0 は nested loop では候補にしない（entry disjoint）

#### Entry guard matrix (summary)

| Candidate (internal fact key) | Route/display label | Required facts (summary) | Excludes (summary) |
| --- | --- | --- | --- |
| `loop_break` | `LoopBreakRecipe` | facts present | excludes `loop_cond_break_continue`, `loop_true_break_continue`, `generic_loop_v1` |
| `loop_array_join` | `LoopArrayJoin` | facts present | excludes `loop_cond_break_continue` |
| `loop_char_map` | `LoopCharMap` | facts present | excludes `generic_loop_v1` |
| `loop_simple_while` | `LoopSimpleWhile` | facts present | excludes `generic_loop_v1` |
| `if_phi_join` | `IfPhiJoin` | facts present | excludes `loop_cond_break_continue` |
| `loop_continue_only` | `LoopContinueOnly` | facts present | excludes `loop_cond_break_continue`, `loop_cond_continue_only` |
| `loop_true_early_exit` | `LoopTrueEarlyExit` | facts present | excludes `loop_true_break_continue` |
| `loop_scan_methods_block_v0` | `LoopScanMethodsBlockV0` | non-nested scan methods | excludes `loop_scan_methods_v0`, `loop_cond_break_continue` |
| `loop_scan_methods_v0` | `LoopScanMethodsV0` | nested (or no block_v0) scan methods | excludes `loop_cond_break_continue` |
| `loop_scan_*` (v0/phi_vars/collect/bundle) | `LoopScan*` | facts present | excludes `loop_cond_break_continue` |
| `loop_bundle_resolver_v0` | `LoopBundleResolverV0` | facts present | excludes `generic_loop_v1` |
| `loop_cond_continue_only` | `LoopContinueOnly` | facts present | excludes `loop_continue_only` |
| `loop_cond_continue_with_return` | `LoopContinueWithReturn` | facts present | see bullet list |
| `loop_cond_return_in_body` | `LoopReturnInBody` | facts present | excludes `loop_cond_break_continue` |
| `loop_true_break_continue` | `LoopTrueBreakContinue` | facts present | excludes `loop_true_early_exit` |
| `loop_cond_break_continue` | `LoopExitIfBreakContinue` | exit-signal / conditional_update / nested-only | excluded by scan_methods candidates and pattern-specific disjoints |
| `scan_with_init` / `split_scan` / `bool_predicate_scan` / `accum_const_loop` | `ScanWithInit` / `SplitScan` / `BoolPredicateScan` / `AccumConstLoop` | facts present | excludes `loop_cond_break_continue` |
| `generic_loop_v0` | `LoopGenericFallbackV0` | fallback facts | release-only routing |
| `generic_loop_v1` | `LoopGenericRecipeV1` | general loop facts | excluded by loop_break (`loop_break`) / loop_char_map (`loop_char_map`) / loop_simple_while (`loop_simple_while`) / `loop_cond_break_continue` / `loop_bundle_resolver_v0` |

Note: This matrix is a summary; the bullet list below is the authoritative SSOT. Display labels follow `entry-name-map-ssot.md`.
Note: Candidate keys follow current semantic fact accessors where available; historical `pattern*` labels survive only in archive/migration docs.
- LoopBreak（facts: loop_break）が成立する場合は loop_cond_break_continue を候補にしない（entry disjoint）
- LoopBreak（facts: loop_break）が成立する場合は loop_true_break_continue を候補にしない（entry disjoint）
- LoopBreak（facts: loop_break）が成立する場合は generic_loop_v1 を候補にしない（entry disjoint）
- LoopCharMap（facts: `loop_char_map`）が成立する場合、generic_loop_v1 を候補にしない
- LoopSimpleWhile（facts: `loop_simple_while`）が成立する場合は generic_loop_v1 を候補にしない
- loop_scan_methods_block_v0 は non-nested のみ候補（segments に nested がある場合は loop_scan_methods_v0 に寄せる）
- loop_cond_break_continue が成立する場合、generic_loop_v1 は候補にしない（conditional_update を含む）
- loop_bundle_resolver_v0 が成立する場合、generic_loop_v1 は候補にしない
- IfPhiJoin（facts: `if_phi_join`）が成立する場合、loop_cond_break_continue は候補にしない
- LoopContinueOnly（facts: `loop_continue_only`）が成立する場合、loop_cond_break_continue は候補にしない
- LoopContinueOnly（facts: `loop_continue_only`）が成立する場合、loop_cond_continue_only は候補にしない
- LoopTrueEarlyExit（facts: `loop_true_early_exit`）が成立する場合、loop_true_break_continue は候補にしない
- LoopArrayJoin（facts: `loop_array_join`）が成立する場合、loop_cond_break_continue は候補にしない
- scan_methods_* が候補に入った場合のみ、loop_cond_break_continue を候補にしない
- loop_cond_return_in_body が成立する場合、loop_cond_break_continue は候補にしない
- loop_scan_*（v0/phi_vars/collect_using_entries/bundle_resolver）が成立する場合、loop_cond_break_continue は候補にしない
- ScanWithInit / SplitScan / BoolPredicateScan（`bool_predicate_scan`）/ AccumConstLoop（`accum_const_loop`）が成立する場合、loop_cond_break_continue は候補にしない
- 必要に応じて debug 時のみ `[plan/trace:entry_candidates]` で候補を可視化（任意・SSOTはこのポリシー）
- 観測: debug 時に `[plan/trace:entry_route]` で entry が recipe_first / none のどれに落ちたかを 1 行で確認できる
- BoxShape: `generic_loop_v1` は strict/dev(+planner_required) で registry 側に寄せ、shadow_adopt の残経路を縮退する（挙動不変）。
- BoxShape: `shadow_adopt` は generic loop（`generic_loop_v1/v0`）を採用しない。generic route は registry の recipe-first に一本化する。

## Freeze responsibility (SSOT)

Note: Phase C* sections below use route names as canonical labels, and keep migration-era numbered route labels only as legacy notes for traceability.

どこで freeze するかを固定する（入口の責務分担）。

- planner_required + recipe-first 対象: `recipe_contract` が無い場合は `[freeze:contract]`（fail-fast）。
- 例外入口（shadow pre-plan guard / direct lower）は strict/dev **+ planner_required** で契約違反があれば freeze する（silent fallback 禁止）。
- 入口整理 Phase-1: shadow pre-plan は guard-only（adopt禁止）。strict/dev+planner_required では contract freeze を返す。
- 入口整理 Phase-2: router で recipe-first が成立した場合は legacy へ落とさない。
- 入口整理 Phase-3: release でも recipe-first 成立後は shadow pre-plan/legacy を通さない。
- 期待された plan が返らない場合の freeze は router で出す（taxonomy: `planfrag-freeze-taxonomy.md`）。
- ScanWithInit/SplitScan の contract は専用タグで freeze（`[joinir/phase29ab/scan_with_init/contract]` / `[joinir/phase29ab/split_scan/contract]`）。

## FlowBox tag emission (SSOT)

- FlowBox adopt tag は **Verified CorePlan の lowering 時に 1 回だけ**出す。
- pre-plan / legacy で直接 tag を出さない（emit するなら router 側の lowering 経由に集約）。
- NotApplicable など「タグを出さない」ケースは router の責務で明示的に分岐する。

## Exceptions (entry bypass) — strict/dev

- Exception routes (for tracking only): `shadow_pre_plan_guard` / `direct lower`.
- Policy: VerifiedRecipeBlock 以外の entry は strict/dev で freeze する（fallback 禁止）。
- Non-planner_required + strict/dev: recipe-first 対象（LoopTrueEarlyExit / ScanWithInit / SplitScan / LoopArrayJoin）では
  `RecipeMatcher::try_match_loop()` を実行し、契約違反は freeze で止める（release は skip のみ）。

## Exception entry inventory (SSOT)

| Entry | Purpose | Allowed scope | FlowBox tag |
| --- | --- | --- | --- |
| `shadow_pre_plan_guard` | strict/dev の pre-plan guard（freeze gate） | planner_required contract / strict nested loop guard | adopt tag を出さない |
| `direct lower` | 既存の release 互換 | release のみ（strict/dev では禁止） | 出さない |

## Entry integration scope (SSOT)

- 統合対象の入口: `router` / `shadow_pre_plan_guard` / `legacy`
- 統合後の責務: FlowBox emit は Verified CorePlan の lowering のみ、freeze は router と例外入口で fail-fast
- 非目標: BoxCount 追加なし、AST rewrite なし
- historical payload-based core_plan adopt lane は現在無効（入口分散を避けるため）

## Entry integration minimal gates (SSOT)

入口統合の回帰確認は **最小セット**を固定する。

- `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh`
- `./tools/smokes/v2/profiles/integration/joinir/phase29bp_planner_required_dev_gate_v4_vm.sh`
- FlowBox tags: `docs/development/current/main/design/flowbox-tag-coverage-map-ssot.md` の gate set

## Historical migration notes

- Phase-by-phase migration notes for the legacy planner payload -> recipe-first migration were moved to:
  - `docs/development/current/main/design/recipe-first-entry-contract-history.md`
- Current runtime contract is defined by the sections above plus:
  - `docs/development/current/main/design/recipe-tree-and-parts-ssot.md`
  - `CURRENT_TASK.md`
- Historical labels (numbered route labels, pilot phase notes, migration checkpoints) are traceability-only and must not be read as current entry semantics.

## Rules for new shapes

- Define a RecipeBlock/RecipeTree contract first.
- Add fixture + fast gate + Acceptance Map update.
- Legacy planner label is optional and must not change semantics.

## Required updates (SSOT)

When adding or modifying entry behavior:

- SSOT: `docs/development/current/main/design/recipe-first-entry-contract-ssot.md`
- SSOT: `docs/development/current/main/design/recipe-tree-and-parts-ssot.md`
- `CURRENT_TASK.md`: record whether Recipe-first was preserved or violated

## Diagnostics

- Recipe contract failures use `[freeze:contract][<area>]`.
- Planner/Facts rejections use `[plan/freeze:*]` / `[plan/reject:*]`.
- JoinIR lowering failures use `[joinir/freeze]` or pattern contract tags.
