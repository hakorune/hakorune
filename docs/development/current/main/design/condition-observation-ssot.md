Status: SSOT
Scope: Condition observation (no rewrite) across Facts / Recipe / Lower
Related:
- `docs/development/current/main/design/recipe-first-entry-contract-ssot.md`
- `docs/development/current/main/design/recipe-tree-and-parts-ssot.md`
- `docs/development/current/main/design/generic-loop-v1-shape-ssot.md`
- `docs/development/current/main/design/generic-loop-v1-acceptance-by-recipe-ssot.md`
- `docs/development/current/main/design/condprofile-ssot.md`
- `docs/development/current/main/design/condprofile-migration-plan-ssot.md`

# Condition Observation SSOT

Purpose: fix where and how conditions are observed, to avoid duplicate or diverging logic.
This is **analysis-only** (no rewrite, no semantic change).

## 1) Entry Observations (SSOT)

### A. ConditionCanon (loop_var candidates)
- Location: `src/mir/builder/control_flow/plan/canon/generic_loop/condition.rs`
- Role: extract loop_var candidates and step compatibility for generic_loop (v0/v1).
- Rule: loop_var candidates **must** come from this layer only.

### B. ConditionShape (scan/route-friendly shapes)
- Location: `src/mir/builder/control_flow/plan/facts/loop_facts/condition_shape.rs`
- Role: classify scan/route-friendly condition forms (e.g., VarLessLength).
- Rule: scan/route extractors **must** consult ConditionShape instead of re-parsing AST.
- Note: Length/Size differences are stored in CondProfile (ConditionShape matching ignores `LengthMethod`).
- Note: Length minus needle details are stored in CondProfile (ConditionShape matching ignores needle var).
- Note: Comparison operator is stored in CondProfile (`CondParam::Cmp`), not in shape variants.
- Note: VarLessLiteral is used for accum-const-loop observation (`Var < integer literal`; legacy numbered label is traceability-only).
- Note: Reverse scan (VarGreaterEqualZero) is recognized via CondProfile (shape is legacy-only).
- Note: idx_var matching for scan_with_init uses CondProfile::LoopVar (shape idx_var is legacy-only).
- Note: StepShape vs CondProfile::StepExpr mismatch is observed (debug-only) for scan facts.
- Note: StepShape k-diff is stored in CondProfile::StepExpr; StepShape only checks var.
- Note: ConditionShape will shrink to skeleton-only (details move to CondProfile) in D8.
- Note: CondProfile completeness is observed (LoopVar/Cmp/Bound/Step) for scan facts (debug-only).
- Note: D9 plans to prioritize CondProfile evaluation, but acceptance remains ConditionShape-based.
- Note: D10 adds priority observation (`[condprofile:priority]`) without changing acceptance.
- Note: D11 plans to switch acceptance to CondProfile (design-only, legacy fallback retained).
- Note: D12 prefers CondProfile acceptance but falls back to legacy (no behavior change).
- Note: D12-obs logs legacy fallback cases (debug-only).
- Note: D13 drops legacy fallback when CondProfile is complete (incomplete still falls back).
- Note: D14 keeps legacy fallback on incomplete (fail-fast is deferred).
- Note: D16 freezes on incomplete for scan facts (no legacy fallback).
- Note: D17 plans expansion order for incomplete-freeze beyond scan facts.
- Note: D18 applies incomplete-freeze to loop-char-map only (legacy numbered label is traceability-only; others keep fallback).
- Note: D19 applies incomplete-freeze to loop-array-join (legacy numbered label is traceability-only).
- Note: D20 applies incomplete-freeze to bool-predicate-scan / accum-const-loop (legacy numbered labels are traceability-only).

### C. CondBlockView (lowering view)
- Location: `src/mir/builder/control_flow/plan/canon/cond_block_view.rs`
- Role: evaluation/lowering view (effects/prelude) used by parts/normalizer.
- Rule: lowering must consume CondBlockView; no ad-hoc condition parsing in parts/normalizer.

Allowed entry (SSOT):
- `CondBlockView::from_expr(&ASTNode)` のみ（BlockExpr の prelude/tail 抽出もここだけ）。
- `CondBlockView { ... }` の直接構築や “BlockExpr だけ特別扱い” の ad-hoc 抽出は禁止。

Drift checks:
- `rg -n "\\bCondBlockView\\s*\\{" src/mir/builder/control_flow/plan --glob '!*.md'` → 0（struct literal を戻さない）

### D. CondProfile (parameterized skeleton, analysis-only)
- Location: `src/mir/policies/cond_profile.rs`
- Role: condition skeleton + params (no routing / no lowering in C19-A).
- Rule: CondCanon → CondProfile is observation-only (no rewrite).
 - Status: CondCanon wiring enabled for generic_loop (C19-B, behavior unchanged).
 - Storage: GenericLoopV0Facts / GenericLoopV1Facts carry `cond_profile` (C19-C).
 - Adapter: ConditionShape → CondProfile mapping added in `scan_shapes.rs` (C19-D, observation-only).
 - Verifier: CondProfile is observed in verifier (C19-E, observation-only).
 - Storage: scan facts carry `cond_profile` for loop-char-map / loop-array-join / bool-predicate-scan / accum-const-loop (legacy numbered labels are traceability-only, C20-B).
 - Verifier: scan facts `cond_profile` observed (C20-C, observation-only).
 - Contract: idx_var (Facts.loop_var) must match CondProfile::LoopVar (C20-D5 prereq).
   Verifier enforces the contract (freeze on mismatch in planner_required).

## 2) Prohibited Observation Paths

- Duplicated condition parsing in extractors (AST checks without ConditionShape).
- Re-deriving loop_var candidates outside ConditionCanon.
- Rewriting conditions (analysis-only view only).

## 3) Responsibilities (SSOT)

| Layer | Responsibility |
| --- | --- |
| Facts | Choose observation path (ConditionCanon / ConditionShape) |
| Recipe | Structural contract (no condition parsing) |
| Verifier | Contract validation only |
| Lower | CondBlockView-only |

## 4) Update Rules

- New condition shapes must be added to **ConditionShape** and referenced by extractors.
- generic_loop condition acceptance must be updated in **ConditionCanon** only.
- Any new condition-related debug tags must be recorded in `ai-handoff-and-debug-contract.md`.

## 5) Minimal Success Criteria

- `rg -n "condition" src/mir/builder/control_flow/plan/extractors` does not show ad-hoc AST parsing
  unless it is delegated to ConditionShape or ConditionCanon.
- Lowering paths use CondBlockView only (no direct AST condition parsing).

## 6) CondProfile (parameterized skeleton)

Goal: reduce shape explosion by representing condition differences as parameters, not enum variants.

Pipeline (no rewrite, C19-A types only):

Facts → ConditionCanon → CondProfile → Verifier → VerifiedRecipe → Lower

Rules:
- CondProfile is **additive** (parallel) at first; existing ConditionShape stays until migrated.
- New condition variants should prefer CondProfile parameters instead of new enum shapes.
- StepShape の縮退は `[condprofile:step_mismatch]` の観測結果が十分低いことを条件とする。
- Verifier remains the only acceptance gate.

## 7) Final Target Shape (Body/Condition separation)

Target:
- Body layer keeps **CFG skeleton** as enum (e.g., `GenericLoopV1ShapeId`) for coverage/diagnostics.
- Condition layer moves to **CondProfile** (parameterized, no shape explosion).

Rationale:
- Skeleton changes affect CFG/SSA → enum is appropriate.
- Condition differences are data-level (cmp/offset/bounds) → parameters are appropriate.
- Verifier is the single acceptance gate; Lower consumes VerifiedRecipe only.
  ShapeId must not be used as an acceptance allowlist (SSOT: `generic-loop-v1-acceptance-by-recipe-ssot.md`).

## Idea: Facts/Canon observation unification (SSOT)

- Rationale: avoid duplicated extraction paths (CondBlockView vs ConditionShape/StepShape).
- Status: design-only (no behavior change).
- Non-goal: rewrite AST or change acceptance.

### Scope (design-only)
- Facts: ConditionShape/StepShape は観測用に残す（受理判断はしない）
- Canon: CondBlockView は観測の入口（ASTからの唯一の入口）
- 目標: 二重抽出の排除（Facts と Canon の役割分担を固定）

### Adapter Plan
- CondBlockView → CondProfile → Facts (observational bridge)
- ConditionShape/StepShape は CondProfile から導出（将来的に縮退）

### Invariants
- AST rewrite はしない
- Verifier が唯一の受理ゲート
- Lower は CondBlockView のみ使用

### Verification (design-only)
- Facts 側で AST 直解析が増えていないこと
- Canon が唯一の入口であること
