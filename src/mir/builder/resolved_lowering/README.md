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
- I1b consumes one pre-Builder verified statement-If flow in source preorder.
  Both branches start from the same post-condition BindingRef baseline; the
  sealed join-source matrix selects final PHI inputs, and all PHIs define
  before one effect-authorized batch publication.
- RegionId and ScopeId stacks remain separate. Statement If consumes exact
  control/branch identities and coverage only; no durable RegionId-to-block
  map is published before SA4.
- legacy statement/expression dispatch, Planner/CorePlan, Lambda, production
  Loop activation, Main, REPL, and ProgramV0 are outside this boundary.
