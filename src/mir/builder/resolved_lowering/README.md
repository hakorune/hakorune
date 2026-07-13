# Resolved lowering boundary

This directory owns the first production consumer of a sealed semantic owner.

Allowed inputs are only `CanonicalFirstFamilyPlanV1` values produced by the
whole-unit compiler preflight. Recursive lowering accepts sealed located-node
carriers and resolves lexical identity through exact source sites.

Invariants:

- `BindingRefV1 -> ValueId` is the canonical value environment.
- names are diagnostic cross-checks, never lookup keys.
- legacy `allocate_binding_id()` is structurally vetoed while an owner is installed.
- declarations, variable uses, assignment targets, and exits must all finish
  source coverage before the function draft can be published.
- legacy statement/expression dispatch, Planner/CorePlan, BlockExpr, Lambda,
  If/Loop, Main, REPL, and ProgramV0 are outside this boundary.
