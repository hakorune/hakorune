# Plan Facts Modules

Responsibility:
- Observe AST/CFG shapes and extract Facts.
- No AST rewrite.
- Conservative: return None when out of scope.

Forbidden:
- Do not call normalizer/lowerer/emit (facts are observation-only).
- Do not synthesize AST nodes or modify input shapes.
