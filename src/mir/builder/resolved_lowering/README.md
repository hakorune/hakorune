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
- canonical lowering seeds separate RegionId and ScopeId stacks from the sealed
  function/function-body roots; BlockExpr consumes one exact pair and retires
  only pair-owned BindingRefs at scope leave.
- I1a owns disconnected branch transactions and conditional CFG/final-PHI
  materialization. It accepts an ordered join domain, never syntax or a flow
  product; may-rebind permits are an upper bound, and no BindingRef can be
  published until every PHI is defined.
- legacy statement/expression dispatch, Planner/CorePlan, Lambda, production
  If/Loop activation, Main, REPL, and ProgramV0 are outside this boundary.
