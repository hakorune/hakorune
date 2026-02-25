Status: Active (Structural, Decision: accepted)
Scope: DomainPlan → Recipe-first migration (phased plan, behavior-preserving)
Related:
- `docs/development/current/main/design/recipe-first-entry-contract-ssot.md`
- `docs/development/current/main/design/recipe-tree-and-parts-ssot.md`
- `docs/development/current/main/design/compiler-expressivity-first-policy.md`
- `docs/development/current/main/design/selfhost-language-v1-freeze-ssot.md`

# Recipe-first Migration Phased Plan (Proposal)

## Intent

Move entry semantics from pattern-based DomainPlan selection to Recipe-first contracts,
without changing CorePlan semantics. This is a **structural refactor** (not a feature
addition) and must keep default behavior unchanged.

## Constraints (Must Respect)

- This is **not** a feature addition; it is structural cleanup with behavior preserved.
- Default behavior must remain unchanged; new paths are opt-in (planner_required/dev only).
- Recipe-first pilot remains **verification-only** until explicitly promoted.

## Phases (Structural Plan)

### Phase A — Recipe foundations (additive only)

**✅ Phase A 完了 (2026-01-23)**

- A1: Add RecipeContract type (new files only).
- A2: Add feature slots to SkeletonFacts (backward compatible).
- A3: Implement RecipeMatcher (parallel path; no routing changes).

### Phase B — Parallel path (planner_required/dev only)

**✅ Phase B 完了 (2026-01-23)**

- B1: Implement RecipeComposer (new).
- B2: Add Router path behind explicit flag (no default change).

### Phase C — Pattern migration (incremental, behavior-preserving)

**✅ Phase C (partial) 完了 (2026-01-23)**

Completed patterns (planner_required/dev only):
- Pattern2Break (C4/C5)
- Pattern3IfPhi (C6–C8)
- Pattern4Continue (C9)
- Pattern5InfiniteEarlyExit (C10)
- Pattern1SimpleWhile (C11)
- Pattern1CharMap (C12)
- Pattern1ArrayJoin (C13)
- Pattern6 ScanWithInit (C14)
- Pattern7 SplitScan (C14)
- Pattern8 BoolPredicateScan (C14)
- Pattern9 AccumConstLoop (C14)
- C15 Scan loops: loop_scan_methods_v0 / loop_scan_methods_block_v0 / loop_scan_phi_vars_v0 / loop_scan_v0
- C16 Collection loops: loop_collect_using_entries_v0 / loop_bundle_resolver_v0 / loop_true_break_continue
- C17 Condition loops: LoopCondBreakContinue / LoopCondContinueOnly / LoopCondContinueWithReturn / LoopCondReturnInBody

Remaining (next focus, ordered by selfhost canary hits):
- C14b: GenericLoop v0/v1 (recipe-first; v1-only in planner_required)

### Phase D — Cleanup (remove pattern names, no semantic change)
- D1: PlanRuleId enum
- D2: DomainPlan variants
- D3: normalizer/pattern*.rs

## Completion Criteria (Phase D)

Recipe-first migration is considered complete when all of the following are true:

- No pattern-specific DomainPlan variants remain (DomainPlan is label-only or removed).
- `normalizer/pattern*.rs` removed; routing uses Recipe contracts + composer.
- Planner rules no longer select by `Pattern*` names (grepable checks below).
- All existing gates pass with default settings (behavior unchanged).

Suggested checks:

- `rg -n "DomainPlan::Pattern" src/mir/builder/control_flow/plan`
- `rg -n "Pattern\\d" src/mir/builder/control_flow/plan`
- `rg --files -g 'src/mir/builder/control_flow/plan/normalize/pattern*.rs'`

## Current Action (Allowed Now)

### Active (now)
- Continue **planner_required-only** Recipe-first migration (verification → compose → recipe-only).
- Any new pattern must be fixture + gate + Acceptance Map + SSOT update, and remain planner_required only.

## Promotion Conditions

- Explicit decision recorded in `docs/development/current/main/20-Decisions.md`.
- Implementation plan scoped to “one pattern per commit.”
