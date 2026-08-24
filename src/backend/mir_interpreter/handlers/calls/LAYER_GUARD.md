Layer Guard — handlers/calls

Scope
- Route MIR Call by typed callee kind (Global/Method/Extern and the existing
  typed unsupported cases).
- Missing `Callee` is a terminal reject; no `func` register or module-name
  lookup is permitted in this owner. The instruction fields remain until R6.

Allowed
- Use `super::*` and `super::super::utils::*` helpers (e.g., `normalize_arity_suffix`).

Forbidden
- Direct provider/registry imports from runtime or plugins.
- Code generation or MIR building is out of scope.
