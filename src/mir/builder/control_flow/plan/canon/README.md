# Plan Canon Modules

Responsibility:
- Analysis-only view built from Facts.
- No AST rewrite, no behavior changes.
- Return None when out of scope.

Boundaries:
- Canon must not lower to CorePlan.
- Canon must not re-parse facts/AST beyond observation.
- generic_loop condition observation lives in `canon/generic_loop/condition.rs`.
- generic_loop update observation lives in `canon/generic_loop/update.rs`.
- generic_loop step observation lives in `canon/generic_loop/step/{extract,placement}.rs`.

Forbidden:
- Do not synthesize or rewrite AST nodes (view only).
