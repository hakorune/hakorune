## plan/skeletons

Responsibility: allocate CorePlan skeletons (blocks/frags) without AST re-analysis.

Rules:
- build skeletons only; no shape detection or policy decisions
- no AST rewrite; no feature-specific logic
- feature deltas belong in plan/features/
