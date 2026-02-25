Layer Guard — handlers/calls

Scope
- Route MIR Call by callee kind (Global/Method/Extern/Legacy only).
- Keep legacy resolver isolated for phased removal.

Allowed
- Use `super::*` and `super::super::utils::*` helpers (e.g., `normalize_arity_suffix`).

Forbidden
- Direct provider/registry imports from runtime or plugins.
- Code generation or MIR building is out of scope.

