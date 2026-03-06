---
Status: SSOT
Scope: Condition Profile (CondProfile) skeleton types
Related:
- `docs/development/current/main/design/condition-observation-ssot.md`
- `src/mir/policies/cond_profile.rs`
---

# CondProfile SSOT

## Purpose

CondProfile captures **condition skeleton + parameters** without changing behavior.
It is an analysis-only structure used for verification and future routing decisions.

## Scope

- Skeleton only (CFG-level shape)
- Parameters only (bound/step/loop_var as data)
- **No routing changes** in this phase
- **Verifier-only** usage (no lowering / no codegen)

## Policy (SSOT)

- CondProfile is additive: it does not replace ConditionShape yet.
- No AST rewrite. CondProfile is built from observation views only.
- Lowering must not depend on CondProfile in C19-A.

## Skeletons

Minimal set (expand later):

- `CondSkeleton::LoopCond` (loop(condition) gate)

## Parameters

Minimal parameter vocabulary:

- `CondParam::LoopVar(String)`
- `CondParam::Bound(BoundExpr)`
- `CondParam::Step(StepExpr)`
- `CondParam::Cmp(CmpOp)`

## Expressions (minimal)

- `BoundExpr::LiteralI64(i64)`
- `BoundExpr::Var(String)`
- `BoundExpr::LengthOfVar(String)`
- `BoundExpr::LengthMinusVar { haystack: String, needle: String }`
- `BoundExpr::Unknown`

- `StepExpr::Delta(i64)`
- `StepExpr::Unknown`

- `CmpOp::Lt | Le | Gt | Ge | Eq | Ne`

## Verification-only contract

Verifier may check:

- `CondSkeleton` is consistent with facts
- Required params exist
- No routing changes based on CondProfile in C19-A

## C19-B: CondCanon → CondProfile

- CondCanon builds CondProfile from loop conditions.
- Params include LoopVar candidates and an optional Bound (literal/var only).
- No routing or lowering changes in this phase.

## C19-C: CondProfile in GenericLoopFacts

- GenericLoopV0Facts / GenericLoopV1Facts carry `cond_profile`.
- Facts storage is observation-only; downstream logic must not depend on it yet.

## C20-B: CondProfile in scan facts

- Char-map / array-join / bool-predicate-scan / accum-const-loop routes carry `cond_profile`
  via `LoopCharMapFacts` / `LoopArrayJoinFacts` / `BoolPredicateScanFacts` /
  `AccumConstLoopFacts`.
- Stored via ScanConditionObservation; observation-only.

## C20-C: Verifier observes scan facts

- Verifier observes `cond_profile` from scan facts (debug only).
- No accept/reject routing changes.

## C19-E: Verifier observes CondProfile

- Verifier receives CondProfile (observation only).
- No reject/route decisions depend on it yet.
- Debug tag is `[condprofile]` and emitted only from Verifier (debug only).

## C20-D5 prereq: idx_var contract (SSOT)

- `CondParam::LoopVar` **must** match Facts `loop_var`.
- Verifier enforces the contract; in planner_required, mismatch is a freeze (fail-fast).
- Release/default behavior remains unchanged (design-only contract at this stage).
- This contract is a prerequisite for C20-D5 (idx_var shrink).

## C20-D6: step mismatch observation (debug-only)

- If StepShape::AssignAddConst{k} and CondProfile::StepExpr::Delta(k) mismatch,
  emit `[condprofile:step_mismatch]` (debug only).

## D9 prereq: CondProfile completeness observation (debug-only)

- Emit `[condprofile:complete]` when LoopVar/Cmp/Bound/Step are present
  for `CondSkeleton::LoopCond`.
- Emit `[condprofile:incomplete]` otherwise (debug only).

## C20-D10: CondProfile priority observation (debug-only)

- Emit `[condprofile:priority] condprofile|legacy` based on completeness.
- Observation only; accept/reject remains ConditionShape-based.

## D12-obs: legacy fallback observation (debug-only)

- Emit `[condprofile:legacy_fallback] pattern=<name>` when CondProfile acceptance falls back.
